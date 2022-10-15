use std::collections::HashMap;
use std::error::Error;
use std::ffi::CString;
use std::io::{Read, Write};
use rustyline::error::ReadlineError;
use rustyline::Editor;
use scan_fmt::scan_fmt;
use uvc;
use sdl2;
use ash::{vk, Entry, vk::Handle};
use ash::prelude::VkResult;
use std::os::raw::c_char;
use crate::pantilt::*;
use crate::quantity::*;

const LAYER_VALIDATION: *const c_char =
    concat!("VK_LAYER_KHRONOS_validation", "\0")
    as *const str as *const [c_char] as *const c_char;
const LAYER_DEBUG: *const c_char =
    concat!("VK_LAYER_LUNARG_api_dump", "\0")
    as *const str as *const [c_char] as *const c_char;

enum DebugLevel {
    None,
    Validation,
    Debug,
}

struct DebugLayer {
    loader: ash::extensions::ext::DebugUtils,
    messenger: vk::DebugUtilsMessengerEXT,
}

impl DebugLayer {
    extern "system" fn callback(
        message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
        message_type: vk::DebugUtilsMessageTypeFlagsEXT,
        p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
        _p_user_data: *mut std::ffi::c_void,
    ) -> vk::Bool32 {
        let message = unsafe { std::ffi::CStr::from_ptr((*p_callback_data).p_message) };
        let severity = format!("{:?}", message_severity);
        let ty = format!("{:?}", message_type);
        println!("[LOG][{}][{}] {}", severity, ty,
                 std::str::from_utf8(message.to_bytes()).unwrap());
        vk::FALSE
    }
}

impl Drop for DebugLayer {
    fn drop(&mut self) {
        unsafe {
            self.loader.destroy_debug_utils_messenger(self.messenger, None)
        }
    }
}

fn vk_str_to_string(vks: &[c_char; 256]) -> String {
    String::from(std::str::from_utf8(unsafe {
        &*(vks as *const [i8] as *const [u8])
    }).unwrap())
}

struct QueueFamilyIndices {
    pub graphics: Option<u32>,
    pub compute: Option<u32>,
    pub transfer: Option<u32>,
    pub sparse_binding: Option<u32>,
    pub present: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn create(entry: &ash::Entry, instance: &ash::Instance,
                  gpu: vk::PhysicalDevice,
                  surface: vk::SurfaceKHR) -> VkResult<Self> {
        let qfp_list: Vec<vk::QueueFamilyProperties> = unsafe {
            instance.get_physical_device_queue_family_properties(gpu)
        };
        let mut result = QueueFamilyIndices {
            graphics: None,
            compute: None,
            transfer: None,
            sparse_binding: None,
            present: None,
        };

        let surface_inst = ash::extensions::khr::Surface::new(entry, instance);

        for (i, qfp) in qfp_list.iter().enumerate() {
            if qfp.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
                println!("Queue family #{} supports graphics", i);
                result.graphics = Some(i as u32);
            }
            if qfp.queue_flags.contains(vk::QueueFlags::COMPUTE) {
                println!("Queue family #{} supports compute", i);
                result.compute = Some(i as u32);
            }
            if qfp.queue_flags.contains(vk::QueueFlags::TRANSFER) {
                println!("Queue family #{} supports transfer", i);
                result.transfer = Some(i as u32);
            }
            if qfp.queue_flags.contains(vk::QueueFlags::SPARSE_BINDING) {
                println!("Queue family #{} supports sparse binding", i);
                result.sparse_binding = Some(i as u32);
            }
            let is_present = unsafe {
                surface_inst.get_physical_device_surface_support(
                    gpu, i as u32, surface)?
            };
            if is_present {
                println!("Queue family #{} supports presentation", i);
                result.present = Some(i as u32);
            }
        }

        if let Some(i) = result.graphics {
            println!("Selected queue family #{} for graphics", i);
        }
        if let Some(i) = result.compute {
            println!("Selected queue family #{} for compute", i);
        }
        if let Some(i) = result.transfer {
            println!("Selected queue family #{} for transfer", i);
        }
        if let Some(i) = result.sparse_binding {
            println!("Selected queue family #{} for sparse binding", i);
        }
        if let Some(i) = result.present {
            println!("Selected queue family #{} for presentation", i);
        }

