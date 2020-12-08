use core::slice::Iter;
use std::collections::HashSet;

use crate::load_input;

#[derive(Debug)]
struct LuggageRule {
    color: String,
    contains: Option<Vec<ContainingRule>>,
}

impl LuggageRule {
    fn from(input: String) -> Option<LuggageRule> {
        let rule_in_words: Vec<&str> = input.split_whitespace().collect();
        let split_index = rule_in_words.iter().position(|w| *w == "bags");
        match split_index {
            Some(i) => {
                let start = String::new();
                let color = rule_in_words[..i]
                    .into_iter()
                    .fold(start, |w1, w2| format!("{} {}", w1, w2))
                    .trim()
                    .to_string();
                let start = i + 2;
                let contains_input = rule_in_words[start..].into_iter();
                let contains = ContainingRule::process_multiple(contains_input);

                return Some(LuggageRule { color, contains });
            }
            None => {
                println!("Failed to parse luggage rule {}", input);
                return None;
            }
        }
    }

    fn can_carry(&self, color: &str) -> bool {
        match &self.contains {
            None => false,
            Some(rules) => rules.into_iter().any(|r| r.color == color),
        }
    }
}

#[derive(Debug, Clone)]
struct ContainingRule {
    color: String,
    count: u32,
}

impl ContainingRule {
    fn from(input: &Vec<&str>) -> Option<ContainingRule> {
        let count = match input.get(0) {
            Some(n) => {
                if n == &"no" {
                    return None;
                } else {
                    match n.parse::<u32>() {
                        Ok(c) => c,
                        Err(_) => {
                            println!("Failed to get count from containing rule: {:?}", input);
                            return None;
                        }
                    }
                }
            }
            None => return None,
        };

        let start = String::new();
        let color = input[1..]
            .into_iter()
            .fold(start, |w1, w2| format!("{} {}", w1, w2))
            .trim()
            .to_string();
        Some(ContainingRule { count, color })
    }

    fn process_multiple(input: Iter<&str>) -> Option<Vec<ContainingRule>> {
        let mut rule_input: Vec<&str> = vec![];
        let mut rules: Vec<ContainingRule> = vec![];
        for word in input {
            if word.ends_with(",") || word.ends_with(".") {
                match ContainingRule::from(&rule_input) {
                    Some(r) => rules.push(r),
                    None => (),
                };
                rule_input.clear();
            } else {
                rule_input.push(word);
            }
        }
        if rules.len() == 0 {
            return None;
        } else {
            Some(rules)
        }
    }
}

struct LuggageRuleSet {
    rules: Vec<LuggageRule>,
}

impl LuggageRuleSet {
    fn from(input: Vec<String>) -> LuggageRuleSet {
        let rules = input
            .into_iter()
            .filter_map(|s| LuggageRule::from(s))
            .collect();
        LuggageRuleSet { rules }
    }

    fn bags_which_carry_direct(&self, color: &String) -> Vec<String> {
        self.rules
            .iter()
            .filter(|r| r.can_carry(&color))
            .map(|r| r.color.to_string())
            .collect()
    }

    fn all_bags_which_carry(&self, color: &String) -> HashSet<String> {
        let mut carries: HashSet<String> = HashSet::new();
        let mut new_carries: HashSet<String> =
            self.bags_which_carry_direct(color).into_iter().collect();
        while new_carries.len() > 0 {
            carries.extend(new_carries.iter().cloned());
            new_carries = new_carries
                .iter()
                .flat_map(|c| self.bags_which_carry_direct(c))
                .filter(|c| !carries.contains(c))
                .collect();
        }

        carries
    }

    fn get_containing_rules(&self, color: &String) -> Vec<ContainingRule> {
        let rules = self.rules.iter().find(|r| &r.color == color);

        match rules {
            None => vec![],
            Some(r) => match &r.contains {
                None => vec![],
                Some(c) => c.to_vec(),
            },
        }
    }

    fn containing_count(&self, color: &String) -> u32 {
        let mut rules: Vec<(u32, Vec<ContainingRule>)> =
            vec![(1, self.get_containing_rules(color))];
        let mut count: u32 = 0;

        while rules.len() > 0 {
            let mut next_rules: Vec<(u32, Vec<ContainingRule>)> = vec![];
            for (multiplier, rules) in rules {
                for rule in rules {
                    let nested_rules = self.get_containing_rules(&rule.color);
                    count += rule.count * multiplier;
                    if nested_rules.len() > 0 {
                        next_rules.push((rule.count * multiplier, nested_rules));
                    }
                }
            }
            rules = next_rules;
        }

        count
    }
}

pub fn part1() {
    let input =
        load_input::load_strings("./resources/Day7Input.txt").expect("Failed to load input");
    let rule_set = LuggageRuleSet::from(input);
    let bags = rule_set.all_bags_which_carry(&"shiny gold".to_string());
    println!("{} bags can contain a shiny gold bag", bags.len());
}

pub fn part2() {
    let input =
    load_input::load_strings("./resources/Day7Input.txt").expect("Failed to load input");
    let rule_set = LuggageRuleSet::from(input);
    let bags = rule_set.containing_count(&"shiny gold".to_string());
    println!("One shiny gold bag must contain {} other bags", bags);
}

#[cfg(test)]
pub mod test {
    use super::*;

    fn get_input_1() -> Vec<String> {
        vec![
            "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ]
    }

    fn get_input_2() -> Vec<String> {
        vec![
            "shiny gold bags contain 2 dark red bags.".to_string(),
            "dark red bags contain 2 dark orange bags.".to_string(),
            "dark orange bags contain 2 dark yellow bags.".to_string(),
            "dark yellow bags contain 2 dark green bags.".to_string(),
            "dark green bags contain 2 dark blue bags.".to_string(),
            "dark blue bags contain 2 dark violet bags.".to_string(),
            "dark violet bags contain no other bags.".to_string(),
        ]
    }

    #[test]
    fn test_can_carry_count() {
        let input = get_input_1();
        let rule_set = LuggageRuleSet::from(input);
        let bags = rule_set.all_bags_which_carry(&"shiny gold".to_string());
        assert_eq!(4, bags.len());
    }

    #[test]
    fn test_carries_count() {
        let input = get_input_2();
        let rule_set = LuggageRuleSet::from(input);
        let bags = rule_set.containing_count(&"shiny gold".to_string());
        assert_eq!(126, bags);
    }
}
