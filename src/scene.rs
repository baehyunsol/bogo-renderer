use crate::camera::Camera;
use crate::color::Color;
use crate::object::Object;
use crate::render::{RenderOption, get_color_by_light_source};
use crate::light::Light;
use crate::ray::{Collision, shoot_ray};
use crate::img::Buffer;
use crate::gi::map_photons;
use rayon::prelude::*;
use std::sync::Arc;


#[derive(Default)]
pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Arc<Object>>,
    pub lights: Vec<Light>,
    pub sight: f64
}


impl Scene {

    pub fn new() -> Scene {
        Scene{
            sight: 900.0,
            ..Scene::default()
        }
    }

    pub fn push_object(&mut self, object: Object) -> &mut Self {
        self.objects.push(Arc::new(object));
        self
    }

    pub fn render(&mut self, mut option: RenderOption) -> Buffer {

        if option.global_illumination_density > 0 || option.global_illumination_iteration > 0 {
            println!("Global Illumination is not implemented yet!");
            option.no_global_illumination();
        }

        let mut objects = if option.render_lights {
            vec![
                self.objects.clone(),
                self.lights.iter().map(|lt| Arc::new(lt.into_object())).collect()
            ].concat()
        } else {
            self.objects.clone()
        };

        objects.sort_by_key(|obj| obj.get_id());

        let resolution = option.resolution;
        let rays_from_camera = self.camera.get_directions(option.resolution);

        println!("Calculating collisions...");
        let collisions_with_objects = rays_from_camera.clone().par_iter().map(
            |ray|
            shoot_ray(&self.camera.pos, ray, &objects, self.sight)
        ).collect::<Vec<Option<Collision>>>();

        let lights_for_soft_shadows = if option.soft_shadow > 0 {
            self.lights.iter().map(|lt| lt.volumetrize(option.soft_shadow as i32)).collect::<Vec<Vec<Light>>>().concat()
        } else {
            self.lights.clone()
        };

        let mut lights_for_global_illumination = vec![];
        let mut lights_for_global_illumination_iteration = self.lights.clone();

        for i in 0..option.global_illumination_iteration {
            lights_for_global_illumination_iteration = map_photons(&lights_for_global_illumination_iteration, &objects, option.global_illumination_density);

            lights_for_global_illumination = vec![
                lights_for_global_illumination,
                lights_for_global_illumination_iteration.clone()
            ].concat();

            println!("Global illumination iteration: {}\n{} Photons so far", i, lights_for_global_illumination.len());
        }

        let lights = vec![
            lights_for_soft_shadows,
            lights_for_global_illumination
        ].concat();

        println!("Calculating colors...");
        let colors_and_distances = collisions_with_objects.par_iter().map(
            |coll|
            match coll {
                None => None,
                Some((obj, pos)) => {
                    let color = get_color_by_light_source(pos, obj, &objects, &lights);

                    Some(color)
                }
            }
        ).collect::<Vec<Option<Color>>>();

        println!("Postprocessing the image...");
        let mut image_buffer = Buffer::new(resolution);
        let mut bright_buffer = Buffer::new(resolution);

        for x in 0..resolution {

            for y in 0..resolution {

                match &colors_and_distances[y * resolution + x] {
                    None => {}
                    Some(color) => {

                        if option.glow > 0 && color.sum() > 720.0 {
                            bright_buffer.draw_pixel(x, y, color.clone());
                        }

                        else {
                            image_buffer.draw_pixel(x, y, color.clone());
                        }

                    }

                }

            }

        }

        if option.glow > 0 {
            bright_buffer = bright_buffer.glow(option.glow as i32);
        }

        image_buffer = image_buffer.add(&bright_buffer);

        if option.blur_image > 0 {
            image_buffer = image_buffer.blur(option.blur_image as i32);
        }

        println!("Done!");
        image_buffer
    }

}
