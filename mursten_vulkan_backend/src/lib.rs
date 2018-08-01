#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate vulkano_shader_derive;
extern crate winit;
extern crate vulkano_win;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

extern crate mursten;


use vulkano_win::VkSurfaceBuild;

use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::DynamicState;
use vulkano::descriptor::PipelineLayoutAbstract;
use vulkano::device::Device;
use vulkano::device::Queue;
use vulkano::framebuffer::Framebuffer;
use vulkano::framebuffer::RenderPass;
use vulkano::framebuffer::RenderPassDesc;
use vulkano::framebuffer::RenderPassAbstract;
use vulkano::framebuffer::Subpass;
use vulkano::image::SwapchainImage;
use vulkano::instance::Instance;
use vulkano::instance::PhysicalDevice;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::AcquireError;
use vulkano::swapchain::PresentMode;
use vulkano::swapchain::Surface;
use vulkano::swapchain::SurfaceTransform;
use vulkano::swapchain::Swapchain;
use vulkano::swapchain::SwapchainCreationError;
use vulkano::swapchain;
use vulkano::sync::GpuFuture;
use vulkano::sync::now;

use winit::Window;
use winit::WindowBuilder;
use winit::EventsLoop;

use std::sync::Arc;
use std::mem;


use mursten::{Backend, Data, RenderChain, UpdateChain};


pub struct VulkanBackend {
    triangles_queue: Vec<Triangle>,
    dimensions: (u32, u32),
}


impl VulkanBackend {
    pub fn new() -> Self {
        Self {
            triangles_queue: Vec::new(),
            dimensions: (0, 0),
        }
    }

    pub fn screen_size(&self) -> (u32, u32) {
        self.dimensions
    }

    pub fn queue_render(&mut self, triangles: Vec<Triangle>) {
        self.triangles_queue.extend(triangles);
    }
}

pub struct Triangle {
    pub v1: [f32; 4],
    pub v2: [f32; 4],
    pub v3: [f32; 4],
    pub v1_color: [f32; 4],
    pub v2_color: [f32; 4],
    pub v3_color: [f32; 4],
    pub v1_tex: [f32; 2],
    pub v2_tex: [f32; 2],
    pub v3_tex: [f32; 2],
}

impl Triangle {
    pub fn white(v1: [f32; 4], v2: [f32; 4], v3: [f32; 4]) -> Self {
        Self {
            v1, v2, v3,
            ..Self::default()
        }
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Triangle {
            v1: [0.0, 0.0, 0.0, 1.0],
            v2: [0.0, 0.0, 0.0, 1.0],
            v3: [0.0, 0.0, 0.0, 1.0],
            v1_color: [1.0, 1.0, 1.0, 1.0],
            v2_color: [1.0, 1.0, 1.0, 1.0],
            v3_color: [1.0, 1.0, 1.0, 1.0],
            v1_tex: [0.0, 0.0],
            v2_tex: [0.0, 0.0],
            v3_tex: [0.0, 0.0],
        }
    }
}

