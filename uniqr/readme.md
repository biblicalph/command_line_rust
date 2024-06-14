# WCR (word count, rust version)

This application is an implementation of the word count (wc) program in Rust for pedagogical purposes.
For more information about `wc`, run `man wc` from a command line.

The implementation is deliberately memory inefficient; the in-memory vector will grow with the file size. A more efficient implementation will simply output values as comparisons are made instead of buffering them. The program was implemented how it was to take advantage of some additional Rust features for learning purposes.

## Usage

### Using cargo

1. Run `cd <project_directory>`, where `<project_directory>` is the path to the project directory
2. Run `cargo run -- -h` for usage information

### Via the binary

Run `./<binary> -h` where `<binary>` is the built binary

## Implementation notes

The program implementation is more complex than it ought to be; the complexity arose mainly from my desire to try several language features including:

- Implementing traits: [PartialEq](https://doc.rust-lang.org/nightly/std/cmp/trait.PartialEq.html), [Display](https://doc.rust-lang.org/nightly/std/fmt/trait.Display.html) and [Add](https://doc.rust-lang.org/nightly/std/ops/trait.Add.html)
- Create module for unit tests
- Fake a file handle for unit testing
- Use generics in method implementation, etc