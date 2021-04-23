mod loader;
mod material;
mod scene;
mod sprite;

use material::Material;
use scene::Scene;
use sprite::Sprite;

use crate::{
    ffi::vk,
    math::Vector2,
};
use std::{
    ffi::CStr,
    mem::MaybeUninit,
    ops::Deref,
    path::Path,
    ptr::{copy_nonoverlapping, null, null_mut, NonNull},
};

pub use loader::InstanceTable;
use loader::{DeviceTable, Loader};

const GRAPHICS: usize = 0;
const PRESENTATION: usize = 1;
const TRANSFER: usize = 2;

struct Device {
    queue_family_indices: [u32; 3],
    queues: Queues,
    device: *mut vk::Device,
    destroy_device: vk::DestroyDevice,
}

impl Device {
    fn new(
        loader: &Loader,
        table: &InstanceTable,
        physical_device: *mut vk::PhysicalDevice,
        surface: *mut vk::SurfaceKHR,
    ) -> (Self, DeviceTable) {
        let queue_family_indices = select_queue_family_indices(table, surface, physical_device);
        let vk_device = create_device(table, physical_device, queue_family_indices);

        let device_table = loader.load_device_functions(vk_device);
        let queues = Queues::retrieve(&device_table, vk_device, queue_family_indices);
        let device = Self {
            queue_family_indices,
            queues,
            device: vk_device,
            destroy_device: table.destroy_device,
        };
        return (device, device_table);
    }
}

impl Deref for Device {
    type Target = *mut vk::Device;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        (self.destroy_device)(self.device, null());
    }
}

struct Instance {
    destroy_instance: vk::DestroyInstance,
    instance: *mut vk::Instance,
}

impl Deref for Instance {
    type Target = *mut vk::Instance;

    fn deref(&self) -> &Self::Target {
        &self.instance
    }
}

impl Drop for Instance {
    fn drop(&mut self) {
        (self.destroy_instance)(self.instance, null());
    }
}

struct ShaderModule {
    shader_module: *mut vk::ShaderModule,
    device: *mut vk::Device,
    destroy_shader_module: vk::DestroyShaderModule,
}

impl ShaderModule {
    fn from_path<P>(table: &DeviceTable, device: *mut vk::Device, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let file = std::fs::read(path).unwrap();
        let info = vk::ShaderModuleCreateInfo {
            stype: vk::StructureType::ShaderModuleCreateInfo,
            next: null(),
            flags: 0,
            code_size: file.len(),
            code: file.as_ptr() as *const u32,
        };

        let mut shader_module = null_mut();
        (table.create_shader_module)(device, &info, null(), &mut shader_module);

        return Self {
            shader_module,
            device,
            destroy_shader_module: table.destroy_shader_module,
        };
    }
}

impl Deref for ShaderModule {
    type Target = *mut vk::ShaderModule;

    fn deref(&self) -> &Self::Target {
        &self.shader_module
    }
}

impl Drop for ShaderModule {
    fn drop(&mut self) {
        (self.destroy_shader_module)(self.device, self.shader_module, null());
    }
}

struct SurfaceKHR {
    instance: *mut vk::Instance,
    destroy_surface_khr: vk::DestroySurfaceKHR,
    surface: *mut vk::SurfaceKHR,
}

impl Deref for SurfaceKHR {
    type Target = *mut vk::SurfaceKHR;

    fn deref(&self) -> &Self::Target {
        &self.surface
    }
}

impl Drop for SurfaceKHR {
    fn drop(&mut self) {
        (self.destroy_surface_khr)(self.instance, self.surface, null())
    }
}

/// A `DeviceMemory`-backed vulkan `Buffer`, Memory-Backed Buffer (MBB).
struct MBB {
    buffer: *mut vk::Buffer,
    memory: NonNull<vk::DeviceMemory>,
    size: vk::DeviceSize,
    flags: vk::MemoryPropertyFlags,
}

impl MBB {
    fn create(
        instance_table: &InstanceTable,
        device_table: &DeviceTable,
        physical_device: *mut vk::PhysicalDevice,
        device: *mut vk::Device,
        size: vk::DeviceSize,
        usage: vk::BufferUsageFlags,
        flags: vk::MemoryPropertyFlags,
    ) -> Self {
        let buffer_info = vk::BufferCreateInfo {
            stype: vk::StructureType::BufferCreateInfo,
            next: null(),
            flags: 0,
            size,
            usage,
            sharing_mode: vk::SharingMode::Exclusive,
            queue_family_index_count: 0,
            queue_family_indices: null(),
        };
        let mut buffer = null_mut();
        (device_table.create_buffer)(device, &buffer_info, null(), &mut buffer);

        let mut requirements = MaybeUninit::uninit();
        (device_table.get_buffer_memory_requirements)(device, buffer, requirements.as_mut_ptr());
        let requirements = unsafe { requirements.assume_init() };

        let memory = allocate_memory(
            instance_table,
            device_table,
            physical_device,
            device,
            requirements,
            flags,
        )
        .expect("Failed to allocate memory!");

        (device_table.bind_buffer_memory)(device, buffer, memory.as_ptr(), 0);

        let mbb = Self {
            buffer,
            memory,
            size,
            flags,
        };
        return mbb;
    }

    fn fill(&self, table: &DeviceTable, device: *mut vk::Device, data: *const u8) {
        debug_assert!(
            ((self.flags & vk::MemoryPropertyFlagBits::HostCoherent as u32) != 0)
                && ((self.flags & vk::MemoryPropertyFlagBits::HostVisible as u32) != 0)
        );

        let mut p = null_mut();
        (table.map_memory)(device, self.memory.as_ptr(), 0, self.size, 0, &mut p);
        unsafe {
            copy_nonoverlapping(data, p as *mut u8, self.size as usize);
        }
        (table.unmap_memory)(device, self.memory.as_ptr());
    }

    fn destroy(self, table: &DeviceTable, device: *mut vk::Device) {
        (table.free_memory)(device, self.memory.as_ptr(), null());
        (table.destroy_buffer)(device, self.buffer, null());
    }

    fn write_region(
        &self,
        table: &DeviceTable,
        device: *mut vk::Device,
        offset: vk::DeviceSize,
        size: vk::DeviceSize,
        data: *const u8,
    ) {
        debug_assert!(
            ((self.flags & vk::MemoryPropertyFlagBits::HostCoherent as u32) != 0)
                && ((self.flags & vk::MemoryPropertyFlagBits::HostVisible as u32) != 0)
        );

        let mut address = null_mut();
        (table.map_memory)(device, self.memory.as_ptr(), offset, size, 0, &mut address);
        unsafe {
            copy_nonoverlapping(data, address as *mut u8, size as usize);
        }
        (table.unmap_memory)(device, self.memory.as_ptr());
    }
}

struct MBI {
    image: NonNull<vk::Image>,
    memory: NonNull<vk::DeviceMemory>,
}

impl MBI {
    fn create(
        instance_table: &InstanceTable,
        device_table: &DeviceTable,
        physical_device: *mut vk::PhysicalDevice,
        device: *mut vk::Device,
        format: vk::Format,
        extent: vk::Extent2D,
        tiling: vk::ImageTiling,
        usage: vk::ImageUsageFlags,
        layout: vk::ImageLayout,
        flags: vk::MemoryPropertyFlags,
    ) -> Self {
        let image = create_image(device_table, device, format, extent, tiling, usage, layout)
            .expect("Failed to create image!");

        let requirements = {
            let mut requirements = MaybeUninit::uninit();
            (device_table.get_image_memory_requirements)(
                device,
                image.as_ptr(),
                requirements.as_mut_ptr(),
            );
            unsafe { requirements.assume_init() }
        };

        let memory = allocate_memory(
            instance_table,
            device_table,
            physical_device,
            device,
            requirements,
            flags,
        )
        .expect("Failed to allocate memory!");

        (device_table.bind_image_memory)(device, image.as_ptr(), memory.as_ptr(), 0);

        return Self { image, memory };
    }
}

