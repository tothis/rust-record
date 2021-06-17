use std::fs::File;
use std::io::Read;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub path: String,
    pub exclude_suffix: Vec<String>,
    pub exclude_prefix: Vec<String>,
}

pub fn read_config() -> Config {
    let path = "config.toml";
    let mut file = match File::open(path) {
        Ok(o) => o,
        Err(e) => panic!("读取文件失败 {}", e)
    };
    let mut str = String::new();
    match file.read_to_string(&mut str) {
        Ok(o) => o,
        Err(e) => panic!("读取文件失败 {}", e)
    };
    match toml::from_str(&str) {
        Ok(o) => o,
        Err(e) => panic!("读取文件失败 {}", e)
    }
}
