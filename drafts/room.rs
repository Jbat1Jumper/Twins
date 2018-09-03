extern crate mursten;
extern crate mursten_blocks;
extern crate mursten_vulkan_backend;
extern crate nalgebra;

use mursten::{Application, Backend, Data};

use mursten_blocks::geometry::{Mesh, Triangle, Vertex};
use mursten_blocks::camera::{Camera, CameraUpdater, GetCamera};
use mursten_blocks::time::{Clock, ClockUpdater, OnTick, Tick};
use mursten_blocks::input::{Key, KeyboardEvent, OnKeyboard, KeyboardUpdater, MouseEvent, OnMouse, MouseUpdater};
use mursten_blocks::mesh_renderer::{GetMeshes, IntoMesh, MeshRenderer};
use mursten_blocks::light::{Light, GetLights, LightUpdater};

use mursten_vulkan_backend::VulkanBackend;

use nalgebra::*;
use std::f32::consts::PI;


pub fn main() {
    let backend = VulkanBackend::new();
    let scene = Scene::new();
    Application::new(backend)
        .add_updater(ClockUpdater::new())
        .add_updater(CameraUpdater::new())
        .add_updater(KeyboardUpdater::new())
        .add_renderer(MeshRenderer::new())
        .run(scene);
}

struct Scene { 
    clock: Clock,
    player: Player,
    floor: Platform,
    walls: Vec<Platform>,
    roof: Platform,
    desk: Desk,
    lamp: Lamp,
}

struct Player {
    camera: Camera,
    height: f32,
    position: Point3<f32>,
    direction: Vector3<f32>,

    moving_towards: Vector3<f32>,
    rotating_towards: f32,
}

impl Player {
    pub fn new(position: Point3<f32>) -> Self {
        Player {
            camera: Camera::perspective(),
            height: 1.7,
            moving_towards: Vector3::new(0.0, 0.0, 0.0),
            rotating_towards: 0.0,
            position,
            direction: Vector3::z(),
        }
    }
}

struct Platform {
    position: Point3<f32>,
    rotation: Rotation3<f32>,
    scale: Vector3<f32>,
    color: Vector3<f32>,
}

impl Platform {
    pub fn new(position: Point3<f32>) -> Self {
        Self {
            position,
            rotation: Rotation3::from_axis_angle(&Vector3::y_axis(), 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            color: Vector3::new(1.0, 1.0, 1.0),
        }
    }
    pub fn rotated(self, rotation: Rotation3<f32>) -> Self {
        Self { rotation, ..self }
    }
    pub fn scaled(self, scale: Vector3<f32>) -> Self {
        Self { scale, ..self }
    }
    pub fn colored(self, color: Vector3<f32>) -> Self {
        Self { color, ..self }
    }
}

impl IntoMesh for Platform {
    fn transform(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&self.position.coords) * self.rotation.to_homogeneous() * Matrix4::new_nonuniform_scaling(&self.scale)
    }
    fn mesh(&self) -> Mesh {
        let v1 = Vertex::at(Point3::new(-0.5, 0.0, -0.5));
        let v2 = Vertex::at(Point3::new(-0.5, 0.0,  0.5));
        let v3 = Vertex::at(Point3::new( 0.5, 0.0,  0.5));
        let v4 = Vertex::at(Point3::new( 0.5, 0.0, -0.5));

        Mesh {
            triangles: vec![
                Triangle::new(v1, v3, v2),
                Triangle::new(v1, v4, v3),
            ],
        }.color(Vector4::new(self.color.x, self.color.y, self.color.z, 1.0))
    }
}

struct Desk {
    position: Point3<f32>,
    rotation: Rotation3<f32>,
}

impl Desk {
    pub fn new(position: Point3<f32>) -> Self {
        Self {
            position,
            rotation: Rotation3::from_axis_angle(&Vector3::y_axis(), 0.0),
        }
    }
    pub fn rotated(self, rotation: Rotation3<f32>) -> Self {
        Self { rotation, ..self }
    }
}

