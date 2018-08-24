extern crate mursten;
extern crate mursten_blocks;
extern crate mursten_vulkan_backend;
extern crate nalgebra;

use mursten::{Application, Backend, Data, Renderer, Updater};
use mursten_blocks::midi::{MidiMessage, MidiUpdater, OnMidiMessage};
use mursten_blocks::time::{Clock, ClockUpdater, OnTick, Tick};
use mursten_vulkan_backend::geometry::{Mesh, Triangle, Vertex};
use mursten_vulkan_backend::{Constants, VulkanBackend};

use nalgebra::*;


pub fn main() {
    let backend = VulkanBackend::new();
    let scene = Scene::new();
    Application::new(backend)
        .add_updater(ClockUpdater::new())
        .add_updater(MidiUpdater::new())
        .add_renderer(Visual::new())
        .run(scene);
}

struct Scene {
    clock: Clock,
    paused: bool,
    active_track: Track,
    w: f32,
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug)]
enum Track {
    W,
    X,
    Y,
    Z,
}

impl Data for Scene { }

impl Scene {
    pub fn new() -> Self {
        Scene {
            clock: Clock::new(),
            paused: false,
            active_track: Track::W,
            w: 1.0,
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
}

impl OnTick for Scene {
    fn on_tick(&mut self, tick: Tick) {
        if !self.paused {
            self.clock += tick;
        }
        std::thread::sleep_ms(20);
    }
}

impl OnMidiMessage for Scene {
    fn on_midi_message(&mut self, msg: MidiMessage) {
        match msg {
            MidiMessage::NoteOn(28, _) => {
                self.active_track = Track::W;
            },
            MidiMessage::NoteOn(29, _) => {
                self.active_track = Track::X;
            },
            MidiMessage::NoteOn(30, _) => {
                self.active_track = Track::Y;
            },
            MidiMessage::NoteOn(31, _) => {
                self.active_track = Track::Z;
            },
            MidiMessage::PitchBendChange(amount) => {
                let value = amount as f32 / 16383.0;
                println!("{:?}: {}", self.active_track, value);
                match self.active_track {
                    Track::W => self.w = value,
                    Track::X => self.x = value,
                    Track::Y => self.y = value,
                    Track::Z => self.z = value,
                }
            },
            msg => { println!("{:?}", msg); },
        }
    }
}

struct Visual { }

impl Visual {
    pub fn new() -> Self {
        Visual { }
    }
}

fn tesselated_rectangle(divisions: u32) -> Mesh {

    let quads = divisions + 1;

    let upper_triangles = (0..quads).map(|i| {
        let z_0 = i as f32 / quads as f32;
        let z_1 = (i + 1) as f32 / quads as f32;
        Triangle {
            v1: Point3::new(-1.0, 0.0, -z_0).into(),
            v2: Point3::new(-1.0, 0.0, -z_1).into(),
            v3: Point3::new( 1.0, 0.0, -z_0).into(),
        }
    });

    let lower_triangles = (0..quads).map(|i| {
        let z_0 = i as f32 / quads as f32;
        let z_1 = (i + 1) as f32 / quads as f32;
        Triangle {
            v1: Point3::new(-1.0, 0.0, -z_1).into(),
            v2: Point3::new( 1.0, 0.0, -z_1).into(),
            v3: Point3::new( 1.0, 0.0, -z_0).into(),
        }
    });

    Mesh {
        // triangles: upper_triangles.chain(lower_triangles).collect(),
        triangles: vec![Triangle {
            v1: Point3::new(-1.0, 0.0, 1.0).into(),
            v2: Point3::new( 1.0, 0.0, -1.0).into(),
            v3: Point3::new( 1.0, 0.0, 1.0).into(),
        }],
        transform: Transform3::identity(),
    }
}

impl Renderer<VulkanBackend, Scene> for Visual {
    fn render(&mut self, backend: &mut VulkanBackend, scene: &Scene) {

        // Units are in centimeters
        //let eye = Point3::new(40.0, 40.0, 40.0);
        let eye = Point3::new(
            scene.x * 400.0 - 200.0, 
            scene.y * 400.0 - 200.0, 
            scene.z * 400.0 - 200.0,
        );
        let target = Point3::new(0.0, 0.0, 0.0);
        //let up = Vector3::y();

        backend.set_constants(Constants {
            projection: Perspective3::new(1.0, 1.57, 1.0, 900.0).to_homogeneous(),
            view: Matrix4::from_euler_angles(0.0, scene.w * 6.0 - 3.0, 0.0) * Matrix4::new_translation(&eye.coords),
            ..Constants::default()
        });

        // Reference Unit Cube
        let reference = Mesh {
            transform: Transform3::identity() * Similarity3::from_scaling(scene.w * 20.0 + 0.001),
            triangles: vec![
                // +Z
                Triangle::new(
                    Vertex::at(Point3::new(-1.0, -1.0,  1.0)).color(0.0, 0.0, 1.0, 1.0),
                    Vertex::at(Point3::new(-1.0,  1.0,  1.0)).color(0.0, 0.0, 1.0, 1.0),
                    Vertex::at(Point3::new( 1.0, -1.0,  1.0)).color(0.0, 0.0, 1.0, 1.0),
                ),
                Triangle::new(
                    Vertex::at(Point3::new( 1.0, -1.0,  1.0)).color(0.0, 0.0, 1.0, 1.0),
                    Vertex::at(Point3::new(-1.0,  1.0,  1.0)).color(0.0, 0.0, 1.0, 1.0),
                    Vertex::at(Point3::new( 1.0,  1.0,  1.0)).color(0.0, 0.0, 1.0, 1.0),
                ),
                // -Z
                Triangle::new(
                    Vertex::at(Point3::new(-1.0, -1.0, -1.0)).color(1.0, 1.0, 0.0, 1.0),
                    Vertex::at(Point3::new(-1.0,  1.0, -1.0)).color(1.0, 1.0, 0.0, 1.0),
                    Vertex::at(Point3::new( 1.0, -1.0, -1.0)).color(1.0, 1.0, 0.0, 1.0),
                ),
                Triangle::new(
                    Vertex::at(Point3::new( 1.0, -1.0, -1.0)).color(1.0, 1.0, 0.0, 1.0),
                    Vertex::at(Point3::new(-1.0,  1.0, -1.0)).color(1.0, 1.0, 0.0, 1.0),
                    Vertex::at(Point3::new( 1.0,  1.0, -1.0)).color(1.0, 1.0, 0.0, 1.0),
                ),
                // +Y
                Triangle::new(
                    Vertex::at(Point3::new(-1.0,  1.0, -1.0)).color(0.0, 1.0, 0.0, 1.0),
                    Vertex::at(Point3::new(-1.0,  1.0,  1.0)).color(0.0, 1.0, 0.0, 1.0),
                    Vertex::at(Point3::new( 1.0,  1.0, -1.0)).color(0.0, 1.0, 0.0, 1.0),
                ),
                Triangle::new(
                    Vertex::at(Point3::new( 1.0,  1.0, -1.0)).color(0.0, 1.0, 0.0, 1.0),
                    Vertex::at(Point3::new(-1.0,  1.0,  1.0)).color(0.0, 1.0, 0.0, 1.0),
                    Vertex::at(Point3::new( 1.0,  1.0,  1.0)).color(0.0, 1.0, 0.0, 1.0),
                ),
                // -Y
                Triangle::new(
                    Vertex::at(Point3::new(-1.0, -1.0, -1.0)).color(1.0, 0.0, 1.0, 1.0),
                    Vertex::at(Point3::new(-1.0, -1.0,  1.0)).color(1.0, 0.0, 1.0, 1.0),
                    Vertex::at(Point3::new( 1.0, -1.0, -1.0)).color(1.0, 0.0, 1.0, 1.0),
                ),
                Triangle::new(
                    Vertex::at(Point3::new( 1.0, -1.0, -1.0)).color(1.0, 0.0, 1.0, 1.0),
                    Vertex::at(Point3::new(-1.0, -1.0,  1.0)).color(1.0, 0.0, 1.0, 1.0),
                    Vertex::at(Point3::new( 1.0, -1.0,  1.0)).color(1.0, 0.0, 1.0, 1.0),
                ),
                // +X
                Triangle::new(
                    Vertex::at(Point3::new( 1.0, -1.0, -1.0)).color(1.0, 0.0, 0.0, 1.0),
                    Vertex::at(Point3::new( 1.0, -1.0,  1.0)).color(1.0, 0.0, 0.0, 1.0),
                    Vertex::at(Point3::new( 1.0,  1.0, -1.0)).color(1.0, 0.0, 0.0, 1.0),
                ),
                Triangle::new(
                    Vertex::at(Point3::new( 1.0,  1.0, -1.0)).color(1.0, 0.0, 0.0, 1.0),
                    Vertex::at(Point3::new( 1.0, -1.0,  1.0)).color(1.0, 0.0, 0.0, 1.0),
                    Vertex::at(Point3::new( 1.0,  1.0,  1.0)).color(1.0, 0.0, 0.0, 1.0),
                ),
                // -X
                Triangle::new(
                    Vertex::at(Point3::new(-1.0, -1.0, -1.0)).color(0.0, 1.0, 1.0, 1.0),
                    Vertex::at(Point3::new(-1.0, -1.0,  1.0)).color(0.0, 1.0, 1.0, 1.0),
                    Vertex::at(Point3::new(-1.0,  1.0, -1.0)).color(0.0, 1.0, 1.0, 1.0),
                ),
                Triangle::new(
                    Vertex::at(Point3::new(-1.0,  1.0, -1.0)).color(0.0, 1.0, 1.0, 1.0),
                    Vertex::at(Point3::new(-1.0, -1.0,  1.0)).color(0.0, 1.0, 1.0, 1.0),
                    Vertex::at(Point3::new(-1.0,  1.0,  1.0)).color(0.0, 1.0, 1.0, 1.0),
                ),
            ],
        };

        // Floor
        let floor = Mesh {
            transform: Transform3::identity(),
            triangles: vec![
                Triangle::new(
                    Vertex::at(Point3::new(0.0, 0.0, 0.0)).color(0.0, 1.0, 0.0, 1.0),
                    Vertex::at(Point3::new(120.0, 0.0, -80.0)),
                    Vertex::at(Point3::new(120.0, 0.0, 0.0)),
                ),
                Triangle::new(
                    Vertex::at(Point3::new(0.0, 0.0, 0.0)).color(0.0, 1.0, 0.0, 1.0),
                    Point3::new(0.0, 0.0, -80.0).into(),
                    Point3::new(120.0, 0.0, -80.0).into(),
                ),
            ],
        };

        // Plot
        // let plot = Mesh {
        //     transform: Transform3::identity() * Translation3::from_vector(target.coords),
        //     ..tesselated_rectangle(32)
        // };

        backend.queue_render(reference);
        backend.queue_render(floor);
        //backend.queue_render(plot);

    }
}

