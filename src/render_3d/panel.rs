use crate::input;

use super::*;

#[derive(Debug)]
pub struct ScenePanel {
    pub scene: Scene,
    pub renderer: Renderer,
    pub debug_object: Option<ObjectId>,
    timer: DeltaTimer,
    time: f32,
    paused: bool,
}

impl ScenePanel {
    pub fn new(scene: Scene) -> Self {
        let (mut width, height) = crossterm::terminal::size().expect("couldn't get terminal size");
        width /= 2;

        let renderer = Renderer::new(width as usize, height as usize);

        Self {
            scene,
            renderer,
            debug_object: None,
            timer: DeltaTimer::new(),
            time: 0.,
            paused: false,
        }
    }

    pub fn create_panel(pos: UVec2, scene: Scene) -> ui::PanelEntity {
        ui::PanelEntity::new(pos, Self::new(scene))
    }
}

impl ui::Panel for ScenePanel {
    fn tick(&mut self, key_events: &ui::InputStream) {
        if key_events.contains_key_code(input::KeyCode::Char(' ')) {
            self.paused = !self.paused;

            if !self.paused {
                self.timer.restart();
            }
        }

        if self.paused {
            return;
        }

        let object = self
            .scene
            .get_object_mut(self.debug_object.unwrap())
            .unwrap();

        self.time += self.timer.delta_s();

        object.transform.rotation =
            Quaternion::from_euler_angles(self.time * 2., self.time / 2., self.time);
    }
    fn render(&mut self) -> ui::Render {
        let (width, height) = self.renderer.get_size();

        self.scene
            .camera
            .set_aspect_ratio(width as f32 / height as f32);

        self.renderer.clear();
        self.renderer.render_scene(&self.scene);
        // self.renderer.render_test();
        let pixel_buffer = self.renderer.buffer();

        let mut text_buffer = ui::RenderBuffer::new();

        for row in pixel_buffer.color.iter().rev() {
            let run_length_colors = row.run_length_encoding();
            for (color, length) in run_length_colors {
                let color = color.to_byte();

                // println!("color: {:?}", color);

                text_buffer.push(&format::get_format(Format::Bg(Color::RGB(
                    color.r, color.g, color.b,
                ))));

                text_buffer.push(&" ".repeat(length * 2));
            }
            text_buffer.push(&format::get_format(Format::Reset));
            text_buffer.push("\r\n");
        }

        ui::Render::new(text_buffer)
    }
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
