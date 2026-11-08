//! Shared types and functions used for specifying an Advent of Code's solutions for a day or year.
const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;

const git_sha = @import("build_config").git_sha;

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

const RunFn = fn ([]const u8, Allocator, Io) anyerror!Result;

/// Create a standardized run function that evaluates a given day in Advent of Code.
/// Accepts the input data, an allocator, and an Io instance.
/// Part1/2 functions expected to allocate the space for their result strings from the
/// passed-in allocator (i.e. no returning static values).
pub fn makeRun(
    comptime T: type,
    comptime parse: fn ([]const u8, Allocator, Io) anyerror!T,
    comptime part1: fn (*const T, Allocator, Io) anyerror![]const u8,
    comptime part2: ?fn (*const T, Allocator, Io) anyerror![]const u8,
    comptime deinit: ?fn (*T, Allocator, Io) void,
) RunFn {
    return struct {
        fn run(input_bytes: []const u8, alloc: Allocator, io: Io) anyerror!Result {
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

const AOC_URL = "https://adventofcode.com/{d}/day/{d}";
const USER_AGENT = std.fmt.comptimePrint("https://github.com/khoover/advent-of-code/tree/main/zig/advent-of-code@{s}", .{git_sha});

pub const FetchError = error{
    ClientError,
    ServerError,
};

pub fn fetchInput(
    year: u16,
    day: u8,
    input_dir: Io.Dir,
    token: []const u8,
    overwrite: bool,
    io: Io,
    alloc: Allocator,
) !void {
    const sub_path = try std.fmt.allocPrint(alloc, "{d}/{d:0>2}.input", .{ year, day });
    defer alloc.free(sub_path);
    const file = try input_dir.createFile(
        io,
        sub_path,
        .{ .truncate = overwrite, .exclusive = !overwrite },
    );
    defer file.close(io);
    var buf: [1024]u8 = undefined;
    const writer = file.writer(io, &buf);
    defer writer.end();

    const client = std.http.Client{ .allocator = alloc, .io = io };
    defer client.deinit();

    const query_url = try std.fmt.allocPrint(alloc, AOC_URL ++ "/input", .{ year, day });
    defer alloc.free(query_url);
    const cookie_header = try std.fmt.allocPrint(alloc, "session={s}", .{token});

    const result = try client.fetch(.{
        .redirect_behavior = .not_allowed,
        .response_writer = &writer.interface,
        .method = .GET,
        .location = .{ .url = query_url },
        .headers = .{ .user_agent = .{ .override = USER_AGENT } },
        .privileged_headers = &.{
            .{ .name = "Cookie", .value = cookie_header },
        },
    });
    switch (result.status.class()) {
        .client_error => {
            return FetchError.ClientError;
        },
        .server_error => {
            return FetchError.ServerError;
        },
        else => {
            return;
        },
    }
}
