use std::io;

use advent_of_code_2020::simple_puzzles::{boarding_passes, declaration_forms, handheld_game_console, joltage_adapter, 
    luggage_rules, northpol_password_validation, number_processing, password_debug, toboggan_navigation, xmas_encryption, 
    waiting_room, ferry_navigation};

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
    println!("9: Boarding Pass Part 1");
    println!("10: Boarding Pass Part 2");
    println!("11: Declaration Forms Part 1");
    println!("12: Declaration Forms Part 2");
    println!("13: Luggage Rules Part 1");
    println!("14: Luggage Rules Part 2");
    println!("15: Game Console Part 1");
    println!("16: Game Console Part 2");
    println!("17: XMAX Encryption Part 1");
    println!("18: XMAX Encryption Part 2");
    println!("19: Joltage Adapter Part 1");
    println!("20: Joltage Adapter Part 2");
    println!("21: Waiting Room Part 1");
    println!("22: Waiting Room Part 2");
    println!("23: Ferry Navigation Part 1");
    println!("24: Ferry Navigation Part 2");

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
        9 => boarding_passes::part1(),
        10 => boarding_passes::part2(),
        11 => declaration_forms::part1(),
        12 => declaration_forms::part2(),
        13 => luggage_rules::part1(),
        14 => luggage_rules::part2(),
        15 => handheld_game_console::part1(),
        16 => handheld_game_console::part2(),
        17 => xmas_encryption::part1(),
        18 => xmas_encryption::part2(),
        19 => joltage_adapter::part1(),
        20 => joltage_adapter::part2(),
        21 => waiting_room::part1(),
        22 => waiting_room::part2(),
        23 => ferry_navigation::part1(),
        24 => ferry_navigation::part2(),
        _ => println!("Invalid input!"),
    };
}