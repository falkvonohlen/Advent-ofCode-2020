use regex::Regex;

use crate::load_input;

struct PasswordWithPolicy {
    password: String,
    check_character: char,
    lower: usize,
    upper: usize,
}

impl PasswordWithPolicy {
    fn from(input: &str) -> Option<PasswordWithPolicy> {
        let input_regex =
            Regex::new("(\\d+)-(\\d+) (.): (.*)").expect("Invalid regular expression");
        if input_regex.is_match(&input) {
            let groups = input_regex.captures_iter(&input).next().unwrap();
            Some(PasswordWithPolicy {
                password: groups[4].to_string(),
                check_character: groups[3]
                    .chars()
                    .next()
                    .expect("Failed to parse the test char"),
                upper: groups[2]
                    .parse::<usize>()
                    .expect("Failed to parse upper range"),
                lower: groups[1]
                    .parse::<usize>()
                    .expect("Failed to parse lower range"),
            })
        } else {
            None
        }
    }

    fn is_valid_sled_rental(&self) -> bool {
        let mut count: usize = 0;
        for c in self.password.chars() {
            if self.check_character == c {
                count += 1;
            }
        }
        self.lower <= count && self.upper >= count
    }

    fn is_valid_toboggan_corporate(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<char>>();
        match (chars.get(self.lower - 1), chars.get(self.upper - 1)) {
            (Some(char_1), Some(char_2)) => {
                (char_1 == &self.check_character && char_2 != &self.check_character)
                    || (char_1 != &self.check_character && char_2 == &self.check_character)
            }
            _ => false,
        }
    }
}

pub fn part1() {
    let input_file =
        load_input::load_strings("./resources/Day2Input.txt").expect("Failed to read the input");

    println!("Found {} passwords", input_file.len());

    let count = input_file
        .into_iter()
        .filter(|s| {
            PasswordWithPolicy::from(s)
                .expect("Invalid Password")
                .is_valid_sled_rental()
        })
        .count();

    println!("{} are valid", count);
}

pub fn part2() {
    let input_file =
        load_input::load_strings("./resources/Day2Input.txt").expect("Failed to read the input");

    println!("Found {} passwords", input_file.len());

    let count = input_file
        .into_iter()
        .filter(|s| {
            PasswordWithPolicy::from(s)
                .expect("Invalid Password")
                .is_valid_toboggan_corporate()
        })
        .count();

    println!("{} are valid", count);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day1_part1() {
        assert!(PasswordWithPolicy::from(&"1-3 a: abcde".to_string())
            .unwrap()
            .is_valid_sled_rental());
        assert!(!PasswordWithPolicy::from(&"1-3 b: cdefg".to_string())
            .unwrap()
            .is_valid_sled_rental());
        assert!(PasswordWithPolicy::from(&"2-9 c: ccccccccc".to_string())
            .unwrap()
            .is_valid_sled_rental());
    }

    #[test]
    fn test_day1_part2() {
        assert!(PasswordWithPolicy::from(&"1-3 a: abcde".to_string())
            .unwrap()
            .is_valid_toboggan_corporate());
        assert!(!PasswordWithPolicy::from(&"1-3 b: cdefg".to_string())
            .unwrap()
            .is_valid_toboggan_corporate());
        assert!(!PasswordWithPolicy::from(&"2-9 c: ccccccccc".to_string())
            .unwrap()
            .is_valid_toboggan_corporate());
    }
}
