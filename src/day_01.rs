use std::collections::HashMap;

fn get_text_number() -> HashMap<String, char> {
    let mut text_number: HashMap<String, char> = HashMap::new();
    text_number.insert("one".to_string(), '1');
    text_number.insert("two".to_string(), '2');
    text_number.insert("three".to_string(), '3');
    text_number.insert("four".to_string(), '4');
    text_number.insert("five".to_string(), '5');
    text_number.insert("six".to_string(), '6');
    text_number.insert("seven".to_string(), '7');
    text_number.insert("eight".to_string(), '8');
    text_number.insert("nine".to_string(), '9');
    text_number
}

fn str_in_get_text_number(text: &String, text_nums: &HashMap<String, char>) -> char {
    if text.len() < 3 {
        return '\0';
    }
    let mut text_start_idx = 0;
    if text_nums.contains_key(text) {
        return text_nums[text];
    }

    while text_start_idx < text.len() {
        let mut take = 3;

        while text_start_idx + take <= text.len() {
            let chars = text.chars().skip(text_start_idx).take(take).into_iter();
            let test = String::from_iter(chars);
            if text_nums.contains_key(&test) {
                return text_nums[&test];
            }
            take += 1;
        }
        text_start_idx += 1;
    }
    '\0'
}

fn find_num(line: &String, text_number: &HashMap<String, char>, reverse: bool) -> char {
    let mut text: String = String::new();
    match reverse {
        false => {
            for c in line.chars() {
                if c.is_numeric() {
                    return c
                } else {
                    text = format!("{}{}", text, c);
                    let txt_num_result = str_in_get_text_number(&text, &text_number);
                    if txt_num_result != '\0' {
                        return txt_num_result;
                    }
                }
            }
        }
        true => {
            for c in line.chars().rev() {
                if c.is_numeric() {
                    return c
                } else {
                    text = format!("{}{}", c, text);
                    let txt_num_result = str_in_get_text_number(&text, &text_number);
                    if txt_num_result != '\0' {
                        return txt_num_result;
                    }
                }
            }
        }
    }
    '0'
}

pub fn calibration_value(input: &Vec<String>) -> i32 {
  let text_number = get_text_number();
  let mut sum = 0;
  for line in input {
    let first_num = find_num(line, &text_number, false);
    let last_num = find_num(line, &text_number, true);
    let value = format!("{}{}", first_num, last_num);
    sum += i32::from_str_radix(&value, 10).expect("Line contained no numbers");
  }
  sum
}

#[test]
fn find_num_should_return_first_number() {
    let text_number = get_text_number();
    let line = String::from("abcd12adf3");
    assert_eq!(find_num(&line, &text_number, false), '1');
}

#[test]
fn find_num_should_return_first_text_number() {
    let text_number = get_text_number();
    let line = String::from("abone2asdfthree4");
    assert_eq!(find_num(&line, &text_number, false), '1');
}

#[test]
fn find_num_should_return_last_number() {
    let text_number = get_text_number();
    let line = String::from("abcd12adf3");
    assert_eq!(find_num(&line, &text_number, true), '3');
}

#[test]
fn find_num_should_return_last_text_number() {
    let text_number = get_text_number();
    let line = String::from("3kmtjlfbgssixmspkfzrgxtctksix4onetwones");
    assert_eq!(find_num(&line, &text_number, true), '1');
}

#[test]
fn find_num_should_return_last_text_number_v2() {
    let text_number = get_text_number();
    let line = String::from("fiveeightfive8vbqtsmhjqr5vgbmsxrkh7four");
    assert_eq!(find_num(&line, &text_number, true), '4');
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
