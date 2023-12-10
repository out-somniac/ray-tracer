use crate::ray::Ray;
use cgmath::Vector3;

pub struct HitRecord {
    pub hit: Vector3<f64>,
    pub distance: f64,
    pub normal: Vector3<f64>
}

pub trait Hittable {
    fn hit(&self, ray: Ray, min_distance: f64, max_distance: f64) -> Option<HitRecord>;
}