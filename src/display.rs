use ferris_says::say;
use std::io::{stdout, BufWriter, Write};
use std::{cmp::min, thread, time::Duration};

use crate::core::{GuessResult, LetterResult};
use crate::sound;

const TIME_IN_MS_BETWEEN_LETTERS: u64 = 50;

const PERFECT_MATCH_LETTER_LIST: [char; 26] = [
    '🅰', '🅱', '🅲', '🅳', '🅴', '🅵', '🅶', '🅷', '🅸', '🅹', '🅺', '🅻', '🅼', '🅽', '🅾', '🅿', '🆀', '🆁', '🆂',
    '🆃', '🆄', '🆅', '🆆', '🆇', '🆈', '🆉',
];

const MISPLACED_LETTER_LIST: [char; 26] = [
    '🅐', '🅑', '🅒', '🅓', '🅔', '🅕', '🅖', '🅗', '🅘', '🅙', '🅚', '🅛', '🅜', '🅝', '🅞', '🅟', '🅠', '🅡', '🅢',
    '🅣', '🅤', '🅥', '🅦', '🅧', '🅨', '🅩',
];

pub fn display_result(results: &Vec<(LetterResult, char)>, guess_count: u8) {
    let audio = sound::init();
    for (_, (letter, c)) in results.iter().enumerate() {
        match letter {
            LetterResult::Perfect => {
                print!(
                    "\x1b[91m{}\x1b[0m",
                    PERFECT_MATCH_LETTER_LIST[*c as usize - 'a' as usize]
                );
                sound::play_perfect_sound(&audio);
            }
            LetterResult::Misplaced => {
                print!(
                    "\x1b[93m{}\x1b[0m",
                    MISPLACED_LETTER_LIST[*c as usize - 'a' as usize]
                );
                sound::play_misplaced_sound(&audio);
            }
            LetterResult::NotFound => {
                print!("{}", c.to_ascii_uppercase());
                sound::play_not_found_sound(&audio);
            }
        }
        print!(" ");
        stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(TIME_IN_MS_BETWEEN_LETTERS));
    }
    print!(" ({}/10)", guess_count);
    println!();
}

fn offuscate_definition(solution: &str, definition: &str) -> String {
    let replacement = "*".repeat(solution.len());

    // fixme: this is ugly
    definition
        .to_ascii_lowercase()
        .replace(solution, &replacement)
        .replace(&solution[..solution.len() - 1], &replacement)
        .replace(&solution[..solution.len() - 2], &replacement)
        .replace(&solution[..solution.len() - 3], &replacement)
}

pub fn hint(solution: &str, definition: &str) {
    let stdout = stdout();
    let message = offuscate_definition(solution, definition);
    let width = min(message.len(), 60);

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn history_to_string(history: Vec<GuessResult>) -> String {
    let mut history_str = String::new();
    for (_, result) in history.iter().enumerate() {
        for (_, (letter_result, _char)) in result.iter().enumerate() {
            match letter_result {
                LetterResult::Perfect => history_str.push('🟥'),
                LetterResult::Misplaced => history_str.push('🟡'),
                LetterResult::NotFound => history_str.push('🟦'),
            }
        }
        history_str.push('\n');
    }
    history_str
}

pub fn ending_message(
    guess_count: u8,
    solution: &str,
    definition: &str,
    history: Vec<GuessResult>,
) {
    println!(
        "\nRUSTOM {}/10\n\n{}",
        guess_count - 1,
        history_to_string(history)
    );

    let solution_and_definition = format!("{}: {}", solution, definition);
    let width = min(solution_and_definition.len(), 60);
    let stdout = stdout();
    
    let mut writer = BufWriter::new(stdout.lock());
    say(solution_and_definition.as_bytes(), width, &mut writer).unwrap();
}
