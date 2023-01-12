// use ansi_term::*;
use super::ansi_term;
use crossterm::terminal;
use static_init::dynamic;

#[dynamic]
static mut TERMINAL_FLAGS: TerminalFlags = TerminalFlags::new();

pub struct TerminalFlags {
    raw_mode: bool,
    alternate_buffer: bool,
}

impl TerminalFlags {
    fn new() -> Self {
        Self {
            raw_mode: false,
            alternate_buffer: false,
        }
    }

    pub fn set_raw_mode(enabled: bool) {
        let mut flags = TERMINAL_FLAGS.write();
        if flags.raw_mode == enabled {
            return;
        }

        flags.raw_mode = enabled;

        if enabled {
            terminal::enable_raw_mode().expect("Couldn't enable raw mode");
        } else {
            terminal::disable_raw_mode().expect("Couldn't enable raw mode");
        }
    }

    pub fn set_alternative_buffer(enabled: bool) {
        let mut flags = TERMINAL_FLAGS.write();

        if flags.alternate_buffer == enabled {
            return;
        }

        flags.alternate_buffer = enabled;

        if enabled {
            ansi_term::screen::activate_alternative_buffer();
        } else {
            ansi_term::screen::disable_alternative_buffer();
        }
    }

    pub fn clean_up() {
        Self::set_raw_mode(false);
        Self::set_alternative_buffer(false);

        ansi_term::flush_commands();
    }
}
