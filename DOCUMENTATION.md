
### CLI
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

### General
---
3000 memory cells are provided for each jasm program. If in graphics mode, cells 1000 to 1400 are mapped to the display. Cells 2000 to 3000 are always used for the stack. Each cell is an unsigned 32 bit number. Instructions are not semicolon separated, separation is inferred by the parser.  
  
The `-e` or `--emit` flag decides what the program will output. If omitted, jasm will run the code. If `--emit parsed` is passed, jasm will output the abstract syntax list (try it to see what that is). `--emit preprocessed` will process all `#include`s and `#define`s and output the single file result.
### Value
----
* `[0-9]+` a decimal number literal number, eg. `234` or `34538473`
* `@value` a memory address pointed to by value.  Note, `value` can be any value, so the @ symbol can be recursive, eg. `@@@34` which is the value pointed to by the value pointed to by memory address 34.
* `b[0-1]+` a binary number, eg `b0100010011` or `b1`
* `'.'` the unicode character as a value, eg. `'a'` or `'%'`

### Instructions
----
Here, `value` refers to any valid value expression, and `dest` (destination) refers to a value that is used to point to a memory address, eg. `23` as a dest means store the result in address 24. `@1` as a dest means store the value in the memory pointed toby address 1.  
* `value -> dest` copies value into destination address
#### Math
All math instructions wrap without warning
* `add value value -> dest` add the two values and stores the result in dest
* `sub value value -> dest` multiply the two values
* `mul value value -> dest` multiply the two values
* `div value value -> dest` divide the two values
* `and value value ->  dest` binary AND the two values
* `or value value ->  dest` binary OR the two values
* `xor value value ->  dest` binary XOR the two values
* `leftshift value value ->  dest` left shift the first value by (second value) bits
* `rightshift value value ->  dest` right shift the first value by (second value) bits
* `invert value -> dest` binary invert the value

Eg. `add @3 34 -> 4` or `and @0 b00001111 -> 0`

#### Control Flow
labelname is defined as `[a-z_]+`.
* `jump equal value value labelname` jumps to labelname if the values are equal
* `jump notequal value value labelname` if values are not equal
* `jump less value value labelname` if first value is less than second value
* `jump greater value value labelname` if first value is greater than second value
* `jump labelname` unconditionally jump to labelname
* `<labelname>` define a label at this locations
* `call labelname` jump to label (and return on `ret` instruction)
* `ret` jump back to caller

#### Stack
Stack pointer is at address 2000, so to get the value of the stack pointer, use `@2000`. The stack pointer initially points to address 3000, which means the first value to be pushed willl reside at 2999, the next at 2998, etc. Popping from an empty stack, and pushing to a full stack (1000 elements) will both result in panic.  
This means you can peek at the top of stack without popping with `@@2000`. Yuo can also manually push data to the stack by copying it to the correct memory adresses, and then decrementing the stack pointer with `sub @2000 decrement_amount -> 2000`
* `push value` push value onto stack
* `pop -> dest` pop value into destination

#### Data
* `data (value )+ -> dest` Stores any number of values starting at dest (one memory cell each), Eg. `data 5 'A' 48 b1011 9 1 -> 5`
* ``data `string` -> dest`` stores each character to its own cell starting at dest

### Syscalls
#### In any mode
* `syscall set_mode` (argument in cell 0) 0 means change to console mode, anything else is graphics mode. You can go from console mode to graphics mode, but not back. Eg. to set graphics mode: `1 -> 0 syscall set_mode` (only in interpreters with "graphics" feature)
* `syscall delay` gets the value in cell 0, and pauses for that many milliseconds
#### Console mode only
* `syscall print` Prints the value in address 0
* `syscall print_char` Prints the value in address 0 as a utf-8 char
* `syscall print_binary` Prints the value in address 0 as a binary string eg. "10110100001"
* `syscall read` Reads stdin as a u32, and saves to address 0. (Destroys @0)
* `syscall read_string` Read a string from stdin, and saves it as a string of chars, starting at the pointer defined in address 0. Similar to ```data `string` -> poniter```, but at runtime. for example, if the value 5 was stored in @0, then the char #0 of the string would be at @5, char # 1 would be at @6, etc...
* `syscall read_char` Reads the first character of stdin, and converts it to a u32, stored at @0. For example, inputting 'a' will result in 97 being stored at @0. (Destroys @0)
#### Graphics mode only (only in interpreters with "graphics" feature)
* `syscall render_graphics` Draws cells 1000-1400 to the screen.

### Display
----
Jasm allows you to write to a 20x20 black and white display. Memory locations 1000 to 1400 are automatically mapped to the display, and can be drawn to the screen with `syscall render_graphics`. A value of zero means the pixel is off, and other value means the pixel is on.

#### Preprocessor
----
Jasm includes a simple preprocessor that can define constants and include other files.  
`#define CONST_NAME value`  
This searches through the program, and replaces every match of `CONST_NAME` with `value`. NOTE: the entire program is searched, so constants before the `#define` are also replaced. 
`#include "path/to/file.jasm"`  
Replaces this include text with the contents of file.jasm. Files are relative to the parent directory of the file being processed. For example if you are running `/home/me/main.jasm`, you can write `#include "other.jasm"`, and `/home/me/other.jasm` will be included into the file.  
**Example**
```
add PIN_NUMBER 1 -> 0
#define PIN_NUMBER 45
```
Will expand to
```
add 45 1 -> 0
```

#### Comments
----
`add 2 3 -> 4 //can be in this form`  
`add 2 /*or this form*/ 3 -> 4`  
`/*comments*/` can be multi-line

#### Example
----
```
#define GRAPHICS_MODE 1

/*
put into graphics mode
the mode argument is taken in address 0
*/
GRAPHICS_MODE -> 0
syscall set_mode

/*
start at address 1000, the start of the screen virtual memory
this is the pointer to the cell we will change
*/
1000 -> 1 //set memory address 1 to value 1000

<loop_start>
    1 -> @1 //turn pixel on
    add @1 3 -> 1 //advance 3 pixels
    jump less @1 1400 loop_start //if the pointer is less than 1400, goto the start of the loop

// render the graphics
syscall render_graphics

// wait for 2 seconds
2000 -> 0
syscall delay
```