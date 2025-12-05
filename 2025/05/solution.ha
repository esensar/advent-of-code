use fmt;
use os;
use io;
use strings;
use strconv;
use types;

type range = struct {
	start: size,
	end: size
};
type inventory = struct {
	fresh_ranges: []range,
	available: []size
};

export fn main() void = {
	fmt::println("Problem 1 solution: {}", part1())!;
	fmt::println("Problem 2 solution: {}", part2())!;
};

fn inventory_free(i: inventory) void = {
	free(i.fresh_ranges);
	free(i.available);
};

fn parse_inventory(input: str) inventory = {
	const lines = strings::split(input, "\n")!;
	defer free(lines);
	let empty_line_index: size = 0;
	for (let i: size = 0; i < len(lines); i += 1) {
		if (strings::trim(lines[i]) == "") {
			empty_line_index = i;
			break;
		};
	};

	let fresh_ranges: []range = alloc([range {...}...], empty_line_index)!;
	let available: []size = alloc([0...], len(lines) - empty_line_index - 2)!;

	for (let i: size = 0; i < empty_line_index; i += 1) {
		let parts = strings::splitn(lines[i], "-", 2)!;
		defer free(parts);
		fresh_ranges[i] = range {
			start = strconv::stoz(parts[0])!,
			end = strconv::stoz(parts[1])!
		};
	};

	for (let i: size = empty_line_index + 1; i < len(lines) - 1; i += 1) {
		available[i - empty_line_index - 1] = strconv::stoz(lines[i])!;
	};
	return inventory {
		fresh_ranges = fresh_ranges,
		available = available
	};
};

fn read_input() inventory = {
	const input_file = os::open("input.txt")!;
	defer io::close(input_file)!;
	return parse_inventory(strings::fromutf8(io::drain(input_file)!)!);
};

fn range_contains(r: range, id: size) bool = {
	if (id >= r.start && id <= r.end) {
		return true;
	} else {
		return false;
	};
};

@test fn range_contains() void = {
	assert(!range_contains(range { start = 3, end = 5 }, 1));
	assert(range_contains(range { start = 3, end = 5 }, 3));
	assert(range_contains(range { start = 3, end = 5 }, 4));
	assert(range_contains(range { start = 3, end = 5 }, 5));
	assert(!range_contains(range { start = 3, end = 5 }, 8));
};

fn range_len(r: range) size = {
	if (r.start > r.end) {
		return 0;
	};
	return r.end - r.start + 1;
};

fn range_remove_overlap(r: range, other: range) (range, range) = {
	if ((r.start < other.start && r.end < other.start) || (r.start > other.end && r.end > other.end)) {
		return (r, range { start = 1, end = 0});
	};

	// left overlap
	if (r.start <= other.start && r.end <= other.end) {
		return (range {
			start = r.start,
			end = other.start - 1
		}, range { start = 1, end = 0 });
	};

	// right overlap
	if (r.start >= other.start && r.end >= other.end) {
		return (range {
			start = other.end + 1,
			end = r.end
		}, range { start = 1, end = 0});
	};

	if (r.start <= other.start && r.end >= other.end) {
	return (
		range {
			start = r.start,
			end = other.start - 1
		},
		range {
			start = other.end + 1,
			end = r.end
		}
	);
	};

	// full overlap
	return (range {
		start = 1,
		end = 0
	}, range { start = 1, end = 0 });
};

@test fn range_len() void = {
	assert(range_len(range { start = 3, end = 5}) == 3);
	assert(range_len(range { start = 10, end = 14}) == 5);
	assert(range_len(range { start = 16, end = 20}) == 5);
};

fn fresh_ingredients(inv: inventory) size = {
	let result: size = 0;

	for (let i: size = 0; i < len(inv.available); i += 1) {
		for (let j: size = 0; j < len(inv.fresh_ranges); j += 1) {
			if (range_contains(inv.fresh_ranges[j], inv.available[i])) {
				result += 1;
				break;
			};
		};
	};
	return result;
};

@test fn fresh_ingredients() void = {
	let inventory = test_input();
	defer inventory_free(inventory);

	assert(fresh_ingredients(inventory) == 3);
};

fn range_len_without_overlap(inv: inventory, range: range, index: size, limit: size) size = {
	if (range_len(range) == 0) {
		return 0;
	};
	if (index >= limit) {
		return range_len(range);
	};

	let (l, r) = range_remove_overlap(range, inv.fresh_ranges[index]);
	return range_len_without_overlap(inv, l, index + 1, limit) + range_len_without_overlap(inv, r, index + 1, limit);
};

fn all_fresh_ingredients(inv: inventory) size = {
	let result: size = 0;
	for (let i: size = 0; i < len(inv.fresh_ranges); i += 1) {
		result += range_len_without_overlap(inv, inv.fresh_ranges[i], 0, i);
	};
	return result;
};

@test fn all_fresh_ingredients() void = {
	let inventory = test_input();
	defer inventory_free(inventory);

	assert(all_fresh_ingredients(inventory) == 14);
};

fn test_input() inventory = {
	return parse_inventory(`3-5
10-14
16-20
12-18

1
5
8
11
17
32
`);
};

fn part1() size = {
	let inventory = read_input();
	defer inventory_free(inventory);

	return fresh_ingredients(inventory);
};

@test fn part1() void = {
	let inventory = test_input();
	defer inventory_free(inventory);

	assert(fresh_ingredients(inventory) == 3);
};

fn part2() size = {
	let inventory = read_input();
	defer inventory_free(inventory);

	return all_fresh_ingredients(inventory);
};

@test fn part2() void = {
	let inventory = test_input();
	defer inventory_free(inventory);

	assert(all_fresh_ingredients(inventory) == 14);
};
