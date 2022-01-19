use crate::object::Object;
use crate::color::Color;
use crate::point::Point;
use crate::light::Light;
use crate::shape::{Distance, NormVec};
use crate::ray::is_visible;
use std::sync::Arc;


#[derive(Copy, Clone)]
pub struct RenderOption {
    pub soft_shadow: usize,
    pub glow: usize,
    pub render_lights: bool,
    pub blur_image: usize,
    pub resolution: usize,
    pub global_illumination_iteration: usize,
    pub global_illumination_density: usize
}

impl Default for RenderOption {
    fn default() -> RenderOption {
        RenderOption::low()
    }
}


impl RenderOption {

    pub fn low() -> RenderOption {
        RenderOption {
            resolution: 256,
            soft_shadow: 0,
            glow: 0,
            render_lights: true,
            blur_image: 0,
            global_illumination_iteration: 0,
            global_illumination_density: 0,
        }
    }

    pub fn medium() -> RenderOption {
        RenderOption {
            resolution: 324,
            glow: 12,
            ..RenderOption::low()
        }
    }

    pub fn high() -> RenderOption {
        RenderOption {
            resolution: 512,
            soft_shadow: 1,
            glow: 16,
            render_lights: true,
            blur_image: 0,
            global_illumination_iteration: 1,
            global_illumination_density: 12,
        }
    }

    pub fn ultra() -> RenderOption {
        RenderOption {
            resolution: 1024,
            soft_shadow : 2,
            glow: 32,
            blur_image: 1,
            render_lights: true,
            global_illumination_iteration: 2,
            global_illumination_density: 24,
        }
    }

    pub fn show_lights(&mut self, show_lights: bool) -> &mut Self {
        self.render_lights = show_lights;
        self
    }

    pub fn no_global_illumination(&mut self) -> &mut Self {
        self.global_illumination_density = 0;
        self.global_illumination_iteration = 0;
        self
    }

    pub fn set_resolution(&mut self, resolution: usize) -> &mut Self {
        self.resolution = resolution;
        self
    }

    pub fn set_blur(&mut self, blur: usize) -> &mut Self {
        self.blur_image = blur;
        self
    }

    pub fn set_glow(&mut self, glow: usize) -> &mut Self {
        self.glow = glow;
        self
    }
}


pub fn get_color_by_light_source(pos: &Point, object: &Arc<Object>, objects: &Vec<Arc<Object>>, lights: &Vec<Light>) -> Color {

    if object.is_light {
        return Color::white();
    }

    let mut light_intensity = 0.0;

    for lt in lights.iter() {

        if is_visible(pos, &lt.pos, objects, lt.id) {
            light_intensity += get_light_power_at(pos, object, lt);
        }

    }

    object.color.mul(light_intensity)
}


// this function does not check visibility!
pub fn get_light_power_at(pos: &Point, object: &Object, light: &Light) -> f64 {
    let cos = object.get_normal_vector_at(pos).normalize().inner_product(&light.pos.sub(pos).normalize()).abs();
    let dist = pos.get_dist(&light.pos);

    light.power * cos / dist / dist
}
