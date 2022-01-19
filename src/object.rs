use crate::shape::{Shape, NormVec, Distance, GoodToRaycast};
use crate::point::Point;
use crate::shape::line::Line;
use crate::shape::line_seg::LineSeg;
use crate::color::Color;


#[derive(Clone)]
pub struct Object {
    pub id: u128,
    pub shape: Shape,
    pub color: Color,

    pub is_light: bool
}


impl Object {

    pub fn new_sphere(center: Point, radius: f64, color: Color) -> Object {
        Object {
            id: rand::random::<u128>(),
            shape: Shape::new_sphere(center, radius),
            color,
            is_light: false
        }
    }

    pub fn new_plane(a: f64, b: f64, c: f64, d: f64, color: Color) -> Object {
        Object {
            id: rand::random::<u128>(),
            shape: Shape::new_plane(a, b, c, d),
            color,
            is_light: false
        }
    }

    pub fn new_cylinder(axis: Line, radius: f64, color: Color) -> Object {
        Object {
            id: rand::random::<u128>(),
            shape: Shape::new_cylinder(axis, radius),
            color,
            is_light: false
        }
    }

    pub fn new_capsule(axis: LineSeg, radius: f64, color: Color) -> Object {
        Object {
            id: rand::random::<u128>(),
            shape: Shape::new_capsule(axis, radius),
            color,
            is_light: false
        }
    }

    pub fn new_triangle(v1: Point, v2: Point, v3: Point, color: Color) -> Object {
        Object {
            id: rand::random::<u128>(),
            shape: Shape::new_triangle(v1, v2, v3),
            color,
            is_light: false
        }
    }

    pub fn get_id(&self) -> u128 {
        self.id
    }

}


impl Distance for Object {
    fn get_dist(&self, to: &Point) -> f64 {
        self.shape.get_dist(to)
    }
}


impl NormVec for Object {
    fn get_normal_vector_at(&self, at: &Point) -> Point {
        self.shape.get_normal_vector_at(at)
    }
}


impl GoodToRaycast for Object {
    fn is_good_to_raycast(&self, from: &Point, direction: &Point) -> bool {
        self.shape.is_good_to_raycast(from, direction)
    }
}
