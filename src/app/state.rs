use crate::render_3d::*;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    error::OsError,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use winit_input_helper::WinitInputHelper;

pub struct State {
    scene: Scene,
}

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
    window: Window,
    event_loop: EventLoop<()>,
    pixels: Pixels,
    input: WinitInputHelper,
}

impl StateMachine {
    pub fn new(scene: Scene, window_builder: WindowBuilder) -> Result<Self, StateMachineError> {
        let event_loop = EventLoop::new();
        let window = window_builder.build(&event_loop)?;
        let input = WinitInputHelper::new();

        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(window_size.width, window_size.height, surface_texture)?;

        let renderer = Renderer::new(
            window_size.width.max(1) as usize,
            window_size.height.max(1) as usize,
        );

        Ok(Self {
            state: State { scene },
            renderer,
            window,
            event_loop,
            pixels,
            input,
        })
    }

    pub fn run<F>(mut self, mut event_handler: F) -> !
    where
        F: 'static
            + FnMut(
                &mut State,
                &WinitInputHelper,
                // Event<'_, ()>,
                // &EventLoopWindowTarget<()>,
                &mut ControlFlow,
            ),
    {
        self.event_loop.run(move |event, _, control_flow| {
            match event {
                Event::RedrawRequested(id) if id == self.window.id() => {
                    // Redraw
                    self.renderer.render_scene(&self.state.scene);
                    self.pixels.draw_render_buffer(self.renderer.buffer())
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
                if let Some(size) = self.input.window_resized() {
                    self.renderer
                        .resize(size.width as usize, size.height as usize);

                    if let Err(err) = self.pixels.resize_surface(size.width, size.height) {
                        eprintln!("self.pixels.resize_surface() failed: {err}");
                        *control_flow = ControlFlow::Exit;
                        return;
                    }

                    if let Err(err) = self.pixels.resize_buffer(size.width, size.height) {
                        eprintln!("self.pixels.resize_buffer() failed: {err}");
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                }

                event_handler(&mut self.state, &self.input, control_flow);

                // Unsure if this should be outside if statement...
                self.window.request_redraw();
            }
        })
    }
}
