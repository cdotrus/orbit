//
//  Copyright (C) 2022-2024  Chase Ruskin
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

// This help page was automatically generated from the mangen.py tool.
pub const HELP: &str = r#"View the dependency graph.

Usage:
    orbit tree [options]

Options:
    --root <unit>       uppermost hdl unit to starting the dependency tree
    --compress          replace duplicate branches with a referenced label
    --all               include all possible roots in tree
    --format <fmt>      select how to display unit nodes: 'long' or 'short'
    --ascii             restrict tree chars to the original 128 ascii set
    --ip                view the dependency graph at the ip level

Use 'orbit help tree' to read more about the command.
"#;
