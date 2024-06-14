use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
/// Partial implementation of uniq utility in rust
pub struct Uniqr {
    /// The input file or standard input data to filter
    #[arg(default_value = "-")]
    pub input_file: String,
    /// The output file to write to or standard out if omitted
    pub output_file: Option<String>,
    /// Prefix lines by the number of occurrences
    #[arg(short('c'), long("count"))]
    pub count_occurrences: bool,
    /// Ignore differences in case when comparing
    #[arg(short('i'), long("ignore-case"))]
    pub ignore_case: bool,
    /// Output a single copy of each line that is repeated in the input
    #[arg(short('d'), long("repeated"), conflicts_with("unique_only"))]
    pub duplicates_only: bool,
    /// Only output lines that are not repeated in the input
    #[arg(short('u'), long("unique"))]
    pub unique_only: bool,
}

impl Uniqr {
    #[cfg(test)]
    pub fn new() -> Self {
        Self {
            input_file: "tests/in/in.txt".to_string(),
            output_file: None,
            count_occurrences: false,
            ignore_case: false,
            duplicates_only: false,
            unique_only: false,
        }
    }
}