impl IntoMesh for Desk {
    fn transform(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&self.position.coords) * self.rotation.to_homogeneous()
    }
    fn mesh(&self) -> Mesh {
        let mut triangles = Vec::new();

        let desk_leg = |position, rotation| {
            let v1 = Vertex::at(position + rotation * Vector3::new(-0.04,  0.99, -0.04));
            let v2 = Vertex::at(position + rotation * Vector3::new(-0.04,  0.99,  0.02));
            let v3 = Vertex::at(position + rotation * Vector3::new( 0.02,  0.99,  0.02));
            let v4 = Vertex::at(position + rotation * Vector3::new( 0.02,  0.99, -0.04));
            let v5 = Vertex::at(position + rotation * Vector3::new(-0.02,  0.0,  -0.02));
            let v6 = Vertex::at(position + rotation * Vector3::new(-0.02,  0.0,   0.02));
            let v7 = Vertex::at(position + rotation * Vector3::new( 0.02,  0.0,   0.02));
            let v8 = Vertex::at(position + rotation * Vector3::new( 0.02,  0.0,  -0.02));

            vec![
                Triangle::new(v1, v3, v2),
                Triangle::new(v1, v4, v3),
                Triangle::new(v5, v4, v1),
                Triangle::new(v5, v8, v4),
                Triangle::new(v8, v3, v4),
                Triangle::new(v8, v7, v3),
                Triangle::new(v7, v2, v3),
                Triangle::new(v7, v6, v2),
                Triangle::new(v6, v1, v2),
                Triangle::new(v6, v5, v1),
                Triangle::new(v5, v7, v8),
                Triangle::new(v5, v6, v7),
            ]
        };

        let desk_table = || {
            let v1 = Vertex::at(Point3::new(-0.27,  1.0,  -0.52));
            let v2 = Vertex::at(Point3::new(-0.27,  1.0,   0.52));
            let v3 = Vertex::at(Point3::new( 0.27,  1.0,   0.52));
            let v4 = Vertex::at(Point3::new( 0.27,  1.0,  -0.52));
            let v5 = Vertex::at(Point3::new(-0.27,  0.96, -0.52));
            let v6 = Vertex::at(Point3::new(-0.27,  0.96,  0.52));
            let v7 = Vertex::at(Point3::new( 0.27,  0.96,  0.52));
            let v8 = Vertex::at(Point3::new( 0.27,  0.96, -0.52));

            vec![
                Triangle::new(v1, v3, v2),
                Triangle::new(v1, v4, v3),
                Triangle::new(v5, v4, v1),
                Triangle::new(v5, v8, v4),
                Triangle::new(v8, v3, v4),
                Triangle::new(v8, v7, v3),
                Triangle::new(v7, v2, v3),
                Triangle::new(v7, v6, v2),
                Triangle::new(v6, v1, v2),
                Triangle::new(v6, v5, v1),
                Triangle::new(v5, v7, v8),
                Triangle::new(v5, v6, v7),
            ]
        };

        triangles.append(&mut desk_leg(Point3::new( 0.23, 0.0,  0.48), Rotation3::from_axis_angle(&Vector3::y_axis(), 0.0)));
        triangles.append(&mut desk_leg(Point3::new(-0.23, 0.0,  0.48), Rotation3::from_axis_angle(&Vector3::y_axis(), -PI/2.0)));
        triangles.append(&mut desk_leg(Point3::new(-0.23, 0.0, -0.48), Rotation3::from_axis_angle(&Vector3::y_axis(), -PI)));
        triangles.append(&mut desk_leg(Point3::new( 0.23, 0.0, -0.48), Rotation3::from_axis_angle(&Vector3::y_axis(), 3.0 * -PI/2.0)));
        triangles.append(&mut desk_table());

        Mesh { triangles, }.color(Palette::ZinnwalditeBrown.into())
    }
}

struct Lamp {
    position: Point3<f32>,
    rotation: Rotation3<f32>,
}

impl Lamp {
    pub fn new(position: Point3<f32>) -> Self {
        Self {
            position,
            rotation: Rotation3::from_axis_angle(&Vector3::y_axis(), 0.0),
        }
    }
    pub fn rotated(self, rotation: Rotation3<f32>) -> Self {
        Self { rotation, ..self }
    }
}

impl IntoMesh for Lamp {
    fn transform(&self) -> Matrix4<f32> {
        Matrix4::new_translation(&self.position.coords) * self.rotation.to_homogeneous()
    }
    fn mesh(&self) -> Mesh {
        Mesh {
            triangles: Vec::new(),
        }
    }
}

impl Scene {
    pub fn new() -> Self {
        let floor_color = Palette::PewterBlue.into();
        let wall_color = Palette::AntiqueRuby.into();
        let roof_color = Palette::RaisinBlack.into();
        Scene {
            clock: Clock::new(),
            player: Player::new(Point3::new(1.0, 0.0, -2.0)),
            floor: Platform::new(Point3::origin())
                .scaled(Vector3::new(6.0, 1.0, 6.0))
                .colored(floor_color),
            walls: vec![
                Platform::new(Point3::new(-3.0, 1.5, 0.0))
                    .scaled(Vector3::new(3.0, 1.0, 6.0))
                    .colored(wall_color)
                    .rotated(Rotation3::from_axis_angle(&Vector3::z_axis(), -PI/2.0)),
                Platform::new(Point3::new(0.0, 1.5, 3.0))
                    .scaled(Vector3::new(6.0, 1.0, 3.0))
                    .colored(wall_color)
                    .rotated(Rotation3::from_axis_angle(&Vector3::x_axis(), PI/2.0)),
                Platform::new(Point3::new(3.0, 1.5, 0.0))
                    .scaled(Vector3::new(3.0, 1.0, 6.0))
                    .colored(wall_color)
                    .rotated(Rotation3::from_axis_angle(&Vector3::z_axis(), -PI/2.0)),
                Platform::new(Point3::new(0.0, 1.5, -3.0))
                    .scaled(Vector3::new(6.0, 1.0, 3.0))
                    .colored(wall_color)
                    .rotated(Rotation3::from_axis_angle(&Vector3::x_axis(), -PI/2.0)),
            ],
            roof: Platform::new(Point3::origin() + Vector3::y() * 3.0)
                .scaled(Vector3::new(6.0, 1.0, 6.0))
                .colored(roof_color),
            desk: Desk::new(Point3::new(-2.0, 0.0, 0.0)),
            lamp: Lamp::new(Point3::new(-2.0, 1.0, 1.0)),
        }
    }
}

