use std::{error::Error, fs};

pub fn run(config: MinConfig) -> Result<(), Box<dyn Error>> {
    let file_contents = match fs::read_to_string(&config.file_path) {
        Ok(contents) => contents,
        Err(err) => return Err(Box::new(err)),
    };
    for result in search(&config.query, &file_contents) {
        println!("{result}")
    }
    Ok(())
}

pub struct MinConfig {
    query: String,
    file_path: String,
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
        })
    }
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn test_search() {
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
";
        let query = "du";
        let search_results = search(query, contents);
        assert_eq!(vec!["safe, fast, productive."], search_results);
    }
}
