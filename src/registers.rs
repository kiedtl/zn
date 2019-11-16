use crate::parser::*;

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
