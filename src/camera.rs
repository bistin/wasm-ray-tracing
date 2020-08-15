use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
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

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 8.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3{x:0.0, y: 0.0, z: 0.0};
        let horizontal = Vec3{x:viewport_width, y: 0.0, z: 0.0};
        let vertical = Vec3{x:0.0, y: viewport_height, z: 0.0};

        Self {
            origin,
            horizontal,
            vertical, 
            lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - Vec3{x:0.0, y:0.0, z:focal_length}
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        return  Ray{ 
            origin: self.origin, 
            direction: self.lower_left_corner + (self.horizontal * u)  + self.vertical * v  
        };
    }

}