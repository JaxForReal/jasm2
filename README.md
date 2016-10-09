# Jasm2
Compiled with `rustc 1.14.0-nightly (6e8f92f11 2016-10-07)`  
A virtual machine that interprets a custom assembly dialect.
See [documentation](DOCUMENTATION.md) for more info on the assembly language.

```
USAGE:
    jasm2 [OPTIONS] <filename>
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
OPTIONS:
    -e, --emit <emit_type>          [values: preprocessed, parsed]
    -l, --log-level <log_level>     [default: info]  [values: off, error, warn, info, debug, trace]
    -s, --string <string>           Runs the string as a JASM file                 
ARGS:
    <filename>    The file to run.
```