const std = @import("std");

fn read_file(gpa: std.mem.Allocator) !struct { std.ArrayList(u32), std.ArrayList(u32) } {
    const input = try std.fs.cwd().openFile("input.txt", .{ .mode = .read_only });
    defer input.close();
    var read_buf: [4096]u8 = undefined;

    var reader = input.reader(&read_buf);

    const contents = try std.io.Reader.allocRemaining(&reader.interface, gpa, .unlimited);

    var lines = std.mem.tokenizeAny(u8, contents, "\n");
    return try parse_lines(gpa, &lines);
}

fn parse_lines(gpa: std.mem.Allocator, lines: *std.mem.TokenIterator(u8, .any)) !struct { std.ArrayList(u32), std.ArrayList(u32) } {
    var left: std.ArrayList(u32) = .empty;
    var right: std.ArrayList(u32) = .empty;
    while (lines.next()) |line| {
        var objs = std.mem.tokenizeAny(u8, line, " \t");
        try left.append(gpa, try std.fmt.parseInt(u32, objs.next().?, 10));
        try right.append(gpa, try std.fmt.parseInt(u32, objs.next().?, 10));
    }
    return .{ left, right };
}

fn sum_diffs(left: *std.ArrayList(u32), right: *std.ArrayList(u32)) u32 {
    std.mem.sort(u32, left.items, {}, comptime std.sort.asc(u32));
    std.mem.sort(u32, right.items, {}, comptime std.sort.asc(u32));

    var sum: u32 = 0;
    for (left.items, right.items) |l, r| {
        sum += @intCast(@abs(@as(i64, l) - @as(i64, r)));
    }
    return sum;
}

fn similarity_score(left: *std.ArrayList(u32), right: *std.ArrayList(u32)) usize {
    std.mem.sort(u32, left.items, {}, comptime std.sort.asc(u32));
    std.mem.sort(u32, right.items, {}, comptime std.sort.asc(u32));

    var score: usize = 0;
    for (left.items) |l| {
        var equals: usize = 0;
        for (right.items) |r| {
            if (l == r) {
                equals += 1;
            }
        }

        score += l * equals;
    }

    return score;
}

fn part1(gpa: std.mem.Allocator) !usize {
    var left, var right = try read_file(gpa);
    return sum_diffs(&left, &right);
}

fn part2(gpa: std.mem.Allocator) !usize {
    var left, var right = try read_file(gpa);
    return similarity_score(&left, &right);
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();
    std.debug.print("Problem 1 solution: {}\n", .{try part1(alloc)});
    std.debug.print("Problem 2 solution: {}\n", .{try part2(alloc)});
}

test "test_part_1_example" {
    const alloc = std.testing.allocator;
    var lines = std.mem.tokenizeAny(u8, "3\t4\n4\t3\n2\t5\n1\t3\n3\t9\n3\t3\n", "\n");
    var left, var right = try parse_lines(alloc, &lines);
    defer left.deinit(alloc);
    defer right.deinit(alloc);

    try std.testing.expectEqual(11, sum_diffs(&left, &right));
}

test "test_part_2_example" {
    const alloc = std.testing.allocator;
    var lines = std.mem.tokenizeAny(u8, "3\t4\n4\t3\n2\t5\n1\t3\n3\t9\n3\t3\n", "\n");
    var left, var right = try parse_lines(alloc, &lines);
    defer left.deinit(alloc);
    defer right.deinit(alloc);

    try std.testing.expectEqual(31, similarity_score(&left, &right));
}
