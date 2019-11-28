use piet_common::{kurbo::Affine, Color, RenderContext};
use piet_window::Surface;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod picture;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .expect("Open window");

    let physical_size = window.inner_size().to_physical(window.hidpi_factor());

    let mut surface = Surface::new(
        &window,
        physical_size.width.round() as usize,
        physical_size.height.round() as usize,
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                let draw_started = std::time::Instant::now();

                {
                    let mut frame = surface.frame();

                    let mut renderer = frame.renderer();
                    renderer.clear(Color::WHITE);

                    let dpi = window.hidpi_factor();
                    renderer.transform(Affine::scale(dpi));

                    picture::draw(&mut renderer).expect("Draw picture");

                    renderer.finish().expect("Finish redraw");
                }

                dbg!(std::time::Instant::now() - draw_started);
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    let resize_started = std::time::Instant::now();

                    let physical_size = size.to_physical(window.hidpi_factor());

                    surface.resize(
                        physical_size.width.round() as usize,
                        physical_size.height.round() as usize,
                    );

                    dbg!(std::time::Instant::now() - resize_started);
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            },
            _ => {}
        }
    });
}
