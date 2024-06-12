use std::process;

use wcr;

fn main() {
    if let Err(e) = wcr::run() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
