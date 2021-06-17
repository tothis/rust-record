use std::{fs, io};
use std::fs::DirEntry;

use once_cell::sync::Lazy;

mod config;

#[cfg(target_os = "windows")]
const SEPARATOR_CHAR: &str = "\\";
#[cfg(not(target_os = "windows"))]
const SEPARATOR_CHAR: &str = "/";

static CONFIG: Lazy<config::Config> = Lazy::new(|| {
    config::read_config()
});

fn main() {
    recursion(&CONFIG.path);
}

fn recursion(path: &str) {

    // 排除前缀
    for exclude_prefix in &CONFIG.exclude_prefix {
        if path.starts_with(&(CONFIG.path.to_string() + SEPARATOR_CHAR + &exclude_prefix)) {
            return;
        }
    }

    // 排除后缀
    for exclude_suffix in &CONFIG.exclude_suffix {
        if path.ends_with(exclude_suffix) {
            return;
        }
    }

    let file_type = fs::metadata(path).unwrap().file_type();
    let is_file = file_type.is_file();
    let is_dir = file_type.is_dir();

    // 递归目录
    if is_dir {
        let paths: Vec<DirEntry> = fs::read_dir(path).unwrap()
            .collect::<Result<Vec<DirEntry>, io::Error>>().unwrap();
        for path in paths {
            recursion(path.path().to_str().unwrap());
        }
    } else if is_file {
        match fs::read_to_string(path) {
            Ok(str) => {
                if str.len() > 1 {
                    print!("{}", path);
                    let last_char = &str[find_char_boundary(&str, str.len() - 1)..];
                    if !last_char.eq("\n") {
                        println!(" 最后一行不存在换行符");
                        match fs::write(path, str + "\n") {
                            Err(e) => println!("写入出错 {}", e),
                            _ => {}
                        }
                    } else {
                        println!()
                    }
                }
            }
            _ => {}
        };
    }
}

// 判断 index 是否为字符边界，否则向后查找字符边界位置
fn find_char_boundary(s: &str, index: usize) -> usize {
    if s.len() <= index {
        return index;
    }
    let mut new_index = index;
    while !s.is_char_boundary(new_index) {
        new_index += 1;
    }
    new_index
}
