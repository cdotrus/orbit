//
//  Copyright (C) 2022-2025  Chase Ruskin
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.
//

// This manual page was automatically generated from the mangen.py tool.
pub const MANUAL: &str = r#"NAME
    tree - show the dependency graph

SYNOPSIS
    orbit tree [options] [<unit>...]

DESCRIPTION
    Shows the hierarchical tree structure of the hardware design starting from a
    root node.
    
    By default, it will try to automatically detect the root node for the 
    local ip. If there is ambiguity in determining what node can be the root, then 
    all root nodes and their respective trees will be displayed. To only display
    the tree of a particular node, use the '<unit>' option.
    
    The tree can display different kinds of dependencies relative to the current
    ip using the '--edges' option. By default, this command uses "unit". By
    specifying edges as "ip", it will return the ip-level dependency tree. When
    using "unit" or "all", the hdl dependency graph will be displayed. The hdl
    graph shown with "unit" displays the composition of usable entities/modules. 
    To generate this graph, it analyzes each VHDL architecture and ignores Verilog 
    compiler directives. If an unidentified entity is instantiated, it will appear 
    as a leaf in the graph and will be considered as a "black box" denoted by 
    the "?" character next to its position in the tree. The hdl graph shown with
    "all" displays the composition of the design including all primary design unit
    references. Any references (excluding entity instantiations) that are not 
    found will not appear in the dependency graph for the "all" option.
    
    Using the '--format' option can alter how much information is displayed for
    each hdl design unit in the tree composition. By default, only the design
    unit's name is displayed for each unit.
    
    To display the ip dependency graph, use the '--ip' option.
    
    If the tree's character output is not displaying properly, then the tree can
    be displayed using a set of standard ASCII characters with the '--ascii'
    option.

OPTIONS
    <unit>...
        Uppermost hdl unit of the dependency tree

    --edges, -e <kind>
        The kind of dependencies to display (unit, ip, all)

    --format <fmt>
        Determine how to display nodes (long, short)

    --ascii
        Limit the textual tree characters to the 128 ascii set

EXAMPLES
    orbit tree
    orbit tree top --format long
    orbit tree -e ip --ascii
"#;
