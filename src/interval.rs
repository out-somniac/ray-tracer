pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min: min, max: max }
    }

    pub fn is_inside(&self, value: f64) -> bool {
        self.min <= value && value <= self.max 
    }

    pub fn is_outside(&self, value: f64) -> bool {
        !self.is_inside(value)
    }

    pub fn clamp(&self, value: f64) -> f64 {
        if value < self.min { return self.min; }
        if value > self.max { return self.max; }
        return value;
    }
}