struct PresentationSync {
    drawing_finished: Box<[*mut vk::Fence]>,
    image_acquired: Box<[*mut vk::Semaphore]>,
    image_ready: Box<[*mut vk::Semaphore]>,
    current_frame: usize,
    num_images: usize,
}

impl PresentationSync {
    fn create(table: &DeviceTable, device: *mut vk::Device, num_images: usize) -> Self {
        let mut drawing_finished = vec![null_mut(); num_images].into_boxed_slice();
        let mut image_acquired = vec![null_mut(); num_images].into_boxed_slice();
        let mut image_ready = vec![null_mut(); num_images].into_boxed_slice();

        (0..num_images).for_each(|i| {
            drawing_finished[i] =
                create_fence(table, device, vk::FenceCreateFlagBits::Signaled as u32);
            image_acquired[i] = create_semaphore(table, device, 0);
            image_ready[i] = create_semaphore(table, device, 0);
        });

        let sync = Self {
            drawing_finished,
            image_acquired,
            image_ready,
            num_images,
            current_frame: 0,
        };
        return sync;
    }

    fn destroy(self, table: &DeviceTable, device: *mut vk::Device) {
        (0..self.num_images).for_each(|i| {
            (table.destroy_fence)(device, self.drawing_finished[i], null());
            (table.destroy_semaphore)(device, self.image_acquired[i], null());
            (table.destroy_semaphore)(device, self.image_ready[i], null());
        });
    }
}

struct RenderTarget {
    extent: vk::Extent2D,
    framebuffers: Box<[*mut vk::Framebuffer]>,
    views: Box<[*mut vk::ImageView]>,
    images: Box<[*mut vk::Image]>,
    render_pass: *mut vk::RenderPass,
    swapchain: *mut vk::SwapchainKHR,
}

impl RenderTarget {
    fn create(
        device_table: &DeviceTable,
        instance_table: &InstanceTable,
        physical_device: *mut vk::PhysicalDevice,
        device: *mut vk::Device,
        surface: *mut vk::SurfaceKHR,
        window_extent: vk::Extent2D,
    ) -> Self {
        let capabilities = get_capabilities(&instance_table, physical_device, surface);
        let image_extent = select_extent(capabilities, window_extent);
        let surface_format = get_swapchain_format(&instance_table, physical_device, surface);
        let swapchain = create_swapchain(
            &device_table,
            surface,
            device,
            capabilities,
            surface_format,
            image_extent,
        );
        let images = get_swapchain_images(&device_table, device, swapchain);

        let views: Box<[*mut vk::ImageView]> = images
            .iter()
            .map(|&image| create_image_view(&device_table, device, image, surface_format.format))
            .collect();
        let render_pass = create_render_pass(&device_table, device, surface_format.format);
        let framebuffers: Box<[*mut vk::Framebuffer]> = views
            .iter()
            .map(|&view| create_framebuffer(&device_table, device, render_pass, view, image_extent))
            .collect();

        return Self {
            extent: image_extent,
            framebuffers,
            views,
            images,
            render_pass,
            swapchain,
        };
    }

    pub fn destroy(&mut self, device_table: &DeviceTable, device: *mut vk::Device) {
        self.framebuffers
            .iter()
            .for_each(|&f| (device_table.destroy_framebuffer)(device, f, null()));

        (device_table.destroy_render_pass)(device, self.render_pass, null());

        self.views
            .iter()
            .for_each(|&view| (device_table.destroy_image_view)(device, view, null()));

        (device_table.destroy_swapchain_khr)(device, self.swapchain, null());
    }
}

// TODO: Please give this a better name
struct PerFrameResources {
    command_pool: *mut vk::CommandPool,
    primary: *mut vk::CommandBuffer,
    secondaries: Vec<*mut vk::CommandBuffer>,
    descriptor_pool: *mut vk::DescriptorPool,
    descriptor_sets: Vec<*mut vk::DescriptorSet>,
}

impl PerFrameResources {
    fn allocate_descriptors_and_secondaries(
        &mut self,
        device_table: &DeviceTable,
        device: *mut vk::Device,
        set_layout: *mut vk::DescriptorSetLayout,
        count: usize,
    ) -> Box<[usize]> {
        if count == 0 {
            return Box::new([]);
        }
        
        debug_assert_eq!(self.secondaries.len(), self.descriptor_sets.len());

        let offset = self.secondaries.len();

        let secondaries = allocate_command_buffers(
            device_table,
            device,
            self.command_pool,
            vk::CommandBufferLevel::Secondary,
            count,
        )
        .expect("Failed to allocate secondary command buffers!");
        let descriptor_sets = allocate_descriptor_sets(
            device_table,
            device,
            self.descriptor_pool,
            set_layout,
            count,
        )
        .expect("Failed to allocate descriptor sets!");

        self.secondaries.extend_from_slice(&secondaries);
        self.descriptor_sets.extend_from_slice(&descriptor_sets);
        return (offset..self.secondaries.len()).collect();
    }

    fn create(table: &DeviceTable, device: *mut vk::Device, graphics_family_index: u32) -> Self {
        let command_pool = create_command_pool(table, device, 0, graphics_family_index);
        let primary =
            allocate_command_buffer(table, device, command_pool, vk::CommandBufferLevel::Primary);
        let secondaries = Vec::new();

        let descriptor_count = 4;
        let pool_sizes = [vk::DescriptorPoolSize {
            dtype: vk::DescriptorType::CombinedImageSampler,
            descriptor_count,
        }];
        let descriptor_pool = descriptor_pool_create(table, device, descriptor_count, &pool_sizes);
        let descriptor_sets = Vec::new();

        let res = Self {
            command_pool,
            primary,
            secondaries,
            descriptor_pool,
            descriptor_sets,
        };
        return res;
    }

    fn destroy(&mut self, table: &DeviceTable, device: *mut vk::Device) {
        (table.destroy_command_pool)(device, self.command_pool, null());
        self.primary = null_mut();
        self.secondaries.clear();

        (table.destroy_descriptor_pool)(device, self.descriptor_pool, null());
        self.descriptor_sets.clear();
    }

    fn reset(&mut self, table: &DeviceTable, device: *mut vk::Device) {
        (table.reset_command_pool)(device, self.command_pool, 0);
        (table.free_command_buffers)(
            device,
            self.command_pool,
            self.secondaries.len() as u32,
            self.secondaries.as_ptr(),
        );
        self.secondaries.clear();

        // Resetting the descriptor pool returns all descriptor sets back to the pool, unlike
        // resetting a command pool!
        (table.reset_descriptor_pool)(device, self.descriptor_pool, null());
        self.descriptor_sets.clear();
    }
}

struct Texture {
    image: NonNull<vk::Image>,
    view: *mut vk::ImageView,
    sampler: NonNull<vk::Sampler>,
    memory: NonNull<vk::DeviceMemory>,
}

struct Queues {
    graphics: *mut vk::Queue,
    presentation: *mut vk::Queue,
    transfer: *mut vk::Queue,
}

impl Queues {
    fn retrieve(
        device_table: &DeviceTable,
        device: *mut vk::Device,
        queue_family_indices: [u32; 3],
    ) -> Self {
        let get_device_queue = device_table.get_device_queue;

        let mut graphics = null_mut();
        get_device_queue(device, queue_family_indices[GRAPHICS], 0, &mut graphics);

        let mut presentation = null_mut();
        get_device_queue(
            device,
            queue_family_indices[PRESENTATION],
            0,
            &mut presentation,
        );

        let mut transfer = null_mut();
        get_device_queue(device, queue_family_indices[TRANSFER], 0, &mut transfer);

        let queues = Queues {
            graphics,
            presentation,
            transfer,
        };
        return queues;
    }
}

