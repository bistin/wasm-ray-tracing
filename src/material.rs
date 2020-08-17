
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
    pub albedo: Color,
    pub fuzz: f32
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {
        let reflected = r_in.direction.unit_vector().reflect(&hit_record.normal);
        if reflected.dot(&hit_record.normal) > 0.0 {
            return Some(ScatterRecord{
                attenuation: self.albedo,
                scattered: Ray{
                    origin: hit_record.p,
                    direction: reflected + Vec3::random_in_unit_sphere() * self.fuzz
                }
            });
        } else {
            return None;
        }     
    }
}


pub struct Dielectric {
    pub ref_idx: f32
}

/*
virtual bool scatter(
            const ray& r_in, const hit_record& rec, color& attenuation, ray& scattered
        ) const override {
            attenuation = color(1.0, 1.0, 1.0);
            double etai_over_etat = rec.front_face ? (1.0 / ref_idx) : ref_idx;

            vec3 unit_direction = unit_vector(r_in.direction());
            vec3 refracted = refract(unit_direction, rec.normal, etai_over_etat);
            scattered = ray(rec.p, refracted);
            return true;
        }


*/

/*
double schlick(double cosine, double ref_idx) {
    auto r0 = (1-ref_idx) / (1+ref_idx);
    r0 = r0*r0;
    return r0 + (1-r0)*pow((1 - cosine),5);
}*/


fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}



impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_record: &HitRecord) -> Option<ScatterRecord> {

        let etai_over_etat = if hit_record.front_face { (1.0 / self.ref_idx) } else { self.ref_idx};
        let unit_direction = r_in.direction.unit_vector();

        /*
            double cos_theta = fmin(dot(-unit_direction, rec.normal), 1.0);
            double sin_theta = sqrt(1.0 - cos_theta*cos_theta);
            if (etai_over_etat * sin_theta > 1.0 ) {
                vec3 reflected = reflect(unit_direction, rec.normal);
                scattered = ray(rec.p, reflected);
                return true;
            }
        */
        let cos_theta = (unit_direction * -1.0).dot(&hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        if etai_over_etat * sin_theta > 1.0 {
            let reflected = unit_direction.reflect(&hit_record.normal);
            return Some(ScatterRecord{
                attenuation: Color{r:1.0,g:1.0,b:1.0},
                scattered: Ray{
                    origin: hit_record.p,
                    direction: reflected
                }
            });
        }
        /*
         double reflect_prob = schlick(cos_theta, etai_over_etat);
            if (random_double() < reflect_prob)
            {
                vec3 reflected = reflect(unit_direction, rec.normal);
                scattered = ray(rec.p, reflected);
                return true;
            }
        */
        let reflect_prob = schlick(cos_theta, etai_over_etat);
        if Vec3::random_range(0.0, 1.0).x < reflect_prob {
            let reflected = unit_direction.reflect(&hit_record.normal);
            return Some(ScatterRecord{
                attenuation: Color{r:1.0,g:1.0,b:1.0},
                scattered: Ray{
                    origin: hit_record.p,
                    direction: reflected
                }
            });
        }

        let refracted = unit_direction.refract(&hit_record.normal, etai_over_etat);
        
        return Some(ScatterRecord{
            attenuation: Color{r:1.0,g:1.0,b:1.0},
            scattered: Ray{
                origin: hit_record.p,
                direction: refracted
            }
        });

    }
}
