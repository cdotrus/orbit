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
    publish - post an ip to a channel

SYNOPSIS
    orbit publish [options]

DESCRIPTION
    Performs a series of checks on a local ip and then releases it to its specified
    channel(s).
    
    There are multiple checks that are performed before an ip can be published. The
    ip must have an up to date lockfile with no relative dependencies. The ip's
    manifest must also have a value for the source field. Lastly, Orbit must be
    able to construct the hdl source code graph without errors.
    
    Posting an ip to a channel involves copying the ip's manifest file to a path 
    within the channel known as the index. For every publish of an ip, the index 
    corresponds to a unique path within the channel that gets created by Orbit.
    
    By default, it operates a dry run, performing all steps in the process except
    for posting the ip to its channel(s). To fully run the command, use the
    '--ready' flag. When the ip is published, it will also be installed to the cache
    by default. To skip this behavior, use the '--no-install' flag.

OPTIONS
    --ready, -y
        Perform a full run

    --list
        View available channels and exit

EXAMPLES
    orbit publish
    orbit publish --ready
"#;
