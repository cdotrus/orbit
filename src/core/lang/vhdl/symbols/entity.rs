use std::{collections::LinkedList, iter::Peekable};

use serde_derive::Serialize;

use crate::core::lang::vhdl::{error::VhdlError, format::VhdlFormat};

use super::{
    architecture::Architecture, color, Architectures, Delimiter, Generics, Identifier,
    IdentifierList, InterfaceDeclarations, Keyword, Ports, Position, ToColor, Token, VhdlSymbol,
    VhdlToken, ENTITY_NAME,
};

#[derive(Debug, PartialEq, Serialize)]
pub struct Entity {
    #[serde(rename = "entity")]
    name: Identifier,
    generics: Generics,
    ports: Ports,
    architectures: Vec<Architecture>,
    #[serde(skip_serializing)]
    refs: IdentifierList,
    #[serde(skip_serializing)]
    pos: Position,
}

impl Entity {
    /// Returns a new blank `Entity` struct.
    pub fn new() -> Self {
        Self {
            name: Identifier::new(),
            ports: Ports::new(),
            generics: Generics::new(),
            architectures: Vec::new(),
            refs: LinkedList::new(),
            pos: Position::new(),
        }
    }

    /// Creates a basic entity from a `name`. Assumes no other information is
    /// available.
    pub fn black_box(name: Identifier) -> Self {
        Self {
            name: name,
            ports: Ports::new(),
            generics: Generics::new(),
            architectures: Vec::new(),
            refs: LinkedList::new(),
            pos: Position::new(),
        }
    }

    pub fn get_position(&self) -> &Position {
        &self.pos
    }

    /// Checks if the current `Entity` is a testbench.
    ///
    /// This is determined by checking if the ports list is empty.
    pub fn is_testbench(&self) -> bool {
        self.ports.is_empty()
    }

    /// Accesses the entity's identifier.
    pub fn get_name(&self) -> &Identifier {
        &self.name
    }

    /// Accesses the entity's generics.
    pub fn get_generics(&self) -> &Generics {
        &self.generics
    }

    /// Accesses the entity's ports.
    pub fn get_ports(&self) -> &Ports {
        &self.ports
    }

    /// References the references for the entity.
    pub fn get_refs(&self) -> &IdentifierList {
        &self.refs
    }

    // Generates VHDL component code from the entity.
    pub fn into_component(&self, fmt: &VhdlFormat) -> String {
        let mut result = format!("{} ", Keyword::Component.to_color());
        result.push_str(&format!(
            "{}",
            color(&self.get_name().to_string(), ENTITY_NAME)
        ));

        let interface_depth = match fmt.is_indented_interfaces() {
            true => 2,
            false => 1,
        };

        if self.generics.0.len() > 0 {
            result.push('\n');
            if fmt.is_indented_interfaces() == true && fmt.get_tab_size() > 0 {
                result.push_str(&format!(
                    "{:<width$}",
                    " ",
                    width = fmt.get_tab_size() as usize
                ));
            }
            result.push_str(&format!("{}", Keyword::Generic.to_color()));
            result.push_str(
                &self
                    .generics
                    .0
                    .to_interface_part_string(&fmt, interface_depth)
                    .to_string(),
            );
        }
        if self.ports.0.len() > 0 {
            result.push('\n');
            if fmt.is_indented_interfaces() == true && fmt.get_tab_size() > 0 {
                result.push_str(&format!(
                    "{:<width$}",
                    " ",
                    width = fmt.get_tab_size() as usize
                ));
            }
            result.push_str(&format!("{}", Keyword::Port.to_color()));
            result.push_str(
                &self
                    .ports
                    .0
                    .to_interface_part_string(&fmt, interface_depth)
                    .to_string(),
            );
        }
        result.push_str(&format!(
            "\n{} {}{}\n",
            Keyword::End.to_color(),
            Keyword::Component.to_color(),
            Delimiter::Terminator.to_color()
        ));
        result
    }

    /// Generates VHDL signal declaration code from the entity data.
    pub fn into_signals(&self, fmt: &VhdlFormat) -> String {
        self.ports
            .0
            .to_declaration_part_string(Keyword::Signal, &fmt)
            .to_string()
    }

