use crate::commands::helps::download;
use crate::core::catalog::Catalog;
use crate::core::catalog::DownloadSlot;
use crate::core::context::Context;
use crate::core::ip::Ip;
use crate::core::ip::IpSpec;
use crate::core::iparchive::IpArchive;
use crate::core::lockfile::LockEntry;
use crate::core::lockfile::LockFile;
use crate::core::manifest;
use crate::core::manifest::IP_MANIFEST_FILE;
use crate::core::plugin::Process;
use crate::core::protocol::Protocol;
use crate::core::source::Source;
use crate::core::variable::VariableTable;
use crate::util::anyerror::AnyError;
use crate::util::anyerror::Fault;
use crate::util::environment::Environment;
use crate::util::filesystem::Standardize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

use cliproc::{cli, proc};
use cliproc::{Cli, Flag, Help, Optional, Subcommand};

pub type ProtocolMap<'a> = HashMap<&'a str, &'a Protocol>;

#[derive(Debug, PartialEq)]
pub struct Download {
    all: bool,
    missing: bool,
    list: bool,
    queue_dir: Option<PathBuf>,
    verbose: bool,
    force: bool,
}

impl Subcommand<Context> for Download {
    fn construct<'c>(cli: &'c mut Cli) -> cli::Result<Self> {
        cli.check_help(Help::default().text(download::HELP))?;
        Ok(Download {
            // Flags
            all: cli.check_flag(Flag::new("all"))?,
            missing: cli.check_flag(Flag::new("missing"))?,
            list: cli.check_flag(Flag::new("list"))?,
            force: cli.check_flag(Flag::new("force"))?,
            verbose: cli.check_flag(Flag::new("verbose"))?,
            // Options
            queue_dir: cli.check_option(Optional::new("queue").value("dir"))?,
        })
    }

    fn execute(self, c: &Context) -> proc::Result {
        // @idea: display lock entries as JSON? or use different env var for ORBIT_DOWNLOAD_LIST and ORBIT_VERSION_LIST

        // cannot happen
        if self.all == true && self.missing == true {
            panic!("cannot display all and missing lock entries");
        }

        if let Some(dir) = &self.queue_dir {
            if dir.exists() == true {
                panic!("queue directory must be a non-existent directory");
            }
        }

        let proto_map: ProtocolMap = c.get_config().get_protocols();

        // load the catalog (ignore errors because we are only downloading)
        let catalog = match self.force {
            true => {
                let mut cat = Catalog::new();
                cat.set_downloads_path(c.get_downloads_path());
                cat
            }
            false => Catalog::new().downloads(c.get_downloads_path())?,
        };

        // verify running from an IP directory and enter IP's root directory
        c.goto_ip_path()?;

        let ip = Ip::load(c.get_ip_path().unwrap().clone(), true)?;

        // verify a lockfile exists
        if ip.get_lock().is_empty() == true {
            panic!("cannot download due to missing lockfile")
        }

        let env = Environment::new()
            // read config.toml for setting any env variables
            .from_config(c.get_config())?
            // read ip manifest for env variables
            .from_ip(&Ip::load(c.get_ip_path().unwrap().clone(), true)?)?;

        let vtable = VariableTable::new().load_environment(&env)?;
        env.initialize();

        // default behavior is report only missing installations
        let missing_only = self.force == false || self.missing == true;

        // default behavior is to print out to console
        let to_stdout = self.list == true;

        // determine whether to filter out or keep the dev dependencies from the lock file
        let lf = ip.get_lock().keep_dev_dep_entries(&ip, self.all);

        let downloads =
            Self::compile_download_list(&LockEntry::from((&ip, true)), &lf, &catalog, missing_only);
        // print to console
        if to_stdout == true {
            downloads.iter().for_each(|(_, src)| println!("{}", src));
        // execute the command
        } else {
            Self::download_all(
                &downloads,
                &proto_map,
                vtable,
                self.verbose,
                self.queue_dir.as_ref(),
                c.get_downloads_path(),
                self.force,
            )?;
        }
        Ok(())
    }
}

