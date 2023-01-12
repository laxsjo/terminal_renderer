use super::*;

impl Renderable for str {
    fn render(&self) -> Buffer {
        Buffer {
            content: format!("{}\r\n", self),
        }
    }
}
