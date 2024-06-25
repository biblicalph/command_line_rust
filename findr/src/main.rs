use findr::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("Terminating process due to error: {e}");
        std::process::exit(1);
    };
}
