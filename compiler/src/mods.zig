pub const std = @import("std");
pub const util = @import("./util.zig");
pub const lexer = @import("./lexer.zig");
pub const hir = struct {
    pub const hir = @import("./hir/hir.zig");
    pub const parser = @import("./hir/parser.zig");
};
