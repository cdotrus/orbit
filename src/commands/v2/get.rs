use std::collections::HashMap;
use std::path::PathBuf;

use colored::Colorize;
use clif::cmd::{FromCli, Command};
use crate::core::v2::manifest::FromFile;
use crate::core::lang::parser::Symbol;
use crate::core::v2::manifest::IP_MANIFEST_FILE;
use crate::core::v2::manifest::Manifest;
use crate::core::version::AnyVersion;
use crate::core::version::Version;
use crate::core::lang::vhdl::interface;
use crate::core::lang::vhdl::primaryunit::VhdlIdentifierError;
use crate::core::lang::vhdl::symbol::Architecture;
use crate::core::lang::vhdl::symbol::Entity;
use clif::Cli;
use clif::arg::{Positional, Flag, Optional};
use clif::Error as CliError;
use crate::core::context::Context;
use crate::core::lang::vhdl::token::Identifier;
use crate::util::anyerror::{AnyError, Fault};
use crate::core::pkgid::PkgPart;
use crate::OrbitResult;

#[derive(Debug, PartialEq)]
pub struct Get {
    unit: Identifier,
    ip: Option<PkgPart>,
    signals: bool,
    component: bool,
    instance: bool,
    architectures: bool,
    version: Option<AnyVersion>,
    info: bool,
    name: Option<Identifier>,
}

impl FromCli for Get {
    fn from_cli<'c>(cli: &'c mut Cli) -> Result<Self,  CliError> {
        cli.check_help(clif::Help::new().quick_text(HELP).ref_usage(2..4))?;
        let command = Ok(Self {
            signals: cli.check_flag(Flag::new("signals").switch('s'))?,
            component: cli.check_flag(Flag::new("component").switch('c'))?,
            instance: cli.check_flag(Flag::new("instance").switch('i'))?,
            architectures: cli.check_flag(Flag::new("architecture").switch('a'))?,
            version: cli.check_option(Optional::new("variant").switch('v').value("version"))?,
            info: cli.check_flag(Flag::new("info"))?, // @todo: implement
            ip: cli.check_option(Optional::new("ip").value("name"))?,
            name: cli.check_option(Optional::new("name").value("identifier"))?,
            unit: cli.require_positional(Positional::new("unit"))?,
        });
        command
    }
}

use crate::core::lang::parser::Parse;
use crate::core::lang::vhdl;
use crate::core::lang::vhdl::symbol;
use crate::core::lang::vhdl::token::VHDLTokenizer;
use std::env;

impl Command<Context> for Get {
    type Status = OrbitResult;

    fn exec(&self, _c: &Context) -> Self::Status {
        // --name can only be used with --instance is set
        if self.name.is_some() && self.instance == false {
            return Err(AnyError(format!("'{}' can only be used with '{}'", "--name".yellow(), "--instance".yellow())))?
        }
        // @todo: check --version can only be used with --ip (or change --ip option to tack on a version)

        // try to auto-determine the ip (check if in a working ip)
        let ip_path = if let Some(name) = &self.ip {
            // @todo: find the path to the provided ip by searching through the catalog
            todo!("find ip {} in the catalog", name)
        } else {
            let ip = Context::find_ip_path(&env::current_dir().unwrap());  
            if ip.is_none() == true {
                return Err(AnyError(format!("no ip provided or detected")))?
            } else {
                ip.unwrap()
            }
        };

        // load the manifest from the path
        let man = Manifest::from_file(&ip_path.join(IP_MANIFEST_FILE))?;

        self.run(man, &ip_path)
    }
}

impl Get {
    fn run(&self, man: Manifest, dir: &PathBuf) -> Result<(), Fault> {
        // collect all hdl files and parse them
        let ent = match Self::fetch_entity(&self.unit, &dir, &man) {
            Ok(r) => r,
            Err(e) => return Err(GetError::SuggestShow(e.to_string(), man.get_ip().get_name().clone(), man.get_ip().get_version().clone()))?
        };

        // add to dependency list if within a ip and `self.add` is `true`
        // if let Some(mut cur_ip) = current_ip {
        //     // verify it is the not the same package! and we explicitly want to add 
        //     if cur_ip.get_pkgid() != ip.get_pkgid() && self.add == true {
        //         cur_ip.insert_dependency(ip.get_pkgid().clone(), self.version.as_ref().unwrap_or(&AnyVersion::Latest).clone());
        //         cur_ip.get_manifest_mut().save()?;
        //     }
        // }

        // make the library reference the current working ip 'work' if its internal
        let lib = match self.ip.is_none() {
            true => Identifier::new_working(),
            false => match man.get_ip().get_library() {
                Some(lib) => Identifier::from(lib),
                None => Identifier::new_working(),
            }
        };
        
        // display architectures    
        if self.architectures == true {
            println!("{}", ent.get_architectures());
        }

        // display component declaration
        if self.component == true {
            println!("{}", ent.into_component());
        // display library declaration line if displaying instance
        } else if self.instance == true {
            println!("{}", interface::library_statement(&lib));
        }

        // display signal declarations
        if self.signals == true {
            let constants = ent.into_constants();
            if constants.is_empty() == false {
                println!("{}", constants);
            }
            let signals = ent.into_signals();
            if signals.is_empty() == false {
                println!("{}", signals);
            }
        }  

        // only display the direct entity instantiation code if not providing component code
        let lib = if self.component == true { None } else { Some(lib) };

        // display instantiation code
        if self.instance == true {
            let name = match &self.name {
                Some(iden) => iden.clone(),
                None => Identifier::Basic("uX".to_string()),
            };
            println!("{}", ent.into_instance(&name, lib));
        }

        Ok(())
    }

