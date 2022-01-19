use crate::shape::Distance;


#[derive(Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}


impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point {x, y, z}
    }

    pub fn add(&self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn sub(&self, other: &Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn neg(&self,) -> Point {
        Point::new(-self.x, -self.y, -self.z)
    }

    pub fn mul(&self, k: f64) -> Point {
        Point::new(self.x * k, self.y * k, self.z * k)
    }

    pub fn inner_product(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross_product(&self, other: &Point) -> Point {
        Point::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn get_norm_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn get_norm(&self) -> f64 {
        self.get_norm_sqr().sqrt()
    }

    pub fn normalize(&self) -> Point {
        self.mul(1.0 / self.get_norm())
    }

    pub fn is_zero(&self) -> bool {
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }

}


impl Distance for Point {
    fn get_dist(&self, other: &Point) -> f64 {
        self.sub(other).get_norm()
    }
}