pub mod verilog;
pub mod vhdl;

pub mod lexer;
pub mod parser;

pub mod node;
pub mod unit;

use crate::util::anyerror::{AnyError, CodeFault};
use serde_derive::Serialize;
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;
use toml_edit::InlineTable;
use vhdl::primaryunit::PrimaryUnit;

type VhdlIdentifier = vhdl::token::Identifier;
use serde_derive::Deserialize;

use super::pubfile::{PublicList, Visibility};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct Languages {
    modes: Vec<Lang>,
}

impl Languages {
    pub fn with(langs: Vec<Lang>) -> Self {
        Self { modes: langs }
    }

    pub fn new() -> Self {
        Self { modes: Vec::new() }
    }

    pub fn push(&mut self, lang: Lang) {
        self.modes.push(lang);
    }

    pub fn pop(&mut self) -> bool {
        self.modes.pop().is_some()
    }

    pub fn supports_vhdl(&self) -> bool {
        self.modes.contains(&Lang::Vhdl)
    }

    pub fn supports_verilog(&self) -> bool {
        self.modes.contains(&Lang::Verilog)
    }
}

impl Default for Languages {
    fn default() -> Self {
        Self {
            modes: vec![Lang::Vhdl],
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub enum Lang {
    #[serde(rename = "vhdl")]
    Vhdl,
    #[serde(rename = "verilog")]
    Verilog,
}

impl FromStr for Lang {
    type Err = AnyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "vhdl" => Ok(Self::Vhdl),
            "verilog" => Ok(Self::Verilog),
            _ => Err(AnyError(format!("unsupported language {:?}", s))),
        }
    }
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Vhdl => "vhdl",
                Self::Verilog => "verilog",
            }
        )
    }
}

pub trait Code {
    fn get_source_code_file(&self) -> &str;
    fn get_symbol(&self) -> Option<&vhdl::symbols::VhdlSymbol>;
}

#[derive(Debug, PartialEq)]
pub struct SharedData {
    visibility: Visibility,
}

impl SharedData {
    pub fn new() -> Self {
        Self {
            visibility: Visibility::default(),
        }
    }

    pub fn set_visibility(&mut self, v: Visibility) {
        self.visibility = v;
    }

    pub fn get_visibility(&self) -> &Visibility {
        &self.visibility
    }
}

#[derive(Debug, PartialEq)]
pub enum LangUnit {
    Vhdl(PrimaryUnit, SharedData),
    Verilog(String, SharedData),
}

// impl Code for LangUnit {
//     fn get_source_code_file(&self) -> &str {
//         match &self {
//             Self::Vhdl(u) => u.get_unit().get_source_code_file(),
//             Self::Verilog(u) => u.as_ref(),
//         }
//     }

//     fn get_symbol(&self) -> Option<&vhdl::symbol::VHDLSymbol> {
//         match &self {
//             Self::Vhdl(u) => u.get_unit().get_symbol(),
//             Self::Verilog(u) => None,
//         }
//     }
// }

impl LangUnit {
    /// Checks if the module is public.
    pub fn is_listed_public(&self, plist: &PublicList) -> bool {
        plist.is_included(self.get_source_code_file())
    }

    pub fn get_visibility(&self) -> &Visibility {
        match &self {
            Self::Vhdl(_, sd) => sd.get_visibility(),
            Self::Verilog(_, sd) => sd.get_visibility(),
        }
    }

    pub fn set_visibility(&mut self, v: Visibility) {
        match self {
            Self::Vhdl(_, sd) => sd.set_visibility(v),
            Self::Verilog(_, sd) => sd.set_visibility(v),
        };
    }

    /// References the unit's identifier.
    pub fn get_name(&self) -> LangIdentifier {
        match &self {
            Self::Vhdl(u, _) => LangIdentifier::Vhdl(u.get_iden().clone()),
            Self::Verilog(u, _) => LangIdentifier::Verilog(u.clone()),
        }
    }

    /// Denotes the HDL language that is used for this unit.
    pub fn get_lang(&self) -> Lang {
        match &self {
            Self::Vhdl(_, _) => Lang::Vhdl,
            Self::Verilog(_, _) => Lang::Verilog,
        }
    }

