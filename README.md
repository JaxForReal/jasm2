# Jasm2
Compiled with `rustc 1.12.0-nightly (0ef24eed2 2016-08-10)`

## ToDo

- [x] Branching Syntax
- [x] How to handle nested functions and declarations?
- [x] How to pass data to syscalls without hardcoded argument addresses?
- [x] Retrieve data at non-32bit addresses, between cells (sort of done with bit operations)
- [x] Consider using i32 instead of u32 for memory cells (decided against)
- [x] Write tests for parser
- [ ] Write tests for Vm
- [x] Fix enumerate() not working when building function_table and data operations
- [x] Remove need for semicolons at end of lines
- [x] ASCII value types
- [ ] New parser for a machine syntax that is easy to compile to (but less readable)
- [x] imports or links to other jasm files
- [ ] syscall for reading environment arguments
- [x] gui syscalls (see below, doing graphics output instead)
- [ ] Ability to package as exe (with interpreter included?)
- [ ] different arrow syntax
- [ ] New name to not conflict with JVM Bytecode Assembler
- [ ] ability to define constants, and other preprocessor directives
- [ ] A stdlib written in jasm, for pushing and popping on a stack, and string printing options
- [x] Ability to give a printStream to vm, so it can output to tests or stdout
- [ ] Graceful panics when parsing fails
- [x] Graceful panics on vm error
- [x] memory mapping for graphics output
- [ ] memory mapping for mouse location, keys pressed, other SDL events
- [ ] syscalls for input events
- [x] ability to run in either console or graphics mode
- [ ] convert all print_XXXX syscalls to a single print syscall

# Documentation
## General
An array of 2048 memory cells are provided for your program. Each cell is an unsigned 32 bit number. Commands are not comma separated, command separation is inferred. This allows the convienence of two commonly grouped commands on one line eg `1 -> 0 syscall setmode`. 
## Values
A value can be either an address to a cell (a pointer) or a number.  
### Addresses
`@2` gives the value of memory cell #2  
`@@5` gives the value of the memory cell at (the value of memory cell 5)  
  
Unlimited levels of pointer indirection are supported. Ex `@@@@@23` is a valid value.
### Numbers
`259` the decimal value 259.  
`b00101` the binary value (decimal 5). Any number of 1s and 0s are supported: `b1` is valid, as is `b100101101110`.  
`'a` the utf-8 value of character 'a' (decimal 97).

## Binary Operators
currently implemented operators:
* `add`
* `sub`
* `mul`
* `div`
* `and`
* `or`
* `xor`
* `leftshift`
* `rightshift`
* `compare`

### Syntax
```
operator value value -> value
```
NOTE: the destination (after the arrow) is a memory address. So to store a result to memory cell #3, you would write `add 4 5 -> 3`. To store in the cell that cell 4 points to, use `add 4 5 -> @4`. This is a bit counterintuitive, but makes the language more uniform, because values are accepted anywhere in syntax.


### Example
```
add @5 @6 -> 7
```
Gets the values at memory cell #5 and #6, add them, and stores the result in address 7

## Unary Operators
currently only one unary operator:
* invert

### Syntax:
```
operator value -> value
```

## Functions
### Syntax
Defining a function
```
<func_name>
//commands
//more commands
ret
```

Calling a function syntax
```
call func_name
```

## Labels
### Syntax
```
<_label_name>
//commands
```
Labels are defined in the same way as functions, but you should prepend label names with an underscore to differentiate them from functions. Jumping to labels is done with the `jumpzero value label` and`jumpnotzero value label` commands. There is no unconditional jumping, instead use `jumpzero 0 label`.

## Compare Operator
Compares two values and returns a flags.

### Syntax
```
compare value value -> value
```
### Example
```
compare @5 56 -> 6
```

Compare can also be used in conjuntion with `jumpnotzero` to jump based on the relation between two values.
```
<start>
//start with these values
25 -> 0
27 -> 1

//compare and store the flags to cell #3
compare @0 @1 -> 3
//use a bit AND mask to isolate the "less than" flag
and @3 b00000100 -> 3
jumpnotzero @3 _comparison_was_true
<_comparison_was_false>
//do stuff

<_resume_exec>
ret

<_comparison_was_true>
//do stuff
jumpzero 0 _resume_exec
```

### compare flags
Least significant end of value
* 0: equal
* 1: not equal
* 2: less than
* 3: greater than 
* 4: less than or equal
* 5: greater than or equal  

Most significant end of value

## Syscalls
### Syntax
```
syscall name
```

#### In any mode (Console or graphics)

- **set_mode** 
sets the mode based on value in cell 0. 0 means console mode, anything else is graphics mode. You can go from console mode to graphics mode, but not back. eg. to set graphics mode: `1 -> 0 syscall set_mode`

- **delay**
gets the value in cell 0, and pauses for that many milliseconds (uses Sdl2 timer subsystem)

#### Console mode only:


- **print**
Prints the value in address 0

- **print_char**
Prints the value in address 0 as a utf-8 char

- **print_binary**
Prints the value in address 0 as a binary string eg. "10110100001"

- **read**
Reads stdin as a u32, and saves to address 0. (Destroys @0)

- **read_string**
Read a string from stdin, and saves it as a string of chars, starting at the pointer defined in address 0. Similar to ```data `string` -> poniter```, but at runtime. for example, if the value 5 was stored in @0, then the char #0 of the string would be at @5, char # 1 would be at @6, etc...

- **read_char**
Reads the first character of stdin, and converts it to a u32, stored at @0. For example, inputting 'a' will result in 97 being stored at @0. (Destroys @0)

#### Graphics mode only:

- **render_graphics**
Draws cells 1000-1400 to the screen.



## Data in code
### Syntax
```
data value value value value -> value
```
Adds any number of values to memory in sequence, starting at destination. Or:
```
data `string` -> value
```
Adds each character of the string in a new memory cell, starting at destination. characters are utf-8 encoded, as per rust standard.

### Example
```
data 5 'A 48 b1011 9 1 -> 0
```
Will set @0 to 5, @1 to 97, @2 to 11, etc.  
  
```
data `qwerty` -> 10
```
Will set @10 113, @11 to 119, etc...


## Display
Jasm allows you to write to a 20x20 black and white display
### Writing
Memory locations 1000 to 1400 are automatically mapped to the display, and can be drawn to the screen with `syscall render_graphics`  
A value of zero means the pixel is off, and other value means the pixel is on.