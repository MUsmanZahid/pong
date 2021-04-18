use crate::ffi::vk;
use super::loader::DeviceTable;

struct Scene {
    primary: *mut vk::CommandBuffer,
    secondaries: Vec<*mut vk::CommandBuffer>,
    descriptor_sets: Vec<*mut vk::DescriptorSet>,
}

impl Scene {
    pub(crate) fn new(device_table: &DeviceTable, device: *mut vk::Device) -> Self {
        let primary = create_primary_command_buffer();
        
        let scene = Self {
            primary,
            secondaries: Vec::new(),
            descriptor_sets: Vec::new(),
        };
        return scene;
    }
}