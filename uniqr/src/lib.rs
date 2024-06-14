use anyhow;
use clap::Parser;
use cli_args::Uniqr;
use matcher::UniqMatcher;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

mod cli_args;
mod matcher;

pub fn run() -> anyhow::Result<()> {
    let args = Uniqr::parse();

    let mut handle = open(&args.input_file)?;
    let res = UniqMatcher::from_reader(&mut handle, &args)?;

    match &args.output_file {
        Some(filename) => write_output(filename, &res.to_string())?,
        None => print!("{res}"),
    }

    Ok(())
}

/// Open the file or stdin
fn open(filename: &str) -> anyhow::Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin().lock()))),
        file => Ok(Box::new(BufReader::new(File::open(file)?))),
    }
}

fn write_output(filename: &str, output: &str) -> io::Result<()> {
    fs::write(filename, output)
}
