use crate::light::Light;
use crate::object::Object;
use crate::shape::sphere::get_sphere_vertices;
use crate::ray::shoot_ray;
use crate::shape::Distance;
use crate::render::get_light_power_at;
use std::sync::Arc;


pub fn map_photons(lights: &Vec<Light>, objects: &Vec<Arc<Object>>, density: usize) -> Vec<Light> {

    let directions = get_sphere_vertices(density);
    let power_coeff = 5000.0 / directions.len() as f64;
    let mut result = Vec::with_capacity(lights.len() * directions.len());

    for lt in lights.iter() {

        for dir in directions.iter() {

            match shoot_ray(&lt.pos, dir, objects, 900.0) {
                None => {},
                Some((obj, p)) => {

                    // when photons bounce off multiple times, they should not bounce at the same point multiple times.
                    if lt.pos.get_dist(&p) < 0.1 {
                        continue;
                    }

                    let power = get_light_power_at(&p, &obj, lt) * power_coeff;

                    if power * directions.len() as f64 > 28.0 {
                        result.push(Light::new(p, power, 1.0));
                    }

                }
            }

        }

    }

    result
}