        Ok(result)
    }
}

struct SwapChainSupportDetails {
    pub capabilities: vk::SurfaceCapabilitiesKHR,
    pub formats: Vec<vk::SurfaceFormatKHR>,
    pub present_modes: Vec<vk::PresentModeKHR>,
}

impl SwapChainSupportDetails {
    pub fn create(entry: &ash::Entry, instance: &ash::Instance,
                  gpu: vk::PhysicalDevice,
                  surface: vk::SurfaceKHR) -> VkResult<Self> {
        let surface_inst = ash::extensions::khr::Surface::new(entry, instance);
        let capabilities = unsafe {
            surface_inst.get_physical_device_surface_capabilities(gpu, surface)?
        };
        let formats = unsafe {
            surface_inst.get_physical_device_surface_formats(gpu, surface)?
        };
        let present_modes = unsafe {
            surface_inst
                .get_physical_device_surface_present_modes(gpu, surface)?
        };
        Ok(SwapChainSupportDetails { capabilities, formats, present_modes })
    }
}

struct VulkanEngine {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub debug_layer: Option<DebugLayer>,
    pub chosen_gpu: vk::PhysicalDevice,
    pub device: ash::Device,
    pub queue: vk::Queue,
    pub surface: vk::SurfaceKHR,
    pub sdl_context: sdl2::Sdl,
    pub window: sdl2::video::Window,
    pub qfi: QueueFamilyIndices,
    pub swapchain: vk::SwapchainKHR,
    pub swapchain_images: Vec<vk::Image>,
    pub swapchain_format: vk::Format,
    pub swapchain_extent: vk::Extent2D,
    pub swapchain_image_views: Vec<vk::ImageView>,
    pub render_pass: vk::RenderPass,
    pub pipeline_layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
    pub swapchain_framebuffers: Vec<vk::Framebuffer>,
    pub command_pool: vk::CommandPool,
    pub command_buffer: vk::CommandBuffer,
    pub image_available_semaphore: vk::Semaphore,
    pub render_finished_semaphore: vk::Semaphore,
    pub in_flight_fence: vk::Fence,
}

