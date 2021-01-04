use std::collections::HashMap;

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
                current_tiles.insert(point, AreaTile::from(c));
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

    fn transition(&mut self) {
        for c in self.coordinates.iter() {
            let neighbors: Vec<&AreaTile> = self
                .previous_tiles
                .iter()
                .filter_map(|(p, a)| {
                    if p.x >= c.x - 1 && p.x <= c.x + 1 && p.y >= c.y - 1 && p.y <= c.y + 1 {
                        Some(a)
                    } else {
                        None
                    }
                })
                .collect();

            let adjacent_occupied = neighbors
                .into_iter()
                .filter(|s| **s == AreaTile::OccupiedSeat)
                .count();

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
                _ => panic!("Unknown Seat status"),
            };
        }
    }

    fn get_occupied_seat_count(&self) -> usize {
        self.current_tiles
            .iter()
            .filter(|(_, s)| **s == AreaTile::OccupiedSeat)
            .count()
    }

    fn print_area(&self) {
        let max_x = self.coordinates.iter().map(|p| p.x).max().unwrap();
        let max_y = self.coordinates.iter().map(|p| p.y).max().unwrap();
        println!("Current Area:");
        for x in 0..max_x {
            for y in 0..max_y{
                let p = Point{x,y};
                match self.current_tiles[&p] {
                    AreaTile::OccupiedSeat => print!("#"),
                    AreaTile::EmptySeat => print!("L"),
                    AreaTile::Floor => print!("."),
                    AreaTile::Unknown => print!("E"),
                }
            }
            println!("")
        }
    }

    fn transition_one_round(&mut self) -> bool {
        self.prepare_transition();
        self.transition();
        self.print_area();
        self.is_stable()
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
}

#[derive(Clone, PartialEq)]
enum AreaTile {
    EmptySeat,
    OccupiedSeat,
    Floor,
    Unknown,
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
    fn test_transition() {
        let mut area = WaitingArea::from(get_input());
        assert!(!area.transition_one_round());
        assert!(!area.transition_one_round());
        assert!(!area.transition_one_round());
        assert!(!area.transition_one_round());
        assert!(!area.transition_one_round());
    }
}
