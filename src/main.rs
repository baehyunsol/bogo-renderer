mod point;
mod object;
mod shape;
mod camera;
mod color;
mod light;
mod scene;
mod render;
mod img;
mod gi;
mod ray;
mod misc;

use object::Object;
use scene::Scene;
use color::Color;
use light::Light;
use point::Point;
use shape::line::Line;
use shape::line_seg::LineSeg;

fn main() {

    let scenes = vec![
        scene0(),
        scene1(),
        scene2(),
        scene3(),
        scene4(),
        scene5(),
    ];

    let render_option = render::RenderOption::ultra();

    for (ind, mut sc) in scenes.into_iter().enumerate() {
        let image = sc.render(render_option);
        image.save(&format!("scene{}.png", ind)).unwrap();
    }

}


fn scene0() -> Scene {

    let mut result = Scene::new();

    result.push_object(
        Object::new_plane(0.0, 0.0, 1.0, 24.0, Color::new(128, 128, 128))
    );

    for th in 0..7 {
        result.push_object(
            Object::new_cylinder(
                Line::from_direction(
                    Point::new(0.0, 0.0, 1.0),
                    Point::new(64.0 + 16.0 * (0.2 + 3.141592 * 2.0 * th as f64 / 7.0).cos(), 0.0 + 16.0 * (0.2 + 3.141592 * 2.0 * th as f64 / 7.0).sin(), 0.0)
                ),
                0.5,
                Color::new(32, 32, 192)
            )
        );
    }

    result.lights.push(
        Light::new(Point::new(64.0, 0.0, -2.0), 1200.0, 4.0)
    );

    result
}


fn scene1() -> Scene {

    let mut result = Scene::new();

    result.push_object(
        Object::new_plane(0.0, 0.0, 1.0, 24.0, Color::new(128, 128, 128))
    );

    result.push_object(
        Object::new_plane(1.0, 0.0, 0.0, -100.0, Color::new(32, 192, 32))
    );

    for y in 0..8 as i32 {
        result.push_object(
            Object::new_sphere(Point::new(80.0, (y * 12 - 48) as f64, -20.0), 4.0, Color::new(192, 64, 64))
        );
        result.push_object(
            Object::new_sphere(Point::new(56.0, (y * 12 - 48) as f64, -20.0), 4.0, Color::new(192, 64, 64))
        );
    }

    result.push_object(
        Object::new_sphere(Point::new(64.0, 16.0, -12.0), 4.0, Color::new(187, 134, 252))
    );

    result.lights.push(
        Light::new(Point::new(64.0, 0.0, -2.0), 2000.0, 12.0)
    );

    result
}


fn scene2() -> Scene {

    let mut result = Scene::new();

    result.push_object(
        Object::new_plane(0.0, 0.0, 1.0, 24.0, Color::new(128, 128, 128))
    );

    result.push_object(
        Object::new_plane(1.0, 0.0, 0.0, -100.0, Color::new(32, 192, 32))
    );

    result.push_object(
        Object::new_sphere(Point::new(64.0, 16.0, -12.0), 4.0, Color::new(187, 134, 252))
    );

    result.push_object(
        Object::new_cylinder(
            Line::from_points(
                Point::new(64.0, 16.0, -12.0),
                Point::new(56.0, 0.0, -2.0),
            ),
            1.0,
            Color::new(192, 32, 32)
        )
    );

    result.push_object(
        Object::new_cylinder(
            Line::from_points(
                Point::new(56.0, 16.0, -18.0),
                Point::new(68.0, -16.0, -18.0),
            ),
            1.0,
            Color::new(32, 32, 192)
        )
    );

    result.lights.push(
        Light::new(Point::new(64.0, 0.0, -2.0), 2000.0, 12.0)
    );

    result
}


fn scene3() -> Scene {

    let mut result = Scene::new();

    result.push_object(
        Object::new_plane(0.0, 0.0, 1.0, 24.0, Color::new(128, 128, 128))
    );

    for x in -2..3 {
        for y in -2..3 {
            result.push_object(
                Object::new_cylinder(
                    Line::from_direction(
                        Point::new(0.0, 0.0, 1.0),
                        Point::new(58.0 + (x * 12) as f64, 4.0 + (y * 12) as f64, 0.0)
                    ),
                    0.5,
                    Color::new(32, 32, 192)
                )
            );
        }

    }

    result.lights.push(
        Light::new(Point::new(64.0, 0.0, -2.0), 2000.0, 12.0)
    );

    result
}


fn scene4() -> Scene {

    let mut result = Scene::new();

    result.push_object(
        Object::new_plane(0.0, 0.0, 1.0, 24.0, Color::new(192, 64, 64))
    );

    result.push_object(
        Object::new_sphere(Point::new(64.0, 16.0, -20.0), 4.0, Color::new(0, 164, 33))
    );

    result.push_object(
        Object::new_sphere(Point::new(64.0, -16.0, -20.0), 4.0, Color::new(0, 164, 33))
    );

    result.push_object(
        Object::new_capsule(
            LineSeg::new(
                Point::new(88.0, 16.0, -20.0),
                Point::new(88.0, -16.0, -20.0)
            ),
            4.0,
            Color::new(187, 134, 252)
        )
    );

    result.push_object(
        Object::new_capsule(
            LineSeg::new(
                Point::new(53.0, -6.0, -36.0),
                Point::new(53.0, -6.0, -20.0)
            ),
            2.0,
            Color::new(187, 134, 252)
        )
    );

    result.push_object(
        Object::new_capsule(
            LineSeg::new(
                Point::new(75.0, -6.0, -20.0),
                Point::new(75.0, -6.0, -36.0)
            ),
            2.0,
            Color::new(187, 134, 252)
        )
    );

    result.push_object(
        Object::new_sphere(
            Point::new(216.0, -48.0, 56.0),
            8.0,
            Color::new(164, 90, 102)
        )
    );

    result.lights.push(
        Light::new(Point::new(240.0, -80.0, 120.0), 60000.0, 24.0)
    );

    result.lights.push(
        Light::new(Point::new(72.0, 32.0, 8.0), 800.0, 6.0)
    );

    result
}


fn scene5() -> Scene {

    let mut result = Scene::new();

    let edges = 12;
    let center = Point::new(64.0, 0.0, 0.0);
    let rounds = (0..edges + 1).map(
        |i|
        center.add(
            &Point::new(
                (0.2 + 3.14159265 * 2.0 * i as f64 / edges as f64).cos(),
                (0.2 + 3.14159265 * 2.0 * i as f64 / edges as f64).sin(),
                0.0
            ).mul(12.0)
        )
    ).collect::<Vec<Point>>();
    let top = center.add(&Point::new(0.0, 0.0, 16.0));
    let bottom = center.add(&Point::new(0.0, 0.0, -16.0));
    
    for i in 0..edges {
        result.push_object(
            Object::new_triangle(
                top.clone(),
                rounds[i].clone(),
                rounds[i + 1].clone(),
                Color::new(32, 32, 192)
            )
        );
        result.push_object(
            Object::new_triangle(
                bottom.clone(),
                rounds[i].clone(),
                rounds[i + 1].clone(),
                Color::new(32, 32, 192)
            )
        );
    }

    result.push_object(
        Object::new_sphere(
            Point::new(64.0, 0.0, -116.0),
            100.0,
            Color::new(192, 64, 64)
        )
    );

    result.lights.push(
        Light::new(Point::new(72.0, 24.0, 12.0), 2400.0, 6.0)
    );

    result.lights.push(
        Light::new(Point::new(56.0, -24.0, 12.0), 2400.0, 6.0)
    );

    result
}
