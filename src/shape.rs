use crate::point::Point;
use plane::Plane;
use sphere::Sphere;
use cylinder::Cylinder;
use line::Line;
use line_seg::LineSeg;
use capsule::Capsule;
use triangle::Triangle;

pub mod triangle;
pub mod line;
pub mod line_seg;
pub mod capsule;
pub mod cylinder;
pub mod sphere;
pub mod plane;


pub trait Distance {
    fn get_dist(&self, to: &Point) -> f64;
}

pub trait NormVec {
    fn get_normal_vector_at(&self, at: &Point) -> Point;
}

pub trait GoodToRaycast {
    fn is_good_to_raycast(&self, from: &Point, direction: &Point) -> bool;
}


#[derive(Clone)]
pub enum Shape {
    Plane(Plane),
    Sphere(Sphere),
    Cylinder(Cylinder),
    Capsule(Capsule),
    Triangle(Triangle)
}


impl Shape {
    pub fn new_sphere(center: Point, radius: f64) -> Shape {
        Shape::Sphere(Sphere {center, radius})
    }

    pub fn new_plane(a: f64, b: f64, c: f64, d: f64) -> Shape {
        Shape::Plane(Plane::new(a, b, c, d))
    }

    pub fn new_cylinder(axis: Line, radius: f64) -> Shape {
        Shape::Cylinder(Cylinder {axis, radius})
    }

    pub fn new_capsule(axis: LineSeg, radius: f64) -> Shape {
        Shape::Capsule(Capsule {axis, radius})
    }

    pub fn new_triangle(v1: Point, v2: Point, v3: Point) -> Shape {
        Shape::Triangle(Triangle::new(v1, v2, v3))
    }

}


impl Distance for Shape {
    fn get_dist(&self, to: &Point) -> f64 {
        match self {
            Shape::Plane(pl) => pl.get_dist(to),
            Shape::Sphere(sp) => sp.get_dist(to),
            Shape::Cylinder(cl) => (cl.axis.get_dist(to) - cl.radius).abs(),
            Shape::Capsule(cp) => (cp.axis.get_dist(to) - cp.radius).abs(),
            Shape::Triangle(tr) => tr.get_dist(to),
        }
    }
}


impl NormVec for Shape {
    fn get_normal_vector_at(&self, at: &Point) -> Point {
        match self {
            Shape::Plane(pl) => pl.get_normal_vector_at(at),
            Shape::Sphere(sp) => sp.get_normal_vector_at(at),
            Shape::Cylinder(cl) => cl.axis.get_normal_vector_at(at),
            Shape::Capsule(cp) => cp.axis.get_normal_vector_at(at),
            Shape::Triangle(tr) => tr.get_normal_vector_at(at),
        }
    }
}


impl GoodToRaycast for Shape {
    fn is_good_to_raycast(&self, from: &Point, direction: &Point) -> bool {
        match self {
            Shape::Plane(pl) => pl.is_good_to_raycast(from, direction),
            Shape::Sphere(sp) => sp.is_good_to_raycast(from, direction),
            Shape::Cylinder(cl) => cl.is_good_to_raycast(from, direction),
            Shape::Capsule(cp) => cp.is_good_to_raycast(from, direction),
            Shape::Triangle(tr) => tr.is_good_to_raycast(from, direction),
        }
    }
}