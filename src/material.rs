
use crate::hittable::HitRecord;
use crate::color::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct ScatterRecord {
    pub scattered: Ray,
    pub attenuation: Color,
}


pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord>;
}

pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        return Some(ScatterRecord{
            attenuation: self.albedo,
            scattered: Ray{
                origin: hit_record.p,
                direction: hit_record.normal + Vec3::random_in_unit_sphere()
            }
        });
    }
}


pub struct Metal {
    pub albedo: Color
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = r_in.direction.unit_vector().reflect(&hit_record.normal);
        if reflected.dot(&hit_record.normal) > 0.0 {
            return Some(ScatterRecord{
                attenuation: self.albedo,
                scattered: Ray{
                    origin: hit_record.p,
                    direction: reflected
                }
            });
        } else {
            return None;
        }     
    }
}