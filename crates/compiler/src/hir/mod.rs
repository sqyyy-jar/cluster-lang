use crate::prelude::*;

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

pub struct HirType {
    pub name: Str,
}

pub struct HirModule {
    pub name: Str,
}

pub struct HirImport {}

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

pub struct HirFunction {
    pub name: Str,
    pub params: Vec<HirFunctionParam>,
    pub return_type: Option<HirType>,
    pub body: Option<HirBlock>,
}

pub struct HirFunctionParam {
    pub name: Str,
    pub r#type: HirType,
}

pub struct HirStructField {
    pub name: Str,
    pub r#type: Str,
}

pub struct HirEnumVariant {
    pub name: Str,
    pub r#type: Option<HirType>,
}

pub struct HirBlock {
    pub statements: Vec<HirStatement>,
}

pub enum HirStatement {}
