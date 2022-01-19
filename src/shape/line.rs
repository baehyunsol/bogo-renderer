use crate::point::Point;
use super::{Distance, NormVec};


#[derive(Clone)]
pub struct Line {
    direction: Point,  // must be a unit vector!
    reference: Point
}


impl Line {

    pub fn from_points(p1: Point, p2: Point) -> Line {

        let direction = p1.sub(&p2).normalize();

        Line {
            direction,
            reference: p1
        }
    }

    pub fn from_direction(direction: Point, reference: Point) -> Line {
        Line {
            direction: direction.normalize(),
            reference
        }
    }

    pub fn get_perpendicular_foot(&self, at: &Point) -> Point {

        self.reference.add(&self.direction.mul(at.sub(&self.reference).inner_product(&self.direction)))
    }

    pub fn get_dist_between_lines(&self, other: &Line) -> f64 {
        let mut n = self.direction.cross_product(&other.direction);

        if n.is_zero() {
            self.get_dist(&other.reference)
        }

        else {
            n = n.normalize();

            n.inner_product(&self.reference.sub(&other.reference))
        }

    }

}


impl Distance for Line {

    fn get_dist(&self, to: &Point) -> f64 {

        let ref1 = &self.reference;
        let ref2 = &self.reference.add(&self.direction);

        to.sub(&ref1).cross_product(&to.sub(&ref2)).get_norm() / ref1.sub(&ref2).get_norm()
    }

}


impl NormVec for Line {

    fn get_normal_vector_at(&self, at: &Point) -> Point {
        at.sub(&self.get_perpendicular_foot(at)).normalize()
    }

}