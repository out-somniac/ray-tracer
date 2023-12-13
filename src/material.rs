use cgmath::{Vector3, InnerSpace};
use crate::{ray::Ray, hits::HitRecord};
use crate::vector_utils::{rand_normal, reflect};

pub trait Material {
    fn scatter(&self, ray: Ray, record: &HitRecord) -> Option<(Vector3<f64>, Ray)>;
}

pub struct Metal {
    pub albedo: Vector3<f64>,
    pub fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vector3<f64>, fuzz: f64) -> Metal {
        Metal {
            albedo: albedo,
            fuzz: fuzz.clamp(0.0, fuzz)
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, record: &HitRecord) -> Option<(Vector3<f64>, Ray)> {
        let reflected = reflect(ray.direction.normalize(), record.normal);
        let scattered = Ray::new(record.hit, reflected + self.fuzz * rand_normal());
        let attenuation = self.albedo;
        if scattered.direction.dot(record.normal) <= 0.0 {
            return None;
        }

        return Some((attenuation, scattered));
    }
}

pub struct Lambertian {
    pub albedo: Vector3<f64>
}

impl Material for Lambertian {
    fn scatter(&self, ray: Ray, record: &HitRecord) -> Option<(Vector3<f64>, Ray)> {
        let scattered_direction = record.normal + rand_normal();
        let scattered = Ray::new(record.hit, scattered_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    } 
}
