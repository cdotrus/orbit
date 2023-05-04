use std::collections::LinkedList;
use std::fmt::Display;
use std::hash::Hash;

use super::super::parser::*;
use super::super::lexer::*;

use crate::core::lang::vhdl::token::*;
use crate::core::lang::vhdl::interface::*;

pub type IdentifierList = LinkedList<CompoundIdentifier>;

#[derive(Debug, PartialEq)]
pub enum VHDLSymbol {
    // primary design units
    Entity(Entity),
    Context(Context),
    Package(Package),
    Configuration(Configuration),
    // secondary design units
    Architecture(Architecture),
    PackageBody(PackageBody),
}

impl VHDLSymbol {
    /// Casts `self` to identifier.
    pub fn as_iden(&self) -> Option<&Identifier> {
        match self {
            Self::Entity(e) => Some(&e.name),
            Self::Architecture(a) => Some(&a.name),
            Self::Package(p) => Some(&p.name),
            Self::PackageBody(_) => None,
            Self::Configuration(c) => Some(&c.name),
            Self::Context(c) => Some(&c.name),
        }
    }

    /// References the starting location in the file for the primary design unit.
    pub fn get_position(&self) -> &Position {
        match self {
            Self::Entity(unit) => unit.get_position(),
            Self::Architecture(unit) => unit.get_position(),
            Self::Package(unit) => unit.get_position(),
            Self::PackageBody(unit) => unit.get_position(),
            Self::Configuration(unit) => unit.get_position(),
            Self::Context(unit) => unit.get_position(),
        }
    }

    /// Casts `self` to package.
    pub fn as_package(&self) -> Option<&Package> {
        match self {
            Self::Package(p) => Some(p),
            _ => None,
        }
    }

    /// Casts `self` to entity.
    pub fn as_entity(&self) -> Option<&Entity> {
        match self {
            Self::Entity(e) => Some(e),
            _ => None
        }
    }

    /// Casts `self` to configuration.
    pub fn as_configuration(&self) -> Option<&Configuration> {
        match self {
            Self::Configuration(cfg) => Some(cfg),
            _ => None,
        }
    }

    /// Transforms `self` to entity.
    pub fn into_entity(self) -> Option<Entity> {
        match self {
            Self::Entity(e) => Some(e),
            _ => None
        }
    }

    /// Transforms `self` into architecture.
    pub fn into_architecture(self) -> Option<Architecture> {
        match self {
            Self::Architecture(arch) => Some(arch),
            _ => None,
        }
    }

    /// Casts `self` to architecture.
    pub fn as_architecture(&self) -> Option<&Architecture> {
        match self {
            Self::Architecture(arch) => Some(arch),
            _ => None,
        }
    }

    pub fn add_refs(&mut self, refs: &mut IdentifierList) {
        match self {
            Self::Entity(e) => e.refs.append(refs),
            Self::Architecture(a) => a.refs.append(refs),
            Self::Package(p) => p.refs.append(refs),
            Self::PackageBody(pb) => pb.refs.append(refs),
            Self::Context(cx) => cx.refs.append(refs),
            Self::Configuration(cf) => cf.refs.append(refs),
        }
        refs.clear();
    }

    pub fn get_refs(&self) -> &IdentifierList {
        match self {
            Self::Entity(e) => e.get_refs(),
            Self::Architecture(a) => a.get_refs(),
            Self::Package(p) => p.get_refs(),
            Self::PackageBody(pb) => pb.get_refs(),
            Self::Context(cx) => cx.get_refs(),
            Self::Configuration(cf) => cf.get_refs(),
        }
    }
}

impl std::fmt::Display for VHDLSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Entity(e) => format!("entity {} {{ generics={:?} ports={:?} }}", &e.name, e.generics, e.ports),
            Self::PackageBody(pb) => format!("package body- {}", pb),
            Self::Architecture(a) => format!("architecture {} for entity {}", &a.name, &a.owner),
            Self::Package(p) => format!("package {}", &p),
            Self::Configuration(c) => format!("configuration {} for entity {}", &c.name, &c.owner),
            Self::Context(c) => format!("context {}", &c.name),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq)]
pub struct Package {
    name: Identifier,
    generics: Generics,
    body: Option<PackageBody>,
    refs: IdentifierList,
    pos: Position,
}

impl Package {
    /// Accesses the references for the entity.
    pub fn get_refs(&self) -> &IdentifierList {
        &self.refs
    }

    pub fn get_position(&self) -> &Position {
        &self.pos
    }
}

impl Display for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, PartialEq)]
pub struct PackageBody {
    owner: Identifier,
    refs: IdentifierList,
    pos: Position,
}

impl PackageBody {
    /// Accesses the references for the entity.
    pub fn get_refs(&self) -> &IdentifierList {
        &self.refs
    }

    pub fn get_owner(&self) -> &Identifier {
        &self.owner
    }

    pub fn get_position(&self) -> &Position {
        &self.pos
    }

    pub fn take_refs(self) -> IdentifierList {
        self.refs
    }
}

impl Display for PackageBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "package body for {}", self.owner)
    }
}

#[derive(Debug, PartialEq)]
pub struct Entity {
    name: Identifier,
    ports: Ports,
    generics: Generics,
    architectures: Vec<Architecture>,
    refs: IdentifierList,
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

    /// References the references for the entity.
    pub fn get_refs(&self) -> &IdentifierList {
        &self.refs
    }

    // Generates VHDL component code from the entity.
    pub fn into_component(&self) -> String {
        let mut result = format!("{} ", Keyword::Component.to_color());
        result.push_str(&format!("{}", color(&self.get_name().to_string(), ENTITY_NAME)));

        if self.generics.0.len() > 0 {
            result.push_str(&format!("\n{} ", Keyword::Generic.to_color()));
            result.push_str(&self.generics.0.to_interface_part_string().to_string());
        }
        if self.ports.0.len() > 0 {
            result.push_str(&format!("\n{} ", Keyword::Port.to_color()));
            result.push_str(&self.ports.0.to_interface_part_string().to_string());
        }
        result.push_str(&format!("\n{} {}{}\n", Keyword::End.to_color(), Keyword::Component.to_color(), Delimiter::Terminator.to_color()));
        result
    }

    /// Generates VHDL signal declaration code from the entity data.
    pub fn into_signals(&self) -> String {
        self.ports.0.to_declaration_part_string(Keyword::Signal).to_string()
    }

    /// Generates VHDL constant declaration code from the entity data.
    pub fn into_constants(&self) -> String {
        self.generics.0.to_declaration_part_string(Keyword::Constant).to_string()
    }

