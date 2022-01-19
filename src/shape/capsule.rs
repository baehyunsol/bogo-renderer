use crate::point::Point;
use super::line_seg::LineSeg;
use super::GoodToRaycast;
use super::cylinder::Cylinder;


#[derive(Clone)]
pub struct Capsule {
    pub axis: LineSeg,
    pub radius: f64
}


impl GoodToRaycast for Capsule {
    fn is_good_to_raycast(&self, from: &Point, to: &Point) -> bool {
        (Cylinder {axis: self.axis.to_line(), radius: self.radius}).is_good_to_raycast(from, to)
    }
}