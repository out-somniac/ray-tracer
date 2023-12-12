mod shapes;
mod ray;
mod hits;
mod interval;
mod camera;
mod material;
mod vector_utils;

use camera::Camera;
use cgmath::Vector3;
use material::{Metal, Lambertian, Material};
use crate::hits::Hittable;
use crate::shapes::{Sphere, Triangle};
use std::rc::Rc;

fn create_cube(origin: Vector3<f64>, size: f64, material: Rc<dyn Material>) -> Vec<Box<dyn Hittable>> {
    let dx = Vector3::new(size / 2.0, 0.0, 0.0);
    let dy = Vector3::new(0.0, size / 2.0, 0.0);
    let dz = Vector3::new(0.0, 0.0, size / 2.0);
    
    return vec!(
        // Front Face
        Box::new(Triangle {
            vertices: [
                origin - dx + dy + dz,
                origin - dx - dy + dz,
                origin + dx - dy + dz
            ],
            material: material.clone()
        }),
        Box::new(Triangle {
            vertices: [
                origin + dx - dy + dz,
                origin + dx + dy + dz,
                origin - dx + dy + dz
            ],
            material: material.clone()
        }),

        // Back Face
        Box::new(Triangle {vertices: [
                origin - dx + dy - dz,
                origin - dx - dy - dz,
                origin + dx - dy - dz
            ],
            material: material.clone()
        }),
        Box::new(Triangle {vertices: [
                origin + dx - dy - dz,
                origin + dx + dy - dz,
                origin - dx + dy - dz
            ],
            material: material.clone()
        }),

        // Right Face
        Box::new(Triangle {vertices: [
                origin + dx + dy + dz,
                origin + dx - dy + dz,
                origin + dx - dy - dz
            ],
            material: material.clone()
        }),
        Box::new(Triangle {vertices: [
                origin + dx + dy + dz,
                origin + dx - dy - dz,
                origin + dx + dy - dz
            ],
            material: material.clone()
        }),

        // Left Face
        Box::new(Triangle {vertices: [
                origin - dx + dy + dz,
                origin - dx - dy - dz,
                origin - dx - dy + dz
            ],
            material: material.clone()
        }),
        Box::new(Triangle {vertices: [
                origin - dx + dy + dz,
                origin - dx + dy - dz,
                origin - dx - dy - dz
            ],
            material: material.clone()
        }),

        // Bottom Face
        Box::new(Triangle {vertices: [
                origin - dx - dy - dz,
                origin + dx - dy + dz,
                origin - dx - dy + dz
            ],
            material: material.clone()
        }),
        Box::new(Triangle {vertices: [
                origin - dx - dy - dz,
                origin + dx - dy - dz,
                origin + dx - dy + dz
            ],
            material: material.clone()
        }),

        // Top Face
        Box::new(Triangle {vertices: [
                origin - dx + dy - dz,
                origin - dx + dy + dz,
                origin + dx + dy + dz
            ],
            material: material.clone()
        }),
        Box::new(Triangle {vertices: [
                origin - dx + dy - dz,
                origin + dx + dy + dz,
                origin + dx + dy - dz
            ],
            material: material.clone()
        }),
    );
}

fn main() {
    let camera = Camera::new(
        16.0 / 9.0,
        800,
        Vector3::new(0.0, 0.0, 0.0),
        1.0,
        50,
        50
    );

    let mut objects: Vec<Box<dyn Hittable>> = vec!();

    // Cubes
    objects.extend(create_cube(
        Vector3::new(0.25, 0.25, -0.5), 
        0.25,
        Rc::new(Lambertian {
            albedo: Vector3::new(0.76, 0.07, 0.57)
        })
    ));
    objects.extend(create_cube(
        Vector3::new(-0.3, -0.2, -0.3),
        0.2,
        Rc::new(Lambertian {
            albedo: Vector3::new(0.91, 0.74, 0.87)
        })
    ));

    // Sphere
    objects.push(Box::new(Sphere {
        origin: Vector3::new(0.05, -0.2, -0.5),
        radius: 0.25,
        material: Rc::new(Metal {
            albedo: Vector3::new(0.94, 0.25, 0.25)
        })
    }));

    camera.render(&objects).save("test.png").unwrap();
}
