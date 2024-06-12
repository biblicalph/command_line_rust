use anyhow;
use arguments::Headr;
use clap::Parser;
use result::HeadrResult;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod arguments;
mod reader;
mod result;

const PRG_NAME: &str = "headr";

pub fn run() -> anyhow::Result<HeadrResult> {
    let headr = Headr::parse();
    let mut res = HeadrResult::new();

    let mut append_empty_line = false;
    for file in &headr.files {
        match open(file) {
            Err(err) => res.add_error(file, translate_open_error(err)),
            Ok(mut reader) => {
                if append_empty_line {
                    res.add_newline();
                }
                if headr.read_bytes() {
                    match reader::read_bytes(
                        file,
                        reader,
                        headr.bytes.expect("should have bytes value"),
                    ) {
                        Ok(val) => {
                            res.add_outputs(&val);
                            append_empty_line = true;
                        }
                        Err(e) => {
                            res.add_error(file, e);
                            append_empty_line = false;
                        }
                    }
                } else {
                    res.add_outputs(&reader::read_lines(file, &mut reader, headr.lines));
                    append_empty_line = true;
                }
            }
        }
    }

    Ok(res)
}

/// Opens a file or stdin for reading
fn open(filename: &str) -> anyhow::Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin().lock()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

/// Translates an IO error into a friendly format
fn translate_open_error(err: anyhow::Error) -> anyhow::Error {
    match err.downcast_ref::<io::Error>() {
        Some(io_err) => match io_err.kind() {
            io::ErrorKind::NotFound => {
                anyhow::Error::msg("File or directory not found".to_string())
            }
            io::ErrorKind::PermissionDenied => anyhow::Error::msg("Permission denied".to_string()),
            _ => anyhow::Error::msg(io_err.to_string()),
        },
        _ => anyhow::Error::msg(err.to_string()),
    }
}
