const spec = @import("../spec.zig");
const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;

const ParseOutput = struct {};

fn parse(input: []const u8, alloc: Allocator, io: Io) anyerror!ParseOutput {
    _ = input;
    _ = alloc;
    _ = io;
    return error.DayNotImplemented;
}

fn part1(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    _ = parsed;
    _ = alloc;
    _ = io;
    return error.DayNotImplemented;
}

fn part2(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    _ = parsed;
    _ = alloc;
    _ = io;
    return error.DayNotImplemented;
}

fn deinit(parsed: *ParseOutput, alloc: Allocator) void {
    _ = parsed;
    _ = alloc;
}

pub const run = spec.makeDayRun(
    ParseOutput,
    parse,
    part1,
    null, // part2,
    null, // deinit,
);

const TestCase = struct {
    input: []const u8,
    part1_solution: ?[]const u8,
    part2_solution: ?[]const u8,
};

test "run test cases" {
    const cases = [_]TestCase{};
    const io = std.testing.io;
    const alloc = std.testing.allocator;
    for (cases) |*case| {
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
