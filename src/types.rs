#[derive(PartialEq, Copy, Clone)]
pub enum TokenType {
    Address,
    Register,
    StackTop,
    StackBottom,
    RawValue,
    Unknown,
}

#[derive(PartialEq, Clone)]
pub struct Token {
    pub token:      String,
    pub token_type: TokenType,
}

pub struct Command {
    pub command:    String,
    pub arguments:  Vec<Token>,
    pub trailer:    String,   // random chars that attach themselved to end of command. e.g. [, ], (, or )
}

pub struct Registers {
    pub R: Option<u8>,
    pub C: Option<u8>,
    pub I: Option<u8>,
    pub H: Option<u8>,
    pub X: Option<u8>,
    pub A: Option<u8>,
    pub D: Option<u8>,
}

impl Registers {
    pub fn is_valid(token: &Token) -> bool {
        match token.token.as_ref() {
            "r"
                |"c"
                |"i"
                |"h"
                |"x"
                |"a"
                |"d" => true,
            _ => false,
        }
    }
}

pub struct Address {
    pub value: usize,
}

impl Address {
    pub fn is_valid(token: &Token, BUFFER: &[Option<u8>; 65535]) -> bool {
        let mut ret;
        // if it's not an address in the first place...
        if token.token_type != TokenType::Address {
            return false;
        }

        let result = token.token.parse::<usize>();
        match result {
            Ok(_) => ret = true,
            Err(_) => ret = false,
        }

        // if token exeeds the size of the memory buffer,
        // then it's also not valid!
        if result.unwrap() > (BUFFER.len() - 1) {
            ret = false;
        }
        ret
    }
}

// TODO: currently, support is only available for
// type `u8` as raw value; raw_value *should* support
// `Option<u8>` value.
// TODO: support Option<u8>; i.e. support None (as `_N`)
pub struct RawValue {
    pub value: usize,
}

impl RawValue {
    pub fn is_valid(token: Token) -> bool {
        if token.token_type != TokenType::RawValue {
            return false;
        }
        
        let result = token.token.parse::<usize>();
        match result {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

