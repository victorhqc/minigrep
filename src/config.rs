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
    fn case_sensitive() {
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
    fn case_insensitive() {
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
}
