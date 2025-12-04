use fmt;
use os;
use io;
use math;
use math::checked;
use strings;
use strconv;

type slot = enum {
	EMPTY, ROLL
};
type grid = struct {
	slots: [][]slot
};

export fn main() void = {
	fmt::println("Problem 1 solution: {}", part1())!;
	fmt::println("Problem 2 solution: {}", part2())!;
};

fn grid_free(g: grid) void = {
	for (let slot .. g.slots) {
		free(slot);
	};
	free(g.slots);
};

fn grid_copy(g: grid) grid = {
	let slots: [][]slot = alloc([[]...], len(g.slots))!;
	for (let i: size = 0; i < len(g.slots); i += 1) {
		slots[i] = alloc([slot::EMPTY...], len(g.slots[i]))!;
		for (let j: size = 0; j < len(g.slots[i]); j += 1) {
			slots[i][j] = g.slots[i][j];
		};
	};
	return grid {
		slots = slots
	};
};

fn parse_grid(input: str) grid = {
	const row_strings = strings::split(input, "\n")!;
	let slots: [][]slot = alloc([[]...], len(row_strings) - 1)!;
	let i = 0;
	for (let row .. row_strings) {
		if (len(row) == 0) {
			continue;
		};
		const parts = strings::torunes(row)!;
		let row_slots: []slot = alloc([slot::EMPTY...], len(parts))!;
		let j = 0;
		for (let part .. parts) {
			switch (part) {
			case '.' => row_slots[j] = slot::EMPTY;
			case '@' => row_slots[j] = slot::ROLL;
			case => fmt::fatalf("unknown slot {}", part);
			};
			j += 1;
		};
		if (len(row_slots) > 0) {
			slots[i] = row_slots;
		};
		i += 1;
	};
	return grid {
		slots = slots
	};
};

fn read_input() grid = {
	const input_file = os::open("input.txt")!;
	defer io::close(input_file)!;
	return parse_grid(strings::fromutf8(io::drain(input_file)!)!);
};

fn adjacent_rolls(g: grid, x: size, y: size) size = {
	let result: size = 0;
	let imax = y + 1;
	if (imax >= len(g.slots)) {
		imax = len(g.slots) - 1;
	};
	let jmax = x + 1;
	if (jmax >= len(g.slots[0])) {
		jmax = len(g.slots[0]) - 1;
	};
	for (let i: size = math::checked::sat_subz(y, 1); i <= imax; i += 1) {

		for (let j: size = math::checked::sat_subz(x, 1); j <= jmax; j += 1) {
			if ((i != y || j != x) && g.slots[i][j] == slot::ROLL) {
				result += 1;
			};
		};
	};
	return result;
};

@test fn adjacent_rolls() void = {
	let grid = test_input();
	defer grid_free(grid);

	assert(adjacent_rolls(grid, 2, 0) == 3);
	assert(adjacent_rolls(grid, 3, 0) == 3);
	assert(adjacent_rolls(grid, 1, 1) == 6);
};

fn accessible_rolls(g: grid) size = {
	let result: size = 0;
	for (let i: size = 0; i < len(g.slots); i += 1) {
		for (let j: size = 0; j < len(g.slots[i]); j += 1) {
			if (g.slots[i][j] == slot::ROLL && adjacent_rolls(g, j, i) < 4) {
				result += 1;
			};
		};
	};
	return result;
};

fn test_input() grid = {
	return parse_grid(`..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
`);
};

fn remove_rolls(g: grid) (grid, size) = {
	let new_grid = grid_copy(g);
	let result: size = 0;
	for (let i: size = 0; i < len(g.slots); i += 1) {
		for (let j: size = 0; j < len(g.slots[i]); j += 1) {
			if (g.slots[i][j] == slot::ROLL && adjacent_rolls(g, j, i) < 4) {
				result += 1;
				new_grid.slots[i][j] = slot::EMPTY;
			};
		};
	};
	return (new_grid, result);
};

fn removable_rolls(g: grid) size = {
	let result: size = 0;
	let temp_grid = grid_copy(g);
	for (true) {
		let (new_grid, removed) = remove_rolls(temp_grid);
		grid_free(temp_grid);
		temp_grid = new_grid;
		result += removed;
		if (removed == 0) {
			break;
		};
	};
	grid_free(temp_grid);
	return result;
};

fn part1() size = {
	let grid = read_input();
	defer grid_free(grid);

	return accessible_rolls(grid);
};

@test fn part1() void = {
	let grid = test_input();
	defer grid_free(grid);

	assert(accessible_rolls(grid) == 13);
};

fn part2() size = {
	let grid = read_input();
	defer grid_free(grid);

	return removable_rolls(grid);
};

@test fn part2() void = {
	let grid = test_input();
	defer grid_free(grid);

	assert(removable_rolls(grid) == 43);
};
