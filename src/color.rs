pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

impl Color {
    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.red, self.green, self.blue)
    }
}