pub struct Renderer {
    frame_resources: Box<[PerFrameResources]>,
    scene: Scene,
    sprites: Vec<Sprite>,
    textures: Vec<Texture>,
    vertex_buffer: MBB,
    transfer_pool: *mut vk::CommandPool,
    presentation_sync: PresentationSync,
    material_sprite: Material,
    material_text: Material,
    render_target: RenderTarget,
    device: Device,
    physical_device: NonNull<vk::PhysicalDevice>,
    surface: SurfaceKHR,
    _instance: Instance,
    device_table: DeviceTable,
    instance_table: InstanceTable,
    _vulkan: crate::ffi::Library,
}

impl Renderer {
    pub(crate) fn begin_scene(&mut self, r: f32, g: f32, b: f32) -> Option<usize> {
        let current_frame = self.presentation_sync.current_frame;
        let resources = &mut self.frame_resources[current_frame];
        let image_acquired = self.presentation_sync.image_acquired[current_frame];
        let drawing_finished = self.presentation_sync.drawing_finished[current_frame];

        let index = acquire_image(
            &self.device_table,
            *self.device,
            self.render_target.swapchain,
            100_000_000,
            Some(image_acquired),
            None,
        )?;

        fence_wait_reset(&self.device_table, *self.device, drawing_finished);
        resources.reset(&self.device_table, *self.device);
        command_buffer_begin_primary(
            &self.device_table,
            resources.primary,
            vk::CommandBufferUsageFlagBits::OneTimeSubmit as u32,
        );

        self.scene = Scene::default();
        self.clear(index, r, g, b);
        return Some(index as usize);
    }

