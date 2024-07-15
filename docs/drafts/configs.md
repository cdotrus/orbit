<!--
## config.toml

The first config file you may come across is `config.toml`. This file is used to load initial startup settings into orbit and customize a user's program experience.

Here is a very minimal and basic example config file:
``` toml
include = ["profiles/ks-tech/config.toml"]

[env]
QUARTUS_PATH = "C:/IntelFPGA_lite/19.1/quartus/bin64"

[[plugin]]
alias = "zipr"
description = "Compress files into a submission-like format"
command = "python"
args = ["./main/plugins/zipr.py"]
fileset.zip-list = "submission.txt"

[[protocol]]
name = "zip-op"
description = "Handle zip file urls"
command = "python"
args = ["./main/protocols/download.py"]
```

The __home configuration__ is the config.toml file located at your $ORBIT_HOME path.

If you have `cat` installed, you can view your home config file in the console:
```
$ cat "$(orbit env ORBIT_HOME)/config.toml"
```

> __Tip:__ You can modify some values in the configuration file through the command-line by using the `orbit config` command.

## Paths

When specifying a value that is known to be a path, Orbit supports resolving relative paths in relation to the config.toml's path it is currently reading. This allows for a path value to be correct out-of-the-box across users and machines when sharing configurations.

## Precedence

Orbit supports multiple levels of configuration. The order of precedence:

1. local configuration file (located in current IP)

2. global configuration file (located in $ORBIT_HOME)

3. configuration files listed in `include` entry (last has higher precedence than first)

A key's value is overridden upon a configuration file of higher precedence also setting a previously defined key from a lower-precedence file.

## Entries

The following is a list of acceptable entries (key/value pairs) recognized by Orbit in configuration files (`config.toml`).


### `include` : _list_ of _string_
- paths to other configurations files to load before the home configuration
- only supported in the home configuration file

``` toml
include = ["profiles/ks-tech/config.toml"]
```

### `[env]` : _table_
- user-defined additional keys to set as runtime environment variables during build phase
- the following example would set an environment variable ORBIT_ENV_VAR_1 as "100" during runtime

``` toml
[env]
VAR_1 = "100"
# ...
```

<!-- 
### `core.build-dir` : _string_
- directory to create to save blueprint file to
- default is "build"

``` toml
[core]
build-dir = "target"
# ...
```

### `core.user` : _string_
- your name
- useful for template variable substitution

``` toml
[core]
user = "Kepler [KST-001]"
# ...
```

### `core.date-fmt` : _string_
- date formatting for template variable substitution
- default is `"%Y-%m-%d"`
- see chrono's [documentation](https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html#specifiers) for complete list of formatting specifiers

``` toml
[core]
date-fmt = "%B %e, %Y" # July 8, 2001
# ...
``` 

### `[[plugin]]` : _array of tables_
- `alias` : _string_ 
    - plugin name to reference when invoking
    - required
- `command` : _string_
    - first argument to pass to subprocess
    - required
- `description` : _string_
    - short description about the plugin
- `args` : _array_ of _string_
    - additional arguments to follow command in subprocess  
- `fileset` : _inline table_
    - user-defined additional keys to store glob-style file patterns
- `explanation` : _string_
    - long description about the plugin

``` toml
[[plugin]]
alias   = "vvd"
command = "vivado"
description = "Basic toolflow for Vivado Design Suite"
args    = ["-mode", "batch", "-source", "script.tcl"]
fileset.EDA-FLOW    = "*.tcl"
fileset.CONSTRAINTS = "*.xdc"
explanation = """\
    This plugin runs Vivado in non-project mode to perform its tasks.

Usage:
    orbit build --plugin vvd -- [options]

Options:
    -tclarg mode=<num>      0 - synth, 1 - impl, 2 - bit

Environment:
    ORBIT_ENV_VIVADO_PATH   Local path to Vivado binaries   

Dependencies:
    Vivado Design Suite (tested: 2019.2)
"""
```

### `[[protocol]]` : _array of tables_
- `name` : _string_ 
    - protocol name to reference in an IP's manifest
    - required
- `command` : _string_
    - first argument to pass to subprocess
    - required
- `description` : _string_
    - short description about the protocol
    - optional
- `args` : _array_ of _string_
    - additional arguments to follow command in subprocess 
    - optional 
- `explanation` : _string_
    - long description about the protocol
    - optional

``` toml
[[protocol]]
name = "git-op"
description = "Fetch remote repositories using git"
command = "git"
args = ["clone", "-b", "{{ orbit.ip.version }}", "{{ orbit.ip.source.url }}", "{{ orbit.queue }}/{{ orbit.ip.name }}"]
explanation = """\
This protocol tries to clone a repository defined under the source URL at a tag 
matching the IP's version.

Examples:
    [ip]
    # ...
    name = "lab1"
    version = "1.0.0"
    source = { protocol = "git-op", url = "https://github.com/path/to/lab1.git" }
    # ...

Dependencies:
    git (tested: 2.36.0)
"""
```
-->