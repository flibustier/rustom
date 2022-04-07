use ferris_says::say;
use std::io::{stdout, BufWriter, Write};
use std::{cmp::min, thread, time::Duration};

use crate::core::LetterResult;
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

pub fn display_result(results: Vec<(LetterResult, char)>, guess_count: u8) {
    let audio = sound::init();
    for (letter, c) in results {
        match letter {
            LetterResult::Perfect => {
                print!(
                    "\x1b[91m{}\x1b[0m",
                    PERFECT_MATCH_LETTER_LIST[c as usize - 'a' as usize]
                );
                sound::play_perfect_sound(&audio);
            }
            LetterResult::Misplaced => {
                print!(
                    "\x1b[93m{}\x1b[0m",
                    MISPLACED_LETTER_LIST[c as usize - 'a' as usize]
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

pub fn ending_message(guess_count: u8, solution: &str, definition: &str) {
    let stdout = stdout();
    let message = format!(
        "RUSTOM {}/10\n{}: {}",
        guess_count - 1,
        solution,
        definition
    );
    let width = min(message.len(), 60);

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
