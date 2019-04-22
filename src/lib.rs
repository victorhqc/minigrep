use std::error::Error;
use std::fs;

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
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String], case_sensitive: bool) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename, case_sensitive })
    }
}

#[cfg(test)]
mod tesst {
    use super::*;

    #[test]
    fn creates_a_new_config() {
        let query = String::from("Foo");
        let filename = String::from("bar.txt");

        let args = [
            String::from("some/binary"),
            query.clone(),
            filename.clone(),
        ];
        let result = Config::new(&args, false).unwrap();

        assert_eq!(result.query, query);
        assert_eq!(result.filename, filename)
    }

    #[test]
    fn config_is_case_sensitive() {
        let query = String::from("Foo");
        let filename = String::from("bar.txt");

        let args = [
            String::from("some/binary"),
            query.clone(),
            filename.clone(),
        ];
        let result = Config::new(&args, true).unwrap();

        assert!(result.case_sensitive);
    }

    #[test]
    fn config_is_case_insensitive() {
        let query = String::from("Foo");
        let filename = String::from("bar.txt");

        let args = [
            String::from("some/binary"),
            query.clone(),
            filename.clone(),
        ];
        let result = Config::new(&args, false).unwrap();

        assert!(!result.case_sensitive);
    }

    #[test]
    fn fails_when_not_enough_arguments() -> Result<(), String> {
        let query = String::from("Foo");

        let args = [
            String::from("some/binary"),
            query.clone(),
        ];

        if let Err(e) = Config::new(&args, true) {
            assert_eq!(e, "Not enough arguments.");
            Ok(())
        } else {
            Err(String::from("Config::new() does not fail when missing arguments"))
        }
    }

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
