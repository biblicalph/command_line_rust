use clap::Parser;
use cli::FindrArgs;
use error_utils::{ProgramError, ProgramErrorParams};
use std::error;
use walkdir::{DirEntry, WalkDir};

mod cli;

type ProgramResult<'a> = anyhow::Result<(), ProgramError<'a>>;

pub fn run<'b>() -> ProgramResult<'b> {
    let args = FindrArgs::parse();
    find_matches(&args)?;
    Ok(())
}

fn find_matches<'a>(args: &'a FindrArgs) -> ProgramResult<'static> {
    for p in &args.paths {
        for entry in WalkDir::new(p).into_iter().filter(|e| {
            e.is_err()
                || e.as_ref()
                    .is_ok_and(|f| is_type_match(&f, args) && is_name_match(&f, args))
        }) {
            match entry {
                Ok(entry) => println!("{}", entry.path().display()),
                Err(e) => eprintln!("{}", translate_error(Box::new(e), p)),
            }
        }
    }
    Ok(())
}

fn is_type_match(entry: &DirEntry, args: &FindrArgs) -> bool {
    args.file_types.len() == 0
        || args
            .file_types
            .iter()
            .any(|f_type| f_type.is_type(&entry.file_type()))
}

fn is_name_match(entry: &DirEntry, args: &FindrArgs) -> bool {
    args.names.len() == 0
        || args
            .names
            .iter()
            .any(|n| n.is_match(entry.path().to_str().unwrap()))
}

fn translate_error(e: Box<dyn error::Error>, pathname: &str) -> ProgramError {
    let params = ProgramErrorParams::new()
        .pathname(pathname)
        .program("findr")
        .build()
        .expect("builds program error params");

    (e, params).into()
}
