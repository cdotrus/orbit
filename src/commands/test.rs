use cliproc::{cli, proc, stage::Memory, Arg, Cli, Help, Subcommand};

use crate::commands::helps::test;
use crate::core::blueprint::Scheme;
use crate::core::catalog::Catalog;
use crate::core::context::Context;
use crate::core::fileset::Fileset;
use crate::core::ip::Ip;
use crate::core::lang::vhdl::token::Identifier;
use crate::core::lang::Language;
use crate::core::target::Process;
use crate::core::target::Target;
use crate::error::Error;
use crate::error::LastError;
use crate::util::anyerror::Fault;
use crate::util::environment::ORBIT_OUTPUT_PATH;
use crate::util::environment::{EnvVar, Environment, ORBIT_BLUEPRINT, ORBIT_TARGET_DIR};
use crate::util::filesystem;

use super::plan::{self, Plan};

#[derive(Debug, PartialEq)]
pub struct Test {
    target: Option<String>,
    args: Vec<String>,
    list: bool,
    dirty: bool,
    target_dir: Option<String>,
    force: bool,
    all: bool,
    plan: Option<Scheme>,
    verbose: bool,
    dut: Option<Identifier>,
    command: Option<String>,
    filesets: Option<Vec<Fileset>>,
    bench: Option<Identifier>,
}

impl Subcommand<Context> for Test {
    fn interpret<'c>(cli: &'c mut Cli<Memory>) -> cli::Result<Self> {
        cli.help(Help::with(test::HELP))?;
        Ok(Test {
            // Flags
            list: cli.check(Arg::flag("list"))?,
            verbose: cli.check(Arg::flag("verbose"))?,
            force: cli.check(Arg::flag("force"))?,
            all: cli.check(Arg::flag("all"))?,
            dirty: cli.check(Arg::flag("no-clean"))?,
            // Options
            dut: cli.get(Arg::option("dut").value("unit"))?,
            bench: cli.get(Arg::option("tb").value("unit"))?,
            plan: cli.get(Arg::option("plan").value("format"))?,
            target: cli.get(Arg::option("target"))?,
            target_dir: cli.get(Arg::option("target-dir"))?,
            command: cli.get(Arg::option("command").value("path"))?,
            filesets: cli.get_all(Arg::option("fileset").value("key=glob"))?,
            // Remaining args
            args: cli.remainder()?,
        })
    }

    fn execute(self, c: &Context) -> proc::Result {
        // locate the plugin
        let target = c.select_target(&self.target, self.list == false, false)?;

        // display plugin list and exit
        if self.list == true {
            match target {
                // display entire contents about the particular plugin
                Some(tar) => println!("{}", tar.to_string()),
                // display quick overview of all plugins
                None => println!(
                    "{}",
                    Target::list_targets(
                        &mut c
                            .get_config()
                            .get_targets()
                            .values()
                            .into_iter()
                            .collect::<Vec<&&Target>>()
                    )
                ),
            }
            return Ok(());
        }

        let target = target.unwrap();

        // coordinate the plan
        let plan = target.coordinate_plan(&self.plan)?;

        // check that user is in an IP directory
        c.jump_to_working_ip()?;

        // create the ip manifest
        let ip = Ip::load(c.get_ip_path().unwrap().clone(), true)?;

        // @todo: recreate the ip graph from the lockfile, then read each installation
        // see Install::install_from_lock_file

        // determine the build directory (command-line arg overrides configuration setting)
        let default_build_dir = c.get_target_dir();
        let target_dir = match &self.target_dir {
            Some(dir) => dir,
            None => &default_build_dir,
        };

        // gather the catalog and resolve any missing dependencies
        let catalog = Catalog::new()
            .installations(c.get_cache_path())?
            .downloads(c.get_downloads_path())?;
        let catalog = plan::resolve_missing_deps(c, &ip, catalog, self.force)?;

        self.run(
            &ip,
            target_dir,
            target,
            catalog,
            &c.get_languages(),
            &c,
            &plan,
        )
    }
}

impl Test {
    fn run(
        &self,
        working_ip: &Ip,
        target_dir: &str,
        target: &Target,
        catalog: Catalog,
        mode: &Language,
        c: &Context,
        scheme: &Scheme,
    ) -> Result<(), Fault> {
        // plan the target
        let blueprint_name = Plan::run(
            &working_ip,
            target_dir,
            target,
            catalog,
            mode,
            self.dirty == false,
            self.force,
            false,
            self.all,
            &self.bench,
            &self.dut,
            &self.filesets,
            &scheme,
            true,
            true,
        )?
        .unwrap_or_default();

        let output_path = working_ip
            .get_root()
            .join(target_dir)
            .join(&target.get_name());

        // prepare for build
        let envs = Environment::new()
            // read config.toml for setting any env variables
            .from_config(c.get_config())?
            // read ip manifest for env variables
            .from_ip(&working_ip)?
            .add(EnvVar::new().key(ORBIT_BLUEPRINT).value(&blueprint_name))
            .add(
                EnvVar::new()
                    .key(ORBIT_OUTPUT_PATH)
                    .value(&filesystem::into_std_str(output_path.clone())),
            )
            .add(EnvVar::new().key(ORBIT_TARGET_DIR).value(target_dir))
            .from_env_file(&output_path)?;

        // run the command from the output path
        match target.execute(
            &self.command,
            &self.args,
            self.verbose,
            &output_path,
            envs.into_map(),
        ) {
            Ok(()) => Ok(()),
            Err(e) => Err(Error::TargetProcFailed(LastError(e.to_string())))?,
        }
    }
}