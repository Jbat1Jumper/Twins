extern crate midir;
extern crate mursten;
extern crate mursten_vulkan_backend;
extern crate nalgebra;
extern crate rand;

use mursten::{Application, Backend, Data, Renderer, Updater};
use std::error::Error;


use mursten_vulkan_backend::geometry::{Triangle, Vertex};
use mursten_vulkan_backend::VulkanBackend;
use nalgebra::*;

use updaters::time::{Clock, ClockUpdater, OnTick, Tick};
use keyboard::UpdateKeyboard;

pub fn main() {


    let backend = VulkanBackend::new();
    let mut scene = Scene::default();
    Application::new(backend)
        .add_updater(ClockUpdater::new())
        .add_updater(UpdateKeyboard::new())
        .add_renderer(Visual::new())
        .run(scene);
}


struct Scene {
    clock: Clock,
    paused: bool,
    keyboard: [u8; 128],
}

impl Scene {
    pub fn new() -> Self {
        Scene::default()
    }
}

impl Default for Scene {
    fn default() -> Self {
        Scene {
            clock: Clock::new(),
            paused: false,
            keyboard: [0; 128],
        }
    }
}

impl Data for Scene {}

impl OnTick for Scene {
    fn on_tick(&mut self, tick: Tick) {
        if !self.paused {
            self.clock += tick;
        }
    }
}


pub mod keyboard {
    use mursten::Updater;
    use midi::{MidiHandle, MidiMessage, listen_from_port};
    use std::{thread, time};
    use Scene;

    pub struct UpdateKeyboard {
        midi_handle: MidiHandle,
    }

    impl UpdateKeyboard {
        pub fn new() -> Self {
            Self {
                midi_handle: listen_from_port().unwrap(),
            }
        }
    }

    impl<B> Updater<B, Scene> for UpdateKeyboard {
        fn update(&mut self, _: &mut B, scene: &mut Scene) {
            for msg in self.midi_handle.get_messages() {
                // println!("{:?}", msg);
                match msg {
                    MidiMessage::NoteOn(key, vel) => {
                        scene.keyboard[key as usize] = vel;
                    }
                    MidiMessage::NoteOff(key, _) => {
                        scene.keyboard[key as usize] = 0;
                    },
                    _ => {}
                }
            }
            let keys = [
                " A ", " A#", " B ", " C ",
                " C#", " D ", " D#", " E ",
                " F ", " F#", " G ", " G#"];
            for (key, key_state) in scene.keyboard.iter().skip(12).take(48).enumerate() {
                let key = keys[(key + 3) % 12];
                print!("{}", if *key_state > 0 { key } else { "   " });
            }
            println!("");
            thread::sleep(time::Duration::from_millis(20));
        }
    }
}


struct Visual {}

impl Visual {
    pub fn new() -> Self {
        Visual {}
    }
}

