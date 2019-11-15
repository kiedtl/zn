use std::env;
use std::fs::File;
use std::collections::LinkedList;
use std::io::{ BufRead, BufReader };

mod registers;
mod errors;

fn main() {
    // memory buffer
    let mut BUFFER: [u8; 65535] = [0; 65565];

    // registers
    let mut REGSTR = registers::Register {
                        r: 0,
                        i: 0,
                        c: 0,
                        h: 0,
                        x: 0,
                        a: 0,
                        d: 0,
                    };

    // double-ended pointer stack ;)
    let mut DSTACK: collections::LinkedList = LinkedList::new();

    // process file char by char
    // from stackoverflow :P
    // https://stackoverflow.com/questions/35385703/read-file-character-by-character-in-rust
    let mut lineno = 0;
    let mut charno = 0;

    // file
    let mut file = BufReader::new(File::open(env::args()[0])).expect("ERROR: opening file failed.");

    // buffer for file data
    let mut fbuf = Vec::<u8>::new();
    while f.read_until(b'\n', &mut buf).expect("read_until failed") != 0 {
        // this moves the ownership of the read data to s
        // there is no allocation
        let s = String::from_utf8(buf).expect("from_utf8 failed");
        for c in s.chars() {
            println!("Character: {}", c);
        }
        // this returns the ownership of the read data to buf
        // there is no allocation
        buf = s.into_bytes();
        buf.clear();
    }
}
