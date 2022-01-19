use crate::point::Point;
use crate::object::Object;
use crate::shape::{Distance, GoodToRaycast};
use std::sync::Arc;


// id, position
pub type Collision = (Arc<Object>, Point);


pub fn shoot_ray(from: &Point, direction: &Point, objects: &Vec<Arc<Object>>, sight: f64) -> Option<Collision> {

    let objects = objects.iter().filter(|obj| obj.is_good_to_raycast(from, direction)).map(|obj| obj.clone()).collect::<Vec<Arc<Object>>>();

    if objects.len() == 0 {
        return None;
    }

    let direction = direction.normalize();

    let mut curr_pos = from.clone();
    let mut count = 0;
    let mut last_min_dist = -1.0;

    loop {
        let mut curr_min_dist = objects[0].get_dist(&curr_pos);
        let mut curr_closest_obj = &objects[0];

        for obj in objects.iter() {

            if obj.get_dist(&curr_pos) < curr_min_dist {
                curr_min_dist = obj.get_dist(&curr_pos);
                curr_closest_obj = &obj;
            }

        }

        curr_pos = curr_pos.add(&direction.mul(curr_min_dist));

        if curr_min_dist < 0.00001 && curr_min_dist < last_min_dist {
            return Some((curr_closest_obj.clone(), curr_pos));
        }

        if curr_pos.get_dist(from) > sight {
            return None;
        }

        count += 1;
        last_min_dist = curr_min_dist;

        // emergency escape
        if count == 2048 {
            return Some((curr_closest_obj.clone(), curr_pos));
        }
    }

}


pub fn is_visible(viewer: &Point, viewee: &Point, objects: &Vec<Arc<Object>>, viewee_id: u128) -> bool {

    let direction = viewee.sub(viewer);
    let dist = direction.get_norm();

    match shoot_ray(viewer, &direction, objects, dist * 1.1) {
        None => true,
        Some((obj, p)) => {
            if p.get_dist(viewer) < dist && obj.get_id() != viewee_id {
                false
            }
            else {
                true
            }
        }
    }

}