use std::{collections::HashMap, slice::Iter};

use crate::load_input::load_strings;

struct WaitingArea {
    coordinates: Vec<Point>,
    current_tiles: HashMap<Point, AreaTile>,
    previous_tiles: HashMap<Point, AreaTile>,
}

impl WaitingArea {
    fn from(input: Vec<String>) -> WaitingArea {
        let mut coordinates = vec![];
        let mut current_tiles = HashMap::new();
        let mut y = 0;
        for line in input {
            let mut x = 0;
            for c in line.chars() {
                let point = Point::from(x, y);
                coordinates.push(point);
                let tile = AreaTile::from(c);
                current_tiles.insert(point, tile);
                x += 1;
            }
            y += 1;
        }

        WaitingArea {
            coordinates,
            current_tiles,
            previous_tiles: HashMap::new(),
        }
    }

    fn is_stable(&self) -> bool {
        self.coordinates
            .iter()
            .all(|p| self.current_tiles[p] == self.previous_tiles[p])
    }

    fn prepare_transition(&mut self) {
        for p in self.coordinates.iter() {
            let entry = self
                .previous_tiles
                .entry(*p)
                .or_insert(self.current_tiles[p].clone());
            *entry = self.current_tiles[p].clone();
        }
    }

    fn transition_1(&mut self) {
        for c in self.coordinates.iter() {
            let mut adjacent_occupied = 0;
            for direction in ViewDirection::iterator(){
                adjacent_occupied += self.check_seat(c, direction);
            }

            //Update the area tile in the curent tiles hashmap at coordinate c
            *self.current_tiles.get_mut(c).unwrap() = match self.previous_tiles[c] {
                AreaTile::OccupiedSeat => {
                    if adjacent_occupied >= 4 {
                        AreaTile::EmptySeat
                    } else {
                        AreaTile::OccupiedSeat
                    }
                }
                AreaTile::EmptySeat => {
                    if adjacent_occupied > 0 {
                        AreaTile::EmptySeat
                    } else {
                        AreaTile::OccupiedSeat
                    }
                }
                AreaTile::Floor => AreaTile::Floor,
            };
        }
    }

    fn transition_2(&mut self) {
        for c in self.coordinates.iter() {
            let mut adjacent_occupied = 0;
            for direction in ViewDirection::iterator(){
                adjacent_occupied += self.check_seat_in_view(c, direction);
            }

            //Update the area tile in the curent tiles hashmap at coordinate c
            *self.current_tiles.get_mut(c).unwrap() = match self.previous_tiles[c] {
                AreaTile::OccupiedSeat => {
                    if adjacent_occupied >= 5 {
                        AreaTile::EmptySeat
                    } else {
                        AreaTile::OccupiedSeat
                    }
                }
                AreaTile::EmptySeat => {
                    if adjacent_occupied > 0 {
                        AreaTile::EmptySeat
                    } else {
                        AreaTile::OccupiedSeat
                    }
                }
                AreaTile::Floor => AreaTile::Floor,
            };
        }
    }

    fn check_seat(&self, c: &Point, direction: &ViewDirection) -> usize {
        let check = c.get_next_in_view(direction);
        let tile = self.previous_tiles.get(&check);
        return match tile {
            Some(t) => match t {
                AreaTile::OccupiedSeat => 1,
                _ => 0,
            },
            _ => 0,
        };
    }

    fn check_seat_in_view(&self, c: &Point, direction: &ViewDirection) -> usize {
        let check = c.get_next_in_view(direction);
        let tile = self.previous_tiles.get(&check);
        return match tile {
            Some(t) => match t {
                AreaTile::OccupiedSeat => 1,
                AreaTile::EmptySeat => 0,
                AreaTile::Floor => self.check_seat_in_view(&check, direction)
            },
            _ => 0,
        };
    }

    fn get_occupied_seat_count(&self) -> usize {
        self.current_tiles
            .iter()
            .filter(|(_, s)| **s == AreaTile::OccupiedSeat)
            .count()
    }

    fn transition_one_round_1(&mut self) -> bool {
        self.prepare_transition();
        self.transition_1();
        self.is_stable()
    }

    fn transition_one_round_2(&mut self) -> bool {
        self.prepare_transition();
        self.transition_2();
        self.is_stable()
    }

