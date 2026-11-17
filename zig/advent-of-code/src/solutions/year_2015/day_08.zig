const spec = @import("../spec.zig");
const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;

const ParseOutput = []const u8;

fn parse(input: []const u8, alloc: Allocator, io: Io) anyerror!ParseOutput {
    _ = alloc;
    _ = io;
    return input;
}

fn part1(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    _ = io;
    const input = parsed.*;
    var index: usize = 1;
    var delta: u32 = 0;
    while (index < input.len) {
        switch (input[index]) {
            '\\' => {
                if (input[index + 1] == 'x') {
                    delta += 3;
                    index += 4;
                } else {
                    delta += 1;
                    index += 2;
                }
            },
            '"' => {
                delta += 2;
                index += 3;
            },
            else => {
                index += 1;
            },
        }
    }
    return std.fmt.allocPrint(alloc, "{d}", .{delta});
}

fn part2(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    _ = io;
    const input = std.mem.trimEnd(u8, parsed.*, "\n");
    var delta: u32 = 0;
    for (input) |c| {
        switch (c) {
            '\\', '"' => {
                delta += 1;
            },
            '\n' => {
                delta += 2;
            },
            else => {},
        }
    }
    return std.fmt.allocPrint(alloc, "{d}", .{delta + 2});
}

fn deinit(parsed: *ParseOutput, alloc: Allocator, io: Io) void {
    _ = io;
    _ = parsed;
    _ = alloc;
}

pub const run = spec.makeDayRun(
    ParseOutput,
    parse,
    part1,
    part2,
    null, // deinit,
);

const TestCase = struct {
    input: []const u8,
    part1_solution: ?[]const u8 = null,
    part2_solution: ?[]const u8 = null,
};

test "run test cases" {
    const cases = [_]TestCase{
        .{ .input = "\"\"\n\"abc\"\n\"aaa\\\"aaa\"\n\"\\x27\"\n", .part1_solution = "12", .part2_solution = "19" },
    };
    const io = std.testing.io;
    const alloc = std.testing.allocator;
    for (&cases) |*case| {
        const result = try run(case.input, alloc, io);
        defer result.deinit(alloc);
        if (case.part1_solution) |part1_solution| {
            try std.testing.expectEqualStrings(part1_solution, result.part1.solution);
        }
        if (case.part2_solution) |part2_solution| {
            const result_part2 = result.part2 orelse return error.TestExpectedNonNull;
            try std.testing.expectEqualStrings(part2_solution, result_part2.solution);
        }
    }
}
