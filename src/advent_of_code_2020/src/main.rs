use std::io;

use advent_of_code_2020::{
    northpol_password_validation, number_processing, password_debug, toboggan_navigation,
};

fn main() {
    println!("Select which function you want to call:");
    println!("1: Report Repair Part 1");
    println!("2: Report Repair Part 2");
    println!("3: Password Debug Part 1");
    println!("4: Password Debug Part 2");
    println!("5: Navigation Part 1");
    println!("6: Navigation Part 2");
    println!("7: Passport validation Part 1");
    println!("8: Passport validation Part 2");

    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read input");

    let selection: u8 = match selection.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    match selection {
        1 => number_processing::part1(),
        2 => number_processing::part2(),
        3 => password_debug::part1(),
        4 => password_debug::part2(),
        5 => toboggan_navigation::part1(),
        6 => toboggan_navigation::part2(),
        7 => northpol_password_validation::part1(),
        8 => northpol_password_validation::part2(),
        _ => println!("Invalid input!"),
    };
}
