extern crate hyper;
extern crate regex;

use std::io;
use regex::Regex;
use std::io::{Write, BufRead, BufReader};
use utils::config::Config;
use std::fs::OpenOptions;
use std::time::Duration;


pub struct Segments {
    pub segments: Vec<String>
}

impl Segments {
    pub fn get(config: &Config) -> Result<Vec<String>, &'static str> {
        let url = config.url.clone();
        let retry = config.retry;

        let mut client = hyper::Client::new();
        client.set_read_timeout(Some(Duration::new(60, 0)));

        //2. 获取文件分片信息
        let mut content = Vec::<u8>::new();
        for i in 0..retry {
            let mut headers = hyper::header::Headers::new();
            headers.append_raw("User-Agent", "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:52.0) Gecko/20100101 Firefox/52.0".as_bytes().to_owned());
            headers.append_raw("Referer", "http://www.qqqq25.com/swf/HLSplayer.swf?v=1.5".as_bytes().to_owned());

            let result = client.get(url.as_str())
                .headers(headers)
                .send();
            if !result.is_ok() {
                println!("获取下载文件分片失败，重试次数{}次!", i + 1);
                if i == retry - 1 {
                    return Err("获取下载文件分片失败!");
                } else {
                    continue;
                }
            } else {
                let mut res = result.unwrap();
                if !res.status.is_success() {
                    println!("获取下载文件分片失败，重试第{}次!", i + 1);
                    if i == retry - 1 {
                        println!("获取下载文件分片数据时重试{}次仍失败，退出!", retry);
                        return Err("获取下载文件分片失败!");
                    } else {
                        continue;
                    }
                }
                io::copy(&mut res, &mut content).unwrap();
                println!("获取下载文件分片信息完成!");
                break;
            }
        }

        let re = Regex::new(r"#\w+").unwrap();
        let reader = BufReader::new(content.as_slice());
        let mut segments = vec![];
        for line in reader.lines() {
            let segment = line.unwrap();
            if !re.is_match(segment.as_str()) {
                segments.push(segment);
            }
        }
        Ok(segments)
    }

    pub fn download(segments: Vec<String>, config: &Config) -> Result<&'static str, &'static str> {
        let mut skip = true;
        let base = config.base.clone();
        let filename = config.filename.clone();

        let mut client = hyper::Client::new();
        client.set_read_timeout(Some(Duration::new(60, 0)));

        // 创建Or打开文件
        let mut file_path = String::new();
        file_path.push_str("./");
        file_path.push_str(filename.as_str());
        file_path.push_str(".flv");
        let mut file = OpenOptions::new().write(true).create(true).truncate(config.start_segment.len() < 1).open(file_path).expect("创建文件失败！");

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

            for i in 0..config.retry {
                let mut headers = hyper::header::Headers::new();
                headers.append_raw("User-Agent", "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:52.0) Gecko/20100101 Firefox/52.0".as_bytes().to_owned());
                headers.append_raw("Referer", "http://www.qqqq25.com/swf/HLSplayer.swf?v=1.5".as_bytes().to_owned());

                let result = client.get(segment_url.as_str())
                    .headers(headers)
                    .send();
                if !result.is_ok() {
                    println!("下载分片({})出错，重试第{}次!", segment, i + 1);
                    if i == config.retry - 1 {
                        println!("下载分片({})的数据时重试{}次仍失败，退出!", segment, config.retry);
                        return Err("下载分片失败!");
                    } else {
                        continue;
                    }
                } else {
                    let mut res = result.unwrap();
                    //http status is fail
                    if !res.status.is_success() {
                        println!("下载分片({})出错，重试第{}次!", segment, i + 1);
                        if i == config.retry - 1 {
                            println!("下载分片({})的数据时重试{}次仍失败，状态码:{}，退出!", segment, config.retry, res.status);
                            return Err("下载分片失败!");
                        } else {
                            continue;
                        }
                    } else {
                        //4. 保存文件
                        let mut content = Vec::<u8>::new();
                        match io::copy(&mut res, &mut content) {
                            Ok(_) => {}
                            Err(_) => { return Err("读取下载内容失败!"); }
                        }
                        match file.write(content.as_slice()) {
                            Ok(_) => {
                                println!("文件({})的分片({})下载完成!", filename, segment);
                                break;
                            }
                            Err(_) => { return Err("分片文件保存失败!"); }
                        }
                    }
                }
            }
        }
        Ok("下载完成")
    }
}