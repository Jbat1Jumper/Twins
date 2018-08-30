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

mod input {
    use backend;
    use mursten_blocks::input::{Key, KeyModifiers, KeyboardEvent, MouseEvent};
    use mursten_blocks::input::backend::{KeyboardEventSource, MouseEventSource};
    use winit::ElementState;
    use winit::VirtualKeyCode;

    impl KeyboardEventSource for backend::VulkanBackend {
        fn drain_events(&mut self) -> Vec<KeyboardEvent> {
            self.drain_keyboard_events().into_iter().filter_map(|keyboard_input| -> Option<_> {
                let key = keyboard_input.virtual_keycode.map(|vk| match vk {
                    VirtualKeyCode::A => Some(Key::A),
                    VirtualKeyCode::S => Some(Key::S),
                    VirtualKeyCode::D => Some(Key::D),
                    VirtualKeyCode::Q => Some(Key::Q),
                    VirtualKeyCode::W => Some(Key::W),
                    VirtualKeyCode::E => Some(Key::E),
                    _ => None
                })??;
                let modifiers = KeyModifiers {};

                let event = match keyboard_input.state {
                    ElementState::Pressed => KeyboardEvent::Pressed(key, modifiers),
                    ElementState::Released => KeyboardEvent::Released(key, modifiers),
                };
                Some(event)
            }).collect()
        }
    }

    impl MouseEventSource for backend::VulkanBackend {
        fn drain_events(&mut self) -> Vec<MouseEvent> {
            panic!("MouseEventSource is not implemented yet on vulkan backend")
        }
    }
}
