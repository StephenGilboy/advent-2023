use std::fs::read_to_string;
use std::env;

use day_02::Game;
use day_04::Card;
mod day_01;

mod day_02;

mod day_04;

fn main() {
    // Day 01
    let answer = day01();
    println!("Day 1: {}", answer);

    // Day 02
    let answer2 = day02();
    println!("Day 2: {}, {}", answer2.0, answer2.1);

    let answer4 = day04();
    println!("Day 4: {}, {}", answer4.0, answer4.1);
}

fn day01() -> i32 {
    let mut input: Vec<String> = Vec::new();
    let file = env::current_dir().unwrap().join("input/day_01_input.txt");
    for line in read_to_string(file).unwrap().lines() {
        input.push(line.to_string());
    } 
    day_01::calibration_value(&input)
}

fn day02() -> (i32, i32) {
    let mut games: Vec<Game> = Vec::new();
    let file = env::current_dir().unwrap().join("input/day_02_input.txt");
    let mut i = 1;
    let cube_count = day_02::CubeCount::from(12, 13, 14);

    for line in read_to_string(file).unwrap().lines() {
        let input: Vec<&str> = line.split(":").collect();
        let game = Game::from(i, input[1]);
        games.push(game);
        i += 1;
    }

    let mut answer_one_sum = 0;
    let mut answer_two_sum = 0;
    for game in games  {
        if game.less_than_eq_cube_count(&cube_count) {
            answer_one_sum += game.id;
        }
        answer_two_sum += game.power();
    }
    (answer_one_sum, answer_two_sum)
}

fn day04() -> (i32, i32) {
    let mut cards: Vec<Card> = Vec::new();
    let file = env::current_dir().unwrap().join("input/day_04_input.txt");
    let mut i = 1;
    let mut sum = 0;
    for line in read_to_string(file).unwrap().lines() {
        let input: Vec<&str> = line.split(":").collect();
        let card = Card::from(i, input[1]);
        sum += card.points();
        cards.push(card);
        i += 1;
    }
    (sum, 0)
}