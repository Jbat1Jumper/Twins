use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;

use mursten::{
    Backend,
    Data,
    UpdateChain,
//  RenderChain,
};

pub struct PistonBackend {}

impl PistonBackend {
    pub fn new() -> Self {
        PistonBackend {}
    }

    pub fn screen_size(&self) -> (u32, u32) {
        (300, 200)
    }

    pub fn put_pixel(&mut self, _pos: (u32, u32), _color: (f32, f32, f32)) {}

}

impl<D> Backend<D> for PistonBackend
where
    D: Data,
{
    fn run(
        &mut self,
        mut update_chain: UpdateChain<Self, D>,
//      mut render_chain: RenderChain<Self, D>,
        mut data: D,
    ) -> D {
        let mut window: Window = WindowSettings::new(
                "piston: hello_world",
                [300, 200]
            )
            .exit_on_esc(true)
            //.opengl(OpenGL::V2_1) // Set a different OpenGl version
            .build()
            .unwrap();

        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            if let Some(_) = e.render_args() {
                update_chain.update(self, &mut data);
            }

            if let Some(_) = e.update_args() {
//                  render_fn(backend, data);
            }
        }

        data
    }
}
