use clap::{builder::PossibleValue, Parser, ValueEnum};
use regex::Regex;
use std::fs::FileType as LibFileType;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
/// Partial implementation of find program in Rust
pub struct FindrArgs {
    /// Paths to search
    #[arg(default_value("."))]
    pub paths: Vec<String>,
    /// Match file types ending with the specified name pattern
    #[arg(short('n'),long("name"),value_name("NAME"),num_args(0..))]
    pub names: Vec<Regex>,
    /// The file type to match
    #[arg(short('t'),long("type"),value_name("TYPE"),num_args(0..))]
    pub file_types: Vec<FileType>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FileType {
    /// Directory
    Dir,
    /// File
    File,
    /// Symbolic link
    Link,
}

impl ValueEnum for FileType {
    fn value_variants<'a>() -> &'a [Self] {
        &[FileType::File, FileType::Dir, FileType::Link]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        let val = match self {
            FileType::Dir => PossibleValue::new("d"),
            FileType::File => PossibleValue::new("f"),
            FileType::Link => PossibleValue::new("l"),
        };
        Some(val)
    }
}

impl FileType {
    pub fn is_type(&self, file_type: &LibFileType) -> bool {
        match self {
            Self::Dir => file_type.is_dir(),
            Self::File => file_type.is_file(),
            Self::Link => file_type.is_symlink(),
        }
    }
}
