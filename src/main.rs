use std::io::stdin;

mod core;
mod display;
mod sound;
mod words;

fn take_input(size: usize) -> String {
    let mut input = String::new();
    while input.len() < size || !input.is_ascii() {
        input.clear();
        println!(">");
        stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
    }
    input
}

fn main() {
    let (solution, definition) = words::get_today_word();

    let (_, first_clue) = core::validate_guess(solution, solution[0..1].to_string().as_str());
    display::display_result(first_clue, 0);

    let mut is_finished = false;
    let mut guess_count = 1;
    let mut result;
    while !is_finished && guess_count <= 10 {
        if guess_count == 7 {
            display::hint(solution, definition);
        }
        let input = take_input(solution.len());
        (is_finished, result) = core::validate_guess(solution, &input.to_ascii_lowercase());
        display::display_result(result, guess_count);
        guess_count += 1;
    }

    display::ending_message(guess_count, solution, definition);
}
