mod color;
use color::Color;

mod ray;
use ray::Ray;

use cgmath::Vector3;
use cgmath::InnerSpace;

use image::{ImageBuffer, RgbImage};

fn sphere_hit(center: Vector3<f64>, radius: f64, ray: Ray) -> Option<f64> {
    let oc = ray.origin - center;
    let a: f64 = ray.direction.dot(ray.direction);
    let b: f64 = 2.0 * oc.dot(ray.direction);
    let c: f64 = oc.dot(oc) - radius * radius;
    let discriminant = b*b - 4.0*a*c;
    if discriminant < 0.0 {
        return None;
    } else {
        return Some(( - b - discriminant.sqrt() ) / ( 2.0 * a ));
    }
}

fn sky_color(ray: Ray) -> Color {
    let normalized = ray.direction.normalize();
    let alpha = 0.5 * (normalized.y + 1.0);
    let value = (1.0 - alpha)* Vector3::new(1.0, 1.0, 1.0) + alpha * Vector3::new(0.5, 0.7, 1.0);

    return Color {
        red:    (255.999 * value.x) as u8,
        green:  (255.999 * value.y) as u8,
        blue:   (255.999 * value.z) as u8
    };
}

fn ray_color(ray: Ray) -> Color {
    let sphere_center: Vector3<f64> = Vector3::new(0.0, 0.0, -1.0);

    return match sphere_hit(sphere_center, 0.5, ray) {
        Some(distance) => {
            let normal: Vector3<f64> = (ray.at(distance) - sphere_center).normalize();
            let value: Vector3<f64> = 0.5 * (normal + Vector3::new(1.0, 1.0, 1.0));
            return Color { 
                red:   (255.999 * value.x) as u8,
                green: (255.999 * value.y) as u8,
                blue:  (255.999 * value.z) as u8
            };
        }
        None => sky_color(ray)
    };
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
            let color = ray_color(ray);
            let pixel = image.get_pixel_mut(i, j);
            *pixel = image::Rgb([color.red, color.green, color.blue]);
        }
    }
    image.save("test.png").unwrap();
}
