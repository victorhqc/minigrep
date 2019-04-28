pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    /// Create Configuration for minigrep.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::env;
    /// use minigrep::Config;
    ///
    /// Config::new(env::args(), false);
    /// ```
    pub fn new<T>(mut args: T, case_sensitive: bool) -> Result<Config, &'static str>
    where T: Iterator<Item = String> {
        // We Discard the first argument.
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

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

        let args = vec![
            String::from("some/binary"),
            query.clone(),
            filename.clone(),
        ];

        let result = Config::new(args.into_iter(), false).unwrap();

        assert_eq!(result.query, query);
        assert_eq!(result.filename, filename)
    }

    #[test]
    fn case_sensitive() {
        let query = String::from("Foo");
        let filename = String::from("bar.txt");

        let args = vec![
            String::from("some/binary"),
            query.clone(),
            filename.clone(),
        ];
        let result = Config::new(args.into_iter(), true).unwrap();

        assert!(result.case_sensitive);
    }

    #[test]
    fn case_insensitive() {
        let query = String::from("Foo");
        let filename = String::from("bar.txt");

        let args = vec![
            String::from("some/binary"),
            query.clone(),
            filename.clone(),
        ];
        let result = Config::new(args.into_iter(), false).unwrap();

        assert!(!result.case_sensitive);
    }

    #[test]
    fn fails_when_no_query() -> Result<(), String> {
        let args = vec![
            String::from("some/binary"),
        ];

        if let Err(e) = Config::new(args.into_iter(), true) {
            assert_eq!(e, "Didn't get a query string");
            Ok(())
        } else {
            Err(String::from("Config::new() does not fail when missing arguments"))
        }
    }

    #[test]
    fn fails_when_no_file() -> Result<(), String> {
        let query = String::from("Foo");

        let args = vec![
            String::from("some/binary"),
            query.clone(),
        ];

        if let Err(e) = Config::new(args.into_iter(), true) {
            assert_eq!(e, "Didn't get a file name");
            Ok(())
        } else {
            Err(String::from("Config::new() does not fail when missing arguments"))
        }
    }
}
