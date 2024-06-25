use std::{error::Error, io};
use clap::Parser;
use cli::FindrArgs;
use predicates::path;
use walkdir::WalkDir;
use error_utils::{ProgramError, ProgramErrorParams};

mod cli;

type ProgramResult<'a> = anyhow::Result<(), ProgramError<'a>>;

pub fn run<'b>() -> ProgramResult<'b> {
    let args = FindrArgs::parse();
    find_matches(&args)?;
    Ok(())
}

fn find_matches<'a>(args: &'a FindrArgs) -> ProgramResult<'static> {
    for p in &args.paths {
        for entry in WalkDir::new(p).into_iter() {
          match entry {
            Ok(entry) => println!("{}", entry.path().display()),
            Err(e) => eprintln!("{}", translate_error(e, p))
          }
        }
    }
    Ok(())
}

fn translate_error(e: walkdir::Error, pathname: &str) -> ProgramError {
    let params = ProgramErrorParams::new()
      .pathname(pathname)
      .program("findr")
      .build()
      .expect("builds program error params");


    (e, params).into()
}
