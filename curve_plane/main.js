import * as THREE from 'three';
import OrbitControls from 'orbit-controls-es6'

const STEPS = 1000

const scene = new THREE.Scene()
const camera = new THREE.PerspectiveCamera( 90, window.innerWidth / window.innerHeight, 0.1, 100 )
camera.position.set(1, 3, 5)
camera.lookAt(new THREE.Vector3(0, 0, 0))

const renderer = new THREE.WebGLRenderer()
renderer.setSize( window.innerWidth, window.innerHeight )
document.body.appendChild( renderer.domElement )

function render() {
    console.log('render')
    renderer.render(scene, camera)
}

const orbit = new OrbitControls(camera)
orbit.addEventListener('change', render)

const triangle = [
    new THREE.Vector3(1, 0, 0),
    new THREE.Vector3(0, 1, 0),
    new THREE.Vector3(0, 0, 1),
]
// const triangle = [
//     new THREE.Vector3(-3, -5, 0),
//     new THREE.Vector3(0, 2, -4),
//     new THREE.Vector3(3, 5, 2),
// ]

const curve_koefs = [2, 1, 0.5]

// const root_point = new THREE.Vector3(1, 1, 1) // intersect_three_planes TODO

const pivots = [
    new THREE.Vector3(0, 0, 0),
    new THREE.Vector3(0, 0, 0),
    new THREE.Vector3(0, 0, 0),
]
// const pivots = [
//     new THREE.Vector3(0, -7, 0),
//     new THREE.Vector3(0, 0, 0),
//     new THREE.Vector3(10, 0, 0),
// ]

const red_mat = new THREE.LineBasicMaterial({
    // for normal lines
    color: 0xff0000,
    linewidth: 1,
    linecap: 'round', //ignored by WebGLRenderer
    linejoin: 'round', //ignored by WebGLRenderer
 })
const green_mat = new THREE.LineBasicMaterial({
    // for normal lines
    color: 0x00ff00,
    linewidth: 2,
    linecap: 'round', //ignored by WebGLRendererfh
    linejoin: 'round', //ignored by WebGLRenderer
 })
const blue_mat = new THREE.LineBasicMaterial({
    // for normal lines
    color: 0x0000ff,
    linewidth: 1,
    linecap: 'round', //ignored by WebGLRenderer
    linejoin: 'round', //ignored by WebGLRenderer
 })
 const cyan_mat_dash = new THREE.LineDashedMaterial({
    // for normal lines
    color: 0x00ffff,
    linewidth: 1,
    scale: 1,
    dashSize: 0.015,
    gapSize: 0.05
 })
 const magenta_mat_dash = new THREE.LineDashedMaterial({
    // for normal lines
    color: 0xff00ff,
    linewidth: 1,
    scale: 1,
    dashSize: 0.015,
    gapSize: 0.05
 })

draw()

function draw() {
    draw_grid()
    draw_triangle(triangle, red_mat)
    draw_curves()
    draw_triangulation()
    // draw_surface()

    render()
}

function draw_grid() {
    draw_line(
        new THREE.Vector3(10, 0, 0),
        new THREE.Vector3(-10, 0, 0), 
        magenta_mat_dash
    )
    draw_line(
        new THREE.Vector3(0, 10, 0),
        new THREE.Vector3(0, -10, 0), 
        magenta_mat_dash
    )
    draw_line(
        new THREE.Vector3(0, 0, 10),
        new THREE.Vector3(0, 0, -10), 
        magenta_mat_dash
    )
}

/**
 * 
 * @param {[THREE.Vector3, THREE.Vector3, THREE.Vector3]} triangle 
 * @param {THREE.Material} mat 
 */
function draw_triangle(triangle, mat) {
    draw_line(triangle[0], triangle[1], mat)
    draw_line(triangle[1], triangle[2], mat)
    draw_line(triangle[2], triangle[0], mat)
}

