use std::time::SystemTime;

use bvh::bvh::Bvh;
use glium::Rect;

use crate::cpu_buffer::CPUBuffer;

use super::{camera::Camera, CurveTriangle, IntersectionError};

pub fn draw_rect_for_curve_surface(
    cpu_buffer: &mut CPUBuffer,
    rect: &Rect,
    shape: &mut Vec<CurveTriangle>,
    camera: &Camera,
    with_bvh: Option<bool>,
) {
    let bvh_opt;
    if with_bvh.unwrap_or(false) {
        let bench_start = SystemTime::now();
        // build tree
        bvh_opt = Some(Bvh::build(shape));
        let bench_end = SystemTime::now();
        println!("BVH built in {:?}", bench_end.duration_since(bench_start));
        // bvh_opt.as_ref().unwrap().pretty_print();
    } else {
        bvh_opt = None;
    }

    let bench_start = SystemTime::now();
    // for x in rect.width/2-1..rect.width/2+1 {
    //     for y in rect.height/2-1..rect.height/2+1 {
    for x in 0..rect.width {
        for y in 0..rect.height {
            let view_x = (2 * x) as f32 / rect.width as f32 - 1.;
            let view_y = 1. - (2 * y) as f32 / rect.height as f32;
            cpu_buffer[((rect.left + x) as usize, (rect.bottom + y) as usize)] =
                cast_ray(view_x, view_y, camera, shape, bvh_opt.as_ref());
        }
    }
    let bench_end = SystemTime::now();
    println!(
        "Curve Surface ended in {:?}",
        bench_end.duration_since(bench_start)
    );

    // let (x, y) = (94, 5);
    // println!("AABB {:?} {:?} {:?}", shape.get_bounding_box(), shape.get_bounding_box().center(), shape.get_bounding_box().half_size());
    // let view_x = (2 * x) as f32 / cpu_buffer.width as f32 - 1.;
    // let view_y = 1. - (2 * y) as f32 / cpu_buffer.height as f32;
    // println!("{:?}", cast_ray(0., 0., camera, shape));
}

fn cast_ray(
    x: f32,
    y: f32,
    camera: &Camera,
    shape: &Vec<CurveTriangle>,
    bvh_opt: Option<&Bvh<f32, 3>>,
) -> [f32; 3] {
    let ray = camera.get_ray_in_viewport(x, y);

    // println!("({:.2}, {:.2}) -> ({:.2}, {:.2}, {:.2}) ({:.2}, {:.2}, {:.2})",
    //         x, y, ray.direction.x, ray.direction.y, ray.direction.z, ray.origin.x, ray.origin.y, ray.origin.z);
    let mut nearest_t = f32::INFINITY;
    if bvh_opt.is_none() {
        for triange in shape.iter() {
            match triange.intersect(&ray) {
                Ok((t, _)) => {
                    if nearest_t > t {
                        nearest_t = t;
                    }
                }
                Err(_) => {}
            };
        }
    } else {
        let bvh = bvh_opt.unwrap();
        let bvh_ray = ray.bvh_ray();

        let traverce_iter = bvh.traverse_iterator(&bvh_ray, shape);

        for part in traverce_iter {
            match part.intersect(&ray) {
                Ok((t, _)) => {
                    // let dist = 1. - camera.origin.distance(point) / 3.5;
                    if nearest_t > t {
                        nearest_t = t;
                    }

                    // let dir = (shape.base.vertexes[1] - shape.pivots[2]).normalize();
                    // let u = dir.dot(point - shape.pivots[2]); c
                    // [u, 0., 1. - u]
                }
                _ => {} // Err(intersection_error) => return match intersection_error {
                        //     IntersectionError::BehindRay => [0., 0., 0.], // not intersect any
                        //     IntersectionError::CantSubrayBase => [0.0, 0.0, 0.5], // bad barri
                        //     IntersectionError::UndefUndef => [1., 0., 0.], // 0 -> 0
                        //     IntersectionError::UndefOut => [1., 3., 0.], // 0 -> 1
                        //     IntersectionError::UndefIn => [1., 0., 3.], // 0 -> -1
                        //     IntersectionError::OutUndef => [0.6, 1., 0.], // 1 -> 0
                        //     IntersectionError::OutOut => [0., 0.9, 0.], // 1 -> 1
                        //     IntersectionError::OutIn => [0., 1., 0.3], // 1 -> -1
                        //     IntersectionError::InUndef => [0.3, 0., 1.], // -1 -> 0
                        //     IntersectionError::InOut => [0., 0.3, 1.], // -1 -> 1
                        //     IntersectionError::InIn => [0., 0., 1.], // -1 -> -1
                        //     _ => [0., 0., 0.],
                        // }
            };
        }
    }

    if nearest_t.is_finite() {
        let point = ray.get_point(nearest_t);
        if point.x > 1. {
            // println!("{}", nearest_t);
        }
        return [
            (point.x + 1.) * 0.5,
            (point.y + 1.) * 0.5,
            (point.z + 1.) * 0.5,
        ];
    }
    return [0., 0., 0.05];
}
