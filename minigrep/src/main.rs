use std::env;

use minigrep::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    minigrep::run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {}", err);
        std::process::exit(1);
    });
}