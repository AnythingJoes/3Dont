#![no_main]

const ROTATION_SCALE: f32 = 0.001;

extern "C" {
    fn draw_triangle(vertices: *const f32);
}

#[no_mangle]
extern "C" fn render(dt: f32, aspect: f32) {
    let mut triangle = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    let sin_dt = (ROTATION_SCALE * dt).sin();
    let cos_dt = (ROTATION_SCALE * dt).cos();
    let rotation = [
        [cos_dt, -sin_dt, 0.0, 0.0],
        [sin_dt, cos_dt, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    let scale = [
        [aspect, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    for chunk in triangle.chunks_exact_mut(3) {
        let x = chunk[0];
        let y = chunk[1];
        let z = chunk[2];
        chunk[0] = rotation[0][0] * x + rotation[1][0] * y + rotation[2][0] * z + rotation[3][0];
        chunk[1] = rotation[0][1] * x + rotation[1][1] * y + rotation[2][1] * z + rotation[3][1];
        chunk[2] = rotation[0][2] * x + rotation[1][2] * y + rotation[2][2] * z + rotation[3][2];

        let x = chunk[0];
        let y = chunk[1];
        let z = chunk[2];
        chunk[0] = scale[0][0] * x + scale[1][0] * y + scale[2][0] * z + scale[3][0];
        chunk[1] = scale[0][1] * x + scale[1][1] * y + scale[2][1] * z + scale[3][1];
        chunk[2] = scale[0][2] * x + scale[1][2] * y + scale[2][2] * z + scale[3][2];
    }

    unsafe { draw_triangle(triangle.as_ptr()) }
}
