use std::collections::HashMap;

struct WaitingArea<'a> {
    tiles: Vec<AreaTile>,
    neighborhood: HashMap<(u32, u32), Vec<&'a Seat>>,
}

impl<'a> WaitingArea<'a> {
    fn from(input: Vec<String>) -> WaitingArea<'a> {
        let mut tiles: Vec<AreaTile> = Vec::new();
        let mut y = 0;
        for line in input {
            let mut x = 0;
            for c in line.chars() {
                tiles.push(AreaTile::from(c, x, y));
                x += 1;
            }
            y += 1;
        }

        let mut area = WaitingArea {
            tiles,
            neighborhood: HashMap::new(),
        };
        area.set_neigborhood();
        area
    }

    fn set_neigborhood(&'a mut self) {
        let positions = self
            .tiles
            .iter()
            .filter_map(|t| match t {
                AreaTile::Seat(s) => Some((s.x_position, s.y_position)),
                _ => None,
            })
            .collect::<Vec<_>>();

        for (x_position, y_position) in positions {
            let adjacent_seats: Vec<&Seat> = self
                .tiles
                .iter()
                .filter_map(|a| match a {
                    AreaTile::Seat(s) => {
                        if s.x_position >= x_position - 1
                            && s.x_position <= x_position + 1
                            && s.y_position >= y_position - 1
                            && s.y_position <= y_position + 1
                        {
                            Some(s)
                        } else {
                            None
                        }
                    }
                    _ => None,
                })
                .collect();
            self.neighborhood
                .insert((x_position, y_position), adjacent_seats);
        }
    }

    fn is_stable(&self) -> bool {
        self.tiles.iter().all(|t| match t {
            AreaTile::Seat(s) => s.current_status == s.previous_status,
            _ => true,
        })
    }

    fn prepare_transition(&mut self) {
        for tile in self.tiles.iter_mut() {
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
        for tile in self.tiles.iter_mut() {
            match tile {
                AreaTile::Seat(seat) => {
                    let adjacent_occupied = self
                        .neighborhood
                        .get(&(seat.x_position, seat.y_position))
                        .map(|seats| {
                            seats
                                .iter()
                                .filter(|s| s.previous_status == SeatStatus::Occupied)
                                .count()
                        })
                        .expect("Failed to determin occupied adjacent seats");

                    seat.current_status = match seat.previous_status {
                        SeatStatus::Occupied => {
                            if adjacent_occupied >= 4 {
                                SeatStatus::Empty
                            } else {
                                SeatStatus::Occupied
                            }
                        }
                        SeatStatus::Empty => {
                            if adjacent_occupied > 0 {
                                SeatStatus::Empty
                            } else {
                                SeatStatus::Occupied
                            }
                        }
                        _ => panic!("Unknown Seat status"),
                    }
                }
                AreaTile::Floor => (),
            }
        }
    }

    fn transition_one_round(&mut self) -> bool {
        self.prepare_transition();
        self.transition();
        self.is_stable()
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

    #[test]
    fn test_transition() {
        let mut area = WaitingArea::from(get_input());
        area.set_neigborhood();
        assert!(!area.transition_one_round());
        assert!(!area.transition_one_round());
        assert!(!area.transition_one_round());
    }
}
