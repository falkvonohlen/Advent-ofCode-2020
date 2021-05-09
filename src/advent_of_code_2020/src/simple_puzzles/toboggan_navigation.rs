use std::collections::HashMap;

use crate::load_input::load_strings;
use crate::coordination::UPoint;

enum MapElement {
    OpenSquare,
    Tree,
    Invalid,
}

struct Map {
    max_x: usize,
    max_y: usize,
    coordinates: HashMap<UPoint, MapElement>,
}

impl Map {
    fn from<T>(input: Vec<T>) -> Map
    where
        T: Into<String>,
    {
        let mut coordinates = HashMap::new();
        let mut x = 0;
        let mut y = 0;
        for line in input {
            let elements = line_to_elements(line.into());
            x = 0;
            for element in elements {
                coordinates.insert(UPoint { x, y }, element);
                x += 1;
            }
            y += 1;
        }
        Map {
            coordinates,
            max_x: x - 1,
            max_y: y - 1,
        }
    }

    fn count_trees_on_route(&self, right: usize, down: usize) -> usize {
        let mut current_pos = UPoint { x: 0, y: 0 };
        let mut tree_count = 0;
        while current_pos.y <= self.max_y {
            current_pos = current_pos.x_capped_transform(right, down, self.max_x);
            match self.coordinates.get(&current_pos) {
                Some(MapElement::Tree) => tree_count += 1,
                _ => continue,
            }
        }
        tree_count
    }
}

pub fn part1() {
    let input = load_strings("./resources/Day3Input.txt").expect("Failed to read input");
    let map = Map::from(input);
    println!(
        "Found {} trees on the given route",
        map.count_trees_on_route(3, 1)
    );
}

pub fn part2() {
    let input = load_strings("./resources/Day3Input.txt").expect("Failed to read input");
    let map = Map::from(input);
    let test_routes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut multiplied_tree_count: i64 = 1;
    for route in test_routes {
        multiplied_tree_count *= map.count_trees_on_route(route.0, route.1) as i64;
    }
    println!(
        "The result of multipling the tree count for all routes is {}",
        multiplied_tree_count
    );
}

fn line_to_elements<T>(line: T) -> Vec<MapElement>
where
    T: Into<String>,
{
    line.into()
        .chars()
        .map(|c| match c {
            '#' => MapElement::Tree,
            '.' => MapElement::OpenSquare,
            _ => MapElement::Invalid,
        })
        .collect::<Vec<MapElement>>()
}

#[cfg(test)]
pub mod tests {

    use super::*;

    fn get_input() -> Vec<String> {
        vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ]
    }

    #[test]
    fn test_tree_count() {
        let input = get_input();
        let map = Map::from(input);
        assert_eq!(2, map.count_trees_on_route(1, 1));
        assert_eq!(7, map.count_trees_on_route(3, 1));
        assert_eq!(3, map.count_trees_on_route(5, 1));
        assert_eq!(4, map.count_trees_on_route(7, 1));
        assert_eq!(2, map.count_trees_on_route(3, 2));
    }
}
