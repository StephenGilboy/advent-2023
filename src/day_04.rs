pub struct Card {
  id: i32,
  card_numbers: Vec<i32>,
  winning_numbers: Vec<i32>
}

impl Card {
  pub fn from(id: i32, line: &str) -> Card {
    let split: Vec<&str> = line.split("|").collect();
    let mut card_numbers: Vec<i32> = Vec::new();
    let mut winning_numbers: Vec<i32> = Vec::new();

    split[1].split(" ").for_each(|t| {
      match i32::from_str_radix(t.trim(), 10) {
        Ok(i) => card_numbers.push(i),
        Err(_) => ()
      }
    });
    split[0].split(" ").for_each(|t| {
      match i32::from_str_radix(t.trim(), 10) {
        Ok(i) => winning_numbers.push(i),
        Err(_) => ()
      }
    });
    Card {
      id,
      card_numbers,
      winning_numbers
    }
  }

  pub fn points(&self) -> i32 {
    let mut p: i32 = 0;
    self.card_numbers.iter().for_each(|n| {
      if self.winning_numbers.iter().any(|wn| {
        *wn == *n
      }) {
        if p == 0 {
          p = 1
        } else {
          p = p * 2
        }
      }
    });
    p
  }
}

#[test]
fn card_from_should_create_card() {
  let want = Card {
    id: 1,
    winning_numbers: Vec::from_iter([41,48,83,86,17]),
    card_numbers: Vec::from_iter([83, 86, 6, 31, 17, 9, 48, 53])
  };
  let got = Card::from(1, " 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
  
  assert_eq!(want.id, got.id);
  assert_eq!(want.card_numbers.len(), got.card_numbers.len());
  assert_eq!(want.winning_numbers.len(), got.winning_numbers.len());
}

#[test]
fn card_points_should_give_correct_answer() {
  let got = Card::from(1, " 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
  assert_eq!(got.points(), 8);
}
