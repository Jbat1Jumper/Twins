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

mod render {
    use backend;
    use nalgebra::*;
    use mursten_blocks::mesh_renderer::backend::RenderMesh;
    use mursten_blocks::geometry::{Mesh, Triangle, Vertex};

    impl RenderMesh for backend::VulkanBackend {
        fn queue_render(&mut self, m: Matrix4<f32>, mesh: Mesh) {
            let vertexes = mesh.transform(&m).triangles.into_iter().fold(Vec::new(), |mut vs, t| {
                let Triangle { v1, v2, v3 } = t;
                vs.push(v1.into());
                vs.push(v2.into());
                vs.push(v3.into());
                vs
            });
            self.enqueue_vertexes(vertexes);
        }
    }

    impl From<Vertex> for backend::Vertex {
        fn from(v: Vertex) -> backend::Vertex {
            backend::Vertex {
                position: [v.position.x, v.position.y, v.position.z, 1.0],
                color: v.color,
                texture: v.texture,
            }
        }
    }
}
