pub struct Interval<T> {
    pub min: T,
    pub max: T
}

impl<T: std::cmp::PartialOrd> Interval<T> {
    pub fn new(min: T, max: T) -> Interval<T> {
        Interval { min: min, max: max }
    }

    pub fn is_inside(&self, value: T) -> bool {
        self.min <= value && value <= self.max 
    }

    pub fn is_outside(&self, value: T) -> bool {
        !self.is_inside(value)
    }
}