#![allow(non_snake_case, non_upper_case_globals)]

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsCast;
use web_sys::ImageData;

const _ROTATION_SCALE: f32 = 0.001;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

fn rgb_color(r: f32, g: f32, b: f32, a: f32) -> String {
    format!(
        "rgb({}, {}, {}, {})",
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        (a * 255.0) as u8
    )
}

fn request_animation_frame(
    f: &Closure<dyn FnMut(f32) -> Result<(), JsValue>>,
) -> Result<(), JsValue> {
    web_sys::window()
        .ok_or_else(|| "couldn't create window".to_string())?
        .request_animation_frame(f.as_ref().unchecked_ref())
        .map_err(|_e| "couldn't request animation frame".to_string())?;
    Ok(())
}

fn draw_triangle(
    vertices: &[f32; 9],
    buffer: &mut Clamped<Vec<u8>>,
    image_data: &web_sys::ImageData,
) {
    // Assumptions
    // - the bottom two vertices are on the same line
    // - vertices are in normalized space -1..1
    let color = [
        (1.0 * 255.0) as u8,
        (0.5 * 255.0) as u8,
        (0.2 * 255.0) as u8,
        (1.0 * 255.0) as u8,
    ];
    let mut vertices: Vec<[i32; 3]> = vertices
        .chunks(3)
        .map(|chunk| {
            [
                (image_data.width() as f32 * (chunk[0] + 1.0) / 2.0) as i32,
                image_data.height() as i32
                    - (image_data.height() as f32 * (chunk[1] + 1.0) / 2.0) as i32,
                0,
            ]
        })
        .collect();
    vertices.sort_by(|a, b| a[1].cmp(&b[1]));
    // log(&format!("{}, {}", vertices[0][0], vertices[0][1]));
    // log(&format!("{}, {}", vertices[1][0], vertices[1][1]));
    // log(&format!("{}, {}", vertices[2][0], vertices[2][1]));

    let dy = vertices[0][1] - vertices[1][1];

    let dx1 = vertices[0][0] - vertices[1][0];
    let dx1: f32 = dx1 as f32 / dy as f32;

    let dx2 = vertices[0][0] - vertices[2][0];
    let dx2: f32 = dx2 as f32 / dy as f32;
    let mut dxs = [dx1, dx2];
    dxs.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let start_x = vertices[0][0];

    for row in vertices[0][1]..=vertices[1][1] {
        let y = row - vertices[0][1];
        let start = start_x + (y as f32 * dxs[0]) as i32;
        let end = start_x + (y as f32 * dxs[1]) as i32;
        for column in start..=end {
            for (i, &byte) in color.iter().enumerate() {
                buffer[(row * image_data.width() as i32 * 4 + column * 4 + i as i32) as usize] =
                    byte;
            }
        }
    }
}

fn render(
    context: &web_sys::CanvasRenderingContext2d,
    buffer: &mut Clamped<Vec<u8>>,
    image_data: &web_sys::ImageData,
    aspect: f32,
    _dt: f32,
) -> Result<(), JsValue> {
    let mut triangle = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    context.clear_rect(
        0.0,
        0.0,
        image_data.width().into(),
        image_data.height().into(),
    );

    // let sin_dt = (ROTATION_SCALE * dt).sin();
    // let cos_dt = (ROTATION_SCALE * dt).cos();
    // let rotation = [
    //     [cos_dt, -sin_dt, 0.0, 0.0],
    //     [sin_dt, cos_dt, 0.0, 0.0],
    //     [0.0, 0.0, 1.0, 0.0],
    //     [0.0, 0.0, 0.0, 1.0],
    // ];
    let scale = [
        [aspect, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
    for chunk in triangle.chunks_exact_mut(3) {
        // let x = chunk[0];
        // let y = chunk[1];
        // let z = chunk[2];
        // chunk[0] = rotation[0][0] * x + rotation[1][0] * y + rotation[2][0] * z + rotation[3][0];
        // chunk[1] = rotation[0][1] * x + rotation[1][1] * y + rotation[2][1] * z + rotation[3][1];
        // chunk[2] = rotation[0][2] * x + rotation[1][2] * y + rotation[2][2] * z + rotation[3][2];

        let x = chunk[0];
        let y = chunk[1];
        let z = chunk[2];
        chunk[0] = scale[0][0] * x + scale[1][0] * y + scale[2][0] * z + scale[3][0];
        chunk[1] = scale[0][1] * x + scale[1][1] * y + scale[2][1] * z + scale[3][1];
        chunk[2] = scale[0][2] * x + scale[1][2] * y + scale[2][2] * z + scale[3][2];
    }

    draw_triangle(&triangle, buffer, image_data);

    let data = ImageData::new_with_u8_clamped_array_and_sh(
        wasm_bindgen::Clamped(buffer),
        image_data.width(),
        image_data.height(),
    )?;
    context.put_image_data(&data, 0.0, 0.0)
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    let document = web_sys::window()
        .ok_or("couldn't get window")?
        .document()
        .ok_or("couldn't get document")?;
    let canvas = document
        .get_element_by_id("app")
        .ok_or("couldn't get canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;
    let context = canvas
        .get_context("2d")?
        .ok_or("couldn't get context")?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;
    let image_data =
        context.create_image_data_with_sw_and_sh(canvas.width().into(), canvas.height().into())?;
    let mut buffer = image_data.data();
    let aspect = canvas.height() as f32 / canvas.width() as f32;
    canvas
        .style()
        .set_property("background-color", &rgb_color(0.2, 0.3, 0.3, 1.0))?;

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let mut previous = None;
    *g.borrow_mut() = Some(Closure::new(Box::new(move |timestamp| {
        if previous.is_none() {
            previous = Some(timestamp);
        }
        render(
            &context,
            &mut buffer,
            &image_data,
            aspect,
            timestamp - previous.unwrap(),
        )?;
        request_animation_frame(
            f.borrow()
                .as_ref()
                .ok_or("couldn't start animation frame")?,
        )?;
        Ok(())
    })
        as Box<dyn FnMut(f32) -> Result<(), JsValue>>));
    request_animation_frame(
        g.borrow()
            .as_ref()
            .ok_or("couldn't start animation frame")?,
    )?;
    Ok(())
}
