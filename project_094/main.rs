use chrono::{Datelike, NaiveDate};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: <year> <month>");
        return;
    }
    let year: i32 = args[1].parse().unwrap();
    let month: u32 = args[2].parse().unwrap();
    let first = NaiveDate::from_ymd_opt(year, month, 1).unwrap();
    let last_day = match month {
        12 => 31,
        _ => NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap().pred_opt().unwrap().day(),
    };
    println!("Sun Mon Tue Wed Thu Fri Sat");
