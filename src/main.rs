extern crate regex;

mod utils;

use std::env;
use std::process;
use utils::config::Config;
use utils::segment::Segments;

fn main() {
    //1.解析输入参数
    let args: Vec<String> = env::args().collect();
    let config = Config::parse(&args).unwrap_or_else(|e| {
        println!("解析输入参数出错，详情：{}!", e);
        process::exit(1);
    });

    //2. 获取文件分片信息
    let segments = Segments::get(config.as_ref()).unwrap_or_else(|_| {
        process::exit(1);
    });

    //3. 获取每个分片数据
    match Segments::download(segments, config.as_ref()) {
        Ok(_) => {}
        Err(_) => { process::exit(1); }
    }
}
