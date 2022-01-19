use crate::point::Point;
use super::line::Line;
use super::{Distance, NormVec, GoodToRaycast};
use hexasphere::shapes::IcoSphere;


#[derive(Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64
}


impl Distance for Sphere {
    fn get_dist(&self, to: &Point) -> f64 {
        (self.center.get_dist(to) - self.radius).abs()
    }
}


impl NormVec for Sphere {
    fn get_normal_vector_at(&self, at: &Point) -> Point {
        at.sub(&self.center).normalize()
    }
}


impl GoodToRaycast for Sphere {
    fn is_good_to_raycast(&self, from: &Point, direction: &Point) -> bool {
        Line::from_direction(direction.clone(), from.clone()).get_dist(&self.center) < self.radius * 1.1
    }
}


/*
|  theta  |   phi   |       position       |
| 0       | 0       | x-axis positive      |
| 0.1 pi  | 0.1 pi  | all-axis positive    |
*/
pub fn coord(r: f64, theta: f64, phi: f64) -> Point {
    Point::new(
        r * theta.sin() * phi.cos(),
        r * theta.sin() * phi.sin(),
        r * theta.cos()
    )
}


/*
num of vertices
1 42
2 92
3 162
4 252
5 362
6 492
7 642
8 812
9 1002
10 1212
11 1442
12 1692
13 1962
14 2252
15 2562
16 2892
17 3242
18 3612
19 4002
20 4412
21 4842
22 5292
23 5762
*/
pub fn get_sphere_vertices(scale: usize) -> Vec<Point> {

    let sphere = IcoSphere::new(scale, |_| ());
    let points = sphere.raw_points();

    points.iter().map(|pt| Point::new(pt.x as f64, pt.y as f64, pt.z as f64)).collect()
}
