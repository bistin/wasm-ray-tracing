// from https://github.com/nwtgck/ray-tracing-iow-rust/blob/develop/src/vec3.rs

use js_sys::Math;
use std::ops::{Add, Neg, Sub, Mul, Div};
use rand::Rng;
use std::f32::consts;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(value x: {}, value y: {}, value z: {})", self.x, self.y, self.z)
    }
}

impl Vec3 {
    // This is a static method
    // Static methods don't need to be called by an instance
    // These methods are generally used as constructors
    pub fn random() -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 { 
            x: rng.gen_range(0.0, 1.0), 
            y: rng.gen_range(0.0, 1.0), 
            z: rng.gen_range(0.0, 1.0)
        }
    }

    pub fn random_range(min: f32, max: f32) -> Vec3 {
        let mut rng = rand::thread_rng();
        Vec3 { 
            x: rng.gen_range(min, max), 
            y: rng.gen_range(min, max), 
            z: rng.gen_range(min, max)
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut p = Vec3::random_range(-1.0, 1.0);
        loop {
            if p.squared_length() < 1.0 {
                break;
            } 
            p = Vec3::random_range(-1.0, 1.0);
        }
        return p;
    }
    /*
    auto a = random_double(0, 2*pi);
    auto z = random_double(-1, 1);
    auto r = sqrt(1 - z*z);
    return vec3(r*cos(a), r*sin(a), z);
    */
    pub fn random_unit_vector() -> Vec3 {
        let mut rng = rand::thread_rng();
        let a = (rng.gen::<f32>() * 2.0 * consts::PI) as f64;
        let z = rng.gen_range(-1.0, 1.0) as f32;
        let r = ((1.0) - (z * z) ).sqrt() as f32;
        return Vec3{
            x: r * Math::cos(a) as f32,
            y: r * Math::sin(a) as f32,
            z: z
        }
    }
    /*
    vec3 random_in_hemisphere(const vec3& normal) {
        vec3 in_unit_sphere = random_in_unit_sphere();
        if (dot(in_unit_sphere, normal) > 0.0) // In the same hemisphere as the normal
            return in_unit_sphere;
        else
            return -in_unit_sphere;
    }*/

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            return in_unit_sphere;
        } else {
            return in_unit_sphere * -1.0;
        }
    }

    /*
    vec3 random_in_unit_disk() {
        while (true) {
            auto p = vec3(random_double(-1,1), random_double(-1,1), 0);
            if (p.length_squared() >= 1) continue;
            return p;
        }
    }
    */
    pub fn random_in_unit_disk() -> Vec3 {
        let mut rng = rand::thread_rng();
        let mut p = Vec3 { 
            x: rng.gen_range(-1.0, 1.0), 
            y: rng.gen_range(-1.0, 1.0), 
            z: 0.0
        };
        loop {
            if p.squared_length() < 1.0 {
                break;
            } 
            p = Vec3 { 
                x: rng.gen_range(-1.0, 1.0), 
                y: rng.gen_range(-1.0, 1.0), 
                z: 0.0
            };
        }
        return p;
    }
}

impl Vec3 {
    // Length
    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }
    // Squared length
    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    // Unit vector
    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
    // Inner product
    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    // Cross product
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y*rhs.z - rhs.y * self.z,
            y: self.z*rhs.x - rhs.z * self.x,
            z: self.x*rhs.y - rhs.x * self.y
        }
    }

    pub fn to_str(&self) -> String {
    
        format!("(value x: {}, value y: {}, value z: {})", self.x, self.y, self.z)
    }

    pub fn reflect(self, n: &Vec3) -> Vec3 {
        return self - n * (self.dot(n) * 2.0);
    } 
    /*
vec3 refract(const vec3& uv, const vec3& n, double etai_over_etat) {
    auto cos_theta = dot(-uv, n);
    vec3 r_out_perp =  etai_over_etat * (uv + cos_theta*n);
    vec3 r_out_parallel = -sqrt(fabs(1.0 - r_out_perp.length_squared())) * n;
    return r_out_perp + r_out_parallel;
}    
    */
    pub fn refract(&self, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = (self * -1.0).dot(n);
        let r_out_perp = (self + &(n * cos_theta)) * etai_over_etat;
        let r_out_parallel = n * (1.0 - r_out_perp.squared_length()).abs().sqrt() * -1.0;
        return r_out_perp + r_out_parallel;
    }
}

// why &Vec3
// reference https://stackoverflow.com/questions/24594374/how-can-an-operator-be-overloaded-for-different-rhs-types-and-return-values

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3{x: -self.x, y: -self.y, z: -self.z}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}


impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3 {x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}