use crate::ansi_term;
use crate::input::*;
use crate::ui;
use crate::utils::DeltaTimer;
use colors_transform::{self, Color};

#[derive(Debug)]
pub struct DynamicPanel {
    timer: DeltaTimer,
    a: f32,
    b: f32,
}

impl DynamicPanel {
    pub fn new() -> Self {
        Self {
            timer: DeltaTimer::new(),
            a: 0.,
            b: 0.,
        }
    }
}

impl Default for DynamicPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ui::Panel for DynamicPanel {
    fn render(&mut self) -> ui::Render {
        let ds = self.timer.delta_s() as f32;

        self.a += ds * 2.;
        self.b += ds * 360.;
        self.b %= 360.;

        let height = (self.a.sin() * 5. + 5.) as u8;
        let color = colors_transform::Hsl::from(self.b, 80., 80.);
        let (r, g, b) = color.to_rgb().as_tuple();
        let (r, g, b) = (r as u8, g as u8, b as u8);

        let mut buffer = ui::RenderBuffer::new();
        buffer.push("+---------------------+\n");
        buffer.push("| I'm dynamic ╰(^∇^)╮ |\n");

        buffer.push(&format!("| h={: >4}              |\n", height));

        // buffer.push(&format!(
        //     "| {: >19?} |\n",
        //     &ansi_term::format::get_format(ansi_term::Format::Bg(ansi_term::Color::RGB(r, g, b))),
        // ));

        // buffer.push(&format!(
        //     "| {: >19?} |\n",
        //     &ansi_term::format::get_format(ansi_term::Format::Bold)
        // ));
        // buffer.push(&format!(
        //     "| {: >19?} |\n",
        //     &ansi_term::format::get_format(ansi_term::Format::Reset)
        // ));

        for i in 0..10 {
            buffer.push("|      ");

            if i >= 10 - height {
                // buffer.push(&ansi_term::format::get_format(ansi_term::Format::Bg(
                //     ansi_term::Color::RGB(r as u8, g as u8, b as u8),
                // )));
                buffer.push(&ansi_term::format::get_format(ansi_term::Format::Bg(
                    ansi_term::Color::RGB(r, g, b),
                )));
                buffer.push("         ");
                // buffer.push(&ansi_term::format::get_format(ansi_term::Format::Reset));
                buffer.push(&ansi_term::format::get_format(ansi_term::Format::Reset));
            } else {
                buffer.push("         ");
            }

            buffer.push("      |\n");
        }

        buffer.pushln("+---------------------+");

        ui::Render::new(buffer)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
pub struct FpsPanel {
    timer: DeltaTimer,
}

impl FpsPanel {
    pub fn new() -> Self {
        Self {
            timer: DeltaTimer::new(),
        }
    }
}
impl Default for FpsPanel {
    fn default() -> Self {
        Self::new()
    }
}

impl ui::Panel for FpsPanel {
    fn render(&mut self) -> ui::Render {
        let delta_s = self.timer.delta_s();

        let fps = 1. / delta_s;

        let mut buffer = ui::RenderBuffer::new();

        buffer.pushln("+------------+");
        buffer.pushln(&format!("| fps: {: <6.2}|", fps));
        buffer.pushln("+------------+");

        ui::Render::new(buffer)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[derive(Debug)]
pub struct CounterPanel {
    count: i32,
}

impl CounterPanel {
    pub fn new(count: i32) -> Self {
        Self { count }
    }
}

impl ui::Panel for CounterPanel {
    fn tick(&mut self, key_events: &ui::InputStream) {
        if key_events.contains(&InputEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::NONE,
        }) {
            self.count += 1;
        } else if key_events.contains(&InputEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::NONE,
        }) {
            self.count -= 1;
        } else if key_events.contains(&InputEvent {
            code: KeyCode::Up,
            modifiers: KeyModifiers::SHIFT,
        }) {
            self.count += 10;
        } else if key_events.contains(&InputEvent {
            code: KeyCode::Down,
            modifiers: KeyModifiers::SHIFT,
        }) {
            self.count -= 10;
        }
    }

    fn render(&mut self) -> ui::Render {
        let mut buffer = ui::RenderBuffer::new();

        buffer.pushln(&format!("Count: {}", self.count));
        buffer.pushln("---Controls---");
        buffer.pushln("[Up]: +1");
        buffer.pushln("[Down]: -1");
        buffer.pushln("[Shift+Up]: +10");
        buffer.pushln("[Shift+Down]: -10");

        ui::Render::new(buffer)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
