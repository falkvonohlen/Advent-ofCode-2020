use std::io;

use advent_of_code_2020::number_processing;

fn main() {
    println!("Select which function you want to call:");
    println!("1: Report Repair Part 1");
    println!("2: Report Repair Part 2");

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
            _ => println!("Invalid input!"),
        };
}