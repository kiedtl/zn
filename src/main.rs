use std::fs;
use std::env;
use std::process;
use std::collections::LinkedList;

mod parser;
mod lexer;
mod registers;
mod errors;
mod zn;

fn main() -> Result<(), Box<dyn std::error::Error + 'static>> {
    // memory buffer
    #[allow(non_snake_case)]
    let mut BUFFER: [Option<u8>; 65535] = [None; 65535];

    // registers
    #[allow(non_snake_case)]
    let REGSTR = registers::Registers {
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
    let DSTACK: LinkedList<usize> = LinkedList::new();

    // get arguments
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        println!("USAGE:\n\tzn [action] [file]");
        println!("ACTIONS:\n\trun\tExecute script.\n\tstat\tPrint statistics on script.");

        process::exit(1);
    }
    let file = &args[2];

    // file
    let contents = fs::read_to_string(file)?;
    let commands = lexer::lex(contents);
    if &args[1] == "run" {
        zn::execute(&mut BUFFER,
                    &mut REGSTR,
                    &mut DSTACK,
                    file.to_string(),
                    commands);
    } else if &args[1] == "stat" {
        println!("{}: {} commands", file, commands.len());
    } else {
        println!("ERROR: {} is not a valid action.", &args[1]);
    }

    Ok(())
}


