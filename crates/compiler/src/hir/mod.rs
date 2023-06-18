pub mod parser;

use crate::prelude::*;

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
pub struct HirModule {
    pub name: Str,
}

#[derive(Debug)]
pub struct HirImport {
    pub parts: Vec<Str>,
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
pub enum HirStatement {}
