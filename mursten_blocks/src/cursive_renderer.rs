use std::sync::mpsc::Sender;

use cursive::Cursive;
use mursten::{Backend, Data, Renderer};

use super::events::EventEmitter;


/// Reexported dependecies
/// ----------------------

pub use cursive;


/// Structure and trait definition
/// ------------------------------

pub struct CursiveRenderer<V, E>
where
    V: CursiveView<Event=E>,
{
    view: V,
    context: CursiveContext<E>,
}

pub struct CursiveContext<E> {
    screen: Cursive,
    envent_senders: Vec<Sender<E>>
}

pub trait CursiveView {
    type Model;
    type Event;
    fn configure(
        &mut self,
        &mut CursiveContext<Self::Event>,
    );
    fn update(
        &mut self,
        &mut CursiveContext<Self::Event>,
        &Self::Model
    );
    fn need_to_update(&mut self, _: &Self::Model) -> bool {
        true
    }
}


/// Renderer implemtation
/// ---------------------

impl<V, E, D> CursiveRenderer<V, E>
where
    D: Data,
    V: CursiveView<Model=D, Event=E>,
{
    pub fn new(mut view: V) -> Self {
        let mut context = CursiveContext {
            screen: Cursive::default(),
            envent_senders: Vec::new(),
        };
        view.configure(&mut context);
        CursiveRenderer {
            view,
            context,
        }
    }
}

impl<E> CursiveContext<E> {
    pub fn screen<'a>(&'a mut self) -> &'a mut Cursive {
        &mut self.screen
    }
    pub fn step(&mut self) {
        self.screen.step();
    }
}

impl<B, D, V, E> Renderer<B, D> for CursiveRenderer<V, E>
where
    D: Data,
    B: Backend<D>,
    V: CursiveView<Model=D, Event=E>,
{
    fn render(&mut self, _: &mut B, data: &D) {
        if self.view.need_to_update(data) {
            self.view.update(&mut self.context, data);
        }
        self.context.step();
    }
}


/// Event support implemtation
/// --------------------------

impl<E> CursiveContext<E>
where 
    E: Clone + Send,
{
    pub fn dispatch_event(&mut self, ev: E) {
        for sender in self.envent_senders.iter() {
            sender.send(ev.clone())
                .expect("Failed to dispatch event");
        }
    }
}

impl<V, E> EventEmitter<E> for CursiveRenderer<V, E>
where
    E: Clone + Send,
    V: CursiveView<Event=E>,
{
    fn connect_to(&mut self, s: Sender<E>) {
        self.context.connect_to(s)
    }
} 

impl<E> EventEmitter<E> for CursiveContext<E>
where
    E: Clone + Send,
{
    fn connect_to(&mut self, s: Sender<E>) {
        self.envent_senders.push(s);
    }
} 

