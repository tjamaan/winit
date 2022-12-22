#![allow(clippy::single_match)]

use simple_logger::SimpleLogger;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopHandler, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
};

#[derive(Debug)]
struct Application {
    window: Window,
}

impl EventLoopHandler for Application {
    type InitialData = ();

    fn on_init(
        _: Self::InitialData,
        event_loop: &EventLoopWindowTarget,
        _: &mut ControlFlow,
    ) -> Self {
        Self {
            window: WindowBuilder::new()
                .with_title("A fantastic window!")
                .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
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
        control_flow.set_wait();
        println!("{:?}", event);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == self.window.id() => control_flow.set_exit(),
            Event::MainEventsCleared => {
                self.window.request_redraw();
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
