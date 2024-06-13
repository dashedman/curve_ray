use cgmath::Vector3;

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn get_point(&self, t: f32) -> Vector3<f32> {
        self.origin + t * self.direction
    }

    pub fn bvh_ray(&self) -> bvh::ray::Ray<f32, 3> {
        bvh::ray::Ray::new(
            nalgebra::Point3::new(self.origin.x, self.origin.y, self.origin.z),
            nalgebra::Vector3::new(self.direction.x, self.direction.y, self.direction.z),
        )
    }
}
