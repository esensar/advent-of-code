use fmt;
use os;
use io;
use strings;
use strconv;

type range = struct {
	start: u64,
	end: u64
};

export fn main() void = {
	fmt::println("Problem 1 solution: {}", part1())!;
	fmt::println("Problem 2 solution: {}", part2())!;
};

fn read_input() []range = {
	const input_file = os::open("input.txt")!;
	defer io::close(input_file)!;

	const range_strings = strings::split(strings::fromutf8(io::drain(input_file)!)!, ",")!;
	let ranges: []range = alloc([range { start = 0, end = 0 }...], len(range_strings))!;
	let i = 0;
	for (let range .. range_strings) {
		const parts = strings::split(range, "-")!;
		ranges[i].start = strconv::stou64(strings::trim(parts[0]))!;
		ranges[i].end = strconv::stou64(strings::trim(parts[1]))!;
		i += 1;
	};
	return ranges;
};

fn repeated_pattern_twice(id: u64) bool = {
	const strid = strconv::u64tos(id);
	return strings::sub(strid, 0, len(strid) / 2) == strings::sub(strid, len(strid) / 2, strings::end);
};

@test fn repeated_pattern_twice() void = {
	assert(repeated_pattern_twice(123123));
	assert(repeated_pattern_twice(1010));
	assert(repeated_pattern_twice(55));
	assert(repeated_pattern_twice(6464));
	assert(!repeated_pattern_twice(1234));
	assert(!repeated_pattern_twice(101));
	assert(!repeated_pattern_twice(1012));
	assert(!repeated_pattern_twice(1113));
	assert(!repeated_pattern_twice(123123123));
	assert(!repeated_pattern_twice(1212121212));
	assert(!repeated_pattern_twice(1111111));
};

fn repeated_pattern_twice_or_more(id: u64) bool = {
	const strid = strconv::u64tos(id);
	for (let i: size = 2; i <= len(strid); i += 1) {
		if (len(strid) % i == 0) {
			let found = true;
			let length = len(strid) / i;
			for (let j: size = 1; j < i; j += 1) {
				if (strings::sub(strid, 0, length) != strings::sub(strid, j * length, (j + 1) * length)) {
					found = false;
					break;
				};
			};
			if (found) {
				return true;
			};
		};
	};
	return false;
};

@test fn repeated_pattern_twice_or_more() void = {
	assert(repeated_pattern_twice_or_more(12341234));
	assert(repeated_pattern_twice_or_more(123123123));
	assert(repeated_pattern_twice_or_more(1212121212));
	assert(repeated_pattern_twice_or_more(1111111));
	assert(!repeated_pattern_twice_or_more(1234));
	assert(!repeated_pattern_twice_or_more(101));
	assert(!repeated_pattern_twice_or_more(1012));
	assert(!repeated_pattern_twice_or_more(1113));
};

fn sum_invalids(id_range: range, is_invalid: *fn(id: u64) bool) u64 = {
	let sum: u64 = 0;
	for (let i = id_range.start; i <= id_range.end; i += 1) {
		if (is_invalid(i)) {
			sum += i;
		};
	};
	return sum;
};

@test fn sum_invalids() void = {
	assert(sum_invalids(range { start = 11, end = 22 }, &repeated_pattern_twice) == 33);
	assert(sum_invalids(range { start = 99, end = 115 }, &repeated_pattern_twice) == 99);
	assert(sum_invalids(range { start = 1698522, end = 1698528 }, &repeated_pattern_twice) == 0);
	assert(sum_invalids(range { start = 824824821, end = 824824827 }, &repeated_pattern_twice) == 0);
	assert(sum_invalids(range { start = 824824821, end = 824824827 }, &repeated_pattern_twice_or_more) == 824824824);
};

fn test_input() []range = {
	let ranges: []range = alloc([range{ start = 0, end = 0 }...], 11)!;
	ranges[0] = range { start = 11, end = 22 };
	ranges[1] = range { start = 95, end = 115 };
	ranges[2] = range { start = 998, end = 1012 };
	ranges[3] = range { start = 1188511880, end = 1188511890 };
	ranges[4] = range { start = 222220, end = 222224 };
	ranges[5] = range { start = 1698522, end = 1698528 };
	ranges[6] = range { start = 446443, end = 446449 };
	ranges[7] = range { start = 38593856, end = 38593862 };
	ranges[8] = range { start = 565653, end = 565659 };
	ranges[9] = range { start = 824824821, end = 824824827 };
	ranges[10] = range { start = 2121212118, end = 2121212124 };
	return ranges;
};

fn part1() size = {
	let ranges = read_input();
	defer free(ranges);

	let sum: u64 = 0;
	for (let range .. ranges) {
		sum += sum_invalids(range, &repeated_pattern_twice);
	};
	return sum;
};

@test fn part1() void = {
	let ranges = test_input();
	defer free(ranges);

	let sum: u64 = 0;
	for (let range .. ranges) {
		sum += sum_invalids(range, &repeated_pattern_twice);
	};

	assert(sum == 1227775554);
};

fn part2() size = {
	let ranges = read_input();
	defer free(ranges);

	let sum: u64 = 0;
	for (let range .. ranges) {
		sum += sum_invalids(range, &repeated_pattern_twice_or_more);
	};
	return sum;
};

@test fn part2() void = {
	let ranges = test_input();
	defer free(ranges);

	let sum: u64 = 0;
	for (let range .. ranges) {
		sum += sum_invalids(range, &repeated_pattern_twice_or_more);
	};

	assert(sum == 4174379265);
};
