use crate::ray::Ray;
use cgmath::Vector3;
use crate::interval::Interval;
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub hit: Vector3<f64>,
    pub distance: f64,
    pub normal: Vector3<f64>
}

impl PartialOrd for HitRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl PartialEq for HitRecord {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, render_bounds: &Interval<f64>) -> Option<HitRecord>;
}
