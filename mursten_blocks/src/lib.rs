extern crate midir;
extern crate mursten;
extern crate rustyline;


pub mod time {
    use std::time::{SystemTime, UNIX_EPOCH, Duration};

    pub type Time = SystemTime;

    pub const CREATION_TIME: Time = UNIX_EPOCH;

    #[derive(Debug)]
    pub struct Clock {
        time: Time,
        delta: Duration,
        system_time: Time,
        system_delta: Duration,
    }

    impl Clock {
        pub fn new() -> Clock {
            Clock {
                time: CREATION_TIME,
                delta: Duration::new(0, 0),
                system_time: Time::now(),
                system_delta: Duration::new(0, 0),
            }
        }
        pub fn system_time(&self) -> Time {
            self.system_time
        }
        pub fn system_delta(&self) -> Duration {
            self.system_delta
        }
        pub fn time(&self) -> Time {
            self.time
        }
        pub fn delta(&self) -> Duration {
            self.delta
        }
    }

    impl Clock {
        pub fn system_time_in_sec(&self) -> f32 {
            let d = self.system_time.duration_since(CREATION_TIME).unwrap();
            d.as_secs() as f32 + d.subsec_millis() as f32 / 1000.0
        }
        pub fn time_in_sec(&self) -> f32 {
            let d = self.time.duration_since(CREATION_TIME).unwrap();
            d.as_secs() as f32 + d.subsec_millis() as f32 / 1000.0
        }
    }

    use std::ops::{Add, AddAssign};

    impl Add<Tick> for Clock {
        type Output = Clock;
        fn add(self, tick: Tick) -> Clock {
            Clock {
                system_delta: tick.system_time.duration_since(self.system_time).unwrap(),
                system_time: tick.system_time,
                time: self.time + tick.delta,
                delta: tick.delta,
            }
        }
    }

    impl AddAssign<Tick> for Clock {
        fn add_assign(&mut self, tick: Tick) {
            *self = Clock {
                system_delta: tick.system_time.duration_since(self.system_time).unwrap(),
                system_time: tick.system_time,
                time: self.time + tick.delta,
                delta: tick.delta,
            };
        }
    }

    #[derive(Debug)]
    pub struct Tick {
        system_time: Time,
        delta: Duration,
    }
    
    pub trait OnTick {
        fn on_tick(&mut self, tick: Tick);
    }

    pub struct ClockUpdater {
        last_system_time: Time,
    }

    impl ClockUpdater {
        pub fn new() -> ClockUpdater {
            ClockUpdater {
                last_system_time: CREATION_TIME,
            }
        }
    }

    use mursten::{Updater, Data};

    impl<B, D> Updater<B, D> for ClockUpdater 
    where D: Data + OnTick {
        fn update(&mut self, _: &mut B, data: &mut D) {
            let system_time = SystemTime::now();
            let delta = if self.last_system_time == CREATION_TIME {
                Duration::new(0, 0)
            } else {
                system_time.duration_since(self.last_system_time).unwrap()
            };

            let tick = Tick { system_time, delta };
            data.on_tick(tick);
            self.last_system_time = system_time;
        }
    }
}

pub mod midi {

    use std::io::{stdin, stdout, Write};
    use std::sync::mpsc::{channel, Receiver};
    use mursten::{Updater, Data};
    use midir::{MidiInput, MidiInputConnection, Ignore};

    struct MidiHandle {
        receiver: Receiver<MidiMessage>,
        midi_connection: MidiInputConnection<()>,
    }

    impl MidiHandle {
        fn get_messages(&self) -> Vec<MidiMessage> {
            self.receiver.try_iter().collect()
        }
    }

    pub trait OnMidiMessage {
        fn on_midi_message(&mut self, message: MidiMessage);
    }

    #[derive(Debug)]
    pub enum MidiMessage {
        NoteOff(u8, u8),
        NoteOn(u8, u8),
        KeyPressure(u8, u8),
        ControlChange(u8, u8),
        ProgramChange(u8),
        ChannelPressure(u8),
        PitchBendChange(u16),
        Start,
        Stop,
    }

