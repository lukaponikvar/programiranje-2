use std::{env, process};
use _mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = _mini_grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

