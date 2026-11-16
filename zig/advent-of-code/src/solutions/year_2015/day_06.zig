const spec = @import("../spec.zig");
const std = @import("std");
const libs = @import("libs");
const Io = std.Io;
const Allocator = std.mem.Allocator;

const Command = enum {
    on,
    off,
    toggle,
};

const Instruction = struct {
    const Self = @This();

    command: Command,
    x1: u10,
    y1: u10,
    x2: u10,
    y2: u10,

    pub fn fromLine(line: []const u8) Self {
        const command: Command = switch (line[6]) {
            'f' => .off,
            'n' => .on,
            ' ' => .toggle,
            else => unreachable,
        };
        var iter = libs.iter.Unsigned(10){ .buf = line, .cur_index = 7 };
        return .{
            .command = command,
            .x1 = iter.next() orelse unreachable,
            .y1 = iter.next() orelse unreachable,
            .x2 = iter.next() orelse unreachable,
            .y2 = iter.next() orelse unreachable,
        };
    }
};

const ParseOutput = std.ArrayList(Instruction);
const ParIter = libs.parallel.WorkStealingIterator;

fn parse(input: []const u8, alloc: Allocator, io: Io) anyerror!ParseOutput {
    _ = io;
    var res = ParseOutput.empty;
    errdefer res.deinit(alloc);

    var iter = std.mem.splitScalar(u8, input, '\n');
    while (iter.next()) |line| {
        try res.append(alloc, .fromLine(line));
    }
    return res;
}

fn part1(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    var futures: [64]Io.Future(u32) = undefined;
    const threads: u32 = @intCast(@min(64, try std.Thread.getCpuCount()));
    var started: usize = 0;
    errdefer for (0..started) |i| {
        _ = futures[i].cancel(io);
    };

    var children: [64]libs.parallel.State = undefined;
    var iter = try ParIter.init(0, 1000, children[0..threads]);
    for (0..threads) |i| {
        futures[i] = try Io.concurrent(io, iterRowsPart1, .{ iter.child(i), parsed.items });
        started = i;
    }

    var acc: u32 = 0;
    for (futures[0..threads]) |*fut| {
        acc += fut.await(io);
    }

    return std.fmt.allocPrint(alloc, "{d}", .{acc});
}

fn iterRowsPart1(child: ParIter.Child, instructions: []const Instruction) u32 {
    var iter = child;
    var total: u32 = 0;
    while (iter.next()) |row| {
        const y: u10 = @intCast(row);
        var grid: [1024]bool = @splat(false);
        for (instructions) |instruction| {
            if (y < instruction.y1 or y > instruction.y2) continue;
            switch (instruction.command) {
                .on, .off => {
                    const val = instruction.command == .on;
                    @memset(grid[instruction.x1 .. instruction.x2 + 1], val);
                },
                .toggle => {
                    for (instruction.x1..instruction.x2 + 1) |i| {
                        grid[i] = !grid[i];
                    }
                },
            }
        }
        for (grid[0..1000]) |b| {
            total += @intFromBool(b);
        }
    }
    return total;
}

fn part2(parsed: *const ParseOutput, alloc: Allocator, io: Io) anyerror![]const u8 {
    var futures: [64]Io.Future(u32) = undefined;
    const threads: u32 = @intCast(@min(64, try std.Thread.getCpuCount()));
    var started: usize = 0;
    errdefer for (0..started) |i| {
        _ = futures[i].cancel(io);
    };

    var children: [64]libs.parallel.State = undefined;
    var iter = try ParIter.init(0, 1000, children[0..threads]);
    for (0..threads) |i| {
        futures[i] = try Io.concurrent(io, iterRowsPart2, .{ iter.child(i), parsed.items });
        started = i;
    }

    var acc: u32 = 0;
    for (futures[0..threads]) |*fut| {
        acc += fut.await(io);
    }

    return std.fmt.allocPrint(alloc, "{d}", .{acc});
}

fn iterRowsPart2(child: ParIter.Child, instructions: []const Instruction) u32 {
    var iter = child;
    var total: u32 = 0;
    while (iter.next()) |row| {
        const y: u10 = @intCast(row);
        var grid: [1024]u32 = @splat(0);
        for (instructions) |instruction| {
            if (y < instruction.y1 or y > instruction.y2) continue;
            switch (instruction.command) {
                .on, .toggle => {
                    const inc: u32 = if (instruction.command == .on) 1 else 2;
                    for (instruction.x1..instruction.x2 + 1) |i| {
                        grid[i] += inc;
                    }
                },
                .off => {
                    for (instruction.x1..instruction.x2 + 1) |i| {
                        grid[i] -|= 1;
                    }
                },
            }
        }
        for (grid[0..1000]) |x| {
            total += x;
        }
    }
    return total;
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
        .{ .input = "turn on 0,0 through 999,999", .part1_solution = "1000000", .part2_solution = "1000000" },
        .{ .input = "toggle 0,0 through 999,0", .part1_solution = "1000", .part2_solution = "2000" },
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
