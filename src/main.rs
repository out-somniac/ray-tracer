mod shapes;
mod ray;
mod hits;
mod interval;

use ray::Ray;
use shapes::Sphere;
use cgmath::{Vector3, InnerSpace};
use hits::{Hittable};
use image::{ImageBuffer, RgbImage};

// TODO: This should be done better
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

fn ray_color(ray: Ray, max_depth: u32) -> Vector3<f64> {
    if max_depth <= 0 {
        return Vector3::new(255.0, 255.0, 255.0);
    } 
    
    let objects: Vec<Box<dyn Hittable>> = vec!(
        Box::new(Sphere {
            origin: Vector3::new(0.0, 0.0, -1.0),
            radius: 0.5
        }),
        Box::new(Sphere {
            origin: Vector3::new(0.0, -100.5, -1.0),
            radius: 100.0
        })
    );

    let t_max = 100000000.0;
    let t_min = 0.001;

    return match objects.hit(ray, t_min, t_max) {
        Some(record) => {
            let direction = rand_on_hemisphere(record.normal);
            return 0.5 * ray_color(Ray::new(record.hit, direction), max_depth - 1);
        },
        None => sky_color(ray)
    }
}  
 
fn main() {

    //Image setup
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 800;
    let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
    let mut image: RgbImage = ImageBuffer::new(image_width, image_height);

    // Camera parameters
    let focal_length = 1.0;
    let camera_center = Vector3::new(0.0, 0.0, 0.0);

    // Viewport setup
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    // Compute deltas
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
    let delta_u = viewport_u / image_width as f64;
    let delta_v = viewport_v / image_height as f64;
    let viewport_upper_left = camera_center 
                              - Vector3::new(0.0, 0.0, focal_length)
                              - viewport_u / 2.0
                              - viewport_v / 2.0;
    let pixel_upper_left = viewport_upper_left + 0.5 * (delta_u + delta_v);

    for i in 0..image_width {
        for j in 0..image_height {
            let pixel = pixel_upper_left + (i as f64) * delta_u + (j as f64) * delta_v;
            let ray_direction = pixel - camera_center;
            let ray = Ray::new(camera_center, ray_direction.clone());
            let pixel = image.get_pixel_mut(i, j);
            let color = ray_color(ray, 50);
            *pixel = color_from_vector(color);
        }
    }
    image.save("test.png").unwrap();
}