    impl MidiMessage {
        fn from(bytes: &[u8]) -> Option<MidiMessage> {
            let mut bytes = bytes.iter().cloned();
            let b1 = bytes.next()?;
            let msg = match (b1 & 0b1111_0000) >> 4 {
                0b1000 => MidiMessage::NoteOff(bytes.next()?, bytes.next()?),
                0b1001 => {
                    let key = bytes.next()?;
                    let vel = bytes.next()?;
                    if vel > 0 {
                        MidiMessage::NoteOn(key, vel)
                    } else {
                        MidiMessage::NoteOff(key, vel)
                    }
                }
                0b1010 => MidiMessage::KeyPressure(bytes.next()?, bytes.next()?),
                0b1011 => MidiMessage::ControlChange(bytes.next()?, bytes.next()?),
                0b1100 => MidiMessage::ProgramChange(bytes.next()?),
                0b1101 => MidiMessage::ChannelPressure(bytes.next()?),
                0b1110 => {
                    let l = bytes.next()? as u16;
                    let h = bytes.next()? as u16;
                    let value = h * 128 + l;
                    MidiMessage::PitchBendChange(value)
                },
                0b1111 => match b1 {
                    0xFA => MidiMessage::Start,
                    0xFC => MidiMessage::Stop,
                    _ => { return None; },
                },
                _ => { return None; },
            };
            Some(msg)
        }
    }

    pub struct MidiUpdater {
        midi_handle: MidiHandle,
    }

    impl MidiUpdater {
        pub fn prompt() -> Self {
            let midi_in = create_midi_input();
            let name = prompt_port(&midi_in);
            Self {
                midi_handle: listen_from_port(midi_in, &name).unwrap(),
            }
        }
        pub fn new(name: &str) -> Self {
            let midi_in = create_midi_input();
            Self {
                midi_handle: listen_from_port(midi_in, name).unwrap(),
            }
        }
    }

    impl<B, D> Updater<B, D> for MidiUpdater
    where D: Data + OnMidiMessage {
        fn update(&mut self, _: &mut B, data: &mut D) {
            for msg in self.midi_handle.get_messages() {
                data.on_midi_message(msg);
            }
        }
    }

    fn create_midi_input() -> MidiInput {
        let mut midi_in = MidiInput::new("midi_one input port").unwrap();
        midi_in.ignore(Ignore::None);
        midi_in
    }

    fn prompt_port(midi_in: &MidiInput) -> String {
        println!("\n# Please connect to a MIDI input\n#");
        println!("# Available input ports:");
        for i in 0..midi_in.port_count() {
            println!("#   {}: {}", i, midi_in.port_name(i).unwrap());
        }
        print!("# Please select input port: ");
        stdout().flush().unwrap();
        let in_port: usize = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            input.trim().parse().unwrap()
        };
        midi_in.port_name(in_port).unwrap()
    }

    fn listen_from_port(midi_in: MidiInput, name: &str) -> Result<MidiHandle, ()> {

        let (transmitter, receiver) = channel();

        let port_index = |midi_in: &MidiInput, name| {
            let mut res = None;
            for i in 0..midi_in.port_count() {
                if name == midi_in.port_name(i).unwrap() {
                    res = Some(i);
                }
            }
            res
        };

        println!("# \nOpening connection");
        match port_index(&midi_in, name) {
            Some(port_index) => {
                let midi_connection = midi_in.connect(port_index, "midir-forward", move |stamp, message, _| {
                    if let Some(message) = MidiMessage::from(message) {
                        transmitter.send(message);
                    }
                }, ()).unwrap();
                println!("# Connection open, listening to '{}'", name);
                Ok( MidiHandle { receiver, midi_connection } )
            },
            None => {
                println!("# No port found by the name of '{}'", name);
                Err(())
            }
        }
    }

}

pub mod property_editor {
    use mursten::{Updater, Data};
    use properties::{Property, GetProperties};
    
    pub struct PropertyEditor {
    }

    impl<B, D> Updater<B, D> for PropertyEditor
    where D: Data + GetProperties {
        fn update(&mut self, _: &mut B, data: &mut D) {
        }
    }
}

pub mod repl {
    use std::time::Duration;
    use mursten::{Backend, Updater, Data};
    use properties::{GetProperties, Property, Value};
    use rustyline::Editor;
    use rustyline::error::ReadlineError;
    use std::sync::mpsc::{channel, Sender, Receiver, RecvTimeoutError};

