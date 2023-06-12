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

    pub fn next_token(self: *Self) Token {
        const index = self.index;
        const c = self.peek();
        switch (c) {
            0 => {
                return Token.init(.eof, self.source[self.source.len..]);
            },
            '(' => {
                self.eat();
                return Token.init(.left_paren, self.source[index..self.index]);
            },
            ')' => {
                self.eat();
                return Token.init(.right_paren, self.source[index..self.index]);
            },
            '[' => {
                self.eat();
                return Token.init(.left_bracket, self.source[index..self.index]);
            },
            ']' => {
                self.eat();
                return Token.init(.right_bracket, self.source[index..self.index]);
            },
            '{' => {
                self.eat();
                return Token.init(.left_brace, self.source[index..self.index]);
            },
            '}' => {
                self.eat();
                return Token.init(.right_brace, self.source[index..self.index]);
            },
            ':' => {
                self.eat();
                return Token.init(.colon, self.source[index..self.index]);
            },
            ';' => {
                self.eat();
                return Token.init(.semicolon, self.source[index..self.index]);
            },
            '@' => {
                self.eat();
                return Token.init(.at, self.source[index..self.index]);
            },
            '#' => {
                self.eat();
                return Token.init(.hashtag, self.source[index..self.index]);
            },
            ',' => {
                self.eat();
                return Token.init(.comma, self.source[index..self.index]);
            },
            '!' => {
                self.eat();
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    return Token.init(.bang_equal, self.source[index..self.index]);
                }
                return Token.init(.bang, self.source[index..self.index]);
            },
            '&' => {
                self.eat();
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    return Token.init(.and_equal, self.source[index..self.index]);
                }
                return Token.init(.and_sign, self.source[index..self.index]);
            },
            '|' => {
                self.eat();
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    return Token.init(.pipe_equal, self.source[index..self.index]);
                }
                return Token.init(.pipe, self.source[index..self.index]);
            },
            '^' => {
                self.eat();
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    return Token.init(.caret_equal, self.source[index..self.index]);
                }
                return Token.init(.caret, self.source[index..self.index]);
            },
            '+' => {
                self.eat();
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    return Token.init(.plus_equal, self.source[index..self.index]);
                }
                return Token.init(.plus, self.source[index..self.index]);
            },
            '-' => {
                self.eat();
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    return Token.init(.minus_equal, self.source[index..self.index]);
                }
                return Token.init(.minus, self.source[index..self.index]);
            },
            '*' => {
                self.eat();
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    return Token.init(.star_equal, self.source[index..self.index]);
                }
                return Token.init(.star, self.source[index..self.index]);
            },
            '/' => {
                self.eat();
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    return Token.init(.slash_equal, self.source[index..self.index]);
                }
                return Token.init(.slash, self.source[index..self.index]);
            },
            '%' => {
                self.eat();
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    return Token.init(.percent_equal, self.source[index..self.index]);
                }
                return Token.init(.percent, self.source[index..self.index]);
            },
            '=' => {
                self.eat();
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    return Token.init(.equal_equal, self.source[index..self.index]);
                }
                return Token.init(.equal, self.source[index..self.index]);
            },
            '<' => {
                self.eat();
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    return Token.init(.less_equal, self.source[index..self.index]);
                }
                return Token.init(.less, self.source[index..self.index]);
            },
            '>' => {
                self.eat();
                const ac = self.peek();
                if (ac == '=') {
                    self.eat();
                    return Token.init(.greater_equal, self.source[index..self.index]);
                }
                return Token.init(.greater, self.source[index..self.index]);
            },
            '.', '0'...'9' => {
                var is_float = c == '.';
                self.eat();
                const ac = self.peek();
                if (is_float and !isDigit(ac)) {
                    return Token.init(.dot, self.source[index..self.index]);
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
                    return Token.init(.float, self.source[index..self.index]);
                } else {
                    return Token.init(.integer, self.source[index..self.index]);
                }
            },
            '"' => {
                unreachable;
            },
            'a'...'z', 'A'...'Z', '_' => {
                while (true) {
                    const ac = self.peek();
                    if (ac == 0 or (!isAlphanumeric(ac) and ac != '_')) {
                        break;
                    }
                    self.eat();
                }
                return Token.init(.identifier, self.source[index..self.index]);
            },
            else => {
                if (isWhitespace(c)) {
                    self.skip_whitespace();
                    return self.next_token();
                }
                unreachable;
            },
        }
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
