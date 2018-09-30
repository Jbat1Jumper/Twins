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
use mursten_blocks::events::{SimpleEventReceiver, EventReceiver, EventEmitter};


pub fn main() {
    let action_reducer = SimpleEventReceiver::new("reducer", |ev: Action| {
        match ev {
            Action::Quit => eprintln!("Quit!"),
            Action::RandomizeName => eprintln!("Randomize name!"),
        }
        true
    });
    let mut cursive_renderer = CursiveRenderer::new("renderer", View::new());
    cursive_renderer.connect_to(action_reducer.address());

    Application::new(DummyBackend::new())
        .add_updater(action_reducer)
        .add_renderer(cursive_renderer)
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

#[derive(Clone)]
enum Action {
    RandomizeName,
    Quit,
}

impl CursiveView for View {
    type Model = Model;
    type Event = Action;
    fn configure(&mut self, ctx: &mut CursiveContext<Self>) {
        let address = ctx.address();
        let randomize_name = move |_: &mut Cursive| {
            address.send(Action::RandomizeName);
        };

        let address = ctx.address();
        let request_quit = move |_: &mut Cursive| {
            address.send(Action::Quit);
        };

        let mut s = ctx.screen();
        s.add_layer(
            Dialog::around(
                TextView::new("_this will be replaced by the real text_")
                    .with_id("model.name")
            )
                .title("The name in the model")
                .button("Randomize name", randomize_name)
                .button("Quit", request_quit)
        );
    }
    fn update(&mut self, ctx: &mut CursiveContext<Self>, model: &Self::Model) {
        let mut s = ctx.screen();
        s.call_on_id("model.name", |tv: &mut TextView| {
            tv.set_content(model.name.clone());
        });
    }
}