function draw_curves() {
    draw_curve(triangle[0], triangle[1], pivots[0], green_mat, curve_koefs[0])
    draw_curve(triangle[1], triangle[2], pivots[1], green_mat, curve_koefs[1])
    draw_curve(triangle[2], triangle[0], pivots[2], green_mat, curve_koefs[2])
}

function draw_triangulation() {
    let sections = 7
    let subsections = 1
    let previous_points = [triangle[1]]
    let triangles = []

    for(let major_step = 1; major_step <= sections; major_step++) {
        let major_interp = major_step / sections
        let left_point = triangle[1].clone().lerp(triangle[0], major_interp)
        let right_point = triangle[1].clone().lerp(triangle[2], major_interp)
        
        // minor cycle
        let current_points = [left_point]
        for(let minor_step = 1; minor_step <= subsections; minor_step++) {
            let minor_interp = minor_step / subsections
            let next_point = left_point.clone().lerp(right_point, minor_interp)
            triangles.push([
                get_surface_point(previous_points[minor_step - 1].clone()),
                get_surface_point(current_points[minor_step - 1].clone()),
                get_surface_point(next_point.clone())
            ])

            current_points.push(next_point)

            console.log(major_step, minor_step)
        }

        previous_points = current_points
        subsections++
        // generate triangles
        // save prev points
    }

    console.log(triangles)
    for(let tr of triangles) {
        draw_triangle(tr, blue_mat)
    }
}

function draw_surface() {
    let external_edges = 10

    for (let i = 0; i <= external_edges; i++) {
        let t = i / external_edges
        let edge_0 = triangle[1].clone().sub(triangle[0]).multiplyScalar(t).add(triangle[0])
        draw_surface_curve(edge_0, triangle[2], blue_mat)
    }

    for (let i = 0; i < external_edges; i++) {
        let t = i / external_edges
        let edge_1 = triangle[2].clone().sub(triangle[1]).multiplyScalar(t).add(triangle[1])
        draw_surface_curve(edge_1, triangle[0], blue_mat)
    }
    
    for (let i = 0; i <= external_edges; i++) {
        let t = i / external_edges
        let edge_2 = triangle[0].clone().sub(triangle[2]).multiplyScalar(t).add(triangle[2])
        draw_surface_curve(edge_2, triangle[1], blue_mat)
    }
}

function draw_line(v1, v2, mat) {
    const geometry = new THREE.BufferGeometry().setFromPoints([v1, v2])
    const line = new THREE.Line(geometry, mat)
    line.computeLineDistances()
    scene.add(line)
}

function draw_curve(v1, v2, p, mat, curve_koef) {
    let points = []
    for(let i = 0; i <= STEPS; i++) {
        const t = i / STEPS
        points.push(curve(t, v1, v2, p, curve_koef))
    }
    const geometry = new THREE.BufferGeometry().setFromPoints(points)
    const line = new THREE.Line(geometry, mat)
    scene.add(line)
}

/**
 * 
 * @param {THREE.Vector3} v1 
 * @param {THREE.Vector3} v2 
 * @param {*} mat 
 * @param {*} steps 
 */
function draw_surface_curve(v1, v2, mat) {
    let points = []
    let tr_points = [v1, v2]
    for(let i = 0; i <= STEPS; i++) {
        const t = i / STEPS
        let point_on_triangle = v2.clone().sub(v1).multiplyScalar(t).add(v1)
        points.push(get_surface_point(point_on_triangle))
    }
    let zero_vec = new THREE.Vector3(0,0,0)
    // points = points.filter(v => !zero_vec.equals(v))
    
    const geometry = new THREE.BufferGeometry().setFromPoints(points)
    const line = new THREE.Line(geometry, mat)
    scene.add(line)
}


/**
 * 
 * @param {THREE.Vector3} point 
 */
