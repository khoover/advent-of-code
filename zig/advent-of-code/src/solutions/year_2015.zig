const spec = @import("spec.zig");

const days = [_]spec.DaySpec{
    .{ .day = "01".*, .run = &@import("year_2015/day_01.zig").run },
    .{ .day = "02".*, .run = &@import("year_2015/day_02.zig").run },
    .{ .day = "03".*, .run = &@import("year_2015/day_03.zig").run },
    .{ .day = "04".*, .run = &@import("year_2015/day_04.zig").run },
};

pub const YEAR_SPEC: spec.YearSpec = .{
    .year = "2015".*,
    .days = &days,
};

test "2015" {
    @import("std").testing.refAllDecls(@This());
}
