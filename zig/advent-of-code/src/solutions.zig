const spec = @import("solutions/spec.zig");

pub const ALL_YEARS = [_]spec.YearSpec{
    @import("solutions/year_2015.zig").YEAR_SPEC,
};
