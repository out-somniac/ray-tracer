use crate::ray::Ray;
use crate::hits::{HitRecord, Hittable};

use cgmath::Vector3;
use cgmath::InnerSpace;

pub struct Sphere {
    pub origin: Vector3<f64>,
    pub radius: f64
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, min_distance: f64, max_distance: f64) -> Option<f64> {
        let oc = ray.origin - self.origin;
        let a: f64 = ray.direction.dot(ray.direction);
        let b: f64 = 2.0 * oc.dot(ray.direction);
        let c: f64 = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b*b - 4.0*a*c;
        if discriminant < 0.0 {
            return None;
        } else {
            return Some(( - b - discriminant.sqrt() ) / ( 2.0 * a ));
        }
    }
}