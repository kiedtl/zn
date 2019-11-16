use std::collections::LinkedList;
use crate::registers::*;
use std::process;
use crate::errors::*;
use crate::parser::*;

pub fn execute(
    BUFFER: &mut [Option<u8>],
    REGSTR: &mut Registers,
    DSTACK: &mut LinkedList<usize>,
    file: String,
    commands: Vec<String>
) {
    for mut ctr in 0..commands.len() {
        let command = parse(commands[ctr]);
        match command.command {
            "$" => if !DSTACK.is_empty() {
                DSTACK.pop_front();
            } else {
                die(ErrorCause::NilStackPopError, file, commands, ctr);
            },
            "£" => if !DSTACK.is_empty() {
                DSTACK.pop_back();
            } else {
                die(ErrorCause::NilStackPopError, file, commands, ctr);
            }

            "#" => setval(),
            _ => (),
        }
    }
}

fn setval(
    BUFFER: &mut [Option<u8>],
    REGSTR: &mut Registers,
    DSTACK: &mut LinkedList<usize>,
    file: String,
    command: Command,
    command_num: usize,
    commands: Vec<String>,
) {
    let src = command.arguments[1];
    let dest = command.arguments[2];
    match src.token_type {
        TokenType::Address => if src.token.parse::<usize>().unwrap()>=BUFFER.len() {
            die(ErrorCause::BufferOverflowError, file, commands, command_num)
        } else {

        TokenType::Register => ,
        _ => (),
    }
}

fn getval(
    BUFFER: &mut [Option<u8>],
    REGSTR: &mut Registers,
    DSTACK: &mut LinkedList<usize>,
    token: Token,
    // error stuff *sigh*
    file: String,
    commands: Vec<String>,
    command: usize,
) -> u8 {
    // check for invalid tokens first
    let parsed_tokenres = token.token.parse::<usize>();
    match parsed_tokenres {

    match token.token_type {
        TokenType::Address => if token.token.parse::<usize>()>=BUFFER.len() {
            die(ErrorCause::BufferOverflowError, file, commands, command)
        } else {
            BUFFER[token.token]

fn die(
    reason: ErrorCause,
    file: String,
    commands: Vec<String>, 
    command: usize
) {
    // because we don't provide charno and lineno
    // of the error, we provide context.
    let context = commands[command];
    let context_prev = if command == 0 {
        "".to_owned()
    } else {
        commands[command-1]
    };
    let context_next = if command == commands.len()-1 {
        "".to_owned()
    } else {
        commands[command+1]
    };

    let error = Error {
        file: file,
        reason: ErrorCause::NilStackPopError,
        context: format!("{}{}{}", context_prev, context, context_next),
        context_char: 0, // irrelevant, since context_char isn't implemented yet :P
    };
    error.throw();
    process::exit(1);
}
