const std = @import("std");
const atomic = std.atomic;

/// Iterator over a shared counter; has a flag for marking the counter as finished.
/// Intended to be used when each iteration's work is roughly equal; see WorkStealingIterator
/// for the alternative when per-iteration costs can vary between items.
pub fn AtomicIterator(comptime T: type) type {
    return struct {
        const Self = @This();

        counter: atomic.Value(T),
        finished: atomic.Value(bool) = .init(false),
        increment: T,

        pub fn init(start: T, increment: T) Self {
            return .{ .counter = .init(start), .increment = increment };
        }

        pub fn next(self: *Self) ?T {
            if (self.finished.load(.monotonic)) {
                @branchHint(.unlikely);
                return null;
            } else {
                return self.counter.fetchAdd(self.increment, .monotonic);
            }
        }

        pub fn finish(self: *Self) bool {
            return self.finished.swap(true, .monotonic);
        }
    };
}

pub fn CachePadded(comptime T: type) type {
    comptime {
        const alignment = std.mem.alignForward(usize, @alignOf(T), atomic.cache_line);
        std.debug.assert(alignment >= @alignOf(T));
        std.debug.assert(alignment >= atomic.cache_line);
        return struct {
            val: T align(alignment),
        };
    }
}

pub const Bounds = packed struct(u64) { start: u32, end: u32 };
pub const State = CachePadded(atomic.Value(u64));
const INCREMENT: u64 = @backingInt(Bounds{ .start = 1, .end = 0 });

pub const WorkStealingIterator = struct {
    pub const Child = struct {
        index: usize,
        parent: *WorkStealingIterator,

        pub fn next(self: *Child) ?u32 {
            var state = &self.parent.children_state[self.index].val;
            const fetched: Bounds = @fromBackingInt(state.fetchAdd(INCREMENT, .monotonic));
            if (fetched.start < fetched.end) {
                return fetched.start;
            } else {
                return stealWork(self.parent.children_state, self.index, fetched);
            }
        }
    };

    children_state: []State,

    pub fn initAlloc(start: u32, end: u32, worker_count: u32, alloc: std.mem.Allocator) !WorkStealingIterator {
        const states = try alloc.alloc(State, worker_count);
        return .init(start, end, states);
    }

    pub fn init(start: u32, end: u32, children_state: []State) !WorkStealingIterator {
        std.debug.assert(start < end);
        const len = end - start;
        const per_worker_size = try std.math.divCeil(u32, len, @intCast(children_state.len));
        for (children_state, 0..) |*state, i| {
            const state_start = @min(len, (i * per_worker_size));
            const state_end = @min(len, state_start + per_worker_size);
            state.* = .{ .val = .init(@backingInt(Bounds{ .start = state_start, .end = state_end })) };
        }

        return .{ .children_state = children_state };
    }

    pub fn free(self: *WorkStealingIterator, alloc: std.mem.Allocator) void {
        alloc.free(self.children_state);
    }

    pub fn child(self: *WorkStealingIterator, idx: usize) Child {
        return .{ .index = idx, .parent = self };
    }
};

fn stealWork(children_state: []State, index: usize, current_bounds: Bounds) ?u32 {
    @branchHint(.cold);
    std.debug.assert(current_bounds.end -| current_bounds.start == 0);
    while (true) {
        var candidate_idx: usize = undefined;
        var candidate_bounds: Bounds = undefined;
        var candidate_length: u32 = 0;

        // Going like this to reduce contention over the earlier indices
        for (children_state[index + 1 ..], index + 1..) |*other, other_idx| {
            const other_bounds: Bounds = @fromBackingInt(other.val.load(.monotonic));
            const other_length = other_bounds.end -| other_bounds.start;
            if (candidate_length < other_length) {
                candidate_length = other_length;
                candidate_bounds = other_bounds;
                candidate_idx = other_idx;
            }
        }
        for (children_state[0..index], 0..) |*other, other_idx| {
            const other_bounds: Bounds = @fromBackingInt(other.val.load(.monotonic));
            const other_length = other_bounds.end -| other_bounds.start;
            if (candidate_length < other_length) {
                candidate_length = other_length;
                candidate_bounds = other_bounds;
                candidate_idx = other_idx;
            }
        }

        if (candidate_length < 4) return null;

        const middle = candidate_bounds.start + (candidate_length / 2);
        std.debug.assert(candidate_bounds.start < middle and middle < candidate_bounds.end);
        const replacement = Bounds{ .start = candidate_bounds.start, .end = middle };
        if (children_state[candidate_idx].val.cmpxchgWeak(
            @backingInt(candidate_bounds),
            @backingInt(replacement),
            .monotonic,
            .monotonic,
        )) |_| {
            continue;
        } else {
            const mine = Bounds{ .start = middle + 1, .end = candidate_bounds.end };
            children_state[index].val.store(@backingInt(mine), .monotonic);
            return middle;
        }
    }
}
