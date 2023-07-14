use std::io;
use pixels::{Pixels, SurfaceTexture};
use image::GenericImageView;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let mut input_file_path = String::new();
    println!("Enter the file path:");
    io::stdin().read_line(&mut input_file_path).expect("Failed to read input");
    let image_path = input_file_path.as_str();
    let image = image::open(image_path.trim()).expect("Failed to open image");
    let (width, height) = image.dimensions();
    let _image_width = image.width() as u32;
    let _image_height = image.height() as u32;
    let image_pixels = image.to_rgba8().into_raw();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Red Box")
        .with_inner_size(winit::dpi::LogicalSize::new(800, 600))
        .build(&event_loop)
        .unwrap();
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(width, height, surface_texture).unwrap()
    };
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {
                // Clear the pixels buffer
                let frame = pixels.frame_mut();

                frame.copy_from_slice(&image_pixels);
                if pixels
                    .render()
                    .map_err(|e| println!("Failed to render pixels: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            _ => (),
        }
        window.request_redraw();
    });
}