type RGB = [f32; 3];
type RGBA = [f32; 4];


const RED: RGB = [1.0, 0.0, 0.0];
const GREEN: RGB = [0.0, 1.0, 0.0];
const BLUE: RGB = [0.0, 0.0, 1.0];
const YELLOW: RGB = [1.0, 1.0, 0.0];
const CYAN: RGB = [0.0, 1.0, 1.0];
const MAGENTA: RGB = [1.0, 0.0, 1.0];
const WHITE: RGB = [1.0, 1.0, 1.0];
const BLACK: RGB = [0.0, 0.0, 0.0];


struct Material {
    color: RGB,
    alpha: f32,
    refraction_koefficient: f32,
    diffusion_koefficient: f32
}