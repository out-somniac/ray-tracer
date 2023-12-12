use rand::Rng;
use cgmath::{Vector3, InnerSpace};

pub fn rand_normal() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    loop {
        let random = Vector3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0));

        if random.dot(random) < 1.0 {
            return random.normalize();
        }
    }
}

#[allow(dead_code)]
pub fn rand_on_hemisphere(normal: Vector3<f64>) -> Vector3<f64> {
    let random = rand_normal();
    return if random.dot(normal) > 0.0 { random } else { -random }
}

pub fn color_from_vector(color: Vector3<f64>) -> image::Rgb<u8> {
    image::Rgb([
        (255.999 * color.x) as u8,
        (255.999 * color.y) as u8,
        (255.999 * color.z) as u8
    ])
}

pub fn channel_multiply(u: Vector3<f64>, v: Vector3<f64>) -> Vector3<f64> {
    Vector3::new(u.x * v.x, u.y * v.y, u.z * v.z)
}