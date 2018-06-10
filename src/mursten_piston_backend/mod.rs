use piston_window::{
    PistonWindow,
    WindowSettings,
};

use mursten::{
    Backend,
    Data,
    UpdateChain,
    RenderChain,
};

pub struct PistonBackend {
    graphics_queue: Vec<Operation>
}

enum Operation {
    PutPixel((u32, u32), (f32, f32, f32)),
}

impl PistonBackend {
    pub fn new() -> Self {
        PistonBackend {
            graphics_queue: Vec::new(),
        }
    }

    pub fn screen_size(&self) -> (u32, u32) {
        (300, 200)
    }

    pub fn put_pixel(&mut self, pos: (u32, u32), color: (f32, f32, f32)) {
        self.graphics_queue.push(Operation::PutPixel(pos, color));
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

            let operations = self.graphics_queue.drain(..);

            window.draw_2d(&event, |context, graphics| {
                use piston_window::{clear, rectangle};

                clear([1.0; 4], graphics);
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                          [0.0, 0.0, 100.0, 100.0],
                          context.transform,
                          graphics);

                for o in operations {
                    match o {
                        Operation::PutPixel((x, y), (r, g, b)) => {
                            rectangle(
                                [r, g, b, 1.0],
                                [x as f64, y as f64, (x + 1) as f64, (y + 1) as f64],
                                context.transform,
                                graphics
                            );
                        }
                    }
                }
            });
        }

        data
    }
}
