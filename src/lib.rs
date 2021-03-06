#![allow(non_snake_case, non_upper_case_globals)]

use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::ImageData;

const ROTATION_SCALE: f32 = 0.001;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

type Result<T> = std::result::Result<T, JsValue>;

struct Renderer {
    height: u32,
    width: u32,
    context: web_sys::CanvasRenderingContext2d,
    buffer: Vec<u8>,
}

impl Renderer {
    fn new() -> Result<Self> {
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
        let image_data = context
            .create_image_data_with_sw_and_sh(canvas.width().into(), canvas.height().into())?;
        let buffer = image_data.data().to_vec();
        canvas
            .style()
            .set_property("background-color", &rgb_color(0.2, 0.3, 0.3, 1.0))?;

        Ok(Self {
            context,
            buffer,
            height: image_data.height(),
            width: image_data.width(),
        })
    }

    fn render(&mut self, dt: f32) -> Result<()> {
        let mut triangle = [-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
        let aspect = self.height as f32 / self.width as f32;

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
            chunk[0] =
                rotation[0][0] * x + rotation[1][0] * y + rotation[2][0] * z + rotation[3][0];
            chunk[1] =
                rotation[0][1] * x + rotation[1][1] * y + rotation[2][1] * z + rotation[3][1];
            chunk[2] =
                rotation[0][2] * x + rotation[1][2] * y + rotation[2][2] * z + rotation[3][2];

            let x = chunk[0];
            let y = chunk[1];
            let z = chunk[2];
            chunk[0] = scale[0][0] * x + scale[1][0] * y + scale[2][0] * z + scale[3][0];
            chunk[1] = scale[0][1] * x + scale[1][1] * y + scale[2][1] * z + scale[3][1];
            chunk[2] = scale[0][2] * x + scale[1][2] * y + scale[2][2] * z + scale[3][2];
        }

        self.buffer.iter_mut().for_each(|byte| *byte = 0);
        self.draw_triangle(&triangle);

        let data = ImageData::new_with_u8_clamped_array_and_sh(
            wasm_bindgen::Clamped(&self.buffer),
            self.width,
            self.height,
        )?;
        self.context.put_image_data(&data, 0.0, 0.0)
    }

    fn draw_triangle(&mut self, vertices: &[f32; 9]) {
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
                    (self.width as f32 * (chunk[0] + 1.0) / 2.0) as i32,
                    self.height as i32 - (self.height as f32 * (chunk[1] + 1.0) / 2.0) as i32,
                    0,
                ]
            })
            .collect();
        vertices.sort_by(|a, b| a[1].cmp(&b[1]));

        let dy = (vertices[0][1] - vertices[1][1]).abs();

        let dxl = vertices[1][0] - vertices[0][0];
        let mut dxl: f32 = dxl as f32 / dy as f32;

        let dxr = vertices[2][0] - vertices[0][0];
        let mut dxr: f32 = dxr as f32 / dy as f32;

        if dxl > dxr {
            (dxl, dxr) = (dxr, dxl);
        }
        let start_x = vertices[0][0];
        self.buffer
            .chunks_mut(self.width as usize * 4)
            .skip(vertices[0][1] as usize)
            .take(dy as usize)
            .enumerate()
            .for_each(|(i, row)| {
                let start = (start_x + (i as f32 * dxl) as i32) as usize * 4;
                let stop = (start_x + (i as f32 * dxr) as i32) as usize * 4;
                row[start..stop].copy_from_slice(
                    &std::iter::repeat(color)
                        .flatten()
                        .take(stop - start)
                        .collect::<Vec<u8>>(),
                );
            });
    }
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

fn request_animation_frame(f: &Closure<dyn FnMut(f32) -> Result<()>>) -> Result<()> {
    web_sys::window()
        .ok_or_else(|| "couldn't create window".to_string())?
        .request_animation_frame(f.as_ref().unchecked_ref())
        .map_err(|_e| "couldn't request animation frame".to_string())?;
    Ok(())
}

#[wasm_bindgen(start)]
pub fn main() -> Result<()> {
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();
    let mut renderer = Renderer::new()?;

    *g.borrow_mut() = Some(Closure::new(Box::new(move |timestamp| {
        renderer.render(timestamp)?;
        request_animation_frame(
            f.borrow()
                .as_ref()
                .ok_or("couldn't start animation frame")?,
        )?;

        Ok(())
    }) as Box<dyn FnMut(f32) -> Result<()>>));
    request_animation_frame(
        g.borrow()
            .as_ref()
            .ok_or("couldn't start animation frame")?,
    )?;
    Ok(())
}
