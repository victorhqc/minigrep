use std::env;
use std::process;

use minigrep::config::Config;

fn main() {
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    let config = Config::new(env::args(), case_sensitive).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
