//! Shared types and functions used for specifying an Advent of Code's solutions for a day or year.
const std = @import("std");
const Io = std.Io;
const Allocator = std.mem.Allocator;
const log = std.log;

const Client = @import("../Client.zig");

pub const PartResult = struct {
    solution: []const u8,
    wall: Io.Duration,
    cpu: Io.Duration,
};

pub const Day = u5;
pub const Year = u12;

pub const Result = struct {
    parse_wall: Io.Duration,
    parse_cpu: Io.Duration,
    part1: PartResult,
    part2: ?PartResult,

    pub fn deinit(self: *Result, alloc: Allocator) void {
        alloc.free(self.part1.solution);
        if (self.part2) |part2| {
            alloc.free(part2.solution);
        }
    }
};

pub const DayRunFn = fn ([]const u8, Allocator, Io) anyerror!Result;

/// Create a standardized run function that evaluates a given day in Advent of Code.
/// Accepts the input data, an allocator, and an Io instance.
/// Part1/2 functions expected to allocate the space for their result strings from the
/// passed-in allocator (i.e. no returning static values).
pub fn makeDayRun(
    comptime T: type,
    comptime parse: fn ([]const u8, Allocator, Io) anyerror!T,
    comptime part1: fn (*const T, Allocator, Io) anyerror![]const u8,
    comptime part2: ?fn (*const T, Allocator, Io) anyerror![]const u8,
    comptime deinit: ?fn (*T, Allocator, Io) void,
) DayRunFn {
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

pub const YearOptions = struct {
    year_input_dir: Io.Dir,
    day: ?Day,
    client: ?*Client,
    fetch_if_needed: bool,
};

pub const YearError = error{
    DayNotImplemented,
    MissingClient,
    InputNotFound,
};

pub const YearRunFn = fn (*const YearOptions, Allocator, Io) anyerror![]Result;

pub fn makeYearRun(
    comptime year: Year,
    comptime days: []const @Tuple(.{ Day, type }),
) YearRunFn {
    var sorted_days = days;
    std.mem.sortUnstable(@Tuple(.{ Day, type }), &sorted_days, {}, struct {
        fn compare(context: void, a: @Tuple(.{ Day, type }), b: @Tuple(.{ Day, type })) bool {
            _ = context;
            return a[0] < b[0];
        }
    }.compare);
    const run_fns = comptime blk: {
        var run_fns: [sorted_days.len]@Tuple(.{ Day, []const u8, DayRunFn }) = undefined;
        for (sorted_days, 0..) |day, i| {
            run_fns[i][0] = day[0];
            run_fns[i][1] = std.fmt.comptimePrint("{d:0>2}.input", .{day[0]});
            run_fns[i][2] = day[1].run;
            if (i + 1 < sorted_days.len and day[0] == sorted_days[i + 1][0]) {
                @compileError("Cannot provide multiple days with the same date");
            }
        }
        break :blk run_fns;
    };

    return struct {
        fn run(opts: *const YearOptions, alloc: Allocator, io: Io) anyerror![]@Tuple(.{ Day, Result }) {
            var results: std.ArrayList(@Tuple(.{ Day, Result })) = .initCapacity(alloc, if (opts.day == null) run_fns.len else 1);
            errdefer {
                for (results) |result_pair| {
                    result_pair[1].deinit(alloc);
                }
                results.deinit(alloc);
            }

            for (run_fns) |day| {
                if (opts.day != null and opts.day != day[0]) {
                    continue;
                }
                const sub_path = day[1];
                const input_data = try getInputData(
                    opts.client,
                    opts.fetch_if_needed,
                    opts.year_input_dir,
                    year,
                    opts.day,
                    sub_path,
                    io,
                    alloc,
                );
                const run_fn: DayRunFn = day[2];
                results.appendAssumeCapacity(.{
                    opts.day,
                    run_fn(input_data, alloc, io) catch |err| {
                        log.err("Year {d} day {d} failed to run with error: {t}", .{ year, opts.day, err });
                        continue;
                    },
                });
            }

            return results.toOwnedSlice(alloc);
        }
    }.run;
}

fn getInputData(
    client: ?*Client,
    fetch_if_needed: bool,
    year_dir: Io.Dir,
    year: Year,
    day: Day,
    sub_path: []const u8,
    io: Io,
    alloc: Allocator,
) ![]const u8 {
    std.debug.assert(!fetch_if_needed or client != null);

    const input_file = year_dir.openFile(
        io,
        sub_path,
        .{ .mode = .read_only, .allow_directory = false },
    ) catch |err| switch (err) {
        Io.File.OpenError.FileNotFound => blk: {
            if (!fetch_if_needed) {
                return err;
            }
            break :blk try fetchInputData(client orelse unreachable, year_dir, year, day, sub_path, io, alloc);
        },
        else => {
            return err;
        },
    };
    defer input_file.close(io);

    var buf: [1024]u8 = undefined;
    const reader = input_file.reader(io, &buf);
    return reader.interface.allocRemaining(alloc, .limited(1 << 20));
}

fn fetchInputData(
    client: *Client,
    year_dir: Io.Dir,
    year: Year,
    day: Day,
    sub_path: []const u8,
    io: Io,
    alloc: Allocator,
) !Io.File {
    const file = try year_dir.createFile(
        io,
        sub_path,
        .{ .read = true, .lock = true },
    );
    if (file.length(io) catch |err| {
        file.close(io);
        return err;
    } != 0) {
        // Going to assume a non-empty file means something else wrote a solution in.
        return file;
    }
    errdefer {
        file.setLength(io, 0) catch |err| {
            log.err("Unable to truncate input file {s} for year {d}, error {t}", .{ sub_path, year, err });
        };
        file.unlock(io);
        file.close(io);
        year_dir.deleteFile(io, sub_path) catch {
            log.err(
                "Unable to delete potentially broken input file {s} for year {d}, user should manually delete/fix",
                .{ sub_path, year },
            );
        };
    }

    var buf: [128]u8 = undefined;
    const writer = file.writerStreaming(io, &buf);

    try client.fetchInput(year, day, &writer.interface, alloc);
    try writer.end();
    try file.sync(io);
    file.unlock(io);
    return file;
}
