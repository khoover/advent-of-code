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

    pub fn dupe(self: *const PartResult, a: Allocator) !PartResult {
        return .{
            .wall = self.wall,
            .cpu = self.cpu,
            .solution = try a.dupe(u8, self.solution),
        };
    }
};

pub const Day = [2]u8;
pub const Year = [4]u8;

pub const Result = struct {
    parse_wall: Io.Duration,
    parse_cpu: Io.Duration,
    part1: PartResult,
    part2: ?PartResult,

    pub const EMPTY: @This() = .{
        .parse_wall = .fromNanoseconds(0),
        .parse_cpu = .fromNanoseconds(0),
        .part1 = .{
            .solution = "",
            .wall = .fromNanoseconds(0),
            .cpu = .fromNanoseconds(0),
        },
        .part2 = .{
            .solution = "",
            .wall = .fromNanoseconds(0),
            .cpu = .fromNanoseconds(0),
        },
    };

    pub fn deinit(self: *const Result, alloc: Allocator) void {
        alloc.free(self.part1.solution);
        if (self.part2) |part2| {
            alloc.free(part2.solution);
        }
    }

    pub fn dupe(self: *const Result, a: Allocator) !Result {
        const part1 = try self.part1.dupe(a);
        errdefer a.free(part1.solution);
        return .{
            .parse_wall = self.parse_wall,
            .parse_cpu = self.parse_cpu,
            .part1 = part1,
            .part2 = if (self.part2) |p| try p.dupe(a) else null,
        };
    }

    pub fn addDurations(self: *Result, other: *const Result) void {
        self.parse_wall.nanoseconds += other.parse_wall.nanoseconds;
        self.parse_cpu.nanoseconds += other.parse_cpu.nanoseconds;
        self.part1.wall.nanoseconds += other.part1.wall.nanoseconds;
        self.part1.cpu.nanoseconds += other.part1.wall.nanoseconds;
        if (self.part2) |*part2| {
            if (other.part2) |*other_part2| {
                part2.cpu.nanoseconds += other_part2.cpu.nanoseconds;
                part2.wall.nanoseconds += other_part2.wall.nanoseconds;
            }
        } else if (other.part2) |*other_part2| {
            self.part2 = .{
                .solution = "",
                .cpu = other_part2.cpu,
                .wall = other_part2.wall,
            };
        }
    }

    pub fn format(result: *const Result, w: *Io.Writer) Io.Writer.Error!void {
        try w.print(
            "Parse walltime: {f}, CPU time: {f}\n",
            .{ result.parse_wall, result.parse_cpu },
        );
        try w.writeAll("Part 1 ");
        if (result.part1.solution.len > 0) {
            try w.print("solution: {s}, ", .{result.part1.solution});
        }
        try w.print(
            "walltime: {f}, CPU time: {f}\n",
            .{ result.part1.wall, result.part1.cpu },
        );
        try w.writeAll("Part 2 ");
        if (result.part2) |*part2| {
            if (part2.solution.len > 0) {
                try w.print("solution: {s}, ", .{part2.solution});
            }
            try w.print(
                "walltime: {f}, CPU time: {f}\nOverall walltime: {f}, CPU time: {f}",
                .{
                    part2.wall,
                    part2.cpu,
                    Io.Duration{ .nanoseconds = result.parse_wall.nanoseconds + result.part1.wall.nanoseconds + part2.wall.nanoseconds },
                    Io.Duration{ .nanoseconds = result.parse_cpu.nanoseconds + result.part1.cpu.nanoseconds + part2.cpu.nanoseconds },
                },
            );
        } else {
            try w.writeAll("unsolved");
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

pub const DaySpec = struct {
    day: Day,
    run: *const DayRunFn,
};

pub const YearSpec = struct {
    year: Year,
    days: []const DaySpec,
};

pub const LabelledResult = struct {
    year: Year,
    day: Day,
    result: anyerror!Result,
};

pub const YearError = error{
    DayNotImplemented,
    MissingClient,
    InputNotFound,
};

/// The arena allocator is assumed to be free to reset in between days, i.e. it
/// should be in a fresh state when passed in. The returned results should be allocated
/// from the gpa.
pub fn runYear(
    year_spec: *const YearSpec,
    inputs_dir: *Io.Dir,
    specific_day: ?Day,
    client: ?*Client,
    fetch_if_needed: bool,
    arena: *std.heap.ArenaAllocator,
    gpa: Allocator,
    io: Io,
) ![]LabelledResult {
    if (fetch_if_needed and client == null) {
        return YearError.MissingClient;
    }
    // Keep 1GiB as the limit.
    _ = arena.reset(.{ .retain_with_limit = 1 << 30 });
    const arena_allocator = arena.allocator();
    var year_inputs_dir = inputs_dir.openDir(
        io,
        &year_spec.year,
        .{ .access_sub_paths = true, .iterate = false },
    ) catch |err| switch (err) {
        Io.Dir.OpenError.FileNotFound => try inputs_dir.createDirPathOpen(io, &year_spec.year, .{}),
        else => {
            return err;
        },
    };
    defer year_inputs_dir.close(io);

    var results = try std.ArrayList(LabelledResult).initCapacity(gpa, if (specific_day == null) 1 else year_spec.days.len);
    errdefer {
        for (results.items) |*labeled_result| {
            const r = labeled_result.result catch {
                continue;
            };
            r.deinit(gpa);
        }
        results.deinit(gpa);
    }

    for (year_spec.days) |*day| {
        if (!std.mem.eql(u8, &day.day, &(specific_day orelse day.day))) {
            continue;
        }
        defer _ = arena.reset(.{ .retain_with_limit = 1 << 30 });

        const input_data = try getInputData(
            client,
            fetch_if_needed,
            year_inputs_dir,
            year_spec.year,
            day.day,
            io,
            arena_allocator,
        );
        const result = day.run(input_data, arena_allocator, io);
        results.appendAssumeCapacity(.{
            .day = day.day,
            .year = year_spec.year,
            .result = if (result) |r| r.dupe(gpa) else |e| e,
        });
    }

    if (specific_day != null and results.items.len == 0) {
        return YearError.DayNotImplemented;
    }

    return results.toOwnedSlice(gpa);
}

fn getInputData(
    client: ?*Client,
    fetch_if_needed: bool,
    year_dir: Io.Dir,
    year: Year,
    day: Day,
    io: Io,
    alloc: Allocator,
) ![]const u8 {
    std.debug.assert(!fetch_if_needed or client != null);

    const sub_path: [8]u8 = subpath: {
        var template: [8]u8 = "  .input".*;
        template[0] = day[0];
        template[1] = day[1];
        break :subpath template;
    };

    const input_file = year_dir.openFile(
        io,
        &sub_path,
        .{ .mode = .read_only, .allow_directory = false },
    ) catch |err| switch (err) {
        Io.File.OpenError.FileNotFound => blk: {
            if (!fetch_if_needed) {
                return YearError.InputNotFound;
            }
            break :blk try fetchInputData(client orelse unreachable, year_dir, year, day, &sub_path, io);
        },
        else => {
            return err;
        },
    };
    defer input_file.close(io);

    var buf: [1024]u8 = undefined;
    var reader = input_file.reader(io, &buf);
    return reader.interface.allocRemaining(alloc, .limited(1 << 20));
}

fn fetchInputData(
    client: *Client,
    year_dir: Io.Dir,
    year: Year,
    day: Day,
    sub_path: []const u8,
    io: Io,
) !Io.File {
    const file = try year_dir.createFile(
        io,
        sub_path,
        .{ .read = true, .lock = .exclusive },
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
            log.err("Unable to truncate input file {s} for year {s}, error {t}", .{ sub_path, year, err });
        };
        file.unlock(io);
        file.close(io);
        year_dir.deleteFile(io, sub_path) catch {
            log.err(
                "Unable to delete potentially broken input file {s} for year {s}, user should manually delete/fix",
                .{ sub_path, year },
            );
        };
    }

    var buf: [1024]u8 = undefined;
    var writer = file.writerStreaming(io, &buf);

    try client.fetchInput(year, day, &writer.interface);
    try writer.end();
    try file.sync(io);
    file.unlock(io);
    return file;
}
