const std = @import("std");
const lexer = @import("./lexer.zig");
const testing = std.testing;

test "debug lexer" {
    var lex = lexer.Lexer.init(
        \\fun test() {
        \\  println("Hello world!");
        \\}
    );
    while (try lex.nextToken()) |tok| {
        std.debug.print("Token: {any} '{s}'\n", .{ tok.type, tok.slice });
    }
}

test "literals" {
    const src =
        \\10 10.5 "a\"\\\n\r\t\x00"
    ;
    var lex = lexer.Lexer.init(src);
    var tok = try lex.nextToken();
    try testing.expectEqual(tok.type, lexer.TokenType.integer);
    try testing.expectEqualStrings(tok.slice, src[0..2]);
    tok = try lex.nextToken();
    try testing.expectEqual(tok.type, lexer.TokenType.float);
    try testing.expectEqualStrings(tok.slice, src[3..7]);
    tok = try lex.nextToken();
    try testing.expectEqual(tok.type, lexer.TokenType.string);
    try testing.expectEqualStrings(tok.slice, src[8..25]);
}

test "non ASCII" {
    var lex = lexer.Lexer.init(
        \\ä
    );
    try testing.expectError(lexer.LexerError.InvalidToken, lex.nextToken());
}
