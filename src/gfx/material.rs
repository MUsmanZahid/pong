use super::loader::DeviceTable;
use crate::ffi::vk;
use std::{
    path::Path,
    ptr::{null, null_mut},
};

pub(crate) struct DescriptorSetLayoutBinding {
    pub(crate) descriptor_type: vk::DescriptorType,
    pub(crate) descriptor_count: u32,
    pub(crate) stage_flags: vk::ShaderStageFlags,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum MaterialError {
    DescriptorSet,
    PipelineLayout,
}

impl std::fmt::Display for MaterialError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return Ok(());
    }
}

impl std::error::Error for MaterialError {}

pub(crate) struct Material {
    pub(crate) set_layout: *mut vk::DescriptorSetLayout,
    pub(crate) pipeline_layout: *mut vk::PipelineLayout,
    pub(crate) pipeline: *mut vk::Pipeline,
    device: *mut vk::Device,
    destroy_descriptor_set_layout: vk::DestroyDescriptorSetLayout,
    destroy_pipeline_layout: vk::DestroyPipelineLayout,
    destroy_pipeline: vk::DestroyPipeline,
}

impl Material {
    pub(crate) fn new<F, V>(
        device_table: &DeviceTable,
        device: *mut vk::Device,
        bindings: &[DescriptorSetLayoutBinding],
        push_constant_ranges: &[vk::PushConstantRange],
        fragment: F,
        vertex: V,
        input_formats: &[vk::Format],
        render_pass: *mut vk::RenderPass,
    ) -> Result<Self, MaterialError>
    where
        F: AsRef<Path>,
        V: AsRef<Path>,
    {
        let fragment_module = super::ShaderModule::from_path(device_table, device, fragment);
        let vertex_module = super::ShaderModule::from_path(device_table, device, vertex);

        let bindings: Box<[vk::DescriptorSetLayoutBinding]> = bindings
            .iter()
            .scan(0, |index, binding| {
                let binding = vk::DescriptorSetLayoutBinding {
                    binding: *index,
                    descriptor_type: binding.descriptor_type,
                    descriptor_count: binding.descriptor_count,
                    stage_flags: binding.stage_flags,
                    immutable_samplers: null(),
                };

                *index += 1;
                Some(binding)
            })
            .collect();
        let set_layout = create_descriptor_set_layout(device_table, device, &bindings)
            .ok_or(MaterialError::DescriptorSet)?;
        let set_layouts = &[set_layout];

        let pipeline_layout =
            create_pipeline_layout(device_table, device, set_layouts, push_constant_ranges)
                .ok_or(MaterialError::PipelineLayout)?;

        let vertex_input_attributes: Box<[vk::VertexInputAttributeDescription]> = input_formats
            .iter()
            .enumerate()
            .scan(0, |offset, (i, &format)| {
                let attribute_description = vk::VertexInputAttributeDescription {
                    location: i as u32,
                    binding: 0,
                    format,
                    offset: *offset,
                };

                *offset += format_length(format) as u32;
                Some(attribute_description)
            })
            .collect();

        let vertex_binding_description = vk::VertexInputBindingDescription {
            binding: 0,
            stride: input_formats
                .iter()
                .fold(0, |acc, &format| acc + format_length(format)) as u32,
            input_rate: vk::VertexInputRate::Vertex,
        };
        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo {
            stype: vk::StructureType::PipelineVertexInputStateCreateInfo,
            next: null(),
            flags: 0,
            vertex_binding_description_count: 1,
            vertex_binding_descriptions: &vertex_binding_description,
            vertex_attribute_description_count: vertex_input_attributes.len() as u32,
            vertex_attribute_descriptions: vertex_input_attributes.as_ptr(),
        };

        let pipeline = create_graphics_pipeline(
            device_table,
            device,
            pipeline_layout,
            render_pass,
            &vertex_input_state,
            *fragment_module,
            *vertex_module,
        );

        let material = Self {
            set_layout,
            pipeline_layout,
            pipeline,
            device,
            destroy_descriptor_set_layout: device_table.destroy_descriptor_set_layout,
            destroy_pipeline_layout: device_table.destroy_pipeline_layout,
            destroy_pipeline: device_table.destroy_pipeline,
        };

        return Ok(material);
    }