    pub(crate) fn clear(&self, index: u32, r: f32, g: f32, b: f32) {
        let info = vk::RenderPassBeginInfo {
            stype: vk::StructureType::RenderPassBeginInfo,
            next: null(),
            render_pass: self.render_target.render_pass,
            framebuffer: self.render_target.framebuffers[index as usize],
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.render_target.extent,
            },
            clear_value_count: 1,
            clear_values: &vk::ClearValue {
                color: vk::ClearColorValue {
                    float: [r, g, b, 1.0],
                },
            },
        };
        (self.device_table.cmd_begin_render_pass)(
            self.frame_resources[self.presentation_sync.current_frame].primary,
            &info,
            vk::SubpassContents::SecondaryCommandBuffers,
        );
    }

    pub(crate) fn create_sprite_from_path<P>(&mut self, path: P) -> usize
    where
        P: AsRef<Path>,
    {
        let (texture_index, width, height) = self.load_png_from_path(path);
        let sprite = Sprite {
            texture_index,
            width,
            height,
        };

        let index = self.sprites.len();
        self.sprites.push(sprite);
        return index;
    }

    pub(crate) fn deinit(mut self) {
        (self.device_table.device_wait_idle)(*self.device);

        self.sprites.clear();
        (0..self.textures.len()).for_each(|i| self.unload_texture(i));

        for frame_resource in self.frame_resources.iter_mut() {
            frame_resource.destroy(&self.device_table, *self.device);
        }

        self.vertex_buffer.destroy(&self.device_table, *self.device);
        self.presentation_sync
            .destroy(&self.device_table, *self.device);

        (self.device_table.destroy_command_pool)(*self.device, self.transfer_pool, null());
        self.render_target.destroy(&self.device_table, *self.device);
    }

    pub(crate) fn draw(&mut self, sprite_index: usize, position: Vector2) {
        let vertex_data =
            self.sprites[sprite_index].generate_vertex_data(position, self.render_target.extent);
        self.scene.insert_sprite(&vertex_data, sprite_index);
    }

    pub(crate) fn end_scene(&mut self) {
        let current_frame = self.presentation_sync.current_frame;
        let resources = &mut self.frame_resources[current_frame];
        let primary = resources.primary;

        // Write the data into the vertex buffer
        self.vertex_buffer.write_region(
            &self.device_table,
            *self.device,
            0,
            self.scene.data.len() as vk::DeviceSize,
            self.scene.data.as_ptr(),
        );

        // Allocate secondary command buffers and a corresponding descriptor set for each
        let count = self.scene.sprite.len();
        let indices = resources.allocate_descriptors_and_secondaries(
            &self.device_table,
            *self.device,
            self.material_sprite.set_layout,
            count,
        );

        // Record all sprite commands
        let sids = self.scene.sprite.iter();
        let idxs = indices.iter();

        // Update the descriptor set and record the command buffer
        for (sid, &idx) in sids.zip(idxs) {
            descriptor_set_update_sampled_image(
                &self.device_table,
                *self.device,
                resources.descriptor_sets[idx],
                &self.textures[self.sprites[sid.index].texture_index],
            );

            let secondary = resources.secondaries[idx];
            command_buffer_begin_secondary(
                &self.device_table,
                secondary,
                self.render_target.render_pass,
                vk::CommandBufferUsageFlagBits::OneTimeSubmit as u32,
            );
            set_scissor_and_viewport(&self.device_table, secondary, self.render_target.extent);
            bind_graphics_pipeline(&self.device_table, secondary, self.material_sprite.pipeline);
            bind_sampled_image_descriptor(
                &self.device_table,
                secondary,
                self.material_sprite.pipeline_layout,
                resources.descriptor_sets[idx],
            );
            bind_vertex_buffer(
                &self.device_table,
                secondary,
                &self.vertex_buffer,
                sid.offset as vk::DeviceSize,
            );
            (self.device_table.cmd_draw)(secondary, 6, 1, 0, 0);
            (self.device_table.end_command_buffer)(secondary);
        }

        // Recording glyph draw
        let glyph_indices = resources.allocate_descriptors_and_secondaries(
            &self.device_table,
            *self.device,
            self.material_text.set_layout,
            self.scene.glyphs.len(),
        );

        let gids = self.scene.glyphs.iter();
        let idxs = glyph_indices.iter();
        for (gid, &idx) in gids.zip(idxs) {
            descriptor_set_update_sampled_image(
                &self.device_table,
                *self.device,
                resources.descriptor_sets[idx],
                &self.textures[gid.texture_index],
            );

            let secondary = resources.secondaries[idx];
            command_buffer_begin_secondary(
                &self.device_table,
                secondary,
                self.render_target.render_pass,
                vk::CommandBufferUsageFlagBits::OneTimeSubmit as u32,
            );

            set_scissor_and_viewport(&self.device_table, secondary, self.render_target.extent);
            bind_graphics_pipeline(&self.device_table, secondary, self.material_text.pipeline);
            bind_sampled_image_descriptor(
                &self.device_table,
                secondary,
                self.material_text.pipeline_layout,
                resources.descriptor_sets[idx],
            );

            bind_vertex_buffer(
                &self.device_table,
                secondary,
                &self.vertex_buffer,
                gid.offset as vk::DeviceSize,
            );
            (self.device_table.cmd_draw)(secondary, gid.num_vertices as u32, 1, 0, 0);
            (self.device_table.end_command_buffer)(secondary);
        }

        // Execute all recorder secondary command buffers
        (self.device_table.cmd_execute_commands)(
            primary,
            resources.secondaries.len() as u32,
            resources.secondaries.as_ptr(),
        );

        (self.device_table.cmd_end_render_pass)(primary);

        command_buffer_end_and_submit(
            &self.device_table,
            primary,
            self.device.queues.graphics,
            Some(self.presentation_sync.image_acquired[current_frame]),
            vk::PipelineStageFlagBits::ColorAttachmentOutput as u32,
            Some(self.presentation_sync.image_ready[current_frame]),
            Some(self.presentation_sync.drawing_finished[current_frame]),
        );
    }

    pub fn sprite_half_dimensions(&self, sprite_index: usize) -> (f32, f32) {
        return self.sprites[sprite_index].pixels_to_ndc(self.render_target.extent);
    }

    pub fn init(window: &crate::Window) -> Self {
        let mut loader = Loader::init();

        // Instance creation and instance function loading
        let (instance, instance_table) = {
            let vk_instance = create_instance(&loader).expect("Failed to create instance!");
            let instance_table = loader.load_instance_functions(vk_instance);
            let instance = Instance {
                destroy_instance: instance_table.destroy_instance,
                instance: vk_instance,
            };
            (instance, instance_table)
        };

        let physical_device = select_physical_device(&instance_table, *instance)
            .expect("Failed to select physical device!");

        // Surface creation
        let surface = {
            let surface = window
                .create_surface(&instance_table, *instance)
                .expect("Failed to create VkSurfaceKHR!");
            SurfaceKHR {
                instance: *instance,
                destroy_surface_khr: instance_table.destroy_surface_khr,
                surface,
            }
        };
        let (device, device_table) =
            Device::new(&loader, &instance_table, physical_device.as_ptr(), *surface);
        let _vulkan = loader.take_library();

        // Render target
        let render_target = RenderTarget::create(
            &device_table,
            &instance_table,
            physical_device.as_ptr(),
            *device,
            *surface,
            window.dimensions_inner().into(),
        );

        let material_sprite =
            Material::sprite(&device_table, *device, render_target.render_pass).unwrap();
        let material_text =
            Material::text(&device_table, *device, render_target.render_pass).unwrap();
        let num_images = render_target.images.len();
        // Synchronization primitives required for presentation
        let presentation_sync = PresentationSync::create(&device_table, *device, num_images);

        let vertex_buffer = MBB::create(
            &instance_table,
            &device_table,
            physical_device.as_ptr(),
            *device,
            5 * 1024 * 1024,
            vk::BufferUsageFlagBits::VertexBuffer as u32,
            vk::MemoryPropertyFlagBits::HostCoherent as u32
                | vk::MemoryPropertyFlagBits::HostVisible as u32,
        );

        let transfer_pool = create_command_pool(
            &device_table,
            *device,
            0,
            device.queue_family_indices[TRANSFER],
        );
        let frame_resources: Box<[PerFrameResources]> = (0..num_images)
            .map(|_| {
                PerFrameResources::create(
                    &device_table,
                    *device,
                    device.queue_family_indices[GRAPHICS],
                )
            })
            .collect();

        let renderer = Self {
            frame_resources,
            scene: Scene::default(),
            sprites: Vec::new(),
            textures: Vec::new(),
            vertex_buffer,
            transfer_pool,
            material_sprite,
            material_text,
            presentation_sync,
            render_target,
            device,
            physical_device,
            surface,
            _instance: instance,
            device_table,
            instance_table,
            _vulkan,
        };
        return renderer;
    }

    pub fn load_png_from_path<P>(&mut self, path: P) -> (usize, u32, u32)
    where
        P: AsRef<Path>,
    {
        let (width, height, pixels) = crate::read_png(path);
        let index = self.load_texture(
            width,
            height,
            (4 * width * height) as vk::DeviceSize,
            &pixels,
            vk::Format::R8G8B8A8UNORM,
        );
        return (index, width, height);
    }

    pub fn load_texture(
        &mut self,
        width: u32,
        height: u32,
        size: vk::DeviceSize,
        pixels: &[u8],
        format: vk::Format,
    ) -> usize {
        // Prepare staging buffer
        let staging = MBB::create(
            &self.instance_table,
            &self.device_table,
            self.physical_device.as_ptr(),
            *self.device,
            size,
            vk::BufferUsageFlagBits::TransferSource as u32,
            vk::MemoryPropertyFlagBits::HostCoherent as u32
                | vk::MemoryPropertyFlagBits::HostVisible as u32,
        );
        staging.fill(&self.device_table, *self.device, pixels.as_ptr());

        // Create texture image and memory
        let image_usage = vk::ImageUsageFlagBits::TransferDestination as u32
            | vk::ImageUsageFlagBits::Sampled as u32;
        let MBI { image, memory } = MBI::create(
            &self.instance_table,
            &self.device_table,
            self.physical_device.as_ptr(),
            *self.device,
            format,
            (width, height).into(),
            vk::ImageTiling::Optimal,
            image_usage,
            vk::ImageLayout::Undefined,
            vk::MemoryPropertyFlagBits::DeviceLocal as u32,
        );

        let view = create_image_view(&self.device_table, *self.device, image.as_ptr(), format);
        let sampler =
            create_sampler(&self.device_table, *self.device).expect("Failed to create sampler!");

        // Prepare command buffer for recording
        let transfer_buffer = allocate_command_buffer(
            &self.device_table,
            *self.device,
            self.transfer_pool,
            vk::CommandBufferLevel::Primary,
        );
        command_buffer_begin_primary(
            &self.device_table,
            transfer_buffer,
            vk::CommandBufferUsageFlagBits::OneTimeSubmit as u32,
        );

        // Initial barrier transitioning to ImageLayout::TransferDestinationOptimal
        {
            let barrier = ImageMemoryBarrier {
                src_access_mask: 0,
                dst_access_mask: vk::AccessFlagBits::TransferWrite as u32,
                old_layout: vk::ImageLayout::Undefined,
                new_layout: vk::ImageLayout::TransferDestinationOptimal,
                src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
            };
            record_image_memory_barrier(
                &self.device_table,
                transfer_buffer,
                vk::PipelineStageFlagBits::TopOfPipe as u32,
                vk::PipelineStageFlagBits::Transfer as u32,
                barrier,
                image.as_ptr(),
            );
        }

        let image_subresource = vk::ImageSubresourceLayers {
            aspect_mask: vk::ImageAspectFlagBits::Color as u32,
            mip_level: 0,
            base_array_layer: 0,
            layer_count: 1,
        };
        let region = vk::BufferImageCopy {
            buffer_offset: 0,
            buffer_row_length: width,
            buffer_image_height: height,
            image_subresource,
            image_offset: (0, 0, 0).into(),
            image_extent: (width, height, 1).into(),
        };
        (self.device_table.cmd_copy_buffer_to_image)(
            transfer_buffer,
            staging.buffer,
            image.as_ptr(),
            vk::ImageLayout::TransferDestinationOptimal,
            1,
            &region,
        );

        // Final barrier transitioning to ImageLayout::ShaderReadOnlyOptimal
        {
            let barrier = ImageMemoryBarrier {
                src_access_mask: vk::AccessFlagBits::TransferWrite as u32,
                dst_access_mask: vk::AccessFlagBits::ShaderRead as u32,
                old_layout: vk::ImageLayout::TransferDestinationOptimal,
                new_layout: vk::ImageLayout::ShaderReadOnlyOptimal,
                src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
            };
            record_image_memory_barrier(
                &self.device_table,
                transfer_buffer,
                vk::PipelineStageFlagBits::Transfer as u32,
                vk::PipelineStageFlagBits::FragmentShader as u32,
                barrier,
                image.as_ptr(),
            );
        }

        let fence = create_fence(&self.device_table, *self.device, 0);
        command_buffer_end_and_submit(
            &self.device_table,
            transfer_buffer,
            self.device.queues.transfer,
            None,
            0,
            None,
            Some(fence),
        );
        fence_wait_reset(&self.device_table, *self.device, fence);

        (self.device_table.free_command_buffers)(
            *self.device,
            self.transfer_pool,
            1,
            &transfer_buffer,
        );
        staging.destroy(&self.device_table, *self.device);
        fence_destroy(&self.device_table, *self.device, fence);

        let texture = Texture {
            image,
            view,
            sampler,
            memory,
        };

        let index = self.textures.len();
        self.textures.push(texture);
        return index;
    }

    pub fn present(&mut self, index: usize) {
        let info = vk::PresentInfoKHR {
            stype: vk::StructureType::PresentInfoKHR,
            next: null(),
            wait_semaphore_count: 1,
            wait_semaphores: &self.presentation_sync.image_ready
                [self.presentation_sync.current_frame],
            swapchain_count: 1,
            swapchains: &self.render_target.swapchain,
            image_indices: &(index as u32),
            results: null_mut(),
        };
        (self.device_table.queue_present_khr)(self.device.queues.presentation, &info);

        self.presentation_sync.current_frame += 1;
        if self.presentation_sync.num_images <= self.presentation_sync.current_frame {
            self.presentation_sync.current_frame = 0;
        }
    }

    pub fn resize(&mut self, window: &crate::Window) {
        (self.device_table.device_wait_idle)(*self.device);
        self.render_target.destroy(&self.device_table, *self.device);
        self.render_target = RenderTarget::create(
            &self.device_table,
            &self.instance_table,
            self.physical_device.as_ptr(),
            *self.device,
            *self.surface,
            window.dimensions_inner().into(),
        );
    }

    pub fn unload_texture(&mut self, index: usize) {
        let texture = &self.textures[index];
        (self.device_table.destroy_sampler)(*self.device, texture.sampler.as_ptr(), null());
        (self.device_table.destroy_image_view)(*self.device, texture.view, null());
        (self.device_table.destroy_image)(*self.device, texture.image.as_ptr(), null());
        (self.device_table.free_memory)(*self.device, texture.memory.as_ptr(), null());
    }
}