impl Download {
    /// Generates a list of dependencies required to be downloaded from the internet.
    ///
    /// Enabling `missing_only` will only push sources for ip not already installed.
    pub fn compile_download_list<'a>(
        le: &LockEntry,
        lf: &'a LockFile,
        catalog: &Catalog,
        missing_only: bool,
    ) -> Vec<(IpSpec, Source)> {
        let mut vtable = VariableTable::new();
        lf.inner()
            .iter()
            .filter(|p| p.get_source().is_some() == true)
            .filter(|p| {
                p.matches_target(&le) == false
                    && (missing_only == false
                        || catalog.is_downloaded_slot(&p.to_download_slot_key()) == false)
            })
            .map(|f| {
                let spec = f.to_ip_spec();
                vtable.add("orbit.ip.name", spec.get_name().as_ref());
                vtable.add("orbit.ip.version", &spec.get_version().to_string());
                let processed_src = f.get_source().unwrap().clone().replace_vars_in_url(&vtable);
                (spec, processed_src)
            })
            .collect()
    }

    /// Calls a protocol for the given package and then places the download into
    /// the downloads folder.
    pub fn download(
        vtable: &mut VariableTable,
        spec: &IpSpec,
        src: &Source,
        queue: Option<&PathBuf>,
        download_dir: &PathBuf,
        protocols: &HashMap<&str, &Protocol>,
        verbose: bool,
        _force: bool,
    ) -> Result<(), Fault> {
        // use the user-provided queue directory or simply use a temporary directory
        let queue = match queue {
            Some(q) => {
                std::fs::create_dir_all(q)?;
                q.clone()
            }
            None => TempDir::into_path(TempDir::new()?),
        };

        // access the protocol
        if let Some(proto) = src.get_protocol() {
            match protocols.get(proto.as_str()) {
                Some(&entry) => {
                    println!(
                        "info: Downloading {} over \"{}\" protocol ...",
                        spec, &proto
                    );
                    let std_queue = PathBuf::standardize(&queue);
                    vtable.add("orbit.queue", std_queue.to_str().unwrap());
                    // update variable table for this lock entry
                    vtable.add("orbit.ip.name", spec.get_name().as_ref());
                    vtable.add("orbit.ip.version", &spec.get_version().to_string());
                    vtable.add("orbit.ip.source.url", src.get_url());
                    vtable.add("orbit.ip.source.protocol", entry.get_name());
                    vtable.add(
                        "orbit.ip.source.tag",
                        src.get_tag().unwrap_or(&String::new()),
                    );
                    // allow the user to handle placing the code in the queue
                    let entry: Protocol = entry.clone().replace_vars_in_args(&vtable);
                    if let Err(err) = entry.execute(&[], verbose, &std_queue.to_str().unwrap()) {
                        fs::remove_dir_all(queue)?;
                        return Err(err);
                    }
                }
                None => {
                    // potential to use --force here to avoid this error and try with default but not currently implemented that way
                    fs::remove_dir_all(queue)?;
                    return Err(
                        Box::new(AnyError(format!("Unknown protocol \"{}\"", &proto))).into(),
                    );
                }
            }
        }
        // try to use default protocol
        if src.is_default() == true {
            vtable.add("orbit.ip.name", spec.get_name().as_ref());
            vtable.add("orbit.ip.version", &spec.get_version().to_string());

            let processed_src = src.clone().replace_vars_in_url(&vtable);

            println!("info: Downloading {} ...", spec);
            if let Err(err) = Protocol::single_download(processed_src.get_url(), &queue) {
                fs::remove_dir_all(queue)?;
                return Err(err);
            }
        }
        // move the IP to the downloads folder
        if let Err(err) = Self::move_to_download_dir(&queue, download_dir, spec) {
            fs::remove_dir_all(queue)?;
            return Err(err);
        }
        // clean up temporary directory
        fs::remove_dir_all(queue)?;
        Ok(())
    }

    pub fn move_to_download_dir(
        queue: &PathBuf,
        downloads: &PathBuf,
        spec: &IpSpec,
    ) -> Result<(), Fault> {
        // code is in the queue now, move it to the downloads/ folder

        // find the IP
        for entry in manifest::find_file(&queue, IP_MANIFEST_FILE, false)? {
            // check if this is our IP
            match Ip::load(entry.parent().unwrap().to_path_buf(), true) {
                Ok(temp) => {
                    // move to downloads
                    if temp.get_man().get_ip().get_name() == spec.get_name()
                        && temp.get_man().get_ip().get_version() == spec.get_version()
                    {
                        // zip the project to the downloads directory
                        let download_slot_name =
                            DownloadSlot::new(spec.get_name(), spec.get_version(), temp.get_uuid());
                        let full_download_path = downloads.join(&download_slot_name.as_ref());
                        IpArchive::write(&temp, &full_download_path)?;
                        return Ok(());
                    }
                }
                Err(_) => {}
            }
        }
        // could not find the IP
        Err(AnyError(format!("Failed to detect/load the IP's manifest")))?
    }

    pub fn download_all(
        downloads: &Vec<(IpSpec, Source)>,
        proto_map: &HashMap<&str, &Protocol>,
        vtable: VariableTable,
        verbose: bool,
        queue: Option<&PathBuf>,
        download_dir: &PathBuf,
        force: bool,
    ) -> Result<(), Fault> {
        match downloads.len() {
            0 => {
                println!("info: No missing downloads");
                return Ok(());
            }
            1 => {
                println!("info: Downloading 1 package ...")
            }
            _ => {
                println!("info: Downloading {} packages ...", downloads.len())
            }
        }
        let mut vtable = vtable;
        let mut results = downloads.iter().filter_map(|e| {
            match Self::download(
                &mut vtable,
                &e.0,
                &e.1,
                queue,
                &download_dir,
                &proto_map,
                verbose,
                force,
            ) {
                Ok(_) => None,
                Err(e) => Some(e),
            }
        });
        if let Some(n) = results.next() {
            return Err(n);
        }

        Ok(())
    }
}

// add <url> argument to download? with --protocol <alias> option?
