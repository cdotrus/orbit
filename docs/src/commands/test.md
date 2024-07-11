# __orbit test__

## __NAME__

test - run a test

## __SYNOPSIS__

```
orbit test [options] [--] [args]...
```

## __DESCRIPTION__

This command prepares a given target and then executes the target.

While this command functions similar to `orbit build`, the targets that are 
encouraged to be used with this command are ones that are designed to either
"pass" or "fail", typically through a return code. This command requires a
testbench, if you do not want to set a testbench, see `orbit build`.

A target must be provided for the test command to run. A default target can
be specified in a configuration file, which will be used when a target is
omitted from the command-line.

If `--list` is used, then it will display a list of the available targets to
the user. Using `--list` in combination with a target from `--target` will
display any detailed help information the target has documented in its 
definition.

A target typically goes through three steps for the testing process:  
   1. Parse the blueprint file  
   2. Process the referenced files listed in the blueprint  
   3. Verify the hdl source code passes all tests

Any command-line arguments entered after the terminating flag `--` will be
passed in the received order as arguments to the subprocess's command. If a 
target already has defined arguments, the additional arguments passed from the 
command-line will follow the previously defined arguments.

The target's process will spawn from the current working ip's output directory,
which is $ORBIT_TARGET_DIR/$ORBIT_TARGET.

## __OPTIONS__

`--target <name>`  
      Target to execute

`--dut <unit>`  
      Set the device under test

`--tb <unit>`  
      Set the top level testbench unit

`--plan <format>`  
      Set the blueprint file format

`--target-dir <dir>`  
      The relative directory where the target starts

`--command <path>`  
      Overwrite the target's command

`--list`  
      View available targets and exit

`--all`  
      Include all hdl files of the working ip

`--fileset <key=glob>...`  
      A glob-style pattern identified by name to include in the blueprint

`--force`  
      Force the target to execute 

`--verbose`  
      Display the command being executed

`args`  
      Arguments to pass to the target

## __EXAMPLES__

```
orbit test --dut adder --tb adder_tb --target modelsim -- --lint
```