    /// Parses through the vhdl files and returns a desired entity struct.
    fn fetch_entity(iden: &Identifier, dir: &PathBuf, man: &Manifest) -> Result<symbol::Entity, Fault> {
        let files = crate::util::filesystem::gather_current_files(&dir, false);
        // @todo: generate all units first (store architectures, and entities, and then process)
        let mut result: Option<(String, Entity)> = None;
        // store map of all architectures while parsing all code
        let mut architectures: HashMap<Identifier, Vec<Architecture>> = HashMap::new();
        for f in files {
            // lex and parse
            if crate::core::fileset::is_vhdl(&f) == true {
                let text = std::fs::read_to_string(&f)?;
            
                // pull all architectures
                let units: Vec<Symbol<symbol::VHDLSymbol>> = vhdl::symbol::VHDLParser::parse(VHDLTokenizer::from_source_code(&text).into_tokens())
                    .into_iter()
                    .filter_map(|f| if f.is_ok() { 
                        let unit = f.unwrap();
                        match unit.as_ref().as_architecture() {
                            Some(_) => {
                                let arch = unit.take().into_architecture().unwrap();
                                match architectures.get_mut(arch.entity()) {
                                    Some(list) => { list.push(arch); () },
                                    None => { architectures.insert(arch.entity().clone(), vec![arch]); () }
                                }
                                None 
                            },
                            None => Some(unit)
                        }
                    } else { None }
                    ).collect();

                // detect entity
                let requested_entity = units
                    .into_iter() 
                    .filter_map(|r| r.take().into_entity())
                    .find(|p| p.get_name() == iden);

                // verify entity was not already detected (duplicate)
                if let Some(ent) = requested_entity {
                    match result {
                        Some((src_file, dupe)) => return Err(VhdlIdentifierError::DuplicateIdentifier(dupe.get_name().clone(), PathBuf::from(src_file), dupe.get_position().clone(), PathBuf::from(f), ent.get_position().clone()))?,
                        None => result = Some((f, ent)),
                    }
                }
            }
        }
        match result {
            Some((_, mut entity)) => {
                match architectures.remove(entity.get_name()) {
                    Some(archs) => for arch in archs { entity.link_architecture(arch) }
                    None => (),
                }
                Ok(entity)
            }
            None => Err(GetError::EntityNotFound(iden.clone(), man.get_ip().get_name().clone(), man.get_ip().get_version().clone()))?
        }
    }
}

#[derive(Debug)]
pub enum GetError {
    EntityNotFound(Identifier, PkgPart, Version),
    SuggestShow(String, PkgPart, Version),
}

impl std::error::Error for GetError {}

impl std::fmt::Display for GetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EntityNotFound(ent, pkg, ver) => {
                write!(f, "entity '{0}' is not found in ip '{1}' under version '{2}'", ent, pkg, ver)
            },
            Self::SuggestShow(err, pkg, ver) => {
                write!(f, "{}\n\nTry `orbit show {1} -v {2} --units` to see a list of primary design units", err, pkg, ver)
            },
        }
    }
}

const HELP: &str = "\
Fetch an hdl entity for code integration.

Usage:
    orbit get [options] <unit>

Args:
    <unit>                  entity identifier

Options:
    --ip <pkgid>            ip to reference unit from
    --variant, -v <version> ip version to use
    --component, -c         print component declaration
    --signals,   -s         print signal declarations
    --instance,  -i         print instantation
    --info                  access code file's header comment
    --architecture, -a      print available architectures
    --add                   add the ip to the Orbit.toml dependency table
    --name <identifier>     specific instance identifier

Use 'orbit help get' to learn more about the command.
";


// #[cfg(test)]
// mod test {
//     // use super::*;

//     // use std::str::FromStr;

//     // #[test]
//     // #[ignore]
//     // fn fetch_entity() {
//     //     let _ = Get::fetch_entity(&Identifier::from_str("or_gate").unwrap(), &std::path::PathBuf::from("./test/data/gates")).unwrap();
//     //     panic!("inspect")
//     // }
// }