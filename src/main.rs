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
use std::path::Path;
use tobj::{Mesh, LoadOptions};

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

// A simple solution, that's not bad for now but will be in the future
fn load_single_obj(filepath: &Path) -> Mesh {
    let config = LoadOptions {
        #[cfg(feature = "merging")]
        merge_identical_points: false,
        #[cfg(feature = "reordering")]
        reorder_data: false,
        single_index: false,
        triangulate: true,
        ignore_points: true,
        ignore_lines: true,
    };
    let file = tobj::load_obj(filepath, &config);
    assert!(file.is_ok());
    let (models, _) = file.expect("Failed to load OBJ file");
    return models[0].mesh.clone();
}

fn mesh_to_triangle_vec(mesh: Mesh, material: Rc<dyn Material>) -> Vec<Box<dyn Hittable>> {
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

        let offset = Vector3::new(-0.5, 0.0, -2.0);
        result.push(Box::new(Triangle {
            vertices: [offset + v0, offset + v1, offset + v2],
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
        40,
        30
    );

    let mut objects: Vec<Box<dyn Hittable>> = vec!();

    // Cubes
    // objects.extend(create_cube(
    //     Vector3::new(0.25, 0.25, -0.5), 
    //     0.25,
    //     Rc::new(Lambertian {
    //         albedo: Vector3::new(0.76, 0.07, 0.57)
    //     })
    // ));
    // objects.extend(create_cube(
    //     Vector3::new(-0.3, -0.2, -0.3),
    //     0.2,
    //     Rc::new(Lambertian {
    //         albedo: Vector3::new(0.91, 0.74, 0.87)
    //     })
    // ));

    // Spheres
    // objects.push(Box::new(Sphere {
    //     origin: Vector3::new(0.05, -0.2, -0.5),
    //     radius: 0.25,
    //     material: Rc::new(Metal::new(
    //         Vector3::new(0.94, 0.25, 0.25),
    //         0.2
    //     ))
    // }));
    
    objects.push(Box::new(Sphere {
        origin: Vector3::new(0.5, 0.0, -1.0),
        radius: 0.25,
        material: Rc::new(Metal::new(
            Vector3::new(0.94, 0.25, 0.25),
            0.0
        ))
    }));
    let filepath = Path::new("resources/models/suzanne.obj");
    let mesh = load_single_obj(filepath);
    objects.extend(mesh_to_triangle_vec(mesh, Rc::new(Metal::new(
        Vector3::new(0.91, 0.74, 0.87),
        0.1
    ))));
    camera.render(&objects).save("test.png").unwrap();
}
