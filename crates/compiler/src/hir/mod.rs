pub mod parser;

use crate::{lexer::Token, prelude::*};

#[derive(Debug)]
pub struct Hir {
    pub modules: Vec<HirModule>,
    pub imports: Vec<HirImport>,
    pub types: Vec<HirTypeDecl>,
    pub functions: Vec<HirFunction>,
}

impl Default for Hir {
    fn default() -> Self {
        Self {
            modules: Vec::with_capacity(0),
            imports: Vec::with_capacity(0),
            types: Vec::with_capacity(0),
            functions: Vec::with_capacity(0),
        }
    }
}

#[derive(Debug)]
pub struct HirType {
    pub name: Str,
}

#[derive(Debug)]
pub struct HirPath {
    pub parts: Vec<Str>,
}

#[derive(Debug)]
pub struct HirModule {
    pub name: Str,
}

#[derive(Debug)]
pub struct HirImport {
    pub path: HirPath,
}

#[derive(Debug)]
pub enum HirTypeDecl {
    Trait {
        name: Str,
        functions: Vec<HirFunction>,
    },
    Struct {
        name: Str,
        fields: Vec<HirStructField>,
        functions: Vec<HirFunction>,
    },
    Enum {
        name: Str,
        variants: Vec<HirEnumVariant>,
        functions: Vec<HirFunction>,
    },
}

#[derive(Debug)]
pub struct HirFunction {
    pub name: Str,
    pub public: bool,
    pub params: Vec<HirFunctionParam>,
    pub return_type: Option<HirType>,
    pub body: Option<HirBlock>,
}

#[derive(Debug)]
pub struct HirFunctionParam {
    pub name: Str,
    pub r#type: HirType,
}

#[derive(Debug)]
pub struct HirStructField {
    pub name: Str,
    pub r#type: Str,
}

#[derive(Debug)]
pub struct HirEnumVariant {
    pub name: Str,
    pub r#type: Option<HirType>,
}

#[derive(Debug)]
pub struct HirBlock {
    pub statements: Vec<HirStatement>,
}

#[derive(Debug)]
pub enum HirStatement {
    VarDecl {
        name: Str,
        r#type: Option<HirType>,
        expr: Option<HirExpression>,
    },
    ConstDecl {
        name: Str,
        r#type: Option<HirType>,
        expr: Option<HirExpression>,
    },
    Assign {
        name: Str,
        expr: HirExpression,
    },
    If {
        cond: HirExpression,
        block: HirBlock,
    },
    While {
        cond: HirExpression,
        block: HirBlock,
    },
    For {
        name: Str,
        expr: HirExpression,
        block: HirBlock,
    },
    Call {},
    DotCall {},
}

#[derive(Debug)]
pub enum HirExpression {
    Int {
        slice: Str,
    },
    Float {
        slice: Str,
    },
    String {
        slice: Str,
    },
    /// `sum(x)` with `x`
    Access {
        name: Str,
    },
    /// `sum(x.y)` with `x` and `y`
    DotAccess {
        expr: Box<HirExpression>,
        name: Str,
    },
    Call {
        name: Str,
        // todo
    },
    DotCall {
        expr: Box<HirExpression>,
        // todo
    },
    UnaryOp {
        op: Token,
        arg: Box<HirExpression>,
    },
    BinaryOp {
        op: Token,
        args: Box<[HirExpression; 2]>,
    },
}
