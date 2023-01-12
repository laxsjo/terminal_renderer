extern crate approx;

use ansi_term::*;
use itertools::Itertools;
// use ansi_term_old::Color;
// use crossterm::event::{KeyCode, KeyboardEnhancementFlags};
use crate::math::*;
// use crossterm::{cursor, terminal, QueueableCommand};
use crossterm::terminal;
#[allow(unused_imports)]
use fun::*;
use render_3d::*;
// use std::io::{stdout, Write};
use rasmus_lib::flags::TerminalFlags;
use std::num::NonZeroUsize;
// use std::panic::PanicInfo;

// pub use rasmus_lib::flags::TERMINAL_FLAGS;
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
pub const HUMAN_NUMBER_DIFFERENCE: usize = 1;

pub struct CleanUp;

pub fn clean_up() {
    // let mut stdout = stdout();

    // screen::disable_alternative_buffer();
    // flush_commands();

    // // execute!(stdout, crossterm::event::PopKeyboardEnhancementFlags)
    // //     .expect("couldn't remove keyboard enhancement flags");

    // terminal::disable_raw_mode().expect("Couldn't disable raw mode");
    // stdout
    //     .queue(cursor::Show)
    //     .expect("I hate this")
    //     .queue(cursor::DisableBlinking)
    //     .expect("I hate this")
    //     .flush()
    //     .expect("I hate this");

    cursor::show();
    format::print_format(Format::Reset);
    TerminalFlags::clean_up();
}

impl Drop for CleanUp {
    fn drop(&mut self) {
        clean_up();
    }
}

pub fn run() {
    let mut gui = ui::Gui::new();

    let fps_panel = FpsPanel::new();
    gui.add_panel(uvec2(1, 1), fps_panel);

    let debug_panel = ui::TextPanel::new("".to_owned());
    let debug_panel_ref = gui.add_panel(uvec2(20, 1), debug_panel);

    let mut scene_panel = render_3d::ScenePanel::new(render_3d::Scene::new());

    let cube_ref = scene_panel
        .scene
        .add_object(SceneObject::Object(Object::new(
            ObjMeshLoader::load(test_data::SUZANNE_SMOOTH_OBJ_FILE).expect("failed to load mesh"),
            Transform::identity(),
            hsl(0.5, 1., 0.5).into(),
            // rgb(1., 0., 0.),
        )));

    scene_panel.debug_object = Some(cube_ref);

    {
        let scene = &mut scene_panel.scene;

        let SceneObject::Object(cube) = scene.get_object_mut(cube_ref).unwrap()/*  else {
            panic!();
        } */;
        cube.transform.rotate_mut(Quaternion::from_euler_angles(
            25.0.to_radians(),
            45.0.to_radians(),
            0.,
        ));
    }
    {
        let camera = &mut scene_panel.scene.camera;

        *camera = render_3d::OrthographicCamera::new(
            render_3d::Transform::identity(),
            8.,
            8.,
            100.,
            0.01,
        );

        camera.position += vec3(0., 0., 2.);
        // camera
        //     .rotation
        //     .rotate_by_mut(Quaternion::from_axis_angle(Vec3::X_AXIS, 45.0.to_radians()));

        // clean_up();

        // let point = vec3(1., 1., -1.);
        // dbg_crlf!(camera.project_point(point));
        // panic!();
    }

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
                .as_panel_mut::<render_3d::ScenePanel>()
                .expect("couldn't convert scene_panel");

            let scene = &mut scene_panel.scene;

            let SceneObject::Object(debug_object) =
                scene.get_object(cube_ref).expect("object dissapeared!");

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

// dyn testing stuff
// pub trait Updater {
//     fn do_stuff(&self);
// }

// pub struct UpdateManager<T>
// where
//     T: Updater + ?Sized,
// {
//     pub data: T,
// }

// pub struct BigThing {
//     pub managers: Vec<Box<UpdateManager<dyn Updater + 'static>>>,
// }
