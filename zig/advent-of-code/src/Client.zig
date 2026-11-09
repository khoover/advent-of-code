const std = @import("std");
const Io = std.Io;
const log = std.log;
const Allocator = std.mem.Allocator;

const git_sha = @import("build_config").git_sha;

const spec = @import("solutions/spec.zig");

const Client = @This();
cookie_header: []const u8,
client: std.http.Client,
io: Io,
last_request_at: std.atomic.Value(i64) = .init(0),

const AOC_URL = "https://adventofcode.com/{d}/day/{d}";
const USER_AGENT = std.fmt.comptimePrint("github.com/khoover/advent-of-code/tree/main/zig/advent-of-code@{s}", .{git_sha});

pub const FetchError = error{
    InvalidToken,
    ClientError,
    ServerError,
};

pub fn init(token: []const u8, io: Io, alloc: Allocator) !Client {
    const cookie_header = try std.fmt.allocPrint(alloc, "session={s}", .{token});
    return .{
        .cookie_header = cookie_header,
        .client = .{ .io = io, .allocator = alloc },
        .io = io,
    };
}

pub fn deinit(client: *Client, alloc: Allocator) void {
    alloc.free(client.cookie_header);
    client.client.deinit();
}

fn waitForCooldown(client: *Client) Io.Cancelable!void {
    const CLOCK = Io.Clock.real;
    const INTER_REQUEST_COOLDOWN: Io.Clock.Duration = .{ .raw = .fromSeconds(1), .clock = CLOCK };

    var last_at_seconds = client.last_request_at.load(.monotonic);
    var last_at: Io.Clock.Timestamp = .{ .raw = undefined, .clock = .real };
    while (true) {
        last_at.raw = .fromNanoseconds(@as(i96, last_at_seconds) * std.time.ns_per_s);
        const next_at = last_at.addDuration(INTER_REQUEST_COOLDOWN);
        var now = Io.Clock.Timestamp.now(client.io, CLOCK);
        if (now.compare(.lt, next_at)) {
            @branchHint(.unlikely);
            log.info("Rate-limiting requests to the server, waiting for {f}", .{now.durationTo(next_at).raw});
            try next_at.wait(client.io);
            now = .now(client.io, CLOCK);
        }
        if (client.last_request_at.cmpxchgWeak(
            last_at_seconds,
            now.raw.toSeconds(),
            .monotonic,
            .monotonic,
        )) |x| {
            last_at_seconds = x;
        } else {
            return;
        }
    }
}

/// Fetches the input for the given year and day, writing it as-is into the given writer.
pub fn fetchInput(
    client: *Client,
    year: spec.Year,
    day: spec.Day,
    writer: *Io.Writer,
    alloc: Allocator,
) !void {
    const query_url = try std.fmt.allocPrint(alloc, AOC_URL ++ "/input", .{ year, day });
    defer alloc.free(query_url);

    try client.waitForCooldown();

    const result = try client.client.fetch(.{
        .redirect_behavior = .not_allowed,
        .response_writer = writer,
        .method = .GET,
        .location = .{ .url = query_url },
        .headers = .{ .user_agent = .{ .override = USER_AGENT } },
        .extra_headers = &.{
            .{ .name = "Cookie", .value = client.cookie_header },
        },
    });

    switch (@intFromEnum(result.status)) {
        400...499 => |code| {
            if (code == 401 or code == 403) {
                log.err("Failed to get data, invalid token passed to Client", .{});
                return FetchError.InvalidToken;
            } else {
                log.err("Unexpected client error: {d}", .{code});
                return FetchError.ClientError;
            }
        },
        500...599 => |code| {
            log.err("Unexpected server error: {d}", .{code});
            return FetchError.ServerError;
        },
        200...299 => {
            @branchHint(.likely);
            return;
        },
        else => |code| {
            log.warn("Unexpected HTTP status received: {d}", .{code});
        },
    }
}
