# minigrep

A small `grep`-like command-line program written in Rust.

This project was created while working through **Chapter 12 — An I/O Project: Building a Command Line Program** from *The Rust Programming Language*, with a few additional features added along the way.

`minigrep` searches for matching lines in a file or from piped standard input.

## Features

- Search for text inside a file
- Read input from `stdin`
- Pipe output from other commands into `minigrep`
- Case-sensitive search by default
- Case-insensitive search with `-i`
- Error handling with `Result`
- Unit tests for search functionality
- No external dependencies

## Requirements

You need Rust and Cargo installed.

Check your installation with:

```bash
rustc --version
cargo --version
```

## Installation

Clone the repository:

```bash
git clone <repository-url>
cd minigrep
```

Build the project:

```bash
cargo build --release
```

The compiled binary will be available at:

```text
target/release/minigrep
```

You can also install it into your Cargo binary directory:

```bash
cargo install --path .
```

After that, you can run it directly:

```bash
minigrep <query> [filename]
```

## Usage

### Search inside a file

```bash
minigrep <query> <filename>
```

Example:

```bash
minigrep Rust poem.txt
```

This prints every line in `poem.txt` that contains `Rust`.

### Case-insensitive search

Use the `-i` flag:

```bash
minigrep -i rust poem.txt
```

This matches values such as:

```text
Rust
RUST
rust
RuSt
```

### Pipe input into minigrep

If no filename is provided, `minigrep` reads from standard input.

For example:

```bash
cat poem.txt | minigrep Rust
```

You can also pipe output from other commands:

```bash
ls | minigrep src
```

or:

```bash
ps aux | minigrep firefox
```

This makes `minigrep` usable as part of normal Unix pipelines.

## Running with Cargo

You can run the project without installing it:

```bash
cargo run -- Rust poem.txt
```

The `--` separates Cargo's arguments from the arguments passed to `minigrep`.

Case-insensitive search:

```bash
cargo run -- -i rust poem.txt
```

Using piped input:

```bash
cat poem.txt | cargo run -- rust
```

## Examples

Given a file:

```text
Rust:
safe, fast, productive.
Pick three.
Trust me.
```

Running:

```bash
minigrep duct poem.txt
```

outputs:

```text
safe, fast, productive.
```

A case-insensitive search:

```bash
minigrep -i rust poem.txt
```

can match:

```text
Rust:
Trust me.
```

## Running Tests

Run the test suite with:

```bash
cargo test
```

The project contains tests for both case-sensitive and case-insensitive searching.

## Project Structure

```text
minigrep/
├── src/
│   ├── main.rs
│   └── lib.rs
├── Cargo.toml
├── Cargo.lock
└── poem.txt
```

### `src/main.rs`

The binary entry point.

It:

- reads command-line arguments
- builds the configuration
- runs the application
- reports errors to `stderr`

### `src/lib.rs`

Contains the main application logic, including:

- `Config`
- argument parsing
- file and stdin reading
- case-sensitive search
- case-insensitive search
- unit tests

## How It Works

Arguments are parsed into a `Config`:

```rust
pub struct Config {
    pub query: String,
    pub filename: Option<String>,
    pub case_sensitive: bool,
}
```

If a filename is provided, `minigrep` reads from that file.

```text
minigrep rust poem.txt
             │
             ▼
          poem.txt
```

If no filename is provided, it reads from standard input instead:

```text
ls ──────► stdin ──────► minigrep src
```

The program then selects either the case-sensitive or case-insensitive search implementation and prints matching lines to standard output.

## What I Learned

This project was built as a learning exercise for several Rust concepts:

- command-line argument handling
- ownership and borrowing
- `String` and `&str`
- vectors
- `Option`
- `Result`
- pattern matching with `match`
- error propagation with `?`
- reading files
- reading from `stdin`
- writing to `stdout` and `stderr`
- lifetimes
- separating library and binary code
- unit testing
- working with Unix pipes

## Inspiration

This project is based on the **minigrep** project from Chapter 12 of *The Rust Programming Language*.

The original chapter builds a simplified version of the classic Unix `grep` utility as a way to practice Rust's command-line I/O, error handling, code organization, testing, and related concepts.

This repository extends that exercise with support for command-line flags and piped standard input.

## Disclaimer

This is a learning project and is not intended to replace `grep` or tools such as `ripgrep`.

It exists primarily to practice Rust and understand how command-line utilities work.