fn acquire_image(
    device_table: &DeviceTable,
    device: *mut vk::Device,
    swapchain: *mut vk::SwapchainKHR,
    timeout: u64,
    semaphore: Option<*mut vk::Semaphore>,
    fence: Option<*mut vk::Fence>,
) -> Option<u32> {
    let mut index = 0;
    let result = (device_table.acquire_next_image_khr)(
        device,
        swapchain,
        timeout,
        semaphore.unwrap_or_else(null_mut),
        fence.unwrap_or_else(null_mut),
        &mut index,
    );

    return match result {
        vk::Result::Success => Some(index),
        _ => None,
    };
}

fn allocate_command_buffer(
    table: &DeviceTable,
    device: *mut vk::Device,
    command_pool: *mut vk::CommandPool,
    level: vk::CommandBufferLevel,
) -> *mut vk::CommandBuffer {
    let command_buffer_info = vk::CommandBufferAllocateInfo {
        stype: vk::StructureType::CommandBufferAllocateInfo,
        next: null(),
        command_pool,
        level,
        command_buffer_count: 1,
    };

    let mut command_buffer = null_mut();
    (table.allocate_command_buffers)(device, &command_buffer_info, &mut command_buffer);
    return command_buffer;
}

fn allocate_command_buffers(
    device_table: &DeviceTable,
    device: *mut vk::Device,
    command_pool: *mut vk::CommandPool,
    level: vk::CommandBufferLevel,
    count: usize,
) -> Option<Box<[*mut vk::CommandBuffer]>> {
    let info = vk::CommandBufferAllocateInfo {
        stype: vk::StructureType::CommandBufferAllocateInfo,
        next: null(),
        command_pool,
        level,
        command_buffer_count: count as u32,
    };

    let mut command_buffers = vec![null_mut(); count].into_boxed_slice();
    let result =
        (device_table.allocate_command_buffers)(device, &info, command_buffers.as_mut_ptr());
    if result != vk::Result::Success || command_buffers.iter().any(|c| c.is_null()) {
        return None;
    }

    return Some(command_buffers);
}

fn allocate_descriptor_sets(
    device_table: &DeviceTable,
    device: *mut vk::Device,
    pool: *mut vk::DescriptorPool,
    set_layout: *mut vk::DescriptorSetLayout,
    count: usize,
) -> Option<Box<[*mut vk::DescriptorSet]>> {
    let set_layouts = vec![set_layout; count];
    let info = vk::DescriptorSetAllocateInfo {
        stype: vk::StructureType::DescriptorSetAllocateInfo,
        next: null(),
        descriptor_pool: pool,
        descriptor_set_count: count as u32,
        set_layouts: set_layouts.as_ptr(),
    };

    let mut sets = vec![null_mut(); count].into_boxed_slice();
    let result = (device_table.allocate_descriptor_sets)(device, &info, sets.as_mut_ptr());
    if result != vk::Result::Success || sets.iter().any(|set| set.is_null()) {
        return None;
    }

    return Some(sets);
}

fn allocate_memory(
    instance_table: &InstanceTable,
    device_table: &DeviceTable,
    physical_device: *mut vk::PhysicalDevice,
    device: *mut vk::Device,
    requirements: vk::MemoryRequirements,
    flags: vk::MemoryPropertyFlags,
) -> Option<NonNull<vk::DeviceMemory>> {
    let memory_type_index = find_memory_type(
        instance_table,
        physical_device,
        requirements.memory_type_bits,
        flags,
    )
    .expect("Failed to find appropriate memory type!");

    let allocate_info = vk::MemoryAllocateInfo {
        stype: vk::StructureType::MemoryAllocateInfo,
        next: null(),
        allocation_size: requirements.size,
        memory_type_index,
    };
    let mut memory = null_mut();
    (device_table.allocate_memory)(device, &allocate_info, null(), &mut memory);
    return NonNull::new(memory);
}

fn bind_sampled_image_descriptor(
    device_table: &DeviceTable,
    command_buffer: *mut vk::CommandBuffer,
    layout: *mut vk::PipelineLayout,
    descriptor_set: *mut vk::DescriptorSet,
) {
    (device_table.cmd_bind_descriptor_sets)(
        command_buffer,
        vk::PipelineBindPoint::Graphics,
        layout,
        0,
        1,
        &descriptor_set,
        0,
        null(),
    );
}

fn bind_graphics_pipeline(
    dt: &DeviceTable,
    command_buffer: *mut vk::CommandBuffer,
    pipeline: *mut vk::Pipeline,
) {
    (dt.cmd_bind_pipeline)(command_buffer, vk::PipelineBindPoint::Graphics, pipeline);
}

fn bind_vertex_buffer(
    device_table: &DeviceTable,
    command_buffer: *mut vk::CommandBuffer,
    mbb: &MBB,
    offset: vk::DeviceSize,
) {
    (device_table.cmd_bind_vertex_buffers)(command_buffer, 0, 1, &mbb.buffer, &offset);
}

fn command_buffer_begin_primary(
    device_table: &DeviceTable,
    command_buffer: *mut vk::CommandBuffer,
    usage: vk::CommandBufferUsageFlags,
) {
    let info = vk::CommandBufferBeginInfo {
        stype: vk::StructureType::CommandBufferBeginInfo,
        next: null(),
        usage,
        inheritance_info: null(),
    };
    (device_table.begin_command_buffer)(command_buffer, &info);
}

fn command_buffer_begin_secondary(
    device_table: &DeviceTable,
    command_buffer: *mut vk::CommandBuffer,
    render_pass: *mut vk::RenderPass,
    usage: vk::CommandBufferUsageFlags,
) {
    let inheritance_info = vk::CommandBufferInheritanceInfo {
        stype: vk::StructureType::CommandBufferInheritanceInfo,
        next: null(),
        render_pass,
        subpass: 0,
        framebuffer: null_mut(),
        occlusion_query_enable: false as u32,
        query_flags: 0,
        pipeline_statistics: 0,
    };
    let info = vk::CommandBufferBeginInfo {
        stype: vk::StructureType::CommandBufferBeginInfo,
        next: null(),
        usage: usage | vk::CommandBufferUsageFlagBits::RenderPassContinue as u32,
        inheritance_info: &inheritance_info,
    };
    (device_table.begin_command_buffer)(command_buffer, &info);
}

