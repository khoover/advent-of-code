const std = @import("std");
const Io = std.Io;

pub fn SharedIterator(comptime T: type) type {
    return struct {
        const Self = @This();

        counter: std.atomic.Value(T),
        finished: std.atomic.Value(bool) = .init(false),
        increment: T,

        pub fn init(start: T, increment: T) Self {
            return .{ .counter = .init(start), .increment = increment };
        }

        pub fn next(self: *Self) ?T {
            if (self.finished.load(.acquire)) {
                @branchHint(.unlikely);
                return null;
            } else {
                return self.counter.fetchAdd(self.increment, .acq_rel);
            }
        }

        pub fn finish(self: *Self) bool {
            return self.finished.swap(true, .acq_rel);
        }
    };
}

const md5_k: [64]u32 = .{ 0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1, 0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391 };
fn Md5DataType(comptime N: usize) type {
    return struct {
        a: @Vector(N, u32),
        b: @Vector(N, u32),
        c: @Vector(N, u32),
        d: @Vector(N, u32),
    };
}

const ProblemState = struct { iter: SharedIterator(u32), solution_1: std.atomic.Value(u32), solution_2: std.atomic.Value(u32) };

fn worker(comptime N: usize, io: Io, state: *ProblemState, prefix: []const u8) Io.Cancelable!void {
    var message_buffers: [N][64]u8 = undefined;
    @memcpy(message_buffers[0][0..prefix.len], prefix);
    @memset(message_buffers[0][prefix.len..64], 0);
    while (state.iter.next()) |base| {
        check_chunk(N, &message_buffers, prefix.len, base, state);
        try io.checkCancel();
    }
}

fn check_chunk(comptime md5_chunk_size: usize, message_buffers: *[md5_chunk_size][64]u8, prefix_len: usize, base: u32, state: *ProblemState) void {
    std.debug.assert(base % 1000 == 0);
    std.debug.assert(prefix_len <= 32);
    const leftovers = 1000 % md5_chunk_size;

    const offset_start_idx = init_buffers(md5_chunk_size, message_buffers, prefix_len, base);

    var chunk_offset: u32 = 0;
    const check_mask_1: @Vector(md5_chunk_size, u32) = @splat(0x00F0FFFF);
    const check_mask_2: @Vector(md5_chunk_size, u32) = @splat(0x00FFFFFF);
    const zeros: @Vector(md5_chunk_size, u32) = @splat(0);
    while (chunk_offset + (md5_chunk_size - 1) < 1000) : (chunk_offset += md5_chunk_size) {
        write_offsets(md5_chunk_size, message_buffers, offset_start_idx, chunk_offset);
        const first_four_bytes = md5(md5_chunk_size, message_buffers).a;
        const mask_comp_1: @Int(.unsigned, md5_chunk_size) = @bitCast(@intFromBool(check_mask_1 & first_four_bytes == zeros));
        const mask_comp_2: @Int(.unsigned, md5_chunk_size) = @bitCast(@intFromBool(check_mask_2 & first_four_bytes == zeros));
        if (mask_comp_1 != 0) {
            @branchHint(.unlikely);
            const i = @ctz(mask_comp_1);
            _ = state.solution_1.fetchMin(@as(u32, @truncate(i)) + chunk_offset + base, .acq_rel);
            if (mask_comp_2 != 0) {
                @branchHint(.unlikely);
                const j = @ctz(mask_comp_2);
                _ = state.solution_2.fetchMin(@as(u32, @truncate(j)) + chunk_offset + base, .acq_rel);
                _ = state.iter.finish();
                return;
            }
        }
    }
    if (comptime leftovers != 0) {
        comptime std.debug.assert(std.math.isPowerOfTwo(leftovers));
        chunk_offset = 1000 - leftovers;
        write_offsets(leftovers, message_buffers[0..leftovers], offset_start_idx, chunk_offset);
        const first_four_bytes = md5(leftovers, message_buffers[0..leftovers]).a;
        const mask_comp_1: @Int(.unsigned, leftovers) = @bitCast(@intFromBool(first_four_bytes & @as(@Vector(leftovers, u32), @splat(0x00F0FFFF)) == @as(@Vector(leftovers, u32), @splat(0))));
        const mask_comp_2: @Int(.unsigned, leftovers) = @bitCast(@intFromBool(first_four_bytes & @as(@Vector(leftovers, u32), @splat(0x00FFFFFF)) == @as(@Vector(leftovers, u32), @splat(0))));
        if (mask_comp_1 != 0) {
            @branchHint(.unlikely);
            const i = @ctz(mask_comp_1);
            _ = state.solution_1.fetchMin(@as(u32, @truncate(i)) + chunk_offset + base, .acq_rel);
            if (mask_comp_2 != 0) {
                @branchHint(.unlikely);
                const j = @ctz(mask_comp_2);
                _ = state.solution_2.fetchMin(@as(u32, @truncate(j)) + chunk_offset + base, .acq_rel);
                _ = state.iter.finish();
                return;
            }
        }
    }
}

