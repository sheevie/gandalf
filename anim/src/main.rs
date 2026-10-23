use winit::event_loop::EventLoop;

mod app;
mod state;
fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = app::App::default();
    event_loop.run_app(&mut app).unwrap();
}
