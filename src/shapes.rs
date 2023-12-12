use crate::ray::Ray;
use crate::hits::{HitRecord, Hittable};
use crate::interval::Interval;
use cgmath::Vector3;
use cgmath::InnerSpace;

pub struct Sphere {
    pub origin: Vector3<f64>,
    pub radius: f64
}

impl Hittable for Vec<Box<dyn Hittable>> {
    fn hit(&self, ray: Ray, render_bounds: &Interval) -> Option<HitRecord> {
        let hits = self
            .into_iter()
            .map(|obj| obj.hit(ray, render_bounds))
            .filter(|hit| hit.is_some())
            .map(|hit| hit.unwrap());
    
        let closest_hit = hits
            .min_by(|a, b| a.partial_cmp(&b).expect("Failed during HitRecord partial comparison"));

        return closest_hit;
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, render_bounds: &Interval) -> Option<HitRecord> {
        // TODO: This function should be refactored once I have some time
        let oc = ray.origin - self.origin;
        let a: f64 = ray.direction.dot(ray.direction);
        let half_b: f64 = ray.direction.dot(oc);
        let c: f64 = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 { return None; }

        let sqrtd = discriminant.sqrt();
        
        let left_root = (-half_b - sqrtd) / a;
        let right_root = (-half_b + sqrtd) / a;

        let mut closest_root = left_root;
        if render_bounds.is_outside(left_root) {
            closest_root = right_root;
            if render_bounds.is_outside(right_root) {
                return None;
            }
        }

        let outward_normal = (ray.at(closest_root) - self.origin) / self.radius;
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        
        return Some(HitRecord {
            hit: ray.at(closest_root),
            distance: closest_root,
            normal: if front_face { outward_normal } else { -outward_normal }
        });
    }
}