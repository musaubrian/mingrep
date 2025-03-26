use std::{error::Error, fs};

pub fn run(config: MinConfig) -> Result<(), Box<dyn Error>> {
    let _file_contents = match fs::read_to_string(&config.file_path) {
        Ok(contents) => contents,
        Err(err) => return Err(Box::new(err)),
    };
    println!(
        "File => {}\nQuery => {}\n---\n{_file_contents}\n---",
        config.query, config.file_path
    );
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
