#![feature(proc_macro_hygiene)]

fn main() {
    supreme::run().unwrap_or_else(|err| {
        eprintln!("We found an error: {}", err);
        std::process::exit(1);
    });
}
