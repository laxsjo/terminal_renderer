use super::queue_sequence;

/// Clear from the current cursor position to the end of the line.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn clear_to_end() {
    queue_sequence("0K");
}

/// Clear from the beginning of the current line to the cursor position.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn clear_to_start() {
    queue_sequence("1K");
}

/// Clear the current line.
///
/// **Note:** this will only queue the command. You need to call `flush_commands`
/// for it to activate.
pub fn clear() {
    queue_sequence("2K");
}
