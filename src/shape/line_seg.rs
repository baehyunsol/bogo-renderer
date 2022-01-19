use crate::point::Point;
use super::line::Line;
use super::{Distance, NormVec};


#[derive(Clone)]
pub struct LineSeg {
    p1: Point,
    p2: Point,
    length_sqr: f64,
    dir: Point  // p2 - p1
}


impl LineSeg {
    pub fn new(p1: Point, p2: Point) -> LineSeg {
        let dir = p2.sub(&p1);

        LineSeg{length_sqr: dir.get_norm_sqr(), dir, p1, p2}
    }

    pub fn to_line(&self) -> Line {
        Line::from_points(self.p1.clone(), self.p2.clone())
    }
}


impl Distance for LineSeg {

    fn get_dist(&self, to: &Point) -> f64 {

        if self.length_sqr == 0.0 {
            self.p1.get_dist(to)
        }

        else {
            let t = (to.sub(&self.p1).inner_product(&self.dir) / self.length_sqr).min(1.0).max(0.0);
            let proj = self.p1.add(&self.dir.mul(t));

            proj.get_dist(to)
        }

    }

}


impl NormVec for LineSeg {

    fn get_normal_vector_at(&self, at: &Point) -> Point {

        if self.length_sqr == 0.0 {
            at.sub(&self.p1)
        }

        else {
            let t = at.sub(&self.p1).inner_product(&self.dir) / self.length_sqr;

            if t < 0.0 {
                at.sub(&self.p1)
            }

            else if t > 1.0 {
                at.sub(&self.p2)
            }

            else {
                at.sub(&self.p1.add(&self.dir.mul(t)))
            }

        }

    }

}
