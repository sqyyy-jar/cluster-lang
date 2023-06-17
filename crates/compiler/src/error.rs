pub type Result<T> = Result<T, Error>;

pub enum Error {
    UnexpectedEof,
    UnexpectedToken,
    InvalidEscapeSequence,
    InvalidFloat,
    InvalidToken,
}
