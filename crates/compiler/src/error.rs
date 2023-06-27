use crate::lexer::Lexer;
use crate::prelude::Str;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectedEof,
    UnexpectedToken(Str),
    UnexpectedExpression,
    InvalidEscapeSequence(Str),
    InvalidFloat(Str),
    InvalidToken(Str),
    InvalidUnaryExpression(Str),
}

impl Error {
    pub fn slice<'a>(&self, lex: &'a Lexer) -> Option<&'a str> {
        match self {
            Error::UnexpectedEof | Error::UnexpectedExpression => None,
            Error::UnexpectedToken(slice)
            | Error::InvalidEscapeSequence(slice)
            | Error::InvalidFloat(slice)
            | Error::InvalidToken(slice)
            | Error::InvalidUnaryExpression(slice) => Some(lex.slice(*slice)),
        }
    }
}
