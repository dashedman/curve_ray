use cgmath::Vector3;
use glium::Rect;

use crate::cpu_buffer::CPUBuffer;

use super::{camera::Camera, triangle::Triangle, CurveTriangle};

pub fn draw_rect_for_triangles(
    cpu_buffer: &mut CPUBuffer,
    rect: &Rect,
    shape: &CurveTriangle,
    camera: &Camera,
) {
    let red = [1., 0., 0.];
    let green = [0., 1., 0.];
    let blue = [0., 0., 1.];
    draw_triangle(cpu_buffer, rect, camera, blue, &shape.base);
    // draw_triangle(cpu_buffer, rect, camera, green,
    //     &Triangle { vertexes: shape.pivots },
    // ) ;
    // draw_triangle(cpu_buffer, rect, camera, red,
    //     &Triangle { vertexes: shape.opposite_pivots },
    // ) ;
}

pub fn draw_line(
    cpu_buffer: &mut CPUBuffer,
    rect: &Rect,
    camera: &Camera,
    color: [f32; 3],
    v1: Vector3<f32>,
    v2: Vector3<f32>,
) {
    let first_point = camera.get_viewport_by_point(v1);
    let fbuff_x = ((first_point.0 + 1.) * 0.5 * rect.width as f32) as i32;
    let fbuff_y = ((1. - first_point.1) * 0.5 * rect.height as f32) as i32;

    let last_point = camera.get_viewport_by_point(v2);
    let lbuff_x = ((last_point.0 + 1.) * 0.5 * rect.width as f32) as i32;
    let lbuff_y = ((1. - last_point.1) * 0.5 * rect.height as f32) as i32;

    let steps = (fbuff_x - lbuff_x).abs().max((fbuff_y - lbuff_y).abs());
    println!("{:?}  {:?}", v1, v2);

    cpu_buffer[(
        (rect.left + fbuff_x as u32) as usize,
        (rect.bottom + fbuff_y as u32) as usize,
    )] = color;
    for step in 1..steps {
        let t = step as f32 / steps as f32;

        let point = camera.get_viewport_by_point(t * v1 + (1. - t) * v2);
        let buff_x: i32 = ((point.0 + 1.) * 0.5 * rect.width as f32) as i32;
        let buff_y = ((1. - point.1) * 0.5 * rect.height as f32) as i32;

        cpu_buffer[(
            (rect.left + buff_x as u32) as usize,
            (rect.bottom + buff_y as u32) as usize,
        )] = color;
    }
    cpu_buffer[(
        (rect.left + lbuff_x as u32) as usize,
        (rect.bottom + lbuff_y as u32) as usize,
    )] = color;
}

pub fn draw_triangle(
    cpu_buffer: &mut CPUBuffer,
    rect: &Rect,
    camera: &Camera,
    color: [f32; 3],
    triangle: &Triangle,
) {
    draw_line(
        cpu_buffer,
        rect,
        camera,
        color,
        triangle.vertexes[0],
        triangle.vertexes[1],
    );
    // draw_line(
    //     cpu_buffer, rect, camera, color,
    //     triangle.vertexes[0], triangle.vertexes[2]
    // );
    // draw_line(
    //     cpu_buffer, rect, camera, color,
    //     triangle.vertexes[2], triangle.vertexes[1]
    // );
}

pub fn draw_curve(
    cpu_buffer: &mut CPUBuffer,
    rect: &Rect,
    camera: &Camera,
    v1: Vector3<f32>,
    v2: Vector3<f32>,
    p: Vector3<f32>,
) {
}
