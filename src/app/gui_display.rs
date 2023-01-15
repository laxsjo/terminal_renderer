use crate::app;
use crate::render_3d::*;
use pixels::{self, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    // platform::unix::WindowBuilderExtUnix,
    window::WindowBuilder,
};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 500;

pub fn run() -> Result<(), pixels::Error> {
    let event_loop = EventLoop::new();

    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Window Test! :)")
            // .with_x11_screen(0)
            .with_inner_size(size)
            .with_maximized(true)
            .with_resizable(true)
            .build(&event_loop)
            .unwrap()
    };

    let window_size = window.inner_size();

    let mut pixels = {
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(window_size.width, window_size.height, surface_texture)?
    };

    let scene = app::init_scene();
    let mut renderer = Renderer::new(window_size.width as usize, window_size.height as usize);

    event_loop.run(move |event, _, control_flow| {
        if let Event::WindowEvent {
            window_id: _,
            event: WindowEvent::Resized(size),
        } = event
        {
            println!("Resized {:?}", size);

            renderer.resize(size.width.max(1) as usize, size.height.max(1) as usize);
            // println!("Resized buffer");

            // pixels.resize_buffer(size.width, size.height).unwrap();
            // println!("Resized pixels buffer");
            pixels.resize_surface(size.width, size.height).unwrap();
            // println!("Resized pixels");
        }

        if let Event::RedrawRequested(_) = event {
            renderer.render_scene(&scene);

            pixels.draw_render_buffer(renderer.buffer());
            if let Err(err) = pixels.render() {
                eprintln!("pixels.render() failed: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }

        window.request_redraw();
    });

    // Ok(())
}
