use fmt;
use os;
use io;
use math;
use strings;
use strconv;

type bank = struct {
	batteries: []u8
};

export fn main() void = {
	fmt::println("Problem 1 solution: {}", part1())!;
	fmt::println("Problem 2 solution: {}", part2())!;
};

fn read_input() []bank = {
	const input_file = os::open("input.txt")!;
	defer io::close(input_file)!;

	const bank_strings = strings::split(strings::fromutf8(io::drain(input_file)!)!, "\n")!;
	let banks: []bank = alloc([bank { batteries = [] }...], len(bank_strings))!;
	let i = 0;
	for (let bank .. bank_strings) {
		if (len(bank) == 0) {
			continue;
		};
		const parts = strings::torunes(bank)!;
		let batteries: []u8 = alloc([0...], len(parts))!;
		let j = 0;
		for (let part .. parts) {
			batteries[j] = strconv::stou8(strings::fromrunes([part])!)!;
			j += 1;
		};
		banks[i].batteries = batteries;
		i += 1;
	};
	return banks;
};

fn bank_max_joltage(b: bank, length: size) u64 = {
	if (len(b.batteries) == 0) {
		return 0;
	};
	let value: u64 = 0;
	let maxi: size = 0;
	for (let end_limit: size = length - 1; end_limit >= 0; end_limit -= 1) {
		let max: u8 = 0;

		let i: size = maxi;
		for (i < len(b.batteries) - end_limit; i += 1) {
			if (b.batteries[i] > max) {
				max = b.batteries[i];
				maxi = i + 1;
			};
		};

		value += max * (math::powi64(10, end_limit: uint): u64);
		if (end_limit == 0) {
			break;
		};
	};
	return value;
};

@test fn bank_max_joltage() void = {
	assert(bank_max_joltage(bank { batteries = [9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1] }, 2) == 98);
	assert(bank_max_joltage(bank { batteries = [8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9] }, 2) == 89);
	assert(bank_max_joltage(bank { batteries = [2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8] }, 2) == 78);
	assert(bank_max_joltage(bank { batteries = [8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1] }, 2) == 92);
	assert(bank_max_joltage(bank { batteries = [9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1] }, 12) == 987654321111);
	assert(bank_max_joltage(bank { batteries = [8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9] }, 12) == 811111111119);
	assert(bank_max_joltage(bank { batteries = [2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8] }, 12) == 434234234278);
	assert(bank_max_joltage(bank { batteries = [8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1] }, 12) == 888911112111);
};

fn total_max_joltage(banks: []bank, length: size) u64 = {
	let sum: u64 = 0;

	for (let bank .. banks) {
		sum += bank_max_joltage(bank, length);
	};

	return sum;
};

fn test_input() []bank = {
	return alloc([
		bank { batteries = alloc([9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1])! },
		bank { batteries = alloc([8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9])! },
		bank { batteries = alloc([2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8])! },
		bank { batteries = alloc([8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1])! }
	])!;
};

@test fn total_max_joltage() void = {
	assert(total_max_joltage(test_input(), 2) == 357);
	assert(total_max_joltage(test_input(), 12) == 3121910778619);
};

fn part1() size = {
	let banks = read_input();
	defer free(banks);

	return total_max_joltage(banks, 2);
};

@test fn part1() void = {
	let banks = test_input();

	assert(total_max_joltage(banks, 2) == 357);
};

fn part2() size = {
	let banks = read_input();
	defer free(banks);

	return total_max_joltage(banks, 12);
};

@test fn part2() void = {
	let banks = test_input();

	assert(total_max_joltage(banks, 12) == 3121910778619);
};
