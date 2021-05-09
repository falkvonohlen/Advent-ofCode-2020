use crate::{
    coordination::{CompassDirection, IPoint, StandardRotation},
    load_input::load_strings,
};

struct NavigationSystem {
    position: IPoint,
    waypoint: IPoint,
    direction: CompassDirection,
    instructions: Vec<NavigationInstruction>,
    instruction_position: usize,
}

impl NavigationSystem {
    fn from(direction: CompassDirection, input: Vec<String>) -> NavigationSystem {
        let mut instructions: Vec<NavigationInstruction> = vec![];

        for line in input {
            instructions.push(NavigationInstruction::from(line));
        }
        NavigationSystem {
            direction,
            instructions,
            instruction_position: 0,
            position: IPoint::from(0, 0),
            waypoint: IPoint::from(10, 1),
        }
    }

    fn execute_next_ship_instruction(&mut self) -> bool {
        let mut response = true;
        match self.instructions.get(self.instruction_position) {
            Some(instruction) => {
                match instruction {
                    NavigationInstruction::Move(direction, value) => {
                        self.position = self.position.move_compass(*direction, *value)
                    }
                    NavigationInstruction::Left(value) => {
                        self.direction = self.direction.rotate_left(*value)
                    }
                    NavigationInstruction::Right(value) => {
                        self.direction = self.direction.rotate_right(*value)
                    }
                    NavigationInstruction::Forward(value) => {
                        self.position = self.position.move_compass(self.direction, *value)
                    }
                };
            }
            _ => response = false,
        };
        self.instruction_position += 1;
        response
    }

    fn execute_next_waypoint_instruction(&mut self) -> bool {
        let mut response = true;
        match self.instructions.get(self.instruction_position) {
            Some(instruction) => {
                match instruction {
                    NavigationInstruction::Move(direction, value) => {
                        self.waypoint = self.waypoint.move_compass(*direction, *value)
                    }
                    NavigationInstruction::Left(value) => {
                        self.waypoint = self.waypoint.rotate_left(*value)
                    }
                    NavigationInstruction::Right(value) => {
                        self.waypoint = self.waypoint.rotate_right(*value)
                    }
                    NavigationInstruction::Forward(value) => {
                        self.position = self.position + (self.waypoint * *value);
                    }
                };
            }
            _ => response = false,
        };
        self.instruction_position += 1;
        response
    }

    fn execute_all_ship_instruction(&mut self) {
        while self.execute_next_ship_instruction() {}
    }

    fn execute_all_waypoint_instruction(&mut self) {
        while self.execute_next_waypoint_instruction() {}
    }

    fn get_manhatten_distance(&self) -> i32 {
        self.position.get_manhatten_distance(IPoint::from(0, 0))
    }
}

enum NavigationInstruction {
    Move(CompassDirection, i32),
    Left(StandardRotation),
    Right(StandardRotation),
    Forward(i32),
}

impl NavigationInstruction {
    fn from(input: String) -> NavigationInstruction {
        let value = input[1..]
            .parse::<i32>()
            .expect("Failed to parse navigation numeric value");

        match &input[..1] {
            "N" => NavigationInstruction::Move(CompassDirection::North, value),
            "S" => NavigationInstruction::Move(CompassDirection::South, value),
            "E" => NavigationInstruction::Move(CompassDirection::East, value),
            "W" => NavigationInstruction::Move(CompassDirection::West, value),
            "L" => match value {
                90 => NavigationInstruction::Left(StandardRotation::Degree90),
                180 => NavigationInstruction::Left(StandardRotation::Degree180),
                270 => NavigationInstruction::Left(StandardRotation::Degree270),
                _ => panic!("Invalid rotation instruction {}", value)
            },
            "R" => match value {
                90 => NavigationInstruction::Right(StandardRotation::Degree90),
                180 => NavigationInstruction::Right(StandardRotation::Degree180),
                270 => NavigationInstruction::Right(StandardRotation::Degree270),
                _ => panic!("Invalid rotation instruction {}", value)
            },
            "F" => NavigationInstruction::Forward(value),
            _ => panic!("Invalid navigation action input"),
        }
    }
}

pub fn part1() {
    let input = load_strings("./resources/Day12Input.txt").expect("Failed to read input");
    let mut navi = NavigationSystem::from(CompassDirection::East, input);
    navi.execute_all_ship_instruction();
    println!(
        "After executing all instruction the manhatten distance is {}.",
        navi.get_manhatten_distance()
    );
}

pub fn part2() {
    let input = load_strings("./resources/Day12Input.txt").expect("Failed to read input");
    let mut navi = NavigationSystem::from(CompassDirection::East, input);
    navi.execute_all_waypoint_instruction();
    println!(
        "After executing all instruction the manhatten distance is {}.",
        navi.get_manhatten_distance()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        vec![
            "F10".to_string(),
            "N3".to_string(),
            "F7".to_string(),
            "R90".to_string(),
            "F11".to_string(),
        ]
    }

    #[test]
    fn test_follow_instructions_ship() {
        let mut navi = NavigationSystem::from(CompassDirection::East, get_input());
        navi.execute_all_ship_instruction();
        assert_eq!(25, navi.get_manhatten_distance());
    }

    #[test]
    fn test_follow_instructions_waypoint() {
        let mut navi = NavigationSystem::from(CompassDirection::East, get_input());
        navi.execute_all_waypoint_instruction();
        assert_eq!(286, navi.get_manhatten_distance());
    }
}
