use crate::Uniqr;
use anyhow;
use std::fmt::{self, Display, Formatter};
use std::io::BufRead;

#[derive(Clone, Debug)]
pub struct UniqMatcher<'a> {
    matches: Vec<UniqMatchItem>,
    flags: &'a Uniqr,
}
impl<'a> UniqMatcher<'a> {
    pub fn from_reader(handle: &mut impl BufRead, flags: &'a Uniqr) -> anyhow::Result<Self> {
        let mut matches: Vec<UniqMatchItem> = Vec::new();
        let mut line = String::new();

        loop {
            if handle.read_line(&mut line)? == 0 {
                break;
            }
            if let Some(last_item) = matches.last_mut() {
                if last_item.is_line_match(&line, flags.ignore_case) {
                    last_item.increment_count();
                } else {
                    matches.push(UniqMatchItem::with_line(&line));
                }
            } else {
                matches.push(UniqMatchItem::with_line(&line));
            }
            line.clear();
        }

        Ok(Self { matches, flags })
    }
}
impl<'a> Display for UniqMatcher<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let match_str = self
            .matches
            .iter()
            .filter_map(|m| {
                // This section uses match guards.
                // See: https://doc.rust-lang.org/reference/expressions/match-expr.html#match-guards
                let msg = match self.flags {
                    _ if self.flags.count_occurrences => {
                        format!("{:>5} {}", m.count, m.line)
                    }
                    // use only first line for case insensitive match without counts
                    _ if self.flags.ignore_case => format!("{}", m.line),
                    _ => format!("{}", m.line),
                };

                match self.flags {
                    _ if self.flags.duplicates_only => m.is_duplicate().then(|| msg),
                    _ if self.flags.unique_only => m.is_unique().then(|| msg),
                    _ => Some(msg),
                }
            })
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{}", match_str)
    }
}

const NEWLINE_CHARS: &[char] = &['\r', '\n'];

#[derive(Clone, Debug, PartialEq)]
struct UniqMatchItem {
    line: String,
    count: u64,
}

impl UniqMatchItem {
    #[cfg(test)]
    fn new() -> Self {
        Self {
            line: "".to_string(),
            count: 0,
        }
    }
    fn with_line(line: &str) -> Self {
        Self {
            line: line.to_string(),
            count: 1,
        }
    }

    fn increment_count(&mut self) {
        self.count += 1;
    }

    fn is_line_match(&self, line: &str, ignore_case: bool) -> bool {
        let last_line = self.line.trim_end_matches(NEWLINE_CHARS);
        let line = line.trim_end_matches(NEWLINE_CHARS);

        if ignore_case {
            last_line.eq_ignore_ascii_case(line)
        } else {
            last_line.eq(line)
        }
    }

    fn is_unique(&self) -> bool {
        self.count == 1
    }

