use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

pub struct Entry {
  destination_range_start: i32,
  source_range_start: i32,
  range_length: i32
}
pub struct Almanac {
  seeds: Vec<i32>,
  seed_to_soil: Vec<Entry>,
  soil_to_fertilizer: Vec<Entry>,
  fertilizer_to_water: Vec<Entry>,
  water_to_light: Vec<Entry>,
  light_to_temp: Vec<Entry>,
  temp_to_humid: Vec<Entry>,
  humid_to_loc: Vec<Entry>
}

impl Entry {
  fn from_str(line: &str) -> Self {
    let mut dest = -1;
    let mut source = -1;
    let mut range = -1;
    for n in line.split(" ") {
      match i32::from_str_radix(n.trim(), 10) {
        Ok(s) => {
          if dest == -1 {
            dest = s;
          }
          else if source == -1 {
            source = s;
          }
          else if range == -1 {
            range = s;
          }
          else {
            break;
          }
        },
        Err(_) => ()
      }
    }
    Entry {
      destination_range_start: dest,
      source_range_start: source,
      range_length: range
    }
  }
}

enum AlmanacChapters {
  SeedToSoil,
  SoilToFertilizer,
  FertilizerToWater,
  WaterToLight,
  LightToTemp,
  TempToHumid,
  HumidToLocation,
  End
}

impl Almanac {
  pub fn from_file(path: PathBuf) -> Self {
    let mut seeds: Vec<i32> = Vec::new();
    let mut seed_to_soil: Vec<Entry> = Vec::new();
    let mut soil_to_fertilizer: Vec<Entry> = Vec::new();
    let mut fertilizer_to_water: Vec<Entry> = Vec::new();
    let mut water_to_light: Vec<Entry> = Vec::new();
    let mut light_to_temp: Vec<Entry> = Vec::new();
    let mut temp_to_humid: Vec<Entry> = Vec::new();
    let mut humid_to_loc: Vec<Entry> = Vec::new();
    let file = read_to_string(path).unwrap();
    let lines: Vec<&str> = file.lines().collect();
    let mut i = 0;

    while i < lines.len() {
      let line = lines[i];
      let mut l: &str;
      if line.starts_with("seeds:") {
        seeds = parse_seeds(line)
      }
      if line.starts_with("seed-to-soil") {
        i += 1;
        l = lines[i];
        while l.trim() != "" {
          seed_to_soil.push(Entry::from_str(l));
          i += 1;
          l = lines[i];
        }
        continue;
      }
      if line.starts_with("soil-to-fertilizer") {
        i += 1;
        l = lines[i];
        while l.trim() != "" {
          soil_to_fertilizer.push(Entry::from_str(l));
          i += 1;
          l = lines[i];
        }
      }
      if line.starts_with("fertilizer-to-water") {
        i += 1;
        l = lines[i];
        while l.trim() != "" {
          fertilizer_to_water.push(Entry::from_str(l));
          i += 1;
          l = lines[i];
        }
      }
      if line.starts_with("water-to-light") {
        i += 1;
        l = lines[i];
        while l.trim() != "" {
          water_to_light.push(Entry::from_str(l));
          i += 1;
          l = lines[i];
        }
      }
      if line.starts_with("light-to-temperature") {
        i += 1;
        l = lines[i];
        while l.trim() != "" {
          light_to_temp.push(Entry::from_str(l));
          i += 1;
          l = lines[i];
        }
      }
      if line.starts_with("temperature-to-humidity") {
        i += 1;
        l = lines[i];
        while l.trim() != "" {
          temp_to_humid.push(Entry::from_str(l));
          i += 1;
          l = lines[i];
        }
      }
      if line.starts_with("humidity-to-location") {
        i += 1;
        l = lines[i];
        while l.trim() != "" {
          humid_to_loc.push(Entry::from_str(l));
          i += 1;
          if i >= lines.len() {
            break;
          }
          l = lines[i];
        }
      }
      i += 1;
    }

    Almanac {
      seeds,
      seed_to_soil,
      soil_to_fertilizer,
      fertilizer_to_water,
      water_to_light,
      light_to_temp,
      temp_to_humid,
      humid_to_loc,
    }
  }

  pub fn lowest_location(&self) -> i32 {
      self.seeds.iter().map(|seed| {
        let mut current_chapter = AlmanacChapters::SeedToSoil;
        let mut result = 0;
        let mut keep_going = true;
        result = get_result(&self.seed_to_soil, &seed);
        result = get_result(&self.soil_to_fertilizer, &result);
        result = get_result(&self.fertilizer_to_water, &result);
        result = get_result(&self.water_to_light, &result);
        result = get_result(&self.light_to_temp, &result);
        result = get_result(&self.temp_to_humid, &result);
        result = get_result(&self.humid_to_loc, &result);
        result
      }).reduce(|a, b| {
        match a < b {
            true => a,
            false => b
        }
      }).expect("SHould have value")
    }
}

fn get_result(entries: &Vec<Entry>, seed: &i32) -> i32 {
  let mut result = -1;
  for e in entries {
    println!("Check soil");
    println!("ENTRY: {}", e.source_range_start);
    let soil = check_entry(seed, &e);
    let test = soil;
    println!("Soil {:?}", test);
    if result == -1 {
      result = soil;
    }
    if soil != *seed {
      result = soil;
      break;
    }
    result = soil;
  }
  return result;
}

fn check_entry(seed: &i32, entry: &Entry) -> i32 {
  if seed == &entry.source_range_start {
    return &entry.destination_range_start + 0;
  }
  if seed < &entry.source_range_start {
    return *seed;
  }
  if *seed > &entry.source_range_start + &entry.range_length {
    return *seed;
  }
  let mut mid = &entry.range_length / 2;
  let mut sum = &entry.source_range_start + mid;
  println!("{}", mid);
  while sum != *seed {
       if *seed == sum {
        return &entry.destination_range_start + mid;
       }
       if *seed > sum {
        mid = mid / 2 + mid;
       } else {
        mid = mid / 2;
       }
       sum = &entry.source_range_start + mid;
  }
  return &entry.destination_range_start + mid;
}

fn parse_seeds(line: &str) -> Vec<i32> {
  let mut seeds: Vec<i32> = Vec::new();
  for s in line.split(":") {
    if s.contains("s") {
      continue;
    }
    for n in s.split(" ") {
      match i32::from_str_radix(n.trim(), 10) {
        Ok(s) => seeds.push(s),
        Err(_) => ()
      }
    }
  }
  seeds
}

#[test]
fn almanac_from_should_create_almanac() {
  let file = env::current_dir().unwrap().join("input/day_05_test_input.txt");
  let almanac = Almanac::from_file(file);
  assert_eq!(almanac.seeds.len(), 4);
  assert_eq!(almanac.seed_to_soil.len(), 2);
  assert_eq!(almanac.soil_to_fertilizer.len(), 3);
  assert_eq!(almanac.fertilizer_to_water.len(), 4);
  assert_eq!(almanac.water_to_light.len(), 2);
  assert_eq!(almanac.light_to_temp.len(), 3);
  assert_eq!(almanac.temp_to_humid.len(), 2);
  assert_eq!(almanac.humid_to_loc.len(), 2);
}

#[test]
fn almanac_create_map_should_create_maps() {
  let file = env::current_dir().unwrap().join("input/day_05_test_input.txt");
  let almanac = Almanac::from_file(file);
  let lowest_location = almanac.lowest_location();
  assert_eq!(lowest_location, 35);
}