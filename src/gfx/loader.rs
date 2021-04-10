use crate::ffi::{vk, Library};
use std::{ffi::CStr, mem::transmute, ptr::null_mut};

pub struct DeviceTable {
    pub acquire_next_image_khr: vk::AcquireNextImageKHR,
    pub allocate_command_buffers: vk::AllocateCommandBuffers,
    pub allocate_descriptor_sets: vk::AllocateDescriptorSets,
    pub allocate_memory: vk::AllocateMemory,
    pub begin_command_buffer: vk::BeginCommandBuffer,
    pub bind_buffer_memory: vk::BindBufferMemory,
    pub bind_image_memory: vk::BindImageMemory,
    pub cmd_begin_render_pass: vk::CmdBeginRenderPass,
    pub cmd_bind_descriptor_sets: vk::CmdBindDescriptorSets,
    pub cmd_bind_index_buffer: vk::CmdBindIndexBuffer,
    pub cmd_bind_pipeline: vk::CmdBindPipeline,
    pub cmd_bind_vertex_buffers: vk::CmdBindVertexBuffers,
    pub cmd_copy_buffer: vk::CmdCopyBuffer,
    pub cmd_copy_buffer_to_image: vk::CmdCopyBufferToImage,
    pub cmd_copy_image: vk::CmdCopyImage,
    pub cmd_copy_image_to_buffer: vk::CmdCopyImageToBuffer,
    pub cmd_draw: vk::CmdDraw,
    pub cmd_draw_indexed: vk::CmdDrawIndexed,
    pub cmd_draw_indexed_indirect: vk::CmdDrawIndexedIndirect,
    pub cmd_draw_indirect: vk::CmdDrawIndirect,
    pub cmd_end_render_pass: vk::CmdEndRenderPass,
    pub cmd_execute_commands: vk::CmdExecuteCommands,
    pub cmd_pipeline_barrier: vk::CmdPipelineBarrier,
    pub cmd_next_subpass: vk::CmdNextSubpass,
    pub cmd_push_constants: vk::CmdPushConstants,
    pub cmd_set_scissor: vk::CmdSetScissor,
    pub cmd_set_viewport: vk::CmdSetViewport,
    pub create_buffer: vk::CreateBuffer,
    pub create_buffer_view: vk::CreateBufferView,
    pub create_command_pool: vk::CreateCommandPool,
    pub create_compute_pipelines: vk::CreateComputePipelines,
    pub create_descriptor_pool: vk::CreateDescriptorPool,
    pub create_descriptor_set_layout: vk::CreateDescriptorSetLayout,
    pub create_fence: vk::CreateFence,
    pub create_framebuffer: vk::CreateFramebuffer,
    pub create_graphics_pipelines: vk::CreateGraphicsPipelines,
    pub create_image: vk::CreateImage,
    pub create_image_view: vk::CreateImageView,
    pub create_pipeline_layout: vk::CreatePipelineLayout,
    pub create_render_pass: vk::CreateRenderPass,
    pub create_sampler: vk::CreateSampler,
    pub create_semaphore: vk::CreateSemaphore,
    pub create_shader_module: vk::CreateShaderModule,
    pub create_swapchain_khr: vk::CreateSwapchainKHR,
    pub destroy_buffer: vk::DestroyBuffer,
    pub destroy_buffer_view: vk::DestroyBufferView,
    pub destroy_command_pool: vk::DestroyCommandPool,
    pub destroy_descriptor_pool: vk::DestroyDescriptorPool,
    pub destroy_descriptor_set_layout: vk::DestroyDescriptorSetLayout,
    pub destroy_fence: vk::DestroyFence,
    pub destroy_framebuffer: vk::DestroyFramebuffer,
    pub destroy_image: vk::DestroyImage,
    pub destroy_image_view: vk::DestroyImageView,
    pub destroy_pipeline: vk::DestroyPipeline,
    pub destroy_pipeline_layout: vk::DestroyPipelineLayout,
    pub destroy_render_pass: vk::DestroyRenderPass,
    pub destroy_sampler: vk::DestroySampler,
    pub destroy_semaphore: vk::DestroySemaphore,
    pub destroy_shader_module: vk::DestroyShaderModule,
    pub destroy_swapchain_khr: vk::DestroySwapchainKHR,
    pub device_wait_idle: vk::DeviceWaitIdle,
    pub end_command_buffer: vk::EndCommandBuffer,
    pub flush_mapped_memory_ranges: vk::FlushMappedMemoryRanges,
    pub free_command_buffers: vk::FreeCommandBuffers,
    pub free_descriptor_sets: vk::FreeDescriptorSets,
    pub free_memory: vk::FreeMemory,
    pub get_buffer_memory_requirements: vk::GetBufferMemoryRequirements,
    pub get_device_memory_commitment: vk::GetDeviceMemoryCommitment,
    pub get_device_queue: vk::GetDeviceQueue,
    pub get_image_memory_requirements: vk::GetImageMemoryRequirements,
    pub get_swapchain_images_khr: vk::GetSwapchainImagesKHR,
    pub invalidate_mapped_memory_ranges: vk::InvalidateMappedMemoryRanges,
    pub map_memory: vk::MapMemory,
    pub queue_present_khr: vk::QueuePresentKHR,
    pub queue_submit: vk::QueueSubmit,
    pub queue_wait_idle: vk::QueueWaitIdle,
    pub reset_command_buffer: vk::ResetCommandBuffer,
    pub reset_command_pool: vk::ResetCommandPool,
    pub reset_descriptor_pool: vk::ResetDescriptorPool,
    pub reset_fences: vk::ResetFences,
    pub unmap_memory: vk::UnmapMemory,
    pub update_descriptor_sets: vk::UpdateDescriptorSets,
    pub wait_for_fences: vk::WaitForFences,
}

