use std::error;
use std::fs::File;
use std::collections::VecDeque;
use std::io::{prelude::*};

//To do:　簡単なプロット作成

pub struct Config {
    pub query: String,
    pub filename: String,
}

pub enum Software{
    G09,
    G16,
}

impl Config {
    
    /*実行時のコマンドに与えられた引数の数を確認する。
    足りなければErrを返す。
    受け取り側でパニックさせる*/
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3{
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok (Config {query, filename})
    }
} 

/*メイン部分*/
pub fn run(config: Config) -> Result<(), Box<dyn error::Error>>{
    // ファイルが見つかりませんでした
    let mut f = File::open(config.filename)?;

    //ファイルの中身をいったん全部保存する
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;


    let mut numerical_scf_done_lines: VecDeque<f64>;

    let mut scf = Vec::<f64>::new();

    for line in search(&config.query, &contents){
        println!("{}", line);
        /*for numbers in line.matches(char::is_numeric).collect(){
            numerical_scf_done_lines.push(numbers);
        }*/
        numerical_scf_done_lines = line.split_whitespace().filter_map(|k| k.parse().ok()).collect::<VecDeque<f64>>();


        match numerical_scf_done_lines.pop_front() {
            Some(x) => scf.push(x),
            None => println!("Error: Loading SCF energy was failed")
        }

        /*for num in numerical_scf_done_lines {
            println!("{:.9}", num);
        }*/
    }

    {
        let mut i:i32 = 1;
    
        for opt_energy in scf{
            println!("{}: {:.9}",i, opt_energy);
            i+=1;
        }
    }


    // テキストは\n{}です
    //println!("With text:\n{}", contents);

    Ok(())
}

/*出力ファイルの最初の数行から、出力してきたソフトウェアを識別する
識別できなかった場合はエラーを返すのでパニックする。
パニックするときにファイル名を受け取り側で明記する。
ひとまずはハードコーディング */
pub fn identify_calculation_software(contents: &str) -> Result<Software, &'static str>{
    let contents_lines = contents.lines();
    for line in contents_lines{
        if line.contains("G09"){
            return Ok(Software::G09);
        }
        if line.contains("G16"){
            return Ok(Software::G16);
        }
    }
    Err("cannot identify software which output this file")
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

pub fn write_summary_to_file(){

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