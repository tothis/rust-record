use std::env;

use chrono::Local;

use time::{FORMAT, parse_time};

fn main() {
    let mut args = env::args();
    if args.len() == 1 {
        println!("{}", Local::now().format(FORMAT));
        println!("{}", Local::now().timestamp_millis());
        return;
    }
    args.next();
    if args.len() == 1 {
        parse_time(args)
    }
}