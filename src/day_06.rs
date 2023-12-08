use std::{path::PathBuf, env};

pub struct Race {
  time: i64,
  distance: i64
}

impl Race {
  pub fn from(path: &PathBuf) -> Option<Vec<Self>> {
    let file = std::fs::read_to_string(path).unwrap();
    let lines = file.lines();
    let mut times: Vec<i64> = Vec::new();
    let mut distances: Vec<i64> = Vec::new();
    for line in lines {
      if line.starts_with("Time:") {
        times = parse_values(line);
      } else {
        distances = parse_values(line)
      }
    }
    if times.len() == 0 || distances.len() == 0 || times.len() != distances.len() {
      return None;
    }
    let mut races: Vec<Race> = Vec::with_capacity(times.len());
    for i in 0..times.len() {
      races.push(Race {
        time: times[i],
        distance: distances[i]
      });
    }
    Some(races)
  }

  pub fn from_combined(path: &PathBuf) -> Self {
    let file = std::fs::read_to_string(path).unwrap();
    let lines = file.lines();
    let mut time: i64 = 0;
    let mut distance: i64 = 0;
    for line in lines {
      if line.starts_with("Time:") {
        time = parse_combined(line);
      } else {
        distance = parse_combined(line)
      }
    }
    Race {
      time,
      distance
    }
  }

  pub fn travel_times(&self) -> i64 {
    let mut wins = 0;
    for speed in 1..self.time {
      let traveled_distance = (self.time - speed) * speed;
      if traveled_distance > self.distance {
        wins += 1;
      }
    }
    wins
  }
}

pub fn number_of_ways_to_win(races: &Vec<Race>) -> i64 {
  races.iter().map(|r| r.travel_times()).reduce(|a, b| a * b).unwrap()
}

fn parse_values(line: &str) -> Vec<i64> {
  let mut values: Vec<i64> = Vec::new();
  let split = line.split(" ");
  for s in split {
    match i64::from_str_radix(s, 10) {
      Ok(v) => values.push(v),
      Err(_) => ()
    }
  }
  values
}

fn parse_combined(line: &str) -> i64 {
  let mut all_nums = String::new();
  let split = line.split(" ");
  for s in split {
    if s.parse::<i64>().is_ok() {
      all_nums.push_str(s);
    }
  }
  i64::from_str_radix(all_nums.as_str(), 10).unwrap()
}

#[test]
fn race_from_should_create_races() {
  let file = env::current_dir().unwrap().join("input/day_06_test_input.txt");
  let races = Race::from(&file);
  assert_eq!(races.is_some(), true);
  let races = races.unwrap();
  assert_eq!(races.len(), 3);
}

#[test]
fn travel_times_should_return_correct_values() {
  let file = env::current_dir().unwrap().join("input/day_06_test_input.txt");
  let races = Race::from(&file);
  assert_eq!(races.is_some(), true);
  let races = races.unwrap();
  assert_eq!(races.len(), 3);
  assert_eq!(races[0].travel_times(), 4);
  assert_eq!(races[1].travel_times(), 8);
  assert_eq!(races[2].travel_times(), 9);
}

#[test]
fn number_of_ways_to_win_should_return_correct_values() {
  let file = env::current_dir().unwrap().join("input/day_06_test_input.txt");
  let races = Race::from(&file);
  assert_eq!(races.is_some(), true);
  let races = races.unwrap();
  assert_eq!(races.len(), 3);
  assert_eq!(number_of_ways_to_win(&races), 288);
}

#[test]
fn from_combined_should_create_correct_race() {
  let file = env::current_dir().unwrap().join("input/day_06_test_input.txt");
  let race = Race::from_combined(&file);
  assert_eq!(race.time, 71530);
  assert_eq!(race.distance, 940200);
}