mod ascii;

use ascii::{COLON, EIGHT, FIVE, FOUR, NINE, ONE, SEVEN, SIX, THREE, TWO, ZERO};
use inquire::Text;
use std::process;
use std::thread;
use std::time::Duration;
use term_size::dimensions;

const DEFAULT_POMODORO_DURATION_MIN: u32 = 25;
const DEFAULT_BREAK_DURATION_MIN: u32 = 5;

fn main() {
    clear_term();
    let pomodoro_duration: u32 = get_duration(
        "Pomodoro duration (minutes)?",
        DEFAULT_POMODORO_DURATION_MIN,
    );
    let break_duration: u32 = get_duration("Break duration (minutes)?", DEFAULT_BREAK_DURATION_MIN);
    countdown_timer("Work time!", pomodoro_duration);
    countdown_timer("Break", break_duration);
}

fn get_duration(prompt: &str, default: u32) -> u32 {
    let duration = Text::new(prompt)
        .with_default(default.to_string().as_str())
        .prompt();

    match duration {
        Ok(dur) => dur.parse().unwrap_or(default),
        Err(_) => process::exit(0),
    }
}

fn clear_term() {
    print!("{esc}[2J{esc}[H{esc}[48;5;234m", esc = 27 as char);
}

fn print_pomodoro_count(minutes: u32, seconds: u32, term_width: usize) {
    let timer = format!("{:02}:{:02}", minutes, seconds);
    let padding = (term_width - timer.len()) / 2;
    for _ in 0..padding {
        print!(" ");
    }
    let mut ascii_time = "".to_owned();
    for c in timer.chars() {
        match c {
            '0' => append_number(&mut ascii_time, ZERO),
            '1' => append_number(&mut ascii_time, ONE),
            '2' => append_number(&mut ascii_time, TWO),
            '3' => append_number(&mut ascii_time, THREE),
            '4' => append_number(&mut ascii_time, FOUR),
            '5' => append_number(&mut ascii_time, FIVE),
            '6' => append_number(&mut ascii_time, SIX),
            '7' => append_number(&mut ascii_time, SEVEN),
            '8' => append_number(&mut ascii_time, EIGHT),
            '9' => append_number(&mut ascii_time, NINE),
            ':' => append_number(&mut ascii_time, COLON),
            _ => {
                eprintln!("Invalid character in timer: {}", c);
                process::exit(1);
            }
        };
    }
    ascii_time = center_ascii(&ascii_time, term_width);
    println!("{}", ascii_time);
    println!();
}

fn append_number(ascii_time: &mut String, number: &str) {
    let padding = 1;
    let time_lines: Vec<&str> = ascii_time.lines().collect();
    let new_lines: Vec<&str> = number.lines().collect();
    let max_lines = time_lines.len().max(new_lines.len());
    let mut result = String::new();

    let mut new_line_width = 0;
    for i in 0..new_lines.len() {
        new_line_width = new_line_width.max(new_lines.get(i).unwrap_or(&"").len());
    }

    let mut time_lines_width = 0;
    for i in 0..time_lines.len() {
        time_lines_width = time_lines_width.max(time_lines.get(i).unwrap_or(&"").len());
    }

    new_line_width += padding;
    for i in 0..max_lines {
        let time_line = time_lines.get(i).unwrap_or(&"");
        let new_line = new_lines.get(i).unwrap_or(&"");
        let padded_time_line = format!("{:<width$}", time_line, width = time_lines_width);
        let trimmed_new_line = new_line;
        let padded_current_line = format!("{:<width$}", trimmed_new_line, width = new_line_width);

        result.push_str(&padded_time_line);
        result.push_str(&padded_current_line);
        result.push('\n');
    }

    *ascii_time = result.trim_end().to_owned(); // Trim trailing whitespace
}

fn center_ascii(ascii_time: &str, term_width: usize) -> String {
    let mut centered_ascii = String::new();
    let max_line_width = ascii_time.lines().map(|line| line.len()).max().unwrap_or(0);
    for line in ascii_time.lines() {
        // Calculate consistent padding for each line
        let padding = (term_width - max_line_width) / 2;
        let centered_line = format!("{:width$}", "", width = padding);
        centered_ascii.push_str(&centered_line);
        centered_ascii.push_str(line);
        centered_ascii.push('\n');
    }
    centered_ascii
}

fn countdown_timer(phase: &str, duration_mins: u32) {
    let total_seconds = duration_mins * 60;
    let (term_width, term_height) = dimensions().unwrap_or((80, 24));
    let title_padding = (term_width - phase.len()) / 2;
    let vertical_padding = (term_height - 4) / 3; // Adjust the number to control vertical padding

    for remaining_seconds in 0..total_seconds {
        clear_term();
        let minutes = (remaining_seconds % 3600) / 60;
        let seconds = remaining_seconds % 60;

        for _ in 0..vertical_padding {
            println!();
        }
        for _ in 0..title_padding {
            print!(" ");
        }
        println!("{}", phase);
        print_pomodoro_count(minutes, seconds, term_width);

        thread::sleep(Duration::from_secs(1));
    }
}
