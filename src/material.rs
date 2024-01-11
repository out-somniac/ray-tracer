use cgmath::{Vector3, InnerSpace};
use crate::{ray::Ray, hits::HitRecord};
use crate::vector_utils::{rand_normal, reflect};
use rand::Rng;

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
    fn scatter(&self, _ray: Ray, record: &HitRecord) -> Option<(Vector3<f64>, Ray)> {
        let scattered_direction = record.normal + rand_normal();
        let scattered = Ray::new(record.hit, scattered_direction);
        let attenuation = self.albedo;
        Some((attenuation, scattered))
    } 
}

pub struct Dielectric {
    pub refractive_index: f64
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Dielectric {
        Dielectric {
            refractive_index: refractive_index
        }
    }
}

// TODO: Move somewhere else
fn reflectance(cos_theta: f64, refractive_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - refractive_index) / (1.0 + refractive_index).powi(2);
    return r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5);
}

fn refract(uv: Vector3<f64>, normal: Vector3<f64>, refractive_index: f64) -> Vector3<f64> {
    let cos_theta = f64::min(normal.dot(-uv), 1.0);
    let r_out_perp =  refractive_index * (uv + cos_theta * normal);
    let r_out_parallel = -((1.0 - r_out_perp.dot(r_out_perp)).abs()).sqrt() * normal;
    return r_out_perp + r_out_parallel;
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, record: &HitRecord) -> Option<(Vector3<f64>, Ray)> {
        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if ray.direction.dot(record.normal) < 0.0 {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_direction = ray.direction.normalize();
        let cos_theta = f64::min(record.normal.dot(-unit_direction), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let no_refraction = refraction_ratio * sin_theta > 1.0;
        let mut rng = rand::thread_rng();
        let direction = if no_refraction || reflectance(cos_theta, refraction_ratio) > rng.gen_range(0.0..1.0) {
            reflect(unit_direction, record.normal)
        } else {
            refract(unit_direction, record.normal, refraction_ratio)
        };

        let scattered = Ray::new(record.hit, direction);
        return Some((attenuation, scattered));
    } 
}