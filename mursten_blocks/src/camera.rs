use mursten::{Backend, Data, Updater};
use nalgebra::*;

pub trait Camera {
    fn moved(&self) -> bool {
        true
    }
    fn projection<'a>(&'a self) -> &'a Matrix4<f32>;
    fn position<'a>(&'a self) -> &'a Point3<f32>;
    fn direction<'a>(&'a self) -> &'a Vector3<f32>;
}

pub trait GetCamera {
    fn get_camera<'a>(&'a self) -> &'a Camera;
}

pub struct CameraUpdater {}

impl CameraUpdater {
    pub fn new() -> Self {
        CameraUpdater {}
    }
}

impl<B, D> Updater<B, D> for CameraUpdater
where
    D: Data + GetCamera,
    B: Backend<D> + backend::SetCamera,
{
    fn update(&mut self, backend: &mut B, data: &mut D) {
        let camera = data.get_camera();
        if camera.moved() {
            backend.set_camera(camera);
        }
    }
}

pub mod stock {
    use camera::{Camera};
    use nalgebra::*;
    
    pub struct BasicCamera {
        direction: Vector3<f32>,
        position: Point3<f32>,
        projection: Matrix4<f32>,
    }

    impl Camera for BasicCamera {
        fn direction(&self) -> &Vector3<f32> {
            &self.direction
        }
        fn position(&self) -> &Point3<f32> {
            &self.position
        }
        fn projection(&self) -> &Matrix4<f32> {
            &self.projection
        }
    }

    impl BasicCamera {
        pub fn rotate(&mut self, rotation: Rotation3<f32>) {
            self.direction = rotation * self.direction;
        }
        pub fn displace(&mut self, displacement: Vector3<f32>) {
            self.position += displacement;
        }
        pub fn look_at(&mut self, _point: Point3<f32>) {
            panic!("look_at is not yet implemented for BasicCamera")
        }
    }

    pub struct OrthographicCamera {}
    
    impl OrthographicCamera {
        pub fn new(position: Point3<f32>, direction: Vector3<f32>) -> BasicCamera {
            BasicCamera {
                direction,
                position,
                projection: Orthographic3::new(-1.0, 1.0, -1.0, 1.0, 10.0, 900.0).to_homogeneous(),
            }
        }
    }

    pub struct PerspectiveCamera {}
    
    impl PerspectiveCamera {
        pub fn new(position: Point3<f32>, direction: Vector3<f32>) -> BasicCamera {
            BasicCamera {
                direction,
                position,
                projection: Perspective3::new(1.0, 1.57, 1.0, 900.0).to_homogeneous(),
            }
        }
    }
}

pub mod backend {
    use camera::Camera;

    pub trait SetCamera {
        fn set_camera(&mut self, &Camera);
    }
}
