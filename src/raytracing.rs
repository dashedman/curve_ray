use glium::Rect;

use crate::cpu_buffer::CPUBuffer;

use self::{camera::Camera, curve_triangle::CurveTriangle, curve_triangle::IntersectionError};

pub mod aabb;
pub mod camera;
pub mod common_raytracing;
pub mod curve_raytracing;
pub mod curve_triangle;
pub mod obb;
mod rasterisator;
pub mod ray;
mod triange_shell;
pub mod triangle;

pub fn draw_to(cpu_buffer: &mut CPUBuffer, camera: &Camera, shape: &mut Vec<CurveTriangle>) {
    let first_half = Rect {
        left: 0,
        bottom: 0,
        width: cpu_buffer.width / 2,
        height: cpu_buffer.height,
    };
    let second_half = Rect {
        left: cpu_buffer.width / 2,
        bottom: 0,
        width: cpu_buffer.width / 2,
        height: cpu_buffer.height,
    };
    // let second_half = Rect {
    //     left: 0, bottom: 0,
    //     width: cpu_buffer.width, height: cpu_buffer.height
    // };
    // rasterisator::draw_rect_for_triangles(cpu_buffer, &first_half, &shape, camera);
    common_raytracing::draw_rect_for_triangulation(
        cpu_buffer,
        &first_half,
        shape,
        camera,
        Some(false),
    );
    curve_raytracing::draw_rect_for_curve_surface(
        cpu_buffer,
        &second_half,
        shape,
        camera,
        Some(false),
    );
}
