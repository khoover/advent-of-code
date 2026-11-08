//! By convention, root.zig is the root source file when making a package.
const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;

/// This is a documentation comment to explain the `printAnotherMessage` function below.
///
/// Accepting an `Io.Writer` instance is a handy way to write reusable code.
pub fn printAnotherMessage(writer: *Io.Writer) Io.Writer.Error!void {
    try writer.print("Run `zig build test` to run the tests.\n", .{});
}

pub fn add(a: i32, b: i32) i32 {
    return a + b;
}

test "basic add functionality" {
    try std.testing.expect(add(3, 7) == 10);
}

pub const PartResult = struct {
    solution: []const u8,
    wall: Io.Duration,
    cpu: Io.Duration,
};

pub const Result = struct {
    parse_wall: Io.Duration,
    parse_cpu: Io.Duration,
    part1: PartResult,
    part2: ?PartResult,
};

/// Create a standardized run function that evaluates a given day in Advent of Code.
/// Accepts the inputs directory, an allocator, and an Io instance.
/// Part1/2 functions expected to allocate the space for their result strings from the
/// passed-in allocator (i.e. no returning static values).
pub fn makeDayRun(
    comptime T: type,
    comptime year: u16,
    comptime day: u8,
    comptime parse: fn ([]const u8, Allocator, Io) anyerror!T,
    comptime part1: fn (*const T, Allocator, Io) anyerror![]const u8,
    comptime part2: ?fn (*const T, Allocator, Io) anyerror![]const u8,
    comptime deinit: ?fn (*T, Allocator, Io) void,
) fn (Io.Dir, Allocator, Io) anyerror!Result {
    const input_sub_path = std.fmt.comptimePrint("{d}/{d:0>2}.input", .{ year, day });

    return struct {
        fn run(input_dir: Io.Dir, alloc: Allocator, io: Io) anyerror!Result {
            const input_bytes = try input_dir.readFileAlloc(io, input_sub_path, alloc, .unlimited);
            defer alloc.free(input_bytes);
            var result: Result = undefined;

            const wall_start = Io.Timestamp.now(io, .awake);
            const cpu_start = Io.Timestamp.now(io, .cpu_process);

            var parsed = try parse(input_bytes, alloc, io);
            defer if (deinit) |f| {
                f(&parsed, alloc, io);
            };
            const wall_parse_end = Io.Timestamp.now(io, .awake);
            const cpu_parse_end = Io.Timestamp.now(io, .cpu_process);

            result.part1.solution = try part1(&parsed, alloc, io);
            errdefer alloc.free(result.part1.solution);
            const wall_part1_end = Io.Timestamp.now(io, .awake);
            const cpu_part1_end = Io.Timestamp.now(io, .cpu_process);

            if (part2) |f| {
                var part2_res: PartResult = undefined;
                part2_res.solution = try f(&parsed, alloc, io);
                part2_res.wall = wall_part1_end.untilNow(io, .awake);
                part2_res.cpu = cpu_part1_end.untilNow(io, .cpu_process);
                result.part2 = part2_res;
            } else {
                result.part2 = null;
            }

            result.parse_wall = wall_start.durationTo(wall_parse_end);
            result.parse_cpu = cpu_start.durationTo(cpu_parse_end);
            result.part1.wall = wall_parse_end.durationTo(wall_part1_end);
            result.part1.cpu = cpu_parse_end.durationTo(cpu_part1_end);
            return result;
        }
    }.run;
}
