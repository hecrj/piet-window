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

    let physical_size = window.inner_size();

    let mut surface = Surface::new(
        &window,
        physical_size.width as usize,
        physical_size.height as usize,
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::MainEventsCleared => {}
            Event::RedrawRequested(_) => {
                let draw_started = std::time::Instant::now();

                {
                    let mut frame = surface.frame();

                    let mut renderer = frame.renderer();
                    renderer.clear(None, Color::WHITE);

                    let dpi = window.scale_factor();
                    let size: winit::dpi::LogicalSize<f64> =
                        window.inner_size().to_logical(dpi);

                    renderer.transform(Affine::scale(dpi));
                    renderer.transform(Affine::translate((
                        (size.width - picture::SIZE.width) / 2.0,
                        (size.height - picture::SIZE.height) / 2.0,
                    )));

                    picture::draw(&mut renderer).expect("Draw picture");

                    renderer.finish().expect("Finish redraw");
                }

                dbg!(std::time::Instant::now() - draw_started);
            }
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    let resize_started = std::time::Instant::now();

                    surface.resize(size.width as usize, size.height as usize);

                    window.request_redraw();

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
