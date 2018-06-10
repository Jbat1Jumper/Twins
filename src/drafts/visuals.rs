use mursten::{
    Application,
    Backend,
    Data,
    Updater,
    Renderer,
};
use mursten_piston_backend::PistonBackend;

pub fn main() {
    let backend = PistonBackend::new();
    let mut variables = Variables::new((200.0, 200.0));
    Application::new(backend)
        .add_updater(ColorRotator)
        .add_renderer(Visual)
        .run(variables);
}

struct Variables {
    center: (f32, f32),
    separation: f32,
    matrix_size: u32,
    ray_proportion: f32,
    glow_amount: f32,     // < 0
    cross_intensity: f32, // < 0
    current_color: (f32, f32, f32),
}

impl Variables {
    pub fn new(center: (f32, f32)) -> Self {
        Variables {
            center,
            ..Variables::default()
        }
    }
}

impl Default for Variables {
    fn default() -> Self {
        Variables {
            center: (0.0, 0.0),
            separation: 20.0,
            matrix_size: 10,
            ray_proportion: 0.5,
            glow_amount: 10.0,
            cross_intensity: 5.0,
            current_color: (0.1, 0.6, 0.9),
        }
    }
}

impl Data for Variables {}

struct ColorRotator;

impl<B> Updater<B, Variables> for ColorRotator
where
    B: Backend<Variables>
{
    fn update(
        &mut self,
        backend: &mut B,
        variables: &mut Variables,
    ) {
        let (r, g, b) = variables.current_color;
        variables.current_color = (g, b, r);
    }
}

struct Visual;

impl Renderer<PistonBackend, Variables> for Visual {
    fn render(&mut self, backend: &mut PistonBackend, variables: &Variables) {
        use piston_window::Context;

        let (w, h) = backend.screen_size();
        let (r, g, b) = variables.current_color;

        backend.draw(Box::new(move |graphics, context: _| {
            use piston_window::*;

            for y in 0..h {
                for x in 0..w {
                    rectangle(
                        [r, g, b, 1.0],
                        [x as f64, y as f64, (x + 1) as f64, (y + 1) as f64],
                        context.transform,
                        graphics
                    );
                }
            }
        }));
    }
}
