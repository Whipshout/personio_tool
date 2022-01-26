use owo_colors::{AnsiColors, OwoColorize};

pub enum Color {
    Black,
    Yellow,
    Red,
    Green,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl Color {
    pub fn parse_color(&self) -> AnsiColors {
        match self {
            Color::Black => AnsiColors::Black,
            Color::Yellow => AnsiColors::Yellow,
            Color::Red => AnsiColors::Red,
            Color::Green => AnsiColors::Green,
            Color::Blue => AnsiColors::Blue,
            Color::Magenta => AnsiColors::Magenta,
            Color::Cyan => AnsiColors::Cyan,
            Color::White => AnsiColors::White,
        }
    }
}

pub struct Logger;

impl Logger {
    pub fn info(&self, msg: &str, text_color: Color) {
        println!("{}", msg.bold().color(text_color.parse_color()));
    }

    pub fn separator(&self, separator_text_color: Color, separator_back_color: Color) {
        println!(
            "{}",
            "--------------------------------------------------------"
                .color(separator_text_color.parse_color())
                .on_color(separator_back_color.parse_color())
        );
    }

    pub fn update_success(&self, day: &str) {
        println!(
            "{} {} {}",
            "Day".green().bold(),
            day.green().bold(),
            "is updated in the calendar".green().bold()
        );
    }

    pub fn update_fail(&self, day: &str, day_type: &str) {
        println!(
            "{} {} {} {}{}",
            "Day".red().bold(),
            day.red().bold(),
            "is".red().bold(),
            day_type.red().bold(),
            ", check in the calendar".red().bold()
        );
    }
}
