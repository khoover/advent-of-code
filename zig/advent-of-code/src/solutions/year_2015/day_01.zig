const spec = @import("../spec.zig");
const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;

const ParseOutput = []const u8;

fn parse(input: []const u8, alloc: Allocator, io: Io) anyerror!ParseOutput {
    _ = alloc;
    _ = io;
    return std.mem.trim(u8, input, &std.ascii.whitespace);
}

fn part1(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    _ = io;

    const input = parsed.*;
    var counter: i64 = 0;
    for (input) |c| switch (c) {
        '(' => counter += 1,
        ')' => counter -= 1,
        else => unreachable,
    };

    return std.fmt.allocPrint(alloc, "{d}", .{counter});
}

fn part2(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    _ = io;

    const input = parsed.*;
    var counter: u64 = 0;
    var index: usize = undefined;
    for (input, 1..) |c, i| switch (c) {
        '(' => counter += 1,
        ')' => {
            counter, const underflowed = @subWithOverflow(counter, 1);
            if (underflowed == 1) {
                @branchHint(.unlikely);
                index = i;
                break;
            }
        },
        else => unreachable,
    };

    return std.fmt.allocPrint(alloc, "{d}", .{index});
}

fn deinit(parsed: *ParseOutput, alloc: Allocator) void {
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
        .{ .input = "(())", .part1_solution = "0" },
        .{ .input = "()()", .part1_solution = "0" },
        .{ .input = ")", .part2_solution = "1" },
        .{ .input = "()())", .part2_solution = "5" },
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
