use crate::point::Point;
use crate::shape::sphere;


pub struct Camera {
    pub pos: Point,
    pub phi: f64,
    pub theta: f64,
    pub zoom: f64
}


impl Default for Camera {
    fn default() -> Camera {
        Camera {
            pos: Point::new(0.0, 0.0, 0.0),
            phi: 0.0,
            theta: 3.14159265 / 2.0,
            zoom: 1.0
        }
    }
}


impl Camera {

    // directions[y * resolution + x]
    pub fn get_directions(&self, resolution: usize) -> Vec<Point> {

        let range = 1.0 / self.zoom;
        let delta_angle = range / resolution as f64;
        let mut directions = Vec::with_capacity(resolution * resolution);

        let phi_begin = self.phi - range / 2.0;
        let theta_begin = self.theta - range / 2.0;

        for theta in 0..resolution {

            for phi in 0..resolution {
                directions.push(sphere::coord(1.0, theta_begin + delta_angle * theta as f64, phi_begin + delta_angle * phi as f64));
            }

        }

        directions
    }

    pub fn set_zoom(&mut self, zoom: f64) -> &mut Self {
        self.zoom = zoom;
        self
    }

    pub fn set_theta(&mut self, theta: f64) -> &mut Self {
        self.theta = theta;
        self
    }

    pub fn set_phi(&mut self, phi: f64) -> &mut Self {
        self.phi = phi;
        self
    }

}