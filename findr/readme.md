# Findr

Partial implementation of [find](https://en.wikipedia.org/wiki/Find_(Unix)) utility in Rust.

## Project goals

The program will locate files, directories, or links in one or more directories having names that match one or more regular expressions, or patterns of text.

### Implemented functionality

The following functionality is implemented by this program:

- Search for files, directories and symbolic links within the current directory or a list of directories. Multiple `--type` parameters implies an `or` operation
- Search for files, directories and symbolic links matching a given name regex, case sensitive. Multiple `--name` parameters implies an `or` operation
- Limit the depth of traversal using the `--maxdepth` and `--mindepth` flags.

To learn more, clone the repository and run the `-h` command of the program.

### Running the program

To run with [cargo](https://github.com/rust-lang/cargo) - `cargo run -- -h`
Via the program binary - `./<program_name> -h`

### Learning objectives

- Use regular expressions
- Use enumerated types
- Use external crates
- Use iterator methods
- Compile code conditionally for a specific platform
