use std::io;
use std::io::Write;

pub mod cursor;
pub mod format;
pub mod line;
pub mod screen;

// pub use cursor;
// pub use macros;

pub fn get_sequence(command_str: &str) -> String {
    format!("\x1b[{}", command_str)
}

pub fn queue_sequence(command_str: &str) {
    print!("\x1b[{}", command_str);
    // io::stdout().flush().expect("failed to flush stdout");
}
pub fn get_private_sequence(command_str: &str) -> String {
    format!("\x1b{}", command_str)
    // io::stdout().flush().expect("failed to flush stdout");
}
pub fn queue_private_sequence(command_str: &str) {
    print!("\x1b{}", command_str);
    // io::stdout().flush().expect("failed to flush stdout");
}

pub fn flush_commands() {
    io::stdout().flush().expect("failed to flush stdout")
}
