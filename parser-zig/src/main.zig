const std = @import("std");
const testing = std.testing;
pub const lexer = @import("./lexer.zig");

test "debug lexer" {
    var lex = lexer.Lexer.init(
        \\let x = y * z;
    );
    std.debug.print("len: {}", .{lex.source.len});
    while (true) {
        const elem = lex.next_token();
        if (elem.type == .eof) {
            break;
        }
        std.debug.print("Token: {any} '{s}'\n", .{ elem.type, elem.slice });
    }
}