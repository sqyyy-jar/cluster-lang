const std = @import("std");

pub const Hir = struct {
    const Self = @This();

    modules: std.ArrayList(HirModule),
    imports: std.ArrayList(HirImport),
    types: std.ArrayList(HirType),
    functions: std.ArrayList(HirFunction),
};

// todo
pub const HirModule = struct {
    statement: []const u8,
    name: []const u8,
};

// todo
pub const HirImport = struct {
    name: []const u8,
};

// todo
pub const HirType = union(enum) {
    trait_type: struct {},
    struct_type: struct {},
    enum_type: struct {},
};

// todo
pub const HirFunction = struct {};
