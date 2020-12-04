use std::{fs, path};

use crate::load_input;

trait VecExtension<T> {
    fn count_of(&self, compare: T) -> u32;
}

impl VecExtension<i32> for Vec<i32> {
    fn count_of(&self, compare: i32) -> u32 {
        let mut result: u32 = 0;
        for i in self {
            if i == &compare {
                result += 1;
            }
        }
        result
    }
}

pub fn part1() {
    let target_number = 2020;
    let input_file = path::Path::new("./resources/Day1Input.txt");
    let input_file = fs::canonicalize(&input_file)
        .expect("File not found")
        .into_os_string()
        .into_string()
        .expect("Failed to transform the filepath to a string");

    let input = match load_input::load_integers(&input_file) {
        Ok(vec) => vec,
        Err(s) => {
            println!("{}", s);
            return;
        }
    };

    println!(
        "Puzzle input has been read from the file {} with {} inputs",
        input_file,
        input.len()
    );
    println!(
        "The target number for adding two numbers is {}",
        target_number
    );

    match find_adding_numbers_in_one_vector(input, target_number) {
        Some((num_a, num_b)) => println!("Found {} * {} = {}", num_a, num_b, num_a * num_b),
        None => println!(
            "The input does not contain two numbers which add up to {}",
            target_number
        ),
    };
}

pub fn part2() {
    let target_number = 2020;
    let input_file = path::Path::new("./resources/Day1Input.txt");
    let input_file = fs::canonicalize(&input_file)
        .expect("File not found")
        .into_os_string()
        .into_string()
        .expect("Failed to transform the filepath to a string");

    let input = match load_input::load_integers(&input_file) {
        Ok(vec) => vec,
        Err(s) => {
            println!("{}", s);
            return;
        }
    };

    println!(
        "Puzzle input has been read from the file {} with {} inputs",
        input_file,
        input.len()
    );
    println!(
        "The target number for adding two numbers is {}",
        target_number
    );

    match find_three_adding_numbers(input, target_number) {
        Some((num_a, num_b, num_c)) => println!(
            "Found {} * {} * {} = {}",
            num_a,
            num_b,
            num_c,
            num_a * num_b * num_c
        ),
        None => println!(
            "The input does not contain three numbers which add up to {}",
            target_number
        ),
    };
}

fn find_three_adding_numbers(input: Vec<i32>, target: i32) -> Option<(i32, i32, i32)> {
    let length = input.len() - 1;
    for index in 0..length {
        let mut cloned = input.clone();
        cloned.remove(index);
        match find_adding_numbers_in_one_vector(cloned, target - input[index]) {
            Some((num_a, num_b)) => return Some((num_a, num_b, input[index])),
            None => continue,
        };
    }

    None
}

fn find_adding_numbers_in_one_vector(input: Vec<i32>, target: i32) -> Option<(i32, i32)> {
    let even = target % 2 == 0;
    let (mut even_input, mut odd_input): (Vec<_>, Vec<_>) =
        input.into_iter().partition(|i| i % 2 == 0);
    even_input.sort();
    odd_input.sort();

    if even {
        match search_in_only_even_or_odd(even_input, target) {
            Some(a) => Some(a),
            None => search_in_only_even_or_odd(odd_input, target),
        }
    } else {
        //If the target is odd one number needs to be odd and one even in order to create an odd target number
        return find_adding_numbers_in_two_vectors(even_input, odd_input, target);
    }
}

fn search_in_only_even_or_odd(input: Vec<i32>, target: i32) -> Option<(i32, i32)> {
    let half_of_target = target / 2;
    //If the target is even the two number which adding up are either both even or both odd
    let (greater, less): (Vec<_>, Vec<_>) = input.into_iter().partition(|i| i > &half_of_target);

    //If we have the half of the target multiple times we are done as two of them will add up to the target
    if less.count_of(half_of_target) > 1 {
        return Some((target, target));
    } else {
        return find_adding_numbers_in_two_vectors(less, greater, target);
    }
}

fn find_adding_numbers_in_two_vectors(
    input1: Vec<i32>,
    input2: Vec<i32>,
    target: i32,
) -> Option<(i32, i32)> {
    for number_1 in input1 {
        match find_adding_number_in_vector(number_1, &input2, target) {
            Some(num) => return Some((number_1, num)),
            None => continue,
        }
    }

    None
}

fn find_adding_number_in_vector(test_number: i32, input: &[i32], target: i32) -> Option<i32> {
    let length = input.len();
    if length == 0 {
        None
    } else {
        let length = length / 2;
        let lower = &input[..length];
        let upper = &input[length..];
        let control = target - (upper[0] + test_number);
        if control == 0 {
            Some(upper[0])
        } else if control < 0 {
            find_adding_number_in_vector(test_number, lower, target)
        } else {
            find_adding_number_in_vector(test_number, &upper[1..], target)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_part_1() {
        let test_input = vec![1721, 979, 366, 299, 675, 1456];
        let (num_a, num_b) = match find_adding_numbers_in_one_vector(test_input, 2020) {
            Some(x) => x,
            None => panic!("No numbers found"),
        };
        assert_eq!(514579, num_a * num_b);
    }

    #[test]
    fn test_day_1_part_2() {
        let test_input = vec![1721, 979, 366, 299, 675, 1456];
        let (num_a, num_b, num_c) = match find_three_adding_numbers(test_input, 2020) {
            Some(x) => x,
            None => panic!("No numbers found"),
        };
        assert_eq!(241861950, num_a * num_b * num_c);
    }

    #[test]
    fn test_find_adding_number_in_vector() {
        let mut test_input = vec![1721, 979, 366, 675, 1456];
        test_input.sort();
        assert_eq!(
            1721,
            find_adding_number_in_vector(299, &test_input, 2020).unwrap()
        );
    }
}
