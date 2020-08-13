use crate::ray::Ray;
use crate::hittable::{Hittable,HitRecord};


pub struct HittableList {
    pub hitables: Vec<Box<dyn Hittable>>
}



impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max:f32) -> Option<HitRecord> {
        // let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut hit_result: Option<HitRecord> = None;

        for object in self.hitables.iter() {
            //let hit_result = object.hit(r, t_min, closest_so_far);
            if let Some(hit_record) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                hit_result = Some(hit_record);
            }
        }
        hit_result
    }
}


/*
bool hittable_list::hit(const ray& r, double t_min, double t_max, hit_record& rec) const {
    hit_record temp_rec;
    bool hit_anything = false;
    auto closest_so_far = t_max;

    for (const auto& object : objects) {
        if (object->hit(r, t_min, closest_so_far, temp_rec)) {
            hit_anything = true;
            closest_so_far = temp_rec.t;
            rec = temp_rec;
        }
    }

    return hit_anything;
}

*/