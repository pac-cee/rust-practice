use std::env;
use std::fs;
use std::io::{self, Read, Write};

fn xor_file(input: &str, output: &str, key: u8) -> io::Result<()> {
    let mut data = fs::read(input)?;
    for byte in &mut data {
        *byte ^= key;
    }
    fs::write(output, data)?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        println!("Usage: encrypt <input> <output> <key> | decrypt <input> <output> <key>");
