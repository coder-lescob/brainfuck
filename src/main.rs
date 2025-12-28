use std::fs;
use std::io;
use std::env;

fn main() {
    // get the name of the program
    let mut args: Vec<String> = env::args()
        .collect();
    if args.len() < 2 {
        println!("no input file given, defaulted to test.bf.");
        args.push(String::from("test.bf"));
    }
    let program_name      = &args[1];

    // read input file
    let contents = fs::read_to_string(program_name)
        .expect("unable to read input file.");

    let array_size = 100000;

    // create the array
    let mut array: Vec<u8> = Vec::with_capacity(array_size);
    unsafe  {
        array.set_len(array_size);
    }
    
    for i in 0..array.len() {
        array[i] = 0;
    }

    let mut ptr        : usize      = 0x00;
    let mut buffer                  = String::new();
    let mut while_stack: Vec<usize> = vec![];

    let mut i            : usize = 0x00;
    let mut max_input_len: usize = 0xFF; // 255

    // loop over each character
    while i < contents.len() {
        // get the letter
        let letter_wrapped: Option<char> = contents
            .chars()
            .nth(i);

        if letter_wrapped == None {
            i += 1;
            continue;
        }

        let mut letter = letter_wrapped.unwrap();

        match letter {
            '>' => ptr += 1,
            '<' => {
                if ptr > 0 {
                    ptr -= 1;
                }
                else {
                    ptr = array.len() - 1;
                }
            },
            // allow operations overflow
            '+' => array[ptr] = array[ptr].wrapping_add(1)  ,
            '-' => array[ptr] = array[ptr].wrapping_sub(1),
            '[' =>
                {
                    while_stack.push(i);

                    if array[ptr] == 0 {
                        let mut whiles: usize = 1;
                        while
                        i < contents.len()
                            &&
                            whiles > 0 {
                            i += 1;
                            letter = contents
                                .chars()
                                .nth(i)
                                .unwrap();

                            if letter == '[' {
                                whiles += 1;
                            } else if letter == ']' {
                                whiles -= 1;
                            }
                        }
                    }
                }
            ']' =>
                {
                    if while_stack.len() == 0 {
                        println!("no matching opening loop at char number {i}");
                        return;
                    }
                    i = while_stack.pop().unwrap();
                    continue;
                }
            '.' => print!("{}", array[ptr] as char),
            ',' => {
                // not implemented as the original who just get a char
                io::stdin().read_line(&mut buffer)
                    .expect("unable to find prompt");

                let mut j: usize = 0;

                while j < buffer.len() && j < max_input_len {
                    let letter_wrapped: Option<char> = buffer.chars().nth(j);
                    if letter_wrapped == None {
                        j += 1;
                        continue;
                    }

                    let letter = letter_wrapped.unwrap();
                    array[ptr + j] = letter as u8;
                    j += 1;
                }
            }
            // not in vanilla Brainfuck
            '*' =>
            // the * char returns the ptr in the slot the ptr is
                array[ptr] = ptr as u8,
            '#' => // allow only the number of chars in the current slot to be inputted by ,
            // NOTE : the default value is 255 chars
                max_input_len = array[ptr] as usize,
            '@' => // @ jumps the pointer to the address in the current slot and the next combined
            // NOTE : current is the heigh byte and next is the low one
                ptr = (array[ptr] as usize) << 8 | array[ptr + 1] as usize,
            _ => {/* do nothing ! */}
        }
        i += 1; // next char
    }
}