use mursten::{
    Application,
    Backend,
    Data,
    Updater,
    Renderer,
};
use mursten_piston_backend::PistonBackend;

use na::*;

pub fn main() {
    let backend = PistonBackend::new();
    let mut variables = Variables::default();
    Application::new(backend)
        .add_updater(ColorRotator)
        .add_renderer(Visual::new())
        .run(variables);
}

struct Variables {
    center: Point2<f32>,
    separation: Vector2<f32>,
    matrix_size: Vector2<f32>,
    ray_proportion: f32,
    glow_amount: f32,     // < 0
    cross_intensity: f32, // < 0
    current_color: Vector3<f32>,
}

impl Variables {
    pub fn new(center: Point2<f32>) -> Self {
        Variables {
            center,
            ..Variables::default()
        }
    }
}

impl Default for Variables {
    fn default() -> Self {
        Variables {
            center: Point2::new(90.0, 60.0),
            separation: Vector2::repeat(5.0),
            matrix_size: Vector2::new(20.0, 15.0),
            ray_proportion: 4.0,
            glow_amount: 5.0,
            cross_intensity: 6.0,
            current_color: Vector3::new(0.1, 0.6, 0.9),
        }
    }
}

impl Data for Variables {}

struct ColorRotator;

impl<B> Updater<B, Variables> for ColorRotator
{
    fn update(
        &mut self,
        _: &mut B,
        var: &mut Variables,
    ) {
        var.current_color = Matrix3::new(0.0, 0.0, 1.0,
                                         1.0, 0.0, 0.0,
                                         0.0, 1.0, 0.0) * var.current_color;
    }
}

struct Visual {
}

impl Visual {
    pub fn new() -> Self {
        Visual {}
    }
}

impl Renderer<PistonBackend, Variables> for Visual {
    fn render(
        &mut self,
        backend: &mut PistonBackend,
        var: &Variables
    ) {
        let (w, h) = backend.screen_size();

        let mut Q: Vec<(Point2<f32>, Rotation2<f32>)> = Vec::new();

        for j in 0..(var.matrix_size.y as u32 * 2 + 1) {
            for i in 0..(var.matrix_size.x as u32 * 2 + 1) {
                if (i, j) == (var.matrix_size.x as u32, var.matrix_size.y as u32) {
                    continue;
                }
                let p = var.matrix_size - Vector2::new(i as f32, j as f32);
                let q = var.center + p.component_mul(&var.separation);
                let r = Rotation2::rotation_between(&p, &Vector2::x());
                Q.push((q, r));
            }
        }

        for y in 0..h {
            // if y % 200 != self.current_row {
            //     continue;
            // }
            for x in 0..w {
                use drafts::visuals::equations::*;

                let mut color = Vector3::new(0.0, 0.0, 0.0);

                for (q, rot) in &Q {
                    let p = Point2::new(x as f32, y as f32);

                    let p2 = transform(&p, &var.center, q, rot, var.ray_proportion);
                    let i = ray_intensity(&p2) * cross_intensity(&(var.center - p.coords), var.cross_intensity);
                    color += i * Vector3::new(red_intensity(p2.x),
                                              green_intensity(p2.x),
                                              blue_intensity(p2.x));
                }

                color += glow_amount((var.center - Point2::new(x as f32, y as f32)).norm(), var.glow_amount) * Vector3::repeat(1.0);
                color = color.map(|c| { clamp(c, 0.0, 1.0) });
                backend.put_pixel((x, y), (color.x, color.y, color.z));
            }

            use std::io::{stdout, Write};
            print!("\rRow {} of {}", y, h);
            stdout().flush().unwrap();
        }
        println!(" ..done");
        // self.current_row = (self.current_row + 1) % 200;
    }
}

mod equations {
    use std::f32::EPSILON;
    use std::f32::consts::{PI, E};
    use na::*;

    pub fn transform(point: &Point2<f32>, center: &Point2<f32>, pivot: &Point2<f32>, rot: &Rotation2<f32>, proportion: f32) -> Point2<f32> {
        let scale = Matrix3::new_nonuniform_scaling(&Vector2::new(1.0 / proportion, 1.0));
        Point2::from_homogeneous(scale * (rot * (point - pivot.coords)).to_homogeneous()).unwrap()
    }
    pub fn ray_intensity(point: &Point2<f32>) -> f32 {
        let (x, y) = (point.x, point.y);
        E.powf(-4.0 * x.powi(4) + 8.0 * x.powi(3) -4.0 * x.powi(2) -100.0 * y.powi(6))
    }
    pub fn cross_intensity(point: &Point2<f32>, intensity: f32) -> f32 {
        E.powf(-(0.001/intensity.powi(4)) * (point.x * point.y).powi(2))
    }
    pub fn red_intensity(scalar: f32) -> f32 {
        (PI * scalar.min(0.5)).cos().powi(2)
    }
    pub fn green_intensity(scalar: f32) -> f32 {
        (PI * scalar).sin().powi(2)
    }
    pub fn blue_intensity(scalar: f32) -> f32 {
        (PI * scalar.max(0.5)).cos().powi(2)
    }
    pub fn glow_amount(scalar: f32, intensity: f32) -> f32 {
        E.powf(-scalar / intensity)
    }

    #[test]
    fn test_cross_intensity() {
        // Center glows at full intensity
        let v = Point2::new(0.0, 0.0);
        let i = 0.123123;
        assert_eq!(cross_intensity(&v, i), 1.0);

        // Also the axis
        let v = Point2::new(0.0, 4621.0);
        let i = 1.0123;
        assert_eq!(cross_intensity(&v, i), 1.0);

        // With intensity 1 at a distance ~7 the rays intensity ~halves
        let v = Point2::new(5.0, 5.0);
        let i = 1.0;
        assert!(cross_intensity(&v, i) - 0.53526145 < EPSILON);

        // With intensity 1 at a distance ~14 the rays intensity already unnoticeable
        let v = Point2::new(10.0, 10.0);
        let i = 1.0;
        assert!(cross_intensity(&v, i) < 0.0001);

        // You need to duplicate the intensity to compensate
        let v = Point2::new(10.0, 10.0);
        let i = 2.0;
        assert!(cross_intensity(&v, i) - 0.53526145 < EPSILON);
    }
}
