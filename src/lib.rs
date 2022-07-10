#![allow(non_snake_case, non_upper_case_globals)]

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

const ROTATION_SCALE: f32 = 0.001;

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
    context: &web_sys::CanvasRenderingContext2d,
    canvas: &web_sys::HtmlCanvasElement,
) -> Result<(), JsValue> {
    context.begin_path();
    context.set_fill_style(&rgb_color(1.0, 0.5, 0.2, 1.0).into());
    context.set_stroke_style(&rgb_color(1.0, 0.5, 0.2, 1.0).into());

    let [mut x, mut y, ..] = vertices;
    x = ((x + 1.0) / 2.0) * canvas.width() as f32;
    y = canvas.height() as f32 - ((y + 1.0) / 2.0) * canvas.height() as f32;

    context.move_to(x.into(), y.into());

    for chunk in vertices[3..].chunks(3) {
        let x = ((chunk[0] + 1.0) / 2.0) * canvas.width() as f32;
        let y = canvas.height() as f32 - ((chunk[1] + 1.0) / 2.0) * canvas.height() as f32;
        context.line_to(x.into(), y.into());
    }
    context.line_to(x.into(), y.into());

    context.stroke();
    context.fill();
    Ok(())
}

fn render(
    context: &web_sys::CanvasRenderingContext2d,
    canvas: &web_sys::HtmlCanvasElement,
    aspect: f32,
    dt: f32,
) -> Result<(), JsValue> {
    let mut triangle = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
    context.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

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

    draw_triangle(&triangle, context, canvas)
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
        render(&context, &canvas, aspect, timestamp - previous.unwrap())?;
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