    pub(crate) fn sprite(
        device_table: &DeviceTable,
        device: *mut vk::Device,
        render_pass: *mut vk::RenderPass,
    ) -> Result<Self, MaterialError> {
        let sprite = DescriptorSetLayoutBinding {
            descriptor_type: vk::DescriptorType::CombinedImageSampler,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlagBits::Fragment as u32,
        };
        let bindings = [sprite];
        let push_constant_range = [];

        let fragment = "shaders/triangle.frag.spv";
        let vertex = "shaders/triangle.vert.spv";

        let formats = [vk::Format::R32G32SFLOAT, vk::Format::R32G32SFLOAT];

        let material = Self::new(
            device_table,
            device,
            &bindings,
            &push_constant_range,
            fragment,
            vertex,
            &formats,
            render_pass,
        );
        return material;
    }

    pub(crate) fn text(
        device_table: &DeviceTable,
        device: *mut vk::Device,
        render_pass: *mut vk::RenderPass,
    ) -> Result<Self, MaterialError> {
        let glyphs = DescriptorSetLayoutBinding {
            descriptor_type: vk::DescriptorType::CombinedImageSampler,
            descriptor_count: 1,
            stage_flags: vk::ShaderStageFlagBits::Fragment as u32,
        };
        let bindings = [glyphs];
        let push_constant_range = [];

        let fragment = "shaders/text.frag.spv";
        let vertex = "shaders/text.vert.spv";

        let formats = [
            vk::Format::R32G32SFLOAT,
            vk::Format::R32G32SFLOAT,
            vk::Format::R32G32B32A32SFLOAT,
        ];

        let material = Self::new(
            device_table,
            device,
            &bindings,
            &push_constant_range,
            fragment,
            vertex,
            &formats,
            render_pass,
        );
        return material;
    }
}

impl Drop for Material {
    fn drop(&mut self) {
        (self.destroy_pipeline)(self.device, self.pipeline, null());
        (self.destroy_pipeline_layout)(self.device, self.pipeline_layout, null());
        (self.destroy_descriptor_set_layout)(self.device, self.set_layout, null());
    }
}

fn create_descriptor_set_layout(
    device_table: &DeviceTable,
    device: *mut vk::Device,
    bindings: &[vk::DescriptorSetLayoutBinding],
) -> Option<*mut vk::DescriptorSetLayout> {
    let info = vk::DescriptorSetLayoutCreateInfo {
        stype: vk::StructureType::DescriptorSetLayoutCreateInfo,
        next: null(),
        flags: 0,
        binding_count: bindings.len() as u32,
        bindings: bindings.as_ptr(),
    };

    let mut set_layout = null_mut();
    let result =
        (device_table.create_descriptor_set_layout)(device, &info, null(), &mut set_layout);

    if result != vk::Result::Success || set_layout.is_null() {
        return None;
    }

    return Some(set_layout);
}

