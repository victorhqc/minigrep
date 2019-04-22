use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tesst {
    use super::*;

    #[test]
    fn it_creates_a_new_config() {
        let query = String::from("Foo");
        let filename = String::from("bar.txt");

        let args = [
            String::from("some/binary"),
            query.clone(),
            filename.clone(),
        ];
        let result = Config::new(&args).unwrap();

        assert_eq!(result.query, query);

        assert_eq!(result.filename, filename)
    }

    #[test]
    fn it_fails_when_not_enough_arguments() -> Result<(), String> {
        let query = String::from("Foo");

        let args = [
            String::from("some/binary"),
            query.clone(),
        ];

        if let Err(e) = Config::new(&args) {
            assert_eq!(e, "Not enough arguments.");
            Ok(())
        } else {
            Err(String::from("Config::new() does not fail when missing arguments"))
        }
    }
}