    pub fn get_source_code_file(&self) -> &str {
        match &self {
            Self::Vhdl(u, _) => u.get_unit().get_source_code_file(),
            Self::Verilog(u, _) => u.as_ref(),
        }
    }

    pub fn get_symbol(&self) -> Option<&vhdl::symbols::VhdlSymbol> {
        match &self {
            Self::Vhdl(u, _) => u.get_unit().get_symbol(),
            Self::Verilog(_u, _) => None,
        }
    }

    pub fn get_references(&self) -> Vec<LangIdentifier> {
        match &self {
            Self::Vhdl(u, _) => match u.get_unit().get_symbol() {
                Some(sym) => sym
                    .get_refs()
                    .into_iter()
                    .map(|f| LangIdentifier::Vhdl(f.get_suffix().clone()))
                    .collect(),
                None => Vec::new(),
            },
            Self::Verilog(_u, _) => Vec::new(),
        }
    }

    /// Serializes the data into a toml inline table
    pub fn to_toml(&self) -> toml_edit::Value {
        let mut item = toml_edit::Value::InlineTable(InlineTable::new());
        let tbl = item.as_inline_table_mut().unwrap();
        tbl.insert(
            "language",
            toml_edit::value(&self.get_lang().to_string())
                .into_value()
                .unwrap(),
        );
        tbl.insert(
            "identifier",
            toml_edit::value(&self.get_name().to_string())
                .into_value()
                .unwrap(),
        );
        tbl.insert(
            "type",
            toml_edit::value(&self.to_string()).into_value().unwrap(),
        );
        item
    }

    /// Deserializes the data from a toml inline table.
    pub fn from_toml(tbl: &toml_edit::InlineTable) -> Option<Self> {
        let entry = tbl.get("language")?.as_str()?;
        match entry {
            "vhdl" => Some(Self::Vhdl(PrimaryUnit::from_toml(tbl)?, SharedData::new())),
            "verilog" => Some(Self::Verilog(String::new(), SharedData::new())),
            _ => panic!("unknown entry in serialized toml table {}", entry),
        }
    }
}

impl FromStr for LangIdentifier {
    type Err = vhdl::token::identifier::IdentifierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::Vhdl(VhdlIdentifier::from_str(&s)?))
    }
}

impl Display for LangUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Vhdl(u, _) => write!(f, "{}", u),
            Self::Verilog(u, _) => write!(f, "{}", u),
        }
    }
}

#[derive(Debug, PartialEq, Hash, Eq, Clone, PartialOrd, Ord)]
pub enum LangIdentifier {
    Vhdl(VhdlIdentifier),
    Verilog(String),
}

impl LangIdentifier {
    pub fn as_vhdl_id(&self) -> Option<&VhdlIdentifier> {
        match &self {
            Self::Vhdl(name) => Some(name),
            _ => None,
        }
    }
}

impl Display for LangIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::Vhdl(u) => write!(f, "{}", u),
            Self::Verilog(u) => write!(f, "{}", u),
        }
    }
}

pub fn collect_units(
    files: &Vec<String>,
    lang_mode: &Languages,
) -> Result<HashMap<LangIdentifier, LangUnit>, CodeFault> {
    // collect the VHDL units
    let vhdl_units = match lang_mode.supports_vhdl() {
        true => vhdl::primaryunit::collect_units(&files)?,
        false => HashMap::new(),
    };

    // collect the Verilog units
    let verilog_units = match lang_mode.supports_verilog() {
        true => verilog::primaryunit::collect_units(&files)?,
        false => HashMap::new(),
    };

    // merge the two results into a common struct
    let mut results = HashMap::with_capacity(vhdl_units.len() + verilog_units.len());
    for (k, v) in vhdl_units {
        results.insert(
            LangIdentifier::Vhdl(k),
            LangUnit::Vhdl(v, SharedData::new()),
        );
    }
    for (k, v) in verilog_units {
        results.insert(
            LangIdentifier::Verilog(k),
            LangUnit::Verilog(v, SharedData::new()),
        );
    }

    Ok(results)
}
