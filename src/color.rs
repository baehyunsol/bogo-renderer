use crate::point::Point;


#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}


impl Color {

    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color {r, g, b}
    }

    pub fn mul(&self, k: f64) -> Color {
        Color::new(
            (self.r as f64 * k).min(255.0) as u8,
            (self.g as f64 * k).min(255.0) as u8,
            (self.b as f64 * k).min(255.0) as u8,
        )
    }

    pub fn add(&self, other: &Color) -> Color {
        Color::new(
            (self.r as usize + other.r as usize).min(255) as u8,
            (self.g as usize + other.g as usize).min(255) as u8,
            (self.b as usize + other.b as usize).min(255) as u8,
        )
    }

    pub fn as_point(&self) -> Point {
        Point::new(self.r as f64, self.g as f64, self.b as f64)
    }

    pub fn from_point(p: &Point) -> Color {
        Color::new(p.x.min(255.0).max(0.0) as u8, p.y.min(255.0).max(0.0) as u8, p.z.min(255.0).max(0.0) as u8)
    }

    pub fn sum(&self) -> f64 {
        self.r as f64 + self.g as f64 + self.b as f64
    }

    pub fn white() -> Color {
        Color::new(255, 255, 255)
    }

    pub fn into_8bit(&self) -> Color {
        Color::new(
            self.r / 32 * 32,
            self.g / 32 * 32,
            self.b / 64 * 64,
        )
    }

}
