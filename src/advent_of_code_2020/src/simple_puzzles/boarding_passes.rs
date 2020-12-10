use crate::load_input;

#[derive(Debug)]
struct BoardingPass {
    row: u8,
    column: u8,
    seat_id: u32,
}

#[derive(Debug)]
enum BoardingPassError {
    InvalidSize(usize),
    InvalidRowChar(char),
    InvalidColumnChar(char),
}

impl BoardingPass {
    fn from(input: String) -> Result<BoardingPass, BoardingPassError> {
        match input.chars().count() {
            10 => {
                let row = BoardingPass::determine_row(&input[..7])?;
                let column = BoardingPass::determine_column(&input[7..])?;
                let seat_id = (row as u32) * 8 + column as u32;

                Ok(BoardingPass {
                    row,
                    column,
                    seat_id,
                })
            }
            s => Err(BoardingPassError::InvalidSize(s)),
        }
    }

    fn determine_row(input: &str) -> Result<u8, BoardingPassError> {
        let mut row: u8 = 0;
        let mut to_add: u8 = 64;
        for c in input.chars() {
            match c {
                'F' => (),
                'B' => row += to_add,
                a => return Err(BoardingPassError::InvalidRowChar(a)),
            }
            to_add = to_add / 2;
        }

        Ok(row)
    }

    fn determine_column(input: &str) -> Result<u8, BoardingPassError> {
        let mut column: u8 = 0;
        let mut to_add: u8 = 4;
        for c in input.chars() {
            match c {
                'L' => (),
                'R' => column += to_add,
                a => return Err(BoardingPassError::InvalidColumnChar(a)),
            }
            to_add = to_add / 2;
        }
        Ok(column)
    }
}

pub fn part1() {
    let board_passes = load_boarding_passes();

    match board_passes.iter().max_by_key(|bp| bp.seat_id) {
        Some(bp) => println!("The maximum seat id is {}", bp.seat_id),
        None => println!("None boaringd pass found with a maximum id"),
    };
}

pub fn part2() {
    let mut board_passes = load_boarding_passes();
    board_passes.sort_by_key(|p| p.seat_id);

    let mut last_used_seat = None;
    for bp in board_passes {
        match last_used_seat {
            None => (),
            Some(id) => {
                if bp.seat_id == id + 2 {
                    println!("An open seat found at {}", bp.seat_id - 1);
                }
            }
        }
        last_used_seat = Some(bp.seat_id);
    }
}

fn load_boarding_passes() -> Vec<BoardingPass> {
    let input_file =
        load_input::load_strings("./resources/Day5Input.txt").expect("Failed to read the input");

    input_file
        .into_iter()
        .map(|l| BoardingPass::from(l).expect("Failed to parse a boarding pass"))
        .collect::<Vec<_>>()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_boarding_pass_ctor() {
        let input_1 = "BFFFBBFRRR".to_string();
        let input_2 = "FFFBBBFRRR".to_string();
        let input_3 = "BBFFBBFRLL".to_string();

        let pass_1 = BoardingPass::from(input_1).expect("Failed to parse pass 1");
        let pass_2 = BoardingPass::from(input_2).expect("Failed to parse pass 2");
        let pass_3 = BoardingPass::from(input_3).expect("Failed to parse pass 3");

        assert_eq!(567, pass_1.seat_id);
        assert_eq!(119, pass_2.seat_id);
        assert_eq!(820, pass_3.seat_id);
    }
}
