extern crate image;
extern crate mursten;
extern crate piston_window;

use piston_window::{PistonWindow, Texture, TextureSettings, Transformed, WindowSettings};

use mursten::{Backend, Data, RenderChain, UpdateChain};

use image as im;

pub struct PistonBackend {
    graphics_queue: Vec<Operation>,
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
        (180, 120)
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
        mut self,
        mut update_chain: UpdateChain<Self, D>,
        mut render_chain: RenderChain<Self, D>,
        mut data: D,
    ) -> D {
        let mut window: PistonWindow = WindowSettings::new(
                "piston: hello_world",
                [900, 600]
            )
            .exit_on_esc(true)
            //.opengl(OpenGL::V2_1) // Set a different OpenGl version
            .build()
            .unwrap();

        while let Some(event) = window.next() {
            update_chain.update(&mut self, &mut data);
            render_chain.render(&mut self, &mut data);

            let canvas = {
                let (w, h) = self.screen_size();
                let mut canvas = im::ImageBuffer::new(w, h);
                for op in self.graphics_queue.drain(..) {
                    match op {
                        Operation::PutPixel((x, y), (r, g, b)) => {
                            canvas.put_pixel(
                                x,
                                y,
                                im::Rgba([
                                    (r * 255.0) as u8,
                                    (g * 255.0) as u8,
                                    (b * 255.0) as u8,
                                    255,
                                ]),
                            );
                        }
                    }
                }
                canvas
            };

            let mut texture =
                Texture::from_image(&mut window.factory, &canvas, &TextureSettings::new()).unwrap();

            texture.update(&mut window.encoder, &canvas).unwrap();

            window.draw_2d(&event, |context, graphics| {
                use piston_window::{clear, image};

                clear([1.0; 4], graphics);
                image(&texture, context.zoom(5.0).transform, graphics);
            });
        }

        data
    }

    fn quit(&mut self) {
        panic!("A delicate exit");
    }
}
