use crate::{lexer::Token, prelude::*};

pub mod parser;

#[derive(Debug)]
pub struct Hir {
    pub modules: Vec<HirModule>,
    pub imports: Vec<HirImport>,
    pub constants: Vec<HirConst>,
    pub types: Vec<HirTypeDecl>,
    pub impls: Vec<HirImpl>,
    pub functions: Vec<HirFunction>,
}

impl Default for Hir {
    fn default() -> Self {
        Self {
            modules: Vec::with_capacity(0),
            imports: Vec::with_capacity(0),
            constants: Vec::with_capacity(0),
            types: Vec::with_capacity(0),
            impls: Vec::with_capacity(0),
            functions: Vec::with_capacity(0),
        }
    }
}

#[derive(Debug)]
pub struct HirPath {
    pub parts: Vec<Str>,
}

// todo: generics
#[derive(Debug)]
pub enum HirType {
    SelfType,
    Direct { path: HirPath },
    Reference { r#type: Box<HirType> },
    ConstReference { r#type: Box<HirType> },
}

#[derive(Debug)]
pub struct HirModule {
    pub name: Str,
    pub public: bool,
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

// todo: generics
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

// todo: generics
#[derive(Debug)]
pub struct HirImpl {
    pub target: HirType,
    pub r#trait: HirType,
    pub functions: Vec<HirFunction>,
}

// todo: generics
#[derive(Debug)]
pub struct HirFunction {
    pub name: Str,
    pub public: bool,
    pub params: Vec<HirFunctionParam>,
    pub return_type: Option<HirType>,
    pub body: Option<HirBlock>,
}

#[derive(Debug)]
pub struct HirStructField {
    pub name: Str,
    pub public: bool,
    pub r#type: HirType,
}

#[derive(Debug)]
pub enum HirEnumVariant {
    Empty {
        name: Str,
    },
    Tuple {
        name: Str,
        types: Vec<HirType>,
    },
    Struct {
        name: Str,
        fields: Vec<HirStructField>,
    },
}

#[derive(Debug)]
pub struct HirFunctionParam {
    pub name: Str,
    pub r#type: HirType,
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
    // todo: generics
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
    // todo: generics
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
