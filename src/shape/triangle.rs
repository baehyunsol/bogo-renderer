use super::plane::Plane;
use super::line_seg::LineSeg;
use super::{Distance, NormVec, GoodToRaycast};
use crate::point::Point;


#[derive(Clone)]
pub struct Triangle {
    v1: Point,
    v2: Point,
    v3: Point,

    e1: LineSeg,
    e2: LineSeg,
    e3: LineSeg,

    surface: Plane,
}


impl Triangle {

    pub fn new(v1: Point, v2: Point, v3: Point) -> Triangle {

        let e1 = LineSeg::new(v1.clone(), v2.clone());
        let e2 = LineSeg::new(v2.clone(), v3.clone());
        let e3 = LineSeg::new(v3.clone(), v1.clone());

        let surface_norm = v1.sub(&v2).cross_product(&v2.sub(&v3));

        if surface_norm.is_zero() {
            panic!("Invalid triangle!");
        }

        let surface = Plane::new(
            surface_norm.x,
            surface_norm.y,
            surface_norm.z,
            -surface_norm.inner_product(&v1)
        );

        Triangle {
            v1, v2, v3,
            e1, e2, e3,
            surface
        }
    }

}


fn is_on_same_side(p1: &Point, p2: &Point, line1: &Point, line2: &Point) -> bool {

    let line = line1.sub(&line2);

    let check1 = p1.sub(&line2).cross_product(&line);
    let check2 = p2.sub(&line2).cross_product(&line);

    check1.inner_product(&check2) > 0.0
}


// https://blackpawn.com/texts/pointinpoly/
pub fn is_inside_triangle(p: &Point, triangle1: &Point, triangle2: &Point, triangle3: &Point) -> bool {
    
    return is_on_same_side(p, triangle1, triangle2, triangle3) && 
    is_on_same_side(p, triangle2, triangle1, triangle3) &&
    is_on_same_side(p, triangle3, triangle1, triangle2);
}


impl Distance for Triangle {

    fn get_dist(&self, to: &Point) -> f64 {

        let foot = self.surface.get_perpendicular_foot(to);

        if is_inside_triangle(&foot, &self.v1, &self.v2, &self.v3) {
            foot.get_dist(to)
        }

        else {
            self.e1.get_dist(to).min(
                self.e2.get_dist(to)
            ).min(
                self.e3.get_dist(to)
            )
        }

    }

}


impl NormVec for Triangle {

    fn get_normal_vector_at(&self, at: &Point) -> Point {

        if at.sub(&self.v1).inner_product(&self.surface.norm_vec) > 0.0 {
            self.surface.norm_vec.clone()
        }

        else {
            self.surface.norm_vec.neg()
        }

    }

}


impl GoodToRaycast for Triangle {

    fn is_good_to_raycast(&self, from: &Point, direction: &Point) -> bool {
        let t = (-self.surface.d - self.surface.norm_vec.inner_product(from)) / self.surface.norm_vec.inner_product(direction);
        let inter = from.add(&direction.mul(t));

        is_inside_triangle(&inter, &self.v1, &self.v2, &self.v3)
    }

}