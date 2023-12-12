use cgmath::Vector3;
use crate::{ray::Ray, hits::HitRecord};
use crate::vector_utils::rand_normal;

pub trait Material {
    fn scatter(&self, ray: Ray, record: &HitRecord) -> Option<(Vector3<f64>, Ray)>;
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