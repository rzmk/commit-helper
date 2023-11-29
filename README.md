# commit-helper (ch)

A simple tool to help write commit messages and run git commands.

![Demo](demo.gif)

## Installation

```bash
cargo install --git https://github.com/rzmk/commit-helper
```

## Usage

To run the tool, simply run `ch` in your terminal.

```bash
ch
```

### `-a`

If you want to run `git add -A` before committing, use the `-a` flag

```bash
ch -a
```

### `--dry-run` or `-d`

If you want to do a dry run without actually adding or committing, use the `ch -d` or `ch --dry-run` flag.

```bash
ch -d
```

### `--debug`

If you want to see the debug output, use the `ch --debug` flag.

```bash
ch --debug
```

Note that the debug output will not be printed if you use the `--dry-run` flag.

## Tech Stack

-   [Rust](https://www.rust-lang.org/)
-   [inquire](https://github.com/mikaelmello/inquire)
