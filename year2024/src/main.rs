mod day1;
mod day2;

fn main() {
    use crate::*;
    println!("{}", day2::day2::part1(input("./day2.txt")));
}

use std::fs;

fn input(file_path: &str) -> String {
    fs::read_to_string(String::from(file_path))
        .expect("Could not read from file")
}