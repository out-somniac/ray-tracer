mod shapes;
mod ray;
mod hits;
mod interval;
mod camera;

use camera::Camera;
use cgmath::Vector3;
use image::RgbImage;

fn main() {
    let camera = Camera::new(
        16.0 / 9.0,
        800,
        Vector3::new(0.0, 0.0, 0.0),
        1.0
    );

    camera.render().save("test.png").unwrap();
}
