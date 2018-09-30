extern crate mursten;
extern crate mursten_blocks;
extern crate nalgebra;
extern crate rand;

use mursten::{Application, Data};
use mursten::dummy::DummyBackend;
use mursten_blocks::cursive_renderer::{CursiveRenderer, CursiveView, CursiveContext};
use mursten_blocks::cursive_renderer::cursive::Cursive;
use mursten_blocks::cursive_renderer::cursive::views::*;
use mursten_blocks::cursive_renderer::cursive::traits::*;
use mursten_blocks::events::SimpleEventReceiver;


pub fn main() {
    Application::new(DummyBackend::new())
        .add_renderer(CursiveRenderer::new(View::new()))
        .run(Model::new());
}

struct Model {
    name: String,
}

impl Model {
    pub fn new() -> Self {
        Self { name: "Pedro".to_owned() }
    }
}

impl Data for Model { }

struct View {
}

impl View {
    pub fn new() -> Self {
        Self {}
    }
}

impl CursiveView for View {
    type Model = Model;
    type Event = ();
    fn configure(&mut self, ctx: &mut CursiveContext<Self::Event>) {
        let mut s = ctx.screen();
        s.add_layer(
            Dialog::around(
                TextView::new("_this will be replaced by the real text_")
                    .with_id("model.name")
            )
                .title("The name in the model")
                .button("Randomize name", |_| { })
                .button("Quit", |_| { })
        );
    }
    fn update(&mut self, ctx: &mut CursiveContext<Self::Event>, model: &Self::Model) {
        let mut s = ctx.screen();
        s.call_on_id("model.name", |tv: &mut TextView| {
            tv.set_content(model.name.clone());
        });
    }
}