    /// Generates VHDL constant declaration code from the entity data.
    pub fn into_constants(&self, fmt: &VhdlFormat) -> String {
        self.generics
            .0
            .to_declaration_part_string(Keyword::Constant, &fmt)
            .to_string()
    }

    /// Generates VHDL instantiation code from the entity data.
    pub fn into_instance(
        &self,
        inst: &Option<Identifier>,
        library: Option<Identifier>,
        fmt: &VhdlFormat,
    ) -> String {
        let prefix = match library {
            Some(lib) => format!(
                "{} {}{}",
                Keyword::Entity.to_color(),
                color(&lib.to_string(), ENTITY_NAME),
                Delimiter::Dot.to_color()
            ),
            None => String::new(),
        };

        let name = match &inst {
            Some(iden) => iden.clone(),
            None => Identifier::Basic(fmt.get_instance_name().to_string()),
        };

        let mapping_depth = match fmt.is_indented_interfaces() {
            true => 2,
            false => 1,
        };

        let mut result = String::new();

        result.push_str(&format!("{}", name.to_color()));
        if fmt.get_type_offset() > 0 {
            result.push_str(&format!(
                "{:<width$}",
                " ",
                width = fmt.get_type_offset() as usize
            ));
        }
        result.push_str(&format!(
            "{} {}{}",
            Delimiter::Colon.to_color(),
            prefix,
            color(&self.get_name().to_string(), ENTITY_NAME)
        ));
        if self.generics.0.len() > 0 {
            result.push('\n');
            if fmt.is_indented_interfaces() == true && fmt.get_tab_size() > 0 {
                result.push_str(&format!(
                    "{:<width$}",
                    " ",
                    width = fmt.get_tab_size() as usize
                ));
            }
            result.push_str(&(format!("{}", Keyword::Generic.to_color())));
            result.push_str(
                &self
                    .generics
                    .0
                    .to_instantiation_part(&fmt, mapping_depth)
                    .to_string(),
            )
        }
        if self.ports.0.len() > 0 {
            // add extra spacing
            result.push('\n');
            if fmt.is_indented_interfaces() == true && fmt.get_tab_size() > 0 {
                result.push_str(&format!(
                    "{:<width$}",
                    " ",
                    width = fmt.get_tab_size() as usize
                ));
            }
            result.push_str(&format!("{}", Keyword::Port.to_color()));
            result.push_str(
                &self
                    .ports
                    .0
                    .to_instantiation_part(&fmt, mapping_depth)
                    .to_string(),
            )
        }
        result.push_str(&Delimiter::Terminator.to_string());
        result
    }

    /// Generates list of available architectures.
    ///
    /// Note: This fn must be ran after linking entities and architectures in the
    /// current ip.
    pub fn get_architectures(&self) -> Architectures {
        Architectures::new(&self.architectures)
    }

    pub fn link_architecture(&mut self, arch: Architecture) -> () {
        self.architectures.push(arch);
    }

    /// Parses an `Entity` primary design unit from the entity's identifier to
    /// the END closing statement.
    pub fn from_tokens<I>(tokens: &mut Peekable<I>, pos: Position) -> Result<Self, VhdlError>
    where
        I: Iterator<Item = Token<VhdlToken>>,
    {
        // take entity name
        let entity_name = tokens.next().take().unwrap().take();
        let (generics, ports, entity_refs) = VhdlSymbol::parse_entity_declaration(tokens)?;

        let generics = generics
            .into_iter()
            .map(|f| f.0)
            .collect::<Vec<Vec<Token<VhdlToken>>>>();

        let ports = ports
            .into_iter()
            .map(|f| f.0)
            .collect::<Vec<Vec<Token<VhdlToken>>>>();

        Ok(Entity {
            name: match entity_name {
                VhdlToken::Identifier(id) => id,
                // expecting identifier
                _ => return Err(VhdlError::Vague),
            },
            architectures: Vec::new(),
            generics: Generics(InterfaceDeclarations::from_double_listed_tokens(generics)),
            ports: Ports(InterfaceDeclarations::from_double_listed_tokens(ports)),
            refs: entity_refs,
            pos: pos,
        })
    }

    /// Accesses the references as mutable for the entity.
    pub fn get_refs_mut(&mut self) -> &mut IdentifierList {
        &mut self.refs
    }
}