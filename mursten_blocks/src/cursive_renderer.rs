pub use cursive;
use cursive::Cursive;
use mursten::{Backend, Data, Renderer};


pub struct CursiveRenderer<View> {
    view: View,
    screen: Cursive,
}

impl<View, D> CursiveRenderer<View>
where
    D: Data,
    View: CursiveView<Model=D>,
{
    pub fn new(mut view: View) -> Self {
        let mut screen = Cursive::default();
        view.configure(&mut screen);
        CursiveRenderer { view, screen }
    }
}

impl<B, D, View> Renderer<B, D> for CursiveRenderer<View>
where
    B: Backend<D>,
    D: Data,
    View: CursiveView<Model=D>,
{
    fn render(&mut self, _: &mut B, data: &D) {
        if self.view.need_to_update(data) {
            self.view.update(&mut self.screen, data);
        }
        self.screen.step();
    }
}

pub trait CursiveView {
    type Model;
    fn configure(&mut self, &mut Cursive);
    fn update(&mut self, &mut Cursive, &Self::Model);
    fn need_to_update(&mut self, _: &Self::Model) -> bool {
        true
    }
}

