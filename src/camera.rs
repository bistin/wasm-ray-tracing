use crate::ray::Ray;
use crate::vec3::Vec3;
use js_sys::Math;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32
}


/*
    camera() {
            auto aspect_ratio = 16.0 / 9.0;
            auto viewport_height = 2.0;
            auto viewport_width = aspect_ratio * viewport_height;
            auto focal_length = 1.0;

            origin = point3(0, 0, 0);
            horizontal = vec3(viewport_width, 0.0, 0.0);
            vertical = vec3(0.0, viewport_height, 0.0);
            lower_left_corner = origin - horizontal/2 - vertical/2 - vec3(0, 0, focal_length);
        }
         ray get_ray(double u, double v) const {
            return ray(origin, lower_left_corner + u*horizontal + v*vertical - origin);
        }

*/


fn degrees_to_radians(degrees: f32) -> f32 {
    return degrees * std::f32::consts::PI / 180.0;
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f32, aspect_ratio: f32, aperture: f32, focus_dist: f32) -> Self {
        // let viewport_height = 2.0;
        // let viewport_width = aspect_ratio * viewport_height;

        let theta = degrees_to_radians(vfov);
        let h = Math::tan((theta/2.0).into());
        let viewport_height = 2.0 * h as f32;
        let viewport_width = aspect_ratio * viewport_height;
    

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = u * (viewport_width * focus_dist);
        let vertical = v * (viewport_height * focus_dist);

        let lens_radius = aperture / 2.0;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - w * focus_dist,
            u,
            v,
            w,
            lens_radius
        }
    }

    /*
    
    ray get_ray(double s, double t) const {
            vec3 rd = lens_radius * random_in_unit_disk();
            vec3 offset = u * rd.x() + v * rd.y();

            return ray(
                origin + offset,
                lower_left_corner + s*horizontal + t*vertical - origin - offset
            );
        }
    
    */
    pub fn get_ray(&self, s: f32, t: f32) -> Ray {

        let rd = Vec3::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        return  Ray{ 
            origin: self.origin + offset, 
            direction: self.lower_left_corner + (self.horizontal * s)  + self.vertical * t - self.origin - offset 
        };
    }

}