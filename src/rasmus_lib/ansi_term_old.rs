use std::io::{stdout, Write};

#[allow(dead_code)]
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

#[allow(dead_code)]
pub enum Format {
    Reset,
    Bold,
    Dim,
    Italic,
    Underline,
    Blinking,
    Reverse,
    Hidden,
    Strikethrough,
    Fg(Color),
    Bg(Color),
}

pub fn format(modifiers: &[Format]) -> String {
    let mut seperate_modifiers: Vec<String> = Vec::new();
    let mut simple_modifiers: Vec<u8> = Vec::new();

    simple_modifiers.extend(modifiers.iter().filter_map(|format| -> Option<u8> {
        match format {
            Format::Reset => Some(0),
            Format::Bold => Some(1),
            Format::Dim => Some(2),
            Format::Italic => Some(3),
            Format::Underline => Some(4),
            Format::Blinking => Some(5),
            Format::Reverse => Some(7),
            Format::Hidden => Some(8),
            Format::Strikethrough => Some(9),
            Format::Fg(color) => match color {
                Color::Black => Some(30),
                Color::Red => Some(31),
                Color::Green => Some(32),
                Color::Yellow => Some(33),
                Color::Blue => Some(34),
                Color::Magenta => Some(35),
                Color::Cyan => Some(36),
                Color::White => Some(37),
                Color::Default => Some(39),
                Color::ColorId(id) => {
                    seperate_modifiers.push(format!("\x1b[38;5;{}m", id));
                    None
                }
                Color::RGB(r, g, b) => {
                    seperate_modifiers.push(format!("\x1b[38;2;{};{};{}m", r, g, b));
                    None
                }
            },
            Format::Bg(color) => match color {
                Color::Black => Some(40),
                Color::Red => Some(41),
                Color::Green => Some(42),
                Color::Yellow => Some(43),
                Color::Blue => Some(44),
                Color::Magenta => Some(45),
                Color::Cyan => Some(46),
                Color::White => Some(47),
                Color::Default => Some(49),
                Color::ColorId(id) => {
                    seperate_modifiers.push(format!("\x1b[48;5;{}m", id));
                    None
                }
                Color::RGB(r, g, b) => {
                    seperate_modifiers.push(format!("\x1b[48;2;{};{};{}m", r, g, b));
                    None
                }
            },
        }
    }));

    let mut out = String::new();

    for modifier in seperate_modifiers {
        out.push_str(&modifier);
    }

    out.push_str(&format!(
        "\x1b[{}m",
        simple_modifiers
            .iter()
            .map(|modifier| -> String { modifier.to_string() })
            .collect::<Vec<_>>()
            .join(";")
    ));

    out
}

pub fn format_string(string: &str, modifiers: &[Format]) -> String {
    format!("{}{}\x1b[0m", format(modifiers), string)
}

pub enum CursorMovement {
    HomePosition,
    ToPosition(u32, u32),
    Up(u32),
    Down(u32),
    Right(u32),
    Left(u32),
    DownToLineStart(u32),
    UpToLineStart(u32),
    ToColumn(u32),
    UpScroll(u32),
    SavePos,
    RestorePos,
}

pub enum Command {
    EraseToScreenEnd,
    EraseToScreenStart,
    EraseScreen,
    EraseToLineEnd,
    EraseToLineStart,
    EraseLine,
    EraseUp(u32),
    EraseDown(u32),
    Move(CursorMovement),
}

pub fn perform_commands(commands: &[Command]) {
    let command_strings: Vec<String> = commands
        .iter()
        .filter_map(|command| match command {
            Command::EraseToScreenEnd => Some("[0J".to_string()),
            Command::EraseToScreenStart => Some("[1J".to_string()),
            Command::EraseScreen => Some("[2J".to_string()),
            Command::EraseToLineEnd => Some("[0K".to_string()),
            Command::EraseToLineStart => Some("[1K".to_string()),
            Command::EraseLine => Some("[2K".to_string()),
            Command::EraseUp(n) => {
                for _ in 0..*n {
                    print!("\x1b[1F\x1b[2K");
                }
                print!("\x1b[2K");
                None
            }
            Command::EraseDown(n) => {
                for _ in 0..*n {
                    print!("\x1b[1E\x1b[2K");
                }
                print!("\x1b[2K");
                None
            }
            Command::Move(movement) => match movement {
                CursorMovement::HomePosition => Some("[H".to_string()),
                CursorMovement::ToPosition(line, col) => Some(format!("[{};{}H", line, col)),
                CursorMovement::Up(n) => {
                    if *n == 0 {
                        None
                    } else {
                        Some(format!("[{}A", n))
                    }
                }
                CursorMovement::Down(n) => {
                    if *n == 0 {
                        None
                    } else {
                        Some(format!("[{}B", n))
                    }
                }
                CursorMovement::Right(n) => {
                    if *n == 0 {
                        None
                    } else {
                        Some(format!("[{}C", n))
                    }
                }
                CursorMovement::Left(n) => {
                    if *n == 0 {
                        None
                    } else {
                        Some(format!("[{}D", n))
                    }
                }
                CursorMovement::DownToLineStart(n) => {
                    if *n == 0 {
                        None
                    } else {
                        Some(format!("[{}E", n))
                    }
                }
                CursorMovement::UpToLineStart(n) => {
                    if *n == 0 {
                        None
                    } else {
                        Some(format!("[{}F", n))
                    }
                }
                CursorMovement::ToColumn(n) => Some(format!("[{}G", n)),
                CursorMovement::UpScroll(n) => {
                    for _ in 0..*n {
                        print!("\x1b M");
                    }
                    None
                }
                CursorMovement::SavePos => Some(" 7".to_string()),
                CursorMovement::RestorePos => Some(" 8".to_string()),
            },
        })
        .collect();

    for command in command_strings {
        print!("\x1b{}", command);
    }

    stdout().flush().expect("stdout flush failed");
}
