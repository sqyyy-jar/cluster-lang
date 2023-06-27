use std::rc::Rc;

use crate::hir::{HirImpl, HirStructField, HirTypeDecl};
use crate::{
    hir::{HirConst, HirFunction},
    lexer::{Lexer, Token, TokenType},
    prelude::*,
};

use super::{
    Hir, HirBlock, HirExpression, HirFunctionParam, HirImport, HirModule, HirPath, HirStatement,
    HirType,
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
            return Err(Error::UnexpectedToken(tok.slice));
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
                TokenType::KwPub => {
                    let next = self.expect_one()?;
                    match next.r#type {
                        TokenType::KwModule => self.parse_root_module(true)?,
                        TokenType::KwConst => self.parse_root_const(true)?,
                        TokenType::KwTrait => self.parse_root_trait(true)?,
                        TokenType::KwStruct => self.parse_root_struct(true)?,
                        TokenType::KwFun => self.parse_root_function(true)?,
                        _ => return Err(Error::UnexpectedToken(next.slice)),
                    }
                }
                TokenType::KwModule => self.parse_root_module(false)?,
                TokenType::KwImport => {
                    self.parse_root_import(&mut Vec::new())?;
                    self.expect(TokenType::Semicolon)?;
                }
                TokenType::KwConst => self.parse_root_const(false)?,
                TokenType::KwTrait => self.parse_root_trait(false)?,
                TokenType::KwStruct => self.parse_root_struct(false)?,
                TokenType::KwImpl => self.parse_root_impl()?,
                TokenType::KwFun => self.parse_root_function(false)?,
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

    fn parse_root_module(&mut self, public: bool) -> Result<()> {
        let name = self.expect(TokenType::Identifier)?.slice;
        self.expect(TokenType::Semicolon)?;
        self.ast.modules.push(HirModule { name, public });
        Ok(())
    }

    fn parse_root_import(&mut self, buf: &mut Vec<Str>) -> Result<()> {
        let root = self.expect(TokenType::Identifier)?;
        buf.push(root.slice);
        let next = self.peek()?;
        match next.r#type {
            TokenType::Dot => {
                self.expect_one()?;
                self.parse_root_import(buf)?;
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

    fn parse_root_const(&mut self, public: bool) -> Result<()> {
        let (name, r#type, expr) = self.parse_var_decl()?;
        self.ast.constants.push(HirConst {
            name,
            public,
            r#type,
            expr,
        });
        Ok(())
    }

    fn parse_root_trait(&mut self, public: bool) -> Result<()> {
        let name = self.expect(TokenType::Identifier)?.slice;
        // todo: generics
        let mut functions = Vec::with_capacity(0);
        while self.maybe(TokenType::RightBrace)?.is_none() {
            functions.push(self.parse_function(false)?);
        }
        self.ast.types.push(HirTypeDecl::Trait {
            name,
            public,
            functions,
        });
        Ok(())
    }

    fn parse_root_struct(&mut self, public: bool) -> Result<()> {
        let name = self.expect(TokenType::Identifier)?.slice;
        // todo: generics
        let mut fields = Vec::with_capacity(0);
        let mut functions = Vec::with_capacity(0);
        while self.maybe(TokenType::RightBrace)?.is_none() {
            if self.maybe(TokenType::KwPub)?.is_some() {
                if self.maybe(TokenType::KwFun)?.is_some() {
                    self.expect_one()?;
                    functions.push(self.parse_function(true)?);
                    continue;
                }
                fields.push(self.parse_struct_field(true)?);
                continue;
            }
            if self.maybe(TokenType::KwFun)?.is_some() {
                self.expect_one()?;
                functions.push(self.parse_function(false)?);
                continue;
            }
            fields.push(self.parse_struct_field(false)?);
        }
        self.ast.types.push(HirTypeDecl::Struct {
            name,
            public,
            fields,
            functions,
        });
        Ok(())
    }

    fn parse_root_impl(&mut self) -> Result<()> {
        let target = self.parse_type()?;
        self.expect(TokenType::Colon)?;
        let r#trait = self.parse_type()?;
        self.expect(TokenType::LeftBrace)?;
        let mut functions = Vec::with_capacity(0);
        while self.maybe(TokenType::RightBrace)?.is_none() {
            if self.maybe(TokenType::KwPub)?.is_some() {
                self.expect(TokenType::KwFun)?;
                functions.push(self.parse_function(true)?);
            }
            self.expect(TokenType::KwFun)?;
            functions.push(self.parse_function(false)?);
        }
        self.ast.impls.push(HirImpl {
            target,
            r#trait,
            functions,
        });
        Ok(())
    }

    fn parse_root_function(&mut self, public: bool) -> Result<()> {
        let function = self.parse_function(public)?;
        self.ast.functions.push(function);
        Ok(())
    }

    fn parse_struct_field(&mut self, public: bool) -> Result<HirStructField> {
        let r#type = self.parse_type()?;
        let name = self.expect(TokenType::Identifier)?.slice;
        self.expect(TokenType::Semicolon)?;
        Ok(HirStructField {
            name,
            public,
            r#type,
        })
    }

    fn parse_function(&mut self, public: bool) -> Result<HirFunction> {
        let name = self.expect(TokenType::Identifier)?.slice;
        // todo: generics
        let params = self.parse_function_params()?;
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
        Ok(HirFunction {
            name,
            public,
            params,
            return_type,
            body,
        })
    }

    fn parse_type(&mut self) -> Result<HirType> {
        let name = self.expect(TokenType::Identifier)?;
        // todo: generics
        Ok(HirType { name: name.slice })
    }

    fn parse_import_group(&mut self, buf: &mut Vec<Str>) -> Result<()> {
        self.expect(TokenType::LeftBrace)?;
        loop {
            self.parse_root_import(buf)?;
            let next = self.expect_one()?;
            match next.r#type {
                TokenType::RightBrace => break,
                TokenType::Comma => {}
                _ => return Err(Error::UnexpectedToken(next.slice)),
            }
        }
        Ok(())
    }

    fn parse_function_params(&mut self) -> Result<Vec<HirFunctionParam>> {
        let mut params = Vec::with_capacity(0);
        self.expect(TokenType::LeftParen)?;
        loop {
            if self.peek()?.r#type == TokenType::RightParen {
                break;
            }
            let r#type = self.parse_type()?;
            let name = self.expect(TokenType::Identifier)?.slice;
            params.push(HirFunctionParam { name, r#type });
            if self.peek()?.r#type != TokenType::Comma {
                break;
            }
        }
        self.expect(TokenType::RightParen)?;
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
        let tok = self.peek()?;
        match tok.r#type {
            TokenType::KwVar => {
                self.expect_one()?;
                let (name, r#type, expr) = self.parse_var_decl()?;
                Ok(HirStatement::VarDecl { name, r#type, expr })
            }
            TokenType::KwConst => {
                self.expect_one()?;
                let (name, r#type, expr) = self.parse_var_decl()?;
                Ok(HirStatement::ConstDecl { name, r#type, expr })
            }
            TokenType::KwIf => {
                self.expect_one()?;
                let cond = self.parse_expression()?;
                let block = self.parse_block()?;
                let else_block = self.parse_else()?;
                Ok(HirStatement::If {
                    cond,
                    block,
                    else_block,
                })
            }
            TokenType::KwWhile => {
                self.expect_one()?;
                let cond = self.parse_expression()?;
                let block = self.parse_block()?;
                Ok(HirStatement::While { cond, block })
            }
            TokenType::KwFor => {
                self.expect_one()?;
                let name = self.expect(TokenType::Identifier)?.slice;
                self.expect(TokenType::KwIn)?;
                let expr = self.parse_expression()?;
                let block = self.parse_block()?;
                Ok(HirStatement::For { name, expr, block })
            }
            TokenType::KwReturn => {
                self.expect_one()?;
                let expr = self.parse_expression()?;
                self.expect(TokenType::Semicolon)?;
                Ok(HirStatement::Return { expr })
            }
            TokenType::KwContinue => {
                self.expect_one()?;
                self.expect(TokenType::Semicolon)?;
                Ok(HirStatement::Continue)
            }
            TokenType::KwBreak => {
                self.expect_one()?;
                self.expect(TokenType::Semicolon)?;
                Ok(HirStatement::Break)
            }
            _ => {
                let expr = self.parse_expression()?;
                match expr {
                    HirExpression::Call { expr, args } => {
                        self.expect(TokenType::Semicolon)?;
                        Ok(HirStatement::Call { expr: *expr, args })
                    }
                    HirExpression::Access { .. } => {
                        self.expect(TokenType::Equal)?;
                        let value = self.parse_expression()?;
                        self.expect(TokenType::Semicolon)?;
                        Ok(HirStatement::Assign { expr, value })
                    }
                    HirExpression::DotAccess { .. } => {
                        self.expect(TokenType::Equal)?;
                        let value = self.parse_expression()?;
                        self.expect(TokenType::Semicolon)?;
                        Ok(HirStatement::Assign { expr, value })
                    }
                    HirExpression::IndexAccess { .. } => {
                        self.expect(TokenType::Equal)?;
                        let value = self.parse_expression()?;
                        self.expect(TokenType::Semicolon)?;
                        Ok(HirStatement::Assign { expr, value })
                    }
                    _ => Err(Error::UnexpectedExpression),
                }
            }
        }
    }

    fn parse_var_decl(&mut self) -> Result<(Str, Option<HirType>, Option<HirExpression>)> {
        let name = self.expect(TokenType::Identifier)?.slice;
        let r#type = if self.maybe(TokenType::Colon)?.is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };
        let expr = if self.maybe(TokenType::Equal)?.is_some() {
            Some(self.parse_expression()?)
        } else {
            None
        };
        self.expect(TokenType::Semicolon)?;
        Ok((name, r#type, expr))
    }

    fn parse_expression(&mut self) -> Result<HirExpression> {
        let next = self.peek()?;
        let left = self.parse_unary_expression(next)?;
        let op = self.peek()?;
        if !op.r#type.is_binary_op() {
            return Ok(left);
        }
        self.parse_binary_expression(left)
    }

    fn parse_unary_expression(&mut self, left: Token) -> Result<HirExpression> {
        if left.r#type.is_unary_op() {
            self.expect_one()?;
            let next = self.peek()?;
            return Ok(apply_unary(left, self.parse_unary_expression(next)?));
        }
        match left.r#type {
            TokenType::LeftParen => {
                self.expect_one()?;
                let expr = self.parse_expression()?;
                self.expect(TokenType::RightParen)?;
                self.parse_access_expression(expr)
            }
            TokenType::Integer => {
                self.expect_one()?;
                Ok(HirExpression::Int { slice: left.slice })
            }
            TokenType::Float => {
                self.expect_one()?;
                Ok(HirExpression::Float { slice: left.slice })
            }
            TokenType::String => {
                self.expect_one()?;
                self.parse_access_expression(HirExpression::String { slice: left.slice })
            }
            TokenType::Identifier => {
                self.expect_one()?;
                let mut left = HirExpression::Access { name: left.slice };
                if self.peek()?.r#type == TokenType::LeftParen {
                    left = HirExpression::Call {
                        expr: Box::new(left),
                        args: self.parse_function_call_args()?,
                    };
                }
                self.parse_access_expression(left)
            }
            _ => Err(Error::InvalidUnaryExpression),
        }
    }

    fn parse_binary_expression(&mut self, left: HirExpression) -> Result<HirExpression> {
        let op = self.expect_one()?;
        let next = self.peek()?;
        let right = self.parse_unary_expression(next)?;
        let next = self.peek()?;
        if !next.r#type.is_binary_op() {
            return Ok(apply_binary(op, left, right));
        }
        if op.r#type.precedence() < next.r#type.precedence() {
            return Ok(apply_binary(op, left, self.parse_binary_expression(right)?));
        }
        self.parse_binary_expression(apply_binary(op, left, right))
    }

    fn parse_access_expression(&mut self, left: HirExpression) -> Result<HirExpression> {
        match self.peek()?.r#type {
            TokenType::Dot => {
                self.expect_one()?;
                let name = self.expect(TokenType::Identifier)?.slice;
                let right = if self.peek()?.r#type == TokenType::LeftParen {
                    HirExpression::Call {
                        expr: Box::new(HirExpression::DotAccess {
                            expr: Box::new(left),
                            name,
                        }),
                        args: self.parse_function_call_args()?,
                    }
                } else {
                    HirExpression::DotAccess {
                        expr: Box::new(left),
                        name,
                    }
                };
                self.parse_access_expression(right)
            }
            TokenType::LeftBracket => {
                self.expect_one()?;
                let index = self.parse_expression()?;
                self.expect(TokenType::RightBracket)?;
                let right = if self.peek()?.r#type == TokenType::LeftParen {
                    HirExpression::Call {
                        expr: Box::new(HirExpression::IndexAccess {
                            expr: Box::new(left),
                            index: Box::new(index),
                        }),
                        args: self.parse_function_call_args()?,
                    }
                } else {
                    HirExpression::IndexAccess {
                        expr: Box::new(left),
                        index: Box::new(index),
                    }
                };
                self.parse_access_expression(right)
            }
            _ => Ok(left),
        }
    }

    fn parse_function_call_args(&mut self) -> Result<Vec<HirExpression>> {
        self.expect(TokenType::LeftParen)?;
        let mut expressions = Vec::with_capacity(0);
        loop {
            if self.peek()?.r#type == TokenType::RightParen {
                break;
            }
            expressions.push(self.parse_expression()?);
            if self.maybe(TokenType::Comma)?.is_none() {
                break;
            }
        }
        self.expect(TokenType::RightParen)?;
        Ok(expressions)
    }

    fn parse_else(&mut self) -> Result<Option<HirBlock>> {
        match self.peek()?.r#type {
            TokenType::KwElseif => {
                self.expect_one()?;
                let cond = self.parse_expression()?;
                let block = self.parse_block()?;
                let else_block = self.parse_else()?;
                Ok(Some(HirBlock {
                    statements: vec![HirStatement::If {
                        cond,
                        block,
                        else_block,
                    }],
                }))
            }
            TokenType::KwElse => {
                self.expect_one()?;
                self.parse_block().map(Some)
            }
            _ => Ok(None),
        }
    }
}

fn apply_unary(op: Token, arg: HirExpression) -> HirExpression {
    HirExpression::UnaryOp {
        op,
        arg: Box::new(arg),
    }
}

fn apply_binary(op: Token, lhs: HirExpression, rhs: HirExpression) -> HirExpression {
    HirExpression::BinaryOp {
        op,
        args: Box::new([lhs, rhs]),
    }
}
