mod camera {
    use backend::{VulkanBackend, Constants};
    use nalgebra::*;

    use mursten_blocks::camera::Camera;
    use mursten_blocks::camera::backend::SetCamera;

    impl SetCamera for VulkanBackend {
        fn set_camera(&mut self, transform: Matrix4<f32>, camera: &Camera) {
            self.set_constants(Constants {
                projection: camera.projection.clone(),
                view: transform,
                ..Constants::default()
            });
        }
    }

}

