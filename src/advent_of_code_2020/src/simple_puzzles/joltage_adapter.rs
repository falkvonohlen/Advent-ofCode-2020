use crate::load_input;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
struct AdapterChain {
    diff_jolt_1: usize,
    diff_jolt_2: usize,
    diff_jolt_3: usize,
    jolt_output: usize,
    available_adapters: Vec<usize>,
    used_adapters: Vec<usize>,
}

#[derive(PartialEq)]
enum AddAdapterResult {
    Successful,
    NoAdapterFound,
    NoAdapterAvailable,
}

#[derive(PartialEq, Debug)]
enum FindChainResult {
    Successful,
    IncompleteChain,
}

impl AdapterChain {
    fn from(mut input: Vec<usize>, outlet_jolt: usize) -> AdapterChain {
        input.sort();
        AdapterChain {
            diff_jolt_1: 0,
            diff_jolt_2: 0,
            diff_jolt_3: 0,
            jolt_output: outlet_jolt,
            available_adapters: input,
            used_adapters: vec![],
        }
    }

    fn get_available_next_adapters(&mut self) -> Vec<(usize, usize)> {
        let mut result: Vec<(usize, usize)> = vec![];
        let min = self.jolt_output + 1;
        let max = self.jolt_output + 3;
        let range = 0..self.available_adapters.len();
        for index in range {
            let adapter = self.available_adapters[index];
            if min <= adapter && max >= adapter {
                result.push((index, adapter));
            }
        }
        result
    }

    fn use_adapter(&mut self, index: usize, adapter: usize) {
        self.used_adapters.push(adapter);
        self.available_adapters.remove(index);
        let jolt_jump = adapter - self.jolt_output;
        match jolt_jump {
            1 => self.diff_jolt_1 += 1,
            2 => self.diff_jolt_2 += 1,
            3 => self.diff_jolt_3 += 1,
            _ => panic!("Invalid jolt jump"),
        }
        self.jolt_output = adapter;
    }

    fn use_next_adapter(&mut self) -> AddAdapterResult {
        if self.available_adapters.len() == 0 {
            return AddAdapterResult::NoAdapterAvailable;
        }

        let candidats = self.get_available_next_adapters();
        if candidats.len() == 0 {
            return AddAdapterResult::NoAdapterFound;
        }

        match candidats.iter().min_by_key(|c| c.1) {
            Some((index, adapter)) => {
                self.use_adapter(*index, *adapter);
                return AddAdapterResult::Successful;
            }
            None => AddAdapterResult::NoAdapterFound,
        }
    }

    fn get_available_adapters(&self, current_output: usize) -> Vec<usize> {
        let min = current_output + 1;
        let max = current_output + 3;
        self.available_adapters
            .iter()
            .filter(|a| *a >= &min && *a <= &max)
            .copied()
            .collect()
    }

    fn get_way_to_map(&self, current_output: usize) -> HashMap<usize, u128> {
        let mut map: HashMap<usize, u128> = HashMap::new();
        for initial in self.get_available_adapters(current_output) {
            map.insert(initial, 1);
        }

        for adapter in self.available_adapters.iter() {
            let target_count = map.get(adapter).map(|c| *c);
            match target_count {
                Some(ways_to) => {
                    for output in self.get_available_adapters(*adapter) {
                        let current_count = map.entry(output).or_insert(0);
                        *current_count += ways_to;
                    }
                }
                None => panic!("No ways lead to {}", adapter),
            }
        }

        map
    }

    fn find_chain(&mut self) -> FindChainResult {
        let mut result = AddAdapterResult::Successful;
        while result == AddAdapterResult::Successful {
            result = self.use_next_adapter();
        }

        match result {
            AddAdapterResult::NoAdapterAvailable => FindChainResult::Successful,
            AddAdapterResult::NoAdapterFound => FindChainResult::IncompleteChain,
            _ => panic!("Stopped loop to early"),
        }
    }
}

pub fn part1() {
    let input = load_input::load_usize("./resources/Day10Input.txt").expect("Failed to read input");
    let mut chain = AdapterChain::from(input, 0);
    match chain.find_chain() {
        FindChainResult::Successful => {
            println!(
                "Found a chain and with {} one jolt jumps and {} 3 jolt jumps. Multiplied: {}",
                chain.diff_jolt_1,
                chain.diff_jolt_3 + 1,
                chain.diff_jolt_1 * (chain.diff_jolt_3 + 1)
            );
        }
        _ => println!("Failed to find a chain"),
    }
}

pub fn part2() {
    let input = load_input::load_usize("./resources/Day10Input.txt").expect("Failed to read input");
    let max = *input.iter().max().expect("No max found");
    let chain = AdapterChain::from(input, 0);
    let map = chain.get_way_to_map(0);
    match map.get(&max) {
        Some(count) => println!("There are {} possible combinations", count),
        None => println!("Something went wrong"),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_input() -> Vec<usize> {
        vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ]
    }

    #[test]
    fn test_use_next_adapter() {
        let mut chain = AdapterChain::from(get_input(), 0);
        chain.use_next_adapter();
        assert_eq!(1, chain.jolt_output);
        chain.use_next_adapter();
        assert_eq!(2, chain.jolt_output);
        chain.use_next_adapter();
        assert_eq!(3, chain.jolt_output);
        assert_eq!(3, chain.diff_jolt_1);
    }

    #[test]
    fn test_find_chain() {
        let mut chain = AdapterChain::from(get_input(), 0);
        assert_eq!(FindChainResult::Successful, chain.find_chain());
        assert_eq!(22, chain.diff_jolt_1);
        assert_eq!(9, chain.diff_jolt_3);
    }

    #[test]
    fn test_get_ways_to() {
        let chain = AdapterChain::from(get_input(), 0);
        let map = chain.get_way_to_map(0);
        let count = map.get(&49);
        assert_eq!(Some(&19208), count);
    }
}
