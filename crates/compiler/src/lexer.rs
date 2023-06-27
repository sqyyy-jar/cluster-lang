use std::rc::Rc;

use phf::{phf_map, Map};

use crate::prelude::*;

pub const KEYWORDS: Map<&str, TokenType> = phf_map! {
    "module" => TokenType::KwModule,
    "import" => TokenType::KwImport,
    "trait" => TokenType::KwTrait,
    "struct" => TokenType::KwStruct,
    "enum" => TokenType::KwEnum,
    "impl" => TokenType::KwImpl,
    "fun" => TokenType::KwFun,
    "const" => TokenType::KwConst,
    "var" => TokenType::KwVar,
    "if" => TokenType::KwIf,
    "else" => TokenType::KwElse,
    "elseif" => TokenType::KwElseif,
    "for" => TokenType::KwFor,
    "while" => TokenType::KwWhile,
    "in" => TokenType::KwIn,
    "return" => TokenType::KwReturn,
    "continue" => TokenType::KwContinue,
    "break" => TokenType::KwBreak,
    "and" => TokenType::KwAnd,
    "or" => TokenType::KwOr,
};

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub slice: Str,
}

impl Token {
    pub fn new(r#type: TokenType, slice: Str) -> Self {
        Self { r#type, slice }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
    // Brackets
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    // Punctuation
    Colon,
    Semicolon,
    At,
    Hashtag,
    Dot,
    DotDot,
    DotDotDot,
    Arrow,
    Comma,
    // Operators
    Bang,
    And,
    Pipe,
    Caret,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    Less,
    Greater,
    // Combined
    BangEqual,
    AndEqual,
    PipeEqual,
    CaretEqual,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    LessLess,
    GreaterGreater,
    // Literals
    Integer,
    Float,
    String,
    Identifier,
    // Keywords
    KwModule,
    KwImport,
    KwPub,
    KwTrait,
    KwStruct,
    KwEnum,
    KwImpl,
    KwFun,
    KwConst,
    KwVar,
    KwIf,
    KwElse,
    KwElseif,
    KwFor,
    KwWhile,
    KwIn,
    KwReturn,
    KwContinue,
    KwBreak,
    KwAnd,
    KwOr,
}

impl TokenType {
    pub fn is_unary_op(&self) -> bool {
        matches!(
            self,
            Self::Bang
                | Self::And
                // | Self::Pipe
                // | Self::Caret
                | Self::Plus
                | Self::Minus
                | Self::Star
        )
        /*
        | Self::Slash
        | Self::Percent
        | Self::Equal
        | Self::Less
        | Self::Greater
        | Self::LessLess
        | Self::GreaterGreater
         */
    }

    pub fn is_binary_op(&self) -> bool {
        matches!(
            self,
            Self::Bang
                | Self::And
                | Self::Pipe
                | Self::Caret
                | Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Percent
                | Self::Less
                | Self::Greater
                | Self::BangEqual
                | Self::EqualEqual
                | Self::LessEqual
                | Self::GreaterEqual
                | Self::LessLess
                | Self::GreaterGreater
                | Self::KwAnd
                | Self::KwOr
        )
    }

    pub fn precedence(&self) -> usize {
        match self {
            Self::KwOr => 1,
            Self::KwAnd => 2,
            Self::Pipe => 3,
            Self::Caret => 4,
            Self::And => 5,
            Self::EqualEqual | Self::BangEqual => 6,
            Self::Less | Self::Greater | Self::LessEqual | Self::GreaterEqual => 7,
            Self::LessLess | Self::GreaterGreater => 8,
            Self::Plus | Self::Minus => 9,
            Self::Star | Self::Slash | Self::Percent => 10,
            _ => panic!("Invalid operator"),
        }
    }
}

pub struct Lexer {
    source: Rc<str>,
    index: u32,
}

impl Lexer {
    pub fn new(source: Rc<str>) -> Self {
        Self { source, index: 0 }
    }

    pub fn has_next(&self) -> bool {
        self.index < self.source.len() as u32
    }

    pub fn peek(&self) -> Result<char> {
        self.source
            .get(self.index as usize..)
            .ok_or(Error::UnexpectedEof)?
            .chars()
            .next()
            .ok_or(Error::UnexpectedEof)
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Result<char> {
        let c = self.peek()?;
        self.index += c.len_utf8() as u32;
        Ok(c)
    }

    pub fn eat(&mut self) {
        self.index += self.peek().unwrap().len_utf8() as u32;
    }

    pub fn maybe(&mut self, c: char) -> bool {
        if self.peek() == Ok(c) {
            self.eat();
            return true;
        }
        false
    }

    pub fn skip_whitespace(&mut self) {
        while let Ok(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.eat();
        }
    }

    pub fn slice(&self, slice: Str) -> &str {
        &self.source[slice.0 as usize..(slice.0 + slice.1) as usize]
    }

    pub fn next_token(&mut self) -> Result<Option<Token>> {
        loop {
            self.skip_whitespace();
            let index = self.index;
            let Ok(c) = self.next() else {
                return Ok(None);
            };
            let token_type: TokenType = match c {
                '(' => TokenType::LeftParen,
                ')' => TokenType::RightParen,
                '[' => TokenType::LeftBracket,
                ']' => TokenType::RightBracket,
                '{' => TokenType::LeftBrace,
                '}' => TokenType::RightBrace,
                ':' => TokenType::Colon,
                ';' => TokenType::Semicolon,
                '@' => TokenType::At,
                '#' => TokenType::Hashtag,
                ',' => TokenType::Comma,
                '!' => {
                    if self.maybe('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    }
                }
                '&' => {
                    if self.maybe('=') {
                        TokenType::AndEqual
                    } else {
                        TokenType::And
                    }
                }
                '|' => {
                    if self.maybe('=') {
                        TokenType::PipeEqual
                    } else {
                        TokenType::Pipe
                    }
                }
                '^' => {
                    if self.maybe('=') {
                        TokenType::CaretEqual
                    } else {
                        TokenType::Caret
                    }
                }
                '+' => {
                    if self.maybe('=') {
                        TokenType::PlusEqual
                    } else {
                        TokenType::Plus
                    }
                }
                '-' => match self.peek() {
                    Ok('=') => {
                        self.eat();
                        TokenType::MinusEqual
                    }
                    Ok('>') => {
                        self.eat();
                        TokenType::Arrow
                    }
                    _ => TokenType::Minus,
                },
                '*' => {
                    if self.maybe('=') {
                        TokenType::StarEqual
                    } else {
                        TokenType::Star
                    }
                }
                '/' => match self.peek() {
                    Ok('=') => {
                        self.eat();
                        TokenType::SlashEqual
                    }
                    Ok('/') => {
                        self.eat();
                        while let Ok(c) = self.peek() {
                            if c == '\n' {
                                break;
                            }
                            self.eat();
                        }
                        continue;
                    }
                    _ => TokenType::Slash,
                },
                '%' => {
                    if self.maybe('=') {
                        TokenType::PercentEqual
                    } else {
                        TokenType::Percent
                    }
                }
                '=' => {
                    if self.maybe('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    }
                }
                '<' => match self.peek() {
                    Ok('=') => {
                        self.eat();
                        TokenType::LessEqual
                    }
                    Ok('<') => {
                        self.eat();
                        TokenType::LessLess
                    }
                    _ => TokenType::Less,
                },
                '>' => match self.peek() {
                    Ok('=') => {
                        self.eat();
                        TokenType::LessEqual
                    }
                    Ok('>') => {
                        self.eat();
                        TokenType::GreaterGreater
                    }
                    _ => TokenType::Greater,
                },
                '.' | '0'..='9' => 'blk: {
                    let mut is_float = c == '.';
                    let ac = self.peek();
                    if is_float && !matches!(ac, Ok('0'..='9')) {
                        break 'blk TokenType::Dot;
                    }
                    let ac = ac.unwrap();
                    if is_float && ac == '.' {
                        self.eat();
                        if self.maybe('.') {
                            break 'blk TokenType::DotDotDot;
                        }
                        break 'blk TokenType::DotDot;
                    }
                    while let Ok(bc) = self.peek() {
                        if self.maybe('.') {
                            if is_float {
                                return Err(Error::InvalidFloat(Str(index, self.index)));
                            }
                            is_float = true;
                            continue;
                        }
                        if !bc.is_ascii_digit() {
                            break;
                        }
                        self.eat();
                    }
                    if is_float {
                        TokenType::Float
                    } else {
                        TokenType::Integer
                    }
                }
                '"' => {
                    loop {
                        let ac = self.peek()?;
                        if ac == '\\' {
                            let escape_sequence_start = self.index;
                            self.eat();
                            let bc = self.peek()?;
                            match bc {
                                '"' | '\\' | 'n' | 't' | 'r' => self.eat(),
                                'x' => {
                                    self.eat();
                                    let cc = self.next()?;
                                    if !matches!(cc, '0'..='9' | 'a'..='f' | 'A'..='F') {
                                        return Err(Error::InvalidEscapeSequence(Str(
                                            escape_sequence_start,
                                            self.index,
                                        )));
                                    }
                                    let dc = self.next()?;
                                    if !matches!(dc, '0'..='9' | 'a'..='f' | 'A'..='F') {
                                        return Err(Error::InvalidEscapeSequence(Str(
                                            escape_sequence_start,
                                            self.index,
                                        )));
                                    }
                                }
                                _ => {
                                    return Err(Error::InvalidEscapeSequence(Str(
                                        escape_sequence_start,
                                        self.index,
                                    )))
                                }
                            }
                            continue;
                        }
                        self.eat();
                        if ac == '"' {
                            break;
                        }
                    }
                    TokenType::String
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    while let Ok(ac) = self.peek() {
                        if !ac.is_ascii_alphanumeric() && ac != '_' {
                            break;
                        }
                        self.eat();
                    }
                    let ident = self.slice(Str(index, self.index - index));
                    if let Some(kw) = KEYWORDS.get(ident) {
                        *kw
                    } else {
                        TokenType::Identifier
                    }
                }
                _ => return Err(Error::InvalidToken(Str(index, self.index))),
            };
            return Ok(Some(Token::new(token_type, Str(index, self.index - index))));
        }
    }
}
