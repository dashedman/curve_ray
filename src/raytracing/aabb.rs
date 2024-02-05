use cgmath::{ElementWise, Vector3};

use crate::utils::VectorExt;

use super::ray::Ray;

#[derive(Debug)]
pub struct AABBox {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub min_z: f32,
    pub max_z: f32,
}

impl AABBox {
    pub fn center(&self) -> Vector3<f32> {
        Vector3::new(
            (self.max_x + self.min_x) / 2.,
            (self.min_y + self.max_y) / 2.,
            (self.min_z + self.max_z) / 2.,
        )
    }

    pub fn half_size(&self) -> Vector3<f32> {
        Vector3::new(
            (self.max_x - self.min_x) / 2.,
            (self.max_y - self.min_y) / 2.,
            (self.max_z - self.min_z) / 2.,
        )
    }

    pub fn get_slice_for_ray(&self, ray: &Ray) -> (f32, f32) {
        // https://iquilezles.org/articles/intersectors/

        let inv_dir = 1.0 / ray.direction;
        let n = inv_dir.mul_element_wise(ray.origin - self.center());
        let k = inv_dir.abs().mul_element_wise(self.half_size());
        let t1 = -n - k;
        let t2 = -n + k;

        let t_n = t1.x.max(t1.y.max(t1.z));
        let t_f = t2.x.min(t2.y.min(t2.z));

        // no intersection
        if t_n > t_f || t_f < 0.0 {
            return (-1.0, -1.0);
        }

        // this is normal of side
        // oN = -sign(rd)*step(t1.yzx,t1.xyz)*step(t1.zxy,t1.xyz);

        (t_n.max(0.), t_f)
    }
}
