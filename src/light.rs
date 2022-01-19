use crate::point::Point;
use crate::object::Object;
use crate::color::Color;


#[derive(Clone)]
pub struct Light {
    pub id: u128,
    pub pos: Point,
    pub power: f64,
    pub size: f64
}


impl Light {

    pub fn new(pos: Point, power: f64, size: f64) -> Light {
        Light {pos, power, size, id: rand::random::<u128>()}
    }

    pub fn volumetrize(&self, count: i32) -> Vec<Light> {

        let count_cube = (2 * count + 1) * (2 * count + 1) * (2 * count + 1);
        let mut result = Vec::with_capacity(count_cube as usize);

        let count_f64 = count as f64;
        let new_power = self.power / count_cube as f64;

        for x in -count..count + 1 {
            for y in -count..count + 1 {
                for z in -count..count + 1 {
                    result.push(
                        Light::new(
                            self.pos.add(&Point::new(x as f64, y as f64, z as f64).mul(self.size / count_f64 / 2.0)),
                            new_power,
                            1.0
                        )
                    );
                }
            }
        }

        result
    }

    pub fn into_object(&self) -> Object {
        let mut result = Object::new_sphere(self.pos.clone(), self.size / 4.0, Color::new(255, 255, 255));
        result.is_light = true;
        result.id = self.id;

        result
    }

}