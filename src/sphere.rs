use crate::material::Material;
use crate::ray::Ray;
use crate::hittable::{Hittable,HitRecord};
use crate::vec3::Vec3;

// #[derive(Clone)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>
}



impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max:f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.squared_length();
        let half_b = oc.dot(&r.direction);
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = half_b* half_b - a * c;

        if discriminant <= 0.0 { return None;}
        let root = discriminant.sqrt();
        let mut temp = (-half_b - root) / a;
        if temp < t_max && temp > t_min {
            let mut ret = HitRecord{
                t: temp,
                p: r.point_at_parameter(temp),
                normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                front_face: true,
                material: &self.material
            };
            ret.set_face_normal(r, (ret.p - self.center)/self.radius);
            return Some(ret);
        }

        temp = (-half_b + root) / a;
        if temp < t_max && temp > t_min {
            let mut ret = HitRecord{
                t: temp,
                p: r.point_at_parameter(temp),
                normal: (r.point_at_parameter(temp) - self.center) / self.radius,
                front_face: true,
                material: &self.material
            };
            ret.set_face_normal(r, (ret.p - self.center)/self.radius);
            return Some(ret);
        }
        return None;
    }
}