use crate::Wcr;
use std::fmt;
use std::ops::Add;

pub struct Result {
    items: Vec<ResultItem>,
}

impl Result {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn print(&self) {
        for item in &self.items {
            match item {
                ResultItem::Data(_) => println!("{}", item),
                ResultItem::Err { .. } => eprintln!("{}", item),
            }
        }
    }

    pub fn add_item(&mut self, item: ResultItem) {
        self.items.push(item);
    }

    pub fn add_totals(&mut self, totals: ResultItem) {
        if self.items.len() > 1 {
            self.items.push(totals);
        }
    }
}

#[derive(Clone, Debug)]
pub enum ResultItem {
    Data(FileCount),
    Err { filename: String, msg: String },
}
impl fmt::Display for ResultItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Data(item) => {
                write!(f, "{}", item)
            }
            Self::Err { filename, msg } => {
                write!(f, "wcr: {}: {}", filename, msg)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FileCount {
    filename: String,
    byte_count: Option<usize>,
    char_count: Option<usize>,
    line_count: Option<usize>,
    word_count: Option<usize>,
}
impl FileCount {
    pub fn new(filename: &str, args: &Wcr) -> Self {
        Self {
            filename: filename.to_string(),
            byte_count: Self::value_or_default(args.show_byte_count),
            char_count: Self::value_or_default(args.show_char_count),
            line_count: Self::value_or_default(args.show_line_count),
            word_count: Self::value_or_default(args.show_word_count),
        }
    }
    #[cfg(test)]
    pub fn with_counts(
        filename: &str,
        byte_count: Option<usize>,
        char_count: Option<usize>,
        line_count: Option<usize>,
        word_count: Option<usize>,
    ) -> Self {
        Self {
            filename: filename.to_string(),
            byte_count,
            char_count,
            line_count,
            word_count,
        }
    }
    fn value_or_default(show_count: bool) -> Option<usize> {
        if show_count {
            Some(0)
        } else {
            None
        }
    }
    fn increment_count(option: &mut Option<usize>, count: usize) {
        *option = Some(option.unwrap_or(0) + count);
    }
    fn add_counts(first_count: Option<usize>, second_count: Option<usize>) -> Option<usize> {
        if let Some(count) = second_count {
            Some(first_count.unwrap_or(0) + count)
        } else {
            first_count
        }
    }
    pub fn increment_byte_count(&mut self, count: usize) {
        Self::increment_count(&mut self.byte_count, count);
    }
    pub fn increment_char_count(&mut self, count: usize) {
        Self::increment_count(&mut self.char_count, count);
    }
    pub fn increment_line_count(&mut self, count: usize) {
        Self::increment_count(&mut self.line_count, count);
    }
    pub fn increment_word_count(&mut self, count: usize) {
        Self::increment_count(&mut self.word_count, count);
    }
}
impl fmt::Display for FileCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::new();
        if self.line_count.is_some() {
            output.push_str(&format!("{:>5}", self.line_count.unwrap()));
        }
        if self.word_count.is_some() {
            output.push_str(&format!("{:>5}", self.word_count.unwrap()));
        }
        // byte count and char count are mutually exclusive
        if self.byte_count.is_some() {
            output.push_str(&format!("{:>5}", self.byte_count.unwrap()));
        }
        if self.char_count.is_some() {
            output.push_str(&format!("{:>5}", self.char_count.unwrap()));
        }
        write!(f, "{} {}", output, self.filename)
    }
}
impl Add for FileCount {
    type Output = FileCount;

    fn add(self, other: Self) -> Self {
        Self {
            filename: self.filename,
            byte_count: Self::add_counts(self.byte_count, other.byte_count),
            char_count: Self::add_counts(self.char_count, other.char_count),
            line_count: Self::add_counts(self.line_count, other.line_count),
            word_count: Self::add_counts(self.word_count, other.word_count),
        }
    }
}