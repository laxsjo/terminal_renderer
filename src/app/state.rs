use std::time::Duration;

use crate::{render_3d::*, utils::DeltaTimer};
use pixels::{
    wgpu::{self, RequestAdapterOptionsBase},
    Pixels, PixelsBuilder, SurfaceTexture,
};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    error::{ExternalError, OsError},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

pub struct State {
    pub scene: Scene,
    pub debug_object: ObjectId,
    time: Duration,
    delta_time: Duration,
    window_size: PhysicalSize<u32>,
}

impl State {
    pub fn time(&self) -> Duration {
        self.time
    }
    pub fn get_delta_time(&self) -> Duration {
        self.delta_time
    }

    pub fn get_time_s(&self) -> f32 {
        self.time.as_secs_f32()
    }
    pub fn delta_s(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }

    pub fn get_window_size(&self) -> PhysicalSize<u32> {
        self.window_size
    }
}

#[derive(Debug)]
pub enum StateMachineError {
    OsError(OsError),
    PixelsError(pixels::Error),
}

impl From<OsError> for StateMachineError {
    fn from(err: OsError) -> Self {
        Self::OsError(err)
    }
}

impl From<pixels::Error> for StateMachineError {
    fn from(err: pixels::Error) -> Self {
        Self::PixelsError(err)
    }
}

pub struct StateMachine {
    pub state: State,
    renderer: Renderer,
    timer: DeltaTimer,
    window: Window,
    event_loop: EventLoop<()>,
    pixels: Pixels,
    input: WinitInputHelper,
}

impl StateMachine {
    pub fn new(
        scene: Scene,
        window_builder: WindowBuilder,
        render_height: u32,
    ) -> Result<Self, StateMachineError> {
        let event_loop = EventLoop::new();
        let window = window_builder.build(&event_loop)?;
        let input = WinitInputHelper::new();

        let window_size = window.inner_size();

        let aspect_ratio = window_size.width as f32 / window_size.height as f32;
        let buffer_height = render_height;
        let buffer_size =
            PhysicalSize::new((buffer_height as f32 * aspect_ratio) as u32, buffer_height);

        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        // let pixels = Pixels::new(buffer_size.width, buffer_size.height,
        // surface_texture)?;
        let pixels = PixelsBuilder::new(buffer_size.width, buffer_size.height, surface_texture)
            .request_adapter_options(wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: true,
                compatible_surface: None,
            })
            .build()?;

        let renderer = Renderer::new(
            buffer_size.width.max(1) as usize,
            buffer_size.height.max(1) as usize,
        );

        let timer = DeltaTimer::new();

        let debug_object = scene.object_refs().next().expect("no debug object found");

        Ok(Self {
            state: State {
                scene,
                debug_object,
                time: Duration::new(0, 0),
                delta_time: Duration::new(0, 0),
                window_size,
            },
            renderer,
            timer,
            window,
            event_loop,
            pixels,
            input,
        })
    }

    pub fn run<F>(mut self, mut event_handler: F) -> !
    where
        F: 'static + FnMut(&mut State, &WinitInputHelper, &Window, &mut ControlFlow),
    {
        self.timer.restart();

        self.window.focus_window();
        // self.window.set_cursor_position(position);
        match self
            .window
            .set_cursor_grab(winit::window::CursorGrabMode::Locked)
        {
            Ok(_) => {}
            Err(err) => eprintln!("Encountered error {:?}", err),
        };

        self.event_loop.run(move |event, _, control_flow| {
            // println!("event: {:?}, window id: {:?}", event, self.window.id());
            match event {
                Event::RedrawRequested(id) if id == self.window.id() => {
                    let delta_time = self.timer.delta_time();
                    self.state.time += delta_time;
                    self.state.delta_time = delta_time;

                    let mut timer = DeltaTimer::new();
                    self.renderer.clear();
                    self.renderer.render_scene(&self.state.scene);
                    timer.restart();
                    println!("Rendered scene for {} ms", timer.delta_time().as_millis());

                    self.pixels.draw_render_buffer(self.renderer.buffer());
                    println!(
                        "Drew render buffer for {} ms",
                        timer.delta_time().as_millis()
                    );

                    timer.restart();
                    if let Err(err) = self.pixels.render() {
                        eprintln!("pixels.render() failed: {err}");
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    println!("Rendered pixels for {} ms", timer.delta_time().as_millis());
                }
                Event::WindowEvent {
                    window_id,
                    event: WindowEvent::CloseRequested,
                } if window_id == self.window.id() => {
                    // Maybe this should be moved into the WinitInputHelper section?
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                _ => {}
            }
            if self.input.update(&event) {
                // self.window
                //     .set_cursor_position(PhysicalPosition::new(100, 100))
                //     .unwrap();
                if let Some(size) = self.input.window_resized() {
                    let aspect_ratio = size.width as f32 / size.height as f32;
                    let buffer_height = self.pixels.context().texture_extent.height;
                    let buffer_size = PhysicalSize::new(
                        (buffer_height as f32 * aspect_ratio) as u32,
                        buffer_height,
                    );
                    // self.renderer
                    //     .resize(buffer_size.width as usize, buffer_size.height as usize);

                    if let Err(err) = self.pixels.resize_surface(size.width, size.height) {
                        eprintln!("self.pixels.resize_surface() failed: {err}");
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    println!("Old buffer height: {:?}", buffer_height);
                    if let Err(err) = self
                        .pixels
                        // .resize_buffer(300, 200)
                        .resize_buffer(buffer_size.width, buffer_size.height)
                    {
                        eprintln!("self.pixels.resize_buffer() failed: {err}");
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    self.state.window_size = size;

                    println!("Resized window: {:?}, buffer: {:?}", size, buffer_size);
                }

                event_handler(&mut self.state, &self.input, &self.window, control_flow);
            }
            self.window.request_redraw();
        })
    }
}
