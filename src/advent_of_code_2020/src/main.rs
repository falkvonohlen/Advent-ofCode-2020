use std::io;

use advent_of_code_2020::{number_processing, password_debug, toboggan_navigation};

fn main() {
    println!("Select which function you want to call:");
    println!("1: Report Repair Part 1");
    println!("2: Report Repair Part 2");
    println!("3: Password Debug Part 1");
    println!("4: Password Debug Part 2");
    println!("5: Navigation Part 1");
    println!("6: Navigation Part 2");

    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read input");

    let selection: u8 = match selection.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    match selection {
        1 => number_processing::solve_day1_puzzle_part1(),
        2 => number_processing::solve_day1_puzzle_part2(),
        3 => password_debug::solve_day2_puzzle_part1(),
        4 => password_debug::solve_day2_puzzle_part2(),
        5 => toboggan_navigation::solve_day3_part1(),
        6 => toboggan_navigation::solve_day3_part2(),
        _ => println!("Invalid input!"),
    };
}
