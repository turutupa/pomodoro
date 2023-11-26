mod ascii;

use inquire::Text;
use std::process::Command;
use std::thread;
use std::time::Duration;
use termion::clear;

const DEFAULT_POMODORO_DURATION_MIN: u32 = 25;
const DEFAULT_BREAK_DURATION_MIN: u32 = 5;

fn main() {
    let pomodoro_duration: u32 = get_duration(
        "Pomodoro duration (minutes)?",
        DEFAULT_POMODORO_DURATION_MIN,
    );
    let break_duration: u32 = get_duration("Break duration (minutes)?", DEFAULT_BREAK_DURATION_MIN);

    countdown_timer("Pomodoro", pomodoro_duration);
}

fn get_duration(prompt: &str, default: u32) -> u32 {
    let duration = Text::new(prompt)
        .with_default(default.to_string().as_str())
        .prompt();

    match duration {
        Ok(dur) => dur.parse().unwrap_or(default),
        Err(_) => default,
    }
}

fn countdown_timer(phase: &str, duration_mins: u32) {
    let total_seconds = duration_mins * 60;
    for remaining_seconds in (0..total_seconds) {
        let minutes = (remaining_seconds % 3600) / 60;
        let seconds = remaining_seconds % 60;

        print!("{esc}[2J{esc}[H", esc = 27 as char); // clear term
        println!(
            "{:-^width$}\n{} Countdown: {:02}:{:02}\n{:-^width$}",
            "",
            phase,
            minutes,
            seconds,
            "",
            width = 40,
        );

        thread::sleep(Duration::from_secs(1));
    }
}
