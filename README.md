# AoC 2021

<https://adventofcode.com/2021>

## Setup

```text
├── inputs/
│   └── day01.txt
├── src/
│   ├── bin/
│   │   └── day01.rs
│   ├── day01.rs
│   └── lib.rs
├── .env
├── Cargo.toml
├── gen
├── get
└── run
```

Each day is a separate binary target that reads STDIN and calls exported functions.

### Scripts

- `gen`: Generate files for a day
- `get`: Get input for a day
- `run`: Run solutions for a day

## Usage

Add `.env` with `AOC_SESSION` with the value from your cookie.

```bash
# get input for day 1
./get 1
# run solutions for day 1
./run 1
```

`./gen` can be used to generate boilerplate:

```bash
# generate files `src/day03.rs` and `src/bin/day03.rs`
# then append `pub mod day03;` to `src/lib.rs`
./gen 3
```
