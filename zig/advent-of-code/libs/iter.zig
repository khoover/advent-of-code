const ascii = @import("std").ascii;

/// Iterate over an input buffer, pulling out contiguous sequences of digits
/// as unsigned integers.
pub fn Unsigned(comptime width: usize) type {
    return struct {
        const Result: type = @Int(.unsigned, width);

        buf: []const u8,
        /// Can change this to
        cur_index: usize = 0,

        pub fn next(self: *@This()) ?Result {
            while (self.cur_index < self.buf.len and
                !ascii.isDigit(self.buf[self.cur_index])) : (self.cur_index += 1)
            {}
            if (self.cur_index >= self.buf.len) return null;

            const Acc = @Int(.unsigned, @max(8, width));
            var acc: Acc = self.buf[self.cur_index] -% '0';
            self.cur_index += 1;
            while (self.cur_index < self.buf.len and
                ascii.isDigit(self.buf[self.cur_index])) : (self.cur_index += 1)
            {
                acc = 10 * acc + (self.buf[self.cur_index] -% '0');
            }
            // We know cur_index is either OOB or not a digit, so we increment;
            // either still OOB or we eliminate an unnecessary check next iter.
            self.cur_index += 1;
            return @intCast(acc);
        }
    };
}

test Unsigned {
    const testing = @import("std").testing;

    var iter = Unsigned(8){ .buf = "Hello, 42 is 100-0" };

    try testing.expectEqual(iter.next(), 42);
    try testing.expectEqual(iter.next(), 100);
    try testing.expectEqual(iter.next(), 0);
    try testing.expectEqual(iter.next(), null);
}
