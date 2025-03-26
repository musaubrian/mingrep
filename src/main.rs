use std::{env, fs};

macro_rules! err {
    ($val:expr) => {
        eprintln!("Error: {}", $val);
    };
}

fn main() {
    let mut args = env::args();
    let _program = args.next();

    let program_args = args.collect::<Vec<String>>();
    let query = match program_args.first() {
        Some(q) => q,
        None => {
            err!("Missing Query");
            std::process::exit(69);
        }
    };

    let file_path = match program_args.get(1) {
        Some(file) => file,
        None => {
            err!("Missing File");
            std::process::exit(69);
        }
    };
    let _file_contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(err) => {
            err!(err);
            std::process::exit(69);
        }
    };
    println!("File => {file_path}\nQuery => {query}\n---\n{_file_contents}\n---");
}
