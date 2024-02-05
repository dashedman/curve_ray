use std::{
    io::{Error as IoError, Result as IoResult},
    mem::swap,
};

use cgmath::{ElementWise, Vector3};
use glium::{self, glutin::event_loop::EventLoop};

pub fn load_shaders_sources() -> Result<(String, String), IoError> {
    let vertex_shader = match std::fs::read_to_string("resources\\display.vert") {
        IoResult::Ok(shader) => shader,
        IoResult::Err(error) => return Err(error),
    };

    let fragment_shader = match std::fs::read_to_string("resources\\display.frag") {
        IoResult::Ok(shader) => shader,
        IoResult::Err(error) => return Err(error),
    };

    return Ok((vertex_shader, fragment_shader));
}

pub fn init_window(width: u32, height: u32) -> (glium::Display, EventLoop<()>) {
    use glium::glutin::{dpi::LogicalSize, window::WindowBuilder, Api, ContextBuilder, GlRequest};
    let logical_dpi = LogicalSize::new(width, height);

    let event_loop = EventLoop::new();

    let wb = WindowBuilder::new()
        .with_inner_size(logical_dpi)
        .with_title("CurveRay");

    let cb = ContextBuilder::new()
        .with_vsync(true)
        .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)));

    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    return (display, event_loop);
}

pub fn get_vectors_relation(v1: Vector3<f32>, v2: Vector3<f32>, p: Vector3<f32>) -> f32 {
    let tmp = (p - v1).div_element_wise(v2 - v1);
    if tmp.x.is_finite() {
        if tmp.y.is_finite() {
            if tmp.z.is_finite() {
                (tmp.x + tmp.y + tmp.z) / 3.
            } else {
                (tmp.x + tmp.y) / 2.
            }
        } else {
            if tmp.z.is_finite() {
                (tmp.x + tmp.z) / 2.
            } else {
                tmp.x
            }
        }
    } else {
        if tmp.y.is_finite() {
            if tmp.z.is_finite() {
                (tmp.y + tmp.z) / 2.
            } else {
                tmp.y
            }
        } else {
            tmp.z
        }
    }
}

pub trait MinMaxIterExt: Iterator {
    fn min_max(self) -> (f32, f32);
}

impl<T: Iterator<Item = f32>> MinMaxIterExt for T {
    fn min_max(mut self) -> (f32, f32) {
        let mut max_val = self.next().unwrap();
        let mut min_val = self.next().unwrap();

        if min_val > max_val {
            swap(&mut min_val, &mut max_val);
        }

        for val in self {
            max_val = val.max(max_val);
            min_val = val.min(min_val);
        }

        return (min_val, max_val);
    }
}

pub trait F32Ext {
    fn suppress_tail(self) -> f32;
    fn upowf(self, x: f32) -> f32;
}

impl F32Ext for f32 {
    fn suppress_tail(self) -> f32 {
        const LAST_MANTISA_BITS_MASK: u32 = !0b111;
        f32::from_bits(self.to_bits() & LAST_MANTISA_BITS_MASK)
    }

    fn upowf(self, x: f32) -> f32 {
        const SIGN_BITS_MASK: u32 = 0x80000000;
        let sign = self.to_bits() & SIGN_BITS_MASK;
        f32::from_bits(self.abs().powf(x).to_bits() | sign)
    }
}

pub trait VectorExt {
    fn all_nan(&self) -> bool;
    fn any_nan(&self) -> bool;
    fn abs(&self) -> Vector3<f32>;
    fn upowf(&self, x: f32) -> Vector3<f32>;
    fn suppress_tail(&self) -> Vector3<f32>;
    fn step(&self, edge: f32) -> Vector3<f32>;
    fn lerp(self, other: Vector3<f32>, amount: Vector3<f32>) -> Vector3<f32>;
}

impl VectorExt for Vector3<f32> {
    fn all_nan(&self) -> bool {
        self.x.is_nan() && self.y.is_nan() && self.z.is_nan()
    }

    fn any_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    fn abs(&self) -> Vector3<f32> {
        Vector3::new(self.x.abs(), self.y.abs(), self.z.abs())
    }

    fn upowf(&self, x: f32) -> Vector3<f32> {
        Vector3::new(self.x.upowf(x), self.y.upowf(x), self.z.upowf(x))
    }

    fn suppress_tail(&self) -> Vector3<f32> {
        Vector3::new(
            self.x.suppress_tail(),
            self.y.suppress_tail(),
            self.z.suppress_tail(),
        )
    }

    fn step(&self, edge: f32) -> Vector3<f32> {
        self.map(|v| if v < edge { 0.0 } else { 1.0 })
    }

    fn lerp(self, other: Vector3<f32>, amount: Vector3<f32>) -> Vector3<f32> {
        self.mul_element_wise(1. - amount.x) + other.mul_element_wise(amount)
    }
}

// pub trait RangeExt {
//     fn into
// }

// impl IntoIterator for RangeInclusive<f32> {
//     fn into_iter(self) -> Self::IntoIter {

//     }
// }
