use backend::{VulkanBackend, Constants};
use mursten_blocks::camera::Camera;
use mursten_blocks::camera::backend::SetCamera;
use nalgebra::*;

impl SetCamera for VulkanBackend {
    fn set_camera(&mut self, camera: &Camera) {
        self.set_constants(Constants {
            projection: camera.projection().clone(),
            view: Matrix4::look_at_lh(
                camera.position(),
                &(camera.position() + camera.direction()),
                &Vector3::y(),
            ),
            ..Constants::default()
        });
    }
}

