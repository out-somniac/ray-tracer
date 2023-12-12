use cgmath::Vector3;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>   
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        Ray {
            origin: origin,
            direction: direction
        }
    }

    pub fn at(&self, t: f64) -> Vector3<f64> {
        return self.origin + t * self.direction;
    }
}