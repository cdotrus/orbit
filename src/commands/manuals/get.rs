// This manual page was automatically generated from the rsmangen.py tool.
pub const MANUAL: &str = "\
NAME
    get - pull in an hdl entity to use

SYNOPSIS
    orbit get [options] <entity-path>

DESCRIPTION
    This command will add the requested ip as a dependency to the current 
    project. It will grab information about the primary design unit to copy and
    paste into the current project.
     
    If the ip pkgid is omitted, it will assume to search the current working ip
    for the requested unit. 
     
    If the --instance flag is used without the --component flag, it will
    display the direct instantiation style code (VHDL-93 feature).

OPTIONS
    <entity-path>  
          The pkgid and entity identifier to request [pkgid:]<entity>
     
    --component, -c  
          Display the component declaration
     
    --signals, -s  
          Display the corresponding signal declarations
     
    --instance, -i  
          Display the instance declaration
     
    --info  
          Display the code file's initial comment header block
     
    --architecture, -a
          Display a list of available architectures

EXAMPLES
    orbit get ks-tech.rary.gates:nor_gate -csi
";