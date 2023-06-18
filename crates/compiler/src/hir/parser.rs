use std::rc::Rc;

use crate::{
    lexer::{Lexer, Token, TokenType},
    prelude::*,
};

use super::{Hir, HirImport, HirModule};

pub struct Parser {
    pub errors: Vec<Error>,
    pub lex: Lexer,
    pub ast: Hir,
    pub peek_buf: Option<Token>,
}

impl Parser {
    pub fn new(source: Rc<str>) -> Self {
        Self {
            errors: Vec::with_capacity(0),
            lex: Lexer::new(source),
            ast: Hir::default(),
            peek_buf: None,
        }
    }

    pub fn expect(&mut self, token_type: TokenType) -> Result<Token> {
        let tok = if let Some(tok_buf) = self.peek_buf.take() {
            tok_buf
        } else {
            self.lex.next_token()?.ok_or(Error::UnexpectedEof)?
        };
        if tok.r#type != token_type {
            return Err(Error::UnexpectedToken);
        }
        Ok(tok)
    }

    pub fn expect_one(&mut self) -> Result<Token> {
        if let Some(tok) = self.peek_buf.take() {
            return Ok(tok);
        }
        self.lex
            .next_token()
            .and_then(|it| it.ok_or(Error::UnexpectedEof))
    }

    pub fn peek(&mut self) -> Result<Token> {
        let tok = self
            .lex
            .next_token()
            .and_then(|it| it.ok_or(Error::UnexpectedEof))?;
        self.peek_buf = Some(tok);
        Ok(tok)
    }

    pub fn parse(&mut self) -> Result<()> {
        while let Some(tok) = self.lex.next_token()? {
            match tok.r#type {
                TokenType::KwModule => {
                    let ident = self.expect(TokenType::Identifier)?;
                    self.expect(TokenType::Semicolon)?;
                    self.ast.modules.push(HirModule { name: ident.slice });
                }
                TokenType::KwImport => {
                    let mut buf = Vec::new();
                    self.parse_import(&mut buf)?;
                    self.expect(TokenType::Semicolon)?;
                }
                _ => {
                    eprintln!(
                        "UNHANDLED TOKEN: {:?} {:?}",
                        tok.r#type,
                        self.lex.slice(tok.slice)
                    );
                }
            }
        }
        Ok(())
    }

    fn parse_import(&mut self, buf: &mut Vec<Str>) -> Result<()> {
        let root = self.expect(TokenType::Identifier)?;
        buf.push(root.slice);
        let next = self.peek()?;
        match next.r#type {
            TokenType::Dot => {
                self.expect_one()?;
                self.parse_import(buf)?;
            }
            TokenType::Colon => {
                self.expect_one()?;
                self.parse_import_group(buf)?;
            }
            _ => self.ast.imports.push(HirImport { parts: buf.clone() }),
        }
        buf.pop().unwrap();
        Ok(())
    }

    fn parse_import_group(&mut self, buf: &mut Vec<Str>) -> Result<()> {
        self.expect(TokenType::LeftBrace)?;
        loop {
            self.parse_import(buf)?;
            let next = self.expect_one()?;
            match next.r#type {
                TokenType::RightBrace => break,
                TokenType::Comma => {}
                _ => return Err(Error::UnexpectedToken),
            }
        }
        Ok(())
    }
}
