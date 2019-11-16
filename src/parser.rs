use std::vec::Vec;
use crate::types::*;

// match token type
fn match_token(_char: char) -> TokenType {
    match _char {
        '~' => TokenType::Address,
        '%' => TokenType::Register,
        '@' => TokenType::StackTop,
        '!' => TokenType::StackBottom,
        '_' => TokenType::RawValue,
    }
}

// parse individual commands
pub fn parse(command: String) -> Command {
    let chars = command.chars().collect::<Vec<char>>();
    let arguments = Vec::<Token>::new();
    let trailer: &str = if chars[chars.len()-1] == ']' ||
        chars[chars.len()-1] == ')' 
        {
            &chars[chars.len()-1].to_string()
        } else {
            ""
        };
    let mut ctr = 1;
    for _ in 1..chars.len() {
        let mut c_char = chars[ctr];
        if is_tokentype(c_char) {
            let mut tokbuf = Token { token: "".to_owned(), token_type: match_token(c_char) };
            ctr = ctr + 1;
            if ctr == chars.len() {
                break;
            }
            c_char = chars[ctr];

            while !is_tokentype(c_char) {
                tokbuf.token.push(c_char);
                if ctr == chars.len() {
                    break;
                }
            }
            arguments.push(tokbuf);
        }
        ctr = ctr + 1;
    }
    Command {
        command: &chars[0].to_string(),
        arguments: arguments,
        trailer: trailer,
    }
}

fn is_tokentype(_char: char) -> bool {
    match _char {
        '~'|
            '%'|
            '@'|
            '!'|
            '_' => true,
        _ => false,
    }
}
