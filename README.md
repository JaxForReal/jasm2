# Jasm2
Compiled with `rustc 1.11.0-nightly (51d2d3da8 2016-06-12)`

## ToDo

- [x] Branching Syntax
- [x] How to handle nested functions and declarations?
- [x] How to pass data to syscalls without hardcoded argument addresses?
- [ ] Retrieve data at non-32bit addresses (between cells)
- [ ] Consider using i32 instead of u32 for memory cells
- [ ] Write tests
- [ ] Fix enumarate() not working when building function_table and data operations
- [x] Remove need for semicolons at end of lines
- [ ] ASCII value types
- [ ] New parser for a machine syntax that is easy to compile to (but less readable)
- [ ] imports or links to other jasm files
- [ ] syscall for reading arguments
- [ ] Ability to package as exe (with interpreter included?)
- [ ]

# Language
## Values
A value can be either an address or a number.  
### Addresses
`@2` gives the value of memory cell #2  
`@@5` gives the value of the memory cell at (the value of memory cell 5)  
  
Unlimited levels of pointer indirection are supported. Ex `@@@@@23` is a valid value.
### Numbers
`259` the decimal value 258  
`b00101` the binary value (decimal 5)  
any number of 1s and 0s are supported: `b1` is valid, as is `b100101101110`

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
Gets the values at memory cell #5 and 6, add them, and stores the result in address 7

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

## Compare Operator
Compares two values and returns a flags.

### Syntax
```
compare value value -> destination
```
### Example
```
compare @5 56 -> 6
```
#### compare flags
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
#### print_ascii
prints the value in address 0 as a utf-8 char


## Data in code
### Syntax
```
data value value value value -> destination
```
Adds any number of values to memory in sequence, starting at destination.

### Example
```
data 5 3 48 5 9 1 -> 0
```
will set @0 to 3, @2 to 3, @3 to 48, etc.
