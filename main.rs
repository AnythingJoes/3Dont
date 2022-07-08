#![no_main]

extern "C" {
    fn draw_triangle(vertices: *const f32);
}

#[no_mangle]
extern "C" fn render() {
    let triangle = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    unsafe { draw_triangle(triangle.as_ptr()) }
}