fn command_buffer_end_and_submit(
    table: &DeviceTable,
    command_buffer: *mut vk::CommandBuffer,
    queue: *mut vk::Queue,
    wait_semaphore: Option<*mut vk::Semaphore>,
    wait_dst_stage_mask: vk::PipelineStageFlags,
    signal_semaphore: Option<*mut vk::Semaphore>,
    fence: Option<*mut vk::Fence>,
) {
    (table.end_command_buffer)(command_buffer);

    let (wait_semaphore_count, wait_semaphore): (u32, *mut _) = if let Some(s) = wait_semaphore {
        (1, s)
    } else {
        (0, null_mut())
    };

    let (signal_semaphore_count, signal_semaphore): (u32, *mut _) =
        if let Some(s) = signal_semaphore {
            (1, s)
        } else {
            (0, null_mut())
        };

    let submit_info = vk::SubmitInfo {
        stype: vk::StructureType::SubmitInfo,
        next: null(),
        wait_semaphore_count,
        wait_semaphores: &wait_semaphore,
        wait_dst_stage_mask: &wait_dst_stage_mask,
        command_buffer_count: 1,
        command_buffers: &command_buffer,
        signal_semaphore_count,
        signal_semaphores: &signal_semaphore,
    };

    (table.queue_submit)(queue, 1, &submit_info, fence.unwrap_or(null_mut()));
}

fn create_command_pool(
    table: &DeviceTable,
    device: *mut vk::Device,
    flags: vk::CommandPoolCreateFlags,
    queue_family_index: u32,
) -> *mut vk::CommandPool {
    let pool_info = vk::CommandPoolCreateInfo {
        stype: vk::StructureType::CommandPoolCreateInfo,
        next: null(),
        flags,
        queue_family_index,
    };

    let mut command_pool = null_mut();
    (table.create_command_pool)(device, &pool_info, null(), &mut command_pool);
    return command_pool;
}

fn create_device(
    table: &InstanceTable,
    physical_device: *mut vk::PhysicalDevice,
    queue_family_indices: [u32; 3],
) -> *mut vk::Device {
    let priority = 1.0;
    let queue_infos: Box<[vk::DeviceQueueCreateInfo]> = queue_family_indices
        .iter()
        .enumerate()
        .filter(|&(i, current)| {
            queue_family_indices[..i]
                .iter()
                .all(|previous| *previous != *current)
        })
        .map(|(unique_index, _)| vk::DeviceQueueCreateInfo {
            stype: vk::StructureType::DeviceQueueCreateInfo,
            next: null(),
            flags: 0,
            queue_family_index: unique_index as u32,
            queue_count: 1,
            queue_priorities: &priority,
        })
        .collect();

    let swapchain = "VK_KHR_swapchain\0";
    let swapchain_c = CStr::from_bytes_with_nul(&swapchain.as_bytes()).unwrap();
    let extensions = [swapchain_c.as_ptr()];

    let info = vk::DeviceCreateInfo {
        stype: vk::StructureType::DeviceCreateInfo,
        next: null(),
        flags: 0,
        queue_create_info_count: queue_infos.len() as u32,
        queue_create_infos: queue_infos.as_ptr(),
        enabled_layer_count: 0,      // Deprecated
        enabled_layer_names: null(), // Deprecated
        enabled_extension_count: extensions.len() as u32,
        enabled_extension_names: extensions.as_ptr(),
        enabled_features: null(),
    };

    let mut device = null_mut();
    (table.create_device)(physical_device, &info, null(), &mut device);

    return device;
}

fn create_fence(
    table: &DeviceTable,
    device: *mut vk::Device,
    flags: vk::FenceCreateFlags,
) -> *mut vk::Fence {
    let info = vk::FenceCreateInfo {
        stype: vk::StructureType::FenceCreateInfo,
        next: null(),
        flags,
    };

    let mut fence = null_mut();
    (table.create_fence)(device, &info, null(), &mut fence);
    return fence;
}

fn create_framebuffer(
    table: &DeviceTable,
    device: *mut vk::Device,
    render_pass: *mut vk::RenderPass,
    view: *mut vk::ImageView,
    image_extent: vk::Extent2D,
) -> *mut vk::Framebuffer {
    let info = vk::FramebufferCreateInfo {
        stype: vk::StructureType::FramebufferCreateInfo,
        next: null(),
        flags: 0,
        render_pass,
        attachment_count: 1,
        attachments: &view,
        width: image_extent.width,
        height: image_extent.height,
        layers: 1,
    };

    let mut framebuffer = null_mut();
    (table.create_framebuffer)(device, &info, null(), &mut framebuffer);
    return framebuffer;
}

fn create_image(
    table: &DeviceTable,
    device: *mut vk::Device,
    format: vk::Format,
    extent: vk::Extent2D,
    tiling: vk::ImageTiling,
    usage: vk::ImageUsageFlags,
    initial_layout: vk::ImageLayout,
) -> Option<NonNull<vk::Image>> {
    match initial_layout {
        vk::ImageLayout::Preinitialized | vk::ImageLayout::Undefined => {}
        _ => return None,
    }

    let info = vk::ImageCreateInfo {
        stype: vk::StructureType::ImageCreateInfo,
        next: null(),
        flags: 0,
        image_type: vk::ImageType::TwoDimensional,
        format,
        extent: (extent.width, extent.height, 1).into(),
        mip_levels: 1,
        array_layers: 1,
        samples: vk::SampleCountFlagBits::One as u32,
        tiling,
        usage,
        sharing_mode: vk::SharingMode::Exclusive,
        queue_family_index_count: 0,
        queue_family_indices: null(),
        initial_layout,
    };

    let mut image = null_mut();
    (table.create_image)(device, &info, null(), &mut image);
    return NonNull::new(image);
}

fn create_image_view(
    table: &DeviceTable,
    device: *mut vk::Device,
    image: *mut vk::Image,
    format: vk::Format,
) -> *mut vk::ImageView {
    let subresource_range = vk::ImageSubresourceRange {
        aspect_mask: vk::ImageAspectFlagBits::Color as u32,
        base_mip_level: 0,
        mip_levels: 1,
        base_array_layer: 0,
        array_layers: 1,
    };
    let info = vk::ImageViewCreateInfo {
        stype: vk::StructureType::ImageViewCreateInfo,
        next: null(),
        flags: vk::ImageViewCreateFlags::None,
        image,
        view_type: vk::ImageViewType::TwoDimensional,
        format,
        components: vk::ComponentMapping {
            r: vk::ComponentSwizzle::Identity,
            g: vk::ComponentSwizzle::Identity,
            b: vk::ComponentSwizzle::Identity,
            a: vk::ComponentSwizzle::Identity,
        },
        subresource_range,
    };
    let mut view = null_mut();
    (table.create_image_view)(device, &info, null(), &mut view);

    return view;
}

// TODO: Make this function less sneaky!
fn create_instance(loader: &Loader) -> Option<*mut vk::Instance> {
    let create_instance = loader.load_vk_create_instance();
    let application_name = "Pong!\0";
    let application_name_c = CStr::from_bytes_with_nul(application_name.as_bytes()).unwrap();

    let engine_name = "rose\0";
    let engine_name_c = CStr::from_bytes_with_nul(engine_name.as_bytes()).unwrap();

    let app_info = vk::ApplicationInfo {
        stype: vk::StructureType::ApplicationInfo,
        next: null(),
        application_name: application_name_c.as_ptr(),
        application_version: crate::make_version!(0, 1, 0),
        engine_name: engine_name_c.as_ptr(),
        engine_version: crate::make_version!(0, 1, 0),
        api_version: vk::API_VERSION_1_0,
    };

    let validation_layer = "VK_LAYER_KHRONOS_validation\0";
    let validation_layer_c = CStr::from_bytes_with_nul(validation_layer.as_bytes()).unwrap();
    let layers = [validation_layer_c.as_ptr()];

    let surface = "VK_KHR_surface\0";
    let surface_c = CStr::from_bytes_with_nul(surface.as_bytes()).unwrap();
    let extensions = [
        surface_c.as_ptr(),
        crate::Window::surface_extension_name().as_ptr(),
    ];

    let info = vk::InstanceCreateInfo {
        stype: vk::StructureType::InstanceCreateInfo,
        next: null(),
        flags: 0,
        application_info: &app_info,
        enabled_layer_count: layers.len() as u32,
        enabled_layer_names: layers.as_ptr(),
        enabled_extension_count: extensions.len() as u32,
        enabled_extension_names: extensions.as_ptr(),
    };

    let mut instance = null_mut();
    let result = create_instance(&info, null_mut(), &mut instance);
    if result != vk::Result::Success || instance.is_null() {
        return None;
    } else {
        return Some(instance);
    }
}

