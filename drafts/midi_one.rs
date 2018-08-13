extern crate midir;
extern crate mursten;
extern crate mursten_vulkan_backend;
extern crate nalgebra;
extern crate rand;

use mursten::{Application, Backend, Data, Renderer, Updater};
use std::error::Error;


use mursten_vulkan_backend::geometry::{Triangle, Vertex};
use mursten_vulkan_backend::{Constants, VulkanBackend};
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
                " F ", " F#", " G ", " G#",
            ];
            for (key, key_state) in scene.keyboard.iter().skip(12).take(48).enumerate() {
                let key = keys[(key + 3) % 12];
                // print!("{}", if *key_state > 0 { key } else { "   " });
            }
            // println!("");
            thread::sleep(time::Duration::from_millis(20));
        }
    }
}


struct Visual {
    last_keyboard: Vec<u8>,
}

impl Visual {
    pub fn new() -> Self {
        Visual {
            last_keyboard: (0..36).collect(),
        }
    }
}

fn interpolate(new: Vec<u8>, old: Vec<u8>) -> Vec<u8> {
    new.iter().cloned().zip(old.iter().cloned()).map(|(new, old)| {
        if new > old {
            new
        } else {
            (new + old) / 2
        }
    }).collect()
}

fn spiral_points(keyboard: Vec<u8>) -> Vec<Vertex> {
    let len = keyboard.len();
    keyboard.iter().rev().enumerate().map(|(key, vel)| {
        let rotation = Rotation2::new(f32::two_pi()/12.0).powf(key as f32);
        let len = (key + 1) as f32 / (1 + len) as f32;
        let strength = *vel as f32 / 127.0;
        let pressed = strength * 0.2;
        let pos = rotation * Point2::new(0.0, len + pressed);
        let v = Vertex::at(pos.x, pos.y, 0.0);
        v.color(0.0, 1.0 - 0.8 * strength, 0.3 + 0.7 * strength, 1.0)
    }).collect()
}

impl Renderer<VulkanBackend, Scene> for Visual {
    fn render(&mut self, backend: &mut VulkanBackend, scene: &Scene) {
        let keyboard = interpolate(scene.keyboard.iter().skip(24).take(36).cloned().collect(), self.last_keyboard.clone());
        self.last_keyboard = keyboard.clone();
        let points = spiral_points(keyboard);
        
        let triangles: Vec<Triangle> = points.iter().cloned().skip(1).zip(points.iter().cloned()).map(|(a, b)| {
            let c = Vertex::at(0.0, 0.0, 1.0).color(1.0, 0.0, 0.2, 1.0);
            Triangle::new(a, b, c)
        }).collect();

        {
            let t = scene.clock.time_in_sec();
            let zoom = t.sin() * 0.4 + 0.5 + (t * 200.0 * t.sin()).sin() * 0.1;
            backend.set_constants(Constants { zoom });
        }
        
        backend.queue_render(vec![
            Triangle::new(
                Vertex::at(1.0, 1.0, 0.0).color(0.0, 0.0, 1.0, 1.0),
                Vertex::at(1.0, 0.9, 0.2).color(0.0, 0.0, 1.0, 1.0),
                Vertex::at(0.9, 1.0, 0.2).color(0.0, 0.0, 1.0, 1.0),
            ),
            Triangle::new(
                Vertex::at(1.0, 1.0, 0.1).color(1.0, 0.0, 0.0, 1.0),
                Vertex::at(1.0, 0.9, 0.1).color(1.0, 0.0, 0.0, 1.0),
                Vertex::at(0.9, 1.0, 0.1).color(1.0, 0.0, 0.0, 1.0),
            ),
        ]);
        backend.queue_render(triangles.into_iter().collect());

    }
}

mod updaters {
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

