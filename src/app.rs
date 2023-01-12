// use crate::math::*;
// use crate::render_3d::*;
// use crate::ui;

// pub struct App<'a> {
//     // pub scene: Scene,
//     // renderer: Renderer,
//     pub gui: ui::Gui,
// }

// impl App {
//     pub fn new(camera: OrthographicCamera) -> Self {
//         let mut gui = ui::Gui::new();
//         let panel = ui::Panel::new(uvec2(0, 0), ScenePanelRenderer::new(Scene::new(camera)));
//         gui.add_panel(panel);

//         Self { gui }
//     }

//     pub fn tick(&mut self) {
//         self.gui.tick();
//     }
// }
