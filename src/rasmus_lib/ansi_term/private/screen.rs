use super::get_sequence;
use super::queue_sequence;

/// Save the current screen for later use by `queue_restore`.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn save() {
    queue_sequence("?47h");
}

/// Restore last screen saved by `queue_save`.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn restore() {
    queue_sequence("?47l");
}

/// Activates the alternative screen buffer.
///
/// This is a separate screen that can later be reverted by
/// `queue_disable_alternative_buffer`.
///
/// It doesn't allow scrolling, making it ideal for full screen applications.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn activate_alternative_buffer() {
    queue_sequence("?1049h");
}

/// Disables the alternative screen buffer.
///
/// The alternative buffer is a separate screen that can later be reverted by
/// this function.
///
/// It doesn't allow scrolling, making it ideal for full screen applications.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn disable_alternative_buffer() {
    queue_sequence("?1049l");
}

/// Clear from current cursor position to the screen end.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn clear_to_end() {
    queue_sequence("0J");
}

/// Clear from the screen start to current cursor position.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn clear_to_start() {
    queue_sequence("1J");
}

pub fn get_clear() -> String {
    get_sequence("2J")
}

/// Clear the entire screen.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn clear() {
    queue_sequence("2J");
}

pub fn get_fill_with_char(char: char) -> String {
    let (width, height) = crossterm::terminal::size().expect("couldn't get terminal size");
    std::iter::repeat(char)
        .take(width as usize * height as usize)
        .collect::<String>()
}

/// Fills the entire screen with `char`.
///
/// This can be used like `clear`, except that it's **a lot** faster.
/// Source: https://stackoverflow.com/a/14295787/15507414
/// Also trust me bro, I did some tests. ;)
pub fn fill_with_char(char: char) {
    let (width, height) = crossterm::terminal::size().expect("couldn't get terminal size");
    print!(
        "{}",
        std::iter::repeat(char)
            .take((width * height) as usize)
            .collect::<String>()
    );
}
