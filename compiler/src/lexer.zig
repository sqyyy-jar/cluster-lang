const m = @import("./mods.zig");
const std = m.std;
const isWhitespace = std.ascii.isWhitespace;
const isDigit = std.ascii.isDigit;
const isAlphanumeric = std.ascii.isAlphanumeric;
const util = m.util;

const ident_map = std.ComptimeStringMap(TokenType, .{ //
    .{ "module", .kw_module },
    .{ "import", .kw_import },
    .{ "trait", .kw_trait },
    .{ "struct", .kw_struct },
    .{ "enum", .kw_enum },
    .{ "impl", .kw_impl },
    .{ "fun", .kw_fun },
    .{ "const", .kw_const },
    .{ "var", .kw_var },
    .{ "for", .kw_for },
    .{ "while", .kw_while },
    .{ "in", .kw_in },
    .{ "return", .kw_return },
    .{ "continue", .kw_continue },
    .{ "break", .kw_break },
});

pub const Lexer = struct {
    const Self = @This();

    source: []const u8,
    index: usize,

    pub fn init(source: []const u8) Self {
        return Self{
            .source = source,
            .index = 0,
        };
    }

    pub fn peek(self: *Self) u8 {
        if (self.index >= self.source.len) {
            return 0;
        }
        return self.source[self.index];
    }

    pub fn eat(self: *Self) void {
        self.index += 1;
    }

    pub fn skipWhitespace(self: *Self) void {
        while (true) {
            const c = self.peek();
            if (isWhitespace(c)) {
                self.eat();
                continue;
            }
            break;
        }
    }

    pub fn slice(self: *Self, from: usize) []const u8 {
        return self.source[from..self.index];
    }

    pub fn nextToken(self: *Self) util.Error!?Token {
        self.skipWhitespace();
        const index = self.index;
        const c = self.peek();
        self.eat();
        const token_type: TokenType = switch (c) {
            0 => return null,
            // const last_index = self.source.len - 1;
            // return Token.init(.eof, self.source[last_index..last_index]);
            '(' => .left_paren,
            ')' => .right_paren,
            '[' => .left_bracket,
            ']' => .right_bracket,
            '{' => .left_brace,
            '}' => .right_brace,
            ':' => .colon,
            ';' => .semicolon,
            '@' => .at,
            '#' => .hashtag,
            ',' => .comma,
            '!' => blk: {
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    break :blk .bang_equal;
                }
                break :blk .bang;
            },
            '&' => blk: {
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    break :blk .and_equal;
                }
                break :blk .and_sign;
            },
            '|' => blk: {
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    break :blk .pipe_equal;
                }
                break :blk .pipe;
            },
            '^' => blk: {
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    break :blk .caret_equal;
                }
                break :blk .caret;
            },
            '+' => blk: {
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    break :blk .plus_equal;
                }
                break :blk .plus;
            },
            '-' => blk: {
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    break :blk .minus_equal;
                } else if (ac == '>') {
                    self.eat();
                    break :blk .arrow;
                }
                break :blk .minus;
            },
            '*' => blk: {
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    break :blk .star_equal;
                }
                break :blk .star;
            },
            '/' => blk: {
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    break :blk .slash_equal;
                } else if (ac == '/') {
                    self.eat();
                    while (true) {
                        const bc = self.peek();
                        if (bc == '\n' or bc == 0) {
                            break;
                        }
                        self.eat();
                    }
                    return self.nextToken();
                }
                break :blk .slash;
            },
            '%' => blk: {
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    break :blk .percent_equal;
                }
                break :blk .percent;
            },
            '=' => blk: {
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    break :blk .equal_equal;
                }
                break :blk .equal;
            },
            '<' => blk: {
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    break :blk .less_equal;
                }
                break :blk .less;
            },
            '>' => blk: {
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    break :blk .greater_equal;
                }
                break :blk .greater;
            },
            '.', '0'...'9' => blk: {
                var is_float = c == '.';
                const ac = self.peek();
                if (c == '.' and ac == '.') {
                    self.eat();
                    const bc = self.peek();
                    if (bc == '.') {
                        self.eat();
                        break :blk .dot_dot_dot;
                    }
                    break :blk .dot_dot;
                }
                if (is_float and !isDigit(ac)) {
                    break :blk .dot;
                }
                while (true) {
                    const bc = self.peek();
                    if (bc == '.') {
                        if (is_float) {
                            return error.InvalidFloat;
                        }
                        self.eat();
                        is_float = true;
                        continue;
                    }
                    if (!isDigit(bc)) {
                        break;
                    }
                    self.eat();
                }
                if (is_float and self.index - index < 2) {
                    break :blk .dot;
                }
                if (is_float) {
                    break :blk .float;
                } else {
                    break :blk .integer;
                }
            },
            '"' => blk: {
                while (true) {
                    const ac = self.peek();
                    if (ac == 0) {
                        return error.UnexpectedEof;
                    }
                    if (ac == '\\') {
                        self.eat();
                        const bc = self.peek();
                        switch (bc) {
                            '"', '\\', 'n', 'r', 't' => self.eat(),
                            'x' => {
                                self.eat();
                                const cc = self.peek();
                                if (cc == 0) {
                                    return error.UnexpectedEof;
                                }
                                switch (cc) {
                                    '0'...'9', 'a'...'f', 'A'...'F' => {
                                        self.eat();
                                    },
                                    else => return error.InvalidEscapeSequence,
                                }
                                const dc = self.peek();
                                if (dc == 0) {
                                    return error.UnexpectedEof;
                                }
                                switch (dc) {
                                    '0'...'9', 'a'...'f', 'A'...'F' => {
                                        self.eat();
                                    },
                                    else => return error.InvalidEscapeSequence,
                                }
                            },
                            else => return error.InvalidEscapeSequence,
                        }
                        continue;
                    }
                    if (ac == '"') {
                        self.eat();
                        break;
                    }
                    self.eat();
                }
                break :blk .string;
            },
            'a'...'z', 'A'...'Z', '_' => blk: {
                while (true) {
                    const ac = self.peek();
                    if (ac == 0 or (!isAlphanumeric(ac) and ac != '_')) {
                        break;
                    }
                    self.eat();
                }
                const ident = self.slice(index);
                if (ident_map.get(ident)) |token_type| {
                    break :blk token_type;
                }
                break :blk .identifier;
            },
            else => return error.InvalidToken,
        };
        return Token.init(token_type, self.slice(index));
    }
};

