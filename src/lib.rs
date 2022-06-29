use std::error;
use std::fs::File;
use std::collections::VecDeque;
use std::io::{prelude::*};
use plotters::prelude::*;


//To do:　簡単なプロット作成

pub struct Config {
    pub query: String,
    pub filename: String,
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

    /*{
        let mut i:i32 = 1;
    
        for opt_energy in scf{
            println!("{}: {:.9}",i, opt_energy);
            i+=1;
        }
    }*/

    draww_scf_changes(scf);

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

pub fn draww_scf_changes(scf_ener: Vec::<f64>)-> Result<(), Box<dyn error::Error>>{
   // 描画先をBackendとして指定。ここでは画像に出力するためBitMapBackend
    let root = BitMapBackend::new("chart.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut scf_cycle_num: i32;
    let mut scf_max_ener: f64 = scf_ener[0];
    let mut scf_min_ener: f64 = scf_ener[0];

    scf_cycle_num = scf_ener.len() as i32;

    
    for opt_ener in scf_ener.clone() {
        if scf_max_ener < opt_ener{
            scf_max_ener = opt_ener;
        }

        if scf_min_ener > opt_ener{
            scf_min_ener = opt_ener;
        }
    }

    
    // グラフの軸の設定など


    let mut chart = ChartBuilder::on(&root)
        .caption("y=x^2", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(60)
        .build_cartesian_2d(0i32..scf_cycle_num as i32, scf_min_ener as f64..scf_max_ener as f64).unwrap();

    chart.configure_mesh().draw().unwrap();

    // データの描画。(x, y)のイテレータとしてデータ点を渡す
    chart.draw_series(PointSeries::of_element(
        (0..scf_cycle_num).map(|x| ((x+1) , scf_ener[x as usize])),
        5,
        &RED,
        &|c, s, st| {
            return EmptyElement::at(c)    // We want to construct a composed element on-the-fly
            + Circle::new((0,0),s,st.filled()) // At this point, the new pixel coordinate is established
        },
    )).unwrap();

    chart.draw_series(LineSeries::new(
        (0..scf_cycle_num).map(|x| ((x+1) , scf_ener[x as usize])),
        &RED,
    )).unwrap();




    Ok(())
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