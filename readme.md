# My Brainfuck interpreter

## What is Brainfuck

Brainfuck is a language designed to destroy your brain because of its stupid difficulty to code something 
simple. It uses an array of bytes to work with. Each character of the input file is a different instruction.
For example '>' signify to go to the next cell of the array, '<' to the previous, '+' increment the current
byte cell, '-' decrement it, '.' prints it to the stdout... For more info on the language see 
[the Wikipedia article about it](https://en.wikipedia.org/wiki/Brainfuck).

## My Brainfuck

My Brainfuck is a bit different, it has the vanilla Brainfuck but with added possibilities, like '*' that
returns the address of this slot in it, '#' able it to modify the current length of the input on stdin, 
'@' is the jump instruction that will jump the pointer to the location in the cell.
***NOTE : the current cell is the heigh byte and next is the low one***

## To run it

Please don't, it's good rust code but, please don't program in Brainfuck, if you want difficulty program in
assembly, but anyway if you want to test the interpreter just use **test.bf** which should print hello, world
at the screen.

You really want to suffer ? Ok, first compile with:
````
cargo build
````
Then run your program using the interpreter like so:
````
./target/debug/brainfuck.exe test.bf
````
Or just recompile the interpreter each time (not recommended):
````
cargo run test.bf
````