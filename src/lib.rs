use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok (Config {query, filename})
    }
} 

pub fn run(config: Config) -> Result<(), Box<Error>>{
    // ファイルが見つかりませんでした
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents){
        println!("{}", line);
    }

    // テキストは\n{}です
    //println!("With text:\n{}", contents);

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

pub fn extension_identifier(file_name: &str) -> Option<&str>{
    
    let mut results = Vec::new();

    
    
    if !(file_name.contains(".")){
        results.push("no extension");
    }else{
        results = file_name.split('.').collect();
    }
    

    results.pop()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let file = "dir1/dir1/FileName.ext";
      
        match extension_identifier(file){
            Some(x) => assert_eq!(x, "ext"),
            None => println!("不正なファイル名"),
        }
    }
}