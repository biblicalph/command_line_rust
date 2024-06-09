use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about=None)]
/// Rust implementation of the wordcount (wc) program
pub struct Wcr {
    /// list of files or command line inputs
    #[arg(default_value("-"))]
    pub files: Vec<String>,
    /// Show the character count
    #[arg(short('c'), long("bytes"), conflicts_with("show_char_count"))]
    pub show_byte_count: bool,
    /// Show the character count
    #[arg(short('m'), long("chars"))]
    pub show_char_count: bool,
    /// Show the character count
    #[arg(short('l'), long("lines"))]
    pub show_line_count: bool,
    /// Show the character count
    #[arg(short('w'), long("words"))]
    pub show_word_count: bool,
}

impl Wcr {
    pub fn set_defaults(mut self) -> Self {
        if [
            self.show_byte_count,
            self.show_char_count,
            self.show_line_count,
            self.show_word_count,
        ]
        .iter()
        .all(|v| !v)
        {
            self.show_byte_count = true;
            self.show_line_count = true;
            self.show_word_count = true;
        }
        self
    }
}
