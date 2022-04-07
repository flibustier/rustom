use std::collections::HashMap;
use unidecode::unidecode;

pub enum LetterResult {
    Perfect,
    Misplaced,
    NotFound,
}

pub fn validate_guess(raw_solution: &str, guess: &str) -> (bool, Vec<(LetterResult, char)>) {
    let mut is_finished = true;
    let mut result = vec![];
    let solution = unidecode(raw_solution);

    let misplaced_indexes = find_misplaced_indexes(&solution, guess);

    for (i, solution_char) in solution.chars().enumerate() {
        let c = guess.chars().nth(i).unwrap_or('_');

        if solution_char == c {
            result.push((LetterResult::Perfect, c));
        } else if misplaced_indexes.contains(&i) {
            result.push((LetterResult::Misplaced, c));
            is_finished = false;
        } else {
            result.push((LetterResult::NotFound, c));
            is_finished = false;
        }
    }

    (is_finished, result)
}

fn find_misplaced_indexes(solution: &String, guess: &str) -> Vec<usize> {
    let mut differences = HashMap::new();
    let mut differences_indexes = vec![];
    for (i, c) in solution.chars().enumerate() {
        if c != guess.chars().nth(i).unwrap_or(' ') {
            differences_indexes.push(i);
            differences
                .entry(c)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

    let mut misplaced_indexes = vec![];
    for i in differences_indexes {
        let c = guess.chars().nth(i).unwrap_or(' ');

        if differences.contains_key(&c) && differences.get(&c).unwrap() > &0 {
            misplaced_indexes.push(i);
            differences.entry(c).and_modify(|counter| *counter -= 1);
        }
    }

    misplaced_indexes
}