    pub fn create_repl() -> (Client, Server) {
        let (client_tx, server_rx) = channel();
        let (server_tx, client_rx) = channel();
        (
            Client { tx: client_tx, rx: client_rx },
            Server { tx: server_tx, rx: server_rx }
        )
    }

    pub struct Client {
        tx: Sender<Request>,
        rx: Receiver<Response>,
    }

    pub struct Server {
        rx: Receiver<Request>,
        tx: Sender<Response>,
    }

    pub enum Request {
        Set(String, Value),
        Get(String),
        Exit,
    }

    pub enum Response {
        Ok,
        PropertyNotFound,
        Value(Value),
        ExitSuccessful,
    }

    impl Client {
        pub fn run(self) {
            let mut rl = Editor::<()>::new();
            loop {
                let res = rl.readline(">> ")
                    .map_err(|err| ErrKind::Readline(err))
                    .and_then(|line| {
                        let mut words = line.split_whitespace();
                        match words.next() {
                            Some("set") => match words.next() {
                                Some(key) => match parse_value(words.next()) {
                                    Some(value) => {
                                        self.tx.send(Request::Set(key.to_string(), value));
                                        self.rx.recv_timeout(Duration::from_secs(10))
                                            .map_err(|err| {
                                                match err {
                                                    RecvTimeoutError::Timeout => ErrKind::Response("No response after 10 seconds."),
                                                    RecvTimeoutError::Disconnected => ErrKind::Response("No response, disconnected from server."),
                                                }
                                            })
                                            .and_then(|res| {
                                                match res {
                                                    Response::Ok => Ok(()),
                                                    Response::PropertyNotFound => Err(ErrKind::Response("Property not found.")),
                                                    _ => Err(ErrKind::Response("Unknown response."))
                                                }
                                            })
                                    },
                                    None => Err(ErrKind::Usage("No value provided.")),
                                },
                                None => Err(ErrKind::Usage("No property name provided.")),
                            },
                            Some("get") => match words.next() {
                                Some(key) => {
                                    self.tx.send(Request::Get(key.to_string()));
                                    self.rx.recv_timeout(Duration::from_secs(10))
                                        .map_err(|err| {
                                            match err {
                                                RecvTimeoutError::Timeout => ErrKind::Response("No response after 10 seconds."),
                                                RecvTimeoutError::Disconnected => ErrKind::Response("No response, disconnected from server."),
                                            }
                                        })
                                        .and_then(|res| {
                                            match res {
                                                Response::Value(value) => {
                                                    println!("{:?}", value);
                                                    Ok(())
                                                },
                                                Response::PropertyNotFound => Err(ErrKind::Response("Property not found.")),
                                                _ => Err(ErrKind::Response("Unknown response."))
                                            }
                                        })
                                },
                                None => Err(ErrKind::Usage("No property name provided.")),
                            },
                            Some("exit") => {
                                self.tx.send(Request::Exit);
                                self.rx.recv_timeout(Duration::from_secs(10))
                                    .map_err(|err| {
                                        match err {
                                            RecvTimeoutError::Timeout => ErrKind::Response("No response after 10 seconds. Use `exit!` to force exit."),
                                            RecvTimeoutError::Disconnected => ErrKind::Response("No response, disconnected from server. Use `exit!` to force exit."),
                                        }
                                    })
                                    .and_then(|res| {
                                        match res {
                                            Response::ExitSuccessful => Err(ErrKind::Exit),
                                            _ => Err(ErrKind::Response("Wrong response. Use `exit!` to force exit."))
                                        }
                                    })
                            }
                            Some("exit!") => Err(ErrKind::Exit),
                            Some(_) => Err(ErrKind::Usage("Unknown command.")),
                            None => Ok(()),
                        }
                    });

                match res {
                    Ok(_) => { },
                    Err(err) => match err {
                        ErrKind::Readline(err) => {
                            println!("{:?}!", err);
                            break
                        },
                        ErrKind::Exit => break,
                        ErrKind::Response(msg) | ErrKind::Usage(msg) => println!("{}", msg),
                    }
                }
            }
            enum ErrKind {
                Usage(&'static str),
                Response(&'static str),
                Readline(ReadlineError),
                Exit,
            }
        }
    }

    fn parse_value(s: Option<&str>) -> Option<Value> {
        s.map_or(None, |s| {
            if let Ok(b) = s.parse() {
                Some(Value::Bool(b))
            } else if let Ok(f) = s.parse() {
                Some(Value::Float(f))
            } else if let Ok(i) = s.parse() {
                Some(Value::Integer(i))
            } else {
                None
            }
        })
    }

    impl<B, D> Updater<B, D> for Server
    where D: Data + GetProperties,
          B: Backend<D> {
        fn update(&mut self, backend: &mut B, data: &mut D) {
            let mut ps = data.properties();
            for request in self.rx.try_iter() {
                let response = match request {
                    Request::Set(k, v) => match ps.iter_mut().find(|p| p.name() == k) {
                        Some(p) => {
                            p.set(v);
                            Response::Ok
                        },
                        None => Response::PropertyNotFound,
                    },
                    Request::Get(k) => match ps.iter().find(|p| p.name() == k) {
                        Some(p) => Response::Value(p.get()),
                        None => Response::PropertyNotFound,
                    },
                    Request::Exit => {
                        self.tx.send(Response::ExitSuccessful);
                        backend.quit();
                        Response::ExitSuccessful
                    }
                };
                self.tx.send(response);
            }
        }
    }
}

pub mod properties {
    use std::slice::{Iter, IterMut};

