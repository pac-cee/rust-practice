use std::env;
use std::fs::{OpenOptions, read_to_string};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: add <task> | list");
        return;
    }
    match args[1].as_str() {
        "add" => {
            let task = args[2..].join(" ");
            let mut file = OpenOptions::new().append(true).create(true).open("todo.txt").unwrap();
            writeln!(file, "{}", task).unwrap();
            println!("Added: {}", task);
        },
