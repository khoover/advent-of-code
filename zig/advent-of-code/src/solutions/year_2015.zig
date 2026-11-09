const spec = @import("spec.zig");

const days = [_]spec.DaySpec{
    .{ .day = "04".*, .run = &@import("year_2015/day_04.zig").run },
};

pub const YEAR_SPEC: spec.YearSpec = .{
    .year = "2015".*,
    .days = &days,
};
