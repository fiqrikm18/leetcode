use std::collections::HashMap;

struct RomanToIntSolution {}

impl RomanToIntSolution {
    fn get_roman_number(&self) -> HashMap<&str, i32> {
        let mut roman_number: HashMap<&str, i32> = HashMap::new();
        roman_number.insert("I", 1);
        roman_number.insert("V", 5);
        roman_number.insert("X", 10);
        roman_number.insert("L", 50);
        roman_number.insert("C", 100);
        roman_number.insert("D", 500);
        roman_number.insert("M", 1000);
        roman_number
    }

    pub fn roman_to_int(&self, s: String) -> i32 {
        let mut result: i32 = 0;
        let mut last_char = "";

        let roman_number = self.get_roman_number();
        let reversed_str = s
            .split("")
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|x| *x != "")
            .rev()
            .collect::<Vec<&str>>();

        for (idx, val) in reversed_str.into_iter().enumerate() {
            if idx == 0 {
                result += roman_number.get(val).unwrap();
                last_char = val;

                continue;
            }

            match val {
                "I" => {
                    if last_char == "V" || last_char == "X" {
                        let last_char_val = *roman_number.get(val).unwrap();
                        result = result - last_char_val + (last_char_val - 1)
                    } else {
                        result += roman_number.get(val).unwrap();
                    }
                }
                "X" => {
                    if last_char == "L" || last_char == "C" {
                        let last_char_val = *roman_number.get(val).unwrap();
                        result = result - last_char_val + (last_char_val - 10)
                    } else {
                        result += roman_number.get(val).unwrap();
                    }
                }
                "C" => {
                    if last_char == "D" || last_char == "M" {
                        let last_char_val = *roman_number.get(val).unwrap();
                        result = result - last_char_val + (last_char_val - 100)
                    } else {
                        result += roman_number.get(val).unwrap();
                    }
                }
                _ => {
                    result += roman_number.get(val).unwrap();
                }
            }

            last_char = val;
        }

        result
    }
}

pub fn solve() {
    let sln = RomanToIntSolution {};
    let result = sln.roman_to_int(String::from("XLIV"));

    println!("{result}");
}
