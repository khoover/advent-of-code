const std = @import("std");
const Io = std.Io;
const log = std.log;

const cli = @import("cli");
const Client = @import("Client.zig");
const spec = @import("solutions/spec.zig");
const ALL_YEARS = @import("solutions.zig").ALL_YEARS;

var args = struct {
    input_path: []const u8 = undefined,
    year: ?[]const u8 = null,
    day: ?[]const u8 = null,
    fetch: bool = false,
    token: ?[]const u8 = null,
    io: Io = undefined,
    gpa: std.mem.Allocator = undefined,
}{};

pub fn main(init: std.process.Init) !void {
    args.io = init.io;
    args.gpa = init.gpa;
    var runner = cli.AppRunner.init(&init);
    defer runner.deinit();

    const app: cli.App = .{
        .command = .{
            .name = "aoc-run",
            .description = .{ .one_line = "Run Advent of Code solutions" },
            .target = .{ .action = .{
                .exec = run,
                .positional_args = .{
                    .required = &.{
                        .{
                            .name = "INPUT_DIR",
                            .help = "Path to directory containing problem inputs; expected to be structured as <path>/YYYY/D.input",
                            .value_ref = runner.mkRef(&args.input_path),
                        },
                    },
                    .optional = &.{
                        .{
                            .name = "YEAR",
                            .help = "Year to run on, given as YYYY",
                            .value_ref = runner.mkRef(&args.year),
                        },
                        .{
                            .name = "DAY",
                            .help = "Day to run on, given as D or DD",
                            .value_ref = runner.mkRef(&args.day),
                        },
                    },
                },
            } },
            .options = &.{ .{
                .long_name = "fetch",
                .help = "Whether to fetch problem data before running",
                .value_ref = runner.mkRef(&args.fetch),
            }, .{
                .long_name = "token",
                .help = "Token to use when authenticating to the AOC server",
                .envvar = "AOC_TOKEN",
                .value_name = "SESSION_TOKEN",
                .value_ref = runner.mkRef(&args.token),
            } },
        },
    };
    return runner.run(&app);
}

const RunError = error{
    MissingToken,
    InvalidYear,
    InvalidDay,
};

fn run() !void {
    if (args.token == null and args.fetch) {
        log.err("Cannot specify --fetch without passing a token", .{});
        return RunError.MissingToken;
    }
    const year: ?spec.Year = year: {
        var b: spec.Year = undefined;
        if (args.year) |input_year| {
            if (input_year.len != 4) {
                log.err("Invalid year, must be the 4-digit year format", .{});
                return RunError.InvalidYear;
            }
            @memcpy(&b, input_year);
            break :year b;
        } else {
            break :year null;
        }
    };
    const day: ?spec.Day = day: {
        var b: spec.Day = undefined;
        if (args.day) |input_day| {
            if (input_day.len == 1) {
                b[0] = '0';
                b[1] = input_day[0];
            } else if (input_day.len == 2) {
                b[0] = input_day[0];
                b[1] = input_day[1];
            } else {
                log.err("Invalid year, must be a 1- or 2-digit format", .{});
                return RunError.InvalidDay;
            }
            break :day b;
        } else {
            break :day null;
        }
    };

    std.debug.print(
        "Args are input_path = {s}, year = {?s}, day = {?s}, fetch = {}, token = {?s}\n",
        .{ args.input_path, year, day, args.fetch, args.token },
    );
    var input_dir = try Io.Dir.cwd().openDir(
        args.io,
        args.input_path,
        .{ .access_sub_paths = true },
    );
    defer input_dir.close(args.io);

    var client: ?Client = if (args.fetch) try Client.init(args.token orelse unreachable, args.io, args.gpa) else null;
    defer if (client) |*c| {
        c.deinit(args.gpa);
    };
    var arena = std.heap.ArenaAllocator.init(args.gpa);
    defer arena.deinit();
    // Give it 512MiB to start
    _ = try arena.allocator().alloc(u8, 1 << 29);
    _ = arena.reset(.retain_capacity);
    try preheatIo(args.io);

    for (&ALL_YEARS) |*year_spec| {
        if (!std.mem.eql(u8, &year_spec.year, &(year orelse year_spec.year))) {
            continue;
        }
        const results = try spec.runYear(
            year_spec,
            &input_dir,
            day,
            if (client) |*c| c else null,
            args.fetch,
            &arena,
            args.gpa,
            args.io,
        );
        defer {
            for (results) |*labeled_result| {
                const r = labeled_result.result catch {
                    continue;
                };
                r.deinit(args.gpa);
            }
            args.gpa.free(results);
        }
        for (results) |*labeled_result| {
            log.info("Year {s} Day {s}\n{!f}", .{ labeled_result.year, labeled_result.day, labeled_result.result });
        }
    }

    return;
}

fn preheatIo(io: Io) !void {
    const core_count = try std.Thread.getCpuCount();
    var sem = Io.Semaphore{};
    var group = Io.Group.init;
    for (0..core_count) |_| {
        group.concurrent(io, preheatWait, .{ &sem, io }) catch break;
    }
    for (0..core_count) |_| {
        sem.post(io);
    }
    return group.await(io);
}

fn preheatWait(sem: *Io.Semaphore, io: Io) Io.Cancelable!void {
    return sem.wait(io);
}

test "root" {
    std.testing.refAllDecls(@This());
}
