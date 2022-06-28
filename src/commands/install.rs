use crate::Command;
use crate::FromCli;
use crate::core::ip::Ip;
use crate::interface::cli::Cli;
use crate::interface::arg::Optional;
use crate::interface::errors::CliError;
use crate::core::context::Context;
use crate::core::pkgid::PkgId;
use crate::core::version::Version;
use crate::util::anyerror::AnyError;
use crate::core::version::AnyVersion;
use crate::util::sha256;
use crate::util::sha256::Sha256Hash;

#[derive(Debug, PartialEq)]
pub struct Install {
    ip: Option<PkgId>,
    path: Option<std::path::PathBuf>,
    git: Option<String>,
    version: Option<AnyVersion>,
}

impl FromCli for Install {
    fn from_cli<'c>(cli: &'c mut Cli) -> Result<Self,  CliError<'c>> {
        cli.set_help(HELP);
        let command = Ok(Install {
            git: cli.check_option(Optional::new("git").value("url"))?,
            path: cli.check_option(Optional::new("path"))?,
            version: cli.check_option(Optional::new("ver").switch('v'))?,
            ip: cli.check_option(Optional::new("ip"))?,
        });
        command
    }
}

use colored::Colorize;
use git2::Repository;
use tempfile::tempdir;
use std::str::FromStr;
use crate::commands::search::Search;
use crate::core::extgit::ExtGit;

impl Command for Install {
    type Err = Box<dyn std::error::Error>;
    fn exec(&self, c: &Context) -> Result<(), Self::Err> {
        // verify user is not requesting the dev version to be installed
        let version = match &self.version {
            Some(v) => {
                if v == &AnyVersion::Dev {
                    return Err(AnyError(format!("{}", "a dev version cannot be installed to the cache")))?
                } else {
                    v
                }
            },
            None => &AnyVersion::Latest
        };

        let tempdir = tempdir()?;

        // get to the repository (root path)
        let ip = if let Some(ip) = &self.ip {
            // gather all manifests from all 3 levels
            let mut universe = Search::all_pkgid((c.get_development_path().unwrap(), c.get_cache_path(), &c.get_vendor_path()))?;
            let target = crate::core::ip::find_ip(&ip, universe.keys().into_iter().collect())?;
            // gather all possible versions found for this IP
            let mut inventory = universe.remove(&target).take().unwrap();

            // @TODO check the store/ for the repository

            // use DEV_PATH repository
            if let Some(m) = inventory.0.take() {
                Ip::from_manifest(m)
            // try to clone from remote repository if exists
            } else {
                todo!("clone from repository")
            }
        } else if let Some(url) = &self.git {
            // clone from remote repository
            let path = tempdir.path().to_path_buf();
            ExtGit::new().command(None).clone(url, &path)?;
            Ip::init_from_path(path)?
        } else if let Some(path) = &self.path {
            // traverse filesystem
            Ip::init_from_path(path.to_path_buf())?
        } else {
            return Err(AnyError(format!("select an option to install from '{}', '{}', or '{}'", "--ip".yellow(), "--git".yellow(), "--path".yellow())))?
        };
        let target = ip.get_manifest().as_pkgid();

        let repo = Repository::open(&ip.get_path())?;
        // find the specified version for the given ip
        let space = gather_version_tags(&repo)?;
        let version = get_target_version(&version, &space, &target)?;
        println!("detected version {}", version);

        // move into temporary directory to compute checksum for the tagged version
        let temp = tempfile::tempdir()?;
        let repo = Repository::clone(&ip.get_path().to_str().unwrap(), &temp)?;
        // get the tag
        let obj = repo.revparse_single(version.to_string().as_ref())?;
        // checkout code at the tag's marked timestamp
        repo.checkout_tree(&obj, None)?;

        // @TODO throw repository into the store/ for future use

        // perform sha256 on the directory after collecting all files
        std::env::set_current_dir(&temp)?;

        // must use '.' as current directory when gathering files for consistent checksum
        let ip_files = crate::core::fileset::gather_current_files(&std::path::PathBuf::from("."));
        // println!("{:?}", ip_files);
        let checksum = crate::util::checksum::checksum(&ip_files);
        println!("checksum: {}", checksum);

        // use checksum to create new directory slot
        let cache_slot_name = format!("{}-{}-{}", target.get_name(), version, checksum.to_string().get(0..10).unwrap());
        let cache_slot = c.get_cache_path().join(&cache_slot_name);
        if std::path::Path::exists(&cache_slot) == true {
            // verify the installed version is valid
            if let Some(sha) = Self::get_checksum_proof(&cache_slot, 0) {
                if sha == checksum {
                    return Err(AnyError(format!("IP {} version {} is already installed", target, version)))?
                }
            }
            println!("info: reinstalling due to bad checksum");
            // blow directory up for re-install
            std::fs::remove_dir_all(&cache_slot)?;
        }
        std::fs::create_dir(&cache_slot)?;
        // copy contents into cache slot
        let options = fs_extra::dir::CopyOptions::new();
        let mut from_paths = Vec::new();
        for dir_entry in std::fs::read_dir(temp.path())? {
            match dir_entry {
                Ok(d) => if d.file_name() != ".git" || d.file_type()?.is_dir() != true { from_paths.push(d.path()) },
                Err(_) => (),
            }
        }
        // copy rather than rename because of windows issues
        fs_extra::copy_items(&from_paths, &cache_slot, &options)?;
        // write the checksum to the directory
        std::fs::write(&cache_slot.join(crate::core::fileset::ORBIT_SUM_FILE), checksum.to_string().as_bytes())?;
        self.run()
    }
}