pub const Token = struct {
    const Self = @This();

    type: TokenType,
    slice: []const u8,

    pub fn init(@"type": TokenType, slice: []const u8) Self {
        return Self{
            .type = @"type",
            .slice = slice,
        };
    }
};

pub const TokenType = enum {
    // --- Brackets ---
    left_paren,
    right_paren,
    left_bracket,
    right_bracket,
    left_brace,
    right_brace,
    // --- Punctuation ---
    colon,
    semicolon,
    at,
    hashtag,
    dot,
    dot_dot,
    dot_dot_dot,
    arrow,
    comma,
    bang,
    and_sign,
    pipe,
    caret,
    plus,
    minus,
    star,
    slash,
    percent,
    equal,
    less,
    greater,
    // --- Equal combined ---
    bang_equal,
    and_equal,
    pipe_equal,
    caret_equal,
    plus_equal,
    minus_equal,
    star_equal,
    slash_equal,
    percent_equal,
    equal_equal,
    less_equal,
    greater_equal,
    // --- Literals ---
    integer,
    float,
    string,
    identifier,
    // --- Keywords ---
    kw_module,
    kw_import,
    kw_pub,
    kw_trait,
    kw_struct,
    kw_enum,
    kw_impl,
    kw_fun,
    kw_const,
    kw_var,
    kw_for,
    kw_while,
    kw_in,
    kw_return,
    kw_continue,
    kw_break,
    // eof,
};
