use std::io::stdin;
use std::io::prelude::*;
use super::types::Lines;


pub fn read_input() -> Lines {
    let mut lines: Lines = Vec::new();
    for line in stdin().lock().lines() {
        match line {
            Ok(line) => lines.push(line),
            Err(error) => println!("{:?}", error)
        }
    }
    return lines;
}