/// Finds the most compatible version matching `ver` among the possible `space`.
/// 
/// Errors if no version was found.
pub fn get_target_version<'a>(ver: &AnyVersion, space: &'a Vec<Version>, target: &PkgId) -> Result<&'a Version, AnyError> {
    // find the specified version for the given ip
    let mut latest_version: Option<&Version> = None;
    space.into_iter()
    .filter(|f| match &ver {
        AnyVersion::Specific(v) => crate::core::version::is_compatible(v, f),
        AnyVersion::Latest => true,
        _ => panic!("dev version cannot be filtered")
    })
    .for_each(|tag| {
        if latest_version.is_none() || &tag > latest_version.as_ref().unwrap() {
            latest_version = Some(tag);
        }
    });
    match latest_version {
        Some(v) => Ok(v),
        None => Err(AnyError(format!("\
ip '{}' has no version available as {}

To see all versions try `orbit probe {} --tags`", target, ver, target))),
    }
}

/// Collects all version tags from the given `repo` repository.
fn gather_version_tags(repo: &Repository) -> Result<Vec<Version>, Box<dyn std::error::Error>> {
    let tags = repo.tag_names(Some("*.*.*"))?;
    Ok(tags.into_iter()
        .filter_map(|f| {
            match Version::from_str(f?) {
                Ok(v) => Some(v),
                Err(_) => None,
            }
        })
        .collect())
}

impl Install {
    /// Gets the already calculated checksum from an installed IP from '.orbit-checksum'.
    /// 
    /// This fn can return the different levels of the check-sum, whether its the dynamic
    /// SHA (level 1) or the original SHA (level 0).
    /// 
    /// Returns `None` if the file does not exist, is unable to read into a string, or
    /// if the sha cannot be parsed.
    fn get_checksum_proof(p: &std::path::PathBuf, level: u8) -> Option<Sha256Hash> {
        let sum_file = p.join(crate::core::fileset::ORBIT_SUM_FILE);
        if std::path::Path::exists(&sum_file) == false {
            None
        } else {
            match std::fs::read_to_string(&sum_file) {
                Ok(text) => {
                    let mut sums = text.split_terminator('\n').skip(level.into());
                    match sha256::Sha256Hash::from_str(&sums.next().expect("level was out of bounds")) {
                        Ok(sha) => Some(sha),
                        Err(_) => None,
                    }
                }
                Err(_) => None,
            }
        }
    }

    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        // todo!()
        Ok(())
    }
}

const HELP: &str = "\
Places an immutable version of an ip to the cache for dependency usage.

Usage:
    orbit install [options]

Options:
    --ip <ip>               pkgid to access an orbit ip to install
    --ver, -v <version>     version to install
    --path <path>           local filesystem path to install from
    --git <url>             remote repository to clone
    --force                 install regardless of cache slot occupancy

Use 'orbit help install' to learn more about the command.
";