impl VulkanEngine {
    pub fn create(level: DebugLevel) -> VkResult<Self> {
        let sdl_context = sdl2::init().unwrap();
        let video = sdl_context.video().unwrap();
        let mut window_builder =
            sdl2::video::WindowBuilder::new(&video, "monocle", 1280, 720);
        let window = window_builder.vulkan().build().unwrap();

        let vk_layers = match level {
            DebugLevel::None => vec![],
            DebugLevel::Validation => vec![LAYER_VALIDATION],
            DebugLevel::Debug => vec![LAYER_VALIDATION, LAYER_DEBUG],
        };

        let mut messenger_info = match level {
            DebugLevel::None => vk::DebugUtilsMessengerCreateInfoEXT {
                ..Default::default()
            },
            _ => {
                vk::DebugUtilsMessengerCreateInfoEXT {
                    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                        | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                        | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
                    message_type: vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
                    pfn_user_callback: Some(DebugLayer::callback),
                    ..Default::default()
                }
            }
        };

        let entry: Entry = Entry::linked();
        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 2, 0),
            engine_version: 0,
            ..Default::default()
        };
        let extension_names = [
            ash::extensions::ext::DebugUtils::name().as_ptr(),
            ash::extensions::khr::Surface::name().as_ptr(),
            ash::extensions::khr::XlibSurface::name().as_ptr(),
        ];
        let create_info = vk::InstanceCreateInfo::builder()
            .push_next(&mut messenger_info)
            .application_info(&app_info)
            .enabled_layer_names(&vk_layers)
            .enabled_extension_names(&extension_names);
        let instance = unsafe { entry.create_instance(&create_info, None)? };

        let debug_layer = match level {
            DebugLevel::None => None,
            _ => {
                let loader =
                    ash::extensions::ext::DebugUtils::new(&entry, &instance);
                let messenger = unsafe {
                    loader.create_debug_utils_messenger(&messenger_info, None)?
                };
                Some(DebugLayer{loader, messenger})
            },
        };

        let physical_devices: Vec<vk::PhysicalDevice> = unsafe {
            instance.enumerate_physical_devices()?
        };

        println!("Found {} devices", physical_devices.len());

        for (i, dev) in physical_devices.iter().enumerate() {
            let pdp: vk::PhysicalDeviceProperties = unsafe {
                instance.get_physical_device_properties(*dev)
            };
            println!("Device #{}: {}", i, vk_str_to_string(&pdp.device_name));
        }

        let surface: vk::SurfaceKHR = vk::SurfaceKHR::from_raw(
            window.vulkan_create_surface(instance.handle().as_raw() as usize)
                .unwrap());

        let chosen_gpu: vk::PhysicalDevice = physical_devices[0];
        // TODO: check that chosen gpu matches the chosen surface

        let qfi = QueueFamilyIndices::create(
            &entry, &instance, chosen_gpu, surface)?;

        if qfi.graphics != qfi.compute {
            panic!("Graphics queue family is not the same as compute queue family");
        }

        let device_queue_create_info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(qfi.graphics.unwrap())
            .queue_priorities(&[1.0])
            .build();

        let device_queue_create_infos = [device_queue_create_info];

        let device_features = vk::PhysicalDeviceFeatures::builder();

        let device_extensions = [
            ash::extensions::khr::Swapchain::name().as_ptr(),
        ];

        let device_create_info = vk::DeviceCreateInfo::builder()
            .queue_create_infos(&device_queue_create_infos)
            .enabled_features(&device_features)
            .enabled_extension_names(&device_extensions);

        let device = unsafe {
            instance.create_device(chosen_gpu, &device_create_info, None)?
        };

        let queue = unsafe {
            device.get_device_queue(qfi.graphics.unwrap(), 0)
        };

        Ok(VulkanEngine {
            entry,
            instance,
            debug_layer,
            chosen_gpu,
            device,
            queue,
            surface,
            sdl_context,
            window,
            qfi,
            swapchain: vk::SwapchainKHR::null(),
            swapchain_images: vec![],
            swapchain_format: vk::Format::UNDEFINED,
            swapchain_extent: vk::Extent2D::builder().build(),
            swapchain_image_views: vec![],
            render_pass: vk::RenderPass::null(),
            pipeline_layout: vk::PipelineLayout::null(),
            pipeline: vk::Pipeline::null(),
            swapchain_framebuffers: vec![],
            command_pool: vk::CommandPool::null(),
            command_buffer: vk::CommandBuffer::null(),
            image_available_semaphore: vk::Semaphore::null(),
            render_finished_semaphore: vk::Semaphore::null(),
            in_flight_fence: vk::Fence::null(),
        })
    }

    pub fn populate_swapchain(&mut self) -> VkResult<()> {
        let scsd = SwapChainSupportDetails::create(
            &self.entry, &self.instance, self.chosen_gpu, self.surface)?;

        if scsd.formats.is_empty() {
            panic!("The available swapchains had no formats");
        }
        if scsd.present_modes.is_empty() {
            panic!("The available swapchains had no present modes");
        }

        let mut chosen_format = scsd.formats[0];
        for format in scsd.formats {
            if format.format == vk::Format::B8G8R8A8_SRGB {
                if format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR {
                    chosen_format = format;
                    break;
                }
            }
        }

        self.swapchain_format = chosen_format.format;

        println!("Selected swapchain format: {:?}", chosen_format);

        let mut chosen_present_mode = vk::PresentModeKHR::FIFO;
        // for present_mode in scsd.present_modes {
        //     if present_mode == vk::PresentModeKHR::MAILBOX {
        //         chosen_present_mode = present_mode;
        //     }
        // }

        println!("Selected swapchain present mode: {:?}", chosen_present_mode);

        if scsd.capabilities.current_extent.width == u32::MAX {
            panic!("Swapchain current extent is bad");
        }

        self.swapchain_extent = scsd.capabilities.current_extent;

        let mut image_count = scsd.capabilities.min_image_count + 1;

        if scsd.capabilities.max_image_count > 0 {
            if image_count > scsd.capabilities.max_image_count {
                image_count = scsd.capabilities.max_image_count;
            }
        }

        if self.qfi.graphics != self.qfi.present {
            panic!("Presentation queue is not the same as graphics queue");
        }

        let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
            .surface(self.surface)
            .min_image_count(image_count)
            .image_format(chosen_format.format)
            .image_color_space(chosen_format.color_space)
            .image_extent(self.swapchain_extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(scsd.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(chosen_present_mode)
            .clipped(true)
            .old_swapchain(vk::SwapchainKHR::null());

        let swapchain_inst =
            ash::extensions::khr::Swapchain::new(&self.instance, &self.device);

        self.swapchain = unsafe {
            swapchain_inst.create_swapchain(&swapchain_create_info, None)?
        };

        self.swapchain_images = unsafe {
            swapchain_inst.get_swapchain_images(self.swapchain)?
        };

        let mut swapchain_image_views = Vec::new();
        for swapchain_image in &self.swapchain_images {
            let image_view_create_info = vk::ImageViewCreateInfo::builder()
                .image(*swapchain_image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(self.swapchain_format)
                .components(vk::ComponentMapping {
                    r: vk::ComponentSwizzle::IDENTITY,
                    g: vk::ComponentSwizzle::IDENTITY,
                    b: vk::ComponentSwizzle::IDENTITY,
                    a: vk::ComponentSwizzle::IDENTITY,
                })
                .subresource_range(vk::ImageSubresourceRange::builder()
                                   .aspect_mask(vk::ImageAspectFlags::COLOR)
                                   .base_mip_level(0)
                                   .level_count(1)
                                   .base_array_layer(0)
                                   .layer_count(1)
                                   .build());
            swapchain_image_views.push(unsafe {
                self.device.create_image_view(&image_view_create_info, None)?
            });
        }

        self.swapchain_image_views = swapchain_image_views;

        Ok(())
    }

    pub fn create_shader_module(
        &self, code: shaderc::CompilationArtifact
    ) -> VkResult<vk::ShaderModule> {
        let shader_module_create_info = vk::ShaderModuleCreateInfo::builder()
            .code(code.as_binary());
        unsafe {
            self.device.create_shader_module(&shader_module_create_info, None)
        }
    }

    pub fn populate_render_pass(&mut self) -> VkResult<()> {
        let color_attachments = [
            vk::AttachmentDescription::builder()
                .format(self.swapchain_format)
                .samples(vk::SampleCountFlags::TYPE_1)
                .load_op(vk::AttachmentLoadOp::CLEAR)
                .store_op(vk::AttachmentStoreOp::STORE)
                .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
                .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
                .initial_layout(vk::ImageLayout::UNDEFINED)
                .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
                .build()
        ];
        let color_attachment_ref = vk::AttachmentReference::builder()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build();
        let subpasses = [
            vk::SubpassDescription::builder()
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .color_attachments(&[color_attachment_ref])
                .build() // a bit sus
        ];
        let dependencies = [
            vk::SubpassDependency::builder()
                .src_subpass(vk::SUBPASS_EXTERNAL)
                .dst_subpass(0)
                .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .src_access_mask(vk::AccessFlags::empty())
                .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
                .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
                .build()
        ];
        let render_pass_create_info = vk::RenderPassCreateInfo::builder()
            .attachments(&color_attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);
        self.render_pass = unsafe {
            self.device.create_render_pass(&render_pass_create_info, None)?
        };
        Ok(())
    }

    pub fn populate_graphics_pipeline(&mut self) -> VkResult<()> {
        let vertex_shader_source = r#"
          #version 450

          // layout(location = 0) in vec2 pos;
          // layout(location = 0) out vec2 v_position;
          //
          // void main() {
          //   v_position = (pos + 1.0) / 2.0;
          //   gl_Position = vec4(-pos.x, -pos.y, 0.0, 1.0);
          // }

          layout(location = 0) out vec3 fragColor;

          vec2 positions[3] = vec2[](
              vec2(0.0, -0.5),
              vec2(0.5, 0.5),
              vec2(-0.5, 0.5)
          );

          vec3 colors[3] = vec3[](
              vec3(1.0, 0.0, 0.0),
              vec3(0.0, 1.0, 0.0),
              vec3(0.0, 0.0, 1.0)
          );

          void main() {
              gl_Position = vec4(positions[gl_VertexIndex], 0.0, 1.0);
              fragColor = colors[gl_VertexIndex];
          }
        "#;

        let fragment_shader_source = r#"
          #version 450

          // layout(location = 0) in vec2 v_position;
          // layout(location = 0) out vec4 color;
          // layout(binding = 0) uniform sampler2D u_image;
          //
          // void main() {
          //   vec2 pos = v_position;
          //   color = texture(u_image, pos);
          // }

          layout(location = 0) in vec3 fragColor;
          layout(location = 0) out vec4 outColor;

          void main() {
              outColor = vec4(fragColor, 1.0);
          }
        "#;

        let compiler = shaderc::Compiler::new().unwrap();
        let compile_options = shaderc::CompileOptions::new().unwrap();

        let vertex_shader_binary = compiler.compile_into_spirv(
            vertex_shader_source, shaderc::ShaderKind::Vertex, "shader.glsl",
            "main", Some(&compile_options)).unwrap();
        let fragment_shader_binary = compiler.compile_into_spirv(
            fragment_shader_source, shaderc::ShaderKind::Fragment,
            "shader.glsl", "main", Some(&compile_options)).unwrap();
        println!("Successfully compiled shaders");

        let vertex_shader_module =
            self.create_shader_module(vertex_shader_binary)?;
        let fragment_shader_module =
            self.create_shader_module(fragment_shader_binary)?;
        println!("Successfully created shader modules");

        let main_string = CString::new("main").unwrap();

        let pipeline_shader_create_infos = [
            vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::VERTEX)
                .module(vertex_shader_module)
                .name(&main_string)
                .build(),
            vk::PipelineShaderStageCreateInfo::builder()
                .stage(vk::ShaderStageFlags::FRAGMENT)
                .module(fragment_shader_module)
                .name(&main_string)
                .build(),
        ];

        let pipeline_vertex_input_state_create_info =
            vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(&[])
            .vertex_attribute_descriptions(&[]);

        let pipeline_input_assembly_state_create_info =
            vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let viewports = [
            vk::Viewport::builder()
                .x(0.0).y(0.0)
                .width(self.swapchain_extent.width as f32)
                .height(self.swapchain_extent.height as f32)
                .min_depth(0.0)
                .max_depth(0.0)
                .build()
        ];
        let scissors = [
            vk::Rect2D::builder()
                .offset(vk::Offset2D { x: 0, y: 0 })
                .extent(self.swapchain_extent)
                .build()
        ];

        let pipeline_viewport_state_create_info =
            vk::PipelineViewportStateCreateInfo::builder()
            .viewports(&viewports)
            .scissors(&scissors);

        let pipeline_rasterization_state_create_info =
            vk::PipelineRasterizationStateCreateInfo::builder()
            .depth_clamp_enable(false)
            .rasterizer_discard_enable(false)
            .polygon_mode(vk::PolygonMode::FILL)
            .line_width(1.0)
            .cull_mode(vk::CullModeFlags::BACK)
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false);

        let pipeline_multisample_state_create_info =
            vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let pipeline_color_blend_attachment_states = [
            vk::PipelineColorBlendAttachmentState::builder()
                .color_write_mask(vk::ColorComponentFlags::R
                                  | vk::ColorComponentFlags::G
                                  | vk::ColorComponentFlags::B
                                  | vk::ColorComponentFlags::A)
                .blend_enable(false)
                .build()
        ];

        let pipeline_color_blend_state_create_info =
            vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .attachments(&pipeline_color_blend_attachment_states);

        let pipeline_layout_create_info =
            vk::PipelineLayoutCreateInfo::builder();

        self.pipeline_layout = unsafe {
            self.device.create_pipeline_layout(
                &pipeline_layout_create_info, None)?
        };

        let graphics_pipeline_create_infos = [
            vk::GraphicsPipelineCreateInfo::builder()
                .stages(&pipeline_shader_create_infos)
                .vertex_input_state(&pipeline_vertex_input_state_create_info)
                .input_assembly_state(&pipeline_input_assembly_state_create_info)
                .viewport_state(&pipeline_viewport_state_create_info)
                .rasterization_state(&pipeline_rasterization_state_create_info)
                .multisample_state(&pipeline_multisample_state_create_info)
                .color_blend_state(&pipeline_color_blend_state_create_info)
                .layout(self.pipeline_layout)
                .render_pass(self.render_pass)
                .subpass(0)
                .build()
        ];

        let gp_result = unsafe {
            self.device.create_graphics_pipelines(
                vk::PipelineCache::null(),
                &graphics_pipeline_create_infos,
                None,
            )
        };

        self.pipeline = match gp_result {
            Ok(pipelines) => pipelines[0],
            Err((_, err)) => Err(err)?,
        };

        println!("Successfully created graphics pipeline");

        unsafe {
            self.device.destroy_shader_module(vertex_shader_module, None);
            self.device.destroy_shader_module(fragment_shader_module, None);
        }

        Ok(())
    }

    pub fn populate_framebuffers(&mut self) -> VkResult<()> {
        self.swapchain_framebuffers.resize(self.swapchain_image_views.len(),
                                           vk::Framebuffer::null());
        for (i, image_view) in self.swapchain_image_views.iter().enumerate() {
            let attachments = [*image_view];
            let framebuffer_create_info = vk::FramebufferCreateInfo::builder()
                .render_pass(self.render_pass)
                .attachments(&attachments)
                .width(self.swapchain_extent.width)
                .height(self.swapchain_extent.height)
                .layers(1);
            self.swapchain_framebuffers[i] = unsafe {
                self.device.create_framebuffer(&framebuffer_create_info, None)?
            };
        }

        println!("Successfully created framebuffers");

        Ok(())
    }

    pub fn populate_command_pool_and_buffer(&mut self) -> VkResult<()> {
        let command_pool_create_info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(self.qfi.graphics.unwrap());
        self.command_pool = unsafe {
            self.device.create_command_pool(&command_pool_create_info, None)?
        };

        println!("Successfully created command pool");

        let command_buffer_allocate_info =
            vk::CommandBufferAllocateInfo::builder()
            .command_pool(self.command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        self.command_buffer = unsafe {
            self.device
                .allocate_command_buffers(&command_buffer_allocate_info)?[0]
        };

        println!("Successfully created command buffer");

        Ok(())
    }

    pub fn record_command_buffer(&self, image_index: usize) -> VkResult<()> {
        let command_buffer_begin_info = vk::CommandBufferBeginInfo::builder();
        unsafe {
            self.device.begin_command_buffer(self.command_buffer,
                                             &command_buffer_begin_info)?;
        }

        let clear_values = [
            vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [1.0, 0.5, 0.5, 1.0],
                },
            }
        ];
        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(self.render_pass)
            .framebuffer(self.swapchain_framebuffers[image_index])
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.swapchain_extent,
            })
            .clear_values(&clear_values);
        unsafe {
            self.device.cmd_begin_render_pass(self.command_buffer,
                                              &render_pass_begin_info,
                                              vk::SubpassContents::INLINE);
        }

        unsafe {
            self.device.cmd_bind_pipeline(self.command_buffer,
                                          vk::PipelineBindPoint::GRAPHICS,
                                          self.pipeline);
            self.device.cmd_draw(self.command_buffer, 3, 1, 0, 0);
        }

        unsafe {
            self.device.cmd_end_render_pass(self.command_buffer);
            self.device.end_command_buffer(self.command_buffer)?;
        }

        Ok(())
    }

    pub fn populate_synchronization_primitives(&mut self) -> VkResult<()> {
        {
            let semaphore_create_info = vk::SemaphoreCreateInfo::builder();
            self.image_available_semaphore = unsafe {
                self.device.create_semaphore(&semaphore_create_info, None)?
            };
        }

        {
            let semaphore_create_info = vk::SemaphoreCreateInfo::builder();
            self.render_finished_semaphore = unsafe {
                self.device.create_semaphore(&semaphore_create_info, None)?
            };
        }

        println!("Created semaphores");

        {
            let fence_create_info = vk::FenceCreateInfo::builder()
                .flags(vk::FenceCreateFlags::SIGNALED);
            self.in_flight_fence = unsafe {
                self.device.create_fence(&fence_create_info, None)?
            };
        }

        println!("Created fences");

        Ok(())
    }

    pub fn draw_frame(&self) -> VkResult<()> {
        unsafe {
            self.device.wait_for_fences(
                &[self.in_flight_fence], true, u64::MAX)?;
            self.device.reset_fences(&[self.in_flight_fence])?;
        }

        let swapchain_inst =
            ash::extensions::khr::Swapchain::new(&self.instance, &self.device);

        let (image_index, _is_suboptimal) = unsafe {
            swapchain_inst.acquire_next_image(self.swapchain, u64::MAX,
                                              self.image_available_semaphore,
                                              vk::Fence::null())?
        };

        unsafe {
            self.device.reset_command_buffer(
                self.command_buffer,
                vk::CommandBufferResetFlags::empty())?;
            self.record_command_buffer(image_index as usize)?;
        }

        let wait_semaphores = [self.image_available_semaphore];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let command_buffers = [self.command_buffer];
        let signal_semaphores = [self.render_finished_semaphore];
        let submit_infos = [
            vk::SubmitInfo::builder()
                .wait_semaphores(&wait_semaphores)
                .wait_dst_stage_mask(&wait_stages)
                .command_buffers(&command_buffers)
                .signal_semaphores(&signal_semaphores)
                .build()
        ];
        unsafe {
            self.device.queue_submit(self.queue, &submit_infos,
                                     self.in_flight_fence)?;
        }

        let swapchains = [self.swapchain];
        let image_indices = [image_index];
        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&signal_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);
        unsafe {
            swapchain_inst.queue_present(self.queue, &present_info)?;
        }

        Ok(())
    }

    pub fn destroy(self) {
        unsafe {
            self.device.destroy_semaphore(self.image_available_semaphore, None);
            self.device.destroy_semaphore(self.render_finished_semaphore, None);
            self.device.destroy_fence(self.in_flight_fence, None);

            self.device.destroy_command_pool(self.command_pool, None);

            for framebuffer in self.swapchain_framebuffers {
                self.device.destroy_framebuffer(framebuffer, None);
            }

            self.device.destroy_pipeline(self.pipeline, None);
            self.device.destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_render_pass(self.render_pass, None);

            for image_view in self.swapchain_image_views {
                self.device.destroy_image_view(image_view, None);
            }

            let swapchain_inst = ash::extensions::khr::Swapchain::new(
                &self.instance, &self.device);
            swapchain_inst.destroy_swapchain(self.swapchain, None);
        }
    }
}