pub struct InstanceTable {
    pub create_device: vk::CreateDevice,
    pub create_instance: vk::CreateInstance,
    #[cfg(target_os = "linux")]
    pub create_xcb_surface_khr: vk::CreateXcbSurfaceKHR,
    pub destroy_device: vk::DestroyDevice,
    pub destroy_instance: vk::DestroyInstance,
    pub destroy_surface_khr: vk::DestroySurfaceKHR,
    pub enumerate_instance_extension_properties: vk::EnumerateInstanceExtensionProperties,
    pub enumerate_physical_devices: vk::EnumeratePhysicalDevices,
    pub get_physical_device_features: vk::GetPhysicalDeviceFeatures,
    pub get_physical_device_format_properties: vk::GetPhysicalDeviceFormatProperties,
    pub get_physical_device_image_format_properties: vk::GetPhysicalDeviceImageFormatProperties,
    pub get_physical_device_memory_properties: vk::GetPhysicalDeviceMemoryProperties,
    pub get_physical_device_properties: vk::GetPhysicalDeviceProperties,
    pub get_physical_device_queue_family_properties: vk::GetPhysicalDeviceQueueFamilyProperties,
    pub get_physical_device_surface_capabilities_khr: vk::GetPhysicalDeviceSurfaceCapabilitiesKHR,
    pub get_physical_device_surface_formats_khr: vk::GetPhysicalDeviceSurfaceFormatsKHR,
    pub get_physical_device_surface_support_khr: vk::GetPhysicalDeviceSurfaceSupportKHR,
}

pub struct Loader {
    get_device_proc_addr: Option<vk::GetDeviceProcAddr>,
    get_instance_proc_addr: vk::GetInstanceProcAddr,
    _vulkan: Library,
}

impl Loader {
    pub fn init() -> Self {
        let _vulkan = Library::open("vulkan").expect("Failed to load vulkan!");
        let get_instance_proc_addr = unsafe {
            transmute::<_, Option<_>>(_vulkan.load("vkGetInstanceProcAddr"))
                .expect("Failed to load vkGetInstanceProcAddr!")
        };

        return Self {
            get_device_proc_addr: None,
            get_instance_proc_addr,
            _vulkan,
        };
    }

    fn load_device_symbol(
        get_device_proc_addr: vk::GetDeviceProcAddr,
        device: *mut vk::Device,
        symbol: &str,
    ) -> vk::Void {
        let csymbol = CStr::from_bytes_with_nul(symbol.as_bytes()).unwrap();
        let fp = get_device_proc_addr(device, csymbol.as_ptr())
            .expect(&format!("Failed to load {}", symbol));
        return fp;
    }

    fn load_instance_symbol(&self, instance: *mut vk::Instance, symbol: &str) -> vk::Void {
        let csymbol = CStr::from_bytes_with_nul(symbol.as_bytes()).unwrap();
        let fp = (self.get_instance_proc_addr)(instance, csymbol.as_ptr())
            .expect(&format!("Failed to load {}", symbol));
        return fp;
    }

