//extern crate reqwest;
//extern crate regex;
//
//use std::io;
//use regex::Regex;
//use std::io::{BufRead, BufReader};
//
//pub struct Segments {
//    pub segments : Vec
//}
//
//impl Segments {
//    pub fn new(url: &str, filename: &str) -> Result<Vec, &'static str> {
//
//        //2. 获取文件分片信息
//        let mut res = reqwest::get(url).unwrap();
//        if !res.status().is_success() {
//            return Err("获取下载文件分片信息出错!");
//        }
//        let mut content = Vec::<u8>::new();
//        io::copy(&mut res, &mut content).unwrap();
//        let re = Regex::new(r"#\w+").unwrap();
//        let reader = BufReader::new(content.as_slice());
//        let mut segments = vec!();
//        for line in reader.lines() {
//            let segment = line.unwrap();
//            if !re.is_match(segment.as_str()) {
//                segments.push(segment);
//            }
//        }
//        Ok(segments)
//    }
//}