const spec = @import("../spec.zig");
const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;
const iter = @import("libs").iter;

// Will be in ascending order
const ParseOutput = []const [3]u32;

fn minmax(a: u32, b: u32) struct { u32, u32 } {
    return if (a < b) .{ a, b } else .{ b, a };
}

fn parse(input: []const u8, alloc: Allocator, io: Io) anyerror!ParseOutput {
    _ = io;
    var result = std.ArrayList([3]u32).empty;
    errdefer result.deinit(alloc);

    var input_iter = iter.Unsigned(32){ .buf = input };
    while (input_iter.next()) |first| {
        const a, const second = minmax(first, input_iter.next() orelse unreachable);
        const b, const c = minmax(second, input_iter.next() orelse unreachable);
        try result.append(alloc, [3]u32{ a, b, c });
    }
    return result.toOwnedSlice(alloc);
}

fn part1(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    const input = parsed.*;
    var acc: u64 = 0;
    for (input) |dims| {
        const smallest: u64 = dims[0] * dims[1];
        const side_sum = smallest + @as(u64, dims[0] * dims[2] + dims[1] * dims[2]);
        acc += 2 * side_sum + smallest;
    }
    _ = io;
    return std.fmt.allocPrint(alloc, "{d}", .{acc});
}

fn part2(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    _ = parsed;
    _ = alloc;
    _ = io;
    return error.DayNotImplemented;
}

fn deinit(parsed: *ParseOutput, alloc: Allocator, io: Io) void {
    _ = io;
    alloc.free(parsed.*);
}

pub const run = spec.makeDayRun(
    ParseOutput,
    parse,
    part1,
    null, // part2,
    deinit,
);

const TestCase = struct {
    input: []const u8,
    part1_solution: ?[]const u8 = null,
    part2_solution: ?[]const u8 = null,
};

test "run test cases" {
    const cases = [_]TestCase{
        .{ .input = "2x3x4\n1x1x10\n", .part1_solution = "101" },
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
