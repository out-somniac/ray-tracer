use cgmath::Vector3;

#[derive(Clone, Copy)]
pub struct Ray<T> {
    pub origin: Vector3<T>,
    pub direction: Vector3<T>   
}

impl<T> Ray<T> {
    pub const fn new(origin: Vector3<T>, direction: Vector3<T>) -> Ray<T> {
        Ray {
            origin: origin,
            direction: direction
        }
    }
}