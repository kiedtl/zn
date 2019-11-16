pub enum TokenType {
    Address,
    Register,
    StackTop,
    StackBottom,
    RawValue,
}

pub struct Token {
    pub token:      String,
    pub token_type: TokenType,
}

pub struct Command {
    pub command:    &'static str,
    pub arguments:  Vec<Token>,
    pub trailer     &'static str,   // random chars that attach themselved to end of command. e.g. [, ], (, or )
}

pub struct Registers {
    pub R: u8,
    pub C: u8,
    pub I: u8,
    pub H: u8,
    pub X: u8,
    pub A: u8,
    pub D: u8,
}

impl Registers {
    pub fn is_valid(token: Token) -> bool {
        match token.token {
            "r"|"c"|"i"|"h"|"x"|"a"|"d" => true,
            _ => false,
        }
    }
}

pub struct Address {
    pub value: usize,
}

impl Address {
    pub fn is_valid(token: Token, BUFFER: [Option<u8>]) -> bool {
        let ret;
        // if it's not an address in the first place...
        if !token.token_type == TokenType::Address {
            ret = false;
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

pub struct RawValue {
    pub value: usize,
}

impl RawValue {
    pub fn is_valid(token: Token) -> bool {
        if !token.token_type == TokenType::RawValue {
            false
        }
        
        let result = token.token.parse::<usize>();
        match result {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}

