use fmt;
use os;
use io;
use strings;
use strconv;

type direction = enum { LEFT, RIGHT };
type rotation = struct {
	dir: direction,
	val: u32
};

export fn main() void = {
	fmt::println("Problem 1 solution: {}", part1())!;
	fmt::println("Problem 2 solution: {}", part2())!;
};

fn read_input() []rotation = {
	const input_file = os::open("input.txt")!;
	defer io::close(input_file)!;

	const lines = strings::split(strings::fromutf8(io::drain(input_file)!)!, "\n")!;
	let rotations: []rotation = alloc([rotation { dir = direction::LEFT, val = 0 }...], len(lines))!;
	let i = 0;
	for (let line .. lines) {
		if (len(line) == 0) {
			continue;
		};
		const dir = strings::sub(line, 0, 1);
		const val = strconv::stou32(strings::sub(line, 1, strings::end))!;
		rotations[i].val = val;
		switch (dir) {
		case "R" => rotations[i].dir = direction::RIGHT;
		case "L" => rotations[i].dir = direction::LEFT;
		case => continue;
		};
		i += 1;
	};
	return rotations;
};

fn part1() size = {
	let rotations = read_input();
	defer free(rotations);

	let zeroes: size = 0;
	let current: i64 = 50;

	for (let rotation .. rotations) {
		switch (rotation.dir) {
		case direction::LEFT => current -= rotation.val: i64;
		case direction::RIGHT => current += rotation.val: i64;
		};
		for (current < 0) {
			current += 100;
		};
		current %= 100;
		if (current == 0) {
			zeroes += 1;
		};
	};

	return zeroes;
};

fn part2() size = {
	let rotations = read_input();
	defer free(rotations);

	let zeroes: size = 0;
	let current: i64 = 50;

	for (let rotation .. rotations) {
		let left = rotation.val;
		for (left > 0) {
			switch (rotation.dir) {
			case direction::LEFT => {
				current -= 1;
				if (current == 0) {
					zeroes += 1;
				};
				if (current < 0) {
					current += 100;
				};
			};
		case direction::RIGHT => {
			current += 1;
			if (current == 100) {
				current = 0;
				zeroes += 1;
			};
		};
			};
			left -= 1;
		};
	};

	return zeroes;
};
