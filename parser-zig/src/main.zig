const std = @import("std");
const testing = std.testing;
pub const lexer = @import("./lexer.zig");

test "debug lexer" {
    var lex = lexer.Lexer.init(
        \\fun test() {
        \\  println("Hello world!");
        \\}
    );
    while (true) {
        const elem = try lex.nextToken();
        std.debug.print("Token: {any} '{s}'\n", .{ elem.type, elem.slice });
        if (elem.type == .eof) {
            break;
        }
    }
}
