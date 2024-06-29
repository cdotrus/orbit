use super::plan::PlanError;
use crate::commands::helps::tree;
use crate::commands::plan::Plan;
use crate::core::algo;
use crate::core::algo::IpFileNode;
use crate::core::catalog::Catalog;
use crate::core::context::Context;
use crate::core::fileset;
use crate::core::ip::Ip;
use crate::core::lang::node::HdlNode;
use crate::core::lang::node::IdentifierFormat;
use crate::core::lang::node::SubUnitNode;
use crate::core::lang::parser::ParseError;
use crate::core::lang::vhdl::subunit::SubUnit;
use crate::core::lang::vhdl::symbols::entity::Entity;
use crate::core::lang::vhdl::symbols::CompoundIdentifier;
use crate::core::lang::vhdl::symbols::{VHDLParser, VhdlSymbol};
use crate::core::lang::vhdl::token::Identifier;
use crate::core::lang::Language;
use crate::util::anyerror::Fault;
use crate::util::graph::EdgeStatus;
use crate::util::graphmap::GraphMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use cliproc::{cli, proc, stage::*};
use cliproc::{Arg, Cli, Help, Subcommand};

#[derive(Debug, PartialEq)]
pub struct Tree {
    root: Option<Identifier>,
    compress: bool,
    format: Option<IdentifierFormat>,
    ascii: bool,
    ip: bool,
    all: bool,
}

impl Subcommand<Context> for Tree {
    fn interpret<'c>(cli: &'c mut Cli<Memory>) -> cli::Result<Self> {
        cli.help(Help::with(tree::HELP))?;
        Ok(Tree {
            compress: cli.check(Arg::flag("compress"))?, // @todo: implement
            ascii: cli.check(Arg::flag("ascii"))?,
            ip: cli.check(Arg::flag("ip"))?,
            all: cli.check(Arg::flag("all"))?,
            root: cli.get(Arg::option("root").value("unit"))?,
            format: cli.get(Arg::option("format").value("fmt"))?,
        })
    }

    fn execute(self, c: &Context) -> proc::Result {
        // go to the ip directory
        c.jump_to_working_ip()?;

        if self.compress == true {
            todo!("implement compression logic")
        }

        // get the ip manifest
        let ip = Ip::load(c.get_ip_path().unwrap().clone(), true)?;

        // gather the catalog
        let catalog = Catalog::new().installations(c.get_cache_path())?;

        self.run(ip, catalog, c.get_languages())
    }
}

impl Tree {
    fn run(&self, target: Ip, catalog: Catalog, mode: Language) -> Result<(), Fault> {
        match &self.ip {
            true => self.run_ip_graph(target, catalog, &mode),
            false => self.run_hdl_graph(target, catalog, &mode),
        }
    }

