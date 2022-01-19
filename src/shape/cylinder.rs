use super::line::Line;
use crate::point::Point;
use super::GoodToRaycast;


#[derive(Clone)]
pub struct Cylinder {
    pub axis: Line,
    pub radius: f64
}


impl GoodToRaycast for Cylinder {
    fn is_good_to_raycast(&self, from: &Point, direction: &Point) -> bool {
        Line::from_direction(direction.clone(), from.clone()).get_dist_between_lines(&self.axis) < self.radius * 1.1
    }
}