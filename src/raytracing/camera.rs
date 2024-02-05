use cgmath::{InnerSpace, Quaternion, Vector3};

use super::ray::Ray;

pub struct Camera {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub fov: f32, // Field of View for Vertical angle
    pub ratio: f32,
}

impl Camera {
    pub fn get_ray_in_viewport(&self, shiftX: f32, shiftY: f32) -> Ray {
        // println!("{} {} {:?} {:?}", &shiftX, &shiftY, self.origin, self.direction);okihy6hy6n
        let alpha = shiftX * (self.fov / 2.) * self.ratio;
        let beta = shiftY * (self.fov / 2.);

        let right_dir_cam = Vector3::new(self.direction.z, 0., -self.direction.x).normalize();
        let up_dir_cam = right_dir_cam.xyz().cross(self.direction);

        let pixelAlphaQuaternion =
            Quaternion::from_sv((alpha / 2.).cos(), up_dir_cam * (alpha / 2.).sin());
        let pixelBetaQuaternion =
            Quaternion::from_sv((beta / 2.).cos(), right_dir_cam * (beta / 2.).sin());

        let ray_direction = ((Quaternion::from_sv(0., self.direction) * pixelAlphaQuaternion)
            * pixelBetaQuaternion)
            .v;

        Ray {
            origin: self.origin.clone(),
            direction: ray_direction,
        }
    }

    pub fn get_viewport_by_point(&self, point: Vector3<f32>) -> (f32, f32) {
        let dir = (point - self.origin).normalize();
        println!("dir {:?}   {:?}", dir, self.direction);

        let right_dir_cam = Vector3::new(self.direction.z, 0., -self.direction.x).normalize();
        let up_dir_cam = right_dir_cam.cross(self.direction);

        let horisontal_projection = dir - up_dir_cam * dir.dot(up_dir_cam);
        let vertical_projection = dir - right_dir_cam * dir.dot(right_dir_cam);

        println!(
            "prjct {:?}   {:?}",
            horisontal_projection, vertical_projection
        );

        let alpha = self.direction.dot(horisontal_projection);
        let beta = self.direction.dot(vertical_projection);

        println!("{:.3}   {:.3}", alpha, beta);

        let shiftX = alpha.acos() / (self.fov / 2.) * self.ratio;
        let shiftY = beta.acos() / (self.fov / 2.);

        (shiftX, shiftY)
    }
}
