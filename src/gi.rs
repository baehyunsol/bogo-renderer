use crate::point::Point;
use crate::object::Object;
use crate::light::Light;
use crate::ray::shoot_ray;
use crate::render::get_color_by_light_source;
use crate::color::Color;
use crate::shape::{NormVec, Distance};
use std::sync::Arc;


pub fn gi(pos: &Point, object: Arc<Object>, directions: &Vec<Point>, objects: &Vec<Arc<Object>>, lights: &Vec<Light>) -> Color {

    let norm_vec = object.get_normal_vector_at(pos).normalize();
    let directions = directions.iter().filter(|dir| norm_vec.inner_product(dir) > 0.0).map(|dir| dir.clone()).collect::<Vec<Point>>();
    let directions_len = directions.len() / 2;

    let colors = directions.iter().filter_map(
        |dir| match shoot_ray(&pos, dir, objects, 300.0) {
            None => None,
            Some((collision_obj, collision_pos)) => {
                let collision_dist = collision_pos.get_dist(pos);

                if collision_dist < 0.1 {
                    None  // should not collide to itself
                } 

                else {
                    let collision_color = get_color_by_light_source(&collision_pos, &collision_obj, objects, lights);
                    let collision_cos = norm_vec.inner_product(&pos.sub(&collision_pos).normalize()).abs();

                    Some(collision_color.into_f64().mul(collision_cos / collision_dist / collision_dist / directions_len as f64))
                }

            }

        }
    );

    let mut result = Color::new(0, 0, 0).into_f64();

    for cl in colors {
        result = result.add(&cl);
    }

    result.into_u8()
}