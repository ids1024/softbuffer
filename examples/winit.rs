use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

fn time_call<T, F: FnOnce() -> T>(name: &str, total: &mut std::time::Duration, f: F) -> T {
    let time = std::time::Instant::now();
    let ret = f();
    let elapsed = time.elapsed();
    eprintln!("`{}` time: {:?}", name, elapsed);
    *total += elapsed;
    ret
}

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    #[cfg(target_arch = "wasm32")]
    {
        use winit::platform::web::WindowExtWebSys;

        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&window.canvas())
            .unwrap();
    }

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };

                let mut total_time = std::time::Duration::ZERO;

                time_call("resize", &mut total_time, || surface.resize(width, height));

                let buffer = time_call("buffer_mut", &mut total_time, || surface.buffer_mut());
                for index in 0..(width * height) {
                    let y = index as u32 / width;
                    let x = index as u32 % width;
                    let red = x % 255;
                    let green = y % 255;
                    let blue = (x * y) % 255;

                    buffer[index as usize] = blue | (green << 8) | (red << 16);
                }

                time_call("present", &mut total_time, || surface.present());

                eprintln!("Total time: {:?}", total_time);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                window_id,
            } if window_id == window.id() => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}
