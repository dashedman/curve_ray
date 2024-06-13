extern crate cgmath;
extern crate glium;

pub mod cpu_buffer;
pub mod raytracing;
pub mod utils;
mod shapes;
mod materials;

use cgmath::{Deg, InnerSpace, Rad, Vector3};
use cpu_buffer::CPUBuffer;
use glium::glutin;
use glium::Surface;
use raytracing::camera::Camera;


fn main() {
    let width: u32 = 1000;
    let height: u32 = 500;
    let mut cpu_buffer = CPUBuffer::new(width, height);
    let camera = Camera {
        origin: Vector3 {
            x: 0.,
            y: 0.,
            z: -2.,
        },
        direction: Vector3::new(0., 0., 1.).normalize(),
        fov: Rad::from(Deg(90.)).0,
        ratio: width as f32 / 2. / height as f32,
    };
    let mut shape = get_sphere();
    for part in shape.iter_mut() {
        part.triangulate(5)
    }

    // init window
    let (display, event_loop) = utils::init_window(width, height);
    // dest_texture.as_surface().clear_color(0.0, 0.5, 0.3, 1.0);

    // start draw
    // procese mouse move

    // Here we draw the black background and triangle to the screen using the previously
    // initialised resources.
    //
    // In this case we use a closure for simplicity, however keep in mind that most serious
    // applications should probably use a function that takes the resources as an argument.
    let to_screen = |cpu_buffer: &CPUBuffer, display: &glium::Display| {
        // drawing a frame
        let mut target = display.draw();
        target.clear_color(0.0, 0.5, 0.3, 1.0);
        cpu_buffer
            .as_texture(&display)
            .as_surface()
            .fill(&target, glium::uniforms::MagnifySamplerFilter::Linear);

        // this is for shaders
        //
        // target.draw(
        //     &vertex_buffer, &index_buffer, &program,
        //     &uniforms, &Default::default()
        // ).unwrap();
        target.finish().unwrap();
    };
    // println!("draw");

    // // Draw the triangle to the screen.
    raytracing::draw_to(&mut cpu_buffer, &camera, &mut shape);
    to_screen(&cpu_buffer, &display);

    // the main loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // Break from the main loop when the window is closed.
                glutin::event::WindowEvent::CloseRequested => glutin::event_loop::ControlFlow::Exit,
                // Redraw the triangle when the window is resized.
                glutin::event::WindowEvent::Resized(..) => {
                    println!("resized!");
                    // draw();
                    to_screen(&cpu_buffer, &display);
                    glutin::event_loop::ControlFlow::Poll
                }
                _ => glutin::event_loop::ControlFlow::Poll, // glutin::event_loop::ControlFlow::Poll,
            },
            glutin::event::Event::NewEvents(_poll) => {
                // draw();
                glutin::event_loop::ControlFlow::Poll
            }
            _ => glutin::event_loop::ControlFlow::Poll,
        };
    });
}
