use std::{env, process};

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    // unwrap_or_else method
    // run(config).unwrap_or_else(|err| {
    //     println!("Application error: {}", err);
    //     process::exit(1);
    // })

    // if let method
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };

    //  IGNORE_CASE=1 cargo run -- to poem.txt
}
