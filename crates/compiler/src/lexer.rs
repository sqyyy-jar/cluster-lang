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
    "for" => TokenType::KwFor,
    "while" => TokenType::KwWhile,
    "in" => TokenType::KwIn,
    "return" => TokenType::KwReturn,
    "continue" => TokenType::KwContinue,
    "break" => TokenType::KwBreak,
};

#[derive(Clone, Copy)]
pub struct Token {
    pub r#type: TokenType,
    pub slice: Str,
}

impl Token {
    pub fn new(r#type: TokenType, slice: Str) -> Self {
        Self { r#type, slice }
    }
}

#[derive(Clone, Copy)]
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
    // Equal combined
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
    KwFor,
    KwWhile,
    KwIn,
    KwReturn,
    KwContinue,
    KwBreak,
}

pub struct Lexer {
    source: Rc<str>,
    index: usize,
}

impl Lexer {
    pub fn new(source: Rc<str>) -> Self {
        Self { source, index: 0 }
    }

    pub fn has_next(&self) -> bool {
        self.index < self.source.len()
    }

    pub fn peek(&self) -> Option<char> {
        self.source.get(self.index..)?.chars().next()
    }

    pub fn eat(&mut self) {
        self.index += self
            .source
            .get(self.index..)
            .unwrap()
            .chars()
            .next()
            .unwrap()
            .len_utf8();
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.eat();
        }
    }

    pub fn slice(&self, slice: Str) -> &str {
        &self.source[slice.0..slice.1]
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, ()> {
        loop {
            self.skip_whitespace();
            let index = self.index;
            let Some(c) = self.peek() else {
                return Ok(None);
            };
            self.eat();
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
                    if let Some('=') = self.peek() {
                        self.eat();
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    }
                }
                '&' => {
                    if let Some('=') = self.peek() {
                        self.eat();
                        TokenType::AndEqual
                    } else {
                        TokenType::And
                    }
                }
                '|' => {
                    if let Some('=') = self.peek() {
                        self.eat();
                        TokenType::PipeEqual
                    } else {
                        TokenType::Pipe
                    }
                }
                '^' => {
                    if let Some('=') = self.peek() {
                        self.eat();
                        TokenType::CaretEqual
                    } else {
                        TokenType::Caret
                    }
                }
                '+' => {
                    if let Some('=') = self.peek() {
                        self.eat();
                        TokenType::PlusEqual
                    } else {
                        TokenType::Plus
                    }
                }
                '-' => {
                    if let Some('=') = self.peek() {
                        self.eat();
                        TokenType::MinusEqual
                    } else {
                        TokenType::Minus
                    }
                }
                '*' => {
                    if let Some('=') = self.peek() {
                        self.eat();
                        TokenType::StarEqual
                    } else {
                        TokenType::Star
                    }
                }
                '/' => match self.peek() {
                    Some('=') => {
                        self.eat();
                        TokenType::SlashEqual
                    }
                    Some('/') => {
                        self.eat();
                        while let Some(c) = self.peek() {
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
                    if let Some('=') = self.peek() {
                        self.eat();
                        TokenType::PercentEqual
                    } else {
                        TokenType::Percent
                    }
                }
                '=' => {
                    if let Some('=') = self.peek() {
                        self.eat();
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    }
                }
                '<' => {
                    if let Some('=') = self.peek() {
                        self.eat();
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    }
                }
                '>' => {
                    if let Some('=') = self.peek() {
                        self.eat();
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    }
                }
                '.' | '0'..='9' => 'blk: {
                    let mut is_float = c == '.';
                    let ac = self.peek();
                    if is_float && (ac.is_none() || !ac.unwrap().is_ascii_digit()) {
                        break 'blk TokenType::Dot;
                    }
                    let ac = ac.unwrap();
                    if is_float && ac == '.' {
                        self.eat();
                        let bc = self.peek();
                        if bc == Some('.') {
                            self.eat();
                            break 'blk TokenType::DotDotDot;
                        }
                        break 'blk TokenType::DotDot;
                    }
                    while let Some(bc) = self.peek() {
                        if bc == '.' {
                            if is_float {
                                todo!("Invalid float")
                            }
                            is_float = true;
                            self.eat();
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
                        let ac = self.peek();
                        if ac.is_none() {
                            todo!("Unexpected eof")
                        }
                        let ac = ac.unwrap();
                        if ac == '\\' {
                            self.eat();
                            let Some(bc) = self.peek() else {
                                todo!("Unexpected eof")
                            };
                            match bc {
                                '"' | '\\' | 'n' | 't' | 'r' => self.eat(),
                                'x' => {
                                    self.eat();
                                    let Some(cc) = self.peek() else {
                                        todo!("Invalid escape sequence")
                                    };
                                    if !matches!(cc, '0'..='9' | 'a'..='f' | 'A'..='F') {
                                        todo!("Invalid escape sequence")
                                    }
                                    self.eat();
                                    let Some(dc) = self.peek() else {
                                        todo!("Invalid escape sequence")
                                    };
                                    if !matches!(dc, '0'..='9' | 'a'..='f' | 'A'..='F') {
                                        todo!("Invalid escape sequence")
                                    }
                                    self.eat();
                                }
                                _ => todo!("Invalid escape sequence"),
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
                    while let Some(ac) = self.peek() {
                        if !ac.is_ascii_alphanumeric() && ac != '_' {
                            break;
                        }
                        self.eat();
                    }
                    let ident = self.slice(Str(index, self.index));
                    if let Some(kw) = KEYWORDS.get(ident) {
                        *kw
                    } else {
                        TokenType::Identifier
                    }
                }
                _ => todo!(),
            };
            return Ok(Some(Token::new(token_type, Str(index, self.index))));
        }
    }
}
