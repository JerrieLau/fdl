extern crate reqwest;
extern crate regex;

mod mods {
    pub mod config;
    //    pub mod segment;
}

use std::io;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::env;
use std::process;
use regex::Regex;
use mods::config::Config;


fn main() {
    //1.解析输入参数
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("解析输入参数出错，详情：{}", e);
        process::exit(1);
    });
    let url = config.url;
    let base = config.base;
    let filename = config.filename;

    //2. 获取文件分片信息
    let mut res = reqwest::get(url.as_str()).unwrap();
    if !res.status().is_success() {
        println!("获取下载文件分片信息出错，状态码：{}", res.status());
        process::exit(1);
    }
    let mut content = Vec::<u8>::new();
    io::copy(&mut res, &mut content).unwrap();
    let re = Regex::new(r"#\w+").unwrap();
    let reader = BufReader::new(content.as_slice());
    let mut segments = vec!();
    for line in reader.lines() {
        let segment = line.unwrap();
        if !re.is_match(segment.as_str()) {
            segments.push(segment);
        }
    }


    let mut file_path = String::new();
    file_path.push_str("./");
    file_path.push_str(filename.as_str());
    file_path.push_str(".flv");
    let mut file = File::create(file_path).expect("创建文件失败！");
    //3. 获取每个分片数据
    let mut skip = true;
    for segment in segments {
        //跳过一些分片
        if config.start_segment.len() > 0 && skip {
            if config.start_segment.eq(segment.as_str()) {
                skip = false;
            }
            continue;
        }

        let mut segment_url = base.clone();
        segment_url.push_str(segment.as_str());

        let retry = config.retry;
        for i in 0..retry {
            let mut res = reqwest::get(segment_url.as_str()).unwrap();
            if !res.status().is_success() {
                if i == retry - 1 {
                    println!("下载分片({})的数据时出错，状态码：{}", segment, res.status());
                    process::exit(1);
                } else {
                    continue;
                }
            }
            //4. 保存文件
            io::copy(&mut res, &mut file).unwrap();
            println!("文件({})的分片({})下载完成，", filename, segment);
        }
    }
    //    println!("content is {}", );
}