impl<D> Backend<D> for VulkanBackend
where
    D: Data,
{
    fn run(
        &mut self,
        mut update_chain: UpdateChain<Self, D>,
        mut render_chain: RenderChain<Self, D>,
        mut data: D,
    ) -> D {

        let instance = {
            let extensions = vulkano_win::required_extensions();
            Instance::new(None, &extensions, None).expect("failed to create Vulkan instance")
        };

        let mut physical_devices = vulkano::instance::PhysicalDevice::enumerate(&instance);
        let physical = physical_devices.next().expect("no device available");

        let mut events_loop = EventsLoop::new();
        let window = WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap();

        let mut dimensions = {
            let (width, height) = window.window().get_inner_size().unwrap();
            self.dimensions = (width, height);
            [width, height]
        };

        let queue = physical.queue_families().find(|&q| {
            q.supports_graphics() && window.is_supported(q).unwrap_or(false)
        }).expect("couldn't find a graphical queue family");

        let (device, mut queues) = {
            let device_ext = vulkano::device::DeviceExtensions {
                khr_swapchain: true,
                .. vulkano::device::DeviceExtensions::none()
            };

            Device::new(physical, physical.supported_features(), &device_ext,
                        [(queue, 0.5)].iter().cloned()).expect("failed to create device")
        };
        let queue = queues.next().unwrap();

        let (mut swapchain, mut images) = {
            let caps = window.capabilities(physical)
                              .expect("failed to get surface capabilities");
            let alpha = caps.supported_composite_alpha.iter().next().unwrap();
            dimensions = caps.current_extent.unwrap_or(dimensions);
            self.dimensions = (dimensions[0], dimensions[1]);

            let format = caps.supported_formats[0].0;
            Swapchain::new(device.clone(), window.clone(), caps.min_image_count, format,
                       dimensions, 1, caps.supported_usage_flags, &queue,
                       SurfaceTransform::Identity, alpha, PresentMode::Fifo, true,
                       None).expect("failed to create swapchain")
        };

        let vs = shaders::vs::Shader::load(device.clone()).expect("failed to create shader module");
        let fs = shaders::fs::Shader::load(device.clone()).expect("failed to create shader module");

        let render_pass = Arc::new(single_pass_renderpass!(device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: swapchain.format(),
                    samples: 1,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        ).unwrap());

        let pipeline = Arc::new(GraphicsPipeline::start()
            .vertex_input_single_buffer()
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_list()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fs.main_entry_point(), ())
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap());
        

        let mut framebuffers: Option<Vec<Arc<vulkano::framebuffer::Framebuffer<_,_>>>> = None;
        let mut previous_frame_end = Box::new(now(device.clone())) as Box<GpuFuture>;
        let mut recreate_swapchain = false;


        loop { 
            update_chain.update(self, &mut data);
            render_chain.render(self, &data);

            previous_frame_end.cleanup_finished();

            let vertex_buffer = {
                let vertexes: Vec<Vertex> = self.triangles_queue.iter().flat_map(|t| { t.vertexes() }).collect();
                self.triangles_queue.clear();
                CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), vertexes.iter().cloned()).expect("failed to create buffer")
            };

            if recreate_swapchain {
                dimensions = {
                    let (new_width, new_height) = window.window().get_inner_size().unwrap();
                    self.dimensions = (new_width, new_height);
                    [new_width, new_height]
                };

                let (new_swapchain, new_images) = match swapchain.recreate_with_dimension(dimensions) {
                    Ok(r) => r,
                    Err(SwapchainCreationError::UnsupportedDimensions) => {
                        continue;
                    },
                    Err(err) => panic!("{:?}", err)
                };

                mem::replace(&mut swapchain, new_swapchain);
                mem::replace(&mut images, new_images);

                framebuffers = None;

                recreate_swapchain = false;
            }

            if framebuffers.is_none() {
                let new_framebuffers = Some(images.iter().map(|image| {
                    Arc::new(Framebuffer::start(render_pass.clone())
                             .add(image.clone()).unwrap()
                             .build().unwrap())
                }).collect::<Vec<_>>());
                mem::replace(&mut framebuffers, new_framebuffers);
            }

            let (image_num, acquire_future) = match swapchain::acquire_next_image(swapchain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    recreate_swapchain = true;
                    continue;
                },
                Err(err) => panic!("{:?}", err)
            };

            let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family()).unwrap()
                .begin_render_pass(framebuffers.as_ref().unwrap()[image_num].clone(), false,
                                   vec![[0.1, 0.1, 0.1, 1.0].into()])
                .unwrap()
                .draw(pipeline.clone(),
                      DynamicState {
                          line_width: None,
                          viewports: Some(vec![Viewport {
                              origin: [0.0, 0.0],
                              dimensions: [dimensions[0] as f32, dimensions[1] as f32],
                              depth_range: 0.0 .. 1.0,
                          }]),
                          scissors: None,
                      },
                      vertex_buffer.clone(), (), ())
                .unwrap()
                .end_render_pass()
                .unwrap()
                .build().unwrap();

            let future = previous_frame_end.join(acquire_future)
                .then_execute(queue.clone(), command_buffer).unwrap()
                .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
                .then_signal_fence_and_flush().unwrap();
            previous_frame_end = Box::new(future) as Box<_>;

            let mut done = false;
            events_loop.poll_events(|ev| {
                match ev {
                    winit::Event::WindowEvent { event: winit::WindowEvent::Closed, .. } => done = true,
                    winit::Event::WindowEvent { event: winit::WindowEvent::Resized(_, _), .. } => recreate_swapchain = true,
                    _ => ()
                }
            });
        }

        data
    }

}

#[derive(Debug, Clone)]
struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
    texture: [f32; 2],
}
impl_vertex!(Vertex, position, color, texture);

impl Triangle {
    fn vertexes(&self) -> Vec<Vertex> {
        vec!(
            Vertex { position: self.v1, color: self.v1_color, texture: self.v1_tex, },
            Vertex { position: self.v2, color: self.v2_color, texture: self.v2_tex, },
            Vertex { position: self.v3, color: self.v3_color, texture: self.v3_tex, },
        )
    }
}

pub mod shaders {
    pub mod vs {
        #[derive(VulkanoShader)]
        #[ty = "vertex"]
        #[src = "
            #version 450

            layout(location = 0) in vec4 position;
            layout(location = 4) in vec4 color;
            layout(location = 8) in vec2 texture;
            layout(location = 0) out vec4 outColor;

            void main() {
                gl_Position = position;
                outColor = color;
            }
        "]
        struct Dummy;
    }

    pub mod fs {
        #[derive(VulkanoShader)]
        #[ty = "fragment"]
        #[src = "
            #version 450

            layout(location = 0) in vec4 inColor;
            layout(location = 0) out vec4 outColor;

            float rand(vec2 co) {
                return fract(sin(dot(co.xy ,vec2(12.9898,78.233))) * 43758.5453);
            }

            void main() {
                outColor = inColor;
            }
        "]
        struct Dummy;
    }
}