inline fn init_buffers(comptime N: usize, buffers: *[N][64]u8, prefix_len: usize, base: u32) usize {
    const base_len = encode_base(buffers[0][prefix_len..], base);
    const offset_start_idx = prefix_len + base_len - 3;
    buffers[0][prefix_len + base_len] = 0x80;
    @memset(buffers[0][prefix_len + base_len + 1 .. 56], 0);
    std.mem.writeInt(u64, buffers[0][56..64], @as(u64, @intCast(prefix_len + base_len)) * 8, .little);
    for (1..N) |i| {
        @memcpy(&buffers[i], &buffers[0]);
    }
    return offset_start_idx;
}

inline fn encode_base(buf: []u8, base: u32) usize {
    var cur = base;
    var i = buf.len - 1;
    while (cur > 0) : ({
        cur = @divTrunc(cur, 10);
        i -= 1;
    }) {
        buf[i] = @as(u8, @truncate(cur % 10)) + '0';
    }
    const len = buf.len - i - 1;
    if (len > 0) {
        @branchHint(.likely);
        @memmove(buf[0..len], buf[i + 1 ..][0..len]);
    }
    // TODO: set the bytes from max(i+1, len) to 56 to 0, if any.
    const overwrite_start = @max(i + 1, len);
    const overwrite_end = buf.len - 8;
    if (overwrite_start < overwrite_end) {
        @branchHint(.unlikely);
        @memset(buf[overwrite_start..overwrite_end], 0);
    }
    return len;
}

inline fn write_offsets(comptime N: usize, buf: *[N][64]u8, offset_start_idx: usize, chunk_offset: usize) void {
    const offset: @Vector(N, u32) = @splat(@truncate(chunk_offset));
    var indexes: @Vector(N, u32) = undefined;
    inline for (0..N) |i| {
        indexes[i] = i;
    }
    indexes += offset;
    const tens: @Vector(N, u32) = @splat(10);
    const zero_digit: @Vector(N, u8) = @splat('0');
    const ones_digit = @as(@Vector(N, u8), @truncate(indexes % tens)) + zero_digit;
    const tens_digit = @as(@Vector(N, u8), @truncate(@divTrunc(indexes, tens) % tens)) + zero_digit;
    const huns_digit = @as(@Vector(N, u8), @truncate(@divTrunc(indexes, tens * tens) % tens)) + zero_digit;
    inline for (0..N) |i| {
        buf[i][offset_start_idx + 2] = ones_digit[i];
        buf[i][offset_start_idx + 1] = tens_digit[i];
        buf[i][offset_start_idx] = huns_digit[i];
    }
}

fn md5(comptime N: usize, m: *const [N][64]u8) Md5DataType(N) {
    @setEvalBranchQuota(10000);
    const base_data = Md5DataType(N){ .a = @splat(0x67452301), .b = @splat(0xefcdab89), .c = @splat(0x98badcfe), .d = @splat(0x10325476) };
    var data = base_data;

    var message_vecs: [16]@Vector(N, u32) = undefined;
    inline for (0..16) |i| {
        inline for (0..N) |j| {
            message_vecs[i][j] = std.mem.readInt(u32, m[j][4 * i ..][0..4], .little);
        }
    }

    data = round1(N, &message_vecs, data);
    data = round2(N, &message_vecs, data);
    data = round3(N, &message_vecs, data);
    data = round4(N, &message_vecs, data);

    data.a = data.a +% base_data.a;
    data.b = data.b +% base_data.b;
    data.c = data.c +% base_data.c;
    data.d = data.d +% base_data.d;

    return data;
}

inline fn round1(comptime N: usize, m: *const [16]@Vector(N, u32), data_in: Md5DataType(N)) Md5DataType(N) {
    var data = data_in;
    const shifts = [_]u5{ 7, 12, 17, 22 };
    inline for (0..16) |i| {
        const s = shifts[i % 4];
        const k = md5_k[i];
        const f = (data.b & data.c) | (~data.b & data.d);
        data = common(N, k, s, f, m[i], data);
    }
    return data;
}

