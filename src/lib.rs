pub mod utils;
pub mod vec3;
pub mod ray;
pub mod color;
pub mod hittable;
pub mod sphere;

use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::Color;

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

/*
bool hit_sphere(const point3& center, double radius, const ray& r) {
    vec3 oc = r.origin() - center;
    auto a = dot(r.direction(), r.direction());
    auto b = 2.0 * dot(oc, r.direction());
    auto c = dot(oc, oc) - radius*radius;
    auto discriminant = b*b - 4*a*c;
    return (discriminant > 0);
} */


fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin - *center;
    let a = r.direction.dot(&r.direction);
    let b = oc.dot(&r.direction) * 2.0;
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    //discriminant > 0.0

    if discriminant < 0.0 {
         -1.0
    } else {
        (-b - discriminant.sqrt() ) / (2.0*a)
    }

}


/*
vec3 N = unit_vector(r.at(t) - vec3(0,0,-1));
        return 0.5*color(N.x()+1, N.y()+1, N.z()+1);
*/

fn color(r: &Ray) -> Color {
    let t = hit_sphere(&Vec3{x:0.0, y: 0.0, z:-1.0}, 0.5, r);
    if t > 0.0 {
        let n = (r.point_at_parameter(t) - Vec3{x:0.0, y: 0.0, z:-1.0}).unit_vector();
        return Color{r: n.x + 1.0, g: n.y + 1.0, b: n.z + 1.0}  * 0.5
    }
       
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    Color{r:1.0, g: 1.0, b:1.0} * (1.0 - t)  + Color{r:0.5, g: 0.7, b:1.0} * (t) 
}


fn plot(width: u32, height: u32) -> Vec<u8> {
    let nx = width;
    let ny = height;
    let lower_left_corner = Vec3{x: -2.0, y: -1.0, z: -1.0};
    let horizontal = Vec3{x: 4.0, y: 0.0, z: 0.0};
    let vertical = Vec3{x: 0.0, y: 2.0, z: 0.0};
    let origin = Vec3{x: 0.0, y: 0.0, z: 0.0};
    let mut data = Vec::new();
    for nj in 0..ny {
        let j = ny - nj - 1;
        for i in 0..nx {
            let u = (i as f32) / (nx as f32);
            let v = (j as f32) / (ny as f32);
            let r = Ray{ origin: origin, direction: lower_left_corner + (horizontal * u)  + vertical * v  };
            let col = color(&r) * 255.0;

            // console::log_1(&JsValue::from_str(&col.to_str()));
            data.push((col.r) as u8);
            data.push((col.g) as u8);
            data.push((col.b) as u8);
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


    // Your code goes here!
    // config variables
    let width = 400;
    let height = 200;
    
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



    let mut data = plot(width, height);

    let data = ImageData::new_with_u8_clamped_array_and_sh(Clamped(&mut data), width, height)?;
    context.put_image_data(&data, 0.0, 0.0)?;
    
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
