const m = @import("./mods.zig");
const std = m.std;
const print = std.debug.print;
const util = m.util;
const lexer = m.lexer;
const hir = m.hir.hir;
const parser = m.hir.parser;
const testing = std.testing;

test "debug lexer" {
    const src =
        \\module test;
        \\
        \\fun test() {
        \\  println("Hello world!");
        \\}
    ;
    var test_parser = parser.Parser.init(src);
    try test_parser.parse();
    const result = test_parser.ast;
    print("MODULES: {any}\n", .{result.modules.items});
    print("IMPORTS: {any}\n", .{result.imports.items});
    print("TYPES: {any}\n", .{result.types.items});
    print("FUNCTIONS: {any}\n", .{result.functions.items});
}

test "literals" {
    const src =
        \\10 10.5 "a\"\\\n\r\t\x00"
    ;
    var lex = lexer.Lexer.init(src);
    var tok = (try lex.nextToken()).?;
    try testing.expectEqual(tok.type, lexer.TokenType.integer);
    try testing.expectEqualStrings(tok.slice, src[0..2]);
    tok = (try lex.nextToken()).?;
    try testing.expectEqual(tok.type, lexer.TokenType.float);
    try testing.expectEqualStrings(tok.slice, src[3..7]);
    tok = (try lex.nextToken()).?;
    try testing.expectEqual(tok.type, lexer.TokenType.string);
    try testing.expectEqualStrings(tok.slice, src[8..25]);
}

test "non ASCII" {
    var lex = lexer.Lexer.init(
        \\ä
    );
    try testing.expectError(util.Error.InvalidToken, lex.nextToken());
}
