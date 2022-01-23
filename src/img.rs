use crate::point::Point;
use crate::color::Color;
use image::ImageBuffer;


pub struct Buffer {
    pub buffer: Vec<Vec<Color>>,
    resolution: usize
}


impl Buffer {

    pub fn new(resolution: usize) -> Buffer {
        Buffer {
            buffer: vec![vec![Color::new(0, 0, 0);resolution];resolution],
            resolution
        }
    }

    pub fn draw_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.buffer[y][x] = color;
    }

    pub fn glow(&self, size: i32) -> Buffer {

        let mut result = Buffer::new(self.resolution);

        for x in 0..self.resolution as i32 {

            for y in 0..self.resolution as i32 {
                let mut color_count = 0.0;
                let mut denom_count = 0.0;

                if self.buffer[y as usize][x as usize].sum() > 720.0 {
                    result.draw_pixel(x as usize, y as usize, Color::white());
                    continue;
                }

                for dx in -size..size + 1 {

                    if x + dx < 0 || x + dx >= self.resolution as i32 {
                        continue;
                    }

                    for dy in -size..size + 1 {

                        if y + dy < 0 || y + dy >= self.resolution as i32 {
                            continue;
                        }
                        
                        if self.buffer[(y + dy) as usize][(x + dx) as usize].sum() > 720.0 {
                            color_count += self.buffer[(y + dy) as usize][(x + dx) as usize].sum() / 765.0;
                        }

                        denom_count += 1.0;
                    }

                }

                result.draw_pixel(x as usize, y as usize, Color::white().mul(color_count / denom_count));
            }

        }

        result
    }

    pub fn blur(&self, size: i32) -> Buffer {

        let mut result = Buffer::new(self.resolution);

        for x in 0..self.resolution as i32 {

            for y in 0..self.resolution as i32 {
                let mut color_count = 0.0;
                let mut colors = Point::new(0.0, 0.0, 0.0);

                for dx in -size..size + 1 {

                    if x + dx < 0 || x + dx >= self.resolution as i32 {
                        continue;
                    }

                    for dy in -size..size + 1 {

                        if y + dy < 0 || y + dy >= self.resolution as i32 {
                            continue;
                        }

                        colors = colors.add(&self.buffer[(y + dy) as usize][(x + dx) as usize].as_point());
                        color_count += 1.0;
                    }

                }

                colors = colors.mul(1.0 / color_count);
                result.draw_pixel(x as usize, y as usize, Color::from_point(&colors));
            }

        }

        result
    }

    pub fn contrast(&self, po: usize) -> Buffer {

        let po = po as f64;
        let mut result = Buffer::new(self.resolution);

        for x in 1..self.resolution {

            for y in 1..self.resolution {

                if (self.buffer[y][x - 1].sum() - self.buffer[y][x].sum()).abs() > po {
                    result.draw_pixel(x, y, self.buffer[y][x].clone());
                    result.draw_pixel(x - 1, y, self.buffer[y][x - 1].clone());
                }

                if (self.buffer[y - 1][x].sum() - self.buffer[y][x].sum()).abs() > po {
                    result.draw_pixel(x, y, self.buffer[y][x].clone());
                    result.draw_pixel(x, y - 1, self.buffer[y - 1][x].clone());
                }

            }

        }

        result
    }

    pub fn add(&self, other: &Buffer) -> Buffer {

        assert_eq!(self.resolution, other.resolution);
        let mut result = Buffer::new(self.resolution);

        for x in 0..self.resolution {

            for y in 0..self.resolution {
                result.draw_pixel(x, y, self.buffer[y][x].add(&other.buffer[y][x]));
            }

        }

        result
    }

    pub fn save(&self, save_as: &str) -> Result<(), image::ImageError> {

        let mut into = ImageBuffer::new(self.resolution as u32, self.resolution as u32);

        for x in 0..self.resolution {

            for y in 0..self.resolution {
                let curr = &self.buffer[y][x];

                into.put_pixel(x as u32, y as u32, image::Rgb([curr.r, curr.g, curr.b]));
            }

        }

        into.save(save_as)
    }

    pub fn noise() -> Buffer {

        let mut result = Buffer::new(625);

        for x in 0..25 {
            for y in 0..25 {
                let curr_c = rand::random::<u8>();
                let cc = Color::new(curr_c, curr_c, curr_c);
                for xx in 0..25 {
                    for yy in 0..25 {
                        result.draw_pixel(x * 25 + xx, y * 25 + yy, cc.clone())
                    }
                }
            }
        }

        result.blur(12)
    }

}