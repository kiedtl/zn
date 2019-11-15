use std::fs;
use std::env;
use std::process;
use std::collections::LinkedList;

mod lexer;
mod registers;
mod errors;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // memory buffer
    #[allow(non_snake_case)]
    let mut BUFFER: [Option<u8>; 65535] = [None; 65535];

    // registers
    #[allow(non_snake_case)]
    let mut REGSTR = registers::Registers {
                        r: 0,
                        i: 0,
                        c: 0,
                        h: 0,
                        x: 0,
                        a: 0,
                        d: 0,
                    };

    // double-ended pointer stack ;)
    #[allow(non_snake_case)]
    let mut DSTACK: LinkedList<usize> = LinkedList::new();

    // process file char by char
    // from stackoverflow :P
    // https://stackoverflow.com/questions/35385703/read-file-character-by-character-in-rust
    let mut lineno = 0;
    let mut charno = 0;

    // get arguments
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        println!("usage: zn [file]");
        process::exit(1);
    }
    let file = &args[1];

    // file
    let contents = fs::read_to_string(file)?;
    for command in lexer::lex(contents) {
        println!("token: {}", command);
    }

    Ok(())
}
