extern crate approx;

use ansi_term::*;
// #[allow(unused_imports)]
// use fun::*;
use rasmus_lib::flags::TerminalFlags;

pub use rasmus_lib::input::INPUT;
pub use rasmus_lib::macros::*;
pub use rasmus_lib::*;

pub mod app;
pub mod fun;
pub mod rasmus_lib;
pub mod render_3d;
pub mod test_data;

pub const ACCENT_COLOR: Color = Color::Green;
// Mostly for debugging when you wan't to temporarily not add one to displayed indices
// Not even used for this project ¯\_(ツ)_/¯
pub const HUMAN_NUMBER_DIFFERENCE: usize = 1;

pub struct CleanUp;

pub fn clean_up() {
    cursor::show();
    format::print_format(Format::Reset);
    TerminalFlags::clean_up();
}

impl Drop for CleanUp {
    fn drop(&mut self) {
        clean_up();
    }
}
