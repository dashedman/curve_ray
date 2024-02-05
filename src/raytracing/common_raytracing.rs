use std::time::SystemTime;

use bvh::bvh::BVH;
use cgmath::InnerSpace;
use glium::Rect;

use crate::cpu_buffer::CPUBuffer;

use super::{camera::Camera, triangle::Triangle, CurveTriangle};

pub fn draw_rect_for_triangulation(
    cpu_buffer: &mut CPUBuffer,
    rect: &Rect,
    shape: &mut Vec<CurveTriangle>,
    camera: &Camera,
    with_bvh: Option<bool>,
) {
    let mut full_triangulations = Vec::new();
    for part in shape.iter() {
        full_triangulations.extend(part.triangulation.clone())
    }

    let bvh_opt;
    if with_bvh.unwrap_or(false) {
        let bench_start = SystemTime::now();
        // build tree
        bvh_opt = Some(BVH::build(&mut full_triangulations));
        let bench_end = SystemTime::now();
        println!("BVH built in {:?}", bench_end.duration_since(bench_start));
        // bvh_opt.as_ref().unwrap().pretty_print();
    } else {
        bvh_opt = None;
    }

    println!("triangulation {}", full_triangulations.len());
    let bench_start = SystemTime::now();
    for x in 0..rect.width {
        for y in 0..rect.height {
            let view_x = (2 * x) as f32 / rect.width as f32 - 1.;
            let view_y = 1. - (2 * y) as f32 / rect.height as f32;
            cpu_buffer[((rect.left + x) as usize, (rect.bottom + y) as usize)] = cast_ray(
                view_x,
                view_y,
                camera,
                &full_triangulations,
                bvh_opt.as_ref(),
            );
        }
    }
    let bench_end = SystemTime::now();
    println!(
        "Triangulation ended in {:?}",
        bench_end.duration_since(bench_start)
    );
}

fn cast_ray(
    x: f32,
    y: f32,
    camera: &Camera,
    triangulation: &Vec<Triangle>,
    bvh_opt: Option<&BVH>,
) -> [f32; 3] {
    let ray = camera.get_ray_in_viewport(x, y);

    // println!("({:.2}, {:.2}) -> ({:.2}, {:.2}, {:.2}) ({:.2}, {:.2}, {:.2})",
    //         x, y, ray.direction.x, ray.direction.y, ray.direction.z, ray.origin.x, ray.origin.y, ray.origin.z);
    if bvh_opt.is_none() {
        let mut nearest_t = f32::INFINITY;
        for triange in triangulation.iter() {
            match triange.intersect(&ray) {
                Ok((t, _)) => {
                    if nearest_t > t {
                        nearest_t = t;
                    }
                }
                Err(_) => {}
            };
        }
        if nearest_t.is_finite() {
            let point = ray.get_point(nearest_t);
            return [
                (point.x + 1.) * 0.5,
                (point.y + 1.) * 0.5,
                (point.z + 1.) * 0.5,
            ];
        }
        return [0., 0.05, 0.];
    } else {
        let bvh = bvh_opt.unwrap();
        let bvh_ray = ray.bvh_ray();

        let traverce_iter = bvh.traverse_iterator(&bvh_ray, triangulation);

        let mut nearest_t = f32::INFINITY;
        for triangle in traverce_iter {
            match triangle.intersect(&ray) {
                Ok((t, _)) => {
                    if nearest_t > t {
                        nearest_t = t;
                    }
                }
                Err(_) => {}
            };
        }

        if nearest_t.is_finite() {
            let point = ray.get_point(nearest_t);
            return [
                (point.x + 1.) * 0.5,
                (point.y + 1.) * 0.5,
                (point.z + 1.) * 0.5,
            ];
        }
        return [0., 0.05, 0.];
    }
}
