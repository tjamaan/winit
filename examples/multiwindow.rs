#![allow(clippy::single_match)]

use std::collections::HashMap;

use simple_logger::SimpleLogger;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopHandler, EventLoopWindowTarget},
    window::{Window, WindowId},
};

#[derive(Debug)]
struct Application {
    windows: HashMap<WindowId, Window>,
}

impl EventLoopHandler for Application {
    type InitialData = ();

    fn on_init(
        _: Self::InitialData,
        event_loop: &EventLoopWindowTarget,
        _: &mut ControlFlow,
    ) -> Self {
        let mut windows = HashMap::new();
        for _ in 0..3 {
            let window = Window::new(&event_loop).unwrap();
            println!("Opened a new window: {:?}", window.id());
            windows.insert(window.id(), window);
        }

        println!("Press N to open a new window.");

        Self { windows }
    }

    fn on_event(
        &mut self,
        event_loop: &EventLoopWindowTarget,
        control_flow: &mut ControlFlow,
        event: Event<'static>,
    ) {
        control_flow.set_wait();

        match event {
            Event::WindowEvent { event, window_id } => {
                match event {
                    WindowEvent::CloseRequested => {
                        println!("Window {:?} has received the signal to close", window_id);

                        // This drops the window, causing it to close.
                        self.windows.remove(&window_id);

                        if self.windows.is_empty() {
                            control_flow.set_exit();
                        }
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::N),
                                ..
                            },
                        is_synthetic: false,
                        ..
                    } => {
                        let window = Window::new(event_loop).unwrap();
                        println!("Opened a new window: {:?}", window.id());
                        self.windows.insert(window.id(), window);
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
}

fn main() {
    SimpleLogger::new().init().unwrap();

    let event_loop = EventLoop::new();
    event_loop.run_with_handler::<Application>(());
}
