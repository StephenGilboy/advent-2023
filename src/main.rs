use std::fs::read_to_string;
use std::env;
mod day_01;

fn main() {
    // Day 01
    let answer = day01();
    println!("Day 1: {}", answer)
}

fn day01() -> i32 {
    let mut input: Vec<String> = Vec::new();
    let file = env::current_dir().unwrap().join("input/day_01_input.txt");
    for line in read_to_string(file).unwrap().lines() {
        input.push(line.to_string());
    } 
    day_01::calibration_value(&input)
}
