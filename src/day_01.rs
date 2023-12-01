fn find_num(text: &String, reverse: bool) -> char {
    match reverse {
        false => {
            for c in text.chars() {
                if c.is_numeric() {
                    return c
                }
            }
        }
        true => {
            for c in text.chars().rev() {
                if c.is_numeric() {
                    return c
                }
            }
        }
    }
    '0'
}

pub fn calibration_value(input: &Vec<String>) -> i32 {
  let mut sum = 0;
  for line in input {
    let first_num = find_num(line, false);
    let last_num = find_num(line, true);
    let value = first_num.to_string() + &last_num.to_string();
    sum += i32::from_str_radix(&value, 10).expect("Line contained no numbers")
  }
  sum
}

#[test]
fn find_num_should_return_first_number() {
    let text = String::from("abcd12adf3");
    assert_eq!(find_num(&text, false), '1');
}

#[test]
fn find_num_should_return_last_number() {
    let text = String::from("abcd12adf3");
    assert_eq!(find_num(&text, true), '3');
}

#[test]
fn calibration_value_should_return_correct_value() {
  let mut lines: Vec<String> = Vec::new();
  lines.push(String::from("1abc2"));
  lines.push(String::from("pqr3stu8vwx"));
  lines.push(String::from("a1b2c3d4e5f"));
  lines.push(String::from("treb7uchet"));
  assert_eq!(calibration_value(&lines), 142);
}
