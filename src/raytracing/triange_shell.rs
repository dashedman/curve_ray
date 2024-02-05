use super::{ray::Ray, triangle::Triangle};

#[derive(Debug)]
pub struct TriangleShell<const N: usize> {
    pub triangles: [Triangle; N],
}

impl<const N: usize> TriangleShell<N> {
    pub fn get_slice_for_ray(&self, ray: &Ray) -> (f32, f32) {
        let (mut t_start, mut t_end) = (f32::INFINITY, -1.);

        for triangle in self.triangles.iter() {
            if let Ok((t, _)) = triangle.intersect(ray) {
                t_start = t.min(t_start);
                t_end = t.max(t_end);
            }
        }

        // no intersection
        if t_start > t_end || t_end < 0.0 {
            return (-1.0, -1.0);
        }

        (t_start.max(0.), t_end)
    }
}
