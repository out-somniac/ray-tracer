mod color;
use color::Color;

mod ray;
use ray::Ray;

mod sphere;
use sphere::Sphere;

use cgmath::Vector3;
use cgmath::InnerSpace;

mod hits;
#[allow(unused_imports)]
use crate::hits::{Hittable, HitRecord};

use image::{ImageBuffer, RgbImage};

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
    let objects = vec!(
        Sphere {
            origin: Vector3::new(-0.5, 0.0, -1.0),
            radius: 0.5
        },
        Sphere {
            origin: Vector3::new(0.0, 0.0, -1.5),
            radius: 0.5
        }
    );

    let t_max = 1000.0;
    let t_min = 0.001;

    let hits = objects
        .into_iter()
        .map(|obj| obj.hit(ray, t_min, t_max))
        .filter(|hit| hit.is_some())
        .map(|hit| hit.unwrap());
    

    let closest_hit = hits
        .min_by(|a, b| a.partial_cmp(&b).expect("Failed during HitRecord partial comparison"));

    return match closest_hit {
        Some(record) => {
            let value: Vector3<f64> = 0.5 * (record.normal + Vector3::new(1.0, 1.0, 1.0));
            // TODO: Move to some `from_vector` function in `Color`.
            return Color { 
                red:   (255.999 * value.x) as u8,
                green: (255.999 * value.y) as u8,
                blue:  (255.999 * value.z) as u8
            };
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
            let color = ray_color(ray);
            let pixel = image.get_pixel_mut(i, j);
            *pixel = image::Rgb([color.red, color.green, color.blue]);
        }
    }
    image.save("test.png").unwrap();
}
