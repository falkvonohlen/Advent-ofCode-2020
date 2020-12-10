use crate::load_input;
use regex::Regex;

#[derive(PartialEq)]
enum Height {
    Inch(i32),
    CM(i32),
    Ivalid,
    NotFound,
}

struct Passport {
    birth_year: Option<i32>,
    issue_year: Option<i32>,
    expiration_year: Option<i32>,
    height: Height,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport {
    fn from<T>(input: Vec<(T, T)>) -> Passport
    where
        T: Into<String>,
    {
        let mut birth_year: Option<i32> = None;
        let mut issue_year: Option<i32> = None;
        let mut expiration_year: Option<i32> = None;
        let mut height: Height = Height::NotFound;
        let mut hair_color: Option<String> = None;
        let mut eye_color: Option<String> = None;
        let mut passport_id: Option<String> = None;
        let mut country_id: Option<String> = None;

        for (k, v) in input {
            match k.into().as_str() {
                "byr" => {
                    birth_year = match v.into().parse::<i32>() {
                        Ok(num) => Some(num),
                        Err(_) => None,
                    }
                }
                "iyr" => {
                    issue_year = match v.into().parse::<i32>() {
                        Ok(num) => Some(num),
                        Err(_) => None,
                    }
                }
                "eyr" => {
                    expiration_year = match v.into().parse::<i32>() {
                        Ok(num) => Some(num),
                        Err(_) => None,
                    }
                }
                "hgt" => {
                    let height_string = v.into();
                    let start_index = height_string.len() - 2;
                    let unit = &height_string[start_index..];
                    let size = &height_string[..start_index].to_string().parse::<i32>();
                    height = match (unit, size) {
                        ("cm", Ok(height)) => Height::CM(*height),
                        ("in", Ok(height)) => Height::Inch(*height),
                        _ => Height::Ivalid,
                    }
                }
                "hcl" => hair_color = Some(v.into()),
                "ecl" => eye_color = Some(v.into()),
                "pid" => passport_id = Some(v.into()),
                "cid" => country_id = Some(v.into()),
                a => println!("Found unknown key {}", a),
            };
        }

        Passport {
            birth_year,
            issue_year,
            expiration_year,
            height,
            hair_color,
            eye_color,
            passport_id,
            country_id,
        }
    }

    fn parse_passport_batch(input: &[String]) -> Passport {
        let mut key_value_pairs: Vec<(&str, &str)> = vec![];
        for line in input {
            let k_v_combined = line.split_whitespace().collect::<Vec<_>>();
            for k_v in k_v_combined {
                let splitted = k_v.split(':').collect::<Vec<_>>();
                if splitted.len() == 2 {
                    key_value_pairs.push((splitted[0], splitted[1]));
                } else {
                    println!("Found a invalid key value pair: {}", k_v);
                }
            }
        }
        Passport::from(key_value_pairs)
    }

    fn has_required_fields(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height != Height::NotFound
            && self.birth_year.is_some()
            && self.birth_year.is_some()
            && self.birth_year.is_some()
            && self.birth_year.is_some()
    }

    fn is_valid(&self) -> bool {
        self.is_valid_birth_year()
            && self.is_valid_issue_year()
            && self.is_valid_expiration_year()
            && self.is_valid_height()
            && self.is_valid_hair_color()
            && self.is_valid_eye_color()
            && self.is_valid_passport_id()
            && self.is_valid_country_id()
    }

    fn is_valid_birth_year(&self) -> bool {
        match self.birth_year {
            Some(year) => year >= 1920 && year <= 2002,
            None => false,
        }
    }
    fn is_valid_issue_year(&self) -> bool {
        match self.issue_year {
            Some(year) => year >= 2010 && year <= 2020,
            None => false,
        }
    }
    fn is_valid_expiration_year(&self) -> bool {
        match self.expiration_year {
            Some(year) => year >= 2020 && year <= 2030,
            None => false,
        }
    }
    fn is_valid_height(&self) -> bool {
        match &self.height {
            Height::CM(size) => *size >= 150 && *size <= 193,
            Height::Inch(size) => *size >= 59 && *size <= 76,
            _ => false,
        }
    }
    fn is_valid_hair_color(&self) -> bool {
        let hair_regex = Regex::new("#[0-9 a-f]{6}$").expect("Invalid regular expression");
        match &self.hair_color {
            Some(color) => hair_regex.is_match(color.as_str()),
            None => false,
        }
    }
    fn is_valid_eye_color(&self) -> bool {
        let valid_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        match &self.eye_color {
            Some(color) => valid_colors.contains(&color.as_str()),
            None => false,
        }
    }
    fn is_valid_passport_id(&self) -> bool {
        match &self.passport_id {
            Some(id) => {
                if id.len() != 9 {
                    return false;
                }
                for c in id.chars() {
                    if !c.is_digit(10) {
                        return false;
                    }
                }
                true
            }
            None => false,
        }
    }
    fn is_valid_country_id(&self) -> bool {
        match self.country_id {
            Some(_) => true,
            None => true,
        }
    }
}

pub fn part1() {
    let input_file = load_input::load_empty_line_seperated("./resources/Day4Input.txt")
        .expect("Failed to read the input");

    let ports = input_file
        .iter()
        .map(|batch| Passport::parse_passport_batch(batch))
        .collect::<Vec<_>>();
    let valid_ports = ports
        .into_iter()
        .filter(|p| p.has_required_fields())
        .count();
    println!("{} passports have all required fields", valid_ports);
}

pub fn part2() {
    let input_file = load_input::load_empty_line_seperated("./resources/Day4Input.txt")
        .expect("Failed to read the input");

    let ports = input_file
        .iter()
        .map(|batch| Passport::parse_passport_batch(batch))
        .collect::<Vec<_>>();
    let valid_ports = ports.into_iter().filter(|p| p.is_valid()).count();
    println!("{} passports are valid", valid_ports);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_batch_file() {
        let input_1 = vec![
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd".to_string(),
            "byr:1937 iyr:2017 cid:147 hgt:183cm".to_string(),
        ];

        let input_2 = vec![
            "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884".to_string(),
            "hcl:#cfa07d byr:1929".to_string(),
        ];

        let input_3 = vec!["hcl:#ae17e1 iyr:2013".to_string(), "eyr:2024".to_string()];

        let input_4 = vec![
            "ecl:brn pid:760753108 byr:1931".to_string(),
            "hgt:179cm".to_string(),
        ];

        let input_5 = vec![
            "hcl:#cfa07d eyr:2025 pid:166559648".to_string(),
            "iyr:2011 ecl:brn hgt:59in".to_string(),
        ];

        let input = vec![input_1, input_2, input_3, input_4, input_5];
        let ports = input
            .iter()
            .map(|batch| Passport::parse_passport_batch(batch))
            .collect::<Vec<_>>();

        assert_eq!(
            2,
            ports
                .into_iter()
                .filter(|p| p.has_required_fields())
                .count()
        );
    }

    #[test]
    fn test_validate_passport() {
        let input = [
            "eyr:1972 cid:100".to_string(),
            "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926".to_string(),
        ];
        let port = Passport::parse_passport_batch(&input);
        assert!(!port.is_valid());

        let input = [
            "iyr:2019".to_string(),
            "hcl:#602927 eyr:1967 hgt:170cm".to_string(),
            "ecl:grn pid:012533040 byr:1946".to_string(),
        ];
        let port = Passport::parse_passport_batch(&input);
        assert!(!port.is_valid());

        let input = [
            "hcl:dab227 iyr:2012".to_string(),
            "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277".to_string(),
        ];
        let port = Passport::parse_passport_batch(&input);
        assert!(!port.is_valid());

        let input = [
            "hgt:59cm ecl:zzz".to_string(),
            "eyr:2038 hcl:74454a iyr:2023".to_string(),
            "pid:3556412378 byr:2007".to_string(),
        ];

        let port = Passport::parse_passport_batch(&input);
        assert!(!port.is_valid());

        let input = [
            "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980".to_string(),
            "hcl:#623a2f".to_string(),
        ];
        let port = Passport::parse_passport_batch(&input);
        let is_valid = port.is_valid();
        assert!(is_valid);

        let input = [
            "eyr:2029 ecl:blu cid:129 byr:1989".to_string(),
            "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm".to_string(),
        ];
        let port = Passport::parse_passport_batch(&input);
        let is_valid = port.is_valid();
        assert!(is_valid);

        let input = [
            "hcl:#888785".to_string(),
            "hgt:164cm byr:2001 iyr:2015 cid:88".to_string(),
            "pid:545766238 ecl:hzl".to_string(),
            "eyr:2022".to_string(),
        ];
        let port = Passport::parse_passport_batch(&input);
        let is_valid = port.is_valid();
        assert!(is_valid);

        let input = [
            "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719".to_string(),
        ];
        let port = Passport::parse_passport_batch(&input);
        let is_valid = port.is_valid();
        assert!(is_valid);
    }
}
