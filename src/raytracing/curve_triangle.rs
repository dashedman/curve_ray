use bvh::{
    aabb::{Bounded},
    bounding_hierarchy::BHShape,
};
use bvh::aabb::Aabb;
use cgmath::{num_traits::Inv, Array, ElementWise, InnerSpace, Vector3, VectorSpace};

use crate::utils::{get_vectors_relation, MinMaxIterExt, VectorExt};

use super::{aabb::AABBox, ray::Ray, triange_shell::TriangleShell, triangle::Triangle};

const UNIT_VECTOR: Vector3<f32> = Vector3 {
    x: 1.,
    y: 1.,
    z: 1.,
};

pub enum IntersectionError {
    BehindRay,
    CantSubrayBase,
    NoIntersections,
    UndefUndef,
    UndefOut,
    UndefIn,
    OutUndef,
    OutOut,
    OutIn,
    InUndef,
    InOut,
    InIn,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum ISS {
    Undefined,
    Inside,
    Outside,
}

pub struct CurveTriangle {
    pub base: Triangle,
    pub pivots: [Vector3<f32>; 3],
    pub curve_koefs: [f32; 3],

    pub root_point: Vector3<f32>,
    pub shell_points: [Vector3<f32>; 3],
    pub opposite_root: Vector3<f32>,
    pub triangulation: Vec<Triangle>,

    pub tr_shell: Option<TriangleShell<4>>,

    bhv_node_index: usize,
}

impl CurveTriangle {
    pub fn new(
        triangle: Triangle,
        pivots: [Vector3<f32>; 3],
        curve_koefs: [f32; 3],
    ) -> CurveTriangle {
        let shell_points = [
            triangle.vertexes[0] + triangle.vertexes[0] - pivots[0],
            triangle.vertexes[1] + triangle.vertexes[1] - pivots[1],
            triangle.vertexes[2] + triangle.vertexes[2] - pivots[2],
        ];

        let root_point = (pivots[0] + pivots[1] + pivots[2]).div_element_wise(3.0);
        let opposite_root = triangle.vertexes[0] + triangle.vertexes[1] + triangle.vertexes[2]
            - root_point.mul_element_wise(2.0);

        let mut ct = CurveTriangle {
            base: triangle,
            pivots,
            curve_koefs,
            root_point,
            shell_points,
            opposite_root,
            triangulation: Vec::new(),
            tr_shell: None,
            bhv_node_index: 0,
        };
        ct.precalc_triangle_shell();
        ct
    }

