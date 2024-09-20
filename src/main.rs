use winit::{
    event_loop::{EventLoop, ActiveEventLoop, ControlFlow},
    window::{Window, WindowId},
    event::WindowEvent,
    application::ApplicationHandler,
    raw_window_handle::{HasWindowHandle, WindowHandle}};

#[derive(Default)]
struct App {
    window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attrs = Window::default_attributes()
            .with_title("New Window");
        self.window = Some(event_loop.create_window(attrs).unwrap());
    }
    fn window_event(
        &mut self, 
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                if let Some(window) = &mut self.window {
                    window.request_redraw();
                    println!("{:?}", window.window_handle().expect("Failed to get window_handle"));
                }
            },
            e => {println!("{:?}", e);}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::default();

    event_loop.set_control_flow(ControlFlow::Poll);
    let _ = event_loop.run_app(&mut app);
    println!("Hello, world!");
}
