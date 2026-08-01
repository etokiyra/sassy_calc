# sassy_calc 🧮

A crappy little terminal calculator, written in Rust as a first "real" beginner project.

No fancy math, no GUI, just `<num> <op> <num>` and a bit of attitude. Built step by step while learning Rust basics — ownership, `Result`/`Option`, `match`, and just enough error handling to not immediately crash (usually).

## What it does

- Reads a math expression from the terminal
- Supports `+`, `-`, `*`, `/`
- Loops so you can do multiple calculations
- Yells at you (politely) if your input is garbage

## Requirements

- [Rust and Cargo](https://www.rust-lang.org/tools/install) installed

## Download & run

```bash
git clone https://github.com/etokiyra/sassy_calc.git
cd sassy_calc
cargo run
```

## Usage

Once running, type an expression in the format:

```
<number> <operator> <number>
```

Example:

```
> 3 + 4
= 7
```

Supported operators: `+` `-` `*` `/`

Exit anytime with `Ctrl+C`.

## License

Public domain — see [LICENSE](LICENSE).
