const std = @import("std");

const Direction = enum { left, right };
const Rotation = union(Direction) { left: u32, right: u32 };
const ParseError = error{unknown_direction};

fn read_file(gpa: std.mem.Allocator) !std.ArrayList(Rotation) {
    const input = try std.fs.cwd().openFile("input.txt", .{ .mode = .read_only });
    defer input.close();
    var read_buf: [4096]u8 = undefined;

    var reader = input.reader(&read_buf);

    const contents = try std.io.Reader.allocRemaining(&reader.interface, gpa, .unlimited);

    var lines = std.mem.tokenizeAny(u8, contents, "\n");
    return try parse_lines(gpa, &lines);
}

fn parse_lines(gpa: std.mem.Allocator, lines: *std.mem.TokenIterator(u8, .any)) !std.ArrayList(Rotation) {
    var rotations: std.ArrayList(Rotation) = .empty;
    while (lines.next()) |line| {
        const value = try std.fmt.parseInt(u32, line[1..], 10);
        switch (line[0]) {
            'L' => try rotations.append(gpa, Rotation{ .left = value }),
            'R' => try rotations.append(gpa, Rotation{ .right = value }),
            else => return ParseError.unknown_direction,
        }
    }
    return rotations;
}

fn execute_rotations(gpa: std.mem.Allocator, rotations: *std.ArrayList(Rotation)) !std.ArrayList(u32) {
    var states: std.ArrayList(u32) = .empty;

    var current: i64 = 50;

    for (rotations.items) |rotation| {
        switch (rotation) {
            .left => |val| current -= val,
            .right => |val| current += val,
        }

        while (current < 0) {
            current += 100;
        }
        current = @mod(current, 100);
        try states.append(gpa, @intCast(current));
    }
    return states;
}

fn count_zero_clicks(rotations: *std.ArrayList(Rotation)) !usize {
    var zeroes: usize = 0;

    var current: i64 = 50;

    for (rotations.items) |rotation| {
        switch (rotation) {
            .left => |val| {
                var left = val;
                while (left > 0) {
                    current -= 1;
                    if (current == 0) {
                        zeroes += 1;
                    }
                    if (current < 0) {
                        current += 100;
                    }
                    left -= 1;
                }
            },
            .right => |val| {
                var left = val;
                while (left > 0) {
                    current += 1;
                    if (current == 100) {
                        current = 0;
                        zeroes += 1;
                    }
                    left -= 1;
                }
            },
        }
    }
    return zeroes;
}

fn part1(gpa: std.mem.Allocator) !usize {
    var rotations = try read_file(gpa);
    const states = try execute_rotations(gpa, &rotations);
    var zeroes: usize = 0;
    for (states.items) |state| {
        if (state == 0) {
            zeroes += 1;
        }
    }

    return zeroes;
}

fn part2(gpa: std.mem.Allocator) !usize {
    var rotations = try read_file(gpa);
    return count_zero_clicks(&rotations);
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
    var lines = std.mem.tokenizeAny(u8, "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82", "\n");
    var rotations = try parse_lines(alloc, &lines);
    defer rotations.deinit(alloc);

    var states = try execute_rotations(alloc, &rotations);
    defer states.deinit(alloc);
    var zeroes: usize = 0;
    for (states.items) |state| {
        if (state == 0) {
            zeroes += 1;
        }
    }

    try std.testing.expectEqual(3, zeroes);
}

test "test_part_2_example" {
    const alloc = std.testing.allocator;
    var lines = std.mem.tokenizeAny(u8, "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82", "\n");
    var rotations = try parse_lines(alloc, &lines);
    defer rotations.deinit(alloc);

    try std.testing.expectEqual(6, count_zero_clicks(&rotations));
}
