use cgmath::Vector3;

pub struct Ray<T> {
    pub origin: Vector3<T>,
    pub target: Vector3<T>   
}