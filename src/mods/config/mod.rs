extern crate regex;

use regex::Regex;

pub struct Config {
    pub url: String,
    pub base: String,
    pub filename: String,
    pub start_segment: String,
    pub retry: u32
}

impl AsRef<Config> for Config {
    #[inline]
    fn as_ref(&self) -> &Config {
        self
    }
}

impl Config {
    pub fn parse(args: &[String]) -> Result<Config, &'static str> {
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


        let start_segment = if args.len() > 2 {
            args[2].clone()
        } else {
            "".to_string()
        };

        let retry = if args.len() > 3 {
            args[3].to_string().parse::<u32>().unwrap()
        } else {
            8
        };


        Ok(Config {
            url: args[1].clone(),
            base: caps["base"].to_string(),
            filename: caps["filename"].to_string(),
            start_segment: start_segment,
            retry: retry
        })
    }
}