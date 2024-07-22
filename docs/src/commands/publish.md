# __orbit publish__

## __NAME__

publish - post an ip to a channel

## __SYNOPSIS__

```
orbit publish [options]
```

## __DESCRIPTION__

Performs a series of checks on a stable version of a local ip to then release it
through a channel.

For an ip to be published, it must have its source field defined that directs to
a valid internet location.

By default, it operates a dry run, performing all steps in the process except
for the actual release through the channel. To fully run the command, use the
`--ready` flag. When the ip is published, it will also be installed to the cache
by default. To skip this behavior, use the `--no-install` flag.

## __OPTIONS__

`--ready, -y`  
      Perform a full run

`--no-install`  
      Skip installing the ip

`--list`  
      View available channels and exit

## __EXAMPLES__

```
orbit publish
orbit publish --ready
```
