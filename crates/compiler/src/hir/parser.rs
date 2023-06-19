use std::rc::Rc;

use crate::{
    hir::HirFunction,
    lexer::{Lexer, Token, TokenType},
    prelude::*,
};

use super::{
    Hir, HirBlock, HirFunctionParam, HirImport, HirModule, HirPath, HirStatement, HirType,
};

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
        if let Some(tok) = self.peek_buf {
            return Ok(tok);
        }
        let tok = self
            .lex
            .next_token()
            .and_then(|it| it.ok_or(Error::UnexpectedEof))?;
        self.peek_buf = Some(tok);
        Ok(tok)
    }

    pub fn maybe(&mut self, token_type: TokenType) -> Result<Option<Token>> {
        let tok = self.peek()?;
        Ok(if tok.r#type == token_type {
            self.expect_one()?;
            Some(tok)
        } else {
            None
        })
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
                TokenType::KwPub => {
                    let next = self.expect_one()?;
                    match next.r#type {
                        TokenType::KwFun => self.parse_function(true)?,
                        _ => return Err(Error::UnexpectedToken),
                    }
                }
                TokenType::KwFun => self.parse_function(false)?,
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

    fn parse_type(&mut self) -> Result<HirType> {
        let name = self.expect(TokenType::Identifier)?;
        Ok(HirType { name: name.slice })
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
            _ => self.ast.imports.push(HirImport {
                path: HirPath { parts: buf.clone() },
            }),
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

    fn parse_function(&mut self, public: bool) -> Result<()> {
        let name = self.expect(TokenType::Identifier)?;
        // generics
        self.expect(TokenType::LeftParen)?;
        let params = self.parse_function_params()?;
        self.expect(TokenType::RightParen)?;
        let return_type = if self.maybe(TokenType::Arrow)?.is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };
        let body = if self.maybe(TokenType::Semicolon)?.is_some() {
            None
        } else {
            Some(self.parse_block()?)
        };
        self.ast.functions.push(HirFunction {
            name: name.slice,
            public,
            params,
            return_type,
            body,
        });
        Ok(())
    }

    fn parse_function_params(&mut self) -> Result<Vec<HirFunctionParam>> {
        let mut params = Vec::with_capacity(0);
        loop {
            if self.peek()?.r#type == TokenType::RightParen {
                break;
            }
            let r#type = self.parse_type()?;
            let name = self.expect(TokenType::Identifier)?;
            params.push(HirFunctionParam {
                name: name.slice,
                r#type,
            });
            if self.peek()?.r#type != TokenType::Comma {
                break;
            }
        }
        Ok(params)
    }

    fn parse_block(&mut self) -> Result<HirBlock> {
        self.expect(TokenType::LeftBrace)?;
        let mut statements = Vec::new();
        while self.maybe(TokenType::RightBrace)?.is_none() {
            statements.push(self.parse_statement()?);
        }
        Ok(HirBlock { statements })
    }

    fn parse_statement(&mut self) -> Result<HirStatement> {
        todo!()
    }
}
