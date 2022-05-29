#![cfg(target_os = "android")]

mod support;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
fn main() {
    println!("App started");
    let el = EventLoop::new();
    let window = WindowBuilder::new().with_title("Hello world!").build(&el).unwrap();

    // On Android a window is only available (ndk_glue::native_window() returns Some,
    // to create an EGL context on) after Resumed is received, and disappears when
    // Suspended is received.

    let mut state = None;

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        println!("{:?}", &event);

        match event {
            Event::Resumed => {
                println!("Android window available");
                let windowed_context =
                    glutin::ContextBuilder::new().build_windowed(&window).unwrap();

                let windowed_context = unsafe { windowed_context.make_current().unwrap() };

                let gl = support::load(&windowed_context);

                state = Some((windowed_context, gl));
            }
            Event::Suspended => {
                println!("Android window removed");
                // Destroy the GL context before ndk-glue releases the window back to the system
                state = None;
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                *control_flow = ControlFlow::Exit
            }
            Event::WindowEvent { event: WindowEvent::Resized(_size), .. } => {
                // if let Some((windowed_context, _)) = &state {
                //     windowed_context.resize(size);
                //     windowed_context.window().request_redraw();
                // }
            }
            Event::RedrawRequested(_) => {
                if let Some((windowed_context, gl)) = &state {
                    gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                    windowed_context.swap_buffers().unwrap();
                }
            }
            _ => {}
        }
    });
}
