use crate::preview::PreviewContext;

use anyhow::Result;
use log::error;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct PreviewWindow {
    event_loop: Option<EventLoop<()>>,
    window: Window,
    context: PreviewContext,
}

impl PreviewWindow {
    /// Constructs and initializes the preview window.
    pub async fn create_window() -> Result<PreviewWindow> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();
        let context = PreviewContext::new(&window).await;
        Ok(PreviewWindow {
            event_loop: Some(event_loop),
            window,
            context,
        })
    }

    /// Takes control of this thread and run the window.
    pub fn run(mut self) /* -> ! */
    {
        let event_loop = self
            .event_loop
            .take()
            .expect("Should have valid event loop");
        event_loop.run(move |event, _, control_flow| {
            if let Some(flow) = self.process_window_event(event) {
                *control_flow = flow;
            }
        });
    }

    /// Processes a single event from `EventLoop`.
    fn process_window_event(&mut self, event: Event<()>) -> Option<ControlFlow> {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == self.window.id() => {
                if !self.context.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => Some(ControlFlow::Exit),
                        WindowEvent::Resized(new_size) => {
                            self.context.resize(Some(*new_size));
                            None
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            self.context.resize(Some(**new_inner_size));
                            None
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
            Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                self.context.update();
                match self.context.render() {
                    Ok(()) => None,
                    Err(wgpu::SurfaceError::Lost) => {
                        self.context.resize(None);
                        None
                    }
                    Err(wgpu::SurfaceError::OutOfMemory) => Some(ControlFlow::Exit),
                    Err(e) => {
                        error!("Render error: {}", e);
                        None
                    }
                }
            }
            Event::MainEventsCleared => {
                self.window.request_redraw();
                None
            }
            _ => None,
        }
    }
}
