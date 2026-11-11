const spec = @import("../spec.zig");
const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;
const Point = @import("libs").Point;

const ParseOutput = std.ArrayList(Point);

fn parse(input: []const u8, alloc: Allocator, io: Io) anyerror!ParseOutput {
    _ = io;
    var result = try ParseOutput.initCapacity(alloc, input.len);
    for (input) |c| {
        result.appendAssumeCapacity(Point.fromDirectionChar(c) catch unreachable);
    }
    return result;
}

fn part1(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    _ = io;
    var point_set = std.AutoHashMap(Point, void).init(alloc);
    defer point_set.deinit();
    try point_set.ensureUnusedCapacity(@intCast(parsed.items.len + 1));

    var current = Point.ORIGIN;
    point_set.putAssumeCapacity(current, {});
    for (parsed.items) |p| {
        current = current.add(p);
        point_set.putAssumeCapacity(current, {});
    }
    return std.fmt.allocPrint(alloc, "{d}", .{point_set.unmanaged.size});
}

fn part2(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    _ = io;
    var point_set = std.AutoHashMap(Point, void).init(alloc);
    defer point_set.deinit();
    try point_set.ensureUnusedCapacity(@intCast(parsed.items.len + 1));

    var santa = Point.ORIGIN;
    var robo = Point.ORIGIN;
    point_set.putAssumeCapacity(Point.ORIGIN, {});
    var i: usize = 0;
    while (i + 1 < parsed.items.len) : (i += 2) {
        santa = santa.add(parsed.items[i]);
        robo = robo.add(parsed.items[i + 1]);
        point_set.putAssumeCapacity(santa, {});
        point_set.putAssumeCapacity(robo, {});
    }
    if (i < parsed.items.len) {
        point_set.putAssumeCapacity(santa.add(parsed.items[i]), {});
    }
    return std.fmt.allocPrint(alloc, "{d}", .{point_set.unmanaged.size});
}

fn deinit(parsed: *ParseOutput, alloc: Allocator, io: Io) void {
    _ = io;
    parsed.deinit(alloc);
}

pub const run = spec.makeDayRun(
    ParseOutput,
    parse,
    part1,
    part2,
    deinit,
);

const TestCase = struct {
    input: []const u8,
    part1_solution: ?[]const u8 = null,
    part2_solution: ?[]const u8 = null,
};

test "run test cases" {
    const cases = [_]TestCase{
        .{ .input = ">", .part1_solution = "2" },
        .{ .input = "^v", .part2_solution = "3" },
        .{ .input = "^>v<", .part1_solution = "4", .part2_solution = "3" },
        .{ .input = "^v^v^v^v^v", .part1_solution = "2", .part2_solution = "11" },
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
