use crate::ray::Ray;
use crate::hits::{HitRecord, Hittable};

use cgmath::Vector3;
use cgmath::InnerSpace;

#[derive(Debug)]
pub struct Sphere {
    pub origin: Vector3<f64>,
    pub radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord> {
        // TODO: This function should be refactored once I have some time
        let oc = ray.origin - self.origin;
        let a: f64 = ray.direction.dot(ray.direction);
        let half_b: f64 = oc.dot(ray.direction);
        let c: f64 = oc.dot(oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 { return None; }

        let sqrtd = discriminant.sqrt();
        
        let is_inside_interval = |x| {
            return min_distance <= x ||  x <= max_distance;
        };
        let left_root = (-half_b - sqrtd) / a;
        let right_root = (-half_b - sqrtd) / a;

        let mut closest_root = left_root;
        if !is_inside_interval(left_root) {
            closest_root = right_root;
            if !is_inside_interval(right_root) {
                return None;
            }
        }
        return Some(HitRecord {
            hit: ray.at(closest_root),
            distance: closest_root,
            normal: (ray.at(closest_root) - self.origin) / self.radius
        });
    }
}