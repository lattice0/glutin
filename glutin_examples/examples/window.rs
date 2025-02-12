mod support;

use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

fn main() {
    let el = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        // .with_inner_size(PhysicalSize::<u32>::new(500u32, 500u32))
        .build(&el)
        .unwrap();

    let windowed_context =
        ContextBuilder::new().build_windowed(&window, window.inner_size().into()).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    // println!("Pixel format of the window's GL context: {:?}", windowed_context.get_pixel_format());

    let gl = support::load(&windowed_context);

    el.run(move |event, _, control_flow| {
        println!("{:?}", event);
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => (),
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                gl.draw_frame([1.0, 0.5, 0.7, 1.0]);
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
