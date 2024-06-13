use bvh::{
    aabb::{Bounded},
    bounding_hierarchy::BHShape,
};
use bvh::aabb::Aabb;
use nalgebra;
use cgmath::{InnerSpace, Matrix, Matrix3, Zero, Vector3};

use crate::utils::MinMaxIterExt;

use super::ray::Ray;

#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertexes: [Vector3<f32>; 3],

    bhv_node_index: usize,
}

impl Triangle {
    pub fn new(vertexes: [Vector3<f32>; 3]) -> Triangle {
        Triangle {
            vertexes,
            bhv_node_index: 0,
        }
    }

    pub fn get_bary(&self, point: Vector3<f32>) -> Vector3<f32> {
        let normal =
            (self.vertexes[1] - self.vertexes[0]).cross(self.vertexes[2] - self.vertexes[0]);
        let area_sqr = normal.magnitude2();

        if area_sqr < 1e-8 {
            return Vector3::zero();
        }

        let aligned_v0 = self.vertexes[0] - point;
        let aligned_v1 = self.vertexes[1] - point;
        let aligned_v2 = self.vertexes[2] - point;

        let res = (Matrix3 {
            x: aligned_v1.cross(aligned_v2),
            y: aligned_v2.cross(aligned_v0),
            z: aligned_v0.cross(aligned_v1),
        })
        .transpose()
            * normal
            / area_sqr;

        return res;
    }

    pub fn intersect_plane(&self, ray: &Ray) -> f32 {
        let e1 = self.vertexes[1] - self.vertexes[0];
        let e2 = self.vertexes[2] - self.vertexes[0];
        let tt = ray.origin - self.vertexes[0];

        let e1_e2_crs = e1.cross(e2);

        let c = 1. / -ray.direction.dot(e1_e2_crs);

        let t = e1_e2_crs.dot(tt) * c;
        return t;
    }

    /// Return coords of interseption + barycentric coords or Nothing
    pub fn intersect(&self, ray: &Ray) -> Result<(f32, Vector3<f32>), Vector3<f32>> {
        // solve from https://en.wikipedia.org/wiki/Line%E2%80%93plane_intersection
        let e1 = self.vertexes[1] - self.vertexes[0];
        let e2 = self.vertexes[2] - self.vertexes[0];
        let tt = ray.origin - self.vertexes[0];

        let e1_e2_crs = e1.cross(e2);

        let c = 1. / -ray.direction.dot(e1_e2_crs);

        let w1 = e2.cross(-ray.direction).dot(tt) * c;
        let w2 = (-ray.direction).cross(e1).dot(tt) * c;
        let w0 = 1. - (w1 + w2);

        // println!("({:.2}, {:.2}, {:.2}) ({:.2}, {:.2}, {:.2}) -> [{:.2}, {:.2}, {:.2}, {:.2}]",
        //     ray.direction.x, ray.direction.y, ray.direction.z, ray.origin.x, ray.origin.y, ray.origin.z, t, w1, w2, w3);

        if w1 > 1. || w1 < 0. || w2 > 1. || w2 < 0. || w0 > 1. || w0 < 0. {
            return Err(Vector3::new(w0, w1, w2));
        }

        let t = e1_e2_crs.dot(tt) * c;

        return Ok((t, Vector3::new(w0, w1, w2)));
    }
}

impl Bounded<f32, 3> for Triangle {
    fn aabb(&self) -> Aabb<f32, 3> {
        let (min_x, max_x) = self.vertexes.iter().map(|&v| v[0]).min_max();
        let (min_y, max_y) = self.vertexes.iter().map(|&v| v[1]).min_max();
        let (min_z, max_z) = self.vertexes.iter().map(|&v| v[2]).min_max();

        let min_p = nalgebra::Point3::new(min_x, min_y, min_z);
        let max_p = nalgebra::Point3::new(max_x, max_y, max_z);
        Aabb::with_bounds(min_p, max_p)
    }
}

impl BHShape<f32, 3> for Triangle {
    fn set_bh_node_index(&mut self, index: usize) {
        self.bhv_node_index = index
    }

    fn bh_node_index(&self) -> usize {
        self.bhv_node_index
    }
}