fn create_graphics_pipeline(
    device_table: &DeviceTable,
    device: *mut vk::Device,
    layout: *mut vk::PipelineLayout,
    render_pass: *mut vk::RenderPass,
    vertex_input_state: *const vk::PipelineVertexInputStateCreateInfo,
    fragment: *mut vk::ShaderModule,
    vertex: *mut vk::ShaderModule,
) -> *mut vk::Pipeline {
    let name = cstr!("main");
    let fragment_stage = vk::PipelineShaderStageCreateInfo {
        stype: vk::StructureType::PipelineShaderStageCreateInfo,
        next: null(),
        flags: 0,
        stage: vk::ShaderStageFlagBits::Fragment,
        module: fragment,
        name: name.as_ptr(),
        specialization_info: null(),
    };
    let vertex_stage = vk::PipelineShaderStageCreateInfo {
        stype: vk::StructureType::PipelineShaderStageCreateInfo,
        next: null(),
        flags: 0,
        stage: vk::ShaderStageFlagBits::Vertex,
        module: vertex,
        name: name.as_ptr(),
        specialization_info: null(),
    };
    let stages = [fragment_stage, vertex_stage];

    let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo {
        stype: vk::StructureType::PipelineInputAssemblyStateCreateInfo,
        next: null(),
        flags: 0,
        topology: vk::PrimitiveTopology::TriangleList,
        primitive_restart_enabled: false as u32,
    };

    let viewport_state = vk::PipelineViewportStateCreateInfo {
        stype: vk::StructureType::PipelineViewportStateCreateInfo,
        next: null(),
        flags: 0,
        viewport_count: 1,
        viewports: &vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            min_depth: 0.0,
            max_depth: 0.0,
        },
        scissor_count: 1,
        scissors: &vk::Rect2D {
            offset: (0, 0).into(),
            extent: (0, 0).into(),
        },
    };

    let rasterization_state = vk::PipelineRasterizationStateCreateInfo {
        stype: vk::StructureType::PipelineRasterizationStateCreateInfo,
        next: null(),
        flags: 0,
        depth_clamp_enable: false as u32,
        rasterizer_discard_enable: false as u32,
        polygon_mode: vk::PolygonMode::Fill,
        cull_mode: vk::CullModeBits::Back as u32,
        front_face: vk::FrontFace::Clockwise,
        depth_bias_enable: false as u32,
        depth_bias_constant_factor: 0.0,
        depth_bias_clamp: 0.0,
        depth_bias_slope_factor: 0.0,
        line_width: 1.0,
    };

    let multisample_state = vk::PipelineMultisampleStateCreateInfo {
        stype: vk::StructureType::PipelineMultisampleStateCreateInfo,
        next: null(),
        flags: 0,
        rasterization_samples: vk::SampleCountFlagBits::One,
        sample_shading_enabled: false as u32,
        min_sample_shading: 0.0,
        sample_mask: null(),
        alpha_to_coverage_enable: false as u32,
        alpha_to_one_enable: false as u32,
    };

    let attachment_state = vk::PipelineColorBlendAttachmentState {
        blend_enable: true as u32,
        src_color_blend_factor: vk::BlendFactor::SourceAlpha,
        dst_color_blend_factor: vk::BlendFactor::OneMinusSourceAlpha,
        color_blend_op: vk::BlendOp::Add,
        src_alpha_blend_factor: vk::BlendFactor::One,
        dst_alpha_blend_factor: vk::BlendFactor::Zero,
        alpha_blend_op: vk::BlendOp::Add,
        color_write_mask: vk::ColorComponentFlagBits::R as u32
            | vk::ColorComponentFlagBits::G as u32
            | vk::ColorComponentFlagBits::B as u32
            | vk::ColorComponentFlagBits::A as u32,
    };
    let color_blend_state = vk::PipelineColorBlendStateCreateInfo {
        stype: vk::StructureType::PipelineColorBlendStateCreateInfo,
        next: null(),
        flags: 0,
        logic_op_enable: false as u32,
        logic_op: vk::LogicOp::NoOp,
        attachment_count: 1,
        attachments: &attachment_state,
        blend_constants: [0.0, 0.0, 0.0, 0.0],
    };

    let dynamic_states = [vk::DynamicState::Scissor, vk::DynamicState::Viewport];
    let dynamic_state = vk::PipelineDynamicStateCreateInfo {
        stype: vk::StructureType::PipelineDynamicStateCreateInfo,
        next: null(),
        flags: 0,
        dynamic_state_count: dynamic_states.len() as u32,
        dynamic_states: dynamic_states.as_ptr(),
    };

    let info = vk::GraphicsPipelineCreateInfo {
        stype: vk::StructureType::GraphicsPipelineCreateInfo,
        next: null(),
        flags: 0,
        stage_count: stages.len() as u32,
        stages: stages.as_ptr(),
        vertex_input_state,
        input_assembly_state: &input_assembly_state,
        tessellation_state: null(),
        viewport_state: &viewport_state,
        rasterization_state: &rasterization_state,
        multisample_state: &multisample_state,
        depth_stencil_state: null(),
        color_blend_state: &color_blend_state,
        dynamic_state: &dynamic_state,
        layout,
        render_pass,
        subpass: 0,
        base_pipeline_handle: null_mut(),
        base_pipeline_index: -1,
    };
    let mut pipeline = null_mut();
    (device_table.create_graphics_pipelines)(device, null_mut(), 1, &info, null(), &mut pipeline);
    return pipeline;
}

fn create_pipeline_layout(
    device_table: &DeviceTable,
    device: *mut vk::Device,
    set_layouts: &[*mut vk::DescriptorSetLayout],
    push_constant_ranges: &[vk::PushConstantRange],
) -> Option<*mut vk::PipelineLayout> {
    let info = vk::PipelineLayoutCreateInfo {
        stype: vk::StructureType::PipelineLayoutCreateInfo,
        next: null(),
        flags: 0,
        set_layout_count: set_layouts.len() as u32,
        set_layouts: set_layouts.as_ptr(),
        push_constant_range_count: push_constant_ranges.len() as u32,
        push_constant_ranges: push_constant_ranges.as_ptr(),
    };

    let mut layout = null_mut();
    let result = (device_table.create_pipeline_layout)(device, &info, null(), &mut layout);
    if result != vk::Result::Success || layout.is_null() {
        return None;
    }

    return Some(layout);
}

fn format_length(format: vk::Format) -> usize {
    use vk::Format::*;
    let float_size = std::mem::size_of::<f32>();

    match format {
        R32G32SFLOAT => 2 * float_size,
        R32G32B32A32SFLOAT => 4 * float_size,
        _ => todo!("Unknown format {:?}", format),
    }
}
