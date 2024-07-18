//! A target is a user-defined backend workflow for processing the files collected
//! in the generated blueprint file.

use crate::core::context::Context;
use crate::core::fileset::Fileset;
use crate::core::fileset::Style;
use crate::error::Error;
use crate::util::anyerror::Fault;
use crate::util::filesystem;
use crate::util::filesystem::Standardize;
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use super::blueprint::Scheme;

pub type Targets = Vec<Target>;

type Filesets = HashMap<String, Style>;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Target {
    name: String,
    command: String,
    args: Option<Vec<String>>,
    fileset: Option<Filesets>,
    description: Option<String>,
    plans: Option<Vec<Scheme>>,
    explanation: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    root: Option<PathBuf>,
}

impl Target {
    pub fn get_filesets(&self) -> Option<&Filesets> {
        self.fileset.as_ref()
    }

    pub fn coordinate_plan(&self, plan: &Option<Scheme>) -> Result<Scheme, Error> {
        match plan {
            Some(p) => {
                // verify the plan is supported by the target
                match &self.plans {
                    // plans are listed for the target
                    Some(ps) => match ps.into_iter().find(|&i| i == p).is_some() {
                        true => Ok(p.clone()),
                        false => Err(Error::BlueprintPlanNotSupported(p.clone(), ps.clone())),
                    },
                    // no plans are listed for the target
                    None => match p == &Scheme::default() {
                        true => Ok(p.clone()),
                        false => Err(Error::BlueprintPlanMustBeDefault(
                            p.clone(),
                            Scheme::default(),
                        )),
                    },
                }
            }
            None => {
                match &self.plans {
                    // choose the first plan in the list
                    Some(ps) => match ps.first() {
                        Some(item) => Ok(item.clone()),
                        None => Ok(Scheme::default()),
                    },
                    // choose the default plan
                    None => Ok(Scheme::default()),
                }
            }
        }
    }

    /// Displays a plugin's information in a single line for quick glance.
    pub fn quick_info(&self) -> String {
        format!(
            "{:<16}{}",
            self.name,
            self.description.as_ref().unwrap_or(&String::new())
        )
    }

    pub fn explain(&self) -> String {
        self.explanation
            .as_ref()
            .unwrap_or(&String::default())
            .to_string()
    }

    /// Creates a string to display a list of plugins.
    ///
    /// The string lists the plugins in alphabetical order by `alias`.
    pub fn list_targets(targets: &mut [&&Target]) -> String {
        let mut list = String::new();
        targets.sort_by(|a, b| a.name.cmp(&b.name));
        for t in targets {
            list += &format!("{}\n", t.quick_info());
        }
        list
    }

    /// Sets the root directory from where the command should reference paths from.
    pub fn root(mut self, root: PathBuf) -> Self {
        self.root = Some(root);
        self
    }

    pub fn set_root(&mut self, root: PathBuf) {
        self.root = Some(root);
    }

    /// References the alias to call this plugin.
    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl std::fmt::Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\
Name:    {}
Command: {} {}
Root:    {}
Filesets:
{}{}{}",
            self.name,
            self.command,
            self.args
                .as_ref()
                .unwrap_or(&Vec::new())
                .iter()
                .fold(String::new(), |x, y| { x + "\"" + &y + "\" " }),
            PathBuf::standardize(self.root.as_ref().unwrap()).display(),
            {
                if self.fileset.is_none() {
                    String::from("  None\n")
                } else {
                    self.fileset
                        .as_ref()
                        .unwrap()
                        .iter()
                        .fold(String::new(), |x, (n, p)| {
                            x + &format!("  {:<16}{}\n", Fileset::standardize_name(n), p.inner())
                        })
                }
            },
            {
                if let Some(text) = &self.description {
                    format!("\n{}\n", text)
                } else {
                    String::new()
                }
            },
            {
                if let Some(text) = &self.explanation {
                    format!("\n{}", text)
                } else {
                    String::new()
                }
            },
        )
    }
}

impl FromStr for Target {
    type Err = toml::de::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        toml::from_str(s)
    }
}

pub trait Process {
    fn get_root(&self) -> &PathBuf;