    pub fn load_device_functions(&self, device: *mut vk::Device) -> DeviceTable {
        let get_device_proc_addr = self.get_device_proc_addr.unwrap();

        macro_rules! load_device_functions {
            ( $( $member:ident : $symbol:literal $(,)? ),+ ) => {
                unsafe {
                    DeviceTable {
                        $(
                            $member: transmute(
                                Self::load_device_symbol(
                                    get_device_proc_addr,
                                    device,
                                    $symbol
                                )
                            ),
                        )+
                    }
                }
            };
        }

        let table = load_device_functions!(
            acquire_next_image_khr: "vkAcquireNextImageKHR\0",
            allocate_command_buffers: "vkAllocateCommandBuffers\0",
            allocate_descriptor_sets: "vkAllocateDescriptorSets\0",
            allocate_memory: "vkAllocateMemory\0",
            begin_command_buffer: "vkBeginCommandBuffer\0",
            bind_buffer_memory: "vkBindBufferMemory\0",
            bind_image_memory: "vkBindImageMemory\0",
            cmd_begin_render_pass: "vkCmdBeginRenderPass\0",
            cmd_bind_descriptor_sets: "vkCmdBindDescriptorSets\0",
            cmd_bind_index_buffer: "vkCmdBindIndexBuffer\0",
            cmd_bind_pipeline: "vkCmdBindPipeline\0",
            cmd_bind_vertex_buffers: "vkCmdBindVertexBuffers\0",
            cmd_copy_buffer: "vkCmdCopyBuffer\0",
            cmd_copy_buffer_to_image: "vkCmdCopyBufferToImage\0",
            cmd_copy_image: "vkCmdCopyImage\0",
            cmd_copy_image_to_buffer: "vkCmdCopyImageToBuffer\0",
            cmd_draw: "vkCmdDraw\0",
            cmd_draw_indexed: "vkCmdDrawIndexed\0",
            cmd_draw_indexed_indirect: "vkCmdDrawIndexedIndirect\0",
            cmd_draw_indirect: "vkCmdDrawIndirect\0",
            cmd_end_render_pass: "vkCmdEndRenderPass\0",
            cmd_execute_commands: "vkCmdExecuteCommands\0",
            cmd_pipeline_barrier: "vkCmdPipelineBarrier\0",
            cmd_next_subpass: "vkCmdNextSubpass\0",
            cmd_push_constants: "vkCmdPushConstants\0",
            cmd_set_scissor: "vkCmdSetScissor\0",
            cmd_set_viewport: "vkCmdSetViewport\0",
            create_buffer: "vkCreateBuffer\0",
            create_buffer_view: "vkCreateBufferView\0",
            create_command_pool: "vkCreateCommandPool\0",
            create_compute_pipelines: "vkCreateComputePipelines\0",
            create_descriptor_pool: "vkCreateDescriptorPool\0",
            create_descriptor_set_layout: "vkCreateDescriptorSetLayout\0",
            create_fence: "vkCreateFence\0",
            create_framebuffer: "vkCreateFramebuffer\0",
            create_graphics_pipelines: "vkCreateGraphicsPipelines\0",
            create_image: "vkCreateImage\0",
            create_image_view: "vkCreateImageView\0",
            create_pipeline_layout: "vkCreatePipelineLayout\0",
            create_render_pass: "vkCreateRenderPass\0",
            create_sampler: "vkCreateSampler\0",
            create_semaphore: "vkCreateSemaphore\0",
            create_shader_module: "vkCreateShaderModule\0",
            create_swapchain_khr: "vkCreateSwapchainKHR\0",
            destroy_buffer: "vkDestroyBuffer\0",
            destroy_buffer_view: "vkDestroyBufferView\0",
            destroy_command_pool: "vkDestroyCommandPool\0",
            destroy_descriptor_pool: "vkDestroyDescriptorPool\0",
            destroy_descriptor_set_layout: "vkDestroyDescriptorSetLayout\0",
            destroy_fence: "vkDestroyFence\0",
            destroy_framebuffer: "vkDestroyFramebuffer\0",
            destroy_image: "vkDestroyImage\0",
            destroy_image_view: "vkDestroyImageView\0",
            destroy_pipeline: "vkDestroyPipeline\0",
            destroy_pipeline_layout: "vkDestroyPipelineLayout\0",
            destroy_render_pass: "vkDestroyRenderPass\0",
            destroy_sampler: "vkDestroySampler\0",
            destroy_semaphore: "vkDestroySemaphore\0",
            destroy_shader_module: "vkDestroyShaderModule\0",
            destroy_swapchain_khr: "vkDestroySwapchainKHR\0",
            device_wait_idle: "vkDeviceWaitIdle\0",
            end_command_buffer: "vkEndCommandBuffer\0",
            flush_mapped_memory_ranges: "vkFlushMappedMemoryRanges\0",
            free_command_buffers: "vkFreeCommandBuffers\0",
            free_descriptor_sets: "vkFreeDescriptorSets\0",
            free_memory: "vkFreeMemory\0",
            get_buffer_memory_requirements: "vkGetBufferMemoryRequirements\0",
            get_device_memory_commitment: "vkGetDeviceMemoryCommitment\0",
            get_device_queue: "vkGetDeviceQueue\0",
            get_image_memory_requirements: "vkGetImageMemoryRequirements\0",
            get_swapchain_images_khr: "vkGetSwapchainImagesKHR\0",
            invalidate_mapped_memory_ranges: "vkInvalidateMappedMemoryRanges\0",
            map_memory: "vkMapMemory\0",
            queue_present_khr: "vkQueuePresentKHR\0",
            queue_submit: "vkQueueSubmit\0",
            queue_wait_idle: "vkQueueWaitIdle\0",
            reset_command_buffer: "vkResetCommandBuffer\0",
            reset_command_pool: "vkResetCommandPool\0",
            reset_descriptor_pool: "vkResetDescriptorPool\0",
            reset_fences: "vkResetFences\0",
            unmap_memory: "vkUnmapMemory\0",
            update_descriptor_sets: "vkUpdateDescriptorSets\0",
            wait_for_fences: "vkWaitForFences\0",
        );
        return table;
    }

