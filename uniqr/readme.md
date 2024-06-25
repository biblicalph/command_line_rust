# Uniq (uniq, rust version)

This application is an implementation of the word count (wc) program in Rust for pedagogical purposes.
For more information about `uniq`, run `man uniq` from a command line.

The implementation is deliberately memory inefficient; the in-memory vector will grow with the file size. A more efficient implementation will simply output values as comparisons are made instead of buffering them. The program was implemented how it was to take advantage of some additional Rust features for learning purposes.

## Usage

### Using cargo

1. Run `cd <project_directory>`, where `<project_directory>` is the path to the project directory
2. Run `cargo run -- -h` for usage information

### Via the binary

Run `./<binary> -h` where `<binary>` is the built binary

## Implementation notes

Several language features were used in the implementation including:

- Iterators
- Closures
- Match guards
- write! macro, etc
