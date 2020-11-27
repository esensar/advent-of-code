# Advent of Code

This repository includes my solutions for [Advent of Code](https://adventofcode.com/).

## Scripts

All scripts are expected to be run from repository root. All scripts also expect environment variable AOC_SESSION to be set, which allows it to load user specific data. This can be easily set up using [direnv](https://direnv.net/)

To load problem and input file for current day, run:
```sh
scripts/load_latest_problem
```

To load problem and input file for a specific day, run:
```sh
scripts/load_problem [year] [day]
```

To load only input file for current day, run:
```sh
scripts/load_latest_input
```

To load only input for a specific day, run:
```sh
scripts/load_input [year] [day]
```
