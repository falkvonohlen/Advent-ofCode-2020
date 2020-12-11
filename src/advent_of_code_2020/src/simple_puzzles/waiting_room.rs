use std::{cmp, collections::HashMap};

struct WaitingArea {
    tiles: HashMap<(u32, u32), AreaTile>,
}

impl WaitingArea {
    fn from(input: Vec<String>) -> WaitingArea {
        let mut tiles: HashMap<(u32, u32), AreaTile> = HashMap::new();
        let mut y = 0;
        for line in input {
            let mut x = 0;
            for c in line.chars() {
                tiles.insert((x, y), AreaTile::from(c, x, y));
                x += 1;
            }
            y += 1;
        }

        WaitingArea { tiles }
    }

    fn get_adjacent_seats(&self, seat: &Seat) -> Vec<&Seat> {
        let mut result: Vec<&Seat> = vec![];
        for x_pos in (cmp::max(0, seat.x_position - 1))..(seat.x_position + 1) {
            for y_pos in (cmp::max(0, seat.y_position - 1))..(seat.y_position + 1) {
                match self.tiles.get(&(x_pos, y_pos)) {
                    Some(t) => match t {
                        AreaTile::Seat(s) => result.push(s),
                        _ => (),
                    },
                    None => (),
                }
            }
        }
        result
    }

    fn prepare_transition(&mut self) {
        for tile in self.tiles.values_mut() {
            match tile {
                AreaTile::Seat(s) => {
                    s.previous_status = s.current_status.clone();
                    s.current_status = SeatStatus::Unknown;
                }
                AreaTile::Floor => (),
            }
        }
    }

    fn transition(&mut self) {
        for tile in self.tiles.values_mut() {
            match tile {
                AreaTile::Seat(seat) => {
                    let occupied = self
                        .get_adjacent_seats(seat)
                        .into_iter()
                        .filter(|s| s.previous_status == SeatStatus::Occupied);
                }
                AreaTile::Floor => (),
            }
        }
    }
}

enum AreaTile {
    Seat(Seat),
    Floor,
}

impl AreaTile {
    fn from(input: char, x_position: u32, y_position: u32) -> AreaTile {
        let seat = Seat {
            x_position,
            y_position,
            previous_status: SeatStatus::Unknown,
            current_status: match input {
                'L' => SeatStatus::Empty,
                '#' => SeatStatus::Occupied,
                _ => return AreaTile::Floor,
            },
        };
        AreaTile::Seat(seat)
    }
}

struct Seat {
    x_position: u32,
    y_position: u32,
    previous_status: SeatStatus,
    current_status: SeatStatus,
}

#[derive(Clone, PartialEq)]
enum SeatStatus {
    Empty,
    Occupied,
    Unknown,
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

    fn test_load() {}
}
