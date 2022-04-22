use crate::Command;
use crate::FromCli;
use crate::interface::cli::Cli;
use crate::interface::arg::{Positional};
use crate::interface::errors::CliError;
use crate::core::context::Context;

#[derive(Debug, PartialEq)]
pub struct Help {
    topic: Option<String>,
}

impl Command for Help {
    type Err = Box<dyn std::error::Error>;
    fn exec(&self, _: &Context) -> Result<(), Self::Err> {
        Ok(self.run())
    }
}

impl Help {
    fn run(&self) -> () {
        if let Some(t) = &self.topic {
            if t == "new" {
                println!("{}", crate::commands::manuals::new::MANUAL);
            }
        } else {
            println!("info: displaying help text");
        }
    }
}

impl FromCli for Help {
    fn from_cli<'c>(cli: &'c mut Cli) -> Result<Self,  CliError<'c>> {
        cli.set_help(HELP);
        let command = Ok(Help {
            topic: cli.check_positional(Positional::new("topic"))?,
        });
        command
    }
}

const HELP: &str = "\
Read in-depth documentation around Orbit topics.

Usage:
    orbit help [<topic>]

Args:
    <topic>         a listed topic or any orbit subcommand

Topics:
    cfg             learn about .cfg files
    cache           learn about orbit's caching system
    manifest        learn about the orbit.cfg file
    template        learn about templates
    blueprint       learn about generating a pre-build data file

Use 'orbit help --list' to see all available topics.
";