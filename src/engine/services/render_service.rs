use crate::prelude::*;
use smallvec::{smallvec, SmallVec};
use vulkano::{pipeline::Pipeline, sync::GpuFuture};

/// Handles all rendering related tasks.
///
/// This takes care of making the required calls to graphicsAPIs for different rendering processes.
pub struct RenderService {
    graphics_api: Option<Arc<RwLock<dyn GraphicsAPI>>>,
}

impl RenderService {
    pub fn new() -> Self {
        Self { graphics_api: None }
    }
}

impl Service for RenderService {
    fn update(&self) {
    }

    fn get_data(&self) {}
}

/// A trait a graphicsAPI must implement to be usable by the [`RenderService`].
///
/// This is to allow for easy addition of new APIs in the future by making graphicsAPI calls more
/// generic.
trait GraphicsAPI: Any + Send + Sync {
    fn render_frame(&self);
}

/// Defines a struct containing all vulkan functionalities for different processes.
///
/// [TO-DO]: Add proper error handling
struct Vulkan {
    logical_device: Arc<vulkano::device::Device>,
    queue: Arc<vulkano::device::Queue>,
    swapchain: Arc<vulkano::swapchain::Swapchain>,
    memory_allocator: Arc<vulkano::memory::allocator::StandardMemoryAllocator>,
    command_buffer_allocator:
        Arc<vulkano::command_buffer::allocator::StandardCommandBufferAllocator>,
    descriptor_set_allocator:
        Arc<vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator>,
    graphics_pipeline: Arc<vulkano::pipeline::graphics::GraphicsPipeline>,
    framebuffers: Vec<Arc<vulkano::render_pass::Framebuffer>>,
    viewports: SmallVec<[vulkano::pipeline::graphics::viewport::Viewport; 2]>,
    scissors: SmallVec<[vulkano::pipeline::graphics::viewport::Scissor; 2]>,
    vulkan_objects: HashMap<usize, VulkanObject>,
    window: Arc<winit::window::Window>,
}

impl Vulkan {
    pub fn new(
        event_loop: &winit::event_loop::ActiveEventLoop,
        window: Arc<winit::window::Window>,
        viewport_info: &ViewportInfo,
    ) -> Self {
        log!(Self, Critical, "Setting up vulkan graphicsAPI.");

        let device_extensions = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            ..Default::default()
        };

        let instance = Vulkan::create_instance(event_loop);
        let surface = Vulkan::create_surface(instance.clone(), window.clone());
        let (physical_device, queue_family_index) =
            Vulkan::create_physical_device(instance.clone(), surface.clone(), &device_extensions);
        let (logical_device, queue) = Vulkan::create_logical_device(
            physical_device.clone(),
            queue_family_index,
            &device_extensions,
        );
        let (swapchain, images) = Vulkan::create_swapchain(
            physical_device.clone(),
            logical_device.clone(),
            window.clone(),
            surface.clone(),
        );
        let image_views = Vulkan::create_image_views(&images);
        let render_pass = Vulkan::create_render_pass(logical_device.clone(), swapchain.clone());
        let memory_allocator = Vulkan::create_memory_allocator(logical_device.clone());
        let command_buffer_allocator =
            Vulkan::create_command_buffer_allocator(logical_device.clone());
        let descriptor_set_allocator =
            Vulkan::create_descriptor_set_allocator(logical_device.clone());
        let graphics_pipeline =
            Vulkan::create_graphics_pipeline(logical_device.clone(), render_pass.clone());
        let framebuffers = Vulkan::create_frame_buffers(
            render_pass.clone(),
            image_views.clone(),
            memory_allocator.clone(),
        );

        let viewports = smallvec![vulkano::pipeline::graphics::viewport::Viewport {
            offset: [viewport_info.offset[0], viewport_info.offset[1]],
            extent: [viewport_info.extent[0], viewport_info.extent[1]],
            depth_range: 0.0..=1.0,
        }];

