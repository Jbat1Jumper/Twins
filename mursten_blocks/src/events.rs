use mursten::{Backend, Data, Updater};

use super::events::transport::*;


pub type EventResult = bool;

pub trait EventEmitter<E> {
    fn connect_to(&mut self, Address<E>);
}

pub trait EventReceiver<E> {
    fn address(&self) -> Address<E>;
    fn handle_event(&mut self, E) -> EventResult;
}

pub struct SimpleEventReceiver<E, H> 
where 
    H: Fn(E) -> EventResult
{
    mailbox: Mailbox<E>,
    handler: H,
}

impl<E, H> SimpleEventReceiver<E, H>
where 
    H: Fn(E) -> EventResult
{
    pub fn new(name: &'static str, h: H) -> Self {
        Self {
            handler: h,
            mailbox: Mailbox::new(name),
        }
    }
}

impl<E, H> EventReceiver<E> for SimpleEventReceiver<E, H>
where 
    H: Fn(E) -> EventResult
{
    fn address(&self) -> Address<E> {
        self.mailbox.address()
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
        for ev in self.mailbox.read() {
            self.handle_event(ev);
        }
    }
}

pub mod transport {
    use std::sync::mpsc::{channel, Receiver, Sender};

    pub struct Mailbox<E> {
        name: &'static str,
        receiver: Receiver<E>,
        sender: Sender<E>,
    }

    #[derive(Clone)]
    pub struct Address<E> {
        name: &'static str,
        sender: Sender<E>,
    }

    impl<E> Address<E> {
        fn new(name: &'static str, sender: Sender<E>) -> Self {
            Self { name, sender }
        }
        pub fn send(&self, ev: E) {
            self.sender.send(ev)
                .expect(&format!("Failed to send event to {}", self.name));
        }
    }

    #[derive(Clone)]
    pub struct AddressBook<E> {
        addresses: Vec<Address<E>>,
    }

    impl<E> AddressBook<E>
    where 
        E: Clone
    {
        pub fn new() -> Self {
            Self { addresses: Vec::new() }
        }
        pub fn add(&mut self, a: Address<E>) {
            self.addresses.push(a);
        }
        pub fn send(&self, ev: E) {
            for address in self.addresses.iter() {
                address.send(ev.clone());
            }
        }
    }

    impl<E> Mailbox<E> {
        pub fn new(name: &'static str) -> Self {
            let (sender, receiver) = channel();
            Self { name, sender, receiver }
        }
        pub fn address(&self) -> Address<E> {
            Address::new(self.name, self.sender.clone())
        }
        pub fn read(&mut self) -> Vec<E> {
            self.receiver.try_iter().collect()
        }
    }
}

