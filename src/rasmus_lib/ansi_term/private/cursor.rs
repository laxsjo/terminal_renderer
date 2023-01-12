use super::get_private_sequence;
use super::get_sequence;
use super::queue_private_sequence;
use super::queue_sequence;

pub fn get_move_home() -> String {
    get_sequence("H")
}

/// Move the cursor to the home position `(0, 0)`.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn move_home() {
    queue_sequence("H");
}

pub fn get_move_to_position(column: u16, row: u16) -> String {
    get_sequence(&format!("{};{}H", row, column))
}

/// Move the cursor to the specified row and column.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn move_to_position(column: u16, row: u16) {
    queue_sequence(&format!("{};{}H", row, column));
}

/// Move the cursor up the specified amount of lines.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn move_up(rows: u16) {
    if rows > 0 {
        queue_sequence(&format!("{}A", rows));
    }
}

/// Move the cursor down the specified amount of lines.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn move_down(rows: u16) {
    if rows > 0 {
        queue_sequence(&format!("{}B", rows));
    }
}

pub fn get_move_left(columns: u16) -> String {
    if columns > 0 {
        return get_sequence(&format!("{}D", columns));
    }
    "".to_owned()
}

/// Move the cursor the specified amount of columns to the left.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn move_left(columns: u16) {
    if columns > 0 {
        queue_sequence(&format!("{}D", columns));
    }
}

/// Move the cursor the specified amount of lines to the right.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn move_right(columns: u16) {
    if columns > 0 {
        queue_sequence(&format!("{}C", columns));
    }
}

/// Move the cursor up the specified amount of lines, while also placing it at
/// the beginning of the line.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn move_up_to_line_start(rows: u16) {
    if rows > 0 {
        queue_sequence(&format!("{}F", rows));
    }
}

/// Move the cursor down the specified amount of lines, while also placing it at
/// the beginning of the line.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn move_down_to_line_start(rows: u16) {
    if rows > 0 {
        queue_sequence(&format!("{}E", rows));
    }
}

pub fn get_move_to_column(column: u16) -> String {
    get_sequence(&format!("{}G", column))
}

/// Move the cursor to the specified absolute column.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn move_to_column(column: u16) {
    queue_sequence(&format!("{}G", column));
}

pub fn get_save_position() -> String {
    get_private_sequence("7")
}

/// Save the current cursor position for later use by `queue_restore_position`.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn save_position() {
    queue_private_sequence("7");
}

pub fn get_restore_position() -> String {
    get_private_sequence("8")
}

/// Restores the last position saved by `queue_save_position`.
///
/// I'm actually not sure what will happen if you call this without previously
/// calling `queue_save_position`... (probably nothing?)
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn restore_position() {
    queue_private_sequence("8");
}

pub fn get_show() -> String {
    get_sequence("?25h")
}

/// Makes the cursor visible again.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn show() {
    queue_sequence("?25h");
}

pub fn get_hide() -> String {
    get_sequence("?25l")
}

/// Makes the cursor invisible.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn hide() {
    queue_sequence("?25l");
}
