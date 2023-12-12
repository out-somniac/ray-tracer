use cgmath::{Vector3, InnerSpace};
use crate::ray::Ray;
use crate::hits::Hittable;
use crate::interval::Interval;
use image::{ImageBuffer, RgbImage};


fn rand_normal() -> Vector3<f64> {
    use rand::distributions::{Distribution, Uniform}; 
    let mut rng = rand::thread_rng();
    let distribution = Uniform::new(-1.0, 1.0);
    loop {
        let random = Vector3::new(
            distribution.sample(&mut rng),
            distribution.sample(&mut rng),
            distribution.sample(&mut rng));

        if random.dot(random) < 1.0 {
            return random.normalize();
        }
    }
}

fn rand_on_hemisphere(normal: Vector3<f64>) -> Vector3<f64> {
    let random = rand_normal();
    return if random.dot(normal) > 0.0 { random } else { -random }
}

fn color_from_vector(color: Vector3<f64>) -> image::Rgb<u8> {
    image::Rgb([
        (255.999 * color.x) as u8,
        (255.999 * color.y) as u8,
        (255.999 * color.z) as u8
    ])
}

fn sky_color(ray: Ray) -> Vector3<f64> {
    let normalized = ray.direction.normalize();
    let alpha = 0.5 * (normalized.y + 1.0);
    return (1.0 - alpha)* Vector3::new(1.0, 1.0, 1.0) + alpha * Vector3::new(0.5, 0.7, 1.0)
}

fn ray_color(ray: Ray, objects: &Vec<Box<dyn Hittable>>, max_depth: u32) -> Vector3<f64> {
    if max_depth <= 0 {
        return Vector3::new(255.0, 255.0, 255.0);
    } 
    
    return match objects.hit(ray, &Interval::new(0.0, f64::INFINITY)) {
        Some(record) => {
            let direction = rand_on_hemisphere(record.normal);
            return 0.5 * ray_color(Ray::new(record.hit, direction), objects, max_depth - 1);
        },
        None => sky_color(ray)
    }
}  

pub struct Camera {
    width: u32,
    height: u32,
    position: Vector3<f64>,

    pixel_upper_left: Vector3<f64>,
    pixel_delta_u: Vector3<f64>,
    pixel_delta_v: Vector3<f64>
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, position: Vector3<f64>, focal_length: f64) -> Camera {
        let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
        
        // Viewport setup
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Compute deltas
        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
        let delta_u = viewport_u / image_width as f64;
        let delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = position 
                                - Vector3::new(0.0, 0.0, focal_length)
                                - viewport_u / 2.0
                                - viewport_v / 2.0;
        let pixel_upper_left = viewport_upper_left + 0.5 * (delta_u + delta_v);

        Camera {
            width: image_width,
            height: image_height,
            position: position,
            pixel_upper_left: pixel_upper_left,
            pixel_delta_u: delta_u,
            pixel_delta_v: delta_v
        }
    }


    pub fn render(&self, objects: &Vec<Box<dyn Hittable>>) -> RgbImage {
        let mut image: RgbImage = ImageBuffer::new(self.width, self.height);
        for i in 0..self.width {
            for j in 0..self.height {
                let pixel = self.pixel_upper_left + (i as f64) * self.pixel_delta_u + (j as f64) * self.pixel_delta_v;
                let ray_direction = pixel - self.position;
                let ray = Ray::new(self.position, ray_direction.clone());
                let pixel = image.get_pixel_mut(i, j);
                let color = ray_color(ray, objects, 50);
                *pixel = color_from_vector(color);
            }
        }
        return image;
    }
    
}