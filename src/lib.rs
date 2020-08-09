pub mod utils;
pub mod vec3;

use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::{ImageData};



// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn get_julia_set(width: u32, height: u32) -> Vec<u8> {
    let mut data = Vec::new();
    for x in 0..width {
        for y in 0..height {
            data.push(x as u8);
            data.push(y as u8);
            data.push(((x + y)/2) as u8);
            data.push(255);
        }
    }
    data
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
    
    //console::
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

  

    let mut data = get_julia_set(256, 256);

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), 256, 256)?;
    context.put_image_data(&data, 200.0, 0.0)?;
    console::log_1(&JsValue::from_str("Hello world22222!"));

    Ok(())
}


  // context.begin_path();

    // // Draw the outer circle.
    // context
    //     .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
    //     .unwrap();

    // // Draw the mouth.
    // context.move_to(110.0, 75.0);
    // context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // // Draw the left eye.
    // context.move_to(65.0, 65.0);
    // context
    //     .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
    //     .unwrap();

    // // Draw the right eye.
    // context.move_to(95.0, 65.0);
    // context
    //     .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
    //     .unwrap();

    // context.stroke();