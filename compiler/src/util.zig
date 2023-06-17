pub const Error = error{
    UnexpectedEof,
    UnexpectedToken,
    InvalidEscapeSequence,
    InvalidFloat,
    InvalidToken,
};

pub const FullError = enum {
    unexpected_eof,
    unexpected_token,
    invalid_escape_sequence,
    invalid_float,
    invalid_token,
};

pub fn Result(comptime T: type, comptime E: type) type {
    return union(enum) {
        const Self = @This();

        ok: T,
        err: E,

        pub fn as_ok(self: Self) ?T {
            return switch (self) {
                .ok => self.ok,
                else => null,
            };
        }

        pub fn as_err(self: Self) ?E {
            return switch (self) {
                .err => self.err,
                else => null,
            };
        }
    };
}
