const spec = @import("../spec.zig");
const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;

const ParseOutput = struct { u32, u32 };

fn parse(input: []const u8, alloc: Allocator, io: Io) anyerror!ParseOutput {
    _ = io;
    var tokens = std.mem.tokenizeAny(u8, input, &std.ascii.whitespace);
    var name_mapping = std.StringHashMap(u8).init(alloc);
    defer name_mapping.deinit();
    try name_mapping.ensureTotalCapacity(8);

    while (tokens.next()) |city_a| {
        _ = tokens.next() orelse unreachable;
        const city_b = tokens.next() orelse unreachable;
        _ = tokens.next() orelse unreachable;
        _ = tokens.next() orelse unreachable;

        inline for (.{ city_a, city_b }) |city| {
            const index = name_mapping.count();
            _ = try name_mapping.getOrPutValue(city, @intCast(index));
        }
    }

    tokens.reset();
    const city_count = name_mapping.count();
    std.debug.assert(city_count < 16);
    const as_shift: u4 = @intCast(city_count);
    const two_pow = @as(u16, 1) << as_shift;
    var distances = try alloc.alloc(u32, city_count * city_count);
    defer alloc.free(distances);
    @memset(distances, 0);

    while (tokens.next()) |city_a| {
        _ = tokens.next() orelse unreachable;
        const city_b = tokens.next() orelse unreachable;
        _ = tokens.next() orelse unreachable;
        const distance = try std.fmt.parseInt(u32, tokens.next() orelse unreachable, 10);
        const index_a = name_mapping.get(city_a) orelse unreachable;
        const index_b = name_mapping.get(city_b) orelse unreachable;
        distances[city_count * index_a + index_b] = distance;
        distances[city_count * index_b + index_a] = distance;
    }

    var shortest_table = try alloc.alloc(u32, city_count * two_pow);
    defer alloc.free(shortest_table);
    var longest_table = try alloc.alloc(u32, city_count * two_pow);
    defer alloc.free(longest_table);
    @memset(shortest_table, 0);
    @memset(longest_table, 0);

    for (3..two_pow) |set_mask| {
        if (std.math.isPowerOfTwo(set_mask)) continue;
        const longest_row = longest_table[set_mask * city_count ..][0..city_count];
        const shortest_row = shortest_table[set_mask * city_count ..][0..city_count];

        const Bitset = std.bit_set.IntegerBitSet(16);
        const bitset = Bitset{ .mask = @intCast(set_mask) };
        var bit_iter = bitset.iterator(.{ .kind = .set });
        while (bit_iter.next()) |e_index| {
            const distance_row = distances[e_index * city_count ..][0..city_count];
            var subset = bitset;
            subset.toggle(e_index);
            var shortest: u32 = std.math.maxInt(u32);
            var longest: u32 = 0;
            var subset_iter = subset.iterator(.{ .kind = .set });
            while (subset_iter.next()) |i| {
                const distance = distance_row[i];
                const dp_index = subset.mask * city_count + i;
                shortest = @min(shortest, shortest_table[dp_index] + distance);
                longest = @max(longest, longest_table[dp_index] + distance);
            }
            longest_row[e_index] = longest;
            shortest_row[e_index] = shortest;
        }
    }

    const final_row_start = city_count * (two_pow - 1);
    const shortest_distance = std.mem.min(u32, shortest_table[final_row_start..][0..city_count]);
    const longest_distance = std.mem.max(u32, longest_table[final_row_start..][0..city_count]);
    return .{ shortest_distance, longest_distance };
}

fn part1(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    _ = io;
    return std.fmt.allocPrint(alloc, "{d}", .{parsed[0]});
}

fn part2(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    _ = io;
    return std.fmt.allocPrint(alloc, "{d}", .{parsed[1]});
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
        .{
            .input = "London to Dublin = 464\nLondon to Belfast = 518\nDublin to Belfast = 141",
            .part1_solution = "605",
            .part2_solution = "982",
        },
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
