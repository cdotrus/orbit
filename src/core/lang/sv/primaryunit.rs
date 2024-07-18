use super::{symbols::SystemVerilogSymbol, token::identifier::Identifier};
use crate::{core::lang::sv::symbols::SystemVerilogParser, util::anyerror::CodeFault};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(PartialEq, Hash, Eq, Debug)]
pub enum PrimaryShape {
    Module,
    Config,
    Package,
    Interface,
}

#[derive(PartialEq, Hash, Eq, Debug)]
pub struct PrimaryUnit {
    shape: PrimaryShape,
    unit: Unit,
}

impl PrimaryUnit {
    pub fn get_name(&self) -> &Identifier {
        &self.unit.name
    }

    pub fn get_unit(&self) -> &Unit {
        &self.unit
    }

    /// Deserializes the data from a toml inline table.
    pub fn from_toml(tbl: &toml_edit::InlineTable) -> Option<Self> {
        let unit = Unit {
            name: Identifier::from_str(tbl.get("identifier")?.as_str()?).unwrap(),
            symbol: None,
            source: String::new(),
        };
        let shape = match tbl.get("type")?.as_str()? {
            "module" => PrimaryShape::Module,
            "config" => PrimaryShape::Config,
            "package" => PrimaryShape::Package,
            "interface" => PrimaryShape::Interface,
            _ => return None,
        };
        Some(Self {
            shape: shape,
            unit: unit,
        })
    }
}

impl std::fmt::Display for PrimaryUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.shape {
                PrimaryShape::Config => "config",
                PrimaryShape::Module => "module",
                PrimaryShape::Package => "package",
                PrimaryShape::Interface => "interface",
            }
        )
    }
}

#[derive(Debug)]
pub struct Unit {
    name: Identifier,
    symbol: Option<SystemVerilogSymbol>,
    /// source code file
    source: String,
}

impl Unit {
    pub fn get_symbol(&self) -> Option<&SystemVerilogSymbol> {
        self.symbol.as_ref()
    }

    pub fn get_symbol_mut(&mut self) -> Option<&mut SystemVerilogSymbol> {
        self.symbol.as_mut()
    }

    pub fn get_source_file(&self) -> &str {
        &self.source
    }

    pub fn is_usable_component(&self) -> Option<()> {
        match self.get_symbol()?.as_module()?.is_testbench() {
            true => None,
            false => Some(()),
        }
    }
}

impl std::hash::Hash for Unit {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Unit {}

pub fn collect_units(files: &Vec<String>) -> Result<HashMap<Identifier, PrimaryUnit>, CodeFault> {
    let mut result: HashMap<Identifier, PrimaryUnit> = HashMap::new();
    // iterate through all source files
    for source_file in files {
        // only read the HDL files
        if crate::core::fileset::is_systemverilog(&source_file) == true {
            // parse text into Verilog symbols
            let contents = match std::fs::read_to_string(&source_file) {
                Ok(dump) => dump,
                Err(e) => return Err(CodeFault(Some(source_file.clone()), Box::new(e))),
            };

            let symbols = match SystemVerilogParser::read(&contents) {
                Ok(s) => s.into_symbols(),
                Err(e) => Err(CodeFault(Some(source_file.clone()), Box::new(e)))?,
            };

            // transform into primary design units
            let units: HashMap<Identifier, PrimaryUnit> = symbols
                .into_iter()
                .filter_map(|sym: SystemVerilogSymbol| {
                    let name = sym.as_name();
                    match sym {
                        SystemVerilogSymbol::Module(_) => Some((
                            name.unwrap().clone(),
                            PrimaryUnit {
                                shape: PrimaryShape::Module,
                                unit: Unit {
                                    name: name.unwrap().clone(),
                                    symbol: Some(sym),
                                    source: source_file.clone(),
                                },
                            },
                        )),
                        SystemVerilogSymbol::Config(_) => Some((
                            name.unwrap().clone(),
                            PrimaryUnit {
                                shape: PrimaryShape::Config,
                                unit: Unit {
                                    name: name.unwrap().clone(),
                                    symbol: Some(sym),
                                    source: source_file.clone(),
                                },
                            },
                        )),
                        SystemVerilogSymbol::Package(_) => Some((
                            name.unwrap().clone(),
                            PrimaryUnit {
                                shape: PrimaryShape::Package,
                                unit: Unit {
                                    name: name.unwrap().clone(),
                                    symbol: Some(sym),
                                    source: source_file.clone(),
                                },
                            },
                        )),
                        SystemVerilogSymbol::Interface(_) => Some((
                            name.unwrap().clone(),
                            PrimaryUnit {
                                shape: PrimaryShape::Interface,
                                unit: Unit {
                                    name: name.unwrap().clone(),
                                    symbol: Some(sym),
                                    source: source_file.clone(),
                                },
                            },
                        )),
                    }
                })
                .collect();

            // push to the global list
            result.extend(units);
        }
    }
    Ok(result)
}