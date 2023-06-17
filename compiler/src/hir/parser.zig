const m = @import("../mods.zig");
const std = m.std;
const util = m.util;
const lexer = m.lexer;
const hir = m.hir.hir;

// parse
//  on error
//   -> errors.append(err)
//   -> return err

pub const Parser = struct {
    const Self = @This();

    errors: std.ArrayList(util.FullError),
    lex: lexer.Lexer,
    ast: hir.Hir,

    pub fn init(src: []const u8) Self {
        const alloc = std.heap.page_allocator;
        return Self{
            .errors = std.ArrayList(util.FullError).init(alloc),
            .lex = lexer.Lexer.init(src),
            .ast = hir.Hir.init(alloc),
        };
    }

    pub fn expect(self: *Self, type_: lexer.TokenType) util.Error!lexer.Token {
        const tok = try self.lex.nextToken();
        if (tok == null) {
            return error.UnexpectedEof;
        }
        if (tok.?.type != type_) {
            return error.UnexpectedToken;
        }
        return tok.?;
    }

    pub fn parse(self: *Self) util.Error!void {
        while (try self.lex.nextToken()) |tok| {
            switch (tok.type) {
                .kw_module => {
                    const ident = try self.expect(.identifier);
                    _ = try self.expect(.semicolon);
                    self.ast.modules.append(.{ .name = ident.slice }) catch unreachable;
                },
                else => {
                    std.debug.print("UNHANDLED TOKEN: {any} ('{s}')\n", .{ tok.type, tok.slice });
                },
            }
        }
    }
};