inline fn round2(comptime N: usize, m: *const [16]@Vector(N, u32), data_in: Md5DataType(N)) Md5DataType(N) {
    var data = data_in;
    const shifts = [_]u5{ 5, 9, 14, 20 };
    inline for (16..32) |i| {
        const s = shifts[i % 4];
        const k = md5_k[i];
        const f = (data.d & data.b) | (~data.d & data.c);
        data = common(N, k, s, f, m[(5 * i + 1) % 16], data);
    }
    return data;
}

inline fn round3(comptime N: usize, m: *const [16]@Vector(N, u32), data_in: Md5DataType(N)) Md5DataType(N) {
    var data = data_in;
    const shifts = [_]u5{ 4, 11, 16, 23 };
    inline for (32..48) |i| {
        const s = shifts[i % 4];
        const k = md5_k[i];
        const f = data.b ^ data.c ^ data.d;
        data = common(N, k, s, f, m[(3 * i + 5) % 16], data);
    }
    return data;
}

inline fn round4(comptime N: usize, m: *const [16]@Vector(N, u32), data_in: Md5DataType(N)) Md5DataType(N) {
    var data = data_in;
    const shifts = [_]u5{ 6, 10, 15, 21 };
    inline for (48..64) |i| {
        const s = shifts[i % 4];
        const k = md5_k[i];
        const f = data.c ^ (data.b | ~data.d);
        data = common(N, k, s, f, m[(7 * i) % 16], data);
    }
    return data;
}

inline fn common(comptime N: usize, k: u32, s: u5, f: @Vector(N, u32), m: @Vector(N, u32), data: Md5DataType(N)) Md5DataType(N) {
    const f_post = f +% data.a +% @as(@Vector(N, u32), @splat(k)) +% m;
    const old_b = data.b;
    const new_b = old_b +% std.math.rotl(@Vector(N, u32), f_post, s);
    return .{ .a = data.d, .b = new_b, .c = old_b, .d = data.c };
}

pub fn main(init: std.process.Init) !void {
    const bound_worker = struct {
        fn inner_worker(io: Io, problem_state: *ProblemState, problem_prefix: []const u8) Io.Cancelable!void {
            try worker(16, io, problem_state, problem_prefix);
        }
    }.inner_worker;

    const cwd = Io.Dir.cwd();
    const filename = "inputs/2015/04.input";
    var buf: [32]u8 = undefined;
    const prefix = std.mem.trim(u8, try cwd.readFile(init.io, filename, &buf), " \r\n");

    // This gets logical cores instead of physical, and we're going to be running the cores hard
    const core_count = try std.Thread.getCpuCount();
    std.debug.assert(core_count >= 1);

    var state: ProblemState = undefined;
    const awake_start = std.Io.Clock.now(.awake, init.io);
    const cpu_start = std.Io.Clock.now(.cpu_process, init.io);
    for (0..1000) |_| {
        std.mem.doNotOptimizeAway({
            var group = Io.Group.init;
            errdefer group.cancel(init.io);
            state = ProblemState{ .iter = .init(1000, 1000), .solution_1 = .init(0xFFFFFFFF), .solution_2 = .init(0xFFFFFFFF) };
            for (0..core_count) |_| {
                try group.concurrent(init.io, bound_worker, .{ init.io, &state, prefix });
            }
            // Setting state.iter.start to 0 instead of this loses 300us, likely from the poor worker who
            // pulls the 0 value getting branch mispredictions for life.
            var message_buffers: [16][64]u8 = undefined;
            @memcpy(message_buffers[0][0..prefix.len], prefix);
            @memset(message_buffers[0][prefix.len..64], 0);
            check_chunk(16, &message_buffers, prefix.len, 0, &state);
            try group.await(init.io);
        });
    }
    const awake_duration = awake_start.untilNow(init.io, .awake);
    const cpu_duration = cpu_start.untilNow(init.io, .cpu_process);
    std.debug.print("Average walltime: {}us\nAverage CPU time: {}us\n", .{ @as(f64, @floatFromInt(awake_duration.toMicroseconds())) / 1000, @as(f64, @floatFromInt(cpu_duration.toMicroseconds())) / 1000 });
    std.debug.print("Solution 1: {}\nSolution 2: {}\n", .{ state.solution_1.load(.monotonic), state.solution_2.load(.monotonic) });
}