    pub trait GetProperties {
        fn properties<'a>(&'a mut self) -> Properties;
    }

    pub struct Properties<'a> {
        properties: Vec<Box<Property<'a> + 'a>>,
    }

    pub trait Property<'a> {
        fn name(&self) -> &'static str;
        fn set(&mut self, value: Value);
        fn get(&self) -> Value;
    }

    #[derive(Debug)]
    pub enum Value {
        Float(f32),
        Integer(i32),
        Bool(bool),
    }

    impl<'a> Properties<'a> { 
        pub fn new() -> Self {
            Self {
                properties: Vec::new(),
            }
        }
        pub fn add<T>(mut self, name: &'static str, reference: &'a mut T) -> Self
        where T: Clone + From<Value> + Into<Value> {
            self.properties.retain(|p| { p.name() != name });
            let property_reference = PropertyReference { name, reference };
            self.properties.push(Box::new(property_reference));
            self
        }
        pub fn iter(&self) -> Iter<Box<Property<'a> + 'a>> {
            self.properties.iter()
        }
        pub fn iter_mut(&mut self) -> IterMut<Box<Property<'a> + 'a>> {
            self.properties.iter_mut()
        }
    }

    struct PropertyReference<'a, T>
    where T: 'a {
        name: &'static str,
        reference: &'a mut T,
    }

    impl<'a, T> Property<'a> for PropertyReference<'a, T>
    where T: Clone + From<Value> + Into<Value> {
        fn name(&self) -> &'static str {
            self.name
        }
        fn set(&mut self, value: Value) {
            *(self.reference) = value.into();
        }
        fn get(&self) -> Value {
            (*(self.reference)).clone().into()
        }
    }

    impl From<Value> for f32 {
        fn from(v: Value) -> f32 {
            match v {
                Value::Float(f) => f,
                v => panic!("Invalid cast from {:?} to f32", v),
            }
        }
    }

    impl Into<Value> for f32 {
        fn into(self) -> Value {
            Value::Float(self)
        }
    }

    impl From<Value> for bool {
        fn from(v: Value) -> bool {
            match v {
                Value::Bool(b) => b,
                v => panic!("Invalid cast from {:?} to bool", v),
            }
        }
    }

    impl Into<Value> for bool {
        fn into(self) -> Value {
            Value::Bool(self)
        }
    }

    impl From<Value> for i32 {
        fn from(v: Value) -> i32 {
            match v {
                Value::Integer(i) => i,
                v => panic!("Invalid cast from {:?} to i32", v),
            }
        }
    }

    impl Into<Value> for i32 {
        fn into(self) -> Value {
            Value::Integer(self)
        }
    }
}
