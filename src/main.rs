extern crate file_reader;
use std::env;
//use std::fs::File;
//use std::io::prelude::*;
use std::process;
//use std::error::Error;

use file_reader::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // {}を探しています
    println!("Searching for {}", config.query);
    // {}というファイルの中
    println!("In file {}", config.filename);

    if let Err(e) = file_reader::run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
    
}

/*fn run(config: Config) -> Result<(), Box<Error>>{
    // ファイルが見つかりませんでした
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // テキストは\n{}です
    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok (Config {query, filename})
    }
}    */

