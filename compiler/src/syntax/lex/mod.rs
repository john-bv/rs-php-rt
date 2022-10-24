use std::io::Error;

use self::{cursor::Cursor, token::Token};

pub(crate) mod cursor;
pub mod token;

pub(crate) trait Tokenizer<'a> {
    fn lex(&mut self, cursor: &'a mut Cursor) -> Result<Token, Error>;
}
