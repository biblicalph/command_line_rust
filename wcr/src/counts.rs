use crate::counts::result::{FileCount, Result as CountResult, ResultItem};
use crate::Wcr;
use anyhow;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod result;

/// compute the byte, char, line and word counts for all files or standard input items
pub fn compute(wcr: &Wcr) -> anyhow::Result<CountResult> {
    let mut res = CountResult::new();
    let mut totals = FileCount::new("total", wcr);

    for filename in &wcr.files {
        match open(filename) {
            Err(e) => res.add_item(ResultItem::Err {
                filename: filename.to_string(),
                msg: e.to_string(),
            }),
            Ok(mut handle) => {
                let file_info = get_counts(&wcr, filename, &mut handle)?;
                res.add_item(ResultItem::Data(file_info.clone()));
                totals = totals + file_info;
            }
        }
    }
    res.add_totals(ResultItem::Data(totals));
    Ok(res)
}

/// Open the file or stdin
fn open(filename: &str) -> anyhow::Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin().lock()))),
        file => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}

/// compute the counts for a given file or standard input item
fn get_counts(wcr: &Wcr, filename: &str, reader: &mut impl BufRead) -> anyhow::Result<FileCount> {
    let mut file_info = FileCount::new(filename, wcr);

    let mut line = String::new();
    loop {
        let num_bytes = reader.read_line(&mut line)?;
        if num_bytes == 0 {
            break;
        }
        if wcr.show_byte_count {
            file_info.increment_byte_count(num_bytes);
        }
        if wcr.show_char_count {
            file_info.increment_char_count(line.chars().count());
        }
        if wcr.show_line_count {
            file_info.increment_line_count(1);
        }
        if wcr.show_word_count {
            file_info.increment_word_count(line.split_whitespace().count());
        }
        line.clear();
    }

    Ok(file_info)
}

#[cfg(test)]
mod tests {
    use crate::args::Wcr;
    use crate::counts::result::FileCount;
    use std::io::Cursor;

    const COUNT_TEXT: &str = "It all happened quickly, she said.\n Out of nowhere the agent retorted  in Chinese 闭嘴吧\r\n";

    #[test]
    fn count_all() {
        let mut wcr = create_args("tests/inputs/test.txt");
        wcr.show_byte_count = true;
        wcr.show_char_count = true;
        wcr.show_line_count = true;
        wcr.show_word_count = true;

        run_count_test(
            &wcr,
            COUNT_TEXT,
            FileCount::with_counts(
                "tests/inputs/test.txt",
                Some(93),
                Some(87),
                Some(2),
                Some(15),
            ),
            "count all",
        );
    }

    #[test]
    fn count_with_options() {
        let mut wcr = create_args("tests/inputs/test.txt");
        wcr.show_char_count = true;
        wcr.show_word_count = true;

        run_count_test(
            &wcr,
            COUNT_TEXT,
            FileCount::with_counts("tests/inputs/test.txt", None, Some(87), None, Some(15)),
            "count chars and words",
        );

        let mut wcr = create_args("tests/inputs/test.txt");
        wcr.show_line_count = true;
        run_count_test(
            &wcr,
            COUNT_TEXT,
            FileCount::with_counts("tests/inputs/test.txt", None, None, Some(2), None),
            "count chars and words",
        );
    }

    fn create_args(filename: &str) -> Wcr {
        return Wcr {
            files: vec![filename.to_string()],
            show_byte_count: false,
            show_char_count: false,
            show_line_count: false,
            show_word_count: false,
        };
    }

    fn run_count_test(wcr: &Wcr, text: &str, expected: FileCount, desc: &str) {
        let mut reader = Cursor::new(text);
        let filename = wcr.files.get(0).unwrap();
        let counts = super::get_counts(&wcr, filename, &mut reader);

        assert!(counts.is_ok(), "{}: returns ok", desc);
        let counts = counts.unwrap();
        assert_eq!(counts, expected, "{}: returns expected count", desc);
    }
}
