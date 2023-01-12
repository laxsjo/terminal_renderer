use crossterm::event::KeyCode;

use crate::ansi_term_old;
use crate::ansi_term_old::Color;
use crate::ansi_term_old::Format;
use crate::input::InputEvent;
use crate::input::INPUT;
use crate::linear_ui::*;
use crate::utils::*;
// use crate::ACCENT_COLOR;

pub struct WidgetFrame {
    pub top: String,
    pub bottom: String,
    pub left: String,
    pub right: String,
}

impl WidgetFrame {
    pub fn new(top: &str, bottom: &str, left: &str, right: &str) -> Self {
        Self {
            top: top.into(),
            bottom: bottom.into(),
            left: left.into(),
            right: right.into(),
        }
    }

    pub fn line_count(&self) -> usize {
        self.top.line_count() + self.bottom.line_count()
    }

    pub fn frame_line(&self, line: &str) -> String {
        let line_width = line.visual_len();
        let total_width = self.width();

        let extra_width = if line_width > total_width {
            0
        } else {
            total_width - line_width
        };

        let whitespace = " ".repeat(extra_width);

        format!("{}{}{}{}", self.left, line, whitespace, self.right)
    }

    fn width(&self) -> usize {
        self.top.visual_len().max(self.bottom.visual_len())
            - self.left.visual_len()
            - self.right.visual_len()
    }
}

pub struct MenuDisplay<'a, T> {
    options: MaybeBorrowed<'a, Vec<(T, String)>>,
    selected: usize,
    frame: Option<WidgetFrame>,
}

impl<'a, T> MenuDisplay<'a, T> {
    pub fn new(options: Vec<(T, String)>) -> Self {
        Self {
            options: Owned(options),
            selected: 0,
            frame: None,
        }
    }
    pub fn new_borrowed(options: &'a Vec<(T, String)>) -> Self {
        Self {
            options: Borrowed(options),
            selected: 0,
            frame: None,
        }
    }
    pub fn new_with_frame(options: Vec<(T, String)>, frame: WidgetFrame) -> Self {
        Self {
            options: Owned(options),
            selected: 0,
            frame: Some(frame),
        }
    }
    pub fn new_borrowed_with_frame(options: &'a Vec<(T, String)>, frame: WidgetFrame) -> Self {
        Self {
            options: Borrowed(options),
            selected: 0,
            frame: Some(frame),
        }
    }

    pub fn get_options(&self) -> &Vec<(T, String)> {
        match &self.options {
            Owned(options) => options,
            Borrowed(options) => options,
        }
    }

    fn iter(&self) -> impl Iterator<Item = &(T, String)> {
        match &self.options {
            Owned(options) => options.iter(),
            Borrowed(options) => options.iter(),
        }
    }
}

impl<'a, T> Renderable for MenuDisplay<'a, T> {
    fn render(&self) -> Buffer {
        let mut buffer = Buffer::new();
        if let Some(frame) = &self.frame {
            buffer.push(&frame.top);
            buffer.push("\r\n");
        }

        for (i, (_, option)) in self.iter().enumerate() {
            let formatted: String = if i == self.selected {
                format!(
                    "> {}",
                    ansi_term_old::format_string(option, &[Format::Fg(Color::Green)])
                )
            } else {
                format!("  {}", option)
            };
            match &self.frame {
                None => buffer.pushln(&formatted),
                Some(frame) => buffer.pushln(&frame.frame_line(&formatted)),
            }
        }

        if let Some(frame) = &self.frame {
            buffer.push(&frame.bottom);
            buffer.push("\r\n");
        }

        buffer
    }
}

impl<'a, T> Clearable for MenuDisplay<'a, T> {
    fn line_count(&self) -> u32 {
        self.get_options().len() as u32
            + match &self.frame {
                Some(frame) => frame.line_count(),
                None => 0,
            } as u32
    }
}

pub struct MenuResult<'a, T> {
    pub selected: &'a T,
    pub render_id: usize,
}

pub struct Menu<'a, T> {
    display: MenuDisplay<'a, T>,
}

impl<'a, T> Menu<'a, T> {
    pub fn new(options: Vec<(T, String)>) -> Self {
        Self {
            display: MenuDisplay::new(options),
        }
    }
    pub fn new_borrowed(options: &'a Vec<(T, String)>) -> Self {
        Self {
            display: MenuDisplay::new_borrowed(options),
        }
    }
    pub fn new_with_frame(options: Vec<(T, String)>, frame: (&str, &str, &str, &str)) -> Self {
        Self {
            display: MenuDisplay::new_with_frame(
                options,
                WidgetFrame::new(frame.0, frame.1, frame.2, frame.3),
            ),
        }
    }
    pub fn new_borrowed_with_frame(
        options: &'a Vec<(T, String)>,
        frame: (&str, &str, &str, &str),
    ) -> Self {
        Self {
            display: MenuDisplay::new_borrowed_with_frame(
                options,
                WidgetFrame::new(frame.0, frame.1, frame.2, frame.3),
            ),
        }
    }

    pub fn display(&mut self, scene: &mut UiScene) -> MenuResult<T> {
        let menu_rd = scene.append(self.display.render());

        INPUT.loop_input(|event| {
            match event {
                InputEvent {
                    code: KeyCode::Up, ..
                } => {
                    if self.display.selected != 0 {
                        self.display.selected -= 1;
                    }

                    scene.replace_render(menu_rd, self.display.render());
                }
                InputEvent {
                    code: KeyCode::Down,
                    ..
                } => {
                    self.display.selected += 1;
                    if self.display.selected >= self.display.get_options().len() {
                        self.display.selected = self.display.get_options().len() - 1;
                    }

                    scene.replace_render(menu_rd, self.display.render());
                }
                InputEvent {
                    code: KeyCode::Enter,
                    ..
                } => return false,
                _ => {}
            }
            true
        });

        MenuResult {
            selected: &self.display.get_options()[self.display.selected].0,
            render_id: menu_rd,
        }
    }
}
