use crate::point::Point;
use super::{Distance, NormVec, GoodToRaycast};


#[derive(Clone)]
pub struct Plane {
    // ax + by + cz + d = 0
    // a^2 + b^2 + c^2 = 1
    pub a: f64,
    pub b: f64,
    pub c: f64,
    pub d: f64,

    pub norm_vec: Point  // must be unit vector
}

impl Plane {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Plane {
        let size = (a * a + b * b + c * c).sqrt();

        if size == 0.0 {
            panic!("wrong input!!");
        }

        let (a, b, c, d) = (a / size, b / size, c / size, d / size);

        Plane {
            a, b, c, d,
            norm_vec: Point::new(a, b, c)
        }
    }

    pub fn get_perpendicular_foot(&self, at: &Point) -> Point {

        let k = -(self.norm_vec.inner_product(at) + self.d);
        self.norm_vec.mul(k).add(at)
    }

}

impl NormVec for Plane {
    fn get_normal_vector_at(&self, _: &Point) -> Point {
        self.norm_vec.clone()
    }
}

impl Distance for Plane {
    fn get_dist(&self, to: &Point) -> f64 {
        (self.norm_vec.inner_product(to) + self.d).abs()
        //(to.x * self.a + to.y * self.b + to.z * self.c + self.d).abs()
    }
}

impl GoodToRaycast for Plane {
    fn is_good_to_raycast(&self, _: &Point, _: &Point) -> bool {
        true
    }
}
