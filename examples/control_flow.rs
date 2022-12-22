#![allow(clippy::single_match)]

use std::{thread, time};

use simple_logger::SimpleLogger;
use winit::{
    event::{Event, KeyboardInput, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopHandler, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Wait,
    WaitUntil,
    Poll,
}

const WAIT_TIME: time::Duration = time::Duration::from_millis(100);
const POLL_SLEEP_TIME: time::Duration = time::Duration::from_millis(100);

#[derive(Debug)]
struct Application {
    mode: Mode,
    request_redraw: bool,
    wait_cancelled: bool,
    close_requested: bool,
    window: Window,
}

impl EventLoopHandler for Application {
    type InitialData = ();

    fn on_init(_: Self::InitialData, event_loop: &EventLoopWindowTarget) -> Self {
        println!("Press '1' to switch to Wait mode.");
        println!("Press '2' to switch to WaitUntil mode.");
        println!("Press '3' to switch to Poll mode.");
        println!("Press 'R' to toggle request_redraw() calls.");
        println!("Press 'Esc' to close the window.");

        Self {
            mode: Mode::Wait,
            request_redraw: false,
            wait_cancelled: false,
            close_requested: false,
            window: WindowBuilder::new()
                .with_title(
                    "Press 1, 2, 3 to change control flow mode. Press R to toggle redraw requests.",
                )
                .build(event_loop)
                .unwrap(),
        }
    }

    fn on_event(
        &mut self,
        _: &EventLoopWindowTarget,
        control_flow: &mut ControlFlow,
        event: Event<'static>,
    ) {
        use winit::event::{ElementState, StartCause, VirtualKeyCode};
        println!("{:?}", event);
        match event {
            Event::NewEvents(start_cause) => {
                self.wait_cancelled = match start_cause {
                    StartCause::WaitCancelled { .. } => self.mode == Mode::WaitUntil,
                    _ => false,
                }
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    self.close_requested = true;
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(virtual_code),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => match virtual_code {
                    VirtualKeyCode::Key1 => {
                        self.mode = Mode::Wait;
                        println!("\nmode: {:?}\n", self.mode);
                    }
                    VirtualKeyCode::Key2 => {
                        self.mode = Mode::WaitUntil;
                        println!("\nmode: {:?}\n", self.mode);
                    }
                    VirtualKeyCode::Key3 => {
                        self.mode = Mode::Poll;
                        println!("\nmode: {:?}\n", self.mode);
                    }
                    VirtualKeyCode::R => {
                        self.request_redraw = !self.request_redraw;
                        println!("\nrequest_redraw: {}\n", self.request_redraw);
                    }
                    VirtualKeyCode::Escape => {
                        self.close_requested = true;
                    }
                    _ => (),
                },
                _ => (),
            },
            Event::MainEventsCleared => {
                if self.request_redraw && !self.wait_cancelled && !self.close_requested {
                    self.window.request_redraw();
                }
                if self.close_requested {
                    control_flow.set_exit();
                }
            }
            Event::RedrawRequested(_window_id) => {}
            Event::RedrawEventsCleared => {
                match self.mode {
                    Mode::Wait => control_flow.set_wait(),
                    Mode::WaitUntil => {
                        if !self.wait_cancelled {
                            control_flow.set_wait_until(instant::Instant::now() + WAIT_TIME);
                        }
                    }
                    Mode::Poll => {
                        thread::sleep(POLL_SLEEP_TIME);
                        control_flow.set_poll();
                    }
                };
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
