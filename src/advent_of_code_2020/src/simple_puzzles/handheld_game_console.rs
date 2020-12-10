use crate::load_input;

#[derive(Debug, PartialEq)]
enum GameState {
    Running,
    InfiniteLoopDetection((i32, i32)),
}

struct GameConsole {
    state: GameState,
    accumulator: i32,
    operation_position: i32,
}

impl GameConsole {
    fn new() -> GameConsole {
        GameConsole {
            accumulator: 0,
            operation_position: 0,
            state: GameState::Running,
        }
    }

    fn run_code(&mut self, code: GameCode) {
        self.operation_position = 0;
        let mut executed: Vec<i32> = vec![];

        while self.operation_position < code.instructions.len() as i32
            && self.state == GameState::Running
        {
            if executed.contains(&self.operation_position) {
                self.state =
                    GameState::InfiniteLoopDetection((self.operation_position, self.accumulator));
                break;
            }
            executed.push(self.operation_position);
            self.run_operation(&code.instructions[self.operation_position as usize]);
        }
    }

    fn run_operation(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Acc(argument) => {
                self.operation_position += 1;
                self.accumulator += argument;
            }
            Instruction::Jmp(argument) => self.operation_position += argument,
            Instruction::Nop(_) => self.operation_position += 1,
        };
    }
}

#[derive(Clone)]
struct GameCode {
    instructions: Vec<Instruction>,
}

impl GameCode {
    fn from(input: Vec<String>) -> Result<GameCode, String> {
        let mut instructions = vec![];
        for instruction in input {
            let splitted = instruction.trim().split_whitespace().collect::<Vec<_>>();
            if splitted.len() == 2 {
                let operation = splitted[0];
                let argument = match splitted[1].parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => return Err(format!("Failed to parse argument: {}", instruction)),
                };
                match operation {
                    "acc" => instructions.push(Instruction::Acc(argument)),
                    "jmp" => instructions.push(Instruction::Jmp(argument)),
                    "nop" => instructions.push(Instruction::Nop(argument)),
                    _ => return Err(format!("Unknown operation: {}", instruction)),
                }
            } else {
                return Err(format!("Invalid Instruction format: {}", instruction));
            }
        }
        Ok(GameCode { instructions })
    }
}

#[derive(Clone)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

pub fn part1() {
    let input =
        load_input::load_strings("./resources/Day8Input.txt").expect("Failed to load input");
    let code = GameCode::from(input).expect("Failed to load game code");

    let mut game = GameConsole::new();

    game.run_code(code);
    println!("The game is in state {:?}", game.state);
}

pub fn part2() {
    let input =
        load_input::load_strings("./resources/Day8Input.txt").expect("Failed to load input");
    let code = GameCode::from(input).expect("Failed to load game code");

    let mut index: usize = 0;
    for i in &code.instructions {
        match i {
            Instruction::Nop(num) => {
                let mut test_code = code.clone();
                test_code.instructions[index] = Instruction::Jmp(*num);
                let mut game = GameConsole::new();
                game.run_code(test_code);
                if game.state == GameState::Running {
                    println!(
                        "Code fixed by changing instruction {} to jmp. Accumulator: {}",
                        index, game.accumulator
                    );
                    break;
                }
            }
            Instruction::Jmp(num) => {
                let mut test_code = code.clone();
                test_code.instructions[index] = Instruction::Nop(*num);
                let mut game = GameConsole::new();
                game.run_code(test_code);
                if game.state == GameState::Running {
                    println!(
                        "Code fixed by changing instruction {} to nop. Accumulator: {}",
                        index, game.accumulator
                    );
                    break;
                }
            }
            _ => (),
        }
        index += 1;
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_infinite_loop_detection() {
        let input = vec![
            "nop +0".to_string(),
            "acc +1".to_string(),
            "jmp +4".to_string(),
            "acc +3".to_string(),
            "jmp -3".to_string(),
            "acc -99".to_string(),
            "acc +1".to_string(),
            "jmp -4".to_string(),
            "acc +6".to_string(),
        ];

        let mut game = GameConsole::new();
        let code = GameCode::from(input).expect("Failed to load game code");

        game.run_code(code);
        assert_eq!(game.accumulator, 5);
    }
}
