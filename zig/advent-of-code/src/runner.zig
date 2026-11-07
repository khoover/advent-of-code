const std = @import("std");
const cli = @import("cli");

var args = struct {
    input_path: []const u8 = undefined,
    year: ?u16 = null,
    day: ?u8 = null,
}{};

pub fn main(init: std.process.Init) !void {
    var runner = cli.AppRunner.init(&init);
    defer runner.deinit();

    const app: cli.App = .{
        .command = .{
            .name = "run",
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
        },
    };
    return runner.run(&app);
}

fn run() !void {
    std.debug.print(
        "Args are input_path = {s}, year = {?}, day = {?}\n",
        .{ args.input_path, args.year, args.day },
    );
    return;
}
