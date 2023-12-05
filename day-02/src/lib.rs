#[derive(Debug)]
pub struct Colors {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}
impl Colors {
    pub fn reset(&self) -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    pub fn check_overflow(&self) -> bool {
        self.r > 12 || self.g > 13 || self.b > 14
    }

    pub fn product(&self) -> u32 {
        self.r * self.g * self.b
    }
}
