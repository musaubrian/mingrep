use std::{env, process};

use mingrep::{MinConfig, run};

macro_rules! err {
    ($val:expr) => {
        eprintln!("Error: {}", $val);
    };
}

fn main() {
    let mut args = env::args().collect::<Vec<String>>();
    let mc = MinConfig::new(&mut args).unwrap_or_else(|err| {
        err!(err);
        process::exit(69);
    });

    if let Err(e) = run(mc) {
        err!(e);
        process::exit(69);
    };
}
