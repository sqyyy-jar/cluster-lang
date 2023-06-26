use crate::{lexer::Token, prelude::*};

pub mod parser;

#[derive(Debug)]
pub struct Hir {
    pub modules: Vec<HirModule>,
    pub imports: Vec<HirImport>,
    pub constants: Vec<HirConst>,
    pub types: Vec<HirTypeDecl>,
    pub functions: Vec<HirFunction>,
}

impl Default for Hir {
    fn default() -> Self {
        Self {
            modules: Vec::with_capacity(0),
            imports: Vec::with_capacity(0),
            constants: Vec::with_capacity(0),
            types: Vec::with_capacity(0),
            functions: Vec::with_capacity(0),
        }
    }
}

// todo: generics
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
pub struct HirConst {
    pub name: Str,
    pub public: bool,
    pub r#type: Option<HirType>,
    pub expr: Option<HirExpression>,
}

#[derive(Debug)]
pub enum HirTypeDecl {
    Trait {
        name: Str,
        public: bool,
        functions: Vec<HirFunction>,
    },
    Struct {
        name: Str,
        public: bool,
        fields: Vec<HirStructField>,
        functions: Vec<HirFunction>,
    },
    Enum {
        name: Str,
        public: bool,
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
    pub public: bool,
    pub r#type: HirType,
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
    /// `var name: type = expr;`
    VarDecl {
        name: Str,
        r#type: Option<HirType>,
        expr: Option<HirExpression>,
    },
    /// `const name: type = expr;`
    ConstDecl {
        name: Str,
        r#type: Option<HirType>,
        expr: Option<HirExpression>,
    },
    /// `expr = value;`
    Assign {
        expr: HirExpression,
        value: HirExpression,
    },
    /// `if cond block else else_block`
    If {
        cond: HirExpression,
        block: HirBlock,
        else_block: Option<HirBlock>,
    },
    /// `while cond block`
    While {
        cond: HirExpression,
        block: HirBlock,
    },
    /// `for name in expr block`
    For {
        name: Str,
        expr: HirExpression,
        block: HirBlock,
    },
    /// `expr(args);`
    Call {
        expr: HirExpression,
        args: Vec<HirExpression>,
    },
    /// `return expr;`
    Return { expr: HirExpression },
    /// `continue;`
    Continue,
    /// `break;`
    Break,
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
    /// `name`
    Access {
        name: Str,
    },
    /// `expr.name`
    DotAccess {
        expr: Box<HirExpression>,
        name: Str,
    },
    /// `expr[index]`
    IndexAccess {
        expr: Box<HirExpression>,
        index: Box<HirExpression>,
    },
    /// `expr(args)`
    Call {
        expr: Box<HirExpression>,
        args: Vec<HirExpression>,
    },
    /// `op arg`
    UnaryOp {
        op: Token,
        arg: Box<HirExpression>,
    },
    /// `args[0] op args[1]`
    BinaryOp {
        op: Token,
        args: Box<[HirExpression; 2]>,
    },
}
