const std = @import("std");
const Io = std.Io;

const cli = @import("cli");
const Client = @import("Client.zig");

var args = struct {
    input_path: []const u8 = undefined,
    year: ?u16 = null,
    day: ?u8 = null,
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
                            .help = "Path to directory containing problem inputs; expected to be structured as <path>/YYYY/DD.input",
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

fn run() !void {
    std.debug.print(
        "Args are input_path = {s}, year = {?}, day = {?}, fetch = {}, token = {?s}\n",
        .{ args.input_path, args.year, args.day, args.fetch, args.token },
    );
    const input_dir = try Io.Dir.cwd().openDir(
        args.io,
        args.input_path,
        .{ .iterate = true, .access_sub_paths = false },
    );
    defer Io.Dir.close(input_dir, args.io);
    var walker = try input_dir.walkSelectively(args.gpa);
    defer walker.deinit();

    std.debug.print("Input dir contents are:\n", .{});
    while (try walker.next(args.io)) |entry| {
        std.debug.print("  - {s}\n", .{entry.basename});
    }

    const token = args.token orelse return;
    const year = args.year orelse return;
    const day = args.day orelse return;
    if (args.fetch) {
        var client = try Client.init(token, args.io, args.gpa);
        defer client.deinit(args.gpa);

        for (0..10) |_| {
            var writer = Io.Writer.Allocating.init(args.gpa);
            errdefer writer.deinit();

            try client.fetchInput(year, day, &writer.writer, args.gpa);

            const buf = try writer.toOwnedSlice();
            defer args.gpa.free(buf);

            std.debug.print("\nFetched input from the server, data is:\n<EOF>{s}<EOF>\n", .{buf});
        }
    }

    return;
}
