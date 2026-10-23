use crate::state::State;
use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop};

#[derive(Default)]
pub struct App {
    state: Option<State>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.state = Some(State::new(event_loop).unwrap());
    }
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::RedrawRequested | WindowEvent::Occluded(false) => {
                if let Some(state) = self.state.as_ref() {
                    state.redraw()
                }
            }
            _ => {}
        }
    }
}
