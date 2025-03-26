use std::{env, error::Error, fs};

pub fn run(config: MinConfig) -> Result<(), Box<dyn Error>> {
    let file_contents = match fs::read_to_string(&config.file_path) {
        Ok(contents) => contents,
        Err(err) => return Err(Box::new(err)),
    };
    for result in search(&config.query, &file_contents, config.ignore_case) {
        println!("{result}")
    }
    Ok(())
}

pub struct MinConfig {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl MinConfig {
    pub fn new(args: &mut Vec<String>) -> Result<MinConfig, &'static str> {
        let mut args_iter = args.iter();
        let _program = args_iter.next();
        let query = match args_iter.next() {
            Some(q) => q,
            None => return Err("Missing Query"),
        };
        let file_path = match args_iter.next() {
            Some(f) => f,
            None => return Err("Missing File path"),
        };

        Ok(MinConfig {
            query: query.clone(),
            file_path: file_path.clone(),
            ignore_case: env::var("IGNORE_CASE").is_ok(),
        })
    }
}

fn search<'a>(query: &str, content: &'a str, ignore_case: bool) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| {
            if ignore_case {
                line.to_lowercase().contains(&query.to_lowercase())
            } else {
                line.contains(query)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
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
            search(query, contents, false)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, true));
    }
}