fn create_render_pass(
    table: &DeviceTable,
    device: *mut vk::Device,
    format: vk::Format,
) -> *mut vk::RenderPass {
    let attachment = vk::AttachmentDescription {
        flags: 0,
        format,
        samples: vk::SampleCountFlagBits::One,
        load_op: vk::AttachmentLoadOp::Clear,
        store_op: vk::AttachmentStoreOp::Store,
        stencil_load_op: vk::AttachmentLoadOp::DontCare,
        stencil_store_op: vk::AttachmentStoreOp::DontCare,
        initial_layout: vk::ImageLayout::Undefined,
        final_layout: vk::ImageLayout::PresentSourceKHR,
    };
    let color_attachment = vk::AttachmentReference {
        attachment: 0,
        layout: vk::ImageLayout::ColorAttachmentOptimal,
    };
    let subpass = vk::SubpassDescription {
        flags: 0,
        pipeline_bind_point: vk::PipelineBindPoint::Graphics,
        input_attachment_count: 0,
        input_attachments: null(),
        color_attachment_count: 1,
        color_attachments: &color_attachment,
        resolve_attachments: null(),
        depth_stencil_attachment: null(),
        preserve_attachment_count: 0,
        preserve_attachments: null(),
    };
    let dependency = vk::SubpassDependency {
        source_subpass: vk::SUBPASS_EXTERNAL,
        destination_subpass: 0,
        source_stage_mask: vk::PipelineStageFlagBits::ColorAttachmentOutput as u32,
        destination_stage_mask: vk::PipelineStageFlagBits::ColorAttachmentOutput as u32,
        source_access_mask: 0,
        destination_access_mask: vk::AccessFlagBits::ColorAttachmentWrite as u32,
        dependency_flags: vk::DependencyFlags::None,
    };
    let info = vk::RenderPassCreateInfo {
        stype: vk::StructureType::RenderPassCreateInfo,
        next: null(),
        flags: 0,
        attachment_count: 1,
        attachments: &attachment,
        subpass_count: 1,
        subpasses: &subpass,
        dependency_count: 1,
        dependencies: &dependency,
    };
    let mut render_pass = null_mut();
    (table.create_render_pass)(device, &info, null(), &mut render_pass);
    return render_pass;
}

fn create_sampler(table: &DeviceTable, device: *mut vk::Device) -> Option<NonNull<vk::Sampler>> {
    let info = vk::SamplerCreateInfo {
        stype: vk::StructureType::SamplerCreateInfo,
        next: null(),
        flags: 0,
        mag_filter: vk::Filter::Linear,
        min_filter: vk::Filter::Linear,
        mipmap_mode: vk::SamplerMipmapMode::Linear,
        address_mode_u: vk::SamplerAddressMode::ClampToBorder,
        address_mode_v: vk::SamplerAddressMode::ClampToBorder,
        address_mode_w: vk::SamplerAddressMode::ClampToBorder,
        mip_lod_bias: 0.0,
        anisotropy_enable: false as u32,
        max_anisotropy: 0.0,
        compare_enable: true as u32,
        compare_op: vk::CompareOp::Equal,
        min_lod: 0.0,
        max_lod: 0.0,
        border_color: vk::BorderColor::FloatOpaqueBlack,
        unnormalized_coordinates: false as u32,
    };

    let mut sampler = null_mut();
    (table.create_sampler)(device, &info, null(), &mut sampler);
    return NonNull::new(sampler);
}

fn create_semaphore(
    table: &DeviceTable,
    device: *mut vk::Device,
    flags: vk::SemaphoreCreateFlags,
) -> *mut vk::Semaphore {
    let info = vk::SemaphoreCreateInfo {
        stype: vk::StructureType::SemaphoreCreateInfo,
        next: null(),
        flags,
    };

    let mut semaphore = null_mut();
    (table.create_semaphore)(device, &info, null(), &mut semaphore);
    return semaphore;
}

fn create_swapchain(
    table: &DeviceTable,
    surface: *mut vk::SurfaceKHR,
    device: *mut vk::Device,
    capabilities: vk::SurfaceCapabilitiesKHR,
    surface_format: vk::SurfaceFormatKHR,
    image_extent: vk::Extent2D,
) -> *mut vk::SwapchainKHR {
    let min_image_count = u32::min(
        capabilities.min_image_count,
        u32::max(2, capabilities.max_image_count),
    );

    let info = vk::SwapchainCreateInfoKHR {
        stype: vk::StructureType::SwapchainCreateInfoKHR,
        next: null(),
        flags: vk::SwapchainCreateFlagsKHR::None,
        surface,
        min_image_count,
        image_format: surface_format.format,
        image_color_space: surface_format.color_space,
        image_extent,
        image_array_layers: 1,
        image_usage: vk::ImageUsageFlagBits::ColorAttachment,
        image_sharing_mode: vk::SharingMode::Exclusive,
        queue_family_index_count: 0,
        queue_family_indices: null(),
        pre_transform: capabilities.current_transform,
        composite_alpha: vk::CompositeAlphaFlagsKHR::Opaque, // TODO: Can get this from capabilities
        present_mode: vk::PresentModeKHR::Fifo,
        clipped: true as u32,
        old_swapchain: null_mut(),
    };

    let mut swapchain = null_mut();
    (table.create_swapchain_khr)(device, &info, null(), &mut swapchain);
    return swapchain;
}

fn descriptor_pool_create(
    dt: &DeviceTable,
    device: *mut vk::Device,
    max_sets: u32,
    pool_sizes: &[vk::DescriptorPoolSize],
) -> *mut vk::DescriptorPool {
    let info = vk::DescriptorPoolCreateInfo {
        stype: vk::StructureType::DescriptorPoolCreateInfo,
        next: null(),
        flags: 0,
        max_sets,
        pool_size_count: pool_sizes.len() as u32,
        pool_sizes: pool_sizes.as_ptr(),
    };

    let mut descriptor_pool = null_mut();
    (dt.create_descriptor_pool)(device, &info, null(), &mut descriptor_pool);
    return descriptor_pool;
}

fn descriptor_set_update_sampled_image(
    device_table: &DeviceTable,
    device: *mut vk::Device,
    descriptor_set: *mut vk::DescriptorSet,
    texture: &Texture,
) {
    let image_info = vk::DescriptorImageInfo {
        sampler: texture.sampler.as_ptr(),
        image_view: texture.view,
        image_layout: vk::ImageLayout::ShaderReadOnlyOptimal,
    };
    let write = vk::WriteDescriptorSet {
        stype: vk::StructureType::WriteDescriptorSet,
        next: null(),
        dst_set: descriptor_set,
        dst_binding: 0,
        dst_array_element: 0,
        descriptor_count: 1,
        descriptor_type: vk::DescriptorType::CombinedImageSampler,
        image_info: &image_info,
        buffer_info: null(),
        texel_buffer_view: null(),
    };
    (device_table.update_descriptor_sets)(device, 1, &write, 0, null());
}

