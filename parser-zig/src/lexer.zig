const std = @import("std");
const isWhitespace = std.ascii.isWhitespace;
const isDigit = std.ascii.isDigit;
const isAlphanumeric = std.ascii.isAlphanumeric;

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

    pub fn skip_whitespace(self: *Self) void {
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

    pub fn next_token(self: *Self) Token {
        self.skip_whitespace();
        const index = self.index;
        const c = self.peek();
        self.eat();
        const token_type: TokenType = switch (c) {
            0 => {
                const last_index = self.source.len - 1;
                return Token.init(.eof, self.source[last_index..last_index]);
            },
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
                            // todo: error (float with >1 dot)
                            unreachable;
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
                    // todo: error (only a `.`)
                    unreachable;
                }
                if (is_float) {
                    break :blk .float;
                } else {
                    break :blk .integer;
                }
            },
            '"' => {
                unreachable;
            },
            'a'...'z', 'A'...'Z', '_' => blk: {
                while (true) {
                    const ac = self.peek();
                    if (ac == 0 or (!isAlphanumeric(ac) and ac != '_')) {
                        break;
                    }
                    self.eat();
                }
                break :blk .identifier;
            },
            else => {
                unreachable;
            },
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
    // --- Double width ---
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
    eof,
};
