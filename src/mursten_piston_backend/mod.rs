use piston_window::{
    PistonWindow,
    WindowSettings,
};
use piston_window::{G2d, Context};

use mursten::{
    Backend,
    Data,
    UpdateChain,
    RenderChain,
};

pub struct PistonBackend {
    graphics_queue: Vec<Operation>
}

type Operation = Box<FnMut(&mut G2d, Context)>;

impl PistonBackend {
    pub fn new() -> Self {
        PistonBackend {
            graphics_queue: Vec::new(),
        }
    }

    pub fn screen_size(&self) -> (u32, u32) {
        (300, 200)
    }

    pub fn draw(&mut self, o: Operation) {
        self.graphics_queue.push(o);
    }
}

impl<D> Backend<D> for PistonBackend
where
    D: Data,
{
    fn run(
        &mut self,
        mut update_chain: UpdateChain<Self, D>,
        mut render_chain: RenderChain<Self, D>,
        mut data: D,
    ) -> D {
        let mut window: PistonWindow = WindowSettings::new(
                "piston: hello_world",
                [300, 200]
            )
            .exit_on_esc(true)
            //.opengl(OpenGL::V2_1) // Set a different OpenGl version
            .build()
            .unwrap();

        while let Some(event) = window.next() {

            update_chain.update(self, &mut data);

            render_chain.render(self, &mut data);

            let l = self.graphics_queue.len();
            let mut graphics_queue: Vec<Operation> = self.graphics_queue.drain(0..l).collect();

            window.draw_2d(&event, |context, graphics| {
                use piston_window::{clear, rectangle};

                clear([1.0; 4], graphics);
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                          [0.0, 0.0, 100.0, 100.0],
                          context.transform,
                          graphics);

                for o in graphics_queue.iter_mut() {
                    o(graphics, context);
                }
            });

        }

        data
    }
}
