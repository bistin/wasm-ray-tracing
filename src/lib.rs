pub mod utils;
pub mod vec3;
pub mod ray;
pub mod color;
pub mod hittable;
pub mod sphere;
pub mod hittable_list;
pub mod camera;
pub mod material;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::hittable::Hittable;
use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::Color;
use crate::material::{Lambertian,Metal,Dielectric};
use rand::Rng;

use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
// use web_sys::console;
use web_sys::{ImageData};


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    //console::log_1(&JsValue::from_str( &format!("{}",depth) ));
    if depth <= 0 {
        return Color{r: 0.0, g: 0.0, b: 0.0};
    }

    if let Some(hitt) = world.hit(r, 0.001, std::f32::INFINITY) {
        if let Some(scatt) = hitt.material.scatter(r, &hitt) {
            let a = ray_color(&scatt.scattered, world, depth -1);
            let b = scatt.attenuation;
            return Color{
                r: a.r * b.r,
                g: a.g * b.g,
                b: a.b * b.b
            }
        }
        return Color{
            r: 0.0,
            g: 0.0,
            b: 0.0
        }
    }
    let unit_direction = r.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    return Color{r:1.0, g: 1.0, b:1.0} * (1.0 - t)  + Color{r:0.5, g: 0.7, b:1.0} * (t);
}

fn clamp(input: f32, min: f32, max: f32) -> f32 {
    if input < min { 
        return min 
    }
    if input > max { 
        return max 
    }
    return input;
}


fn write_color(data: &mut Vec<u8>, color :&Color, samples_per_pixel: f32) {
    let scale = 1.0 / samples_per_pixel;
    let r = clamp((color.r * scale).sqrt(), 0.0, 0.999);
    let g = clamp((color.g * scale).sqrt(), 0.0, 0.999);
    let b = clamp((color.b * scale).sqrt(), 0.0, 0.999);

    data.push((r * 255.0) as u8);
    data.push((g * 255.0) as u8);
    data.push((b * 255.0) as u8);
    data.push(255);
}
 

fn plot(width: u32, height: u32) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    // Image
    let nx = width;
    let ny = height; 
    let samples_per_pixel = 20.0;
    let max_depth = 50;
    let mut data: Vec<u8> = Vec::new();

    // World
    let material_ground = Lambertian{ albedo: Color{r: 0.8, g: 0.8, b: 0.0 } };
    let material_center = Lambertian{ albedo: Color{r: 0.1, g: 0.2, b: 0.5 } };
    let material_left   = Dielectric{ ref_idx: 1.5}; //Metal{ albedo: Color{r: 0.8, g: 0.8, b: 0.8 }, fuzz: 0.3 };
    let material_left2   = Dielectric{ ref_idx: 1.5};
    let material_right  = Metal{ albedo: Color{r: 0.8, g: 0.6, b: 0.2 }, fuzz: 0.0 };

    let mut hitables: Vec<Box<dyn Hittable>> = Vec::new();
    hitables.push(Box::new(Sphere {
        center: Vec3{x: 0.0, y: 0.0, z: -1.0},
        radius: 0.5,
        material: Box::new(material_center)
    }));

    hitables.push(Box::new(Sphere {
        center: Vec3{x: 0.0, y: -100.5, z: -1.0},
        radius: 100.0,
        material: Box::new(material_ground)
    }));

    hitables.push(Box::new(Sphere {
        center: Vec3{x: -1.0, y: 0.0, z: -1.0},
        radius: 0.5,
        material: Box::new(material_left)
    }));

    hitables.push(Box::new(Sphere {
        center: Vec3{x: -1.0, y: 0.0, z: -1.0},
        radius: -0.4,
        material: Box::new(material_left2)
    }));

    hitables.push(Box::new(Sphere {
        center: Vec3{x: 1.0, y: 0.0, z: -1.0},
        radius: 0.5,
        material: Box::new(material_right)
    }));

    let world = HittableList{hitables};

    // Camera
    let cam = Camera::new();
    for nj in 0..ny {
        let j = ny - nj - 1;
        for i in 0..nx {
            let mut col = Color{ r:0.0, g:0.0, b: 0.0};
            for _k in 0..(samples_per_pixel as i32) {
                let u = ((i as f32) + rng.gen_range(0.0, 1.0))  / (nx as f32);
                let v = ((j as f32) + rng.gen_range(0.0, 1.0)) / (ny as f32);
                let r = cam.get_ray(u, v);
                col = ray_color(&r, &world, max_depth) + col;
            }
            write_color(&mut data, &col, samples_per_pixel);
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
    let width = 100;
    let height = 50;

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

    Ok(())
}