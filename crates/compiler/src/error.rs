pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnexpectedEof,
    UnexpectedToken,
    UnexpectedExpression,
    InvalidEscapeSequence,
    InvalidFloat,
    InvalidToken,
    InvalidUnaryExpression,
    InvalidBinaryExpression,
}
