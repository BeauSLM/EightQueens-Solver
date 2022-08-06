const std = @import("std");
const dbprint = std.debug.print;

var board = [_]i8{-1} ** 8;

fn solve(row: i8) void {
    if (row > 7) {
        for (board) |item| { dbprint("{d}", .{item}); }
        dbprint("\n", .{});
        std.process.exit(0);
    }
    var col: i8 = 0;
    outer: while (col < 8) : (col += 1) {
        var prev_row: i8 = 0;
        while (prev_row < row) : (prev_row += 1) {
            var prev_col = board[@intCast(usize, prev_row)];
            std.debug.assert(prev_col > -1);
            if (prev_col == col
                or prev_col - prev_row == col - row
                or prev_col + prev_row == col + row) {
                    continue :outer;
                }
        }
        board[@intCast(usize, row)] = col;
        solve(row + 1);
        board[@intCast(usize, row)] = -1;
    }
}

pub fn main() void {
    solve(0);
}
