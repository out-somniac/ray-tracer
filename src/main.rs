mod color;
use color::Color;

mod ray;
use ray::Ray;

use cgmath::Vector3;

fn main() {
    let width: u32 = 800;
    let height: u32 = 800;

    let _example_ray = Ray {
        origin: Vector3::new(0.0, 0.0, 0.0),
        target: Vector3::new(1.0, 1.0, 1.0)
    };

    print!("{}", format!("P3\n{} {}\n255\n", width, height));
    for i in 0..width {
        for j in 0..height {
            let color = Color {
                red: (255.999 * i as f32 / (width - 1) as f32) as u8,
                green: (255.999 * j as f32 / (height - 1) as f32) as u8,
                blue: 0
            };
            println!("{}", color.to_string());
        }
    }
}
