extern crate midir;
extern crate mursten;


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

    impl<'a, B, D> Updater<B, D> for PropertyEditor
    where D: Data + GetProperties<'a> {
        fn update(&mut self, _: &mut B, data: &mut D) {
        }
    }

}

pub mod properties {
    use std::slice::Iter;

    pub trait GetProperties<'a> {
        fn properties(&'a mut self) -> Properties;
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
        fn new() -> Self {
            Self {
                properties: Vec::new(),
            }
        }
        fn add<T>(mut self, name: &'static str, reference: &'a mut T) -> Self
        where T: Clone + From<Value> + Into<Value> {
            self.properties.retain(|p| { p.name() != name });
            let property_reference = PropertyReference { name, reference };
            self.properties.push(Box::new(property_reference));
            self
        }
        fn iter(&self) -> Iter<Box<Property<'a> + 'a>> {
            self.properties.iter()
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
}
