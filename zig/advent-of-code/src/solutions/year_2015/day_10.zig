const spec = @import("../spec.zig");
const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;

const element_data: struct { code_to_index: std.StaticStringMap(u8), successor_indices: [92][]const u8 } = element: {
    @setEvalBranchQuota(1000000);
    const raw_data = std.mem.trim(u8, @embedFile("raw_elements.txt"), &std.ascii.whitespace);
    const count = 92;

    var code_kvs: [count]struct { []const u8, u8 } = undefined;
    var name_kvs: [count]struct { []const u8, u8 } = undefined;
    var raw_successors: [count][]const u8 = undefined;

    var iter = std.mem.splitScalar(u8, raw_data, ',');

    for (0..count) |i| {
        _ = iter.next() orelse unreachable;
        const name = iter.next() orelse unreachable;
        const code = iter.next() orelse unreachable;
        const successors = iter.next() orelse unreachable;

        var code_copy: [code.len]u8 = undefined;
        @memcpy(&code_copy, code);
        const owned_code = code_copy;

        const idx: u8 = @intCast(i);
        code_kvs[i] = .{ &owned_code, idx };
        name_kvs[i] = .{ name, idx };
        raw_successors[i] = successors;
    }

    const name_map = std.StaticStringMap(u8).initComptime(name_kvs);

    var successors_array: [count][]const u8 = undefined;
    for (0..count) |i| {
        const successor_str = raw_successors[i];

        var successor_iter = std.mem.splitScalar(u8, successor_str, '.');
        var successor_count = 0;
        while (successor_iter.next()) |_| successor_count += 1;

        var successor_indices: [successor_count]u8 = undefined;
        successor_iter.reset();
        var j = 0;
        while (successor_iter.next()) |successor_name| : (j += 1) {
            successor_indices[j] = name_map.get(successor_name) orelse unreachable;
        }

        const owned_successors = successor_indices;
        successors_array[i] = &owned_successors;
    }

    break :element .{
        .code_to_index = std.StaticStringMap(u8).initComptime(code_kvs),
        .successor_indices = successors_array,
    };
};

const ParseOutput = struct { usize, usize };

fn parse(input: []const u8, alloc: Allocator, io: Io) anyerror!ParseOutput {
    _ = alloc;
    _ = io;

    var current_elements: [92]u32 = @splat(0);
    const initial = element_data.code_to_index.get(std.mem.trim(u8, input, &std.ascii.whitespace)) orelse unreachable;
    current_elements[initial] = 1;

    for (0..40) |_| {
        current_elements = step(current_elements);
    }
    const part1_res = total_len(&current_elements);
    for (0..10) |_| {
        current_elements = step(current_elements);
    }
    const part2_res = total_len(&current_elements);

    return .{ part1_res, part2_res };
}

inline fn step(current_elements: [92]u32) [92]u32 {
    var next: [92]u32 = @splat(0);

    for (&current_elements, 0..) |count, i| {
        if (count > 0) {
            for (element_data.successor_indices[i]) |next_element| {
                next[next_element] += count;
            }
        }
    }

    return next;
}

inline fn total_len(element_counts: *const [92]u32) usize {
    const kvs = element_data.code_to_index.kvs;
    var total: usize = 0;
    for (0..kvs.len) |i| {
        const index = kvs.values[i];
        total += element_counts[index] * kvs.keys[i].len;
    }
    return total;
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
    const cases = [_]TestCase{};
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