    pub fn triangulate(&mut self, accuracy: i32) {
        let mut line_size = 1;
        let mut previous_points = vec![self.base.vertexes[1]];

        for major_step in 1..=accuracy {
            let major_interp = major_step as f32 / accuracy as f32;
            let left_point =
                VectorSpace::lerp(self.base.vertexes[1], self.base.vertexes[0], major_interp);
            let right_point =
                VectorSpace::lerp(self.base.vertexes[1], self.base.vertexes[2], major_interp);

            // minor cycle
            let mut current_points = vec![left_point];
            for minor_step in 1..=line_size {
                let minor_interp = minor_step as f32 / line_size as f32;
                let next_point = VectorSpace::lerp(left_point, right_point, minor_interp);

                self.triangulation.push(Triangle::new([
                    self.get_surface_point(previous_points[minor_step - 1]),
                    self.get_surface_point(current_points[minor_step - 1]),
                    self.get_surface_point(next_point),
                ]));
                if minor_step > 1 {
                    self.triangulation.push(Triangle::new([
                        self.get_surface_point(previous_points[minor_step - 2]),
                        self.get_surface_point(previous_points[minor_step - 1]),
                        self.get_surface_point(current_points[minor_step - 1]),
                    ]));
                }

                current_points.push(next_point)
            }

            previous_points = current_points;
            line_size += 1;
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Result<(f32, Vector3<f32>), IntersectionError> {
        let (mut t_start, mut t_end) = self.tr_shell.as_ref().unwrap().get_slice_for_ray(ray);
        if t_start < 0. {
            return Err(IntersectionError::BehindRay);
        }

        // first step
        let mut start_sdf = self.intersect_step(t_start, ray);
        let mut end_sdf = self.intersect_step(t_end, ray);
        let mut is_intersected = false;
        // check intersection
        for _ in 0..5 {
            let t_middle = (t_start + t_end) * 0.5;
            let middle_sdf = self.intersect_step(t_middle, ray);

            if start_sdf.signum() != middle_sdf.signum() {
                t_end = t_middle;
                end_sdf = middle_sdf;
                is_intersected = true;
            } else if middle_sdf.signum() != end_sdf.signum() {
                t_start = t_middle;
                start_sdf = middle_sdf;
                is_intersected = true;
            } else {
                if start_sdf.abs() < end_sdf.abs() {
                    t_end = t_middle;
                    end_sdf = middle_sdf;
                } else {
                    t_start = t_middle;
                    start_sdf = middle_sdf;
                }
            }
        }

        if !is_intersected {
            return Err(IntersectionError::NoIntersections);
            // return match (prev_state, state) {
            //     (ISS::Undefined, ISS::Undefined) => Err(IntersectionError::UndefUndef),
            //     (ISS::Undefined, ISS::Outside) => Err(IntersectionError::UndefOut),
            //     (ISS::Undefined, ISS::Inside) => Err(IntersectionError::UndefIn),
            //     (ISS::Outside, ISS::Undefined) => Err(IntersectionError::OutUndef),
            //     (ISS::Outside, ISS::Outside) => Err(IntersectionError::OutOut),
            //     (ISS::Outside, ISS::Inside) => Err(IntersectionError::OutIn),
            //     (ISS::Inside, ISS::Undefined) => Err(IntersectionError::InUndef),
            //     (ISS::Inside, ISS::Outside) => Err(IntersectionError::InOut),
            //     (ISS::Inside, ISS::Inside) => Err(IntersectionError::InIn),
            // }
        }

        // get intersection
        for _ in 0..3 {
            let t_middle = (t_start + t_end) * 0.5;
            let middle_sdf = self.intersect_step(t_middle, ray);

            if start_sdf.signum() != middle_sdf.signum() {
                t_end = t_middle;
            } else {
                t_start = t_middle;
                start_sdf = middle_sdf;
            }
        }
        let t_middle = (t_start + t_end) * 0.5;
        let point = ray.get_point(t_middle);

        // println!("h: {:?}", debug_state_history);

        return match self.base.intersect(&Ray {
            origin: point,
            direction: (self.root_point - point).normalize(),
        }) {
            Ok((_, barri)) => Ok((t_middle, barri)),
            Err(_) => {
                // println!("bad barri   t: {:.2} t_start: {:.2} t_end: {:.2} t_step: {:.2}   point {:?}   on cone {:?}  h: {:?}",
                //          t, t_start, t_end, t_step, point, point_on_cone_pivot, debug_state_history);
                Err(IntersectionError::CantSubrayBase)
            }
        };
        // println!("end   t_start: {:.2} t_end: {:.2} t_step: {:.2}     h: {:?}",
        //             t_start, t_end, t_step, debug_state_history);

        //
    }

    #[inline]
    pub fn intersect_step(&self, t: f32, ray: &Ray) -> f32 {
        let point_on_ray = ray.get_point(t);

        // get point on curve
        // intersect with base triangle
        // intersect with vu0-vu1 slice
        let sub_ray = Ray {
            origin: point_on_ray,
            direction: (self.root_point - point_on_ray).normalize(),
        };

        let trusted_intersection = self.base.intersect(&sub_ray);
        let bary = match trusted_intersection {
            Ok((_, res)) => res,
            Err(res) => res,
        };

        // get point on plane
        // let plane_intersection = sub_ray.get_point(sub_ray_point);

        let point_on_surface = self.get_surface_point_by_bary(bary);

        // distance field
        // can chanche magnitude
        let cone_dfs = (point_on_ray - self.root_point).magnitude()
            - (point_on_surface - self.root_point).magnitude();

        // println!(
        //     "ray {} surf {} cone {}",
        //     (point_on_ray - self.root_point).magnitude(),
        //     (point_on_surface - self.root_point).magnitude(),
        //     cone_dfs
        // );
        // if cone_dfs.is_nan() {
        //     println!("{:?} {} {:?} {:?} {:?} {:?}", point_on_surface, balanced_coef, c0, c1, c2, pair_relations);
        //     println!("raw_bary: {:?} bary: {:?} pair: {:?}  koefs: {:?}", raw_bary, bary, pair_relations, koefs);
        // }

        cone_dfs
    }

    pub fn precalc_triangle_shell(&mut self) {
        self.tr_shell = Some(TriangleShell {
            triangles: [
                Triangle::new([
                    self.shell_points[0],
                    self.shell_points[1],
                    self.shell_points[2],
                ]),
                Triangle::new([self.root_point, self.shell_points[0], self.shell_points[1]]),
                Triangle::new([self.root_point, self.shell_points[1], self.shell_points[2]]),
                Triangle::new([self.root_point, self.shell_points[2], self.shell_points[0]]),
            ],
        });
    }

    #[inline]
    pub fn get_surface_point(&self, point_on_base: Vector3<f32>) -> Vector3<f32> {
        self.get_surface_point_by_bary(self.base.get_bary(point_on_base))
    }

    #[inline]
    pub fn get_surface_point_by_bary(&self, bary_of_point: Vector3<f32>) -> Vector3<f32> {
        let pair_relations = bary_of_point
            .div_element_wise(UNIT_VECTOR - bary_of_point.zxy())
            .map(|v| if v.is_nan() { 1. } else { v });

        // get interpolation koefs
        let koefs = bary_of_point.mul_element_wise(pair_relations.yzx())
            + bary_of_point
                .yzx()
                .mul_element_wise(UNIT_VECTOR - pair_relations.zxy());

        // get points on curves
        let c0 = CurveTriangle::curve(
            1. - pair_relations[0],
            self.base.vertexes[0],
            self.base.vertexes[1],
            self.pivots[0],
            self.curve_koefs[0],
        );
        let c1 = CurveTriangle::curve(
            1. - pair_relations[1],
            self.base.vertexes[1],
            self.base.vertexes[2],
            self.pivots[1],
            self.curve_koefs[1],
        );
        let c2 = CurveTriangle::curve(
            1. - pair_relations[2],
            self.base.vertexes[2],
            self.base.vertexes[0],
            self.pivots[2],
            self.curve_koefs[2],
        );

        let balanced_coef = koefs[0] * self.curve_koefs[0]
            + koefs[1] * self.curve_koefs[1]
            + koefs[2] * self.curve_koefs[2];

        (c0.upowf(balanced_coef) * koefs[0]
            + c1.upowf(balanced_coef) * koefs[1]
            + c2.upowf(balanced_coef) * koefs[2])
            .upowf(balanced_coef.inv())
    }

    #[inline]
    pub fn get_surface_point_by_bary_sqrt(&self, bary_of_point: Vector3<f32>) -> Vector3<f32> {
        let pair_relations = bary_of_point
            .div_element_wise(UNIT_VECTOR - bary_of_point.zxy())
            .map(|v| if v.is_nan() { 1. } else { v });

        // get interpolation koefs
        let koefs = bary_of_point.mul_element_wise(pair_relations.yzx())
            + bary_of_point
                .yzx()
                .mul_element_wise(UNIT_VECTOR - pair_relations.zxy());

        // get points on curves
        let c0 = CurveTriangle::curve_sqrt(
            1. - pair_relations[0],
            self.base.vertexes[0],
            self.base.vertexes[1],
            self.pivots[0],
        );
        let c1 = CurveTriangle::curve_sqrt(
            1. - pair_relations[1],
            self.base.vertexes[1],
            self.base.vertexes[2],
            self.pivots[1],
        );
        let c2 = CurveTriangle::curve_sqrt(
            1. - pair_relations[2],
            self.base.vertexes[2],
            self.base.vertexes[0],
            self.pivots[2],
        );

        let balanced_coef = koefs[0] * 2. + koefs[1] * 2. + koefs[2] * 2.;

        (c0.upowf(balanced_coef) * koefs[0]
            + c1.upowf(balanced_coef) * koefs[1]
            + c2.upowf(balanced_coef) * koefs[2])
            .upowf(balanced_coef.inv())
    }

    pub fn curve(
        t: f32,
        v1: Vector3<f32>,
        v2: Vector3<f32>,
        p: Vector3<f32>,
        curve_koef: f32,
    ) -> Vector3<f32> {
        let fix_t = t.min(1.).max(0.);
        let s = (1. - fix_t).powf(curve_koef);
        let f = fix_t.powf(curve_koef);
        let pow1 = (s / (s + f)).powf(curve_koef.inv());
        let pow2 = (f / (s + f)).powf(curve_koef.inv());
        pow1 * v1 + pow2 * v2 + (1. - pow1 - pow2) * p
    }

    pub fn curve_sqrt(t: f32, v1: Vector3<f32>, v2: Vector3<f32>, p: Vector3<f32>) -> Vector3<f32> {
        let fix_t = t.min(1.).max(0.);
        let s = (1. - fix_t) * (1. - fix_t);
        let f = fix_t * fix_t;
        let pow1 = (s / (s + f)).sqrt();
        let pow2 = (f / (s + f)).sqrt();
        pow1 * v1 + pow2 * v2 + (1. - pow1 - pow2) * p
    }
}

impl Bounded<f32, 3> for CurveTriangle {
    fn aabb(&self) -> Aabb<f32, 3> {
        let (min_x, max_x) = self
            .base
            .vertexes
            .iter()
            .chain(&[self.opposite_root])
            .map(|&v| v[0])
            .min_max();
        let (min_y, max_y) = self
            .base
            .vertexes
            .iter()
            .chain(&[self.opposite_root])
            .map(|&v| v[1])
            .min_max();
        let (min_z, max_z) = self
            .base
            .vertexes
            .iter()
            .chain(&[self.opposite_root])
            .map(|&v| v[2])
            .min_max();

        // get bounds for surface by binary search and ray casting

        let min_p = nalgebra::Point3::new(min_x, min_y, min_z);
        let max_p = nalgebra::Point3::new(max_x, max_y, max_z);
        Aabb::with_bounds(min_p, max_p)
    }
}

impl BHShape<f32, 3> for CurveTriangle {
    fn set_bh_node_index(&mut self, index: usize) {
        self.bhv_node_index = index
    }

    fn bh_node_index(&self) -> usize {
        self.bhv_node_index
    }
}