fn get_swapchain_images(
    table: &DeviceTable,
    device: *mut vk::Device,
    swapchain: *mut vk::SwapchainKHR,
) -> Box<[*mut vk::Image]> {
    let mut count = 0;
    (table.get_swapchain_images_khr)(device, swapchain, &mut count, null_mut());
    let mut images = vec![null_mut(); count as usize].into_boxed_slice();
    (table.get_swapchain_images_khr)(device, swapchain, &mut count, images.as_mut_ptr());

    return images;
}

fn fence_wait_reset(device_table: &DeviceTable, device: *mut vk::Device, fence: *mut vk::Fence) {
    (device_table.wait_for_fences)(device, 1, &fence, true as u32, u64::MAX);
    (device_table.reset_fences)(device, 1, &fence);
}

fn fence_destroy(device_table: &DeviceTable, device: *mut vk::Device, fence: *mut vk::Fence) {
    (device_table.destroy_fence)(device, fence, null());
}

fn find_memory_type(
    instance_table: &InstanceTable,
    physical_device: *mut vk::PhysicalDevice,
    memory_type_bits: u32,
    flags: vk::MemoryPropertyFlags,
) -> Option<u32> {
    let mut properties: vk::PhysicalDeviceMemoryProperties = unsafe { std::mem::zeroed() };
    (instance_table.get_physical_device_memory_properties)(physical_device, &mut properties);

    for i in 0..properties.memory_type_count {
        if (memory_type_bits & (1u32 << i)) == (1u32 << i) {
            if (properties.memory_types[i as usize].property_flags & flags) == flags {
                return Some(i);
            }
        }
    }

    return None;
}

fn find_presentation_queue(
    table: &InstanceTable,
    surface: *mut vk::SurfaceKHR,
    physical_device: *mut vk::PhysicalDevice,
    num_queue_families: u32,
) -> Option<u32> {
    return (0..num_queue_families).find(|&i| {
        let mut presentation_supported = false as u32;
        (table.get_physical_device_surface_support_khr)(
            physical_device,
            i,
            surface,
            &mut presentation_supported,
        );

        presentation_supported != 0
    });
}

fn find_queue(properties: &[vk::QueueFamilyProperties], bit: vk::QueueFlagBits) -> Option<u32> {
    let value = bit as u32;
    return properties
        .iter()
        .enumerate()
        .find(|(_, &prop)| (prop.queue_flags & value) == value)
        .map(|(i, _)| i as u32);
}

fn get_capabilities(
    table: &InstanceTable,
    physical_device: *mut vk::PhysicalDevice,
    surface: *mut vk::SurfaceKHR,
) -> vk::SurfaceCapabilitiesKHR {
    let mut capabilities = MaybeUninit::uninit();
    (table.get_physical_device_surface_capabilities_khr)(
        physical_device,
        surface,
        capabilities.as_mut_ptr(),
    );

    return unsafe { capabilities.assume_init() };
}

fn get_queue_family_properties(
    table: &InstanceTable,
    physical_device: *mut vk::PhysicalDevice,
) -> Box<[vk::QueueFamilyProperties]> {
    let mut count = 0;
    (table.get_physical_device_queue_family_properties)(physical_device, &mut count, null_mut());
    let mut properties = vec![unsafe { std::mem::zeroed() }; count as usize].into_boxed_slice();
    (table.get_physical_device_queue_family_properties)(
        physical_device,
        &mut count,
        properties.as_mut_ptr(),
    );

    return properties;
}

fn get_swapchain_format(
    table: &InstanceTable,
    physical_device: *mut vk::PhysicalDevice,
    surface: *mut vk::SurfaceKHR,
) -> vk::SurfaceFormatKHR {
    let mut count = 1;
    let mut format = MaybeUninit::uninit();
    (table.get_physical_device_surface_formats_khr)(
        physical_device,
        surface,
        &mut count,
        format.as_mut_ptr(),
    );

    return unsafe { format.assume_init() };
}

fn select_extent(
    capabilities: vk::SurfaceCapabilitiesKHR,
    window_extent: vk::Extent2D,
) -> vk::Extent2D {
    let width = if capabilities.current_extent.width == 0xFFFF_FFFF {
        u32::max(
            capabilities.min_image_extent.width,
            u32::min(capabilities.max_image_extent.width, window_extent.width),
        )
    } else {
        capabilities.current_extent.width
    };

    let height = if capabilities.current_extent.height == 0xFFFF_FFFF {
        u32::max(
            capabilities.min_image_extent.height,
            u32::min(capabilities.max_image_extent.height, window_extent.height),
        )
    } else {
        capabilities.current_extent.height
    };

    return vk::Extent2D { width, height };
}

fn select_physical_device(
    instance_table: &InstanceTable,
    instance: *mut vk::Instance,
) -> Option<NonNull<vk::PhysicalDevice>> {
    let mut count = 1;

    let mut physical_device = null_mut();
    (instance_table.enumerate_physical_devices)(instance, &mut count, &mut physical_device);
    return NonNull::new(physical_device);
}

fn select_queue_family_indices(
    table: &InstanceTable,
    surface: *mut vk::SurfaceKHR,
    physical_device: *mut vk::PhysicalDevice,
) -> [u32; 3] {
    let properties = get_queue_family_properties(&table, physical_device);
    let graphics = find_queue(&properties, vk::QueueFlagBits::Graphics).unwrap();
    let presentation =
        find_presentation_queue(&table, surface, physical_device, properties.len() as u32).unwrap();
    let transfer = find_queue(&properties, vk::QueueFlagBits::Transfer).unwrap();

    return [graphics, presentation, transfer];
}

fn set_scissor_and_viewport(
    dt: &DeviceTable,
    command_buffer: *mut vk::CommandBuffer,
    extent: vk::Extent2D,
) {
    let viewport = vk::Viewport {
        x: 0.0,
        y: 0.0,
        width: extent.width as f32,
        height: extent.height as f32,
        min_depth: 0.0,
        max_depth: 1.0,
    };
    (dt.cmd_set_viewport)(command_buffer, 0, 1, &viewport);

    let scissor = vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent,
    };
    (dt.cmd_set_scissor)(command_buffer, 0, 1, &scissor);
}

struct ImageMemoryBarrier {
    src_access_mask: vk::AccessFlags,
    dst_access_mask: vk::AccessFlags,
    old_layout: vk::ImageLayout,
    new_layout: vk::ImageLayout,
    src_queue_family_index: u32,
    dst_queue_family_index: u32,
}

fn record_image_memory_barrier(
    device_table: &DeviceTable,
    command_buffer: *mut vk::CommandBuffer,
    src_stage_mask: vk::PipelineStageFlags,
    dst_stage_mask: vk::PipelineStageFlags,
    barrier: ImageMemoryBarrier,
    image: *mut vk::Image,
) {
    let subresource_range = vk::ImageSubresourceRange {
        aspect_mask: vk::ImageAspectFlagBits::Color as u32,
        base_mip_level: 0,
        mip_levels: 1,
        base_array_layer: 0,
        array_layers: 1,
    };
    let barrier = vk::ImageMemoryBarrier {
        stype: vk::StructureType::ImageMemoryBarrier,
        next: null(),
        src_access_mask: barrier.src_access_mask,
        dst_access_mask: barrier.dst_access_mask,
        old_layout: barrier.old_layout,
        new_layout: barrier.new_layout,
        src_queue_family_index: barrier.src_queue_family_index,
        dst_queue_family_index: barrier.dst_queue_family_index,
        image,
        subresource_range,
    };
    (device_table.cmd_pipeline_barrier)(
        command_buffer,
        src_stage_mask,
        dst_stage_mask,
        vk::DependencyFlags::None,
        0,
        null(),
        0,
        null(),
        1,
        &barrier,
    );
}
