mod ascii;

use ascii::{COLON, EIGHT, FIVE, FOUR, NINE, ONE, SEVEN, SIX, THREE, TWO, ZERO};
use ctrlc;
use inquire::Text;
use std::process;
use std::thread;
use std::time::Duration;
use term_size::dimensions;

const DEFAULT_POMODORO_DURATION_MIN: u32 = 25;
const DEFAULT_BREAK_DURATION_MIN: u32 = 5;
const CATPPUCCIN_BACKGROUND: &str = "#1e1e2e";
const CATPPUCCIN_FOREGROUND: &str = "#CBA6F7";

fn main() {
    set_background_color(CATPPUCCIN_BACKGROUND);

    let running = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, std::sync::atomic::Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    clear_term();
    hide_cursor();
    let pomodoro_duration: u32 = get_duration(
        "Pomodoro duration (minutes)?",
        DEFAULT_POMODORO_DURATION_MIN,
    );
    let break_duration: u32 = get_duration("Break duration (minutes)?", DEFAULT_BREAK_DURATION_MIN);
    countdown_timer("Work time!", pomodoro_duration, &running);
    countdown_timer("Break", break_duration, &running);
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

fn hide_cursor() {
    print!("{esc}[?25l", esc = 27 as char);
}

fn show_cursor() {
    print!("{esc}[?25h", esc = 27 as char);
}

fn clear_term() {
    print!("{esc}[2J{esc}[H", esc = 27 as char);
}

fn set_background_color(hex_color: &str) {
    let ansi_color_code = hex_to_ansi(hex_color);
    print!(
        "{esc}[48;5;{code}m",
        esc = 27 as char,
        code = ansi_color_code
    );
}

fn hex_to_ansi(hex_color: &str) -> String {
    let red = u8::from_str_radix(&hex_color[1..3], 16).unwrap();
    let green = u8::from_str_radix(&hex_color[3..5], 16).unwrap();
    let blue = u8::from_str_radix(&hex_color[5..7], 16).unwrap();
    format!("\x1b[38;2;{};{};{}m", red, green, blue)
}

fn get_time(minutes: u32, seconds: u32, term_width: usize) -> String {
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
    return center_ascii(&ascii_time, term_width);
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

fn countdown_timer(
    title: &str,
    duration_mins: u32,
    running: &std::sync::Arc<std::sync::atomic::AtomicBool>,
) {
    let total_seconds = duration_mins * 60;
    let (term_width, term_height) = dimensions().unwrap_or((80, 24));
    let print = printer(CATPPUCCIN_FOREGROUND.to_owned());

    let centered_title = center_text(title, term_width, term_height);
    let centered_title_lines: Vec<&str> = centered_title.lines().collect(); // Split into lines

    for remaining_seconds in 0..total_seconds {
        clear_term();
        if !running.load(std::sync::atomic::Ordering::SeqCst) {
            let exit_text = center_text("Good job! See you soon!", term_width, term_height);
            print(exit_text);
            thread::sleep(Duration::from_secs(1));
            clear_term();
            show_cursor();
            break;
        }
        let minutes = (remaining_seconds % 3600) / 60;
        let seconds = remaining_seconds % 60;
        let time: String = get_time(minutes, seconds, term_width);
        print(centered_title.to_string());
        print(time);

        thread::sleep(Duration::from_secs(1));
    }
}

fn printer(color: String) -> impl Fn(String) {
    move |text: String| {
        if text.is_empty() {
            println!();
            return;
        }

        let hex_color = hex_to_ansi(&color);
        print!("{}", hex_color);

        for line in text.lines() {
            if line.is_empty() {
                println!();
            } else {
                let formatted_line = format!("{}\x1b[0m", line);
                println!("{}", formatted_line);
            }
        }
    }
}

fn center_text(title: &str, term_width: usize, term_height: usize) -> String {
    let horizontal_padding = (term_width - title.len()) / 2;
    let vertical_padding = (term_height - 4) / 3; // Adjust the number to control vertical padding
    let mut padded_title = String::new();

    for _ in 0..vertical_padding {
        padded_title.push('\n');
    }
    for _ in 0..horizontal_padding {
        padded_title.push(' ');
    }
    padded_title.push_str(title);
    padded_title
}