pub fn main() {
    let mut engine = VulkanEngine::create(DebugLevel::Validation).unwrap();
    engine.populate_swapchain().unwrap();
    engine.populate_render_pass().unwrap();
    engine.populate_graphics_pipeline().unwrap();
    engine.populate_framebuffers().unwrap();
    engine.populate_command_pool_and_buffer().unwrap();
    engine.populate_synchronization_primitives().unwrap();
    let quit_mutex = std::sync::Arc::new(std::sync::Mutex::new(false));
    let handle = {
        let quit_mutex = std::sync::Arc::clone(&quit_mutex);
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(1000));
            *(quit_mutex.lock().unwrap()) = true;
        })
    };
    let mut event_pump = engine.sdl_context.event_pump().unwrap();
    let mut counter = 0;
    loop {
        if *(quit_mutex.lock().unwrap()) {
            break;
        }
        engine.draw_frame().unwrap();
        counter += 1;
    }
    unsafe {
        engine.device.device_wait_idle().unwrap();
    }
    println!("Displayed {} frames", counter);
    handle.join().unwrap();
    engine.destroy();
}


// pub fn parse_command(m: &mut HashMap<String, (Azimuth, Altitude)>,
//                      conn: &mut Connection,
//                      string: &str) {
//     if string == "track" {
//     } else if string == "untrack" {
//     }
// }
//
// pub fn main() {
//     let mut conn = Connection::new().unwrap();
//     // conn.port.write("e".as_bytes()).unwrap();
//     // return;
//     let mut rl = Editor::<()>::new();
//     let mut m: HashMap<String, (Azimuth, Altitude)> = HashMap::new();
//
//     loop {
//         let readline = rl.readline("monocle> ");
//
//         let line;
//         match readline {
//             Ok(l) => {
//                 if l.len() == 0 { continue; }
//                 rl.add_history_entry(l.as_str());
//                 line = l.clone();
//             },
//             Err(ReadlineError::Interrupted) => {
//                 println!("CTRL-C");
//                 return;
//             },
//             Err(ReadlineError::Eof) => {
//                 println!("CTRL-D");
//                 return;
//             },
//             Err(err) => {
//                 println!("Error: {:?}", err);
//                 return;
//             },
//         };
//
//         parse_command(&mut m, &mut conn, &line);
//     }
// }