    fn transition_into_stable_state_1(&mut self) -> usize {
        let mut is_stable = false;
        let mut count: usize = 0;
        while !is_stable {
            count += 1;
            println!("{}", count);
            is_stable = self.transition_one_round_1();
        }
        count
    }

    fn transition_into_stable_state_2(&mut self) -> usize {
        let mut is_stable = false;
        let mut count: usize = 0;
        while !is_stable {
            count += 1;
            println!("{}", count);
            is_stable = self.transition_one_round_2();
        }
        count
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn get_next_in_view(&self, direction: &ViewDirection) -> Point {
        match direction {
            ViewDirection::Top => Point::from(self.x, self.y + 1),
            ViewDirection::TopRight => Point::from(self.x + 1, self.y + 1),
            ViewDirection::Right => Point::from(self.x + 1, self.y),
            ViewDirection::BottomRight => Point::from(self.x + 1, self.y - 1),
            ViewDirection::Bottom => Point::from(self.x, self.y - 1),
            ViewDirection::BottomLeft => Point::from(self.x - 1, self.y - 1),
            ViewDirection::Left => Point::from(self.x - 1, self.y),
            ViewDirection::TopLeft => Point::from(self.x - 1, self.y + 1),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum AreaTile {
    EmptySeat,
    OccupiedSeat,
    Floor,
}

impl AreaTile {
    fn from(input: char) -> AreaTile {
        match input {
            'L' => AreaTile::EmptySeat,
            '#' => AreaTile::OccupiedSeat,
            _ => AreaTile::Floor,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum ViewDirection {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

impl ViewDirection {
    pub fn iterator() -> Iter<'static, ViewDirection> {
        static DIRECTIONS: [ViewDirection; 8] = [
            ViewDirection::Top,
            ViewDirection::TopRight,
            ViewDirection::Right,
            ViewDirection::BottomRight,
            ViewDirection::Bottom,
            ViewDirection::BottomLeft,
            ViewDirection::Left,
            ViewDirection::TopLeft,
        ];
        DIRECTIONS.iter()
    }
}

pub fn part1() {
    let input = load_strings("./resources/Day11Input.txt").expect("Failed to read input");
    let mut area = WaitingArea::from(input);
    let transistions = area.transition_into_stable_state_1();
    let occupied_count = area.get_occupied_seat_count();
    println!(
        "Reach a stable state with {} occupied seats after {} transitions.",
        occupied_count, transistions
    );
}

pub fn part2() {
    let input = load_strings("./resources/Day11Input.txt").expect("Failed to read input");
    let mut area = WaitingArea::from(input);
    let transistions = area.transition_into_stable_state_2();
    let occupied_count = area.get_occupied_seat_count();
    println!(
        "Reach a stable state with {} occupied seats after {} transitions.",
        occupied_count, transistions
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        vec![
            "L.LL.LL.LL".to_string(),
            "LLLLLLL.LL".to_string(),
            "L.L.L..L..".to_string(),
            "LLLL.LL.LL".to_string(),
            "L.LL.LL.LL".to_string(),
            "L.LLLLL.LL".to_string(),
            "..L.L.....".to_string(),
            "LLLLLLLLLL".to_string(),
            "L.LLLLLL.L".to_string(),
            "L.LLLLL.LL".to_string(),
        ]
    }

    #[test]
    fn test_transition_1() {
        let mut area = WaitingArea::from(get_input());

        assert!(!area.transition_one_round_1());
        assert!(!area.transition_one_round_1());
        assert!(!area.transition_one_round_1());
        assert!(!area.transition_one_round_1());
        assert!(!area.transition_one_round_1()); //Here a stable state has been reached
        assert!(area.transition_one_round_1()); //Here we we know that we are in a stable state

        assert_eq!(37, area.get_occupied_seat_count());
    }

    #[test]
    fn test_transition_2() {
        let mut area = WaitingArea::from(get_input());

        assert!(!area.transition_one_round_2());
        assert!(!area.transition_one_round_2());
        assert!(!area.transition_one_round_2());
        assert!(!area.transition_one_round_2());
        assert!(!area.transition_one_round_2());
        assert!(!area.transition_one_round_2()); //Here a stable state has been reached
        assert!(area.transition_one_round_2()); //Here we we know that we are in a stable state

        assert_eq!(26, area.get_occupied_seat_count());
    }
}