    /// Construct and print the graph at an HDL-entity level.
    fn run_hdl_graph(&self, target: Ip, catalog: Catalog, mode: &Language) -> Result<(), Fault> {
        let working_lib = Identifier::new_working();

        // build graph again but with entire set of all files available from all depdendencies
        let ip_graph = algo::compute_final_ip_graph(&target, &catalog, mode)?;
        let files = algo::build_ip_file_list(&ip_graph, &target);

        // build the complete graph (using entities as the nodes)
        let global_graph = Self::build_graph(&files)?;

        if self.all == false {
            let n = {
                // restrict graph to units only found within the current IP
                let local_graph = Plan::compute_local_graph(&global_graph, &target);

                let root_index = if let Some(ent) = &self.root {
                    // check if the identifier exists in the entity graph
                    let i = match local_graph
                        .get_node_by_key(&&CompoundIdentifier::new(working_lib, ent.clone()))
                    {
                        Some(id) => id.index(),
                        None => return Err(PlanError::UnknownEntity(ent.clone()))?,
                    };
                    Plan::local_to_global(i, &global_graph, &local_graph).index()
                // auto-detect the root if possible
                } else {
                    // check if --all is applied
                    // traverse subset of graph by filtering only for working library entities
                    match local_graph.find_root() {
                        Ok(i) => {
                            Plan::local_to_global(i.index(), &global_graph, &local_graph).index()
                        }
                        Err(e) => match e.len() {
                            0 => return Err(PlanError::Empty)?,
                            _ => {
                                return Err(PlanError::Ambiguous(
                                    "roots".to_string(),
                                    e.into_iter()
                                        .map(|f| {
                                            local_graph
                                                .get_node_by_index(f)
                                                .unwrap()
                                                .as_ref()
                                                .get_symbol()
                                                .as_entity()
                                                .unwrap()
                                                .get_name()
                                                .clone()
                                        })
                                        .collect(),
                                ))?
                            }
                        },
                    }
                };
                root_index
            };

            // display the root's tree to the console
            let tree = global_graph.get_graph().treeview(n);
            for twig in &tree {
                let branch_str = match self.ascii {
                    true => Self::to_ascii(&twig.0.to_string()),
                    false => twig.0.to_string(),
                };
                println!(
                    "{}{}",
                    branch_str,
                    global_graph
                        .get_node_by_index(twig.1)
                        .unwrap()
                        .as_ref()
                        .display(self.format.as_ref().unwrap_or(&IdentifierFormat::Short))
                );
            }
        } else {
            // restrict graph to units only found within the current IP
            let local_graph = Plan::compute_local_graph(&global_graph, &target);
            // compile list of all roots
            let mut roots = Vec::new();
            match local_graph.find_root() {
                Ok(i) => roots
                    .push(Plan::local_to_global(i.index(), &global_graph, &local_graph).index()),
                Err(e) => match e.len() {
                    0 => return Err(PlanError::Empty)?,
                    _ => e.into_iter().for_each(|f| {
                        roots.push(Plan::local_to_global(f, &global_graph, &local_graph).index())
                    }),
                },
            }

            // display each root's tree to the console
            roots.iter().for_each(|n| {
                let tree = global_graph.get_graph().treeview(*n);
                for twig in &tree {
                    let branch_str = match self.ascii {
                        true => Self::to_ascii(&twig.0.to_string()),
                        false => twig.0.to_string(),
                    };
                    println!(
                        "{}{}",
                        branch_str,
                        global_graph
                            .get_node_by_index(twig.1)
                            .unwrap()
                            .as_ref()
                            .display(self.format.as_ref().unwrap_or(&IdentifierFormat::Short))
                    );
                }
            });
        }

        Ok(())
    }

    /// Construct and print the graph at an IP dependency level.
    fn run_ip_graph(&self, target: Ip, catalog: Catalog, mode: &Language) -> Result<(), Fault> {
        let ip_graph = algo::compute_final_ip_graph(&target, &catalog, mode)?;

        let tree = ip_graph.get_graph().treeview(0);
        for twig in &tree {
            println!(
                "{}{}",
                twig.0,
                ip_graph
                    .get_node_by_index(twig.1)
                    .unwrap()
                    .as_ref()
                    .as_ip()
                    .get_man()
                    .get_ip()
                    .into_ip_spec()
            );
        }
        Ok(())
    }

    /// Converts the original treeview text from using extended ascii characters
    /// to orginal ascii characters.
    fn to_ascii(s: &str) -> String {
        let mut transform = String::with_capacity(s.len());
        let mut chars = s.chars();
        while let Some(c) = chars.next() {
            match c {
                '─' => transform.push('-'),
                '│' => transform.push('|'),
                '├' => transform.push('+'),
                '└' => transform.push('\\'),
                _ => transform.push(c),
            }
        }
        transform
    }

