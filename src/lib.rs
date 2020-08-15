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
use crate::material::{Lambertian,Metal,Material};
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

/*
bool hit_sphere(const point3& center, double radius, const ray& r) {
    vec3 oc = r.origin() - center;
    auto a = dot(r.direction(), r.direction());
    auto b = 2.0 * dot(oc, r.direction());
    auto c = dot(oc, oc) - radius*radius;
    auto discriminant = b*b - 4*a*c;
    return (discriminant > 0);
} */


// fn hit_sphere(center: &Vec3, radius: f32, r: &Ray) -> f32 {
//     let oc = r.origin - *center;
//     let a = r.direction.dot(&r.direction);
//     let b = oc.dot(&r.direction) * 2.0;
//     let c = oc.dot(&oc) - radius * radius;
//     let discriminant = b * b - 4.0 * a * c;
//     //discriminant > 0.0

//     if discriminant < 0.0 {
//          -1.0
//     } else {
//         (-b - discriminant.sqrt() ) / (2.0*a)
//     }

// }


/*
vec3 N = unit_vector(r.at(t) - vec3(0,0,-1));
        return 0.5*color(N.x()+1, N.y()+1, N.z()+1);
*/

// fn color(r: &Ray) -> Color {
//     let t = hit_sphere(&Vec3{x:0.0, y: 0.0, z:-1.0}, 0.5, r);
//     if t > 0.0 {
//         let n = (r.point_at_parameter(t) - Vec3{x:0.0, y: 0.0, z:-1.0}).unit_vector();
//         return Color{r: n.x + 1.0, g: n.y + 1.0, b: n.z + 1.0}  * 0.5
//     }
       
//     let unit_direction = r.direction.unit_vector();
//     let t = 0.5 * (unit_direction.y + 1.0);
//     Color{r:1.0, g: 1.0, b:1.0} * (1.0 - t)  + Color{r:0.5, g: 0.7, b:1.0} * (t) 
// }

/* 

    if (world.hit(r, 0.001, infinity, rec)) {
        ray scattered;
        color attenuation;
        if (rec.mat_ptr->scatter(r, rec, attenuation, scattered))
            return attenuation * ray_color(scattered, world, depth-1);
        return color(0,0,0);
    }

}*/

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        Color{r: 0.0, g: 0.0, b: 0.0};
    }

    if let Some(hitt) = world.hit(r, 0.001, f32::INFINITY) {
        let t = hitt.p + hitt.normal + Vec3::random_in_hemisphere(&hitt.normal);
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
    Color{r:1.0, g: 1.0, b:1.0} * (1.0 - t)  + Color{r:0.5, g: 0.7, b:1.0} * (t) 
}

fn  clamp(input: f32, min: f32, max: f32) -> f32 {
    if input < min { 
        return min 
    }
    if input > max { 
        return max 
    }
    input  
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
    let samples_per_pixel = 1.0;
    let max_depth = 1;
    let mut data: Vec<u8> = Vec::new();

    // World
    /*
        auto material_ground = make_shared<lambertian>(color(0.8, 0.8, 0.0));
    auto material_center = make_shared<lambertian>(color(0.7, 0.3, 0.3));
    auto material_left   = make_shared<metal>(color(0.8, 0.8, 0.8));
    auto material_right  = make_shared<metal>(color(0.8, 0.6, 0.2));
    */

    let material_ground = Lambertian{ albedo: Color{r: 0.8, g: 0.8, b: 0.0 } };
    let material_center = Lambertian{ albedo: Color{r: 0.7, g: 0.3, b: 0.3 } };
    let material_left   = Metal{ albedo: Color{r: 0.8, g: 0.8, b: 0.8 } };
    let material_right  = Metal{ albedo: Color{r: 0.8, g: 0.6, b: 0.2 } };

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
            for k in 0..(samples_per_pixel as i32) {
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
    let width = 200;
    let height = 100;

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