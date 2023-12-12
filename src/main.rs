mod shapes;
mod ray;
mod hits;
mod interval;
mod camera;

use camera::Camera;
use cgmath::Vector3;
use crate::hits::Hittable;
use crate::shapes::Sphere;

fn main() {
    let camera = Camera::new(
        16.0 / 9.0,
        800,
        Vector3::new(0.0, 0.0, 0.0),
        1.0,
        25,
        10
    );

    let objects: Vec<Box<dyn Hittable>> = vec!(
        Box::new(Sphere {
            origin: Vector3::new(-0.5, 0.0, -1.0),
            radius: 0.5
        }),
        Box::new(Sphere {
            origin: Vector3::new(0.5, 0.0, -1.0),
            radius: 0.5
        }),
        Box::new(Sphere {
            origin: Vector3::new(0.0, -100.5, -1.0),
            radius: 100.0
        })
    );

    camera.render(&objects).save("test.png").unwrap();
}
