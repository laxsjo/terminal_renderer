use super::*;
use crate::ansi_term::*;
// use crate::utils::*;

pub struct Render {
    buffer: RenderBuffer,
}

impl Render {
    pub fn new(buffer: RenderBuffer) -> Self {
        Self { buffer }
    }

    pub fn width(&self) -> u16 {
        self.buffer.width()
    }

    pub fn height(&self) -> u16 {
        self.buffer.line_count()
    }

    /// Print the contents of render to current screen position.
    ///
    /// It is capable of printing starting at any column.
    ///
    /// The cursor is returned to the starting position after printing.
    ///
    /// **Note:** due to this, the saved cursor position is eaten up by calling
    /// this function.
    ///
    pub fn print(&self, buffer: &mut RenderBuffer, column: u16) {
        buffer.push(&cursor::get_save_position());

        let mut is_first = true;
        for line in self.buffer.lines() {
            // let width = line.visual_len();

            // Make sure there is no trailing newline
            if !is_first {
                buffer.push_char('\n');
            } else {
                is_first = false;
            }

            buffer.push(&cursor::get_move_to_column(column));

            buffer.push(line);

            // buffer.push(&cursor::get_move_left(width as u16));
        }

        buffer.push(&format::get_format(Format::Reset));

        buffer.push(&cursor::get_restore_position());
    }
}
