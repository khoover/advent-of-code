const std = @import("std");

pub fn main(init: std.process.Init) !void {
    var iter = try init.minimal.args.iterateAllocator(init.gpa);
    defer iter.deinit();
    const year_day = try get_year_day(&iter);
    std.debug.print("year: {?}, day: {?}\n", .{ year_day.year, year_day.day });
}

// TODO: Make a packed wrapper for the multiple-of-8 sizes.
const YearDay = struct { year: ?u16, day: ?u5 };

fn get_year_day(iter: *std.process.Args.Iterator) !YearDay {
    var res = YearDay{ .year = null, .day = null };
    if (!iter.skip()) {
        return res;
    }
    res.year = try std.fmt.parseInt(u16, iter.next() orelse return res, 10);
    res.day = try std.fmt.parseInt(u5, iter.next() orelse return res, 10);
    return res;
}
