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

// This manual page was automatically generated from the mangen.py tool.
pub const MANUAL: &str = r#"NAME
    view - display metadata of an ip

SYNOPSIS
    orbit show [options] [<ip>]

DESCRIPTION
    This command retrieves various pieces of information about a particular ip to
    gain a better understanding of how to utilize the ip. By default, it displays
    the ip's manifest, if and only if the ip is able to be located.
    
    It will first attempt to return the information from a possible installation. If
    one does not exist, then it searches the downloads location for the ip.
    
    If '--units' is specified, then a list of the ip's HDL units are displayed.
    
    If '--versions' is specified, then a list of the ip's already available versions
    are displayed.
    
    If no spec is provided for '<ip>', then it will retrieve information based on the
    local ip, if one exists.

OPTIONS
    <ip>
        The spec of the ip to query

    --versions, -v
        Display the list of possible versions

    --units, -u
        Display the list of HDL primary design units associated with this ip

    --all, -a
        Include any private or hidden results

EXAMPLES
    orbit view --units
    orbit view gates:1.0.0 -u
    orbit view gates --versions
"#;
