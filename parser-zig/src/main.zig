const std = @import("std");
const testing = std.testing;
pub const lexer = @import("./lexer.zig");

test "debug lexer" {
    var lex = lexer.Lexer.init(
        \\fun test() {
        \\  println("Hello world!");
        \\}
    );
    std.debug.print("len: {}", .{lex.source.len});
    while (true) {
        const elem = lex.next_token();
        std.debug.print("Token: {any} '{s}'\n", .{ elem.type, elem.slice });
        if (elem.type == .eof) {
            break;
        }
    }
}
