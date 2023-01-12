mod app;
// pub mod events;
mod gui;
mod input_manager;
mod panel;
mod render;

pub use gui::*;
pub use input_manager::*;
pub use panel::*;
pub use render::*;

use crate::utils::StrUtils;

#[derive(Debug, Clone)]
pub struct RenderBuffer {
    content: String,
}

impl RenderBuffer {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn from_string(string: String) -> Self {
        Self { content: string }
    }

    pub fn push(&mut self, string: &str) {
        self.content.push_str(string);
    }
    pub fn pushln(&mut self, string: &str) {
        self.content.push_str(string);
        self.content.push_str("\r\n");
    }

    pub fn push_char(&mut self, char: char) {
        self.content.push(char);
    }

    pub fn push_buffer(&mut self, buffer: RenderBuffer) {
        self.content.push_str(&buffer.content);
    }

    pub fn line_count(&self) -> u16 {
        self.content.chars().filter(|char| char == &'\n').count() as u16
    }

    pub fn width(&self) -> u16 {
        self.content
            .lines()
            .map(|line| line.visual_len())
            .max()
            .unwrap_or(0) as u16
    }

    /// An iterator over the lines of a render buffer, as string slices.
    ///
    /// Lines are ended with either a newline (`\n`) or a carriage return with
    /// a line feed (`\r\n`).
    ///
    /// The final line ending is optional. A string that ends with a final line
    /// ending will return the same lines as an otherwise identical string
    /// without a final line ending.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// use terminal_renderer::ui::RenderBuffer;
    ///
    /// let buffer = RenderBuffer::from_string("foo\r\nbar\n\nbaz\n".to_owned());
    /// let mut lines = buffer.lines();
    ///
    /// assert_eq!(Some("foo"), lines.next());
    /// assert_eq!(Some("bar"), lines.next());
    /// assert_eq!(Some(""), lines.next());
    /// assert_eq!(Some("baz"), lines.next());
    ///
    /// assert_eq!(None, lines.next());
    /// ```
    pub fn lines(&self) -> std::str::Lines {
        self.content.lines()
    }

    pub fn as_no_trailing_newline(&self) -> &str {
        let s = self.content.as_str();

        let mut last_iter = self.content.char_indices().rev().take(2);

        let first_last = last_iter.next();
        let second_last = last_iter.next();

        if let Some((index, '\r')) = second_last {
            if let Some((_, '\n')) = first_last {
                return &self.content[..index];
            }
        } else if let Some((index, '\n')) = first_last {
            return &self.content[..index];
        }
        s
    }
}

impl Default for RenderBuffer {
    fn default() -> Self {
        Self::new()
    }
}