function get_surface_point(point) {
    let [w0, w1, w2] = get_barri(point)
    // get points on edges
    let t0 = w2 !== 1 ? w0 / (1. - w2) : 1
    let t1 = w0 !== 1 ? w1 / (1. - w0) : 1
    let t2 = w1 !== 1 ? w2 / (1. - w1) : 1
    
    // get points on curves
    let c0 = curve(1 - t0, triangle[0], triangle[1], pivots[0], curve_koefs[0])
    let c1 = curve(1 - t1, triangle[1], triangle[2], pivots[1], curve_koefs[1])
    let c2 = curve(1 - t2, triangle[2], triangle[0], pivots[2], curve_koefs[2])

    // get interpolation koefs
    let k0 = w0 * t1 + w1 * (1 - t2)
    let k1 = w1 * t2 + w2 * (1 - t0)
    let k2 = w2 * t0 + w0 * (1 - t1)

    let balanced_coef = k0 * curve_koefs[0] + k1 * curve_koefs[1] + k2 * curve_koefs[2]

    let result_point = new THREE.Vector3(
        upow(k0 * upow(c0.x, balanced_coef) + k1 * upow(c1.x, balanced_coef) + k2 * upow(c2.x, balanced_coef), 1 / balanced_coef),
        upow(k0 * upow(c0.y, balanced_coef) + k1 * upow(c1.y, balanced_coef) + k2 * upow(c2.y, balanced_coef), 1 / balanced_coef),
        upow(k0 * upow(c0.z, balanced_coef) + k1 * upow(c1.z, balanced_coef) + k2 * upow(c2.z, balanced_coef), 1 / balanced_coef),
    )

    return result_point
}

/**
 * 
 * @param {THREE.Vector3} point 
 */
function get_barri(point) {
    /*
    vec3  N = cross(v1 - v0, v2 - v0);
    float A = length(N); 
    
    if ( A < 1e-4 )  vec3(0);
    
    v0 -= p;
    v1 -= p;
    v2 -= p;
    
    return N * mat3(
        cross(v1, v2),
        cross(v2, v0),
        cross(v0, v1) 
    ) / (A*A);
    */

    let root = triangle[0]
    let v1 = triangle[1].clone().sub(root)
    let v2 = triangle[2].clone().sub(root)

    let norm = v1.clone().cross(v2)
    let area = norm.length()
    norm.normalize()

    if (area < 1e-4) return [0, 0, 0]

    let tp0 = triangle[0].clone().sub(point)
    let tp1 = triangle[1].clone().sub(point)
    let tp2 = triangle[2].clone().sub(point)

    let asub = (new THREE.Vector3(
        tp1.clone().cross(tp2).dot(norm),
        tp2.clone().cross(tp0).dot(norm),
        tp0.clone().cross(tp1).dot(norm),
    )).divideScalar(area)

    return [
        Math.abs(asub.x),
        Math.abs(asub.y),
        Math.abs(asub.z),
    ]
}

/**
 * 
 * @param {Number} t 
 * @param {THREE.Vector3} v1 
 * @param {THREE.Vector3} v2 
 * @param {THREE.Vector3} pivot 
 * @returns 
 */
function curve(t, v1, v2, pivot, curve_koef) {
    t = Math.min(1., Math.max(t, 0.))
    let s = Math.pow(1. - t, curve_koef)
    let f = Math.pow(t, curve_koef)
    let sqr1 = Math.pow(s / (s + f), 1 / curve_koef)
    let sqr2 = Math.pow(f / (s + f), 1 / curve_koef)

    return v1.clone().multiplyScalar(sqr1).add(
        v2.clone().multiplyScalar(sqr2)
    ).add(
        pivot.clone().multiplyScalar(1. - sqr1 - sqr2)
    )
}

/**
 * 
 * @param {THREE.Vector3} v 
 * @param {Number} p 
 * @returns 
 */
function upow(b, e) {
    return Math.sign(b) * Math.pow(Math.abs(b), e)
}