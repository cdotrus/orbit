use crate::Command;
use crate::FromCli;
use crate::interface::cli::Cli;
use crate::interface::arg::{Flag, Optional};
use crate::interface::errors::CliError;
use crate::core::context::Context;
use crate::core::version::Version;

#[derive(Debug, PartialEq)]
enum VersionField {
    Major,
    Minor,
    Patch,
    Version(Version),
}

impl std::str::FromStr for VersionField {
    type Err = crate::core::version::VersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_ref() {
            "major" => Ok(Self::Major),
            "minor" => Ok(Self::Minor),
            "patch" => Ok(Self::Patch),
            _ => Ok(Self::Version(Version::from_str(s)?)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Launch {
    next: Option<VersionField>,
    ready: bool,
    message: Option<String>,
}

impl FromCli for Launch {
    fn from_cli<'c>(cli: &'c mut Cli) -> Result<Self,  CliError<'c>> {
        cli.set_help(HELP);
        let command = Ok(Launch {
            ready: cli.check_flag(Flag::new("ready"))?,
            next: cli.check_option(Optional::new("next").value("version"))?,
            message: cli.check_option(Optional::new("message"))?,
        });
        command
    }
}

use git2::Repository;
use crate::core::manifest::Manifest;
use std::str::FromStr;
use crate::util::anyerror::AnyError;

/// Retrieves the latest commit in the current repository using git2 API.
/// 
/// Source: https://zsiciarz.github.io/24daysofrust/book/vol2/day16.html
fn find_last_commit(repo: &Repository) -> Result<git2::Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(git2::ObjectType::Commit)?;
    obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
}

impl Command for Launch {
    type Err = Box<dyn std::error::Error>;
    fn exec(&self, c: &Context) -> Result<(), Self::Err> {
        // make sure it is run from an ip directory
        c.goto_ip_path()?;
        // verify the current directory is a git repository
        let repo = Repository::open(c.get_ip_path().unwrap())?;

        // verify the repository has at least one commit
        let latest_commit = find_last_commit(&repo)?;

        if self.message.is_some() && self.next.is_none() {
            return Err(CliError::BrokenRule(format!("option --message is only allowed when using option --next")))?
        }

        // @TODO verify the repository is up-to-date (git remote update, git remote fetch?)

        // grab the version defined in the manifest
        let mut manifest = Manifest::load(c.get_ip_path().unwrap().to_path_buf().join("Orbit.toml"))?;
        let prev_version = Version::from_str(manifest.get_doc()["ip"]["version"].as_str().unwrap()).unwrap();

        // at this point it is safe to assume it is a version because manifest will check that
        let mut version = prev_version.clone();
        
        // println!("already defined version: {}", version);
        // check if we applied --next
        let overwrite = if let Some(ver) = &self.next {
            match ver {
                VersionField::Major => version.inc_major(),
                VersionField::Minor => version.inc_minor(),
                VersionField::Patch => version.inc_patch(),
                VersionField::Version(v) => {
                    version = version.major(v.get_major())
                        .minor(v.get_minor())
                        .patch(v.get_patch());
                    // verify version will be larger than the current version
                    if prev_version >= version {
                        return Err(AnyError(format!("set version {} is not greater than current version {}", version, prev_version)))?
                    }
                }
            }
            println!("info: raising {} --> {}", prev_version, version);
            // update the manifest and add a new commit to the git repository
            manifest.get_mut_doc()["ip"]["version"] = toml_edit::value(version.to_string());
            true
        } else {
            println!("info: setting {}", version);
            false
        };
        
        // @TODO report if there are unsaved changes in the working directory/staging index?

        let ver_str = version.to_string();
        {
            // check if a tag exists for this version
            let tags = repo.tag_names(Some("[0-9]*.[0-9]*.[0-9]*"))?;
            let result = tags.iter()
                .filter_map(|f| f )
                .find(|f| { f == &ver_str });
            
            // the version already exists under a tag
            if let Some(r) = result {
                return Err(AnyError(format!("version \'{}\' is already released", r)))?;
            }
        }
        
        // verify the manifest is committed (not in staging or working directory if not overwriting)
        if overwrite == false {
            let st = repo.status_file(&std::path::PathBuf::from("Orbit.toml"))?;
            if st.is_empty() {
                println!("info: manifest is in clean state")
            } else {
                return Err(AnyError(format!("manifest Orbit.toml is dirty; move changes out of working directory or staging index to enter a clean state")))?
            }
        }

        let message = match &self.message {
            Some(m) => m.to_owned(),
            None => format!("releases version {}", version),
        };

        println!("info: create new commit ... {}", match overwrite {
            true => "yes",
            false => "no",
        });
       
        if overwrite == true {
            println!("info: future commit message \"{}\"", message)
        }

        // verify git things

        // verify Orbit.toml to staging area
        let mut index = repo.index()?;
        index.add_path(&std::path::PathBuf::from("Orbit.toml"))?;

        // verify a signature exists
        let signature = repo.signature()?;

        // tag if ready
        if self.ready == true {
            let marked_commit = if overwrite == true {
                // save the manifest
                manifest.save()?;
                // add manifest to staging
                index.add_path(&std::path::PathBuf::from("Orbit.toml"))?;
                // source: https://github.com/rust-lang/git2-rs/issues/561
                index.write()?;
                // create new commit
                let oid = index.write_tree().unwrap();
                let tree = repo.find_tree(oid)?;
                repo.commit(Some("HEAD"),
                    &signature,
                    &signature,
                    &message,
                    &tree,
                    &[&latest_commit])?;
                // update latest commit to attach with tag
                find_last_commit(&repo)?
            } else {
                latest_commit
            };

            // update the HEAD reference
            repo.tag_lightweight(&ver_str, &marked_commit.as_object(), false)?;
            println!("info: released version {}", version);
        } else {
            println!("info: version {} is ready for launch\n\nhint: include '--ready' flag to proceed", ver_str);
        }

        self.run()
    }
}

impl Launch {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

const HELP: &str = "\
Releases (tags) the current ip's latest commit as the next version.

Usage:
    orbit launch [options]

Options:
    --ready              proceed with the launch process
    --next <version>     semver version or 'major', 'minor', or 'patch'
    --message <message>  message to apply to the commit when using '--next'

Use 'orbit help launch' to learn more about the command.
";