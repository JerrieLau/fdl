extern crate reqwest;
extern crate regex;

use std::io;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::env;
use std::process;
use regex::Regex;

pub struct Config {
    pub url: String,
    pub base: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("请在命令行后添加要访问的URL地地址!");
        }
        let url_str = args[1].clone();
        let url = url_str.as_str();
        let re = Regex::new(r"(?i)(?P<base>http://.+/)(?P<filename>\w+)\.m3u8").unwrap();

        if !re.is_match(url) {
            return Err("输入的URL不满足格式，格式：http://170.178.165.100:8011/B0122/d145/d145.m3u8");
        }

        let caps = match re.captures(url) {
            Some(t) => t,
            None => {
                return Err("输入的URL不满足格式，格式：http://170.178.165.100:8011/B0122/d145/d145.m3u8");
            }
        };

        Ok(Config {
            url: args[1].clone(),
            base: caps["base"].to_string(),
            filename: caps["filename"].to_string()
        })
    }
}

fn main() {
    //0.解析输入参数
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("解析输入参数出错，详情：{}", e);
        process::exit(1);
    });

    //1. 从下载地址中拆解文件名
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
    let mut file_path = String::new();
    file_path.push_str("./");
    file_path.push_str(filename.as_str());
    file_path.push_str(".flv");

    let mut file = File::create(file_path).expect("创建文件失败！");
    let mut segments = vec!();
    for line in reader.lines() {
        let segment = line.unwrap();
        if !re.is_match(segment.as_str()) {
            segments.push(segment);
        }
    }

    //3. 获取每个分片数据
    for segment in segments {
        let mut segment_url = base.clone();
        segment_url.push_str(segment.as_str());
        let mut res = reqwest::get(segment_url.as_str()).unwrap();
        if !res.status().is_success() {
            println!("下载分片({})的数据时出错，状态码：{}", segment, res.status());
            process::exit(1);
        }

        //4. 保存文件
        io::copy(&mut res, &mut file).unwrap();
        println!("文件({})的分片({})下载完成，", filename, segment);
    }
    //    println!("content is {}", );
}