    pub fn load_instance_functions(&mut self, instance: *mut vk::Instance) -> InstanceTable {
        macro_rules! load_instance_functions {
            ( $( $( $os:literal -- )? $member:ident : $symbol:literal $(,)? ),+ ) => {
                unsafe {
                    InstanceTable {
                        $(
                            $(#[cfg(target_os = $os)])?
                            $member: transmute(
                                self.load_instance_symbol(instance, $symbol)
                            ),
                        )+
                    }
                }
            };
        }

        self.get_device_proc_addr =
            unsafe { transmute(self.load_instance_symbol(instance, "vkGetDeviceProcAddr\0")) };

        let table = load_instance_functions!(
            create_device: "vkCreateDevice\0",
            create_instance: "vkCreateInstance\0",
            "linux" -- create_xcb_surface_khr: "vkCreateXcbSurfaceKHR\0",
            destroy_device: "vkDestroyDevice\0",
            destroy_instance: "vkDestroyInstance\0",
            destroy_surface_khr: "vkDestroySurfaceKHR\0",
            enumerate_instance_extension_properties: "vkEnumerateInstanceExtensionProperties\0",
            enumerate_physical_devices: "vkEnumeratePhysicalDevices\0",
            get_physical_device_features: "vkGetPhysicalDeviceFeatures\0",
            get_physical_device_format_properties: "vkGetPhysicalDeviceFormatProperties\0",
            get_physical_device_image_format_properties: "vkGetPhysicalDeviceImageFormatProperties\0",
            get_physical_device_memory_properties: "vkGetPhysicalDeviceMemoryProperties\0",
            get_physical_device_properties: "vkGetPhysicalDeviceProperties\0",
            get_physical_device_queue_family_properties: "vkGetPhysicalDeviceQueueFamilyProperties\0",
            get_physical_device_surface_capabilities_khr: "vkGetPhysicalDeviceSurfaceCapabilitiesKHR\0",
            get_physical_device_surface_formats_khr: "vkGetPhysicalDeviceSurfaceFormatsKHR\0",
            get_physical_device_surface_support_khr: "vkGetPhysicalDeviceSurfaceSupportKHR\0",
        );

        return table;
    }

    pub fn load_vk_create_instance(&self) -> vk::CreateInstance {
        let symbol = "vkCreateInstance\0";
        let csymbol = CStr::from_bytes_with_nul(symbol.as_bytes()).unwrap();
        let void = (self.get_instance_proc_addr)(null_mut(), csymbol.as_ptr());
        let create_instance =
            unsafe { transmute::<_, Option<_>>(void).expect("Failed to load vkCreateInstance") };

        return create_instance;
    }

    pub fn take_library(self) -> Library {
        return self._vulkan;
    }
}
