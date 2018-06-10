use mursten::{
    Application,
    Data,
    Renderer,
};
use mursten_piston_backend::PistonBackend;

pub fn main() {
    let backend = PistonBackend::new();
    let mut variables = Variables::new((200.0, 200.0));
    Application::new(backend)
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
        }
    }
}

impl Data for Variables {}

struct Visual;

impl Renderer<PistonBackend, Variables> for Visual {
    fn render(&mut self, backend: &mut PistonBackend, variables: &Variables) {}
}
