extern crate mursten;
extern crate mursten_blocks;
extern crate mursten_vulkan_backend;
extern crate nalgebra;
extern crate rand;

use mursten::{Application, Backend, Data};
use mursten_blocks::geometry::{Mesh, Triangle, Vertex};
use mursten_blocks::camera::{Camera, CameraUpdater, GetCamera};
use mursten_blocks::time::{Clock, ClockUpdater, OnTick, Tick};
use mursten_blocks::mesh_renderer::{GetMeshes, IntoMesh, MeshRenderer};
use mursten_vulkan_backend::VulkanBackend;
use std::time::Duration;

use nalgebra::*;


pub fn main() {
    let backend = VulkanBackend::new();
    let scene = Scene::new();
    Application::new(backend)
        .add_updater(ClockUpdater::new())
        .add_updater(CameraUpdater::new())
        .add_renderer(MeshRenderer::new())
        .run(scene);
}

struct Scene {
    clock: Clock,
    paused: bool,
    next_change: Duration,
    player: Player,
    floor: Vec<Platform>,
    skybox: Skybox,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            clock: Clock::new(),
            paused: false,
            next_change: Duration::from_secs(1),
            player: Player::new(Point3::origin()),
            floor: {
                let mut v = Vec::new();
                for z in -10..10 {
                    for x in -10..10 {
                        v.push(Platform::new(Point3::new(x as f32, 0.0, z as f32)));
                    }
                }
                v
            },
            skybox: Skybox::new(),
        }
    }
}

struct Player {
    camera: Camera,
    height: f32,
    transform: Matrix4<f32>,
}

impl Player {
    pub fn new(position: Point3<f32>) -> Self {
        Player {
            camera: Camera::perspective(),
            height: 1.7,
            transform: Matrix4::new_translation(&position.coords),
        }
    }
}

struct Platform {
    position: Point3<f32>,
    target_height: f32,
    color: PlatformColor,
}

enum PlatformColor {
    Red(bool),
    Green(bool),
    Blue(bool),
}


impl Platform {
    pub fn new(position: Point3<f32>) -> Self {
        Platform {
            position,
            target_height: 0.0,
            color: PlatformColor::Green(false),
        }
    }

    fn set_vertex_color(&self, v: Vertex) -> Vertex {
        use PlatformColor::*;
        match self.color {
            Red(bright) => v.color(if bright { 0.8 } else { 0.3 }, 0.1, 0.1, 1.0),
            Green(bright) => v.color(0.1, if bright { 0.8 } else { 0.3 }, 0.1, 1.0),
            Blue(bright) => v.color(0.1, 0.1, if bright { 0.8 } else { 0.3 }, 1.0),
        }
    }
}

impl IntoMesh for Platform {
    fn transform(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&self.position.coords)
    }
    fn mesh(&self) -> Mesh {
        let v1 = Vertex::at(Point3::new(-0.5, 0.0, -0.5));
        let v2 = Vertex::at(Point3::new(-0.5, 0.0,  0.5));
        let v3 = Vertex::at(Point3::new( 0.5, 0.0,  0.5));
        let v4 = Vertex::at(Point3::new( 0.5, 0.0, -0.5));
        let v1 = self.set_vertex_color(v1);
        let v2 = self.set_vertex_color(v2);
        let v3 = self.set_vertex_color(v3);
        let v4 = self.set_vertex_color(v4);

        Mesh {
            triangles: vec![
                Triangle::new(v1, v2, v3),
                Triangle::new(v1, v3, v4),
            ],
        }
    }
}

struct Skybox {
}

impl Skybox {
    pub fn new() -> Self {
        Skybox {
        }
    }
}


impl Data for Scene {}

impl OnTick for Scene {
    fn on_tick(&mut self, tick: Tick) {
        use PlatformColor::*;
        if !self.paused {
            self.clock += tick;
        }
        std::thread::sleep_ms(20);

        self.player.transform *= Matrix4::from_euler_angles(0.0, 0.1 * self.clock.delta_as_sec(), 0.0);

        if self.clock.delta() >= self.next_change {
            self.next_change = Duration::from_secs(1);
            {
                use rand::distributions::normal::Normal;
                use rand::distributions::IndependentSample;
                let normal = Normal::new(0.2, 0.2);
                let mut rng = rand::thread_rng();
                for platform in self.floor.iter_mut() {
                    let ph = normal.ind_sample(&mut rng) as f32;
                    platform.target_height = ph;
                    if ph >= 0.3 {
                        platform.color = Red(ph > 0.36);
                    } else if ph >= 0.15 {
                        platform.color = Green(ph > 0.2);
                    } else {
                        platform.color = Blue(ph > 0.05);
                    }
                }
            }
        } else {
            self.next_change -= self.clock.delta();
        }

        for platform in self.floor.iter_mut() {
            let y = platform.position.y;
            let d = (platform.target_height - y) * self.clock.delta_as_sec() * 10.0;
            platform.position.y += d;
        }
    }
}

impl GetMeshes for Scene {
    fn mesh_iter(&self) -> std::vec::IntoIter<&IntoMesh> {
        let mut v: Vec<&IntoMesh> = Vec::new();
        for platform in self.floor.iter() {
            v.push(platform);
        }
        v.into_iter()
    }
}

impl GetCamera for Scene {
    fn get_camera(&self) -> (Matrix4<f32>, &Camera) {
        let camera_v_offset = Vector3::y() * self.player.height;
        (self.player.transform.append_translation(&camera_v_offset), &self.player.camera)
    }
}



