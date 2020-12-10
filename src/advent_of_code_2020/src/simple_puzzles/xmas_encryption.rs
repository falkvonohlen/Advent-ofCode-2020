use crate::load_input;

struct XmasEncryptedData {
    step_size: usize,
    preamble_length: usize,
    encrypted_data: Vec<usize>,
}

impl XmasEncryptedData {
    fn from(
        encrypted_data: Vec<usize>,
        step_size: usize,
        preamble_length: usize,
    ) -> XmasEncryptedData {
        XmasEncryptedData {
            encrypted_data,
            step_size,
            preamble_length,
        }
    }

    fn get_candidats(&self, index: usize) -> Vec<usize> {
        let start = index - self.step_size;
        self.encrypted_data[start..index].to_vec()
    }

    fn is_valid(&self, index: usize) -> bool {
        let number = self.encrypted_data.get(index).expect("Index out of range");
        let candidats = self.get_candidats(index);
        for i in candidats.iter() {
            for j in candidats.iter() {
                if i == j {
                    continue;
                }
                if *number == (i + j) {
                    return true;
                }
            }
        }
        false
    }

    fn first_invalid_number(&self) -> Option<usize> {
        let range = self.preamble_length..(self.encrypted_data.len() - 1);
        for index in range {
            if !self.is_valid(index) {
                return Some(self.encrypted_data[index]);
            }
        }
        None
    }

    fn exploit_weakness(&self, invlaid_number: usize) -> Option<(usize, usize)> {
        let test_range = 2..self.encrypted_data.len() - 1;
        for length in test_range {
            let start_range = 0..(self.encrypted_data.len() - length - 1);
            for start in start_range {
                let end = start + length;
                let set = &self.encrypted_data[start..end];
                let sum: usize = set.into_iter().sum();
                if sum == invlaid_number {
                    return Some((start, end - 1));
                }
            }
        }
        None
    }

    fn get_exploit_weakness_checksum(&self, start: usize, end: usize) -> Option<usize> {
        let end = end + 1;
        let range = self.encrypted_data[start..end].to_vec();
        let max = range.iter().max();
        let min = range.iter().min();
        match (min, max) {
            (Some(num1), Some(num2)) => Some(num1 + num2),
            _ => None,
        }
    }
}

pub fn part1() {
    let input: Vec<usize> = load_input::load_usize("./resources/Day9Input.txt")
        .expect("Failed to load input")
        .into_iter()
        .map(|i| i as usize)
        .collect();

    let data = XmasEncryptedData::from(input, 25, 25);
    match data.first_invalid_number() {
        Some(i) => println!("The first invalid number found is {}", i),
        None => println!("All numbers are valid"),
    }
}

pub fn part2() {
    let input: Vec<usize> = load_input::load_usize("./resources/Day9Input.txt")
        .expect("Failed to load input")
        .into_iter()
        .map(|i| i as usize)
        .collect();

    let data = XmasEncryptedData::from(input, 25, 25);

    let invalid = match data.first_invalid_number() {
        Some(i) => i,
        None => {
            println!("All numbers are valid");
            return ();
        }
    };

    match data.exploit_weakness(invalid) {
        Some((start, end)) => match data.get_exploit_weakness_checksum(start, end) {
            Some(sum) => println!("The checksum is {}", sum),
            None => println!("Failed to compute a checksum"),
        },
        None => println!("All numbers are valid"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<usize> {
        vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ]
    }

    #[test]
    fn test_load() {
        let data = XmasEncryptedData::from(get_input(), 5, 5);
        assert_eq!(20, data.encrypted_data.len());
    }

    #[test]
    fn test_get_candidats() {
        let data = XmasEncryptedData::from(get_input(), 5, 5);
        assert_eq!(vec![35, 20, 15, 25, 47], data.get_candidats(5));
    }

    #[test]
    fn test_is_valid() {
        let data = XmasEncryptedData::from(get_input(), 5, 5);
        assert!(data.is_valid(6));
        assert!(!data.is_valid(14));
    }

    #[test]
    fn test_first_invalid_number() {
        let data = XmasEncryptedData::from(get_input(), 5, 5);
        assert_eq!(Some(127), data.first_invalid_number());
    }
    #[test]
    fn test_exploit_weakness() {
        let data = XmasEncryptedData::from(get_input(), 5, 5);
        let (start, end) = data.exploit_weakness(127).unwrap();
        assert_eq!(2, start);
        assert_eq!(5, end);
        assert_eq!(Some(62), data.get_exploit_weakness_checksum(start, end));
    }
}
