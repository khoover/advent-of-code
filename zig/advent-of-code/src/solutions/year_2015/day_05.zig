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

    const VOWEL_MASK: u32 = comptime 1 | (1 << 4) | (1 << 8) | (1 << 14) | (1 << 20);
    const ILLEGAL_PAIR: u32 = comptime (1 << 1) | (1 << 3) | (1 << 16) | (1 << 24);

    var nice: u64 = 0;
    var iter = std.mem.splitScalar(u8, parsed.*, '\n');

    outer: while (iter.next()) |line| {
        var previous: u32 = 0;
        var pair: bool = false;
        var vowel_count: u32 = 0;

        for (line) |c| {
            std.debug.assert(c >= 'a' and c <= 'z');
            const offset: u5 = @intCast(c - 'a');
            const mask = @as(u32, 1) << offset;
            if (previous << 1 & mask & ILLEGAL_PAIR != 0) continue :outer;
            pair |= previous == mask;
            vowel_count += @intFromBool(VOWEL_MASK & mask != 0);
            previous = mask;
        }
        if (pair and vowel_count >= 3) nice += 1;
    }

    return std.fmt.allocPrint(alloc, "{d}", .{nice});
}

fn part2(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    _ = io;

    // Idea is to store the index of each pair in here; because lines aren't too long, we can do a trick where
    // we add 100 to the base index every iter. That way we don't need to re-zero the whole thing, we just see
    // if the distance between pairs is too far to be on the same line.
    var pair_index: [27 * 27]usize = @splat(0);
    const LINE_DISTANCE: usize = 100;
    var nice: u64 = 0;
    var iter = std.mem.splitScalar(u8, parsed.*, '\n');
    var line_idx: usize = 100;

    while (iter.next()) |line| {
        defer line_idx += LINE_DISTANCE;
        var two_pair = false;
        var aba = false;
        var first: u8 = 0;
        var second: u8 = 0;

        for (line, 0..) |c, i| {
            std.debug.assert(c >= 'a' and c <= 'z');
            std.debug.assert(i < LINE_DISTANCE);
            const offset = c - 'a' + 1;
            defer {
                first = second;
                second = offset;
            }

            aba |= first == offset;

            const idx = 27 * @as(usize, second) + offset;
            const pos = line_idx + i;
            const delta = pos - pair_index[idx];
            if (delta > i) {
                pair_index[idx] = pos;
            } else if (delta > 1) {
                two_pair = true;
            }
        }

        nice += @intFromBool(two_pair and aba);
    }

    return std.fmt.allocPrint(alloc, "{d}", .{nice});
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
        .{ .input = "ugknbfddgicrmopn", .part1_solution = "1" },
        .{ .input = "aaa\njchzalrnumimnmhp", .part1_solution = "1" },
        .{ .input = "haegwjzuvuyypxyu", .part1_solution = "0" },
        .{ .input = "dvszwmarrgswjxmb", .part1_solution = "0" },
        .{ .input = "qjhvhtzxzqqjkmpb", .part2_solution = "1" },
        .{ .input = "xxyxx\nuurcxstgmygtbstg", .part2_solution = "1" },
        .{ .input = "ieodomkazucvgmuy", .part2_solution = "0" },
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
