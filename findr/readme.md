# Findr

Partial implementation of [find](https://en.wikipedia.org/wiki/Find_(Unix)) utility in Rust.

## Project goals

The program will locate files, directories, or links in one or more directories having names that match one or more regular expressions, or patterns of text.

### Learning objectives

- Use regular expressions
- Use enumerated types
- Use external crates
- Use iterator methods
- Compile code conditionally for a specific platform

- Use a regular expression to find a pattern of text
- Create an enumerated type with an implementation
- Recursively search filepaths using the walkdir crate
- Use the Iterator::any function
- Chain multiple filter, map, and filter_map operations
- Compile code conditionally when on Windows or not
- Refactor code