use crate::ray::Ray;
use crate::hits::{HitRecord, Hittable};
use crate::interval::Interval;
use cgmath::Vector3;
use cgmath::InnerSpace;

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

pub struct Sphere {
    pub origin: Vector3<f64>,
    pub radius: f64
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

pub struct Triangle {
    pub vertices: [Vector3<f64>; 3]
}

impl Triangle {
    fn is_inside(&self, intersection: Vector3<f64>, normal: Vector3<f64>) -> bool {
        // Edge 0 
        let edge0 = self.vertices[1] - self.vertices[0];
        let c0 = edge0.cross(intersection - self.vertices[0]);
        if normal.dot(c0) < 0.0 { return false; }

        // Edge 1
        let edge1 = self.vertices[2] - self.vertices[1]; 
        let c1 = edge1.cross(intersection - self.vertices[1]);
        if normal.dot(c1) < 0.0 { return false; }

        // Edge 2
        let edge2 = self.vertices[0] - self.vertices[2]; 
        let c2 = edge2.cross(intersection - self.vertices[2]);
        if normal.dot(c2) < 0.0 { return false; }

        return true;
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: Ray, render_bounds: &Interval) -> Option<HitRecord> {
        let edge1 = self.vertices[1] - self.vertices[0];
        let edge2 = self.vertices[2] - self.vertices[0];

        let normal = edge1.cross(edge2).normalize();

        if normal.dot(ray.direction).abs() < 0.001 {
            // Triangle and plane are parallel
            return None;
        }

        let d = -normal.dot(self.vertices[0]);
        let distance = -(normal.dot(ray.origin) + d) / normal.dot(ray.direction);
        
        if render_bounds.is_outside(distance) {
            return None;
        }

        let intersection = ray.at(distance);
        if !self.is_inside(intersection, normal) {
            return None;
        }

        Some(HitRecord {
            hit: intersection,
            distance: distance,
            normal: normal
        })
    }
}