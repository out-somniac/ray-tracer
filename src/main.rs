mod shapes;
mod ray;
mod hits;
mod interval;
mod camera;
mod material;
mod vector_utils;

use camera::Camera;
use cgmath::Vector3;
use material::{Metal, Material, Lambertian, Dielectric};
use crate::hits::Hittable;
use crate::shapes::{Sphere, Triangle};
use std::rc::Rc;
use std::path::Path;
use tobj::{Mesh, LoadOptions};

// A simple solution, that's not bad for now but will be in the future
fn load_single_obj(filepath: &Path) -> Mesh {
    let config = LoadOptions {
        single_index: false,
        triangulate: true,
        ..tobj::GPU_LOAD_OPTIONS
    };
    let file = tobj::load_obj(filepath, &config);
    assert!(file.is_ok());
    let (models, _) = file.expect("Failed to load OBJ file");
    return models[0].mesh.clone();
}

fn mesh_to_triangle_vec(mesh: Mesh, material: Rc<dyn Material>, position: Vector3<f64>) -> Vec<Box<dyn Hittable>> {
    assert!(mesh.indices.len() % 3 == 0);
    let mut result: Vec<Box<dyn Hittable>> = vec!();

    for k in 0..mesh.indices.len() / 3 {
        let v0 = Vector3::new(
            mesh.positions[mesh.indices[3 * k] as usize * 3] as f64,
            mesh.positions[mesh.indices[3 * k] as usize * 3 + 1] as f64,
            mesh.positions[mesh.indices[3 * k] as usize * 3 + 2] as f64
        );

        let v1 = Vector3::new(
            mesh.positions[mesh.indices[3 * k + 1] as usize * 3] as f64,
            mesh.positions[mesh.indices[3 * k + 1] as usize * 3 + 1] as f64,
            mesh.positions[mesh.indices[3 * k + 1] as usize * 3 + 2] as f64
        );

        let v2 = Vector3::new(
            mesh.positions[mesh.indices[3 * k + 2] as usize * 3] as f64,
            mesh.positions[mesh.indices[3 * k + 2] as usize * 3 + 1] as f64,
            mesh.positions[mesh.indices[3 * k + 2] as usize * 3 + 2] as f64
        );

        result.push(Box::new(Triangle {
            vertices: [position + v0, position + v1, position + v2],
            material: material.clone()
        }));
    }
    return result;
}

fn main() {
    let camera = Camera::new(
        16.0 / 9.0,
        800,
        Vector3::new(0.0, 0.0, 0.0),
        1.0,
        80,
        80
    );

    let mut objects: Vec<Box<dyn Hittable>> = vec!();
    
    objects.push(Box::new(Sphere {
        origin: Vector3::new(1.2, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Metal::new(
            Vector3::new(0.94, 0.25, 0.25),
            0.0
        ))
    }));

    objects.push(Box::new(Sphere {
        origin: Vector3::new(0.0, 0.0, -1.0),
        radius: -0.5,
        material: Rc::new(Dielectric::new(
            Vector3::new(0.94, 0.25, 0.25),
            1.5
        ))
    }));
    
    objects.push(Box::new(Sphere {
        origin: Vector3::new(-1.2, 0.0, -1.0),
        radius: 0.5,
        material: Rc::new(Metal::new(
            Vector3::new(0.23, 0.94, 0.25),
            0.5
        ))
    }));

    objects.push(Box::new(Sphere {
        origin: Vector3::new(0.0, -100.0, 0.0),
        radius: 99.0,
        material: Rc::new(Lambertian {
            albedo: Vector3::new(0.2, 0.2, 0.2),
        })
    }));

    // let filepath = Path::new("resources/models/suzanne.obj");
    // let mesh = load_single_obj(filepath);
    // objects.extend(mesh_to_triangle_vec(mesh, Rc::new(Metal::new(
    //     Vector3::new(0.91, 0.74, 0.87),
    //     0.1,
    // )), Vector3::new(0.0, 0.0, -2.0)));



    camera
        .render(&objects)
        .save("output/test.png")
        .expect("Failed to save to file");
}