    /// Generates VHDL instantiation code from the entity data.
    pub fn into_instance(&self, inst: &Identifier, library: Option<Identifier>) -> String {
        let prefix = match library {
            Some(lib) => format!("{} {}{}", Keyword::Entity.to_color(), color(&lib.to_string(), ENTITY_NAME), Delimiter::Dot.to_color()),
            None => String::new()
        };
        let mut result = String::from(format!("{} {} {}{}\n", inst.to_color(), Delimiter::Colon.to_color(), prefix, color(&self.get_name().to_string(), ENTITY_NAME)));
        if self.generics.0.len() > 0 {
            result.push_str(&(format!("{} ", Keyword::Generic.to_color())));
            result.push_str(&self.generics.0.to_instantiation_part().to_string())
        }
        if self.ports.0.len() > 0 {
            // add extra spacing
            if self.generics.0.len() > 0 { result.push(' '); }
            result.push_str(&format!("{} ", Keyword::Port.to_color()));
            result.push_str(&self.ports.0.to_instantiation_part().to_string())
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
    fn from_tokens<I>(tokens: &mut Peekable<I>, pos: Position) -> Self 
    where I: Iterator<Item=Token<VHDLToken>> {
        // take entity name
        let entity_name = tokens.next().take().unwrap().take();
        let (generics, ports, entity_refs) = VHDLSymbol::parse_entity_declaration(tokens);

        let generics = generics
            .into_iter()
            .map(|f| f.0 )
            .collect::<Vec<Vec<Token<VHDLToken>>>>();

        let ports = ports
            .into_iter()
            .map(|f| f.0 )
            .collect::<Vec<Vec<Token<VHDLToken>>>>();

        Entity { 
            name: match entity_name {
                    VHDLToken::Identifier(id) => id,
                    _ => panic!("expected an identifier")
            },
            architectures: Vec::new(),
            generics: Generics(InterfaceDeclarations::from_double_listed_tokens(generics)),
            ports: Ports(InterfaceDeclarations::from_double_listed_tokens(ports)),
            refs: entity_refs,
            pos: pos,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Architecture {
    name: Identifier,
    owner: Identifier,
    dependencies: IdentifierList,
    refs: IdentifierList,
    pos: Position,
}

impl Architecture {
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn get_position(&self) -> &Position {
        &self.pos
    }

    pub fn entity(&self) -> &Identifier {
        &self.owner
    }

    pub fn edges(&self) -> &IdentifierList {
        &self.dependencies
    }

    /// Accesses the references for the entity.
    pub fn get_refs(&self) -> &IdentifierList {
        &self.refs
    }
}

#[derive(Debug, PartialEq)]
pub struct Context {
    name: Identifier,
    refs: IdentifierList,
    pos: Position,
}

impl Context {
    /// Accesses the references for the entity.
    pub fn get_refs(&self) -> &IdentifierList {
        &self.refs
    }

    pub fn get_position(&self) -> &Position {
        &self.pos
    }
}

#[derive(Debug, PartialEq)]
pub enum ContextUsage {
    ContextDeclaration(Context),
    ContextReference(IdentifierList),
}

#[derive(Debug, PartialEq)]
pub enum ContextStatement {
    LibraryClause,
    UseClause(UseClause),
    // @TODO Context_reference
}

#[derive(Debug, PartialEq)]
pub struct UseClause {
    imports: Vec<SelectedName>,
}

#[derive(Debug, PartialEq)]
pub struct Configuration {
    name: Identifier,
    owner: Identifier,
    dependencies: IdentifierList,
    refs: IdentifierList,
    pos: Position,
}

impl Configuration {
    pub fn name(&self) -> &Identifier {
        &self.name
    }

    pub fn get_position(&self) -> &Position {
        &self.pos
    }

    pub fn entity(&self) -> &Identifier {
        &self.owner
    }

    pub fn edges(&self) -> &IdentifierList {
        &self.dependencies
    }

    /// Accesses the references for the entity.
    pub fn get_refs(&self) -> &IdentifierList {
        &self.refs
    }
}

/* 
    @NOTE In order to detect if a package was used, the best bet is to just 
    iterate through the the tokens and collect all simple names, i.e. 
    library.name.name. , then try check against the data structure if the name 
    matches anywhere. If so, then it is considered a reference and is needed in 
    that design.
*/

/* 
    @NOTE To check instantiations, check for all identifiers against the
    list of known public identifiers from external API.

    Example: given a ip with public primary design units: adder, adder_pkg
*/

#[derive(Debug, PartialEq)]
struct SelectedName(Vec<Identifier>);

impl SelectedName {
    /// Returns the final identifier in the chain.
    fn get_suffix(&self) -> &Identifier {
        self.0.last().unwrap()
    }

    fn take_suffix(mut self) -> Identifier {
        self.0.pop().unwrap()
    }

    /// Casts the list of identifiers into a list of `CompoundIdentifiers`.
    /// 
    /// If `sep_last` is `true`, then an extra compound will be made with just the 
    /// suffix and no prefix.
    fn into_compound_identifiers(self, sep_last: bool) -> IdentifierList {
        let mut result = IdentifierList::new();

        let mut iter = self.0.into_iter().peekable();
        while let Some(iden) = iter.next() {
            match iter.peek() {
                Some(next) => {
                    result.push_back(CompoundIdentifier::new(iden, next.clone()));
                }
                None => {
                    if sep_last == true {
                        result.push_back(CompoundIdentifier::new_minimal(iden));
                    }
                }
            }
        }

        result
    }
}

/// A `CompoundIdentifier` is a pattern in the code that catches `<library>.<primary-unit>`. We
/// assume the pattern can be found anywhere.
/// 
/// A special case is just a simple name (1 identifier) when referencing a component name.
#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct CompoundIdentifier {
    prefix: Option<Identifier>,
    suffix: Identifier,
}

impl Display for CompoundIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.prefix {
            Some(p) => write!(f, "{}.{}", p, self.suffix),
            None => write!(f, "{}", self.suffix),
        }
    }
}

impl CompoundIdentifier {
    pub fn new(prefix: Identifier, suffix: Identifier) -> Self {
        Self { prefix: Some(prefix), suffix: suffix }
    }

    pub fn new_minimal(suffix: Identifier) -> Self {
        Self { prefix: None, suffix: suffix }
    }

    pub fn get_suffix(&self) -> &Identifier {
        &self.suffix
    }

    pub fn get_prefix(&self) -> Option<&Identifier> {
        self.prefix.as_ref()
    }

    /// Checks if the identifiers `prefix` and `suffix` align with the those of
    /// `self`. Ignores checking the `prefix` if self does not have a prefix.
    pub fn is_match(&self, prefix: &Identifier, suffix: &Identifier) -> bool {
        let first_match = match &self.prefix {
            Some(p) => p == prefix,
            None => true,
        };
        first_match && &self.suffix == suffix
    }
}

#[derive(Debug, PartialEq)]
pub struct VHDLParser {
    symbols: Vec<Symbol<VHDLSymbol>>,
}

impl Parse<VHDLToken> for VHDLParser {
    type SymbolType = VHDLSymbol;
    type Err = String;
    
    fn parse(tokens: Vec<Token<VHDLToken>>) -> Vec<Result<Symbol<Self::SymbolType>, SymbolError<Self::Err>>>
        where <Self as Parse<VHDLToken>>::Err: Display {
            
        let mut symbols = Vec::new();
        let mut tokens = tokens.into_iter().peekable();

        let mut global_refs = IdentifierList::new();

        while let Some(t) = tokens.next() {
            // create entity symbol
            if t.as_ref().check_keyword(&Keyword::Entity) {
                // get the position
                let mut ent = VHDLSymbol::parse_entity(&mut tokens, t.into_position());
                ent.add_refs(&mut global_refs);
                // println!("info: detected {}", ent);
                symbols.push(Ok(Symbol::new(ent)));
            // create architecture symbol
            } else if t.as_ref().check_keyword(&Keyword::Architecture) {
                let mut arch = VHDLSymbol::parse_architecture(&mut tokens, t.into_position());
                arch.add_refs(&mut global_refs);
                // println!("info: detected {}", arch);
                symbols.push(Ok(Symbol::new(arch)));
            // create configuration symbol
            } else if t.as_ref().check_keyword(&Keyword::Configuration) {
                let config = VHDLSymbol::parse_configuration(&mut tokens, t.into_position());
                // println!("info: detected {}", config);
                symbols.push(Ok(Symbol::new(config)));
            // create package symbol
            } else if t.as_ref().check_keyword(&Keyword::Package) {
                let mut pack = VHDLSymbol::route_package_parse(&mut tokens, t.into_position());
                pack.add_refs(&mut global_refs);
                // println!("info: detected {}", pack);
                symbols.push(Ok(Symbol::new(pack)));
            // create a context symbol or context reference
            } else if t.as_ref().check_keyword(&Keyword::Context) {
                match VHDLSymbol::parse_context(&mut tokens, t.into_position()) {
                    ContextUsage::ContextDeclaration(dec) => {
                        let mut context = VHDLSymbol::Context(dec);
                        // println!("info: detected {}", context);
                        context.add_refs(&mut global_refs);
                        symbols.push(Ok(Symbol::new(context)));
                    },
                    ContextUsage::ContextReference(mut refs) => {
                        global_refs.append(&mut refs);
                    }
                };
            // handle global statements (`USE`, `LIBRARY` statements, or invalid code)
            } else {
                // update global references list
                let mut clause = VHDLSymbol::parse_statement(&mut tokens);
                global_refs.append(clause.get_refs_mut());
            }
        }
        // println!("{:#?}", symbols);
        symbols
    }
}

impl VHDLParser {
    pub fn read(s: &str) -> Self {
        let symbols = VHDLParser::parse(VHDLTokenizer::from_source_code(&s).into_tokens());
        Self {
            symbols: symbols.into_iter().filter_map(|f| { if f.is_ok() { Some(f.unwrap()) } else { None } }).collect()
        }
    }

    pub fn into_symbols(self) -> Vec<VHDLSymbol> {
        self.symbols.into_iter().map(|f| f.take()).collect()
    }

}

use std::iter::Peekable;

use super::highlight::*;

/// A `Statement` is a vector of tokens, similiar to how a `String` is a vector
/// of characters.
#[derive(PartialEq, Clone)]
struct Statement(Vec<Token<VHDLToken>>, IdentifierList);

impl Statement {
    /// References the mutable list of resource references.
    fn get_refs_mut(&mut self) -> &mut IdentifierList {
        &mut self.1
    }

    /// References the mutable list of vhdl tokens.
    fn get_tokens_mut(&mut self) -> &mut Vec<Token<VHDLToken>> {
        &mut self.0
    }
}

impl std::fmt::Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // determine which delimiters to not add trailing spaces to
        let is_spaced_token = |d: &Delimiter| {
            match d {
                Delimiter::ParenL | Delimiter::ParenR => false,
                _ => true,
            }
        };
        // iterate through the tokens
        let mut iter = self.0.iter().peekable();
        while let Some(t) = iter.next() {
            let trailing_space = match t.as_ref() {
                VHDLToken::Delimiter(d) => is_spaced_token(d),
                _ => {
                    // make sure the next token is not a tight token (no-spaced)
                    if let Some(m) = iter.peek() {
                        match m.as_ref() {
                            VHDLToken::Delimiter(d) => is_spaced_token(d),
                            _ => true
                        }
                    } else {
                        true
                    }
                }
            };
            write!(f, "{}", t.as_ref().to_string())?;
            if trailing_space == true && iter.peek().is_some() {
                write!(f, " ")?
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for t in &self.0 {
            write!(f, "{} ", t.as_ref().to_string())?
        }
        Ok(())
    }
}

impl Statement {
    fn new() -> Self {
        Self(Vec::new(), IdentifierList::new())
    }

    /// References the tokens as VHDLTokens.
    fn as_types(&self) -> Vec<&VHDLToken> {
        self.0.iter().map(|f| f.as_type() ).collect()
    }

    /// References the resource references.
    fn get_refs(&self) -> &IdentifierList {
        &self.1
    }

    /// References the list of tokens.
    fn get_tokens(&self) -> &Vec<Token<VHDLToken>> {
        &self.0
    }

    /// Transforms into only resource references.
    fn take_refs(self) -> IdentifierList {
        self.1
    }
}

impl VHDLSymbol {
    /// Parses an `Entity` primary design unit from the entity's identifier to
    /// the END closing statement.
    fn parse_entity<I>(tokens: &mut Peekable<I>, pos: Position) -> VHDLSymbol 
    where I: Iterator<Item=Token<VHDLToken>>  {
        VHDLSymbol::Entity(Entity::from_tokens(tokens, pos))
    }

    /// Parses a package declaration, from the <package> IS to the END keyword.
    /// 
    /// Assumes the last consumed token was PACKAGE keyword and the next token
    /// is the identifier for the package name.
    fn parse_package_declaration<I>(tokens: &mut Peekable<I>, pos: Position) -> VHDLSymbol 
    where I: Iterator<Item=Token<VHDLToken>>  {

        let mut refs = IdentifierList::new();
        // take package name
        let pack_name = tokens.next().take().unwrap().take();
        // take the IS keyword
        if tokens.next().take().unwrap().as_type().check_keyword(&Keyword::Is) == false {
            panic!("expecting keyword IS")
        }

        // check if there is a NEW keyword to return instantiation
        if tokens.peek().is_some() && tokens.peek().unwrap().as_type().check_keyword(&Keyword::New) == true {
            // parse the statement to take the package instantiation line
            let clause = Self::parse_statement(tokens);
            // construct a new package
            return VHDLSymbol::Package(Package {
                name: match pack_name {
                    VHDLToken::Identifier(id) => id,
                    _ => panic!("expected an identifier")
                },
                generics: Generics::new(),
                refs: clause.take_refs(),
                body: None,
                pos: pos,
            })
        }

        // check if there is a generic clause
        let mut generics = if tokens.peek().is_some() && tokens.peek().unwrap().as_type().check_keyword(&Keyword::Generic) == true {
            // take the GENERIC keyword token
            tokens.next();
            Self::parse_interface_list(tokens)
        } else {
            Vec::new()
        };

        // grab references from generic interface list
        generics.iter_mut().for_each(|f| {
            refs.append(f.get_refs_mut());
        });

        let generics = generics.into_iter()
            .map(|f| f.0 )
            .collect::<Vec<Vec<Token<VHDLToken>>>>();

        // compose the declarative items
        while let Some(t) = tokens.peek() {
            // check for nested package declarations
            if t.as_type().check_keyword(&Keyword::Package) {
                // consume PACKAGE keyword
                let inner_pos = tokens.next().unwrap().into_position();
                // parse nested package declaration and grab references
                let inner_pack = Self::parse_package_declaration(tokens, inner_pos);
                inner_pack.as_package().unwrap().get_refs().into_iter().for_each(|r| {
                    refs.push_back(r.clone());
                });
            // grab component declarations
            } else if t.as_type().check_keyword(&Keyword::Component) {
                let _comp = Self::parse_component(tokens);
                // println!("component declared: {}", comp);
            } else if t.as_type().check_keyword(&Keyword::End) {
                Self::parse_statement(tokens);
                break;
            } else {
                refs.append(&mut Self::parse_statement(tokens).take_refs());
            }
        }

        // println!("*--- unit {}", pack_name);
        VHDLSymbol::Package(Package {
            name: match pack_name {
                VHDLToken::Identifier(id) => id,
                _ => panic!("expected an identifier")
            },
            generics: Generics(InterfaceDeclarations::from_double_listed_tokens(generics)),
            refs: refs,
            body: None,
            pos: pos,
        })
    }

    /// Creates a `Context` struct for primary design unit: context.
    /// 
    /// Assumes the next token to consume is the context's identifier.
    fn parse_context<I>(tokens: &mut Peekable<I>, pos: Position) -> ContextUsage
    where I: Iterator<Item=Token<VHDLToken>>  {
        // grab the identifier name
        let iden = tokens.next().unwrap().take().take_identifier().unwrap();
        // check the next token is the `is` keyword for declaration
        if tokens.peek().unwrap().as_ref().check_keyword(&Keyword::Is) == true {
            ContextUsage::ContextDeclaration(Context { 
                name: iden, 
                refs: Self::parse_context_declaration(tokens),
                pos: pos
            })
        // parse statement
        } else {
            let mut subtokens = vec![Token::new(VHDLToken::Identifier(iden), Position::new())];
            while let Some(t) = tokens.next() {
                if t.as_ref().check_delimiter(&Delimiter::Terminator) == true {
                    subtokens.push(t);
                    break;
                }
                subtokens.push(t);
            }
            let stmt = Self::parse_statement(&mut subtokens.into_iter().peekable());
            ContextUsage::ContextReference(stmt.1)
        }
    }

    /// Creates a `Context` struct for primary design unit: context.
    /// 
    /// Assumes the next token to consume is the keyword `IS`. Stops at the `end`.
    fn parse_context_declaration<I>(tokens: &mut Peekable<I>) -> IdentifierList
    where I: Iterator<Item=Token<VHDLToken>>  {
        let mut result = IdentifierList::new();

        while let Some(t) = tokens.next() {
            let mut stmt = Self::parse_statement(tokens);

            if t.as_ref().check_keyword(&Keyword::End) == true {
                if Self::is_primary_ending(&stmt) == true {
                    break;
                }
            } else {
                // get references
                result.append(&mut stmt.1);
            }
        }
        result
    }

    /// Collects identifiers into a single vector, stopping at a non-identifier token.
    /// 
    /// Assumes the first token to consume is an identifier, and continues to collect
    /// if the next token is a DOT delimiter.
    /// 
    /// @dead_code
    fn compose_name<I>(tokens: &mut Peekable<I>) -> SelectedName
    where I: Iterator<Item=Token<VHDLToken>>  {
        let mut selected_name = Vec::new();
        // take first token as identifier
        let tk_id = tokens.next().expect("expecting name after '.'");
        if let Some(id) = tk_id.take().take_identifier() {
            selected_name.push(id);
        }
        while let Some(t) = tokens.peek() {
            // consume DOT and expect next identifier
            if t.as_type().check_delimiter(&Delimiter::Dot) {
                // consume DOT
                tokens.next();
                // expect identifier or bail
                let tk_id = tokens.next().expect("expecting name after '.'");

                if tk_id.as_type().check_keyword(&Keyword::All) {
                    // @TODO remember in `name` struct that all was used.
                    break;
                } else if let Some(id) = tk_id.take().take_identifier() {
                    selected_name.push(id);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        SelectedName(selected_name)
    }

    /// Parses a package body, taking BODY keyword up until the END keyword.
    /// 
    /// Package declarations within this scope can be ignored because their visibility
    /// is not reached outside of the body.
    fn parse_package_body<I>(tokens: &mut Peekable<I>, pos: Position) -> PackageBody 
    where I: Iterator<Item=Token<VHDLToken>>  {
        // take the 'body' keyword
        tokens.next();
        // take package name
        let pack_name = tokens.next().take().unwrap().take();
        // println!("*--- package {}", pack_name);
        // take the IS keyword
        if tokens.next().take().unwrap().as_type().check_keyword(&Keyword::Is) == false {
            panic!("expecting keyword IS")
        }
        let (_, refs) = VHDLSymbol::parse_body(tokens, &Self::is_primary_ending);
        PackageBody {
            owner: match pack_name {
                VHDLToken::Identifier(id) => id,
                _ => panic!("expected an identifier")
            },
            refs: refs,
            pos: pos,
        }
    }

    /// Detects identifiers configured in the configuration statement section or architecture
    /// declaration section.
    /// 
    /// Assumes the first token to consume is 'for' and there is a ':' token to follow.
    fn parse_configuration_spec(statement: Statement) -> Option<IdentifierList> {
        let mut tokens = Self::statement_to_tokens(statement).into_iter().peekable();
        // force keyword 'for'
        if tokens.next()?.take().check_keyword(&Keyword::For) == false { return None }
        // take tokens until ':'
        while let Some(tkn) = tokens.next() {
            if tkn.as_ref().check_delimiter(&Delimiter::Colon) == true { break }
        }
        // take the component's name that is being replaced
        tokens.next()?.take().as_identifier()?;

        // take the keyword 'use'
        if tokens.next()?.take().check_keyword(&Keyword::Use) == false { return None }
        
        // entity aspect
        // entity_aspect ::=
        //      entity entity_name [ ( architecture_identifier) ]
        //      | configuration configuration_name
        //      | open
        if tokens.peek()?.as_ref().check_keyword(&Keyword::Entity) == true ||
            tokens.peek()?.as_ref().check_keyword(&Keyword::Configuration) == true {
            // take keyword ENTITY or CONFIGURATION
            tokens.next();
            // take the compound identifiers
            let mut deps = IdentifierList::new();
            // take entity identifier
            deps.append(&mut Self::compose_name(&mut tokens).into_compound_identifiers(true));
            // take remaining possible references
            Self::update_deps_from_statement(&mut deps, &mut tokens);
            Some(deps)
        } else {
            None
        }
    }

    /// Transforms a single statement into a list of tokens.
    /// 
    /// Appends a terminator `;` to the end of the token list.
    fn statement_to_tokens(clause: Statement) -> Vec<Token<VHDLToken>> {
        let mut tokens = clause.0;
        // adds terminating delimiter ';'
        tokens.push(Token::new(VHDLToken::Delimiter(Delimiter::Terminator), Position::new()));
        tokens
    }

    /// Updates a dependencies list `deps` by parsing the token stream.
    /// 
    /// Assumes the token stream is built from a single statement. Assumes the first token in the stream is an
    /// indentifier that could be a dependency, if the first dependency detects is not a compound identifier with that matching suffix.
    fn update_deps_from_statement<I>(deps: &mut IdentifierList, tokens: &mut Peekable<I>) -> ()
    where I: Iterator<Item=Token<VHDLToken>> {
        deps.append(&mut Self::parse_statement(tokens).take_refs());
    }

    /// Detects identifiers instantiated in the architecture statement sections.
    /// 
    /// Assumes the next token to consume is instance name of the instantiation and
    /// the token to follow is the COLON ':' delimiter.
    fn parse_instantiation(statement: Statement) -> Option<IdentifierList> {
        let mut tokens = Self::statement_to_tokens(statement).into_iter().peekable();
        // force identifier (instance name)
        tokens.next()?.take().as_identifier()?;
        // force colon
        if tokens.next()?.take().check_delimiter(&Delimiter::Colon) == false { return None };
        // check what is instantiated
        match tokens.peek()?.as_type() {
            VHDLToken::Identifier(_) => {
                let mut deps = IdentifierList::new();
                // take entity identifier
                deps.append(&mut Self::compose_name(&mut tokens).into_compound_identifiers(true));
                // take remaining possible references
                Self::update_deps_from_statement(&mut deps, &mut tokens);
                Some(deps)
            }
            VHDLToken::Keyword(kw) => {
                if kw == &Keyword::Component || kw == &Keyword::Entity || kw == &Keyword::Configuration {
                    tokens.next();
                    match tokens.peek()?.as_type() {
                        VHDLToken::Identifier(_) => {
                            let mut deps = IdentifierList::new();
                            // take entity identifier
                            deps.append(&mut Self::compose_name(&mut tokens).into_compound_identifiers(true));
                            // take remaining possible references
                            Self::update_deps_from_statement(&mut deps, &mut tokens);
                            Some(deps)
                        },
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn parse_configuration<I>(tokens: &mut Peekable<I>, pos: Position) -> VHDLSymbol 
        where I: Iterator<Item=Token<VHDLToken>>  {
        let config_name = match tokens.next().take().unwrap().take() {
            VHDLToken::Identifier(id) => id,
            _ => panic!("expected an identifier")
        };
        let entity_name = VHDLSymbol::parse_owner_design_unit(tokens);

        // force taking the `is` keyword
        if tokens.next().unwrap().as_type().check_keyword(&Keyword::Is) == false { panic!("expecting keyword 'is'") }

        let mut deps = IdentifierList::new();
        let mut refs = IdentifierList::new();
        // parse configuration section
        while let Some(t) = tokens.peek() {
            if t.as_type().check_keyword(&Keyword::End) {
                let stmt = Self::parse_statement(tokens);
                if Self::is_primary_ending(&stmt) { 
                    break; 
                }
            // enter a block configuration
            } else if t.as_type().check_keyword(&Keyword::For) {
                // take the 'for' keyword
                tokens.next().unwrap();
                deps.append(&mut Self::parse_block_configuration(tokens));
            // @todo handle `use` clauses
            } else {
                let mut clause = Self::parse_statement(tokens);
                refs.append(clause.get_refs_mut());
            }
        }

        VHDLSymbol::Configuration(Configuration {
            name: config_name,
            owner: entity_name,
            dependencies: deps,
            refs: refs,
            pos: pos,
        })
    }

    fn parse_block_configuration<I>(tokens: &mut Peekable<I>) -> IdentifierList 
    where I: Iterator<Item=Token<VHDLToken>>  {
        let mut ids = IdentifierList::new();
        // take the identifier
        tokens.next().unwrap();
        // if next token is '(', take until leveling out to ')'
        if tokens.peek().unwrap().as_type().check_delimiter(&Delimiter::ParenL) {
            tokens.next();
            let mut balance = 1;
            while let Some(t) = tokens.next() {
                if t.as_ref().check_delimiter(&Delimiter::ParenL) == true {
                    balance += 1;
                } else if t.as_ref().check_delimiter(&Delimiter::ParenR) == true {
                    balance -= 1;
                }
                if balance == 0 {
                    break;
                }
            }   
        }
        while let Some(t) = tokens.peek() {
            if t.as_type().check_keyword(&Keyword::End) {
                let stmt = Self::parse_statement(tokens);
                // exit the block configuration
                if Self::is_sub_ending(&stmt) { 
                    break; 
                }
            } else {
                // take configuration specification by composing statement
                let stmt = Self::parse_statement(tokens);
                if let Some(mut idens) = Self::parse_configuration_spec(stmt) {
                    ids.append(&mut idens);
                    // take next `end for`
                    let _ending = Self::parse_statement(tokens);
                }
            }
        }
        ids
    }

    /// Consumes tokens after the USE keyword.
    /// 
    /// Assumes the last token consumed was USE and composes a statement of imports.
    /// 
    /// @dead_code
    fn parse_use_clause<I>(tokens: &mut Peekable<I>) -> UseClause 
        where I: Iterator<Item=Token<VHDLToken>> {
        // collect first selected_name
        let mut imports = Vec::new();
        imports.push(Self::compose_name(tokens));
        while let Some(t) = tokens.next() {
            // take the comma, then next selected name
            if t.as_type().check_delimiter(&Delimiter::Comma) {
                imports.push(Self::compose_name(tokens));
            }
            if t.as_type().check_delimiter(&Delimiter::Terminator) {
                break;
            }
        }
        UseClause { imports: imports }
    }

    /// Parses an secondary design unit: architecture.
    /// 
    /// Assumes the next token to consume is the architecture's identifier.
    fn parse_architecture<I>(tokens: &mut Peekable<I>, pos: Position) -> VHDLSymbol 
        where I: Iterator<Item=Token<VHDLToken>> {
        let arch_name = match tokens.next().take().unwrap().take() {
            VHDLToken::Identifier(id) => id,
            _ => panic!("expected an identifier")
        };
        let entity_name = VHDLSymbol::parse_owner_design_unit(tokens);
        // println!("*--- unit {}", arch_name);

        let (deps, refs) =  VHDLSymbol::parse_declaration(tokens, &Self::is_primary_ending);
        VHDLSymbol::Architecture(Architecture {
            name: arch_name,
            owner: entity_name,
            dependencies: deps,
            refs: refs,
            pos: pos,
        })
    }

    /// Checks if the statement `stmt` is the code to enter a valid sub-declaration section.
    fn is_sub_declaration(stmt: &Statement) -> bool {
        let first = match stmt.0.first() {
            Some(tk) => tk.as_ref(),
            None => return false
        };
        // verify first keyword
        if let Some(keyword) = first.as_keyword() {
            match keyword {
                Keyword::Function | Keyword::Procedure => (),
                _ => return false,
            }
        } else {
            return false
        };
        // verify last keyword is BEGIN
        let last = match stmt.0.last() {
            Some(tk) => tk.as_ref(),
            None => return false,
        };
        if let Some(keyword) = last.as_keyword() {
            keyword == &Keyword::Begin
        } else {
            false
        }
    }

    /// Parses together a series of tokens into a single `Statement`.
    /// 
    /// Statements end on a ';' and do not include the ';' token. If the EOF
    /// is reached before completing a statement, it is omitted and a blank
    /// statement is returned.
    fn parse_statement<I>(tokens: &mut Peekable<I>) -> Statement 
        where I: Iterator<Item=Token<VHDLToken>> {

        let mut clause = Statement::new();
        // traverse through token stream
        while let Some(t) = tokens.next() {
            // exit upon encountering terminator ';'
            if t.as_type().check_delimiter(&Delimiter::Terminator) {
                return clause
            // extra keywords to help break up statements early
            } else if t.as_type().check_keyword(&Keyword::Generate) || 
                t.as_type().check_keyword(&Keyword::Begin) || 
                (clause.get_tokens().first().is_some() && clause.get_tokens().first().unwrap().as_type().check_keyword(&Keyword::When) &&
                t.as_type().check_delimiter(&Delimiter::Arrow)) {
                // add the breaking token to the statement before exiting
                clause.get_tokens_mut().push(t);
                return clause
            } else {
                // check for compound identifiers as references to other design units
                let mut took_dot: Option<Token<VHDLToken>> = None;
                if let Some(id) = t.as_type().as_identifier() {
                    // check if next token is a 'dot' delimiter
                    if tokens.peek().is_some() && tokens.peek().unwrap().as_type().check_delimiter(&Delimiter::Dot) {
                        // consume the dot
                        took_dot = tokens.next();
                        // mark as a reference if the next token is an identifier
                        if tokens.peek().is_some() {
                            if let Some(id2) = tokens.peek().unwrap().as_type().as_identifier() {
                                // store the resource reference
                                clause.get_refs_mut()
                                    .push_back(CompoundIdentifier::new(id.clone(), id2.clone()));
                            }
                        }
                    }
                }
                clause.get_tokens_mut().push(t);
                if let Some(dot) = took_dot {
                    clause.get_tokens_mut().push(dot);
                }
            }
        }
        // return empty statement if unable to close with terminator ';'
        Statement::new()
    }

    /// Parses the OF keyword and then returns the following IDENTIFIER.
    /// 
    /// The Identifier should correspond to the architecture's entity name.
    fn parse_owner_design_unit<I>(tokens: &mut Peekable<I>) -> Identifier
    where I: Iterator<Item=Token<VHDLToken>>  {
        // force taking the 'of' keyword
        if tokens.next().unwrap().as_type().check_keyword(&Keyword::Of) == false {
            panic!("expecting 'of' keyword")
        }
        // return the name of the primary design unit
        match tokens.next().take().unwrap().take() {
            VHDLToken::Identifier(id) => id,
            _ => panic!("expected an identifier")
        }
    }

    /// Parses a subprogram header, stopping at first occurence of IS or BEGIN keyword
    fn parse_subprogram_header<I>(_: &mut Peekable<I>) -> ()
    where I: Iterator<Item=Token<VHDLToken>>  {
        // optionally take
        todo!("implement");
    }

    /// Returns a list of interface items as `Statements`. 
    /// 
    /// Assumes the last token consumed was either GENERIC or PORT keywords and
    /// stops at the last statement in the respective list.
    fn parse_interface_list<I>(tokens: &mut Peekable<I>) -> Vec<Statement>
    where I: Iterator<Item=Token<VHDLToken>>  {
        // expect the opening '('
        if tokens.next().unwrap().as_type().check_delimiter(&Delimiter::ParenL) == false {
            panic!("expecting '(' delimiter")
        }
        // collect statements until finding the ')', END, BEGIN, or PORT.
        let mut io: Vec<Statement> = Vec::new();
        while let Some(t) = tokens.peek() {

            if let Some(item) = io.last() {
                // safe to unwrap because of outer condition
                if item.get_tokens().last().unwrap().as_type().check_delimiter(&Delimiter::ParenR) {
                    // compute number of balancing LHS and RHS parentheses
                    let balance = item.get_tokens().iter().fold(0, |acc, x| {
                        if x.as_type().check_delimiter(&Delimiter::ParenL) { acc + 1 }
                        else if x.as_type().check_delimiter(&Delimiter::ParenR) { acc - 1 }
                        else { acc }
                    });
                    // check if there is an uneven amount of RHS parentheses to indicate end of interface
                    if balance == -1 {
                        let index = io.len()-1;
                        // pop off extra RHS parenthesis and exit
                        let last_clause = io.get_mut(index).unwrap();
                        last_clause.get_tokens_mut().pop().expect("expecting closing ')'");
                        break;
                    }
                }
            }
            // the RHS parenthesis is a single statement (';' used on last interface item)
            if t.as_type().check_delimiter(&Delimiter::ParenR) {
                    Self::parse_statement(tokens);
                    break;
            // collect statements
            } else {
                io.push(Self::parse_statement(tokens));
                // println!("{}", statements.last().unwrap());
            }
        }
        // println!("{:?}", statements);
        io
    }

    /// Consumes tokens after `IS` until finding `BEGIN` or `END`.
    /// 
    /// Assumes the next token to consume is `IS` and throws it away. This will
    /// search for interface lists found after GENERIC and PORT keywords.
    fn parse_entity_declaration<I>(tokens: &mut Peekable<I>) -> (Vec<Statement>, Vec<Statement>, IdentifierList)
        where I: Iterator<Item=Token<VHDLToken>> {
        // println!("*--- declaration section");
        // force taking the 'is' keyword
        if tokens.next().unwrap().as_type().check_keyword(&Keyword::Is) == false {
            panic!("expecting 'is' keyword")
        }
        // check entity_header before entering entity declarative part
        // check for generics
        if tokens.peek().is_none() { panic!("expecting END keyword") }
        let mut generics = if tokens.peek().unwrap().as_type().check_keyword(&Keyword::Generic) {
            tokens.next();
            Self::parse_interface_list(tokens)
        } else {
            Vec::new()
        };
        // check for ports
        if tokens.peek().is_none() { panic!("expecting END keyword") }
        let mut ports = if tokens.peek().unwrap().as_type().check_keyword(&Keyword::Port) {
            tokens.next();
            Self::parse_interface_list(tokens)
        } else {
            Vec::new()
        };

        let mut entity_refs = IdentifierList::new();
        ports.iter_mut().for_each(|f| {
            entity_refs.append(f.get_refs_mut());
        });
        generics.iter_mut().for_each(|f| {
            entity_refs.append(f.get_refs_mut());
        });

        while let Some(t) = tokens.peek() {
            // stop the declaration section and enter a statement section
            if t.as_type().check_keyword(&Keyword::Begin) {
                tokens.next();
                let (_, mut refs) = Self::parse_body(tokens, &Self::is_primary_ending);
                entity_refs.append(&mut refs);
                break;
            // the declaration is over and there is no statement section
            } else if t.as_type().check_keyword(&Keyword::End) {
                let stmt = Self::parse_statement(tokens);
                if Self::is_primary_ending(&stmt) { 
                    break; 
                }
            // find a nested package (throw away for now)
            } else if t.as_type().check_keyword(&Keyword::Package) {
                let inner_pos = tokens.next().unwrap().into_position();
                let pack_name = Self::route_package_parse(tokens, inner_pos);
                // add references found from the package
                pack_name.as_package().unwrap().get_refs().into_iter().for_each(|f| {
                    entity_refs.push_back(f.clone());
                });
            // build statements to throw away
            } else {
                let mut clause = Self::parse_statement(tokens);
                entity_refs.append(clause.get_refs_mut());
            } 
        }
        (generics, ports, entity_refs)
    }

    /// Checks if the keyword `kw` is a potential start to a subprogram.
    fn is_subprogram(kw: &Keyword) -> bool {
        match kw {
            Keyword::Function | Keyword::Procedure | Keyword::Pure | Keyword::Impure => true,
            _ => false,
        }
    }

    /// Checks if the statement `stmt` is the ending to a subprogram.
    fn is_subprogram_ending(stmt: &Statement) -> bool {
        // verify the first token is the 'end' keyword
        match stmt.0.get(0) {
            Some(i) => match i.as_ref().as_keyword() {
                Some(kw) => if kw != &Keyword::End { return false },
                None => return false,
            },
            None => return false,
        }
        // verify the next token is the keyword for subprogram
        match stmt.0.get(1) {
            Some(i) => match i.as_ref().as_keyword() {
                Some(kw) => match kw {
                    Keyword::Procedure | Keyword::Function => true,
                    _ => false,
                },
                None => false,
            },
            None => false,
        }
    }

    /// Parses through a subprogram (procedure or function).
    /// 
    /// Returns (`deps`, `refs`).
    fn parse_subprogram<I>(tokens: &mut Peekable<I>) -> (IdentifierList, IdentifierList)
    where I: Iterator<Item=Token<VHDLToken>> {
        let mut refs = IdentifierList::new();
        let mut deps = IdentifierList::new();
        let mut is_return_yet: bool = false;
        while let Some(t) = tokens.peek() {
            // trigger when the statement will be the end of the declaration line
            if t.as_type().check_keyword(&Keyword::Return) == true {
                is_return_yet = true;
            }
            // determine when to branch to declaration section or body section
            if t.as_type().check_keyword(&Keyword::Is) {
                // println!("OUT SUB: {:?}", t);
                let (mut sub_deps, mut sub_refs) = Self::parse_declaration(tokens, &Self::is_subprogram_ending);
                deps.append(&mut sub_deps);
                refs.append(&mut sub_refs);
                break;
            } else if is_return_yet && t.as_type().check_delimiter(&Delimiter::Terminator) {
                // println!("OUT SUB: {:?}", t);
                break;
            } else {
                // println!("IN SUB: {:?}", t);
                tokens.next();
            }
        }
        // @todo: capture references from declaration and body sections
        (deps, refs)
    }

    /// Consumes tokens after `IS` until finding `BEGIN` or `END`.
    /// 
    /// Assumes the next token to consume is `IS` and throws it away.
    /// 
    /// Returns (`deps`, `refs`)
    fn parse_declaration<I>(tokens: &mut Peekable<I>, eval_exit: &dyn Fn(&Statement) -> bool) -> (IdentifierList, IdentifierList)
        where I: Iterator<Item=Token<VHDLToken>> {
        // println!("*--- declaration section");
        // force taking the 'is' keyword
        if tokens.next().unwrap().as_type().check_keyword(&Keyword::Is) == false {
            panic!("expecting 'is' keyword")
        }
        let mut refs = IdentifierList::new();
        let mut deps = IdentifierList::new();
        while let Some(t) = tokens.peek() {
            // println!("dec: {:?}", t);
            // stop the declaration section and enter a statement section
            if t.as_type().check_keyword(&Keyword::Begin) {
                tokens.next();
                // combine refs from declaration and from body
                let (mut body_deps, mut body_refs) = Self::parse_body(tokens, &eval_exit);
                refs.append(&mut body_refs);
                deps.append(&mut body_deps);
                // println!("{}", "stop reading tokens");
                // STOP READING TOKENS
                break;
            // the declaration is over and there is no statement section
            } else if t.as_type().check_keyword(&Keyword::End) {
                let stmt = Self::parse_statement(tokens);
                // println!("end of dec: {:?}", stmt);
                if eval_exit(&stmt) { 
                    break; 
                }
            // find component names (could be in package or architecture declaration)
            } else if t.as_type().check_keyword(&Keyword::Component) {
                let _comp_name = Self::parse_component(tokens);
                // println!("**** INFO: Found component: \"{}\"", comp_name);
            // find a nested package
            } else if t.as_type().check_keyword(&Keyword::Package) {
                let inner_pos = tokens.next().unwrap().into_position();
                let _pack_name = Self::route_package_parse(tokens, inner_pos);
                // println!("**** INFO: detected nested package \"{}\"", pack_name);
            // detect subprograms
            } else if t.as_type().as_keyword().is_some() && Self::is_subprogram(t.as_type().as_keyword().unwrap()) == true {
                // println!("{}", "sub program");
                let (mut sub_deps, mut sub_refs) = Self::parse_subprogram(tokens);
                deps.append(&mut sub_deps);
                refs.append(&mut sub_refs);
            // build statements to throw away
            } else {
                let stmt = Self::parse_statement(tokens);
                // add resource references
                // println!("st {:?}", stmt);
                let (tokens, mut resrefs) = (stmt.0, stmt.1);
                // check if using a configuration specification
                if let Some(mut idens) = Self::parse_configuration_spec(Statement(tokens, IdentifierList::new())) {
                    deps.append(&mut idens);
                } else {
                    refs.append(&mut resrefs);
                }
            }
        }
        (deps, refs)
    }

    /// Checks if the statement is a valid primary unit END statement.
    /// 
    /// Rejects inferior END statements that require an explicit keyword to following
    /// the END keyword. Primary END statements follow: `END [keyword] [identifier];`.
    fn is_primary_ending(stmt: &Statement) -> bool {
        let keyword = if let Some(t) = stmt.0.get(1) {
            t.as_type()
        } else {
            return true // only having "end;" is valid
        };
        match keyword {
            // list mandatory keywords expected after the 'end' keyword for non-primary endings
            VHDLToken::Keyword(kw) => match kw {
                Keyword::Loop | Keyword::Generate | Keyword::Process |
                Keyword::Postponed | Keyword::If | Keyword::Block | 
                Keyword::Protected | Keyword::Record | Keyword::Case | 
                Keyword::Component | Keyword::For => false,
                _ => true,
            },
            _ => true,
        }
    }

    /// Checks if the statement is a valid non-primary unit END statement.
    /// 
    /// This is the negation of `is_primary_ending`.
    fn is_sub_ending(stmt: &Statement) -> bool {
        !Self::is_primary_ending(stmt)
    }

    /// Checks if the keyword indicates a subprogram statement.
    fn enter_subprogram(kw: &Keyword) -> bool {
        match kw {
            Keyword::Function | Keyword::Procedure | Keyword::Impure | 
            Keyword::Pure => true,
            _ => false,
        }
    }

    /// Parses a component declaration, consuming the tokens `COMPONENT` until the end.
    /// 
    /// Assumes the first token to consume is `COMPONENT`.
    fn parse_component<I>(tokens: &mut Peekable<I>) -> CompoundIdentifier
    where I: Iterator<Item=Token<VHDLToken>>  {
        if tokens.next().unwrap().as_type().check_keyword(&Keyword::Component) == false {
            panic!("assumes first token is COMPONENT keyword");
        }
        // take component name
        let comp_name = tokens.next().take().unwrap().take();
        // println!("*--- found component {}", comp_name);
        // take 'is' keyword (optional)
        if tokens.peek().unwrap().as_type().check_keyword(&Keyword::Is) {
            tokens.next();
        }
        // collect port names and generic names until hitting 'END'
        while let Some(t) = tokens.peek() {
            if t.as_type().check_keyword(&Keyword::End) {
                let _stmt = Self::parse_statement(tokens);
                break;
            // collect generic interface
            } else if t.as_type().check_keyword(&Keyword::Generic) {
                // take the GENERIC token
                tokens.next();
                let _generics = Self::parse_interface_list(tokens);
            // collect port interface
            } else if t.as_type().check_keyword(&Keyword::Port) {
                // take the PORT token
                tokens.next();
                let _ports = Self::parse_interface_list(tokens);
            } else {
                let _stmt = Self::parse_statement(tokens);
                // println!("{:?}", stmt);
            }
        }
        match comp_name {
            VHDLToken::Identifier(id) => CompoundIdentifier::new_minimal(id),
            _ => panic!("expected an identifier")
        }
    }

    /// Routes the parsing to either package body or package declaration,
    /// depending on the next token being BODY keyword or identifier.
    fn route_package_parse<I>(tokens: &mut Peekable<I>, pos: Position) -> VHDLSymbol
    where I: Iterator<Item=Token<VHDLToken>> {
        if &VHDLToken::Keyword(Keyword::Body) == tokens.peek().unwrap().as_type() {
            VHDLSymbol::PackageBody(VHDLSymbol::parse_package_body(tokens, pos))
        } else {
            VHDLSymbol::parse_package_declaration(tokens, pos)
        }
    }

    /// Parses a body, consuming tokens from `BEGIN` until `END`.
    /// 
    /// Builds statements and stops after finding the `END` keyword statement. If
    /// the `END` keyword statement is detected, it will have to pass the `eval_exit`
    /// function to properly exit scope. Assumes the last token consumed was `BEGIN`.
    fn parse_body<I>(tokens: &mut Peekable<I>, eval_exit: &dyn Fn(&Statement) -> bool) -> (IdentifierList, IdentifierList)
        where I: Iterator<Item=Token<VHDLToken>> {
        // collect component names
        let mut deps = IdentifierList::new();
        let mut refs = IdentifierList::new();
        // println!("*--- statement section");
        while let Some(t) = tokens.peek() {
            if t.as_type().check_keyword(&Keyword::End) == true {
                let stmt = Self::parse_statement(tokens);
                // println!("in body: {:?}", stmt);
                if eval_exit(&stmt) == true { 
                    break; 
                }
            // enter a subprogram
            } else if t.as_type().check_keyword(&Keyword::Function) || t.as_type().check_keyword(&Keyword::Begin) {
                let mut stmt = Self::parse_statement(tokens);
                // println!("ENTERING SUBPROGRAM {:?}", stmt);
                refs.append(&mut stmt.1);
                let mut inner = Self::parse_body(tokens, &Self::is_sub_ending);
                refs.append(&mut inner.1);
                // println!("EXITING SUBPROGRAM");
            // find component names (could be in package)
            } else if t.as_type().check_keyword(&Keyword::Component) {
                let _comp_name = Self::parse_component(tokens);
                // println!("**** INFO: Found component: \"{}\"", comp_name);
            // find packages 
            } else if t.as_type().check_keyword(&Keyword::Package) {
                let inner_pos = tokens.next().unwrap().into_position();
                let _symbol = Self::route_package_parse(tokens, inner_pos);
                // println!("**** INFO: Detected nested package \"{}\"", symbol);
            // build statements
            } else {
                let mut stmt = Self::parse_statement(tokens);
                // println!("in body: {:?}", stmt);
                refs.append(&mut stmt.1);
                // check if statement is an instantiation
                if let Some(mut inst) = Self::parse_instantiation(stmt) {
                    // println!("info: detected dependency \"{}\"", inst);
                    deps.append(&mut inst);
                }
            }
        }
        // println!("{:?}", deps);
        (deps, refs)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_use_clause() {
        let s = "use eel4712c.pkg1, eel4712c.pkg2; entity";
        let mut tokens = VHDLTokenizer::from_source_code(&s).into_tokens().into_iter().peekable();
        // take USE
        tokens.next();
        let using_imports = VHDLSymbol::parse_use_clause(&mut tokens);
        assert_eq!(using_imports, UseClause { 
            imports: vec![
                SelectedName(vec![
                    Identifier::Basic("eel4712c".to_owned()),
                    Identifier::Basic("pkg1".to_owned()),
                ]),
                SelectedName(vec![
                    Identifier::Basic("eel4712c".to_owned()),
                    Identifier::Basic("pkg2".to_owned()),
                ]),
        ]});
        assert_eq!(tokens.next().unwrap().as_type(), &VHDLToken::Keyword(Keyword::Entity));
    }

    #[test]
    fn compose_statement_2() {
        let s = "P1, P2: inout BIT); constant Delay: TIME := 1 ms;";
        let tokens = VHDLTokenizer::from_source_code(&s).into_tokens();
        let mut iter = tokens.into_iter().peekable();
        assert_eq!(VHDLSymbol::parse_statement(&mut iter).as_types(), vec![
            &VHDLToken::Identifier(Identifier::Basic("P1".to_owned())),
            &VHDLToken::Delimiter(Delimiter::Comma),
            &VHDLToken::Identifier(Identifier::Basic("P2".to_owned())),
            &VHDLToken::Delimiter(Delimiter::Colon),
            &VHDLToken::Keyword(Keyword::Inout),
            &VHDLToken::Identifier(Identifier::Basic("BIT".to_owned())),
            &VHDLToken::Delimiter(Delimiter::ParenR),
        ]);
        assert_eq!(iter.next().unwrap().as_type(), &VHDLToken::Keyword(Keyword::Constant));
    }

    #[test]
    fn parse_simple_name() {
        let s = "eel4712c.nor_gate port";
        let mut tokens = VHDLTokenizer::from_source_code(&s).into_tokens().into_iter().peekable();
        let sel_name = VHDLSymbol::compose_name(&mut tokens);
        assert_eq!(sel_name, SelectedName(vec![
            Identifier::Basic("eel4712c".to_owned()),
            Identifier::Basic("nor_gate".to_owned()),
        ]));
        assert_eq!(tokens.next().unwrap().as_type(), &VHDLToken::Keyword(Keyword::Port));
    }

    #[test]
    fn parse_simple_name_with_all() {
        // @TODO signify within a 'name' struct that the all keyword was used
        let s = "eel4712c.all +";
        let mut tokens = VHDLTokenizer::from_source_code(&s).into_tokens().into_iter().peekable();
        let sel_name = VHDLSymbol::compose_name(&mut tokens);
        assert_eq!(sel_name, SelectedName(vec![
            Identifier::Basic("eel4712c".to_owned()),
        ]));
        assert_eq!(tokens.next().unwrap().as_type(), &VHDLToken::Delimiter(Delimiter::Plus));
    }

    #[test]
    fn parse_port_interface_difficult_ending() {
        let s = "\
port (P1, P2: inout BIT); 
constant Delay: TIME := 1 ms;";
        let mut tokens = VHDLTokenizer::from_source_code(&s).into_tokens().into_iter().peekable();
        tokens.next(); // take PORT
        let ports = VHDLSymbol::parse_interface_list(&mut tokens);
        let ports: Vec<String> = ports.into_iter().map(|m| m.to_string()).collect();
        assert_eq!(ports, vec![
            "P1 , P2 : inout BIT",
        ]);
        assert_eq!(tokens.next().unwrap().as_type(), &VHDLToken::Keyword(Keyword::Constant));
    }

    #[test]
    fn parse_ports_both() {
        let s = "\
generic ( N: positive );
port(
    a: in  std_logic_vector(N-1 downto 0);
    b: in  std_logic_vector(N-1 downto 0);
    c: out std_logic_vector(N-1 downto 0)
);
end;";
        let mut tokens = VHDLTokenizer::from_source_code(&s).into_tokens().into_iter().peekable();
        tokens.next(); // take GENERIC
        let generics = VHDLSymbol::parse_interface_list(&mut tokens);
        // convert to strings for easier verification
        let generics: Vec<String> = generics.into_iter().map(|m| m.to_string()).collect();
        assert_eq!(generics, vec![
            "N : positive",
        ]);
        // take PORT
        tokens.next();
        let ports = VHDLSymbol::parse_interface_list(&mut tokens);
         // convert to strings for easier verification
        let ports: Vec<String> = ports.into_iter().map(|m| m.to_string()).collect();
        assert_eq!(ports, vec![
            "a : in std_logic_vector(N - 1 downto 0)",
            "b : in std_logic_vector(N - 1 downto 0)",
            "c : out std_logic_vector(N - 1 downto 0)",
        ]);
        assert_eq!(tokens.next().unwrap().as_type(), &VHDLToken::Keyword(Keyword::End));
    }

    #[test]
    fn parse_generics_only() {
        let s = "\
generic ( N: positive );
begin
end;";
        let mut tokens = VHDLTokenizer::from_source_code(&s).into_tokens().into_iter().peekable();
        tokens.next(); // take GENERIC
        let generics = VHDLSymbol::parse_interface_list(&mut tokens);
        // convert to strings for easier verification
        let generics: Vec<String> = generics.into_iter().map(|m| m.to_string()).collect();
        assert_eq!(generics, vec![
            "N : positive",
        ]);
        assert_eq!(tokens.next().unwrap().as_type(), &VHDLToken::Keyword(Keyword::Begin));
    }

    #[test]
    fn parse_component() {
        // ends with 'end component nor_gate;' Statement
        let s = "\
component nor_gate is end component nor_gate;

signal ready: std_logic;";
        let mut tokens = VHDLTokenizer::from_source_code(&s).into_tokens().into_iter().peekable();
        let comp = VHDLSymbol::parse_component(&mut tokens);
        assert_eq!(comp.to_string(), "nor_gate");
        assert_eq!(tokens.next().unwrap().as_type(), &VHDLToken::Keyword(Keyword::Signal));
        
        // ends with 'end;' statement
        let s = "\
component nor_gate end;

signal ready: std_logic;";
        let mut tokens = VHDLTokenizer::from_source_code(&s).into_tokens().into_iter().peekable();
        let comp = VHDLSymbol::parse_component(&mut tokens);
        assert_eq!(comp.to_string(), "nor_gate");
        assert_eq!(tokens.next().unwrap().as_type(), &VHDLToken::Keyword(Keyword::Signal));

        // ends with 'end component nor_gate;' statement
        let s = "\
-- declare DUT component
component nor_gate 
    generic( N: positive );
    port(
        a: in  std_logic_vector(N-1 downto 0);
        b: in  std_logic_vector(N-1 downto 0);
        c: out std_logic_vector(N-1 downto 0)
    );
end component nor_gate;

signal ready: std_logic;";
        let mut tokens = VHDLTokenizer::from_source_code(&s).into_tokens().into_iter().peekable();
        let comp = VHDLSymbol::parse_component(&mut tokens);
        assert_eq!(comp.to_string(), "nor_gate");
        assert_eq!(tokens.next().unwrap().as_type(), &VHDLToken::Keyword(Keyword::Signal));
    }

    use super::super::super::lexer::Position;

    #[test]
    fn is_primary_ending() {
        // 'case' is not a primary ending
        let stmt = Statement(vec![
            Token::new(VHDLToken::Keyword(Keyword::End), Position::new()),
            Token::new(VHDLToken::Keyword(Keyword::Case), Position::new()),
            Token::new(VHDLToken::Identifier(Identifier::Basic("case_label".to_owned())), Position::new()),
        ], IdentifierList::new());
        assert_eq!(VHDLSymbol::is_primary_ending(&stmt), false);

        // primary endings can omit keyword and identifier label
        let stmt = Statement(vec![
            Token::new(VHDLToken::Keyword(Keyword::End), Position::new()),
        ], IdentifierList::new());
        assert_eq!(VHDLSymbol::is_primary_ending(&stmt), true);

        // primary endings can include their keyword
        let stmt = Statement(vec![
            Token::new(VHDLToken::Keyword(Keyword::End), Position::new()),
            Token::new(VHDLToken::Keyword(Keyword::Architecture), Position::new()),
            Token::new(VHDLToken::Identifier(Identifier::Basic("architecture_name".to_owned())), Position::new()),
        ], IdentifierList::new());
        assert_eq!(VHDLSymbol::is_primary_ending(&stmt), true);

        // primary endings can have their keyword omitted and also include the identifier label
        let stmt = Statement(vec![
            Token::new(VHDLToken::Keyword(Keyword::End), Position::new()),
            Token::new(VHDLToken::Identifier(Identifier::Basic("architecture_name".to_owned())), Position::new()),
        ], IdentifierList::new());
        assert_eq!(VHDLSymbol::is_primary_ending(&stmt), true);
    }

    #[test]
    fn entity() {
        let s = "\
 nor_gate is
    generic(
        N: positive
    );
    port(
        a: in  std_logic_vector(N-1 downto 0);
        b: in  std_logic_vector(N-1 downto 0);
        c: out std_logic_vector(N-1 downto 0)
    );
end entity nor_gate;";
        let mut tokens = VHDLTokenizer::from_source_code(&s).into_tokens().into_iter().peekable();
        let e = Entity::from_tokens(&mut tokens, Position::place(1, 2));
        assert_eq!(e.pos, Position::place(1, 2));
        assert_eq!(e.name, Identifier::Basic(String::from("nor_gate")));
        assert_eq!(e.generics.0.len(), 1);
        assert_eq!(e.ports.0.len(), 3);
    }

    use std::str::FromStr;

    #[test]
    fn resource_refs() {
        let s = "a : in std_logic_vector(3 downto 0) := work.pack1.p1;";
        let tokens = VHDLTokenizer::from_source_code(&s).into_tokens();
        let mut iter = tokens.into_iter().peekable();
        assert_eq!(VHDLSymbol::parse_statement(&mut iter).get_refs(), &IdentifierList::from([
            CompoundIdentifier::new(Identifier::from_str("work").unwrap(), Identifier::from_str("pack1").unwrap()),
            CompoundIdentifier::new(Identifier::from_str("pack1").unwrap(), Identifier::from_str("p1").unwrap()),
        ]));

        let s = "use work.package_name;";
        let tokens = VHDLTokenizer::from_source_code(&s).into_tokens();
        let mut iter = tokens.into_iter().peekable();
        assert_eq!(VHDLSymbol::parse_statement(&mut iter).get_refs(), &IdentifierList::from([
            CompoundIdentifier::new(Identifier::from_str("work").unwrap(), Identifier::from_str("package_name").unwrap()),
        ]));

        let s = "use MKS.MEASUREMENTS, STD.STANDARD;";
        let tokens = VHDLTokenizer::from_source_code(&s).into_tokens();
        let mut iter = tokens.into_iter().peekable();
        assert_eq!(VHDLSymbol::parse_statement(&mut iter).get_refs(), &IdentifierList::from([
            CompoundIdentifier::new(Identifier::from_str("MKS").unwrap(), Identifier::from_str("MEASUREMENTS").unwrap()),
            CompoundIdentifier::new(Identifier::from_str("STD").unwrap(), Identifier::from_str("STANDARD").unwrap()),
        ]));
    }

    #[test]
    fn compose_statement() {
        let s = "a : in std_logic_vector(3 downto 0);";
        let tokens = VHDLTokenizer::from_source_code(&s).into_tokens();
        let mut iter = tokens.into_iter().peekable();
        assert_eq!(VHDLSymbol::parse_statement(&mut iter).as_types(), vec![
            &VHDLToken::Identifier(Identifier::Basic("a".to_owned())),
            &VHDLToken::Delimiter(Delimiter::Colon),
            &VHDLToken::Keyword(Keyword::In),
            &VHDLToken::Identifier(Identifier::Basic("std_logic_vector".to_owned())),
            &VHDLToken::Delimiter(Delimiter::ParenL),
            &VHDLToken::AbstLiteral(AbstLiteral::Decimal("3".to_owned())),
            &VHDLToken::Keyword(Keyword::Downto),
            &VHDLToken::AbstLiteral(AbstLiteral::Decimal("0".to_owned())),
            &VHDLToken::Delimiter(Delimiter::ParenR),
        ]);

        let s = "a : in std_logic_vector(3 downto 0); ready: out std_logic);";
        let tokens = VHDLTokenizer::from_source_code(&s).into_tokens();
        let mut iter = tokens.into_iter().peekable();
        assert_eq!(VHDLSymbol::parse_statement(&mut iter).as_types(), vec![
            &VHDLToken::Identifier(Identifier::Basic("a".to_owned())),
            &VHDLToken::Delimiter(Delimiter::Colon),
            &VHDLToken::Keyword(Keyword::In),
            &VHDLToken::Identifier(Identifier::Basic("std_logic_vector".to_owned())),
            &VHDLToken::Delimiter(Delimiter::ParenL),
            &VHDLToken::AbstLiteral(AbstLiteral::Decimal("3".to_owned())),
            &VHDLToken::Keyword(Keyword::Downto),
            &VHDLToken::AbstLiteral(AbstLiteral::Decimal("0".to_owned())),
            &VHDLToken::Delimiter(Delimiter::ParenR),
        ]);

        let s = "process(all) is begin end process;";
        let mut tokens = VHDLTokenizer::from_source_code(&s).into_tokens().into_iter().peekable();
        let _ = VHDLSymbol::parse_statement(&mut tokens);
        assert_eq!(tokens.next().unwrap().as_type(), &VHDLToken::Keyword(Keyword::End));
    }

    #[test]
    fn print_statement() {
        let s = "a : in std_logic_vector ( 3 downto 0);";
        let tokens = VHDLTokenizer::from_source_code(&s).into_tokens();
        let mut iter = tokens.into_iter().peekable();
        let st = VHDLSymbol::parse_statement(&mut iter);
        assert_eq!(st.to_string(), "a : in std_logic_vector(3 downto 0)");
    }

    #[test]
    fn configuration() {
        let s = r#"
use work.all;

configuration HA_Config of HA_Entity is
    for HA_Arch
        for HA_Inst : HA_Comp
            use entity HA_Comp_Entity(HA_Comp_Arch_1);
        end for;
        use work.some_pkg.all;
        for HA_Inst : HA_Comp
            use entity HA_Comp_Entity2(HA_Comp_Arch_1);
        end for;
    end for;
end HA_Config;    
"#;
        let symbols = VHDLParser::parse(VHDLTokenizer::from_source_code(&s).into_tokens());
        assert_eq!(symbols.first().unwrap().as_ref().unwrap().as_ref().as_configuration().unwrap().edges(),
            &IdentifierList::from([
                CompoundIdentifier::new_minimal(Identifier::Basic(String::from("HA_Comp_Entity"))), 
                CompoundIdentifier::new_minimal(Identifier::Basic(String::from("HA_Comp_Entity2")))
            ]));
    }

    #[test]
    fn configuration_spec() {
        let s = r#"
for L1: XOR_GATE use entity WORK.XOR_GATE(Behavior) -- or L1 = 'others' = 'L1, L2, ...' = 'all'
        generic map (3 ns, 3 ns)
        port map (I1 => work.lab1_pkg.MAGIC_NUM, I2 => I2, O => O);    
"#;
        let tokens = VHDLTokenizer::from_source_code(&s).into_tokens();
        let mut iter = tokens.into_iter().peekable();
        let st = VHDLSymbol::parse_statement(&mut iter);
        let iden = VHDLSymbol::parse_configuration_spec(st);
        assert_eq!(iden.unwrap(), IdentifierList::from([
            CompoundIdentifier::new(Identifier::Basic(String::from("WORK")), Identifier::Basic(String::from("XOR_GATE"))),
            CompoundIdentifier::new_minimal(Identifier::Basic(String::from("XOR_GATE"))),
            CompoundIdentifier::new(Identifier::Basic(String::from("work")), Identifier::Basic(String::from("lab1_pkg"))),
            CompoundIdentifier::new(Identifier::Basic(String::from("lab1_pkg")), Identifier::Basic(String::from("MAGIC_NUM"))),
        ]));

        let s = r#"
for all: xor_gate use configuration cfg1;    
"#;
        let tokens = VHDLTokenizer::from_source_code(&s).into_tokens();
        let mut iter = tokens.into_iter().peekable();
        let st = VHDLSymbol::parse_statement(&mut iter);
        let iden = VHDLSymbol::parse_configuration_spec(st);
        assert_eq!(iden.unwrap(), IdentifierList::from([
            CompoundIdentifier::new_minimal(Identifier::Basic(String::from("cfg1")))
            ]));
    }

    #[test]
    fn playground_fn_in_arch_dec() {
        let s = r#"
--------------------------------------------------------------------------------
--! Project   : crus.eel5721.lab1
--! Engineer  : Chase Ruskin
--! Created   : 2022-09-06
--! Testbench : fib_tb
--! Details   :
--!     Tests the `fib` design entity for generating fibonacci sequence values.
--!     
--!     The range of values tested are 0 <= n < 47. 47 is omitted from the range
--!     due to complications loading the resulting value into a 32-bit vector.
--!
--!     Uses file i/o to load a text file "inputs.dat" of randomized `n` values 
--!     for input to be checked against the expected results for fib(n) loaded 
--!     from another text file "outputs.dat".
--------------------------------------------------------------------------------
library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;
use std.textio.all;

entity fib_tb is 
end entity fib_tb;


architecture sim of fib_tb is
    --! internal testbench signals
    signal clk: std_logic := '0';
    signal rst: std_logic := '0';
    -- the simulation will halt when `halt` becomes '1'
    signal halt: std_logic := '0';

    constant period: time := 10 ns;

    --! unit-under-test (UUT) interface wires
    signal go     : std_logic;
    signal n      : std_logic_vector(5 downto 0);
    signal result : std_logic_vector(31 downto 0);
    signal done   : std_logic;

    impure function read_int_to_logic_vector(
        file f : text;
        len : in positive
    )
    return std_logic_vector is
        variable text_line : line;
        variable text_int  : integer;
    begin
        readline(f, text_line);
        read(text_line, text_int);
        return std_logic_vector(to_unsigned(text_int, len));
    end function;

begin
    --! UUT instantiation
    UUT : entity work.fib
    port map (
        clk    => clk,
        rst    => rst,
        go     => go,
        n      => n,
        result => result,
        done   => done
    );

    --! generate clock with 50% duty cycle
    clk <= not clk after period/2 when halt = '0';

    --! initial reset to start from known state
    boot: process
    begin
        rst <= '1';
        wait for period*4;
        rst <= '0';
        wait;
    end process;

    --! feed inputs into the UUT to begin processing
    input: process
        file inputs: text open read_mode is "inputs.dat";
        variable text_line : line;
        variable text_int  : integer;
    begin
        wait until rst = '0';

        while not endfile(inputs) loop
            -- drive the current input `n`
            readline(inputs, text_line);
            read(text_line, text_int);
            n <= std_logic_vector(to_unsigned(text_int, 6));

            -- assert the go signal to begin the simulation
            go <= '1';
            wait until rising_edge(clk);
            go <= '0';
            n <= (others => '0');
            wait until rising_edge(clk);

            -- assert that done has been set low
            assert done = '0' report "Done is not lowered during computation." severity error;

            wait until done = '1';

            -- allow 2 cycles to check done remaining asserted in bench process
            wait until rising_edge(clk);
            wait until rising_edge(clk);
        end loop;
        wait;
    end process;

    --! assert the received outputs match expected model values
    bench: process
        file outputs: text open read_mode is "outputs.dat";
        variable text_line    : line;
        variable text_int     : integer;
        variable ideal_result : std_logic_vector(31 downto 0);
        variable ideal_result_us : unsigned(31 downto 0);
    begin
        wait until rst = '0';

        while not endfile(outputs) loop
            readline(outputs, text_line);
            read(text_line, text_int);
            ideal_result := std_logic_vector(to_unsigned(text_int, 32));

            -- wait until done is asserted
            wait until done = '1';

            assert ideal_result = result report "Mismatched result." severity error;
            
            -- assert done remains '1'
            wait until rising_edge(clk);
            assert done = '1' report "Done is lowered too soon." severity error;        
        end loop;
        -- stop the simulation
        halt <= '1';
        report "Simulation complete.";
        wait;
    end process;

end architecture sim;
        "#;

        use crate::core::lang::vhdl::symbol::Identifier::Basic;

        let syms = VHDLParser::parse(VHDLTokenizer::from_source_code(&s).into_tokens());
        println!("{:?}", syms);
        assert_eq!(
            syms.get(1).as_ref().unwrap().as_ref().unwrap().as_ref().as_architecture().unwrap().edges(), 
            &LinkedList::from([
                CompoundIdentifier { prefix: Some(Basic("work".to_string())), suffix: Basic("fib".to_string()) }, 
                CompoundIdentifier { prefix: None, suffix: Basic("fib".to_string()) }])
        );
    }

    #[test]
    #[ignore]
    fn parse_basic() {
      let s = "\
-- design file for a nor_gate
library ieee;
use ieee.std_logic_1164.all;

entity nor_gate is -- comment on this line
    generic(
        N: positive
    );
    port(
        a : in std_logic_vector(N-1 downto 0);
        b : in std_logic_vector(N-1 downto 0);
        c : out std_logic_vector(N-1 downto 0)
    );
begin
end entity nor_gate;

architecture rtl of nor_gate is
    constant GO_ADDR_MMAP:integer:=2#001_1100.001#E14;
    constant freq_hz : unsigned := 50_000_000;
    signal   MAGIC_NUM_3 : bit_vector(3 downto 0) := 0sx\"\";
    constant MAGIC_NUM_1 : integer := 2#10101#; -- test constants against tokenizer
    constant MAGIC_NUM_2 : std_logic_vector(7 downto 0) := 0; -- 8c\"11\";
begin
    c <= a nor \\In\\;

end architecture rtl; /* long comment */

entity nor_gate_tb is end;

architecture tb of nor_gate_tb is 
-- declare DUT component
component nor_gate 
	generic(
		N: positive
	);
	port(
		a: in  std_logic_vector(N-1 downto 0);
		b: in  std_logic_vector(N-1 downto 0);
		c: out std_logic_vector(N-1 downto 0)
	);
end component nor_gate;
begin 
	DUT : component nor_gate 
    generic map     (
		N   => N
	) port map(
		a => w_a,
		b => w_b,
		c => w_c
	);

end;

package P is

function F return INTEGER;
attribute FOREIGN of F: function is \"implementation-dependent information\"; 

end package P;

package outer is
    package inner is 
        component some_component is
        end component;
    end package;
end package;

entity X is
port (P1, P2: inout BIT); 
constant Delay: TIME := 1 ms;
begin
CheckTiming (P1, P2, 2*Delay); end X ;

package TimeConstants is 
constant tPLH : Time := 9 ns;
constant tPHL : Time := 10 ns;
constant tPLZ :  Time := 12 ns;
constant tPZL :  Time := 7 ns;
constant tPHZ :  Time := 8 ns;
constant tPZH : Time := 8 ns;
end TimeConstants ;

package body TriState is

    package ent is 

    end;
    function BitVal (Value: Tri) return Bit is
        constant Bits : Bit_Vector := \"0100\"; 
    begin
        return Bits(Tri'Pos(Value)); 
    end;

    function TriVal (Value: Bit) return Tri is 
    begin
        return Tri'Val(Bit'Pos(Value)); 
    end;

    function Resolve (Sources: TriVector) return Tri is 
        variable V: Tri := 'Z';
    begin
        for i in Sources'Range loop
            if Sources(i) /= 'Z' then 
                if V = 'Z' then
                    V := Sources(i);
                else 
                    return 'E';
                end if; 
            end if;
        end loop; 
        return V;
    end;

end package body TriState;

architecture test of nor_gate is 

begin 

GEN_ADD: for I in 0 to 7 generate

LOWER_BIT: if I=0 generate
  U0: HALFADD port map
     (A(I),B(I),S(I),C(I));
end generate LOWER_BIT;

UPPER_BITS: if I>0 generate
  UX: FULLADD port map
     (A(I),B(I),C(I-1),S(I),C(I));
end generate UPPER_BITS;

end generate GEN_ADD;

end architecture;

architecture rtl of complex_multiplier is
begin

    mult_structure : case implementation generate 
        when single_cycle => 
            signal real_pp1, real_pp2 : ...;
            ...;
            begin
                real_mult1 : component multiplier
                    port map ( ... ); 
            end;
        when multicycle =>
            signal real_pp1, real_pp2 : ...;
            ...;
            begin
                mult : component multiplier
                    port map ( ... );
            end;
        when pipelined => 
            mult1 : component multiplier
                port map ( ... );
    end generate mutl_structure;

end architecture rtl;
";
        let _ = VHDLParser::parse(VHDLTokenizer::from_source_code(&s).into_tokens());
        panic!("manually inspect token list")
    }
}