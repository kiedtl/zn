use std::collections::LinkedList;
use crate::types::*;
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

// setval operation: equivalent to `#` command
// sets the value of (only) an address or register,
// from an existing address, register, value, or stack item.
fn setval(
    BUFFER: &mut [Option<u8>],
    REGSTR: &mut Registers,
    DSTACK: &mut LinkedList<usize>,
    file: String,
    command: Command,
    command_num: usize,
    commands: Vec<String>,
) {
    // destination for new value
    let dest = command.arguments[1];
    // src of new value
    let src = command.arguments[2];

    // check if destination is valid, and if so, perform setval operation
    match dest.token_type {
        TokenType::Address =>  if !Address::is_valid(dest, BUFFER) {
            die(ErrorCause::InvalidTokenError, file, commands, command_num);
        } else {
            BUFFER[dest.token] = getval(BUFFER, REGSTR, DSTACK, src,
                                       file, commands, command_num);
        },
        TokenType::Register => if !Registers::is_valid(dest) {
            die(ErrorCause::InvalidRegistorError, file, commands, command_num);
        } else {
            match dest.token {
                "r" => REGSTR.R = getval(BUFFER, REGSTR, DSTACK, src, 
                                         file, commands, command_num),
                "c" => REGSTR.C = getval(BUFFER, REGSTR, DSTACK, src,
                                         file, commands, command_num),
                "i" => REGSTR.I = getval(BUFFER, REGSTR, DSTACK, src,
                                         file, commands, command_num),
                "h" => REGSTR.H = getval(BUFFER, REGSTR, DSTACK, src,
                                         file, commands, command_num),
                "x" => REGSTR.X = getval(BUFFER, REGSTR, DSTACK, src,
                                         file, commands, command_num),
                "a" => REGSTR.A = getval(BUFFER, REGSTR, DSTACK, src,
                                         file, commands, command_num),
                "d" => REGSTR.D = getval(BUFFER, REGSTR, DSTACK, src,
                                         file, commands, command_num),
                _ => (),
            }
        },
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
    let parsed_token;
    // check for invalid tokens first
    match token.token_type {
        TokenType::Address => if !Address::is_valid(token, BUFFER) {
            die(ErrorCause::InvalidTokenError, file, commands, command);
        } else { parsed_token = token.token.parse::<usize>().unwrap(); }
        TokenType::Register => if !Register::is_valid(token) {
            die(ErrorCause::InvalidRegisterError, file, commands, command);
        } else { parsed_token = token.token.parse::<usize>().unwrap(); }
        TokenType::RawValue => if !RawValue::is_valid(token) {
            die(ErrorCause::InvalidTokenError, file, commands, command);
        } else { parsed_token = token.token.parse::<usize>().unwrap(); }
        _ => (),
    }

    match token.token_type {
        TokenType::Address => BUFFER[parsed_token],
        TokenType::Register => match token.token {
            "r" => REGSTR.R,
            "c" => REGSTR.C,
            "i" => REGSTR.I,
            "h" => REGSTR.H,
            "x" => REGSTR.X,
            "a" => REGSTR.A,
            "d" => REGSTR.D,
        },
        TokenType::StackTop => *DSTACK.front(),
        TokenType::StackBottom => *DSTACK.back(),
        TokenType::RawValue => parsed_token,
    }
    0
}

//
// helper functions
//

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