    fn is_duplicate(&self) -> bool {
        self.count > 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::io::Cursor;

    #[test]
    fn finds_uniq_matches() {
        for (desc, test_item) in build_find_unique_cases() {
            let mut reader = Cursor::new(test_item.input_text);
            let matcher = UniqMatcher::from_reader(&mut reader, &test_item.flags)
                .expect("should create matcher for '{desc}'");

            assert_eq!(matcher.to_string(), test_item.expected_output, "{desc}");
        }
    }

    #[test]
    fn is_line_match() {
        assert!(
            !UniqMatchItem::new().is_line_match("some", false),
            "when match has no lines"
        );
        assert!(
            UniqMatchItem::with_line("some\r\n").is_line_match("some", false),
            "when match has line with carriage returns"
        );
        assert!(
            !UniqMatchItem::with_line("some\r\n").is_line_match("Some", false),
            "when values have different cases"
        );
        assert!(
            UniqMatchItem::with_line("some\r\n").is_line_match("Some", true),
            "when values have different cases (ignore case)"
        );
        assert!(
            !UniqMatchItem::with_line("some\r\n").is_line_match("Something", true),
            "when values are different"
        );
    }

    struct FindUniqTestCase {
        flags: Uniqr,
        input_text: String,
        expected_output: String,
    }

    fn build_find_unique_cases() -> HashMap<&'static str, FindUniqTestCase> {
        let input_text = vec![
            "summer", " ", "summer", "SummeR", "came", "CAME", "Came ", "quite", "early", "quite",
            "QUITE\r", "quite\n",
        ]
        .join("\n");
        let mut cases = HashMap::new();
        cases.insert(
            "with no flags",
            FindUniqTestCase {
                flags: Uniqr::new(),
                input_text: input_text.clone(),
                expected_output: input_text.clone(),
            },
        );
        cases.insert(
            "with duplicates only flag",
            FindUniqTestCase {
                flags: || -> Uniqr {
                    let mut flags = Uniqr::new();
                    flags.duplicates_only = true;
                    flags
                }(),
                input_text: input_text.clone(),
                expected_output: "".to_string(),
            },
        );
        cases.insert(
            "with unique only flag",
            FindUniqTestCase {
                flags: || -> Uniqr {
                    let mut flags = Uniqr::new();
                    flags.unique_only = true;
                    flags
                }(),
                input_text: input_text.clone(),
                expected_output: input_text.clone(),
            },
        );
        cases.insert(
            "with counts flag",
            FindUniqTestCase {
                flags: || -> Uniqr {
                    let mut flags = Uniqr::new();
                    flags.count_occurrences = true;
                    flags
                }(),
                input_text: input_text.clone(),
                expected_output: vec![
                    format!("{:>5} summer", 1),
                    format!("{:>5} {}", 1, " "),
                    format!("{:>5} summer", 1),
                    format!("{:>5} SummeR", 1),
                    format!("{:>5} came", 1),
                    format!("{:>5} CAME", 1),
                    format!("{:>5} Came ", 1),
                    format!("{:>5} quite", 1),
                    format!("{:>5} early", 1),
                    format!("{:>5} quite", 1),
                    format!("{:>5} QUITE\r", 1),
                    format!("{:>5} quite\n", 1),
                ]
                .join("\n"),
            },
        );
        cases.insert(
            "with case insensitive flag",
            FindUniqTestCase {
                flags: || -> Uniqr {
                    let mut flags = Uniqr::new();
                    flags.ignore_case = true;
                    flags
                }(),
                input_text: input_text.clone(),
                expected_output: vec![
                    "summer", " ", "summer", "came", "Came ", "quite", "early", "quite\n",
                ]
                .join("\n"),
            },
        );
        cases.insert(
            "with case insensitive uniques",
            FindUniqTestCase {
                flags: || -> Uniqr {
                    let mut flags = Uniqr::new();
                    flags.ignore_case = true;
                    flags.unique_only = true;
                    flags
                }(),
                input_text: input_text.clone(),
                expected_output: vec!["summer", " ", "Came ", "quite", "early\n"].join("\n"),
            },
        );
        cases.insert(
            "with case insensitive duplicates",
            FindUniqTestCase {
                flags: || -> Uniqr {
                    let mut flags = Uniqr::new();
                    flags.ignore_case = true;
                    flags.duplicates_only = true;
                    flags
                }(),
                input_text: input_text.clone(),
                expected_output: vec!["summer", "came", "quite\n"].join("\n"),
            },
        );
        cases.insert(
            "with case insensitive counts",
            FindUniqTestCase {
                flags: || -> Uniqr {
                    let mut flags = Uniqr::new();
                    flags.count_occurrences = true;
                    flags.ignore_case = true;
                    flags
                }(),
                input_text: input_text.clone(),
                expected_output: vec![
                    format!("{:>5} summer", 1),
                    format!("{:>5} {}", 1, " "),
                    format!("{:>5} summer", 2),
                    format!("{:>5} came", 2),
                    format!("{:>5} Came ", 1),
                    format!("{:>5} quite", 1),
                    format!("{:>5} early", 1),
                    format!("{:>5} quite\n", 3),
                ]
                .join("\n"),
            },
        );
        cases.insert(
            "with case insensitive duplicate counts",
            FindUniqTestCase {
                flags: || -> Uniqr {
                    let mut flags = Uniqr::new();
                    flags.count_occurrences = true;
                    flags.ignore_case = true;
                    flags.duplicates_only = true;
                    flags
                }(),
                input_text: input_text.clone(),
                expected_output: vec![
                    format!("{:>5} summer", 2),
                    format!("{:>5} came", 2),
                    format!("{:>5} quite\n", 3),
                ]
                .join("\n"),
            },
        );
        cases.insert(
            "with case insensitive unique counts",
            FindUniqTestCase {
                flags: || -> Uniqr {
                    let mut flags = Uniqr::new();
                    flags.count_occurrences = true;
                    flags.ignore_case = true;
                    flags.unique_only = true;
                    flags
                }(),
                input_text: input_text.clone(),
                expected_output: vec![
                    format!("{:>5} summer", 1),
                    format!("{:>5} {}", 1, " "),
                    format!("{:>5} Came ", 1),
                    format!("{:>5} quite", 1),
                    format!("{:>5} early\n", 1),
                ]
                .join("\n"),
            },
        );

        cases
    }
}
