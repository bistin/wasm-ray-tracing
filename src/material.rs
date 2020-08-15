
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
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {


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

/*
class metal : public material {
    public:
        metal(const color& a) : albedo(a) {}

        virtual bool scatter(
            const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered
        ) const override {
            vec3 reflected = reflect(unit_vector(r_in.direction()), rec.normal);
            scattered = ray(rec.p, reflected);
            attenuation = albedo;
            return (dot(scattered.direction(), rec.normal) > 0);
        }

    public:
        color albedo;
};
*/

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