use cursive::Cursive;
use mursten::{Backend, Data, Renderer};

use super::events::{EventEmitter, EventReceiver, EventResult};
use super::events::transport::{Mailbox, Address, AddressBook};


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
    address_book: AddressBook<E>, 
    mailbox: Mailbox<E>,
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
    E: Clone,
{
    pub fn new(name: &'static str, mut view: V) -> Self {
        let mut context = CursiveContext {
            screen: Cursive::default(),
            address_book: AddressBook::new(),
            mailbox: Mailbox::new(name),
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
    E: Clone,
{
    fn render(&mut self, _: &mut B, data: &D) {
        if self.view.need_to_update(data) {
            self.view.update(&mut self.context, data);
        }
        self.context.step();
        self.context.pump_events();
    }
}


/// Event support implemtation
/// --------------------------

impl<V, E> EventEmitter<E> for CursiveRenderer<V, E>
where
    V: CursiveView<Event=E>,
    E: Clone,
{
    fn connect_to(&mut self, addr: Address<E>) {
        self.context.connect_to(addr)
    }
} 

impl<E> EventEmitter<E> for CursiveContext<E>
where
    E: Clone,
{
    fn connect_to(&mut self, addr: Address<E>) {
        self.address_book.add(addr);
    }
} 

impl<E> EventReceiver<E> for CursiveContext<E>
where
    E: Clone,
{
    fn address(&self) -> Address<E> {
        self.mailbox.address()
    }
    fn handle_event(&mut self, ev: E) -> EventResult {
        self.address_book.send(ev.clone());
        true
    }
}

impl<E> CursiveContext<E>
where 
    E: Clone,
{
    fn pump_events(&mut self) {
        for ev in self.mailbox.read() {
            self.handle_event(ev);
        }
    }
}