    fn get_command(&self) -> &String;

    fn get_args(&self) -> Vec<&String>;

    /// Runs the given `command` with the set `args` for the plugin.
    fn execute(
        &self,
        overloaded_command: &Option<String>,
        extra_args: &[String],
        verbose: bool,
        cwd: &PathBuf,
        envs: HashMap<String, String>,
    ) -> Result<(), Fault> {
        // resolve the relative paths in the command and arguments defined in original configuration
        let command = match overloaded_command {
            Some(c) => c,
            None => self.get_command(),
        };

        let root_path = self.get_root();

        let command = filesystem::resolve_rel_path(root_path, command);

        let arguments: Vec<String> = self
            .get_args()
            .iter()
            .map(|f| filesystem::resolve_rel_path(root_path, f))
            .collect();

        // append args set on the command-line to the base-line of arguments
        let args = [&arguments, extra_args].concat();
        // display the literal command being ran
        if verbose == true {
            let s = args
                .iter()
                .fold(String::new(), |x, y| x + "\"" + &y + "\" ");
            println!("info: running: {} {}", command, s);
        }
        let mut proc = filesystem::invoke(
            cwd,
            &command,
            &args,
            Context::enable_windows_bat_file_match(),
            envs,
        )?;
        let exit_code = proc.wait()?;
        match exit_code.code() {
            Some(num) => {
                if num != 0 {
                    Err(Error::ChildProcErrorCode(num))?
                } else {
                    Ok(())
                }
            }
            None => Err(Error::ChildProcTerminated)?,
        }
    }
}

impl Process for Target {
    fn get_root(&self) -> &PathBuf {
        &self.root.as_ref().unwrap()
    }

    fn get_args(&self) -> Vec<&String> {
        match &self.args {
            Some(list) => list.iter().map(|e| e).collect(),
            None => Vec::new(),
        }
    }

    fn get_command(&self) -> &String {
        &self.command
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    pub struct Plugins {
        plugin: Vec<Target>,
    }

    impl Plugins {
        pub fn new() -> Self {
            Self { plugin: Vec::new() }
        }
    }

    impl FromStr for Plugins {
        type Err = toml::de::Error;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            toml::from_str(s)
        }
    }

    const P_1: &str = r#" 
name = "ghdl"
description = "Backend script for simulating VHDL with GHDL."  
command = "python"
args = ["./scripts/ghdl.py"]
fileset.py-model = "{{orbit.bench}}.py"
fileset.text = "*.txt"
"#;

    const P_2: &str = r#"
name = "ffi"
command = "bash"
args = ["~/scripts/download.bash"]    
"#;

    #[test]
    fn from_toml_string() {
        let plug = Target::from_str(P_1).unwrap();
        assert_eq!(
            plug,
            Target {
                name: String::from("ghdl"),
                command: String::from("python"),
                plans: None,
                args: Some(vec![String::from("./scripts/ghdl.py")]),
                description: Some(String::from(
                    "Backend script for simulating VHDL with GHDL."
                )),
                fileset: Some(HashMap::from([
                    (
                        String::from("py-model"),
                        Style::from_str("{{orbit.bench}}.py").unwrap()
                    ),
                    (String::from("text"), Style::from_str("*.txt").unwrap()),
                ])),
                explanation: None,
                root: None,
            }
        );

        let plug = Target::from_str(P_2).unwrap();
        assert_eq!(
            plug,
            Target {
                name: String::from("ffi"),
                command: String::from("bash"),
                args: Some(vec![String::from("~/scripts/download.bash")]),
                description: None,
                plans: None,
                fileset: None,
                explanation: None,
                root: None,
            }
        );
    }

    #[test]
    fn series_of_plugins() {
        let contents = format!("{0}{1}\n{0}{2}", "[[plugin]]", P_1, P_2);
        // assemble the list of protocols
        let plugs = Plugins::from_str(&contents).unwrap();
        assert_eq!(
            plugs,
            Plugins {
                plugin: vec![
                    Target::from_str(P_1).unwrap(),
                    Target::from_str(P_2).unwrap()
                ],
            }
        );
    }
}