const m = @import("../mods.zig");
const std = m.std;
const lexer = m.lexer;

pub const Hir = struct {
    const Self = @This();

    modules: std.ArrayList(HirModule),
    imports: std.ArrayList(HirImport),
    types: std.ArrayList(HirTypeDecl),
    functions: std.ArrayList(HirFunction),

    pub fn init(alloc: std.mem.Allocator) Self {
        return Self{
            .modules = std.ArrayList(HirModule).init(alloc),
            .imports = std.ArrayList(HirImport).init(alloc),
            .types = std.ArrayList(HirTypeDecl).init(alloc),
            .functions = std.ArrayList(HirFunction).init(alloc),
        };
    }
};

// todo
// - add generics
pub const HirType = struct {
    name: []const u8,
};

// todo
pub const HirModule = struct {
    name: []const u8,
};

// todo
pub const HirImport = struct {
    name: []const u8,
};

pub const HirImportNode = struct {};

// todo
pub const HirTypeDecl = union(enum) {
    trait_type: struct {
        name: []const u8,
        functions: std.ArrayList(HirFunction),
    },
    struct_type: struct {
        name: []const u8,
        fields: std.ArrayList(HirStructField),
        functions: std.ArrayList(HirFunction),
    },
    enum_type: struct {
        name: []const u8,
        variants: std.ArrayList(HirEnumVariant),
        functions: std.ArrayList(HirFunction),
    },
};

// todo
pub const HirFunction = struct {
    name: []const u8,
    params: std.ArrayList(HirFunctionParam),
    return_type: ?HirType,
    body: ?HirBlock,
};

// todo
pub const HirFunctionParam = struct {
    name: []const u8,
    type: HirType,
};

// todo
pub const HirStructField = struct {
    name: []const u8,
    type: HirType,
};

// todo
pub const HirEnumVariant = struct {
    name: []const u8,
    type: ?HirType,
};

// todo
pub const HirBlock = struct {
    statements: std.ArrayList(HirStatement),
};

// todo
pub const HirStatement = union(enum) {
    call: struct {},
    _return: struct {},
};
