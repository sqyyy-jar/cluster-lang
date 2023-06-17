const lexer = @import("../lexer.zig");
const hir = @import("./hir.zig");

pub fn parse(src: []const u8) hir.HirError!hir.Hir {
    var lex = lexer.Lexer.init(src);
    while (true) {}
    _ = lex;
}
