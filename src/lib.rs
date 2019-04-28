use std::error::Error;
use std::fs;

mod config;
pub use config::Config;

/// Run minigrep
///
/// # Examples
///
/// ```
/// use minigrep::Config;
/// use std::process;
///
/// let args = vec![
///     String::from("some/binary"),
///     String::from("body"),
///     String::from("poem.txt"),
/// ];
///
/// let config = Config::new(args.into_iter(), false).unwrap_or_else(|err| {
///     eprintln!("Problem parsing arguments: {}", err);
///     process::exit(1);
/// });
/// if let Err(e) = minigrep::run(config) {
///     eprintln!("Application error: {}", e);
///
///     process::exit(1);
/// }
/// ```
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tesst {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents),
        );
    }
}
