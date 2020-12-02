# Advent of Code

This repository includes my solutions for [Advent of Code](https://adventofcode.com/).

## Scripts

All scripts are expected to be run from repository root. All scripts also expect environment variable `AOC_SESSION` to be set, which allows it to load user specific data. This can be easily set up using [direnv](https://direnv.net/).

### Loading

To load problem and input file for current day, run:
```sh
scripts/load_latest_problem
```

To load problem and input file for a specific day, run:
```sh
scripts/load_problem [year] [day]
```

### Submitting

To submit solution for current day, run:
```sh
scripts/submit_latest_solution [solution]
```

To submit solution for a specific day, run:
```sh
scripts/submit_solution [year] [day] [solution]
```

### Load input file only

To load only input file for current day, run:
```sh
scripts/load_latest_input
```

To load only input for a specific day, run:
```sh
scripts/load_input [year] [day]
```

### Workflow

Load problem using load_latest_problem script. Work on a solution in its directory (YEAR/DAY/). When done and result is available, submit solution using submit_latest_solution script.

After submitting solution, to load second part, use same scripts as for loading the first part. Solution is submitted the same way as well.