impl mursten::Data for Scene {}

impl OnTick for Scene {
    fn on_tick(&mut self, tick: Tick) {
        self.clock += tick;
        std::thread::sleep_ms(20);

        const player_speed: f32 = 2.0;
        let translation = self.player.moving_towards * self.clock.delta_as_sec() * player_speed ;
        self.player.position += Rotation3::rotation_between(&Vector3::z(), &self.player.direction).unwrap() * translation;
        let rotation_angle = self.player.rotating_towards * self.clock.delta_as_sec() * player_speed * 0.3;
        self.player.direction = Rotation3::from_axis_angle(&Vector3::y_axis(), rotation_angle) * self.player.direction;
    }
}

impl GetMeshes for Scene {
    fn mesh_iter(&self) -> std::vec::IntoIter<&IntoMesh> {
        let mut v: Vec<&IntoMesh> = Vec::new();
        v.push(&self.floor);
        v.push(&self.roof);
        for wall in self.walls.iter() {
            v.push(wall);
        }
        v.push(&self.desk);
        v.into_iter()
    }
}

impl GetCamera for Scene {
    fn get_camera(&self) -> (Matrix4<f32>, &Camera) {
        let camera_v_offset = Vector3::y() * self.player.height;
        let eye = self.player.position + camera_v_offset;
        let target = eye + self.player.direction;
        let view = Matrix4::new(1.0, 0.0, 0.0, 0.0,
                                0.0, 1.0, 0.0, 0.0,
                                0.0, 0.0,-1.0, 0.0,
                                0.0, 0.0, 0.0, 1.0) * Matrix4::look_at_lh(&eye, &target, &Vector3::y());
        (view, &self.player.camera)
    }
}

impl GetLights for Scene {
    fn get_light(&self) -> Light {
        let p = Point3::origin() + Rotation3::from_axis_angle(&Vector3::y_axis(), self.clock.time_in_sec()) * Vector3::new(2.0, 3.0, 0.0);
        Light::new(p, Vector3::new(1.0, 1.0, 1.0), 0.0)
    }
}

impl OnKeyboard for Scene {
    fn handle(&mut self, event: KeyboardEvent) {
        let mt = &mut self.player.moving_towards;
        let rt = &mut self.player.rotating_towards;
        match event {
            KeyboardEvent::Pressed(key, _) => {
                match key {
                    Key::A => mt.x = -1.0,
                    Key::S => mt.z = -1.0,
                    Key::D => mt.x = 1.0,
                    Key::W => mt.z = 1.0,
                    Key::Q => *rt = -1.0,
                    Key::E => *rt = 1.0,
                };
            }
            KeyboardEvent::Released(key, _) => {
                match key {
                    Key::A | Key::D => mt.x = 0.0,
                    Key::S | Key::W => mt.z = 0.0,
                    Key::Q | Key::E => *rt = 0.0,
                    _ => (),
                };
            }
        }
    }
}

impl OnMouse for Scene {
    fn handle(&mut self, event: MouseEvent) {
        match event {
            MouseEvent::Wheel(displacement) => {
                let r = Rotation3::rotation_between(&Vector3::z(), &Vector3::new(displacement.x, -displacement.y, 100.0)).unwrap();
                self.player.direction = r * self.player.direction;
            },
            _ => (),
        }
    }
}

enum Palette {
    RaisinBlack,
    AntiqueRuby,
    ZinnwalditeBrown,
    PewterBlue,
    LapisLazuli,
}

impl Into<Vector3<f32>> for Palette {
    fn into(self) -> Vector3<f32> {
        match self {
            Palette::RaisinBlack      => Vector3::new(0x26 as f32, 0x26 as f32, 0x26 as f32) / 256.0,
            Palette::AntiqueRuby      => Vector3::new(0x88 as f32, 0x29 as f32, 0x2F as f32) / 256.0,
            Palette::ZinnwalditeBrown => Vector3::new(0x2E as f32, 0x1E as f32, 0x0F as f32) / 256.0,
            Palette::PewterBlue       => Vector3::new(0x90 as f32, 0xA9 as f32, 0xB7 as f32) / 256.0,
            Palette::LapisLazuli      => Vector3::new(0x25 as f32, 0x5C as f32, 0x99 as f32) / 256.0,
        }
    }
}

impl Into<Vector4<f32>> for Palette {
    fn into(self) -> Vector4<f32> {
        let c: Vector3<f32> = self.into();
        Vector4::new(c.x, c.y, c.z, 1.0)
    }
}
