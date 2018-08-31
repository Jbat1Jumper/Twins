mod camera {
    use backend::{VulkanBackend, Uniforms};
    use nalgebra::*;

    use mursten_blocks::camera::Camera;
    use mursten_blocks::camera::backend::SetCamera;

    impl SetCamera for VulkanBackend {
        fn set_camera(&mut self, transform: Matrix4<f32>, camera: &Camera) {
            self.set_uniforms(Uniforms {
                projection: camera.projection.clone(),
                view: transform,
                ..Uniforms::default()
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

                let n1 = (v1.position - v3.position).cross(&(v1.position - v2.position)); // Le podes haber pifiado a la dirección de la normal
                vs.push((n1, v1).into());

                let n2 = (v2.position - v1.position).cross(&(v2.position - v3.position)); // Le podes haber pifiado a la dirección de la normal
                vs.push((n2, v2).into());

                let n3 = (v3.position - v2.position).cross(&(v3.position - v1.position)); // Le podes haber pifiado a la dirección de la normal
                vs.push((n3, v3).into());
                vs
            });
            self.enqueue_vertexes(vertexes);
        }
    }

    impl From<(Vector3<f32>, Vertex)> for backend::Vertex {
        fn from(pair: (Vector3<f32>, Vertex)) -> backend::Vertex {
            let (n, v) = pair;
            backend::Vertex {
                position: v.position.to_homogeneous().into(),
                normal: n.to_homogeneous().into(),
                color: v.color.into(),
                texture: [v.texture.x, v.texture.y],
            }
        }
    }
}

mod light {
    use backend;
    use mursten_blocks::light::Light;
    use mursten_blocks::light::backend::SetLights;
    use nalgebra::*;

    impl SetLights for backend::VulkanBackend {
        fn set_light(&mut self, light: Light) {
            let Light { point, color, strength } = light;
            let mut uniforms = self.get_uniforms();
            uniforms.diffuse_origin = Vector4::new(point.x, point.y, point.z, 1.0);
            uniforms.diffuse_color = Vector4::new(color.x, point.y, point.z, 1.0);
            uniforms.diffuse_strength = strength;
            self.set_uniforms(uniforms);
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