fn ray(pos: Point2<f32>, rot: Rotation2<f32>, len: f32) -> Vec<Triangle> {

    // Transformaciones esteticas
    let scale = 0.04;
    let rot = {
        let rpos = rot * pos;
        Rotation3::rotation_between(&Vector3::new(pos.x, pos.y, 0.0), &Vector3::new(rpos.x, rpos.y, 0.0)).unwrap()
    };
    let pos = Point3::new(pos.x, pos.y, 0.0);
    let len = len.sqrt();

    let r  = Vertex::from( pos + Vector3::z() * len                                ).color(1.0, 0.0, 0.0, 1.0);
    let g  = Vertex::from( pos + rot * Vector3::new( 2.0 * len,  0.0, len) * scale ).color(0.0, 1.0, 0.0, 1.0);
    let b  = Vertex::from( pos + rot * Vector3::new( 4.0 * len,  0.0, len) * scale ).color(0.0, 0.0, 1.0, 1.0);
    let v1 = Vertex::from( pos + rot * Vector3::new(-1.0 * len,  0.4, len) * scale ).color(0.0, 0.0, 0.0, 0.0);
    let v2 = Vertex::from( pos + rot * Vector3::new( 1.0 * len,  0.4, len) * scale ).color(0.0, 0.0, 0.0, 0.0);
    let v3 = Vertex::from( pos + rot * Vector3::new( 3.0 * len,  0.4, len) * scale ).color(0.0, 0.0, 0.0, 0.0);
    let v4 = Vertex::from( pos + rot * Vector3::new( 5.0 * len,  0.4, len) * scale ).color(0.0, 0.0, 0.0, 0.0);
    let v5 = Vertex::from( pos + rot * Vector3::new(-1.0 * len, -0.4, len) * scale ).color(0.0, 0.0, 0.0, 0.0);
    let v6 = Vertex::from( pos + rot * Vector3::new( 1.0 * len, -0.4, len) * scale ).color(0.0, 0.0, 0.0, 0.0);
    let v7 = Vertex::from( pos + rot * Vector3::new( 3.0 * len, -0.4, len) * scale ).color(0.0, 0.0, 0.0, 0.0);
    let v8 = Vertex::from( pos + rot * Vector3::new( 5.0 * len, -0.4, len) * scale ).color(0.0, 0.0, 0.0, 0.0);


    vec!(
        Triangle::new( r, v1, v2),
        Triangle::new( r, v5, v1),
        Triangle::new( r, v6, v5),
        Triangle::new(v2,  g,  r),
        Triangle::new(v6,  r,  g),
        Triangle::new( g, v2, v3),
        Triangle::new( g, v7, v6),
        Triangle::new(v3,  b,  g),
        Triangle::new(v7,  g,  b),
        Triangle::new( b, v3, v4),
        Triangle::new( b, v4, v8),
        Triangle::new( b, v8, v7),
    )
}

impl Renderer<VulkanBackend, Scene> for Visual {
    fn render(&mut self, backend: &mut VulkanBackend, scene: &Scene) {
    }
}

mod updaters {

    pub mod time {
        use std::time::{SystemTime, UNIX_EPOCH, Duration};

        pub type Time = SystemTime;

        pub const CREATION_TIME: Time = UNIX_EPOCH;

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
                    system_time.duration_since(self.last_system_time).unwrap()
                } else {
                    Duration::new(0, 0)
                };

                let tick = Tick { system_time, delta };
                data.on_tick(tick);
                self.last_system_time = system_time;
            }
        }
    }
}

mod midi {
    use std::io::{stdin, stdout, Write};
    use std::sync::mpsc::{channel, Receiver};
    use midir::{MidiInput, MidiInputConnection, Ignore};

    pub struct MidiHandle {
        receiver: Receiver<MidiMessage>,
        midi_connection: MidiInputConnection<()>,
    }

    impl MidiHandle {
        pub fn get_messages(&self) -> Vec<MidiMessage> {
            self.receiver.try_iter().collect()
        }
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

    pub fn listen_from_port() -> Result<MidiHandle, ()> {
        println!("\n# Please connect to a MIDI input\n");
        let mut midi_in = MidiInput::new("midi_one input port").unwrap();
        midi_in.ignore(Ignore::None);
        
        println!("Available input ports:");
        for i in 0..midi_in.port_count() {
            println!("{}: {}", i, midi_in.port_name(i).unwrap());
        }
        print!("Please select input port: ");
        stdout().flush().unwrap();
        let in_port: usize = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            input.trim().parse().unwrap()
        };
        
        println!("\nOpening connection");
        let in_port_name = midi_in.port_name(in_port).unwrap();


        let (transmitter, receiver) = channel();

        let midi_connection = midi_in.connect(in_port, "midir-forward", move |stamp, message, _| {
            if let Some(message) = MidiMessage::from(message) {
                transmitter.send(message);
            }
        }, ()).unwrap();
        
        println!("Connection open, listening to '{}'", in_port_name);


        
        Ok( MidiHandle { receiver, midi_connection } )
    }

}

