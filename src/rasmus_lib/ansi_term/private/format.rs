use super::get_sequence;
use super::queue_sequence;

#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
    ColorId(u8),
    RGB(u8, u8, u8),
}

#[derive(Clone, Copy)]
pub enum Format {
    Reset,
    Bold,
    Dim,
    Italic,
    Underline,
    BlinkingCursor,
    ReverseColors,
    Hidden,
    Strikethrough,
    Fg(Color),
    Bg(Color),
}

#[derive(Clone, Copy)]
enum ColorStyle {
    Fg(Color),
    Bg(Color),
}

enum FormatString {
    Static(&'static str),
    Dynamic(String),
}

fn get_color_string(color: ColorStyle) -> FormatString {
    match color {
        ColorStyle::Fg(Color::Black) => FormatString::Static("30m"),
        ColorStyle::Fg(Color::Red) => FormatString::Static("31m"),
        ColorStyle::Fg(Color::Green) => FormatString::Static("32m"),
        ColorStyle::Fg(Color::Yellow) => FormatString::Static("33m"),
        ColorStyle::Fg(Color::Blue) => FormatString::Static("34m"),
        ColorStyle::Fg(Color::Magenta) => FormatString::Static("35m"),
        ColorStyle::Fg(Color::Cyan) => FormatString::Static("36m"),
        ColorStyle::Fg(Color::White) => FormatString::Static("37m"),
        ColorStyle::Fg(Color::Default) => FormatString::Static("39m"),
        ColorStyle::Fg(Color::ColorId(id)) => FormatString::Dynamic(format!("38;5;{}m", id)),
        ColorStyle::Fg(Color::RGB(r, g, b)) => {
            FormatString::Dynamic(format!("38;2;{};{};{}m", r, g, b))
        }
        ColorStyle::Bg(Color::Black) => FormatString::Static("40m"),
        ColorStyle::Bg(Color::Red) => FormatString::Static("41m"),
        ColorStyle::Bg(Color::Green) => FormatString::Static("42m"),
        ColorStyle::Bg(Color::Yellow) => FormatString::Static("43m"),
        ColorStyle::Bg(Color::Blue) => FormatString::Static("44m"),
        ColorStyle::Bg(Color::Magenta) => FormatString::Static("45m"),
        ColorStyle::Bg(Color::Cyan) => FormatString::Static("46m"),
        ColorStyle::Bg(Color::White) => FormatString::Static("47m"),
        ColorStyle::Bg(Color::Default) => FormatString::Static("49m"),
        ColorStyle::Bg(Color::ColorId(id)) => FormatString::Dynamic(format!("48;5;{}m", id)),
        ColorStyle::Bg(Color::RGB(r, g, b)) => {
            FormatString::Dynamic(format!("48;2;{};{};{}m", r, g, b))
        }
    }
}

fn get_format_string(format: Format) -> FormatString {
    match format {
        Format::Reset => FormatString::Static("0m"),
        Format::Bold => FormatString::Static("1m"),
        Format::Dim => FormatString::Static("2m"),
        Format::Italic => FormatString::Static("3m"),
        Format::Underline => FormatString::Static("4m"),
        Format::BlinkingCursor => FormatString::Static("5m"),
        Format::ReverseColors => FormatString::Static("7m"),
        Format::Hidden => FormatString::Static("8m"),
        Format::Strikethrough => FormatString::Static("9m"),
        Format::Fg(color) => get_color_string(ColorStyle::Fg(color)),
        Format::Bg(color) => get_color_string(ColorStyle::Bg(color)),
    }
}

pub fn print_format(format: Format) {
    let result = get_format_string(format);
    let format_str = match result {
        FormatString::Static(str) => str,
        FormatString::Dynamic(ref string) => string.as_str(),
    };

    queue_sequence(format_str);
}

pub fn get_format(format: Format) -> String {
    let result = get_format_string(format);
    let format_str = match result {
        FormatString::Static(str) => str,
        FormatString::Dynamic(ref string) => string.as_str(),
    };

    get_sequence(format_str)
}

pub fn format_str(string: &str, formats: &[Format]) -> String {
    let mut result = String::new();

    for format in formats.iter() {
        let format_result = get_format_string(*format);
        let format_str = match format_result {
            FormatString::Static(str) => str,
            FormatString::Dynamic(ref string) => string.as_str(),
        };

        result += &get_sequence(format_str);
    }

    result += string;

    result += "\x1b[0m";

    result
}
