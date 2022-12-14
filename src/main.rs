#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]

#[macro_use] extern crate impl_ops;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::*;

pub mod graphics;
use graphics::api::GraphicsAPI;
use graphics::gl::GLContext;

// main
fn main() {
	unsafe {
		let event_loop: EventLoop<()> = EventLoop::new();
		let window = glutin::ContextBuilder::new().build_windowed(
				glutin::window::WindowBuilder::new().with_title("tetris").with_inner_size(glutin::dpi::LogicalSize::new(600., 400.)), &event_loop
			).unwrap().make_current().unwrap();

		// Sets everything up
		let mut g: Box<dyn GraphicsAPI> = Box::<GLContext>::new(GLContext::new(&window));
		g.setup();
		let img = g.load_image("./res/bg.jpg").unwrap();

		event_loop.run(move |event, _, control_flow| {
			//*control_flow = ControlFlow::Wait;
			match event {
				Event::LoopDestroyed => { return; }
				Event::MainEventsCleared => { window.window().request_redraw(); }
				Event::RedrawRequested(_) => {
					g.rect(0.0, 0.0, 0.5, 0.5);
					g.draw();
					window.swap_buffers().unwrap();
				}
				Event::WindowEvent { ref event, .. } => match event {
					WindowEvent::Resized(physical_size) => {
						window.resize(*physical_size);
					}
					WindowEvent::CloseRequested => {
						g.destroy();
						*control_flow = ControlFlow::Exit
					}
					_ => (),
				},
				_ => (),
			}
		});
	}
}

