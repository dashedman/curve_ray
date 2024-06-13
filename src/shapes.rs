use cgmath::Vector3;
use crate::raytracing::curve_triangle::CurveTriangle;
use crate::raytracing::triangle::Triangle;


struct Shape {
    polygons: Vec<Triangle>,
    color:
}


fn get_scene() {
    // construct walls
    get_wall();
    get_wall();
    get_wall();
    // floor
    get_wall();
    // ceil
    get_wall();

    // set boxes
    get_box();
    // set spheres
    get_sphere();
}


fn get_wall() {

}


fn get_box() {}


fn get_sphere() {}


fn get_curve_sphere() -> Vec<CurveTriangle> {
    let mut sphere = Vec::new();

    for index in (0 as i8)..(8 as i8) {
        let (x, y, z) = (
            ((index & 1) == 0) as i8 * 2 - 1,
            ((index & 2) == 0) as i8 * 2 - 1,
            ((index & 4) == 0) as i8 * 2 - 1,
        );
        let sphere_part = CurveTriangle::new(
            Triangle::new([
                Vector3::new(x as f32, 0., 0.),
                Vector3::new(0., y as f32, 0.),
                Vector3::new(0., 0., z as f32),
            ]),
            [
                Vector3::new(0., 0., 0.),
                Vector3::new(0., 0., 0.),
                Vector3::new(0., 0., 0.),
            ],
            [2., 2., 2.],
        );
        sphere.push(sphere_part);
    }
    sphere
}
