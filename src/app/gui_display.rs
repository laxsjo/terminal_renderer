use crate::{app::state::*, math::*, render_3d::*, has_value_changed, dbg_value_changed};
use std::f32::consts;
use winit::{
    dpi::LogicalSize,
    event::VirtualKeyCode,
    event_loop::ControlFlow,
    // platform::unix::WindowBuilderExtUnix,
    window::WindowBuilder,
};

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;

const BUFFER_HEIGHT: u32 = 300;

pub fn run() {
    // let mut scene = Scene::new();
    let scene = super::init_scene();

    let size = LogicalSize::new(WIDTH as f32, HEIGHT as f32);
    let window_builder = WindowBuilder::new()
        .with_title("(Not so) Terminal Renderer")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .with_maximized(false);

    let state_machine = match StateMachine::new(scene, window_builder, BUFFER_HEIGHT) {
        Ok(ok) => ok,
        Err(err) => {
            panic!("state machine initialization failed: {:?}", err);
        }
    };

    let mut mouse_rotation = vec2(0., 0.);

    state_machine.run(move |state, input, _window, control_flow| {
        if input.key_pressed(winit::event::VirtualKeyCode::Escape) {
            *control_flow = ControlFlow::Exit;
            return;
        }

        let delta_s = state.delta_s();

        let _time = state.time().as_secs_f32();
        

{
        let object = state.scene.get_object_mut(state.debug_object).unwrap();

        // object.transform.rot = Quaternion::IDENTITY;

        // object.transform.rotation =
        //     Quaternion::from_euler_angles(time * 1.456_453_2, time * 1.704_831_2, time);

        dbg_value_changed!(&object.transform);
}
        let delta_mouse_pos: Vec2 =
            Vec2::from(input.mouse_diff()) / Vec2::from(state.get_window_size());
        // if has_value_changed!(&delta_mouse_pos) {
        //     println!("Mouse moved: {}", mouse_rotation);
        // }
        // if delta_mouse_pos != vec2(0., 0.) {
        //     println!("Mouse moved: {}", mouse_rotation);
        // }
        // let delta_rotation =
        //     Quaternion::from_euler_angles(delta_mouse_pos.x, delta_mouse_pos.y, 0.);

        

        if let Some(camera) = state.scene.get_camera_mut() {
            const CAMERA_SPEED: f32 = 3.;

            mouse_rotation += delta_mouse_pos;
            mouse_rotation.y = mouse_rotation.y.clamp(-0.25, 0.25);

            camera.rot = 
            // Quaternion::IDENTITY
                Quaternion::from_look_rotation(-Vec3::Z_AXIS, -Vec3::Y_AXIS)
                .rotate_axis_angle(Vec3::Y_AXIS, mouse_rotation.x * consts::PI * 2.)
                .rotate_axis_angle(Vec3::X_AXIS, -mouse_rotation.y * consts::PI * 2.);

            // Note: This assumes a workman keyboard layout. If you don't have one; To bad!
            let velocity = vec3(
                input.key_held(VirtualKeyCode::H) as i32 as f32
                    - input.key_held(VirtualKeyCode::A) as i32 as f32,
                0.,
                input.key_held(VirtualKeyCode::D) as i32 as f32
                    - input.key_held(VirtualKeyCode::S) as i32 as f32,
            ) * CAMERA_SPEED;

            camera.pos += camera.rotation().rotate_point(velocity) * delta_s;
            // println!("Moved camera {}", velocity * delta_s);

            if input.key_pressed(VirtualKeyCode::R) {
                // camera.rotation = Quaternion::identity();
                mouse_rotation = vec2(0., 0.);
                camera.pos = vec3(0., 0., 0.);
            }

            // dbg_value_changed!(&camera.pos);
            // dbg_value_changed!(&camera.rot);

        }
    })
}

/* // use crate::app::state::*;
// use crate::render_3d::*;
// use pixels::{self, Pixels, SurfaceTexture};
// use winit::{
//     dpi::LogicalSize,
//     event::{Event, WindowEvent},
//     event_loop::{ControlFlow, EventLoop},
//     // platform::unix::WindowBuilderExtUnix,
//     window::WindowBuilder,
// };

// pub fn run() -> Result<(), pixels::Error> {
//     let event_loop = EventLoop::new();

//     let window = {
//         let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
//         WindowBuilder::new()
//             .with_title("Window Test! :)")
//             // .with_x11_screen(0)
//             .with_inner_size(size)
//             .with_maximized(true)
//             .with_resizable(true)
//             .build(&event_loop)
//             .unwrap()
//     };

//     let window_size = window.inner_size();

//     let mut pixels = {
//         let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
//         Pixels::new(window_size.width, window_size.height, surface_texture)?
//     };

//     let scene = app::init_scene();
//     let mut renderer = Renderer::new(window_size.width as usize, window_size.height as usize);

//     event_loop.run(move |event, _, control_flow| {
//         if let Event::WindowEvent {
//             window_id: _,
//             event: WindowEvent::Resized(size),
//         } = event
//         {
//             println!("Resized {:?}", size);

//             renderer.resize(size.width.max(1) as usize, size.height.max(1) as usize);
//             // println!("Resized buffer");

//             // pixels.resize_buffer(size.width, size.height).unwrap();
//             // println!("Resized pixels buffer");
//             pixels.resize_surface(size.width, size.height).unwrap();
//             // println!("Resized pixels");
//         }

//         if let Event::RedrawRequested(_) = event {
//             renderer.render_scene(&scene);

//             pixels.draw_render_buffer(renderer.buffer());
//             if let Err(err) = pixels.render() {
//                 eprintln!("pixels.render() failed: {err}");
//                 *control_flow = ControlFlow::Exit;
//                 return;
//             }
//         }

//         match event {
//             Event::WindowEvent {
//                 event: WindowEvent::CloseRequested,
//                 window_id,
//             } if window_id == window.id() => *control_flow = ControlFlow::Exit,
//             _ => (),
//         }

//         window.request_redraw();
//     });

//     // Ok(())
// }
 */
