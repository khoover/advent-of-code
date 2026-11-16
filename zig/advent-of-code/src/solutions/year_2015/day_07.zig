const spec = @import("../spec.zig");
const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;

inline fn indexOf(ident: []const u8) u16 {
    std.debug.assert(ident.len == 1 or ident.len == 2);
    std.debug.assert('a' <= ident[0] and ident[0] <= 'z');
    std.debug.assert(ident.len == 1 or ('a' <= ident[1] and ident[1] <= 'z'));

    if (ident.len == 1) {
        return ident[0] - 'a';
    } else {
        return @as(u16, (ident[0] - 'a' + 1)) * 27 + (ident[1] - 'a');
    }
}

const CIRCUIT_LEN = indexOf("zz") + 1;

const Op = enum {
    unknown,
    constant,
    ident,
    not,
    parity,
    bitand,
    bitor,
    lshift,
    rshift,
};

const Instruction = union(Op) {
    unknown: void,
    constant: u16,
    ident: u16,
    not: u16,
    parity: u16,
    bitand: struct { u16, u16 },
    bitor: struct { u16, u16 },
    lshift: struct { u16, u4 },
    rshift: struct { u16, u4 },
};

const ParseOutput = struct { u16, u16 };

fn parse(input: []const u8, alloc: Allocator, io: Io) anyerror!ParseOutput {
    _ = io;
    _ = alloc;

    var iter = std.mem.tokenizeAny(u8, input, &std.ascii.whitespace);
    var parsed: [CIRCUIT_LEN]Instruction = @splat(.unknown);

    while (iter.next()) |first_token| {
        const second_token = iter.next() orelse unreachable;
        const instruction: Instruction = blk: {
            if (std.mem.eql(u8, first_token, "NOT")) {
                _ = iter.next() orelse unreachable;
                break :blk .{ .not = indexOf(second_token) };
            } else if (std.mem.eql(u8, second_token, "->")) {
                if (std.ascii.isDigit(first_token[0])) {
                    break :blk .{ .constant = std.fmt.parseInt(u16, first_token, 10) catch unreachable };
                } else {
                    break :blk .{ .ident = indexOf(first_token) };
                }
            } else {
                const third_token = iter.next() orelse unreachable;
                _ = iter.next();
                break :blk switch (second_token.len) {
                    2 => .{ .bitor = .{ indexOf(first_token), indexOf(third_token) } },
                    3 => if (first_token[0] == '1') .{
                        .parity = indexOf(third_token),
                    } else .{
                        .bitand = .{ indexOf(first_token), indexOf(third_token) },
                    },
                    6 => shift: {
                        const index = indexOf(first_token);
                        const shift = std.fmt.parseInt(u4, third_token, 10) catch unreachable;
                        break :shift if (second_token[0] == 'L') .{
                            .lshift = .{ index, shift },
                        } else .{
                            .rshift = .{ index, shift },
                        };
                    },
                    else => unreachable,
                };
            }
        };

        const output_wire = iter.next() orelse unreachable;
        parsed[indexOf(output_wire)] = instruction;
    }

    var cache: [CIRCUIT_LEN]?u16 = @splat(null);
    const index_a = indexOf("a");
    const part1_res = evalGate(&parsed, &cache, index_a);
    @memset(&cache, null);
    cache[indexOf("b")] = part1_res;
    const part2_res = evalGate(&parsed, &cache, index_a);
    return .{ part1_res, part2_res };
}

fn part1(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    _ = io;
    return std.fmt.allocPrint(alloc, "{d}", .{parsed[0]});
}

fn evalGate(instructions: *const [CIRCUIT_LEN]Instruction, cache: *[CIRCUIT_LEN]?u16, index: u16) u16 {
    if (cache[index]) |cached| {
        return cached;
    }

    const res: u16 = switch (instructions[index]) {
        .constant => |c| c,
        .ident => |i| evalGate(instructions, cache, i),
        .not => |i| ~evalGate(instructions, cache, i),
        .parity => |i| 1 & evalGate(instructions, cache, i),
        .bitand => |pair| evalGate(instructions, cache, pair[0]) & evalGate(instructions, cache, pair[1]),
        .bitor => |pair| evalGate(instructions, cache, pair[0]) | evalGate(instructions, cache, pair[1]),
        .lshift => |pair| evalGate(instructions, cache, pair[0]) << pair[1],
        .rshift => |pair| evalGate(instructions, cache, pair[0]) >> pair[1],
        .unknown => unreachable,
    };
    cache[index] = res;
    return res;
}

fn part2(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    _ = io;
    return std.fmt.allocPrint(alloc, "{d}", .{parsed[1]});
}

pub const run = spec.makeDayRun(
    ParseOutput,
    parse,
    part1,
    part2,
    null,
);

const TestCase = struct {
    input: []const u8,
    part1_solution: ?[]const u8 = null,
    part2_solution: ?[]const u8 = null,
};

test "run test cases" {
    const cases = [_]TestCase{
        .{ .input = "123 -> x\n456 -> a", .part1_solution = "456" },
        .{ .input = "123 -> x\n456 -> y\nx AND y -> a", .part1_solution = "72" },
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
