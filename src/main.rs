// there are 3 pieces that modern winit revolves around;
// 1. EventLoop - owns the OS event queue. you create one, then hand control to it by calling run_app(...)
// 2. ApplicationHandler - a trait you implement on your own struct. winit calls the methods as stuff happens.
// 3. Window - the actual OS window handle, created inside of your handler once the event loop is running.

use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, ElementState},
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
    dpi::LogicalSize,
    keyboard::{KeyCode, PhysicalKey}
};

// our application state. This holds the window once it is created,
// it's an Option bc there is a brief moment before resumed is called,
// meaning that before resume is called, technically no window exists
// therefore trying to access window will return None in that brief period

#[derive(Default, Debug)]
struct App {
    window: Option<Window>
}

// implementing ApplicationHandler
impl ApplicationHandler for App {
    // called whenever the event loop is resumed
    // we create the window
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Winit windowww yay")
            .with_inner_size(LogicalSize::new(800, 600));

        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create window");

        self.window = Some(window);
    }

    // called for literally any event on the window (input, resize, close, etc)
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        // each event here as a binding
        match event {
            // close upon request
            WindowEvent::CloseRequested => {
                println!("close has been pressed, we are not exiting");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                // draw frame if we had a renderer or smth
            }
            // close upon pressing esc
            WindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => {
                if event.state == ElementState::Pressed && event.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                    event_loop.exit();
                }
            }
            _ => {}
        }
    }

    // called only once the vent loop has proessed all other pending events for this iteration, and is about to go idle or wait for the next one
    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        // ask the window to redraw upon the next loop iteration, because
        // it will never fire on its own
        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("event loop fucked up");

    // constantly poll instead of waiting for OS events. this is a good default
    // for apps that need to render every frame. use `Wait` for GUIs that only
    // need to be redrawn upon inputs.
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

    let mut app = App::default();
    event_loop.run_app(&mut app).expect("event loop error");
}