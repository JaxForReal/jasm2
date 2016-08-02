# Jasm2
Compiled with `rustc 1.11.0-nightly (ad7fe6521 2016-06-23)`

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
- [ ] imports or links to other jasm files
- [ ] syscall for reading environment arguments
- [x] gui syscalls (see below, doing graphics output instead)
- [ ] Ability to package as exe (with interpreter included?)
- [ ] different arrow syntax
- [ ] New name to not conflict with JVM Bytecode Assembler
- [ ] ability to define constants, and other preprocessor directives
- [ ] A stdlib written in jasm, for pushing and popping on a stack, and string printing options
- [x] Ability to give a printStream to vm, so it can output to tests or stdout
- [ ] Graceful panics when parsing fails
- [ ] memory mapping for graphics output
- [ ] memory mapping for mouse location, keys pressed, other SDL events
- [ ] syscalls for input events

# Language
## Values
A value can be either an address or a number.  
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
* add
* sub
* mul
* div
* and
* or
* xor
* leftshift
* rightshift
* compare

### Syntax
```
operator value value -> value
```
NOTE: the destination (after the arrow) is a memory address. So to store a result to memory cell #3, you would write `add 4 5 -> 3`


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

##Labels
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


## Memory usage conventions
### cells 0-9:
Function arguments, assumed to be destroyed on function call or syscall also used by functions to return data.


## Syscalls
### Syntax
```
syscall name
```
#### print
Prints the value in address 0
#### print_char
prints the value in address 0 as a utf-8 char
#### read
reads stdin as a u32, and saves to address 0
#### read_string
read a string from stdin, and saves it as a string of chars, starting at the pointer defined in address 0. Similar to ```data `string` -> poniter```, but at runtime. for example, if the value 5 was stored in @0, then the char #0 of the string would be at @5, char # 1 would be at @6, etc...
#### read_char
reads the first character of stdin, and converts it to a u32, stored at @0. For example, inputting 'a' will result in 97 being stored at @0.

## Data in code
### Syntax
```
data value value value value -> value
```
Adds any number of values to memory in sequence, starting at destination.  
  
Or
```
data `string` -> value
```
Adds each character of the string in a new memory cell, starting at destination. characters are utf-8 encoded, as per rust standard.

### Example
```
data 5 3 48 5 9 1 -> 0
```
will set @0 to 3, @1 to 3, @2 to 48, etc.  
  
```
data `qwerty` -> 10
```
will set @0 113, @1 to 119, etc...




<test>