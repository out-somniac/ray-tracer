mod shapes;
mod ray;
mod hits;
mod interval;
mod camera;

use camera::Camera;
use cgmath::Vector3;
use shapes::Triangle;
use crate::hits::Hittable;
use crate::shapes::Sphere;

fn create_cube(origin: Vector3<f64>, size: f64) -> Vec<Box<dyn Hittable>> {
    let dx = Vector3::new(size / 2.0, 0.0, 0.0);
    let dy = Vector3::new(0.0, size / 2.0, 0.0);
    let dz = Vector3::new(0.0, 0.0, size / 2.0);
    
    return vec!(
        // Front Face
        Box::new(Triangle {vertices: [
                origin - dx + dy + dz,
                origin - dx - dy + dz,
                origin + dx - dy + dz]}),
        Box::new(Triangle {vertices: [
                origin + dx - dy + dz,
                origin + dx + dy + dz,
                origin - dx + dy + dz]}),

        // Back Face
        Box::new(Triangle {vertices: [
            origin - dx + dy - dz,
            origin - dx - dy - dz,
            origin + dx - dy - dz]}),
        Box::new(Triangle {vertices: [
            origin + dx - dy - dz,
            origin + dx + dy - dz,
            origin - dx + dy - dz]}),

        // Right Face
        Box::new(Triangle {vertices: [
                origin + dx + dy + dz,
                origin + dx - dy + dz,
                origin + dx - dy - dz]}),
        Box::new(Triangle {vertices: [
                origin + dx + dy + dz,
                origin + dx - dy - dz,
                origin + dx + dy - dz]}),

        // Left Face
        Box::new(Triangle {vertices: [
            origin - dx + dy + dz,
            origin - dx - dy + dz,
            origin - dx - dy - dz]}),
        Box::new(Triangle {vertices: [
            origin - dx + dy + dz,
            origin - dx - dy - dz,
            origin - dx + dy - dz]}),

        // Bottom Face
        Box::new(Triangle {vertices: [
            origin - dx - dy - dz,
            origin - dx - dy + dz,
            origin + dx - dy + dz]}),
        Box::new(Triangle {vertices: [
            origin - dx - dy - dz,
            origin + dx - dy + dz,
            origin + dx - dy - dz]}),

        // Top Face
        Box::new(Triangle {vertices: [
            origin - dx + dy - dz,
            origin - dx + dy + dz,
            origin + dx + dy + dz]}),
        Box::new(Triangle {vertices: [
            origin - dx + dy - dz,
            origin + dx + dy + dz,
            origin + dx + dy - dz]}),
    );
}

fn main() {
    let camera = Camera::new(
        16.0 / 9.0,
        800,
        Vector3::new(0.0, 0.0, 0.0),
        1.0,
        25,
        10
    );

    let mut objects: Vec<Box<dyn Hittable>> = vec!();
    objects.extend(create_cube(Vector3::new(0.25, 0.25, -0.5), 0.25));
    objects.extend(create_cube(Vector3::new(-0.3, -0.1, -0.3), 0.2));
    objects.push(Box::new(Sphere {
        origin: Vector3::new(0.2, -0.2, -0.5),
        radius: 0.25
    }));


    camera.render(&objects).save("test.png").unwrap();
}