    /// Constructs a graph of the design heirarchy with entity nodes.
    fn build_graph<'a>(
        files: &'a Vec<IpFileNode>,
    ) -> Result<GraphMap<CompoundIdentifier, HdlNode<'a>, ()>, Fault> {
        // entity identifier, HashNode (hash-node holds entity structs)
        let mut graph = GraphMap::<CompoundIdentifier, HdlNode, ()>::new();

        let mut sub_nodes: Vec<(Identifier, SubUnitNode)> = Vec::new();
        // store the (suffix, prefix) for all entities
        let mut component_pairs: HashMap<Identifier, Identifier> = HashMap::new();

        let mut package_identifiers: HashSet<Identifier> = HashSet::new();
        // read all files
        for source_file in files {
            // skip files that are not VHDL
            if fileset::is_vhdl(&source_file.get_file()) == false {
                continue;
            }
            // parse VHDL code
            let contents = fs::read_to_string(&source_file.get_file()).unwrap();
            let symbols = match VHDLParser::read(&contents) {
                Ok(s) => s.into_symbols(),
                Err(e) => Err(ParseError::SourceCodeError(
                    source_file.get_file().clone(),
                    e.to_string(),
                ))?,
            };

            let lib = source_file.get_library();
            let lib = lib.as_vhdl_name().take().unwrap();

            // add all entities to a graph and store architectures for later analysis
            symbols.into_iter().for_each(|sym| match sym {
                VhdlSymbol::Entity(e) => {
                    component_pairs.insert(e.get_name().clone(), lib.clone());
                    graph.add_node(
                        CompoundIdentifier::new(lib.clone(), e.get_name().clone()),
                        HdlNode::new(VhdlSymbol::from(e), source_file),
                    );
                }
                VhdlSymbol::Architecture(arch) => {
                    sub_nodes.push((
                        lib.clone(),
                        SubUnitNode::new(SubUnit::from_arch(arch), source_file),
                    ));
                }
                VhdlSymbol::Configuration(cfg) => {
                    sub_nodes.push((
                        lib.clone(),
                        SubUnitNode::new(SubUnit::from_config(cfg), source_file),
                    ));
                }
                VhdlSymbol::Package(_) => {
                    package_identifiers.insert(sym.get_name().unwrap().clone());
                }
                _ => (),
            });
        }

        // go through all subunits and make the connections
        let mut sub_nodes_iter = sub_nodes.into_iter();
        while let Some((lib, node)) = sub_nodes_iter.next() {
            let node_name = CompoundIdentifier::new(lib, node.get_sub().get_entity().clone());

            // link to the owner and add subunit's source file
            // note: this also occurs in `plan.rs`
            let entity_node = match graph.get_node_by_key_mut(&node_name) {
                Some(en) => en,
                // @todo: issue error because the entity (owner) is not declared
                None => continue,
            };
            entity_node.as_ref_mut().add_file(node.get_file());
            // create edges by ordered edge list
            for dep in node.get_sub().get_edge_list() {
                // verify we are not a package (will mismatch and make inaccurate graph)
                if package_identifiers.contains(dep.get_suffix()) == true {
                    continue;
                }
                // need to locate the key with a suffix matching `dep` if it was a component instantiation
                if dep.get_prefix().is_none() {
                    if let Some(lib) = component_pairs.get(dep.get_suffix()) {
                        let b = graph.add_edge_by_key(
                            &CompoundIdentifier::new(lib.clone(), dep.get_suffix().clone()),
                            &node_name,
                            (),
                        );
                        match b {
                            // create black box entity
                            EdgeStatus::MissingSource => {
                                let dep_name =
                                    CompoundIdentifier::new(lib.clone(), dep.get_suffix().clone());

                                graph.add_node(
                                    dep_name.clone(),
                                    HdlNode::black_box(VhdlSymbol::from(Entity::black_box(
                                        dep.get_suffix().clone(),
                                    ))),
                                );
                                graph.add_edge_by_key(&dep_name, &node_name, ());
                            }
                            _ => (),
                        }
                    // this entity does not exist or was not logged
                    } else {
                        // create new node for black box entity
                        if graph.has_node_by_key(dep) == false {
                            graph.add_node(
                                dep.clone(),
                                HdlNode::black_box(VhdlSymbol::from(Entity::black_box(
                                    dep.get_suffix().clone(),
                                ))),
                            );
                        }
                        graph.add_edge_by_key(&dep, &node_name, ());
                    }
                // the dependency has a prefix (a library) with it
                } else {
                    // verify we are not coming from a package (will mismatch and make inaccurate graph)
                    if package_identifiers.contains(dep.get_prefix().unwrap()) == true {
                        continue;
                    }
                    graph.add_edge_by_key(dep, &node_name, ());
                };
            }
        }
        Ok(graph)
    }
}
