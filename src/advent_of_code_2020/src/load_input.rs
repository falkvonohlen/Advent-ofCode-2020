use std::{
    fs::File,
    io::{self, BufRead},
};

pub fn load_integers(filename: &str) -> Result<Vec<i32>, String> {
    let lines = load_strings(filename)?;
    let mut v: Vec<i32> = Vec::new();
    for line in lines {
        let line = match line.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                return Err(format!(
                    "Failed to parse the line {} file {}",
                    line, filename
                ))
            }
        };
        v.push(line);
    }

    Ok(v)
}

pub fn load_strings(filename: &str) -> Result<Vec<String>, String> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(_) => return Err(format!("Failed to open the file {}", filename)),
    };
    let reader = io::BufReader::new(file);
    let mut v: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(_) => return Err("Failed to read a line.".to_string()),
        };
        v.push(line);
    }

    Ok(v)
}

pub fn load_empty_line_seperated(filename: &str) -> Result<Vec<Vec<String>>, String> {
    let lines = load_strings(filename)?;
    let mut batches = vec![];
    let mut next_line: usize = 0;

    while next_line < lines.len() {
        let unparsed_input = &lines[next_line..];
        let empty_line = unparsed_input.iter().position(|l| l.as_str() == "");
        let end = match empty_line {
            Some(num) => num + next_line,
            None => lines.len(),
        };
        let batch = lines[next_line..end].to_vec();
        batches.push(batch);
        next_line = end + 1;
    }

    Ok(batches)
}
