use crate::{app, flags, fun::*, math::*, render_3d::*, ui};
use crossterm::terminal;
use itertools::*;
use std::num::NonZeroUsize;

pub fn run() {
    flags::TerminalFlags::set_raw_mode(true);

    let mut gui = ui::Gui::new();

    let fps_panel = FpsPanel::new();
    gui.add_panel(uvec2(1, 1), fps_panel);

    let debug_panel = ui::TextPanel::new("".to_owned());
    let debug_panel_ref = gui.add_panel(uvec2(20, 1), debug_panel);

    let mut scene_panel = ScenePanel::new(app::init_scene());

    let debug_ref = scene_panel
        .scene
        .object_refs()
        .next()
        .expect("no debug object found");

    scene_panel.debug_object = Some(debug_ref);

    let (width, height) = terminal::size().unwrap();

    scene_panel.renderer.set_size(
        NonZeroUsize::new(width as usize / 2).unwrap(),
        NonZeroUsize::new(height as usize - 5).unwrap(),
    );

    let scene_panel_ref = gui.add_panel(uvec2(1, 6), scene_panel);

    gui.add_ticker(move |panels, _inputs| {
        let normals;
        let triangles;
        {
            let scene_panel = panels
                .get_mut(&scene_panel_ref)
                .expect("couldn't get scene_panel")
                .as_panel_mut::<ScenePanel>()
                .expect("couldn't convert scene_panel");

            let scene = &mut scene_panel.scene;

            let debug_object = scene.get_object(debug_ref).expect("object dissapeared!");

            triangles = debug_object
                .mesh
                .triangles_iter()
                .map(|tri| tri.transform(&debug_object.transform))
                .take(2)
                .collect::<Vec<_>>();

            normals = triangles.iter().map(|tri| tri.normal()).collect::<Vec<_>>();
        }
        {
            let debug_panel = panels
                .get_mut(&debug_panel_ref)
                .expect("couldn't get debug_panel")
                .as_panel_mut::<ui::TextPanel>()
                .expect("couldn't convert debug_panel");

            let debug_text = normals
                .into_iter()
                .enumerate()
                .map(|(i, normal)| format!("normal {}: {: >6.3}", i, normal))
                .chain(
                    triangles
                        .into_iter()
                        .enumerate()
                        .map(|(i, tri)| format!("face {}: {: >6.3}", i, tri)),
                )
                .join("\n");

            debug_panel.set_contents(debug_text);
        }
    });

    // let alligator_panel = ui::TextPanel::new(test_data::ALLIGATOR_ART.to_owned());

    // let text = "\
    // +-----------------+\n\
    // | Hi, I'm a panel |\n\
    // +-----------------+\n";
    // let panel_a = ui::TextPanel::new(text.to_owned());

    // let text = "\
    // +------------------------+\n\
    // | Wow, I'm also that! :D |\n\
    // +------------------------+\n";
    // let panel_b = ui::TextPanel::new(text.to_owned());

    // let dyn_panel = DynamicPanel::new();

    // let counter_panel = CounterPanel::new(0);

    // gui.add_panel(uvec2(2, 3), panel_a);
    // gui.add_panel(uvec2(5, 10), panel_b);
    // gui.add_panel(uvec2(33, 3), dyn_panel);
    // gui.add_panel(uvec2(1, 4), alligator_panel);
    // gui.add_panel(uvec2(33, 20), counter_panel);

    loop {
        gui.tick();
    }
}
