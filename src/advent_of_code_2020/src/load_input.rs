use std::{fs::File, io::{self, BufRead}};

pub fn load_integers(filename: &str) -> Result<Vec<i32>, String> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => return Err(format!("Failed to open the file {}", filename)),
    };
    let reader = io::BufReader::new(file);
    let mut v: Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => return Err("Failed to read a line.".to_string()),
        };
        let line = match line.parse::<i32>() {
            Ok(num) => num,
            Err(_) => return Err(format!("Failed to parse the line {} file {}", line, filename)),
        };
        v.push(line);
    };

    Ok(v)
}