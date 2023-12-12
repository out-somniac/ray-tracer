use crate::ray::Ray;
use cgmath::Vector3;
use crate::interval::Interval;
use std::cmp::Ordering;
use crate::material::Material;
use std::rc::Rc;

pub struct HitRecord {
    pub hit: Vector3<f64>,
    pub distance: f64,
    pub normal: Vector3<f64>,
    pub material: Rc<dyn Material>
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
    fn hit(&self, ray: Ray, render_bounds: &Interval) -> Option<HitRecord>;
}
