use std::process;

use headr::run;

fn main() {
    match run() {
        Ok(headr) => headr.print(),
        Err(e) => {
            eprintln!("Application error: {e}");
            process::exit(1);
        }
    };
}
