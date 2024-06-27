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
    args.paths.iter().for_each(|p| process_path(p, args));
    Ok(())
}

fn process_path(p: &str, args: &FindrArgs) {
    let type_filter = |entry: &DirEntry| -> bool {
        args.file_types.is_empty()
            || args
                .file_types
                .iter()
                .any(|f_type| f_type.is_type(&entry.file_type()))
    };
    let name_filter = |entry: &DirEntry| -> bool {
        args.names.is_empty()
            || args
                .names
                .iter()
                .any(|name| name.is_match(&entry.path().to_string_lossy()))
    };

    let out = WalkDir::new(p)
        .into_iter()
        .filter_map(|res| match res {
            Err(e) => {
                eprintln!("{}", translate_error(Box::new(e), p));
                None
            }
            Ok(entry) => Some(entry),
        })
        .filter(type_filter)
        .filter(name_filter)
        .map(|entry| entry.path().display().to_string())
        .collect::<Vec<_>>()
        .join("\n");

    println!("{out}");
}

fn translate_error(e: Box<dyn error::Error>, pathname: &str) -> ProgramError {
    let params = ProgramErrorParams::new()
        .pathname(pathname)
        .program("findr")
        .build()
        .expect("builds program error params");

    (e, params).into()
}
