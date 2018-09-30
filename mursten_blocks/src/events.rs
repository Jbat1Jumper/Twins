use mursten::{Backend, Data, Updater};
use std::sync::mpsc::{channel, Receiver, Sender};

pub type EventResult = bool;

pub trait EventEmitter<E> {
    fn connect_to(&mut self, Sender<E>);
}

pub trait EventReceiver<E> {
    fn input(&mut self) -> Sender<E>;
    fn handle_event(&mut self, E) -> EventResult;
}

pub struct SimpleEventReceiver<E, H> 
where 
    H: Fn(E) -> EventResult
{
    receiver: Receiver<E>,
    sender: Sender<E>,
    handler: H,
}

impl<E, H> SimpleEventReceiver<E, H>
where 
    H: Fn(E) -> EventResult
{
    pub fn new(h: H) -> Self {
        let (s, r) = channel();
        Self {
            handler: h,
            sender: s,
            receiver: r,
        }
    }
}

impl<E, H> EventReceiver<E> for SimpleEventReceiver<E, H>
where 
    H: Fn(E) -> EventResult
{
    fn input(&mut self) -> Sender<E> {
        self.sender.clone()
    }
    fn handle_event(&mut self, ev: E) -> EventResult {
        (self.handler)(ev)
    }
}

impl<B, D, E, H> Updater<B, D> for SimpleEventReceiver<E, H>
where
    H: Fn(E) -> EventResult,
    D: Data,
{
    fn update(&mut self, _backend: &mut B, _data: &mut D) {
        let events: Vec<E> = self.receiver.try_iter().collect();
        for ev in events {
            self.handle_event(ev);
        }
    }
}