        let scissors = smallvec![vulkano::pipeline::graphics::viewport::Scissor {
            offset: [
                viewport_info.offset[0] as u32,
                viewport_info.offset[1] as u32
            ],
            extent: [
                viewport_info.extent[0] as u32,
                viewport_info.extent[1] as u32
            ],
        }];

        Self {
            logical_device,
            queue,
            swapchain,
            memory_allocator,
            command_buffer_allocator,
            descriptor_set_allocator,
            graphics_pipeline,
            framebuffers,
            viewports,
            scissors,
            vulkan_objects: HashMap::new(),
            window,
        }
    }

    /// Creates a Vulkan instance.
    fn create_instance(
        active_event_loop: &winit::event_loop::ActiveEventLoop,
    ) -> std::sync::Arc<vulkano::instance::Instance> {
        log!(Self, Critical, "Creating vulkan instance.");

        let library = vulkano::library::VulkanLibrary::new().unwrap();
        let extensions =
            vulkano::swapchain::Surface::required_extensions(&active_event_loop).unwrap();
        let instance_create_info = vulkano::instance::InstanceCreateInfo {
            flags: vulkano::instance::InstanceCreateFlags::ENUMERATE_PORTABILITY,
            enabled_extensions: extensions,
            ..Default::default()
        };
        let instance = vulkano::instance::Instance::new(library, instance_create_info).unwrap();

        log!(Self, Critical, "Created vulkan instance.");
        instance
    }

    /// Creates a Vulkan surface.
    fn create_surface(
        instance: std::sync::Arc<vulkano::instance::Instance>,
        window: std::sync::Arc<winit::window::Window>,
    ) -> std::sync::Arc<vulkano::swapchain::Surface> {
        log!(Self, Critical, "Creating vulkan surface.");

        let surface = vulkano::swapchain::Surface::from_window(instance, window).unwrap();

        log!(Self, Critical, "Created vulkan surface.");
        surface
    }

    /// Creates a Vulkan physical device collected from the given instance and scored on type
    fn create_physical_device(
        instance: std::sync::Arc<vulkano::instance::Instance>,
        surface: std::sync::Arc<vulkano::swapchain::Surface>,
        device_extensions: &vulkano::device::DeviceExtensions,
    ) -> (
        std::sync::Arc<vulkano::device::physical::PhysicalDevice>,
        u32,
    ) {
        log!(Self, High, "Creating physical device.");

        let physical_devices = instance
            .enumerate_physical_devices()
            .expect("Could not enumerate physical devices");

        log!(
            Self,
            Critical,
            &format!("Found {} physical devices.", physical_devices.len())
        );

        let physical_device = physical_devices
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags
                            .contains(vulkano::device::QueueFlags::GRAPHICS)
                            && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|q| (p, q as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                vulkano::device::physical::PhysicalDeviceType::DiscreteGpu => 0,
                vulkano::device::physical::PhysicalDeviceType::IntegratedGpu => 1,
                vulkano::device::physical::PhysicalDeviceType::VirtualGpu => 2,
                vulkano::device::physical::PhysicalDeviceType::Cpu => 3,
                _ => 4,
            })
            .expect("no device available");

        log!(Self, High, "Physical device created successfully.");
        physical_device
    }

    /// Creates a Vulkan logical device from the provided physical devices based on the selected
    /// device extensions.
    fn create_logical_device(
        physical_device: std::sync::Arc<vulkano::device::physical::PhysicalDevice>,
        queue_family_index: u32,
        device_extensions: &vulkano::device::DeviceExtensions,
    ) -> (
        std::sync::Arc<vulkano::device::Device>,
        std::sync::Arc<vulkano::device::Queue>,
    ) {
        log!(Self, High, "Creating logical device...");

        let (device, mut queues) = vulkano::device::Device::new(
            physical_device.clone(),
            vulkano::device::DeviceCreateInfo {
                queue_create_infos: vec![vulkano::device::QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                enabled_extensions: *device_extensions,
                ..Default::default()
            },
        )
        .expect("failed to create device");

        log!(Self, High, "Logical device created successfully.");
        (device, queues.next().unwrap())
    }

    /// Creates a Vulkan swapchain and a vector of images from the provided surface, window and
    /// devices.
    fn create_swapchain(
        physical_device: std::sync::Arc<vulkano::device::physical::PhysicalDevice>,
        logical_device: std::sync::Arc<vulkano::device::Device>,
        window: std::sync::Arc<winit::window::Window>,
        surface: std::sync::Arc<vulkano::swapchain::Surface>,
    ) -> (
        std::sync::Arc<vulkano::swapchain::Swapchain>,
        Vec<std::sync::Arc<vulkano::image::Image>>,
    ) {
        log!(Self, High, "Creating swapchain...");

        let capabilities = physical_device
            .surface_capabilities(&surface, Default::default())
            .expect("Could not get surface capabilities");

        let dimensions = window.inner_size();
        let composite_alpha = capabilities
            .supported_composite_alpha
            .into_iter()
            .next()
            .unwrap();
        let image_format = physical_device
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0;

        let swapchain_create_info = vulkano::swapchain::SwapchainCreateInfo {
            min_image_count: capabilities.min_image_count + 1,
            image_format,
            image_extent: dimensions.into(),
            image_usage: vulkano::image::ImageUsage::COLOR_ATTACHMENT,
            composite_alpha,
            present_mode: vulkano::swapchain::PresentMode::Fifo,
            ..Default::default()
        };

        let (swapchain, images) = vulkano::swapchain::Swapchain::new(
            logical_device.clone(),
            surface.clone(),
            swapchain_create_info,
        )
        .unwrap();

        log!(Self, High, "Swapchain created successfully.");
        (swapchain, images)
    }

    fn create_image_views(
        images: &Vec<std::sync::Arc<vulkano::image::Image>>,
    ) -> Vec<std::sync::Arc<vulkano::image::view::ImageView>> {
        log!(Self, High, "Creating image views...");

        let mut image_views: Vec<std::sync::Arc<vulkano::image::view::ImageView>> = Vec::new();
        for image in images {
            let create_info = vulkano::image::view::ImageViewCreateInfo {
                view_type: vulkano::image::view::ImageViewType::Dim2d,
                format: image.format(),
                component_mapping: vulkano::image::sampler::ComponentMapping {
                    r: vulkano::image::sampler::ComponentSwizzle::Identity,
                    g: vulkano::image::sampler::ComponentSwizzle::Identity,
                    b: vulkano::image::sampler::ComponentSwizzle::Identity,
                    a: vulkano::image::sampler::ComponentSwizzle::Identity,
                },
                subresource_range: vulkano::image::ImageSubresourceRange {
                    aspects: vulkano::image::ImageAspect::Color.into(),
                    mip_levels: std::ops::Range { start: 0, end: 1 },
                    array_layers: std::ops::Range { start: 0, end: 1 },
                },
                ..Default::default()
            };

            let image_view = vulkano::image::view::ImageView::new(image.clone(), create_info);
            image_views.push(image_view.unwrap());
        }

        log!(Self, High, "Image views created successfully.");
        image_views
    }

    fn create_render_pass(
        logical_device: std::sync::Arc<vulkano::device::Device>,
        swapchain: std::sync::Arc<vulkano::swapchain::Swapchain>,
    ) -> std::sync::Arc<vulkano::render_pass::RenderPass> {
        log!(Self, High, "Creating renderpass...");

        let render_pass = vulkano::single_pass_renderpass!(
            logical_device.clone(),
            attachments: {
                color: {
                    format: swapchain.image_format(),
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
                depth: {
                    format: vulkano::format::Format::D16_UNORM,
                    samples: 1,
                    load_op: Clear,
                    store_op: DontCare,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {depth},
            }
        )
        .unwrap();

        log!(Self, High, "Created renderpass.");
        render_pass
    }

    fn create_graphics_pipeline(
        logical_device: std::sync::Arc<vulkano::device::Device>,
        render_pass: std::sync::Arc<vulkano::render_pass::RenderPass>,
    ) -> std::sync::Arc<vulkano::pipeline::GraphicsPipeline> {
        log!(Self, High, "Creating graphics pipeline...");

        let vs_path = std::path::Path::new(env!("OUT_DIR")).join("shader.vert.spv");
        let fs_path = std::path::Path::new(env!("OUT_DIR")).join("shader.frag.spv");

        let vs = Vulkan::load_shader(logical_device.clone(), vs_path);
        let fs = Vulkan::load_shader(logical_device.clone(), fs_path);

        let stages = smallvec![
            vulkano::pipeline::PipelineShaderStageCreateInfo::new(vs.entry_point("main").unwrap()),
            vulkano::pipeline::PipelineShaderStageCreateInfo::new(fs.entry_point("main").unwrap()),
        ];

        let mut uniform_binding =
            vulkano::descriptor_set::layout::DescriptorSetLayoutBinding::descriptor_type(
                vulkano::descriptor_set::layout::DescriptorType::UniformBuffer,
            );
        uniform_binding.stages = vulkano::shader::ShaderStages::VERTEX;

        let mut texture_binding =
            vulkano::descriptor_set::layout::DescriptorSetLayoutBinding::descriptor_type(
                vulkano::descriptor_set::layout::DescriptorType::CombinedImageSampler,
            );
        texture_binding.stages = vulkano::shader::ShaderStages::FRAGMENT;

        let bindings =
            std::collections::BTreeMap::from([(0, uniform_binding), (1, texture_binding)]);

        let descriptor_set_layout = vulkano::descriptor_set::layout::DescriptorSetLayout::new(
            logical_device.clone(),
            vulkano::descriptor_set::layout::DescriptorSetLayoutCreateInfo {
                bindings,
                ..Default::default()
            },
        );

        let pipeline_layout = vulkano::pipeline::layout::PipelineLayout::new(
            logical_device.clone(),
            vulkano::pipeline::layout::PipelineLayoutCreateInfo {
                set_layouts: vec![descriptor_set_layout.unwrap()],
                push_constant_ranges: vec![vulkano::pipeline::layout::PushConstantRange {
                    stages: vulkano::shader::ShaderStages::VERTEX,
                    offset: 0,
                    size: std::mem::size_of::<PushConstants>() as u32,
                }],
                ..Default::default()
            },
        );

        let mut pipeline_info = vulkano::pipeline::graphics::GraphicsPipelineCreateInfo::layout(
            pipeline_layout.unwrap().clone(),
        );

        pipeline_info.stages = stages;
        pipeline_info.vertex_input_state = Some(
            vulkano::pipeline::graphics::vertex_input::VertexDefinition::definition(
                &<Vertex as vulkano::pipeline::graphics::vertex_input::Vertex>::per_vertex(),
                &vs.entry_point("main").unwrap(),
            )
            .unwrap(),
        );
        pipeline_info.input_assembly_state =
            Some(vulkano::pipeline::graphics::input_assembly::InputAssemblyState::default());
        pipeline_info.dynamic_state = std::collections::HashSet::from_iter([
            vulkano::pipeline::DynamicState::ViewportWithCount,
            vulkano::pipeline::DynamicState::ScissorWithCount,
        ]);
        pipeline_info.viewport_state = Some(vulkano::pipeline::graphics::viewport::ViewportState {
            viewports: smallvec![],
            scissors: smallvec![],
            ..Default::default()
        });
        pipeline_info.rasterization_state =
            Some(vulkano::pipeline::graphics::rasterization::RasterizationState::default());
        pipeline_info.multisample_state =
            Some(vulkano::pipeline::graphics::multisample::MultisampleState::default());
        pipeline_info.color_blend_state =
            Some(vulkano::pipeline::graphics::color_blend::ColorBlendState {
                attachments: vec![
                    vulkano::pipeline::graphics::color_blend::ColorBlendAttachmentState {
                        blend: None,
                        color_write_mask:
                            vulkano::pipeline::graphics::color_blend::ColorComponents::all(),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            });
        pipeline_info.subpass = Some(
            vulkano::render_pass::Subpass::from(render_pass.clone(), 0)
                .unwrap()
                .into(),
        );

        let depth_stencil_state = vulkano::pipeline::graphics::depth_stencil::DepthStencilState {
            depth: Some(vulkano::pipeline::graphics::depth_stencil::DepthState::simple()),
            ..Default::default()
        };
        pipeline_info.depth_stencil_state = Some(depth_stencil_state);

        let pipeline =
            vulkano::pipeline::GraphicsPipeline::new(logical_device.clone(), None, pipeline_info)
                .unwrap();

        log!(Self, High, "Graphics pipeline created successfully.");
        pipeline
    }

    fn create_frame_buffers(
        render_pass: std::sync::Arc<vulkano::render_pass::RenderPass>,
        image_views: Vec<std::sync::Arc<vulkano::image::view::ImageView>>,
        memory_allocator: std::sync::Arc<vulkano::memory::allocator::StandardMemoryAllocator>,
    ) -> Vec<std::sync::Arc<vulkano::render_pass::Framebuffer>> {
        log!(Self, High, "Creating frame buffer...");

        let mut framebuffers: Vec<std::sync::Arc<vulkano::render_pass::Framebuffer>> = vec![];
        for image_view in image_views.iter() {
            let depth_image_create_info = vulkano::image::ImageCreateInfo {
                image_type: vulkano::image::ImageType::Dim2d,
                format: render_pass.attachments()[1].format,
                extent: [
                    image_view.image().extent()[0],
                    image_view.image().extent()[1],
                    1,
                ],
                usage: vulkano::image::ImageUsage::DEPTH_STENCIL_ATTACHMENT,
                ..Default::default()
            };
            let depth_image_allocation_info = vulkano::memory::allocator::AllocationCreateInfo {
                memory_type_filter: vulkano::memory::allocator::MemoryTypeFilter::PREFER_DEVICE,
                ..Default::default()
            };
            let depth_image = vulkano::image::Image::new(
                memory_allocator.clone(),
                depth_image_create_info,
                depth_image_allocation_info,
            )
            .unwrap();
            let depth_view = vulkano::image::view::ImageView::new_default(depth_image).unwrap();

            framebuffers.push(
                vulkano::render_pass::Framebuffer::new(
                    render_pass.clone(),
                    vulkano::render_pass::FramebufferCreateInfo {
                        attachments: vec![image_view.clone(), depth_view.clone()],
                        ..Default::default()
                    },
                )
                .unwrap(),
            );
        }
        log!(Self, High, "Created frame buffer...");
        framebuffers
    }

    fn create_memory_allocator(
        logical_device: std::sync::Arc<vulkano::device::Device>,
    ) -> std::sync::Arc<vulkano::memory::allocator::StandardMemoryAllocator> {
        log!(Self, High, "Creating memory allocator...");

        let memory_allocator = std::sync::Arc::new(
            vulkano::memory::allocator::StandardMemoryAllocator::new_default(
                logical_device.clone(),
            ),
        );

        log!(Self, High, "Created memory allocator.");
        memory_allocator
    }

    fn create_command_buffer_allocator(
        logical_device: std::sync::Arc<vulkano::device::Device>,
    ) -> std::sync::Arc<vulkano::command_buffer::allocator::StandardCommandBufferAllocator> {
        std::sync::Arc::new(
            vulkano::command_buffer::allocator::StandardCommandBufferAllocator::new(
                logical_device.clone(),
                Default::default(),
            ),
        )
    }

    fn create_descriptor_set_allocator(
        logical_device: std::sync::Arc<vulkano::device::Device>,
    ) -> std::sync::Arc<vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator> {
        std::sync::Arc::new(vulkano::descriptor_set::allocator::StandardDescriptorSetAllocator::new(
            logical_device,
            vulkano::descriptor_set::allocator::StandardDescriptorSetAllocatorCreateInfo::default(),
        ))
    }

    fn load_shader(
        device: std::sync::Arc<vulkano::device::Device>,
        path: impl AsRef<std::path::Path>,
    ) -> std::sync::Arc<vulkano::shader::ShaderModule> {
        use std::io::Read;
        let mut file = std::fs::File::open(path).expect("Failed to open shader file");
        let mut bytes = vec![];

        file.read_to_end(&mut bytes)
            .expect("Failed to read shader file");

        let words = vulkano::shader::spirv::bytes_to_words(&bytes);

        unsafe {
            vulkano::shader::ShaderModule::new(
                device,
                vulkano::shader::ShaderModuleCreateInfo::new(&words.unwrap()),
            )
            .expect("Failed to create shader module")
        }
    }

    pub fn create_vulkan_object(&mut self, id: &usize, mesh: Mesh, object_transform: &Transform) {
        log!(Self, High, "Creating vulkan object...");

        let vertex_buffer = vulkano::buffer::Buffer::from_iter(
            self.memory_allocator.clone(),
            vulkano::buffer::BufferCreateInfo {
                usage: vulkano::buffer::BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            vulkano::memory::allocator::AllocationCreateInfo {
                memory_type_filter: vulkano::memory::allocator::MemoryTypeFilter::PREFER_DEVICE
                    | vulkano::memory::allocator::MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            mesh.vertices,
        )
        .expect("Failed to create vertex buffer");

        let index_buffer = vulkano::buffer::Buffer::from_iter(
            self.memory_allocator.clone(),
            vulkano::buffer::BufferCreateInfo {
                usage: vulkano::buffer::BufferUsage::INDEX_BUFFER,
                ..Default::default()
            },
            vulkano::memory::allocator::AllocationCreateInfo {
                memory_type_filter: vulkano::memory::allocator::MemoryTypeFilter::PREFER_DEVICE
                    | vulkano::memory::allocator::MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            mesh.indices,
        )
        .expect("Failed to create index buffer");

        let (texture_view, texture_sample) = self.process_image_data(mesh.image);

        let descriptor_set = vulkano::descriptor_set::DescriptorSet::new(
            self.descriptor_set_allocator.clone(),
            self.graphics_pipeline
                .layout()
                .set_layouts()
                .first()
                .unwrap()
                .clone(),
            [
                vulkano::descriptor_set::WriteDescriptorSet::image_view_sampler(
                    1,
                    texture_view.clone(),
                    texture_sample.clone(),
                ),
            ],
            [],
        )
        .unwrap();

        let vulkan_object = VulkanObject::new(
            vertex_buffer,
            index_buffer,
            object_transform.clone(),
            descriptor_set,
        );
        self.vulkan_objects.insert(*id, vulkan_object);
        log!(Self, High, "Vulkan object created successfully.");
    }

    fn create_command_buffer(
        &mut self,
        image_index: usize,
        view_projection: glam::Mat4,
    ) -> std::sync::Arc<vulkano::command_buffer::PrimaryAutoCommandBuffer> {
        let mut builder = vulkano::command_buffer::AutoCommandBufferBuilder::primary(
            self.command_buffer_allocator.clone(),
            self.logical_device.active_queue_family_indices()[0],
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .begin_render_pass(
                vulkano::command_buffer::RenderPassBeginInfo {
                    clear_values: vec![
                        Some([0.0, 0.0, 0.0, 1.0].into()),
                        Some(vulkano::format::ClearValue::Depth(1.0)),
                    ],
                    ..vulkano::command_buffer::RenderPassBeginInfo::framebuffer(
                        self.framebuffers[image_index].clone(),
                    )
                },
                vulkano::command_buffer::SubpassBeginInfo {
                    contents: vulkano::command_buffer::SubpassContents::Inline,
                    ..Default::default()
                },
            )
            .unwrap();
        builder
            .bind_pipeline_graphics(self.graphics_pipeline.clone())
            .unwrap();
        builder
            .set_viewport_with_count(self.viewports.clone())
            .unwrap();
        builder
            .set_scissor_with_count(self.scissors.clone())
            .unwrap();

        for vulkan_object in self.vulkan_objects.iter() {
            let model = glam::Mat4::from_translation(vulkan_object.1.get_transform().position);
            let mvp = view_projection * model;
            let push_constants = PushConstants::new(mvp);

            let vertex_buffer = vulkan_object.1.get_vertex_buffer().clone();
            let index_buffer = vulkan_object.1.get_index_buffer().clone();
            builder
                .bind_vertex_buffers(0, vertex_buffer.clone())
                .unwrap();
            builder.bind_index_buffer(index_buffer.clone()).unwrap();

            builder
                .bind_descriptor_sets(
                    vulkano::pipeline::PipelineBindPoint::Graphics,
                    self.graphics_pipeline.layout().clone(),
                    0,
                    vulkan_object.1.get_descriptor_set(),
                )
                .unwrap();
            builder
                .push_constants(self.graphics_pipeline.layout().clone(), 0, push_constants)
                .unwrap();
            unsafe {
                builder
                    .draw_indexed(index_buffer.len().try_into().unwrap(), 1, 0, 0, 0)
                    .unwrap()
            };
        }

        builder
            .end_render_pass(vulkano::command_buffer::SubpassEndInfo::default())
            .unwrap();

        builder.build().unwrap()
    }

    pub fn draw_frame(&mut self, camera_transform: Transform) {
        let (image_index, _, acquire_future) =
            vulkano::swapchain::acquire_next_image(self.swapchain.clone(), None).unwrap();
        let view_projection = Vulkan::make_view_projection(
            self.viewports[0].extent[0] / self.viewports[0].extent[1],
            &camera_transform.get_position(),
            &camera_transform.get_rotation(),
        );

        let command_buffer =
            self.create_command_buffer(image_index.try_into().unwrap(), view_projection);
        let future = vulkano::sync::GpuFuture::then_signal_fence_and_flush(
            vulkano::sync::GpuFuture::then_swapchain_present(
                vulkano::sync::GpuFuture::then_execute(
                    acquire_future,
                    self.queue.clone(),
                    command_buffer,
                )
                .unwrap(),
                self.queue.clone(),
                vulkano::swapchain::SwapchainPresentInfo::swapchain_image_index(
                    self.swapchain.clone(),
                    image_index,
                ),
            ),
        );

        match future {
            Ok(fut) => {
                fut.wait(None).unwrap();
            }
            Err(e) => {
                eprintln!("Failed to flush frame: {:?}", e);
            }
        }

        self.window.request_redraw();
    }

    fn make_view_projection(
        aspect_ratio: f32,
        camera_location: &glam::Vec3,
        camera_rotation: &glam::Vec3,
    ) -> glam::Mat4 {
        let rotation_x = glam::Mat4::from_rotation_x(camera_rotation.x);
        let rotation_y = glam::Mat4::from_rotation_y(camera_rotation.y);
        let rotation_z = glam::Mat4::from_rotation_z(camera_rotation.z);
        let rotation = rotation_x * rotation_y * rotation_z;

        let translation = glam::Mat4::from_translation(*camera_location);
        let view = rotation * translation;
        let proj = glam::Mat4::perspective_rh_gl(45.0_f32.to_radians(), aspect_ratio, 0.1, 1000.0);
        let view_projection = proj * view;

        return view_projection;
    }
    fn process_image_data(
        &mut self,
        image_data: gltf::image::Data,
    ) -> (
        Arc<vulkano::image::view::ImageView>,
        Arc<vulkano::image::sampler::Sampler>,
    ) {
        let staging_buffer = vulkano::buffer::Buffer::from_iter(
            self.memory_allocator.clone(),
            vulkano::buffer::BufferCreateInfo {
                usage: vulkano::buffer::BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            vulkano::memory::allocator::AllocationCreateInfo {
                memory_type_filter: vulkano::memory::allocator::MemoryTypeFilter::PREFER_HOST
                    | vulkano::memory::allocator::MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            image_data.pixels,
        )
        .unwrap_or_else(|_| {
            log!(Self, Critical, "Failed to process_image_data");
            panic!()
        });

        let image = vulkano::image::Image::new(
            self.memory_allocator.clone(),
            vulkano::image::ImageCreateInfo {
                image_type: vulkano::image::ImageType::Dim2d,
                format: vulkano::format::Format::R8G8B8A8_SRGB,
                extent: [image_data.width, image_data.height, 1],
                usage: vulkano::image::ImageUsage::TRANSFER_DST
                    | vulkano::image::ImageUsage::SAMPLED,
                ..Default::default()
            },
            vulkano::memory::allocator::AllocationCreateInfo::default(),
        )
        .unwrap_or_else(|_| {
            log!(Self, Critical, "Failed to process_image_data");
            panic!()
        });

        let mut builder = vulkano::command_buffer::AutoCommandBufferBuilder::primary(
            self.command_buffer_allocator.clone(),
            self.queue.queue_family_index(),
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap_or_else(|_| {
            log!(Self, Critical, "Failed to process_image_data");
            panic!()
        });

        builder
            .copy_buffer_to_image(
                vulkano::command_buffer::CopyBufferToImageInfo::buffer_image(
                    staging_buffer,
                    image.clone(),
                ),
            )
            .unwrap_or_else(|_| {
                log!(Self, Critical, "Failed to process_image_data");
                panic!()
            });

        let command_buffer = builder.build().unwrap();
        vulkano::sync::now(self.queue.device().clone())
            .then_execute(self.queue.clone(), command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap_or_else(|_| {
                log!(Self, Critical, "Failed to process_image_data");
                panic!()
            });

        // Image view
        let image_view = vulkano::image::view::ImageView::new_default(image).unwrap();

        // Sampler
        let sampler = vulkano::image::sampler::Sampler::new(
            self.queue.device().clone(),
            vulkano::image::sampler::SamplerCreateInfo {
                mag_filter: vulkano::image::sampler::Filter::Linear,
                min_filter: vulkano::image::sampler::Filter::Linear,
                mipmap_mode: vulkano::image::sampler::SamplerMipmapMode::Linear,
                address_mode: [vulkano::image::sampler::SamplerAddressMode::Repeat; 3],
                ..Default::default()
            },
        )
        .unwrap_or_else(|_| {
            log!(Self, Critical, "Failed to process_image_data");
            panic!()
        });

        (image_view, sampler)
    }
}

impl GraphicsAPI for Vulkan {
    fn render_frame(&self) {
        todo!();
    }
}

#[derive(Debug)]
pub struct VulkanObject {
    vertex_buffer: vulkano::buffer::subbuffer::Subbuffer<[Vertex]>,
    index_buffer: vulkano::buffer::subbuffer::Subbuffer<[u32]>,
    object_transform: Transform,
    texture_descriptor_set: Arc<vulkano::descriptor_set::DescriptorSet>,
}

impl VulkanObject {
    pub fn new(
        vertex_buffer: vulkano::buffer::Subbuffer<[Vertex]>,
        index_buffer: vulkano::buffer::subbuffer::Subbuffer<[u32]>,
        object_transform: Transform,
        texture_descriptor_set: Arc<vulkano::descriptor_set::DescriptorSet>,
    ) -> Self {
        return VulkanObject {
            vertex_buffer,
            index_buffer,
            object_transform,
            texture_descriptor_set,
        };
    }

    pub fn get_transform(&self) -> &Transform {
        return &self.object_transform;
    }

    pub fn get_vertex_buffer(&self) -> &vulkano::buffer::Subbuffer<[Vertex]> {
        return &self.vertex_buffer;
    }

    pub fn get_index_buffer(&self) -> &vulkano::buffer::Subbuffer<[u32]> {
        return &self.index_buffer;
    }

    pub fn get_descriptor_set(&self) -> Arc<vulkano::descriptor_set::DescriptorSet> {
        return self.texture_descriptor_set.clone();
    }
}
