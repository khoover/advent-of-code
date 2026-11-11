//! Represents both a 2D point on a grid along with offsets on the grid.
//! Uses the standard coordinate system where +y is up, +x is right.
const Point = @This();

x: i32,
y: i32,

pub const ORIGIN: Point = .{ .x = 0, .y = 0 };
pub const UP: Point = .{ .x = 0, .y = 1 };
pub const DOWN: Point = .{ .x = 0, .y = -1 };
pub const LEFT: Point = .{ .x = -1, .y = 0 };
pub const RIGHT: Point = .{ .x = 1, .y = 0 };

/// Maps ^v<> and UDLR into unit direction points.
pub fn fromDirectionChar(c: u8) !Point {
    return switch (c) {
        '^', 'U' => UP,
        'v', 'D' => DOWN,
        '<', 'L' => LEFT,
        '>', 'R' => RIGHT,
        else => error.InvalidDirectionCharacter,
    };
}

pub fn add(a: Point, b: Point) Point {
    return .{ .x = a.x + b.x, .y = a.y + b.y };
}

pub fn sub(a: Point, b: Point) Point {
    return .{ .x = a.x - b.x, .y = a.y - b.y };
}

pub fn neg(p: Point) Point {
    return .{ .x = -p.x, .y = -p.y };
}

pub fn rot90Cw(p: Point) Point {
    return .{ .x = p.y, .y = -p.x };
}

pub fn rot90Ccw(p: Point) Point {
    return .{ .x = -p.y, .y = p.x };
}

test add {
    const testing = @import("std").testing;

    try testing.expectEqual(ORIGIN.add(UP), UP);
    try testing.expectEqual(UP.add(DOWN), ORIGIN);
    try testing.expectEqual(UP.add(LEFT), Point{ .x = -1, .y = 1 });
}

test sub {
    const testing = @import("std").testing;

    try testing.expectEqual(ORIGIN.sub(UP), DOWN);
    try testing.expectEqual(UP.sub(UP), ORIGIN);
}

test neg {
    const testing = @import("std").testing;

    try testing.expectEqual(ORIGIN.neg(), ORIGIN);
    try testing.expectEqual(UP.neg(), DOWN);
    try testing.expectEqual(LEFT.neg(), RIGHT);
}

test rot90Cw {
    const testing = @import("std").testing;

    try testing.expectEqual(ORIGIN.rot90Cw(), ORIGIN);
    try testing.expectEqual(UP.rot90Cw(), RIGHT);
}

test rot90Ccw {
    const testing = @import("std").testing;

    try testing.expectEqual(ORIGIN.rot90Ccw(), ORIGIN);
    try testing.expectEqual(UP.rot90Ccw(), LEFT);
}
