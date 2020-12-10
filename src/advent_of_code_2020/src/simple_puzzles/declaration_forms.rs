use std::collections::HashSet;

use crate::load_input;

struct DeclarationFormGrouping {
    forms: Vec<DeclarationForm>,
}

impl DeclarationFormGrouping {
    fn from(input: Vec<String>) -> DeclarationFormGrouping {
        DeclarationFormGrouping {
            forms: input
                .iter()
                .map(|line| DeclarationForm::from(line))
                .collect::<Vec<_>>(),
        }
    }

    fn get_anyones_yes_answered_questions(&self) -> HashSet<char> {
        self.forms
            .iter()
            .flat_map(|f| f.answered_yes.iter().cloned())
            .collect::<HashSet<_>>()
    }

    fn get_everyones_yes_answered_questions(&self) -> HashSet<char> {
        let yes_answers = self
            .forms
            .iter()
            .map(|f| f.answered_yes.iter().cloned().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        if yes_answers.iter().len() > 0 {
            let start: HashSet<char> = yes_answers
                .get(0)
                .unwrap()
                .into_iter()
                .cloned()
                .collect::<HashSet<_>>();
            yes_answers.into_iter().fold(start, |folded, next| {
                let next_set = next.into_iter().collect::<HashSet<_>>();
                let result = folded.intersection(&next_set).cloned().collect();
                result
            })
        } else {
            HashSet::new()
        }
    }
}

struct DeclarationForm {
    answered_yes: Vec<char>,
}

impl DeclarationForm {
    fn from(input: &String) -> DeclarationForm {
        DeclarationForm {
            answered_yes: input.chars().collect::<Vec<_>>(),
        }
    }
}

pub fn part1() {
    let forms = load_declaration_forms();
    let yes_answers: Vec<Vec<char>> = forms
        .into_iter()
        .map(|df| {
            df.get_anyones_yes_answered_questions()
                .into_iter()
                .collect()
        })
        .collect();

    let numbers: Vec<usize> = yes_answers.into_iter().map(|v| v.len()).collect();
    let summed: usize = numbers.into_iter().sum();
    println!("The sum is {}", summed);
}

pub fn part2() {
    let forms = load_declaration_forms();
    let yes_answers: Vec<Vec<char>> = forms
        .into_iter()
        .map(|df| {
            df.get_everyones_yes_answered_questions()
                .into_iter()
                .collect()
        })
        .collect();

    let numbers: Vec<usize> = yes_answers.into_iter().map(|v| v.len()).collect();
    let summed: usize = numbers.into_iter().sum();
    println!("The sum is {}", summed);
}

fn load_declaration_forms() -> Vec<DeclarationFormGrouping> {
    let input = load_input::load_empty_line_seperated("./resources/Day6Input.txt")
        .expect("Failed loading input");
    input
        .into_iter()
        .map(|input| DeclarationFormGrouping::from(input))
        .collect()
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::collections::HashSet;

    fn get_expected_output(input: Vec<char>) -> HashSet<char> {
        input.iter().cloned().collect::<HashSet<_>>()
    }

    fn get_output_anyone(input: Vec<&str>) -> HashSet<char> {
        DeclarationFormGrouping::from(input.into_iter().map(|s| s.to_string()).collect())
            .get_anyones_yes_answered_questions()
            .into_iter()
            .collect::<HashSet<_>>()
    }

    fn get_output_erveryone(input: Vec<&str>) -> HashSet<char> {
        DeclarationFormGrouping::from(input.into_iter().map(|s| s.to_string()).collect())
            .get_everyones_yes_answered_questions()
            .into_iter()
            .collect::<HashSet<_>>()
    }

    fn get_diff(output: HashSet<char>, expected_output: HashSet<char>) -> Vec<char> {
        output
            .symmetric_difference(&expected_output)
            .map(|c| *c)
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_get_anyones_yes_answered_questions() {
        let input_1 = vec!["a", "b", "c"];
        let expected_output_1 = vec!['a', 'b', 'c'];

        let input_2 = vec!["ab", "ac"];
        let expected_output_2 = vec!['a', 'b', 'c'];

        let input_3 = vec!["a", "a", "a", "a"];
        let expected_output_3 = vec!['a'];

        let input_4 = vec!["b"];
        let expected_output_4 = vec!['b'];

        assert_eq!(
            get_diff(
                get_output_anyone(input_1),
                get_expected_output(expected_output_1)
            )
            .iter()
            .count(),
            0
        );
        assert_eq!(
            get_diff(
                get_output_anyone(input_2),
                get_expected_output(expected_output_2)
            )
            .iter()
            .count(),
            0
        );
        assert_eq!(
            get_diff(
                get_output_anyone(input_3),
                get_expected_output(expected_output_3)
            )
            .iter()
            .count(),
            0
        );
        assert_eq!(
            get_diff(
                get_output_anyone(input_4),
                get_expected_output(expected_output_4)
            )
            .iter()
            .count(),
            0
        );
    }

    #[test]
    fn test_get_everyones_yes_answered_questions() {
        let input_1 = vec!["a", "b", "c"];
        let expected_output_1 = vec![];

        let input_2 = vec!["ab", "ac"];
        let expected_output_2 = vec!['a'];
        let output_2 = get_output_erveryone(input_2);

        let input_3 = vec!["a", "a", "a", "a"];
        let expected_output_3 = vec!['a'];

        let input_4 = vec!["b"];
        let expected_output_4 = vec!['b'];

        assert_eq!(
            get_diff(
                get_output_erveryone(input_1),
                get_expected_output(expected_output_1)
            )
            .iter()
            .count(),
            0
        );
        assert_eq!(
            get_diff(output_2, get_expected_output(expected_output_2))
                .iter()
                .count(),
            0
        );
        assert_eq!(
            get_diff(
                get_output_erveryone(input_3),
                get_expected_output(expected_output_3)
            )
            .iter()
            .count(),
            0
        );
        assert_eq!(
            get_diff(
                get_output_erveryone(input_4),
                get_expected_output(expected_output_4)
            )
            .iter()
            .count(),
            0
        );
    }
}
