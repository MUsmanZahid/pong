use std::{ffi::c_void, os::raw::c_char};

#[macro_export]
macro_rules! make_version {
    ( $major:expr, $minor:expr, $patch:expr ) => {
        ($major << 22) | ($minor << 12) | $patch
    };
}

// Version of this file, corresponding to vulkan_core.h version
pub const HEADER_VERSION: u32 = 162;
pub const HEADER_VERSION_COMPLETE: u32 = make_version!(1, 2, HEADER_VERSION);

// Constants
pub const API_VERSION_1_0: u32 = make_version!(1, 0, 0);
pub const API_VERSION_1_2: u32 = make_version!(1, 2, 0);
pub const MAX_DESCRIPTION_SIZE: usize = 256;
pub const MAX_EXTENSION_NAME_SIZE: usize = 256;
pub const MAX_MEMORY_HEAPS: usize = 16;
pub const MAX_MEMORY_TYPES: usize = 32;
pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;
pub const QUEUE_FAMILY_IGNORED: u32 = !0;
pub const SUBPASS_EXTERNAL: u32 = !0;
pub const UUID_SIZE: usize = 16;
pub const WHOLE_SIZE: u64 = !0;

// Type aliases
pub type AccessFlags = Flags;
pub type AttachmentDescriptionFlags = Flags;
pub type Bool32 = u32;
pub type BufferCreateFlags = u32;
pub type BufferUsageFlags = u32;
pub type BufferViewCreateFlags = u32;
pub type ColorComponentFlags = u32;
pub type CommandBufferResetFlags = Flags;
pub type CommandBufferUsageFlags = Flags;
pub type CommandPoolCreateFlags = Flags;
pub type CommandPoolResetFlags = Flags;
pub type CullModeFlags = Flags;
pub type DescriptorPoolCreateFlags = Flags;
pub type DescriptorPoolResetFlags = Flags;
pub type DescriptorSetLayoutCreateFlags = Flags;
pub type DeviceSize = u64;
pub type FenceCreateFlags = Flags;
pub type Flags = u32;
pub type FramebufferCreateFlags = Flags;
pub type ImageAspectFlags = Flags;
pub type ImageCreateFlags = Flags;
pub type ImageUsageFlags = Flags;
pub type InstanceCreateFlags = Flags;
pub type MemoryHeapFlags = Flags;
pub type MemoryMapFlags = Flags;
pub type MemoryPropertyFlags = Flags;
pub type PipelineCacheCreateFlags = Flags;
pub type PipelineColorBlendStateCreateFlags = Flags;
pub type PipelineCreateFlags = Flags;
pub type PipelineDepthStencilStateCreateFlags = Flags;
pub type PipelineDynamicStateCreateFlags = Flags;
pub type PipelineInputAssemblyStateCreateFlags = Flags;
pub type PipelineLayoutCreateFlags = Flags;
pub type PipelineMultisampleStateCreateFlags = Flags;
pub type PipelineRasterizationStateCreateFlags = Flags;
pub type PipelineShaderStageCreateFlags = Flags;
pub type PipelineStageFlags = Flags;
pub type PipelineStatisticFlags = Flags;
pub type PipelineTesselationStateCreateFlags = Flags;
pub type PipelineVertexInputStateCreateFlags = Flags;
pub type PipelineViewportStateCreateFlags = Flags;
pub type QueryControlFlags = Flags;
pub type QueueFlags = Flags;
pub type RenderPassCreateFlags = Flags;
pub type SamplerCreateFlags = Flags;
pub type SampleCountFlags = Flags;
pub type SampleMask = u32;
pub type SemaphoreCreateFlags = Flags;
pub type ShaderModuleCreateFlags = Flags;
pub type ShaderStageFlags = Flags;
pub type SubpassDescriptionFlags =  Flags;
pub type SurfaceTransformFlagsKHR = u32;
pub type XcbSurfaceCreateFlagsKHR = u32;

// Enumerations
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum AccessFlagBits {
    IndirectCommandRead = 0x00000001,
    IndexRead = 0x00000002,
    VertexAttributeRead = 0x00000004,
    UniformRead = 0x00000008,
    InputAttachmentRead = 0x00000010,
    ShaderRead = 0x00000020,
    ShaderWrite = 0x00000040,
    ColorAttachmentRead = 0x00000080,
    ColorAttachmentWrite = 0x00000100,
    DepthStencilAttachmentRead = 0x00000200,
    DepthStencilAttachmentWrite = 0x00000400,
    TransferRead = 0x00000800,
    TransferWrite = 0x00001000,
    HostRead = 0x00002000,
    HostWrite = 0x00004000,
    MemoryRead = 0x00008000,
    MemoryWrite = 0x00010000,
    // Provided by VK_EXT_transform_feedback
    TransformFeedbackWriteEXT = 0x02000000,
    // Provided by VK_EXT_transform_feedback
    TransformFeedbackCounterReadEXT = 0x04000000,
    // Provided by VK_EXT_transform_feedback
    TransformFeedbackCounterWriteEXT = 0x08000000,
    // Provided by VK_EXT_conditional_rendering
    ConditionalRenderingReadEXT = 0x00100000,
    // Provided by VK_EXT_blend_operation_advanced
    ColorAttachmentReadNoncoherentEXT = 0x00080000,
    // Provided by VK_KHR_acceleration_structure
    AccelerationStructureReadKHR = 0x00200000,
    // Provided by VK_KHR_acceleration_structure
    AccelerationStructureWriteKHR = 0x00400000,
    // Provided by VK_NV_shading_rate_image
    ShadingRateImageReadNV = 0x00800000,
    // Provided by VK_EXT_fragment_density_map
    FragmentDensityMapReadEXT = 0x01000000,
    // Provided by VK_NV_device_generated_commands
    CommandPreprocessReadNV = 0x00020000,
    // Provided by VK_NV_device_generated_commands
    CommandPreprocessWriteNV = 0x00040000,
    // Provided by VK_KHR_synchronization2
    NoneKHR = 0,
    // Provided by VK_NV_ray_tracing
    // AccelerationStructureReadNV = AccelerationStructureReadKHR,
    // Provided by VK_NV_ray_tracing
    // AccelerationStructureWriteNV = AccelerationStructureWriteKHR,
    // Provided by VK_KHR_fragment_shading_rate
    // FragmentShadingRateAttachmentReadKHR = ShadingRateImageReadNV,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum AttachmentDescriptionFlagBits {
    MayAlias = 0x0000_0001,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum AttachmentLoadOp {
    Load = 0,
    Clear = 1,
    DontCare = 2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum AttachmentStoreOp {
    Store = 0,
    DontCare = 1,
    NoneQCOM = 2,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum BlendOp {
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
    ZeroEXT = 1_000_148_000,
    SourceEXT = 1_000_148_001,
    DestinationEXT = 1_000_148_002,
    SourceOverEXT = 1_000_148_003,
    DestinationOverExt = 1_000_148_004,
    SourceInEXT = 1_000_148_005,
    DestinationInEXT = 1_000_148_006,
    SourceOutEXT = 1_000_148_007,
    DestinationOutEXT = 1_000_148_008,
    SourceAtopEXT = 1_000_148_009,
    DestinationAtopEXT = 1_000_148_010,
    XorEXT = 1_000_148_011,
    MultiplyEXT = 1_000_148_012,
    ScreenEXT = 1_000_148_013,
    OverlayEXT = 1_000_148_014,
    DarkenEXT = 1_000_148_015,
    LightenEXT = 1_000_148_016,
    ColorDodgeEXT = 1_000_148_017,
    ColorBurnEXT = 1_000_148_018,
    HardLightEXT = 1_000_148_019,
    SoftLightEXT = 1_000_148_020,
    DifferenceEXT = 1_000_148_021,
    ExclusionEXT = 1_000_148_022,
    InvertEXT = 1_000_148_023,
    InvertRGBEXT = 1_000_148_024,
    LinearDodgeEXT = 1_000_148_025,
    LinearBurnEXT = 1_000_148_026,
    VividLightEXT = 1_000_148_027,
    LinearLightEXT = 1_000_148_028,
    PinLightEXT = 1_000_148_029,
    HardMixEXT = 1_000_148_030,
    HSLHueEXT = 1_000_148_031,
    HSLSaturationEXT = 1_000_148_032,
    HSLColorEXT = 1_000_148_033,
    HSLLuminosityEXT = 1_000_148_034,
    PlusEXT = 1_000_148_035,
    PlusClampedEXT = 1_000_148_036,
    PlusClampedAlphaEXT = 1_000_148_037,
    DarkerEXT = 1_000_148_038,
    MinusEXT = 1_000_148_039,
    MinusClampedEXT = 1_000_148_040,
    ContrastEXT = 1_000_148_041,
    InvertOVGEXT = 1_000_148_042,
    RedEXT = 1_000_148_043,
    GreenEXT = 1_000_148_044,
    BlueEXT = 1_000_148_045,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum BlendFactor {
    Zero = 0,
    One = 1,
    SourceColor = 2,
    OneMinusSourceColor = 3,
    DestinationColor = 4,
    OneMinusDestinationColor = 5,
    SourceAlpha = 6,
    OneMinusSourceAlpha = 7,
    DestinationAlpha = 8,
    OneMinusDestinationAlpha = 9,
    ConstantColor = 10,
    OneMinusConstantColor = 11,
    ConstantAlpha = 12,
    OneMinusConstantAlpha = 13,
    SourceAlphaSaturate = 14,
    Source1Color = 15,
    OneMinusSource1Color = 16,
    Source1Alpha = 17,
    OneMinusSource1Alpha = 18,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum BorderColor {
    FloatTransparentBlack = 0,
    IntTransparentBlack = 1,
    FloatOpaqueBlack = 2,
    IntOpaqueBlack = 3,
    FloatOpaqueWhite = 4,
    IntOpaqueWhite = 5,
    FloatCustomEXT = 1_000_287_003,
    IntCustomEXT = 1_000_287_004,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum BufferCreateFlagBits {
    SparseBinding = 0x0000_0001,
    SparseResidency = 0x0000_0002,
    SparseAliased = 0x0000_0004,
    Protected = 0x0000_0008,
    DeviceAddressCaptureReplayKHR = 0x0000_0010,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum BufferUsageFlagBits {
    TransferSource = 0x0000_0001,
    TransferDestination = 0x0000_0002,
    UniformTexelBuffer = 0x0000_0004,
    StorageTexelBuffer = 0x0000_0008,
    UniformBuffer = 0x0000_0010,
    StorageBuffer = 0x0000_0020,
    IndexBuffer = 0x0000_0040,
    VertexBuffer = 0x0000_0080,
    IndirectBuffer = 0x0000_0100,
    ShaderDeviceAddress = 0x0002_0000,
    TransformFeedbackBufferEXT = 0x0000_0800,
    TransformFeedbackCounterBufferEXT = 0x0000_1000,
    ConditionalRenderingEXT = 0x0000_0200,
    AccelerationStructureBuildInputReadOnlyKHR = 0x0008_000,
    AccelerationStructureStorageKHR = 0x0010_0000,
    ShaderBindingTableBitKHR = 0x0000_0400,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum ColorComponentFlagBits {
    R = 0x0000_0001,
    G = 0x0000_0002,
    B = 0x0000_0004,
    A = 0x0000_0008,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum ColorSpaceKHR {
    SRGBNonlinearKHR = 0,
    DisplayP3NonlinearEXT = 1000104001,
    ExtendedSRGBLinearExt = 1000104002,
    DisplayP3LinearEXT = 1000104003,
    DCIP3NonlinearEXT = 1000104004,
    BT709LinearEXT = 1000104005,
    BT709NonlinearEXT = 1000104006,
    HDR10ST2084EXT = 1000104008,
    DolbyVisionEXT = 1000104009,
    HDR10HLGEXT = 1000104010,
    AdobeRGBLinearEXT = 1000104011,
    AdobeRGBNonlinearEXT = 1000104012,
    PassThroughEXT = 1000104013,
    ExtendedSRGBNonlinearEXT = 1000104014,
    DisplayNativeAMD = 1000213000,
    // RGBNonlinearKHR = SRGBNonlinearKHR,
    // DCIP3LinearEXT = DisplayP3LinearEXT,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum CompareOp {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessOrEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterOrEqual = 6,
    Always = 7,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum CompositeAlphaFlagsKHR {
    Opaque = 0x0000_0001,
    PreMultiplied = 0x0000_0002,
    PostMultiplied = 0x0000_0004,
    Inherit = 0x0000_0008,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum CommandBufferLevel {
    Primary = 0,
    Secondary = 1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum CommandBufferResetFlagBits {
    ReleaseResources = 0x0000_0001,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum CommandBufferUsageFlagBits {
    OneTimeSubmit = 0x0000_0001,
    RenderPassContinue = 0x0000_0002,
    SimultaneousUse = 0x0000_0004,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum CommandPoolCreateFlagBits {
    Transient = 0x00000001,
    ResetCommandBuffer = 0x00000002,
    Protected = 0x00000004,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum CommandPoolResetFlagBits {
    ReleaseResources = 0x0000_0001,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum ComponentSwizzle {
    Identity = 0,
    Zero = 1,
    One = 2,
    R = 3,
    G = 4,
    B = 5,
    A = 6,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum CullModeBits {
    None = 0,
    Front = 0x0000_0001,
    Back = 0x0000_0002,
    FrontAndBack = 0x0000_0003,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum DependencyFlags {
    None = 0x0000_0000,
    ByRegion = 0x0000_0001,
    // Provided by VK_KHR_multiview
    ViewLocalKHR = 0x0000_0002,
    // Provided by VK_KHR_multiview
    DeviceGroupKHR = 0x0000_0004,
    // Provided by VK_VERSION_1_1
    // ViewLocal = 0x0000_0002,
    // Provided by VK_VERSION_1_1
    // DeviceGroup = 0x0000_0004,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum DescriptorPoolCreateFlagBits {
    FreeDescriptorSet = 0x0000_0001,
    UpdateAfterBindEXT = 0x0000_0002,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum DescriptorSetLayoutCreateFlagBits {
    PushDescriptorKHR = 0x0000_0001,
    UpdateAfterBindPoolEXT = 0x0000_0002,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum DescriptorType {
    Sampler = 0,
    CombinedImageSampler = 1,
    SamplerImage = 2,
    StorageImage = 3,
    UniformTexelBuffer = 4,
    StorageTexelBuffer = 5,
    UniformBuffer = 6,
    StorageBuffer = 7,
    UniformBufferDynamic = 8,
    StorageBufferDynamic = 9,
    InputAttachment = 10,
    InlineUniformBlockEXT = 1_000_138_000,
    AccelerationStructureKHR = 1_000_150_000,
    AccelerationStructureNV = 1_000_165_000,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum DynamicState {
    Viewport = 0,
    Scissor = 1,
    LineWidth = 2,
    DepthBias = 3,
    BlendConstants = 4,
    DepthBounds = 5,
    StencilCompareMask = 6,
    StencilWriteMask = 7,
    StencilReference = 8,
    ViewportWScalingNV = 1_000_087_000,
    DiscardRectangleEXT = 1_000_099_000,
    SampleLocationsEXT = 1_000_143_000,
    RayTracingPipelineStackSizeKHR = 1_000_347_000,
    ViewportShadingRatePaletteNV = 1_000_164_004,
    ViewportCoarseSampleOrderNV = 1_000_164_006,
    ExclusiveScissorNV = 1_000_205_001,
    FragmentShadingRateKHR = 1_000_226_000,
    LineStippleEXT = 1_000_259_000,
    CullModeEXT = 1_000_267_000,
    FrontFaceEXT = 1_000_267_001,
    PrimitiveTopologyEXT = 1_000_267_002,
    ViewportWithCountEXT = 1_000_267_003,
    ScissorWithCountEXT = 1_000_267_004,
    VertexInputBindingStrideEXT = 1_000_267_005,
    DepthTestEnableEXT = 1_000_267_006,
    DepthWriteEnableEXT = 1_000_267_007,
    DepthCompareOpEXT = 1_000_267_008,
    DepthBoundsTestEnableEXT = 1_000_267_009,
    StencilTestEnableEXT = 1_000_267_010,
    StencilOpEXT = 1_000_267_011,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum FenceCreateFlagBits {
    Signaled = 0x0000_0001,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum Filter {
    Nerest = 0,
    Linear = 1,
    CubicEXT = 1_000_015_000,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum Format {
    Undefined = 0,
    R4G4UNORMPack8 = 1,
    R4G4B4A4UNORMPack16 = 2,
    B4G4R4A4UNORMPack16 = 3,
    R5G6B5UNORMPack16 = 4,
    B5G6R5UNORMPack16 = 5,
    R5G5B5A1UNORMPack16 = 6,
    B5G5R5A1UNORMPack16 = 7,
    A1R5G5B5UNORMPack16 = 8,
    R8UNORM = 9,
    R8SNORM = 10,
    R8USCALED = 11,
    R8SSCALED = 12,
    R8UINT = 13,
    R8SINT = 14,
    R8SRGB = 15,
    R8G8UNORM = 16,
    R8G8SNORM = 17,
    R8G8USCALED = 18,
    R8G8SSCALED = 19,
    R8G8UINT = 20,
    R8G8SINT = 21,
    R8G8SRGB = 22,
    R8G8B8UNORM = 23,
    R8G8B8SNORM = 24,
    R8G8B8USCALED = 25,
    R8G8B8SSCALED = 26,
    R8G8B8UINT = 27,
    R8G8B8SINT = 28,
    R8G8B8SRGB = 29,
    B8G8R8UNORM = 30,
    B8G8R8SNORM = 31,
    B8G8R8USCALED = 32,
    B8G8R8SSCALED = 33,
    B8G8R8UINT = 34,
    B8G8R8SINT = 35,
    B8G8R8SRGB = 36,
    R8G8B8A8UNORM = 37,
    R8G8B8A8SNORM = 38,
    R8G8B8A8USCALED = 39,
    R8G8B8A8SSCALED = 40,
    R8G8B8A8UINT = 41,
    R8G8B8A8SINT = 42,
    R8G8B8A8SRGB = 43,
    B8G8R8A8UNORM = 44,
    B8G8R8A8SNORM = 45,
    B8G8R8A8USCALED = 46,
    B8G8R8A8SSCALED = 47,
    B8G8R8A8UINT = 48,
    B8G8R8A8SINT = 49,
    B8G8R8A8SRGB = 50,
    A8B8G8R8UNORMPack32 = 51,
    A8B8G8R8SNORMPack32 = 52,
    A8B8G8R8USCALEDPack32 = 53,
    A8B8G8R8SSCALEDPack32 = 54,
    A8B8G8R8UINTPack32 = 55,
    A8B8G8R8SINTPack32 = 56,
    A8B8G8R8SRGBPack32 = 57,
    A2R10G10B10UNORMPack32 = 58,
    A2R10G10B10SNORMPack32 = 59,
    A2R10G10B10USCALEDPack32 = 60,
    A2R10G10B10SSCALEDPack32 = 61,
    A2R10G10B10UINTPack32 = 62,
    A2R10G10B10SINTPack32 = 63,
    A2B10G10R10UNORMPack32 = 64,
    A2B10G10R10SNORMPack32 = 65,
    A2B10G10R10USCALEDPack32 = 66,
    A2B10G10R10SSCALEDPack32 = 67,
    A2B10G10R10UINTPack32 = 68,
    A2B10G10R10SINTPack32 = 69,
    R16UNORM = 70,
    R16SNORM = 71,
    R16USCALED = 72,
    R16SSCALED = 73,
    R16UINT = 74,
    R16SINT = 75,
    R16SFLOAT = 76,
    R16G16UNORM = 77,
    R16G16SNORM = 78,
    R16G16USCALED = 79,
    R16G16SSCALED = 80,
    R16G16UINT = 81,
    R16G16SINT = 82,
    R16G16SFLOAT = 83,
    R16G16B16UNORM = 84,
    R16G16B16SNORM = 85,
    R16G16B16USCALED = 86,
    R16G16B16SSCALED = 87,
    R16G16B16UINT = 88,
    R16G16B16SINT = 89,
    R16G16B16SFLOAT = 90,
    R16G16B16A16UNORM = 91,
    R16G16B16A16SNORM = 92,
    R16G16B16A16USCALED = 93,
    R16G16B16A16SSCALED = 94,
    R16G16B16A16UINT = 95,
    R16G16B16A16SINT = 96,
    R16G16B16A16SFLOAT = 97,
    R32UINT = 98,
    R32SINT = 99,
    R32SFLOAT = 100,
    R32G32UINT = 101,
    R32G32SINT = 102,
    R32G32SFLOAT = 103,
    R32G32B32UINT = 104,
    R32G32B32SINT = 105,
    R32G32B32SFLOAT = 106,
    R32G32B32A32UINT = 107,
    R32G32B32A32SINT = 108,
    R32G32B32A32SFLOAT = 109,
    R64UINT = 110,
    R64SINT = 111,
    R64SFLOAT = 112,
    R64G64UINT = 113,
    R64G64SINT = 114,
    R64G64SFLOAT = 115,
    R64G64B64UINT = 116,
    R64G64B64SINT = 117,
    R64G64B64SFLOAT = 118,
    R64G64B64A64UINT = 119,
    R64G64B64A64SINT = 120,
    R64G64B64A64SFLOAT = 121,
    B10G11R11UFLOATPack32 = 122,
    E5B9G9R9UFLOATPack32 = 123,
    D16UNORM = 124,
    X8D24UNORMPack32 = 125,
    D32SFLOAT = 126,
    S8UINT = 127,
    D16UNORMS8UINT = 128,
    D24UNORMS8UINT = 129,
    D32SFLOATS8UINT = 130,
    BC1RGBUNORMBlock = 131,
    BC1RGBSRGBBlock = 132,
    BC1RGBAUNORMBlock = 133,
    BC1RGBASRGBBlock = 134,
    BC2UNORMBlock = 135,
    BC2SRGBBlock = 136,
    BC3UNORMBlock = 137,
    BC3SRGBBlock = 138,
    BC4UNORMBlock = 139,
    BC4SNORMBlock = 140,
    BC5UNORMBlock = 141,
    BC5SNORMBlock = 142,
    BC6HUFLOATBlock = 143,
    BC6HSFLOATBlock = 144,
    BC7UNORMBlock = 145,
    BC7SRGBBlock = 146,
    ETC2R8G8B8UNORMBlock = 147,
    ETC2R8G8B8SRGBBlock = 148,
    ETC2R8G8B8A1UNORMBlock = 149,
    ETC2R8G8B8A1SRGBBlock = 150,
    ETC2R8G8B8A8UNORMBlock = 151,
    ETC2R8G8B8A8SRGBBlock = 152,
    EACR11UNORMBlock = 153,
    EACR11SNORMBlock = 154,
    EACR11G11UNORMBlock = 155,
    EACR11G11SNORMBlock = 156,
    ASTC4X4UNORMBlock = 157,
    ASTC4X4SRGBBlock = 158,
    ASTC5X4UNORMBlock = 159,
    ASTC5X4SRGBBlock = 160,
    ASTC5X5UNORMBlock = 161,
    ASTC5X5SRGBBlock = 162,
    ASTC6X5UNORMBlock = 163,
    ASTC6X5SRGBBlock = 164,
    ASTC6X6UNORMBlock = 165,
    ASTC6X6SRGBBlock = 166,
    ASTC8X5UNORMBlock = 167,
    ASTC8X5SRGBBlock = 168,
    ASTC8X6UNORMBlock = 169,
    ASTC8X6SRGBBlock = 170,
    ASTC8X8UNORMBlock = 171,
    ASTC8X8SRGBBlock = 172,
    ASTC10X5UNORMBlock = 173,
    ASTC10X5SRGBBlock = 174,
    ASTC10X6UNORMBlock = 175,
    ASTC10X6SRGBBlock = 176,
    ASTC10X8UNORMBlock = 177,
    ASTC10X8SRGBBlock = 178,
    ASTC10X10UNORMBlock = 179,
    ASTC10X10SRGBBlock = 180,
    ASTC12X10UNORMBlock = 181,
    ASTC12X10SRGBBlock = 182,
    ASTC12X12UNORMBlock = 183,
    ASTC12X12SRGBBlock = 184,
    G8B8G8R8422UNORM = 1000156000,
    B8G8R8G8422UNORM = 1000156001,
    G8B8R83PLANE420UNORM = 1000156002,
    G8B8R82PLANE420UNORM = 1000156003,
    G8B8R83PLANE422UNORM = 1000156004,
    G8B8R82PLANE422UNORM = 1000156005,
    G8B8R83PLANE444UNORM = 1000156006,
    R10X6UNORMPack16 = 1000156007,
    R10X6G10X6UNORM2Pack16 = 1000156008,
    R10X6G10X6B10X6A10X6UNORM4Pack16 = 1000156009,
    G10X6B10X6G10X6R10X6422UNORM4Pack16 = 1000156010,
    B10X6G10X6R10X6G10X6422UNORM4Pack16 = 1000156011,
    G10X6B10X6R10X63PLANE420UNORM3Pack16 = 1000156012,
    G10X6B10X6R10X62PLANE420UNORM3Pack16 = 1000156013,
    G10X6B10X6R10X63PLANE422UNORM3Pack16 = 1000156014,
    G10X6B10X6R10X62PLANE422UNORM3Pack16 = 1000156015,
    G10X6B10X6R10X63PLANE444UNORM3Pack16 = 1000156016,
    R12X4UNORMPack16 = 1000156017,
    R12X4G12X4UNORM2Pack16 = 1000156018,
    R12X4G12X4B12X4A12X4UNORM4Pack16 = 1000156019,
    G12X4B12X4G12X4R12X4422UNORM4Pack16 = 1000156020,
    B12X4G12X4R12X4G12X4422UNORM4Pack16 = 1000156021,
    G12X4B12X4R12X43PLANE420UNORM3Pack16 = 1000156022,
    G12X4B12X4R12X42PLANE420UNORM3Pack16 = 1000156023,
    G12X4B12X4R12X43PLANE422UNORM3Pack16 = 1000156024,
    G12X4B12X4R12X42PLANE422UNORM3Pack16 = 1000156025,
    G12X4B12X4R12X43PLANE444UNORM3Pack16 = 1000156026,
    G16B16G16R16422UNORM = 1000156027,
    B16G16R16G16422UNORM = 1000156028,
    G16B16R163PLANE420UNORM = 1000156029,
    G16B16R162PLANE420UNORM = 1000156030,
    G16B16R163PLANE422UNORM = 1000156031,
    G16B16R162PLANE422UNORM = 1000156032,
    G16B16R163PLANE444UNORM = 1000156033,
    PVRTC12BPPUNORMBlockIMG = 1000054000,
    PVRTC14BPPUNORMBlockIMG = 1000054001,
    PVRTC22BPPUNORMBlockIMG = 1000054002,
    PVRTC24BPPUNORMBlockIMG = 1000054003,
    PVRTC12BPPSRGBBlockIMG = 1000054004,
    PVRTC14BPPSRGBBlockIMG = 1000054005,
    PVRTC22BPPSRGBBlockIMG = 1000054006,
    PVRTC24BPPSRGBBlockIMG = 1000054007,
    ASTC4X4SFLOATBlockEXT = 1000066000,
    ASTC5X4SFLOATBlockEXT = 1000066001,
    ASTC5X5SFLOATBlockEXT = 1000066002,
    ASTC6X5SFLOATBlockEXT = 1000066003,
    ASTC6X6SFLOATBlockEXT = 1000066004,
    ASTC8X5SFLOATBlockEXT = 1000066005,
    ASTC8X6SFLOATBlockEXT = 1000066006,
    ASTC8X8SFLOATBlockEXT = 1000066007,
    ASTC10X5SFLOATBlockEXT = 1000066008,
    ASTC10X6SFLOATBlockEXT = 1000066009,
    ASTC10X8SFLOATBlockEXT = 1000066010,
    ASTC10X10SFLOATBlockEXT = 1000066011,
    ASTC12X10SFLOATBlockEXT = 1000066012,
    ASTC12X12SFLOATBlockEXT = 1000066013,
    A4R4G4B4UNORMPack16EXT = 1000340000,
    A4B4G4R4UNORMPack16EXT = 1000340001,
    // G8B8G8R8_422_UNORM_KHR = G8B8G8R8_422_UNORM,
    // B8G8R8G8_422_UNORM_KHR = B8G8R8G8_422_UNORM,
    // G8_B8_R8_3PLANE_420_UNORM_KHR = G8_B8_R8_3PLANE_420_UNORM,
    // G8_B8R8_2PLANE_420_UNORM_KHR = G8_B8R8_2PLANE_420_UNORM,
    // G8_B8_R8_3PLANE_422_UNORM_KHR = G8_B8_R8_3PLANE_422_UNORM,
    // G8_B8R8_2PLANE_422_UNORM_KHR = G8_B8R8_2PLANE_422_UNORM,
    // G8_B8_R8_3PLANE_444_UNORM_KHR = G8_B8_R8_3PLANE_444_UNORM,
    // R10X6_UNORM_PACK16_KHR = R10X6_UNORM_PACK16,
    // R10X6G10X6_UNORM_2PACK16_KHR = R10X6G10X6_UNORM_2PACK16,
    // R10X6G10X6B10X6A10X6_UNORM_4PACK16_KHR = R10X6G10X6B10X6A10X6_UNO
    // G10X6B10X6G10X6R10X6_422_UNORM_4PACK16_KHR =
    // 10X6G10X6R10X6_422_UNORM_4PACK16,
    // B10X6G10X6R10X6G10X6_422_UNORM_4PACK16_KHR =
    // 10X6R10X6G10X6_422_UNORM_4PACK16,
    // G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16_KHR =
    // B10X6_R10X6_3PLANE_420_UNORM_3PACK16,
    // G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16_KHR =
    // B10X6R10X6_2PLANE_420_UNORM_3PACK16,
    // G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16_KHR =
    // B10X6_R10X6_3PLANE_422_UNORM_3PACK16,
    // G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16_KHR =
    // B10X6R10X6_2PLANE_422_UNORM_3PACK16,
    // G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16_KHR =
    // B10X6_R10X6_3PLANE_444_UNORM_3PACK16,
    // R12X4_UNORM_PACK16_KHR = R12X4_UNORM_PACK16,
    // R12X4G12X4_UNORM_2PACK16_KHR = R12X4G12X4_UNORM_2PACK16,
    // R12X4G12X4B12X4A12X4_UNORM_4PACK16_KHR = R12X4G12X4B12X4A12X4_UNO
    // G12X4B12X4G12X4R12X4_422_UNORM_4PACK16_KHR =
    // 12X4G12X4R12X4_422_UNORM_4PACK16,
    // B12X4G12X4R12X4G12X4_422_UNORM_4PACK16_KHR =
    // 12X4R12X4G12X4_422_UNORM_4PACK16,
    // G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16_KHR =
    // B12X4_R12X4_3PLANE_420_UNORM_3PACK16,
    // G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16_KHR =
    // B12X4R12X4_2PLANE_420_UNORM_3PACK16,
    // G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16_KHR =
    // B12X4_R12X4_3PLANE_422_UNORM_3PACK16,
    // G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16_KHR =
    // B12X4R12X4_2PLANE_422_UNORM_3PACK16,
    // G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16_KHR =
    // B12X4_R12X4_3PLANE_444_UNORM_3PACK16,
    // G16B16G16R16_422_UNORM_KHR = G16B16G16R16_422_UNORM,
    // B16G16R16G16_422_UNORM_KHR = B16G16R16G16_422_UNORM,
    // G16_B16_R16_3PLANE_420_UNORM_KHR = G16_B16_R16_3PLANE_420_UNORM,
    // G16_B16R16_2PLANE_420_UNORM_KHR = G16_B16R16_2PLANE_420_UNORM,
    // G16_B16_R16_3PLANE_422_UNORM_KHR = G16_B16_R16_3PLANE_422_UNORM,
    // G16_B16R16_2PLANE_422_UNORM_KHR = G16_B16R16_2PLANE_422_UNORM,
    // G16_B16_R16_3PLANE_444_UNORM_KHR = G16_B16_R16_3PLANE_444_UNORM,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum FramebufferCreateFlagBits {
    None = 0x0000_0000,
    // Provided by VK_KHR_imageless_framebuffer
    CreateImagelessKHR = 0x0000_0001,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum FrontFace {
    CounterClockwise = 0,
    Clockwise = 1,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum ImageAspectFlagBits {
    Color = 0x0000_0001,
    Depth = 0x0000_0002,
    Stencil = 0x0000_0004,
    Metadata = 0x0000_0008,
    Plane0EXT = 0x0000_0010,
    Plane1EXT = 0x0000_0020,
    Plane2EXT = 0x0000_0040,
    MemoryPlane0EXT = 0x0000_0080,
    MemoryPlane1EXT = 0x0000_0100,
    MemoryPlane2EXT = 0x0000_0200,
    MemoryPlane3EXT = 0x0000_0400,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum ImageCreateFlagBits {
    None = 0x0000_0000,
    SparseBinding = 0x0000_0001,
    SparseResidency = 0x0000_0002,
    SparseAliased = 0x0000_0004,
    MutableFormat = 0x0000_0008,
    CubeCompatible = 0x0000_0010,
    Array2DCompatibleKHR = 0x0000_0020,
    SplitInstanceBindRegionsKHR = 0x0000_0040,
    BlockTexelViewCompatibleKHR = 0x0000_0080,
    ExtendedUsageKHR = 0x0000_0100,
    DisjointKHR = 0x0000_0200,
    AliasKHR = 0x0000_0400,
    Protected = 0x0000_0800,
    CornerSampledNV = 0x0000_2000,
    SampleLocationsCompatibleDepthEXT = 0x0000_1000,
    SubsampledEXT = 0x0000_4000,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum ImageLayout {
    Undefined = 0,
    General = 1,
    ColorAttachmentOptimal = 2,
    DepthStencilAttachmentOptimal = 3,
    DepthStencilReadOnlyOptimal = 4,
    ShaderReadOnlyOptimal = 5,
    TransferSourceOptimal = 6,
    TransferDestinationOptimal = 7,
    Preinitialized = 8,
    DepthReadOnlyStencilAttachmentOptimalKHR = 1000117000,
    DepthAttachmentStencilReadOnlyOptimalKHR = 1000117001,
    DepthAttachmentOptimalKHR = 1000241000,
    DepthReadOnlyOptimalKHR = 1000241001,
    StencilAttachmentOptimalKHR = 1000241002,
    StencilReadOnlyOptimalKHR = 1000241003,
    PresentSourceKHR = 1000001002,
    SharedPresentKHR = 1000111000,
    FragmentShadingRateAttachmentOptimalKHR = 1000164003, // ShadingRateOptimalNV
    FragmentDensityMapOptimalEXT = 1000218000,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum ImageUsageFlagBits {
    TransferSource = 0x0000_0001,
    TransferDestination = 0x0000_0002,
    Sampled = 0x0000_0004,
    Storage = 0x0000_0008,
    ColorAttachment = 0x0000_0010,
    DepthStencilAttachment = 0x0000_0020,
    TransientAttachment = 0x0000_0040,
    InputAttachment = 0x0000_0080,
    ShadingRateAttachmentBitKHR = 0x0000_0100, // ImageUsageShadingRateImageBitNV
    FragmentDensityMap = 0x0000_0200,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum ImageViewCreateFlags {
    None = 0x0000_0000,
    FragmentDensityMapDynamicEXT = 0x0000_0001,
    FragmentDensityMapDeferredEXT = 0x0000_0002,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum ImageViewType {
    OneDimensional = 0,
    TwoDimensional = 1,
    ThreeDimensional = 2,
    Cube = 3,
    Array1D = 4,
    Array2D = 5,
    ArrayCube = 6,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub enum ImageTiling {
    Optimal = 0,
    Linear = 1,
    DRMFormatModifierEXT = 1000158000,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum ImageType {
    OneDimensional = 0,
    TwoDimensional = 1,
    ThreeDimensional = 2,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum IndexType {
    Uint16 = 0,
    Uint32 = 1,
    NoneKHR = 1_000_165_000,
    Uint8EXT = 1_000_265_000,
    // NoneNV = NoneKHR,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum InternalAllocationType {
    Executable = 0,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum LogicOp {
    Clear = 0,
    And = 1,
    AndReverse = 2,
    Copy = 3,
    AndInverted = 4,
    NoOp = 5,
    Xor = 6,
    Or = 7,
    Nor = 8,
    Equivalent = 9,
    Invert = 10,
    OrReverse = 11,
    CopyInverted = 12,
    OrInverted = 13,
    Nand = 14,
    Set = 15,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum MemoryHeapFlagBits {
    DeviceLocal = 0x0000_0001,
    MultiInstanceKHR = 0x0000_0002,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum MemoryPropertyFlagBits {
    DeviceLocal = 0x0000_0001,
    HostVisible = 0x0000_0002,
    HostCoherent = 0x0000_0004,
    Cached = 0x0000_0008,
    LazilyAllocated = 0x0000_0010,
    Protected = 0x0000_0020,
    DeviceCoherentAMD = 0x0000_0040,
    DeviceUncachedAMD = 0x0000_0080,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum MipmapMode {
    Nearest = 0,
    Linear = 1,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum PhysicalDeviceType {
    Other = 0,
    IntegratedGPU = 1,
    DiscreteGPU = 2,
    VirtualGPU = 3,
    CPU = 4,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum PipelineBindPoint {
    Graphics = 0,
    Compute = 1,
    RayTracingKHR = 10_0016_5000,
    // RayTracingNV = RayTracingKHR,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum PipelineCacheCreateFlagBits {
    ExternallySynchronizedEXT = 0x0000_0001,
}

/// TODO: Incomplete
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum PipelineCreateFlagBits {
    DisableOptimization = 0x0000_0001,
    AllowDerivatives = 0x0000_0002,
    CreateDerivative = 0x0000_0004,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum PipelineShaderStageCreateFlagBits {
    AllowVaryingSubgroupSizeEXT = 0x0000_0001,
    RequireFullSubgroupsEXT = 0x0000_0002,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum PipelineStageFlagBits {
    TopOfPipe = 0x0000_0001,
    DrawIndirect = 0x0000_0002,
    VertexInput = 0x0000_0004,
    VertexShader = 0x0000_0008,
    TessellationControlShader = 0x0000_0010,
    TessellationEvaluationShader = 0x0000_0020,
    GeometryShader = 0x0000_0040,
    FragmentShader = 0x0000_0080,
    EarlyFragmentTests = 0x0000_0100,
    LateFragmentTests = 0x0000_0200,
    ColorAttachmentOutput = 0x0000_0400,
    ComputeShader = 0x0000_0800,
    Transfer = 0x0000_1000,
    BottomOfPipe = 0x0000_2000,
    Host = 0x0000_4000,
    AllGraphics = 0x0000_8000,
    AllCommands = 0x0001_0000,
    TransformFeedbackEXT = 0x0100_0000,
    ConditionalRenderingEXT = 0x0004_0000,
    AccelerationStructureBuildKHR = 0x0200_0000,
    RayTracingShaderKHR = 0x0020_0000,
    FragmentShadingRateAttachmentKHR = 0x0040_0000,
    TaskShaderNV = 0x0008_0000,
    MeshShaderNV = 0x0010_0000,
    FragmentDensityProcessEXT = 0x0080_0000,
    CommandPreprocessNV = 0x0002_0000,
    // RayTracingShaderNV = RAY_TRACING_SHADER_KHR,
    // AccelerationStructureBuildNV = ACCELERATION_STRUCTURE_BUILD_KHR,
    // FragmentShadingRateAttachmentKHR = SHADING_RATE_IMAGE_NV,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum PolygonMode {
    Fill = 0,
    Line = 1,
    Point = 2,
    FillRectangleNV = 1_000_153_000,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum PresentModeKHR {
    Immediate = 0,
    Mailbox = 1,
    Fifo = 2,
    FifoRelaxed = 3,
    SharedDemandRefresh = 1000111000,
    SharedContinuousRefresh = 1000111001,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum PrimitiveTopology {
    PointList = 0,
    LineList = 1,
    LineStrip = 2,
    TriangleList = 3,
    TriangleStrip = 4,
    TriangleFan = 5,
    LineListWithAdjacency = 6,
    LineStripWithAdjacency = 7,
    TriangleListWithAdjacency = 8,
    TriangleStripWithAdjacency = 9,
    PatchList = 10,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum QueryControlFlagBits {
    None = 0x0000_0000,
    Precise = 0x0000_0001,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum QueryPipelineStatiticFlagBits {
    None = 0x0000_0000,
    InputAssemblyVertices = 0x0000_0001,
    InputAssemblyPrimities = 0x0000_0002,
    VertexShaderInvocations = 0x0000_0004,
    GeometryShaderInvocations = 0x0000_0008,
    GeometryShaderPrimities = 0x0000_0010,
    ClippingInvocations = 0x0000_0020,
    ClippingPrimitives = 0x0000_0040,
    FragmentShaderInvocations = 0x0000_0080,
    TesselationControShaderPatches = 0x0000_0100,
    TesselationEvaluationShaderInvocations = 0x0000_0200,
    ComputeShaderInvocations = 0x0000_0400,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum QueueFlagBits {
    Graphics = 0x0000_0001,
    Compute = 0x0000_0002,
    Transfer = 0x0000_0004,
    SparseBinding = 0x0000_0008,
    Protected = 0x0000_0010,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum RenderPassCreateFlagBits {
    TransformQCOM = 0x0000_0002,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum Result {
    Success = 0,
    NotReady = 1,
    Timeout = 2,
    EventSet = 3,
    EventReset = 4,
    Incomplete = 5,
    ErrorOutOfHostMemory = -1,
    ErrorOutOfDeviceMemory = -2,
    ErrorInitializationFailed = -3,
    ErrorDeviceLost = -4,
    ErrorMemoryMapFailed = -5,
    ErrorLayerNotPresent = -6,
    ErrorExtensionNotPresent = -7,
    ErrorFeatureNotPresent = -8,
    ErrorIncompatibleDriver = -9,
    ErrorTooManyObjects = -10,
    ErrorFormatNotSupported = -11,
    ErrorFragmentedPool = -12,
    ErrorUnknown = -13,
    ErrorSurfaceLostKHR = -1_000_000_000,
    ErrorNativeWindowInUseKHR = -1_000_000_001,
    SuboptimalKHR = 1_000_001_003,
    ErrorOutOfDateKHR = -1_000_001_004,
    ErrorIncompatibleDisplayKHR = -1_000_003_001,
    ErrorValidationFailedEXT = -1_000_011_001,
    ErrorInvalidShaderNV = -1_000_012_000,
    ErrorInvalidDRMFormatModifierPlaneLayoutEXT = -1_000_158_000,
    ErrorNotPermittedEXT = -1_000_174_001,
    ErrorFullScreenExclusiveModeLostEXT = -1_000_255_000,
    ThreadIdleKHR = 1_000_268_000,
    ThreadDoneKHR = 1_000_268_001,
    OperationDeferredKHR = 1_000_268_002,
    OperationNotDeferredKHR = 1_000_268_003,
    PipelineCompileRequiredEXT = 1_000_297_000,
    ErrorOutOfPoolMemoryKHR = -1_000_069_000,
    ErrorInvalidExternalHandleKHR = -1_000_072_003,
    ErrorFragmentationEXT = -1_000_161_000,
    ErrorInvalidOpaqueCaptureAddressKHR = -1_000_257_000,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum SamplerAddressMode {
    Repeat = 0,
    MirroredRepeat = 1,
    ClampToEdge = 2,
    ClampToBorder = 3,
    MirrorClampToEdgeKHR = 4,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum SampleCountFlagBits {
    One = 0x00000001,
    Two = 0x00000002,
    Four = 0x00000004,
    Eight = 0x00000008,
    Sixteen = 0x00000010,
    ThirtyTwo = 0x00000020,
    SixtyFour = 0x00000040,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum SamplerMipmapMode {
    Nearest = 0,
    Linear = 1,
}

/// TODO: Incomplete
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum ShaderStageFlagBits {
    Vertex = 0x0000_0001,
    Fragment = 0x0000_0010,
    Compute = 0x0000_0020,
    AllGraphics = 0x0000_001F,
    All = 0x7FFF_FFFF,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum SharingMode {
    Exclusive = 0,
    Concurrent = 1,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum StencilOp {
    Keep = 0,
    Zero = 1,
    Replace = 2,
    IncrementAndClamp = 3,
    DecrementAndClamp = 4,
    Invert = 5,
    IncrementAndWrap = 6,
    DecrementAndWrap = 7,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum StructureType {
    ApplicationInfo = 0,
    InstanceCreateInfo = 1,
    DeviceQueueCreateInfo = 2,
    DeviceCreateInfo = 3,
    SubmitInfo = 4,
    MemoryAllocateInfo = 5,
    MappedMemoryRange = 6,
    FenceCreateInfo = 8,
    SemaphoreCreateInfo = 9,
    BufferCreateInfo = 12,
    BufferViewCreateInfo = 13,
    ImageCreateInfo = 14,
    ImageViewCreateInfo = 15,
    ShaderModuleCreateInfo = 16,
    PipelineShaderStageCreateInfo = 18,
    PipelineVertexInputStateCreateInfo = 19,
    PipelineInputAssemblyStateCreateInfo = 20,
    PIpelineTesselationStateCreateInfo = 21,
    PipelineViewportStateCreateInfo = 22,
    PipelineRasterizationStateCreateInfo = 23,
    PipelineMultisampleStateCreateInfo = 24,
    PipelineDepthStencilStateCreateInfo = 25,
    PipelineColorBlendStateCreateInfo = 26,
    PipelineDynamicStateCreateInfo = 27,
    GraphicsPipelineCreateInfo = 28,
    ComputePipelineCreateInfo = 29,
    PipelineLayoutCreateInfo = 30,
    SamplerCreateInfo = 31,
    DescriptorSetLayoutCreateInfo = 32,
    DescriptorPoolCreateInfo = 33,
    DescriptorSetAllocateInfo = 34,
    WriteDescriptorSet = 35,
    CopyDescriptorSet = 36,
    FramebufferCreateInfo = 37,
    RenderPassCreateInfo = 38,
    CommandPoolCreateInfo = 39,
    CommandBufferAllocateInfo = 40,
    CommandBufferInheritanceInfo = 41,
    CommandBufferBeginInfo = 42,
    RenderPassBeginInfo = 43,
    BufferMemoryBarrier = 44,
    ImageMemoryBarrier = 45,
    MemoryBarrier = 46,
    BindBufferMemoryInfo = 1_000_157_000,
    BindImageMemoryInfo = 1_000_157_001,
    ImageViewUsageCreateInfo = 1_000_117_002,
    MemoryAllocateFlagsInfo = 1_000_060_000,
    DescriptorUpdateTemplateCreateInfo = 1_000_085_000,
    SubpassBeginInfo = 1_000_109_005,
    SubpassEndInfo = 1_000_109_006,
    PhysicalDeviceTimelineSemaphoreFeatures = 1_000_207_000,
    PhysicalDeviceTimelineSemaphoreProperties = 1_000_207_001,
    SemaphoreTypeCreateInfo = 1_000_207_002,
    TimelineSemaphoreSubmitInfo = 1_000_207_003,
    SemaphoreWaitInfo = 1_000_207_004,
    SemaphoreSignalInfo = 1_000_207_005,
    SwapchainCreateInfoKHR = 1_000_001_000,
    PresentInfoKHR = 1_000_001_001,
    XCBSurfaceCreateInfoKHR = 1_000_005_000,
    WIN32SurfaceCreateInfoKHR = 1_000_009_000,
    PipelineInfoKHR = 1_000_269_001,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum SubpassContents {
    Inline = 0,
    SecondaryCommandBuffers = 1,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum SubpassDescriptionFlagBits {
    PerViewAttribtuesNVX = 0x0000_0001,
    PerViewPositionXOnlyNVX = 0x0000_0002,
    FragmentRegionQCOM = 0x0000_0004,
    ShaderResolveQCOM = 0x0000_0008,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum SurfaceTransformFlagBitsKHR {
    Identity = 0x0000_0001,
    Rotate90 = 0x0000_0002,
    Rotate180 = 0x0000_0004,
    Rotate270 = 0x0000_0008,
    HorizontalMirror = 0x0000_0010,
    HorizontalMirrorRotate90 = 0x0000_0020,
    HorizontalMirrorRotate180 = 0x0000_0040,
    HorizontalMirrorRotate270 = 0x0000_0080,
    Inherit = 0x0000_0100,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub enum SwapchainCreateFlagsKHR {
    None = 0x0000_0000,
    SplitInstanceBindRegions = 0x0000_0001,
    Protected = 0x0000_0002,
    MutableFormat = 0x0000_0004,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum SystemAllocationScope {
    Command = 0,
    Object = 1,
    Cache = 2,
    Device = 3,
    Instance = 4,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum VertexInputRate {
    Vertex = 0,
    Instance = 1,
}

// Opaque structures (Handles)
// Handles in Vulkan are divided into dispatchable and non-dispatchable handles.
// Dispatchable handles are always the size of pointers, whereas non-dispatchable handles
// must always be 64-bits wide (implemented as an opaque handle on x86_64 and a u64 on x86)
//  even if the program is running on a 32-bit architecture.
//
// We are only supporting 64-bit architectures, hence, we don't actually distinguish
// between dispatchable and non-dispatchable handles.
define_handle!(
    Buffer,
    BufferView,
    CommandBuffer,
    CommandPool,
    DescriptorPool,
    DescriptorSet,
    DescriptorSetLayout,
    Device,
    DeviceMemory,
    Event,
    Fence,
    Framebuffer,
    Image,
    ImageView,
    Instance,
    PhysicalDevice,
    Pipeline,
    PipelineCache,
    PipelineLayout,
    QueryPool,
    Queue,
    RenderPass,
    Sampler,
    Semaphore,
    ShaderModule,
    SurfaceKHR,
    SwapchainKHR
);

// Public structures
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AllocationCallbacks {
    pub user_data: *const c_void,
    pub alloc: Allocation,
    pub realloc: Reallocation,
    pub free: Free,
    pub internal_alloc: InternalAllocation,
    pub internal_free: InternalFree,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ApplicationInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub application_name: *const c_char,
    pub application_version: u32,
    pub engine_name: *const c_char,
    pub engine_version: u32,
    pub api_version: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct AttachmentDescription {
    pub flags: AttachmentDescriptionFlags,
    pub format: Format,
    pub samples: SampleCountFlagBits,
    pub load_op: AttachmentLoadOp,
    pub store_op: AttachmentStoreOp,
    pub stencil_load_op: AttachmentLoadOp,
    pub stencil_store_op: AttachmentStoreOp,
    pub initial_layout: ImageLayout,
    pub final_layout: ImageLayout,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct AttachmentReference {
    pub attachment: u32,
    pub layout: ImageLayout,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct BufferCopy {
    pub src_offset: DeviceSize,
    pub dst_offset: DeviceSize,
    pub size: DeviceSize,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct BufferCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: BufferCreateFlags,
    pub size: DeviceSize,
    pub usage: BufferUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub queue_family_indices: *const u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct BufferImageCopy {
    pub buffer_offset: DeviceSize,
    pub buffer_row_length: u32,
    pub buffer_image_height: u32,
    pub image_subresource: ImageSubresourceLayers,
    pub image_offset: Offset3D,
    pub image_extent: Extent3D,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct BufferMemoryBarrier {
    pub stype: StructureType,
    pub next: *const c_void,
    pub source_access_mask: AccessFlags,
    pub destination_access_mask: AccessFlags,
    pub source_queue_family_index: u32,
    pub destination_queue_family_index: u32,
    pub buffer: *mut Buffer,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct BufferViewCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: BufferViewCreateFlags,
    pub buffer: *mut Buffer,
    pub format: Format,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct ClearAttachment {
    pub aspect_mask: ImageAspectFlags,
    pub color_attachment: u32,
    pub clear_value: ClearValue,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ClearDepthStencilValue {
    pub depth: f32,
    pub stencil: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ClearRect {
    pub rect: Rect2D,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct CommandBufferAllocateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub command_pool: *mut CommandPool,
    pub level: CommandBufferLevel,
    pub command_buffer_count: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct CommandBufferBeginInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub usage: CommandBufferUsageFlags,
    pub inheritance_info: *const CommandBufferInheritanceInfo,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct CommandBufferInheritanceInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub render_pass: *mut RenderPass,
    pub subpass: u32,
    pub framebuffer: *mut Framebuffer,
    pub occlusion_query_enable: Bool32,
    pub query_flags: QueryControlFlags,
    pub pipeline_statistics: PipelineStatisticFlags,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct CommandPoolCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: CommandPoolCreateFlags,
    pub queue_family_index: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ComponentMapping {
    pub r: ComponentSwizzle,
    pub g: ComponentSwizzle,
    pub b: ComponentSwizzle,
    pub a: ComponentSwizzle,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ComputePipelineCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage: PipelineShaderStageCreateInfo,
    pub layout: *mut PipelineLayout,
    pub base_pipeline_handle: *mut Pipeline,
    pub base_pipeline_index: i32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct CopyDescriptorSet {
    pub stype: StructureType,
    pub next: *const c_void,
    pub src_set: *mut DescriptorSet,
    pub src_binding: u32,
    pub src_array_element: u32,
    pub dst_set: *mut DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DescriptorBufferInfo {
    pub buffer: *mut Buffer,
    pub offset: DeviceSize,
    pub range: DeviceSize,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DescriptorImageInfo {
    pub sampler: *mut Sampler,
    pub image_view: *mut ImageView,
    pub image_layout: ImageLayout,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DescriptorPoolCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: DescriptorPoolCreateFlags,
    pub max_sets: u32,
    pub pool_size_count: u32,
    pub pool_sizes: *const DescriptorPoolSize,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DescriptorPoolSize {
    pub dtype: DescriptorType,
    pub descriptor_count: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DescriptorSetAllocateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub descriptor_pool: *mut DescriptorPool,
    pub descriptor_set_count: u32,
    pub set_layouts: *const *mut DescriptorSetLayout,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DescriptorSetLayoutBinding {
    pub binding: u32,
    pub descriptor_type: DescriptorType,
    pub descriptor_count: u32,
    pub stage_flags: ShaderStageFlags,
    pub immutable_samplers: *const *mut Sampler,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DescriptorSetLayoutCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: DescriptorSetLayoutCreateFlags,
    pub binding_count: u32,
    pub bindings: *const DescriptorSetLayoutBinding,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct DeviceCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: u32,
    pub queue_create_info_count: u32,
    pub queue_create_infos: *const DeviceQueueCreateInfo,
    pub enabled_layer_count: u32,
    pub enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub enabled_extension_names: *const *const c_char,
    pub enabled_features: *const PhysicalDeviceFeatures,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct DeviceQueueCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: u32,
    pub queue_family_index: u32,
    pub queue_count: u32,
    pub queue_priorities: *const f32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DispatchIndirectCommand {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DrawIndexedIndirectCommand {
    pub index_count: u32,
    pub instance_count: u32,
    pub first_index: u32,
    pub vertex_offset: u32,
    pub first_instance: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct DrawIndirectCommand {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub first_vertex: u32,
    pub first_instance: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ExtensionProperties {
    pub extension_name: [c_char; MAX_EXTENSION_NAME_SIZE],
    pub spec_version: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Extent2D {
    pub width: u32,
    pub height: u32,
}

impl From<(u32, u32)> for Extent2D {
    fn from(t: (u32, u32)) -> Self {
        Self {
            width: t.0,
            height: t.1,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(C)]
pub struct Extent3D {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl From<(u32, u32, u32)> for Extent3D {
    fn from(t: (u32, u32, u32)) -> Self {
        Self {
            width: t.0,
            height: t.1,
            depth: t.2,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct FenceCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: FenceCreateFlags,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct FramebufferCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: FramebufferCreateFlags,
    pub render_pass: *mut RenderPass,
    pub attachment_count: u32,
    pub attachments: *const *mut ImageView,
    pub width: u32,
    pub height: u32,
    pub layers: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct FormatProperties {
    pub max_extent: Extent3D,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: SampleCountFlags,
    pub max_resource_size: DeviceSize,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct GraphicsPipelineCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: PipelineCreateFlags,
    pub stage_count: u32,
    pub stages: *const PipelineShaderStageCreateInfo,
    pub vertex_input_state: *const PipelineVertexInputStateCreateInfo,
    pub input_assembly_state: *const PipelineInputAssemblyStateCreateInfo,
    pub tessellation_state: *const PipelineTesselationStateCreateInfo,
    pub viewport_state: *const PipelineViewportStateCreateInfo,
    pub rasterization_state: *const PipelineRasterizationStateCreateInfo,
    pub multisample_state: *const PipelineMultisampleStateCreateInfo,
    pub depth_stencil_state: *const PipelineDepthStencilStateCreateInfo,
    pub color_blend_state: *const PipelineColorBlendStateCreateInfo,
    pub dynamic_state: *const PipelineDynamicStateCreateInfo,
    pub layout: *mut PipelineLayout,
    pub render_pass: *mut RenderPass,
    pub subpass: u32,
    pub base_pipeline_handle: *mut Pipeline,
    pub base_pipeline_index: i32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ImageCopy {
    pub src_subresource: ImageSubresourceLayers,
    pub src_offset: Offset3D,
    pub dst_subresources: ImageSubresourceLayers,
    pub dst_offset: Offset3D,
    pub extent: Extent3D,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ImageCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: ImageCreateFlags,
    pub image_type: ImageType,
    pub format: Format,
    pub extent: Extent3D,
    pub mip_levels: u32,
    pub array_layers: u32,
    pub samples: SampleCountFlags,
    pub tiling: ImageTiling,
    pub usage: ImageUsageFlags,
    pub sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub queue_family_indices: *const u32,
    pub initial_layout: ImageLayout,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ImageFormatProperties {
    pub max_extent: Extent3D,
    pub max_mip_levels: u32,
    pub max_array_layers: u32,
    pub sample_counts: SampleCountFlags,
    pub max_resource_size: DeviceSize,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ImageMemoryBarrier {
    pub stype: StructureType,
    pub next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
    pub old_layout: ImageLayout,
    pub new_layout: ImageLayout,
    pub src_queue_family_index: u32,
    pub dst_queue_family_index: u32,
    pub image: *mut Image,
    pub subresource_range: ImageSubresourceRange,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ImageSubresource {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub array_layer: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ImageSubresourceLayers {
    pub aspect_mask: ImageAspectFlags,
    pub mip_level: u32,
    pub base_array_layer: u32,
    pub layer_count: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ImageSubresourceRange {
    pub aspect_mask: ImageAspectFlags,
    pub base_mip_level: u32,
    pub mip_levels: u32,
    pub base_array_layer: u32,
    pub array_layers: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct ImageViewCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: ImageViewCreateFlags,
    pub image: *mut Image,
    pub view_type: ImageViewType,
    pub format: Format,
    pub components: ComponentMapping,
    pub subresource_range: ImageSubresourceRange,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct InstanceCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: InstanceCreateFlags,
    pub application_info: *const ApplicationInfo,
    pub enabled_layer_count: u32,
    pub enabled_layer_names: *const *const c_char,
    pub enabled_extension_count: u32,
    pub enabled_extension_names: *const *const c_char,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MappedMemoryRange {
    pub stype: StructureType,
    pub next: *const c_void,
    pub memory: DeviceMemory,
    pub offset: DeviceSize,
    pub size: DeviceSize,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MemoryAllocateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub allocation_size: DeviceSize,
    pub memory_type_index: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MemoryBarrier {
    pub stype: StructureType,
    pub next: *const c_void,
    pub src_access_mask: AccessFlags,
    pub dst_access_mask: AccessFlags,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MemoryHeap {
    pub size: DeviceSize,
    pub flags: MemoryHeapFlags,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MemoryRequirements {
    pub size: DeviceSize,
    pub alignment: DeviceSize,
    pub memory_type_bits: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct MemoryType {
    pub property_flags: MemoryPropertyFlags,
    pub heap_index: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Offset2D {
    pub x: i32,
    pub y: i32,
}

impl From<(i32, i32)> for Offset2D {
    fn from((x, y): (i32, i32)) -> Self {
        return Self { x, y };
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Offset3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl From<(i32, i32, i32)> for Offset3D {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        return Self { x, y, z };
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct PhysicalDeviceFeatures {
    pub robust_buffer_access: Bool32,
    pub full_draw_index_uint32: Bool32,
    pub image_cube_array: Bool32,
    pub independent_blend: Bool32,
    pub geometry_shader: Bool32,
    pub tessellation_shader: Bool32,
    pub sample_rate_shading: Bool32,
    pub dual_src_blend: Bool32,
    pub logic_op: Bool32,
    pub multi_draw_indirect: Bool32,
    pub draw_indirect_first_instance: Bool32,
    pub depth_clamp: Bool32,
    pub depth_bias_clamp: Bool32,
    pub fill_mode_non_solid: Bool32,
    pub depth_bounds: Bool32,
    pub wide_lines: Bool32,
    pub large_points: Bool32,
    pub alpha_to_one: Bool32,
    pub multi_viewport: Bool32,
    pub sampler_anisotropy: Bool32,
    pub texture_compression_etc2: Bool32,
    pub texture_compression_astc_ldr: Bool32,
    pub texture_compression_b_c: Bool32,
    pub occlusion_query_precise: Bool32,
    pub pipeline_statistics_query: Bool32,
    pub vertex_pipeline_stores_and_atomics: Bool32,
    pub fragment_stores_and_atomics: Bool32,
    pub shader_tessellation_and_geometry_point_size: Bool32,
    pub shader_image_gather_extended: Bool32,
    pub shader_storage_image_extended_formats: Bool32,
    pub shader_storage_image_multisample: Bool32,
    pub shader_storage_image_read_without_format: Bool32,
    pub shader_storage_image_write_without_format: Bool32,
    pub shader_uniform_buffer_array_dynamic_indexing: Bool32,
    pub shader_sampled_image_array_dynamic_indexing: Bool32,
    pub shader_storage_buffer_array_dynamic_indexing: Bool32,
    pub shader_storage_image_array_dynamic_indexing: Bool32,
    pub shader_clip_distance: Bool32,
    pub shader_cull_distance: Bool32,
    pub shader_float64: Bool32,
    pub shader_int64: Bool32,
    pub shader_int16: Bool32,
    pub shader_resource_residency: Bool32,
    pub shader_resource_min_lod: Bool32,
    pub sparse_binding: Bool32,
    pub sparse_residency_buffer: Bool32,
    pub sparse_residency_image2_d: Bool32,
    pub sparse_residency_image3_d: Bool32,
    pub sparse_residency2_samples: Bool32,
    pub sparse_residency4_samples: Bool32,
    pub sparse_residency8_samples: Bool32,
    pub sparse_residency16_samples: Bool32,
    pub sparse_residency_aliased: Bool32,
    pub variable_multisample_rate: Bool32,
    pub inherited_queries: Bool32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct PhysicalDeviceLimits {
    pub max_image_dimension_1d: u32,
    pub max_image_dimension_2d: u32,
    pub max_image_dimension_3d: u32,
    pub max_image_dimension_cube: u32,
    pub max_image_array_layers: u32,
    pub max_texel_buffer_elements: u32,
    pub max_uniform_buffer_range: u32,
    pub max_storage_buffer_range: u32,
    pub max_push_constants_size: u32,
    pub max_memory_allocation_count: u32,
    pub max_sampler_allocation_count: u32,
    pub buffer_image_granularity: DeviceSize,
    pub sparse_address_space_size: DeviceSize,
    pub max_bound_descriptor_sets: u32,
    pub max_per_stage_descriptor_samplers: u32,
    pub max_per_stage_descriptor_uniform_buffers: u32,
    pub max_per_stage_descriptor_storage_buffers: u32,
    pub max_per_stage_descriptor_sampled_images: u32,
    pub max_per_stage_descriptor_storage_images: u32,
    pub max_per_stage_descriptor_input_attachments: u32,
    pub max_per_stage_resources: u32,
    pub max_descriptor_set_samplers: u32,
    pub max_descriptor_set_uniform_buffers: u32,
    pub max_descriptor_set_uniform_buffers_dynamic: u32,
    pub max_descriptor_set_storage_buffers: u32,
    pub max_descriptor_set_storage_buffers_dynamic: u32,
    pub max_descriptor_set_sampled_images: u32,
    pub max_descriptor_set_storage_images: u32,
    pub max_descriptor_set_input_attachments: u32,
    pub max_vertex_input_attributes: u32,
    pub max_vertex_input_bindings: u32,
    pub max_vertex_input_attribute_offset: u32,
    pub max_vertex_input_binding_stride: u32,
    pub max_vertex_output_components: u32,
    pub max_tessellation_generation_level: u32,
    pub max_tessellation_patch_size: u32,
    pub max_tessellation_control_per_vertex_input_components: u32,
    pub max_tessellation_control_per_vertex_output_components: u32,
    pub max_tessellation_control_per_patch_output_components: u32,
    pub max_tessellation_control_total_output_components: u32,
    pub max_tessellation_evaluation_input_components: u32,
    pub max_tessellation_evaluation_output_components: u32,
    pub max_geometry_shader_invocations: u32,
    pub max_geometry_input_components: u32,
    pub max_geometry_output_components: u32,
    pub max_geometry_output_vertices: u32,
    pub max_geometry_total_output_components: u32,
    pub max_fragment_input_components: u32,
    pub max_fragment_output_attachments: u32,
    pub max_fragment_dual_src_attachments: u32,
    pub max_fragment_combined_output_resources: u32,
    pub max_compute_shared_memory_size: u32,
    pub max_compute_work_group_count: [u32; 3],
    pub max_compute_work_group_invocations: u32,
    pub max_compute_work_group_size: [u32; 3],
    pub sub_pixel_precision_bits: u32,
    pub sub_texel_precision_bits: u32,
    pub mipmap_precision_bits: u32,
    pub max_draw_indexed_index_value: u32,
    pub max_draw_indirect_count: u32,
    pub max_sampler_lod_bias: f32,
    pub max_sampler_anisotropy: f32,
    pub max_viewports: u32,
    pub max_viewport_dimensions: [u32; 2],
    pub viewport_bounds_range: [f32; 2],
    pub viewport_sub_pixel_bits: u32,
    pub min_memory_map_alignment: usize,
    pub min_texel_buffer_offset_alignment: DeviceSize,
    pub min_uniform_buffer_offset_alignment: DeviceSize,
    pub min_storage_buffer_offset_alignment: DeviceSize,
    pub min_texel_offset: i32,
    pub max_texel_offset: u32,
    pub min_texel_gather_offset: i32,
    pub max_texel_gather_offset: u32,
    pub min_interpolation_offset: f32,
    pub max_interpolation_offset: f32,
    pub sub_pixel_interpolation_offset_bits: u32,
    pub max_framebuffer_width: u32,
    pub max_framebuffer_height: u32,
    pub max_framebuffer_layers: u32,
    pub framebuffer_color_sample_counts: SampleCountFlags,
    pub framebuffer_depth_sample_counts: SampleCountFlags,
    pub framebuffer_stencil_sample_counts: SampleCountFlags,
    pub framebuffer_no_attachments_sample_counts: SampleCountFlags,
    pub max_color_attachments: u32,
    pub sampled_image_color_sample_counts: SampleCountFlags,
    pub sampled_image_integer_sample_counts: SampleCountFlags,
    pub sampled_image_depth_sample_counts: SampleCountFlags,
    pub sampled_image_stencil_sample_counts: SampleCountFlags,
    pub storage_image_sample_counts: SampleCountFlags,
    pub max_sample_mask_words: u32,
    pub timestamp_compute_and_graphics: u32,
    pub timestamp_period: f32,
    pub max_clip_distances: u32,
    pub max_cull_distances: u32,
    pub max_combined_clip_and_cull_distances: u32,
    pub discrete_queue_priorities: u32,
    pub point_size_range: [f32; 2],
    pub line_width_range: [f32; 2],
    pub point_size_granularity: f32,
    pub line_width_granularity: f32,
    pub strict_lines: u32,
    pub standard_sample_locations: u32,
    pub optimal_buffer_copy_offset_alignment: DeviceSize,
    pub optimal_buffer_copy_row_pitch_alignment: DeviceSize,
    pub non_coherent_atom_size: DeviceSize,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PhysicalDeviceMemoryProperties {
    pub memory_type_count: u32,
    pub memory_types: [MemoryType; MAX_MEMORY_TYPES],
    pub memory_heap_count: u32,
    pub memory_heaps: [MemoryHeap; MAX_MEMORY_HEAPS],
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct PhysicalDeviceProperties {
    pub api_version: u32,
    pub driver_version: u32,
    pub vendor_id: u32,
    pub device_id: u32,
    pub device_type: PhysicalDeviceType,
    pub device_name: [u8; MAX_PHYSICAL_DEVICE_NAME_SIZE],
    pub pipeline_cache_uuid: [u8; UUID_SIZE],
    pub limits: PhysicalDeviceLimits,
    pub sparse_properties: PhysicalDeviceSparseProperties,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct PhysicalDeviceSparseProperties {
    pub residency_standard_2d_block_shape: u32,
    pub residency_standard_2d_multisample_block_shape: u32,
    pub residency_standard_3d_block_shape: u32,
    pub residency_aligned_mip_size: u32,
    pub residency_no_resident_strict: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineCacheCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: PipelineCacheCreateFlags,
    pub initial_data_size: usize,
    pub initial_data: *const c_void,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineColorBlendAttachmentState {
    pub blend_enable: Bool32,
    pub src_color_blend_factor: BlendFactor,
    pub dst_color_blend_factor: BlendFactor,
    pub color_blend_op: BlendOp,
    pub src_alpha_blend_factor: BlendFactor,
    pub dst_alpha_blend_factor: BlendFactor,
    pub alpha_blend_op: BlendOp,
    pub color_write_mask: ColorComponentFlags,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineColorBlendStateCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: PipelineColorBlendStateCreateFlags,
    pub logic_op_enable: Bool32,
    pub logic_op: LogicOp,
    pub attachment_count: u32,
    pub attachments: *const PipelineColorBlendAttachmentState,
    pub blend_constants: [f32; 4],
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineDepthStencilStateCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: PipelineDepthStencilStateCreateFlags,
    pub depth_test_enable: Bool32,
    pub depth_write_enable: Bool32,
    pub front: StencilOpState,
    pub back: StencilOpState,
    pub min_depth_bounds: f32,
    pub max_depth_bounds: f32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineDynamicStateCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: PipelineDynamicStateCreateFlags,
    pub dynamic_state_count: u32,
    pub dynamic_states: *const DynamicState,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineInputAssemblyStateCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: PipelineInputAssemblyStateCreateFlags,
    pub topology: PrimitiveTopology,
    pub primitive_restart_enabled: Bool32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineLayoutCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: PipelineLayoutCreateFlags,
    pub set_layout_count: u32,
    pub set_layouts: *const *mut DescriptorSetLayout,
    pub push_constant_range_count: u32,
    pub push_constant_ranges: *const PushConstantRange,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineMultisampleStateCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: PipelineMultisampleStateCreateFlags,
    pub rasterization_samples: SampleCountFlagBits,
    pub sample_shading_enabled: Bool32,
    pub min_sample_shading: f32,
    pub sample_mask: *const SampleMask,
    pub alpha_to_coverage_enable: Bool32,
    pub alpha_to_one_enable: Bool32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineRasterizationStateCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: PipelineRasterizationStateCreateFlags,
    pub depth_clamp_enable: Bool32,
    pub rasterizer_discard_enable: u32,
    pub polygon_mode: PolygonMode,
    pub cull_mode: CullModeFlags,
    pub front_face: FrontFace,
    pub depth_bias_enable: Bool32,
    pub depth_bias_constant_factor: f32,
    pub depth_bias_clamp: f32,
    pub depth_bias_slope_factor: f32,
    pub line_width: f32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineShaderStageCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: PipelineShaderStageCreateFlags,
    pub stage: ShaderStageFlagBits,
    pub module: *mut ShaderModule,
    pub name: *const c_char,
    pub specialization_info: *const SpecializationInfo,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineTesselationStateCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: PipelineTesselationStateCreateFlags,
    pub patch_control_points: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineVertexInputStateCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: PipelineVertexInputStateCreateFlags,
    pub vertex_binding_description_count: u32,
    pub vertex_binding_descriptions: *const VertexInputBindingDescription,
    pub vertex_attribute_description_count: u32,
    pub vertex_attribute_descriptions: *const VertexInputAttributeDescription,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PipelineViewportStateCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: PipelineViewportStateCreateFlags,
    pub viewport_count: u32,
    pub viewports: *const Viewport,
    pub scissor_count: u32,
    pub scissors: *const Rect2D,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct PresentInfoKHR {
    pub stype: StructureType,
    pub next: *const c_void,
    pub wait_semaphore_count: u32,
    pub wait_semaphores: *const *mut Semaphore,
    pub swapchain_count: u32,
    pub swapchains: *const *mut SwapchainKHR,
    pub image_indices: *const u32,
    pub results: *mut Result,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PushConstantRange {
    pub stage_flags: ShaderStageFlags,
    pub offset: u32,
    pub size: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct QueueFamilyProperties {
    pub queue_flags: QueueFlags,
    pub queue_count: u32,
    pub timestamp_valid_bits: u32,
    pub min_image_transfer_granularity: Extent3D,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Rect2D {
    pub offset: Offset2D,
    pub extent: Extent2D,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct RenderPassBeginInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub render_pass: *mut RenderPass,
    pub framebuffer: *mut Framebuffer,
    pub render_area: Rect2D,
    pub clear_value_count: u32,
    pub clear_values: *const ClearValue,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct RenderPassCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: RenderPassCreateFlags,
    pub attachment_count: u32,
    pub attachments: *const AttachmentDescription,
    pub subpass_count: u32,
    pub subpasses: *const SubpassDescription,
    pub dependency_count: u32,
    pub dependencies: *const SubpassDependency,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SamplerCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: SamplerCreateFlags,
    pub mag_filter: Filter,
    pub min_filter: Filter,
    pub mipmap_mode: SamplerMipmapMode,
    pub address_mode_u: SamplerAddressMode,
    pub address_mode_v: SamplerAddressMode,
    pub address_mode_w: SamplerAddressMode,
    pub mip_lod_bias: f32,
    pub anisotropy_enable: Bool32,
    pub max_anisotropy: f32,
    pub compare_enable: Bool32,
    pub compare_op: CompareOp,
    pub min_lod: f32,
    pub max_lod: f32,
    pub border_color: BorderColor,
    pub unnormalized_coordinates: Bool32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct ShaderModuleCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: ShaderModuleCreateFlags,
    pub code_size: usize,
    pub code: *const u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SemaphoreCreateInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: SemaphoreCreateFlags,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SpecializationMapEntry {
    pub constant_id: u32,
    pub offset: u32,
    pub size: usize,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SpecializationInfo {
    pub map_entry_count: u32,
    pub map_entries: *const SpecializationMapEntry,
    pub data_size: usize,
    pub data: *const c_void,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct StencilOpState {
    pub fail_op: StencilOp,
    pub pass_op: StencilOp,
    pub depth_fail_op: StencilOp,
    pub compare_op: CompareOp,
    pub compare_mask: u32,
    pub write_mask: u32,
    pub reference: u32,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SubmitInfo {
    pub stype: StructureType,
    pub next: *const c_void,
    pub wait_semaphore_count: u32,
    pub wait_semaphores: *const *mut Semaphore,
    pub wait_dst_stage_mask: *const PipelineStageFlags,
    pub command_buffer_count: u32,
    pub command_buffers: *const *mut CommandBuffer,
    pub signal_semaphore_count: u32,
    pub signal_semaphores: *const *mut Semaphore,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SubpassDependency {
    pub source_subpass: u32,
    pub destination_subpass: u32,
    pub source_stage_mask: PipelineStageFlags,
    pub destination_stage_mask: PipelineStageFlags,
    pub source_access_mask: AccessFlags,
    pub destination_access_mask: AccessFlags,
    pub dependency_flags: DependencyFlags,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SubpassDescription {
    pub flags: SubpassDescriptionFlags,
    pub pipeline_bind_point: PipelineBindPoint,
    pub input_attachment_count: u32,
    pub input_attachments: *const AttachmentReference,
    pub color_attachment_count: u32,
    pub color_attachments: *const AttachmentReference,
    pub resolve_attachments: *const AttachmentReference,
    pub depth_stencil_attachment: *const AttachmentReference,
    pub preserve_attachment_count: u32,
    pub preserve_attachments: *const u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct SubresourceLayout {
    pub offset: DeviceSize,
    pub size: DeviceSize,
    pub row_pitch: DeviceSize,
    pub array_pitch: DeviceSize,
    pub depth_pitch: DeviceSize,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SurfaceCapabilitiesKHR {
    pub min_image_count: u32,
    pub max_image_count: u32,
    pub current_extent: Extent2D,
    pub min_image_extent: Extent2D,
    pub max_image_extent: Extent2D,
    pub max_image_array_layers: u32,
    pub supported_transforms: SurfaceTransformFlagsKHR,
    pub current_transform: SurfaceTransformFlagBitsKHR,
    pub supported_composite_alpha: CompositeAlphaFlagsKHR,
    pub supported_usage_flags: ImageUsageFlagBits,
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct SurfaceFormatKHR {
    pub format: Format,
    pub color_space: ColorSpaceKHR,
}

#[repr(C)]
pub struct SwapchainCreateInfoKHR {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: SwapchainCreateFlagsKHR,
    pub surface: *mut SurfaceKHR,
    pub min_image_count: u32,
    pub image_format: Format,
    pub image_color_space: ColorSpaceKHR,
    pub image_extent: Extent2D,
    pub image_array_layers: u32,
    pub image_usage: ImageUsageFlagBits,
    pub image_sharing_mode: SharingMode,
    pub queue_family_index_count: u32,
    pub queue_family_indices: *const u32,
    pub pre_transform: SurfaceTransformFlagBitsKHR,
    pub composite_alpha: CompositeAlphaFlagsKHR,
    pub present_mode: PresentModeKHR,
    pub clipped: Bool32,
    pub old_swapchain: *mut SwapchainKHR,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VertexInputAttributeDescription {
    pub location: u32,
    pub binding: u32,
    pub format: Format,
    pub offset: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct VertexInputBindingDescription {
    pub binding: u32,
    pub stride: u32,
    pub input_rate: VertexInputRate,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Viewport {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct WriteDescriptorSet {
    pub stype: StructureType,
    pub next: *const c_void,
    pub dst_set: *mut DescriptorSet,
    pub dst_binding: u32,
    pub dst_array_element: u32,
    pub descriptor_count: u32,
    pub descriptor_type: DescriptorType,
    pub image_info: *const DescriptorImageInfo,
    pub buffer_info: *const DescriptorBufferInfo,
    pub texel_buffer_view: *const BufferView,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct XcbSurfaceCreateInfoKHR {
    pub stype: StructureType,
    pub next: *const c_void,
    pub flags: XcbSurfaceCreateFlagsKHR,
    pub connection: *mut super::xcb::Connection,
    pub window_id: u32,
}

// Unions
#[derive(Clone, Copy)]
#[repr(C)]
pub union ClearColorValue {
    pub float: [f32; 4],
    pub int: [i32; 4],
    pub uint: [u32; 4],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub union ClearValue {
    pub color: ClearColorValue,
    pub depth_stencil: ClearDepthStencilValue,
}

// Function pointer definitions
pub type AcquireNextImageKHR = extern "system" fn(
    device: *mut Device,
    swapchain: *mut SwapchainKHR,
    timeout: u64,
    semaphore: *mut Semaphore,
    fence: *mut Fence,
    image_index: *mut u32,
) -> Result;

pub type AllocateCommandBuffers = extern "system" fn(
    device: *mut Device,
    info: *const CommandBufferAllocateInfo,
    command_buffers: *mut *mut CommandBuffer,
) -> Result;

pub type AllocateDescriptorSets = extern "system" fn(
    device: *mut Device,
    info: *const DescriptorSetAllocateInfo,
    descriptor_sets: *mut *mut DescriptorSet,
) -> Result;

pub type AllocateMemory = extern "system" fn(
    device: *mut Device,
    allocate_info: *const MemoryAllocateInfo,
    allocator: *const AllocationCallbacks,
    memory: *mut *mut DeviceMemory,
) -> Result;

pub type Allocation = extern "system" fn(
    user_data: *mut c_void,
    size: usize,
    alignment: usize,
    scope: SystemAllocationScope,
) -> *mut c_void;

pub type BeginCommandBuffer = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    info: *const CommandBufferBeginInfo,
) -> Result;

pub type BindBufferMemory = extern "system" fn(
    device: *mut Device,
    buffer: *mut Buffer,
    memory: *mut DeviceMemory,
    memory_offset: DeviceSize,
) -> Result;

pub type BindImageMemory = extern "system" fn(
    device: *mut Device,
    image: *mut Image,
    memory: *mut DeviceMemory,
    memory_offset: DeviceSize,
) -> Result;

pub type CmdBeginRenderPass = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    info: *const RenderPassBeginInfo,
    contents: SubpassContents,
);

pub type CmdBindDescriptorSets = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: *mut PipelineLayout,
    first_set: u32,
    descriptor_set_count: u32,
    descriptor_sets: *const *mut DescriptorSet,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
);

pub type CmdBindIndexBuffer = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    buffer: *mut Buffer,
    offset: DeviceSize,
    index_type: IndexType,
);

pub type CmdBindPipeline = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: *mut Pipeline,
);

pub type CmdBindVertexBuffers = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    first_binding: u32,
    binding_count: u32,
    buffers: *const *mut Buffer,
    offsets: *const DeviceSize,
);

pub type CmdCopyBuffer = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    src_buffer: *mut Buffer,
    dst_buffer: *mut Buffer,
    region_count: u32,
    regions: *const BufferCopy,
);

pub type CmdCopyBufferToImage = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    src_buffer: *mut Buffer,
    dst_image: *mut Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    regions: *const BufferImageCopy,
);

pub type CmdCopyImage = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    src_image: *mut Image,
    src_image_layout: ImageLayout,
    dst_image: *mut Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    regions: *const ImageCopy,
);

pub type CmdCopyImageToBuffer = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    src_image: *mut Image,
    src_image_layout: ImageLayout,
    dst_buffer: *mut Buffer,
    region_count: u32,
    regions: *const BufferImageCopy,
);

pub type CmdDraw = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
);

pub type CmdDrawIndexed = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
);

pub type CmdDrawIndexedIndirect = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    buffer: *mut Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);

pub type CmdDrawIndirect = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    buffer: *mut Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);

pub type CmdEndRenderPass = extern "system" fn(command_buffer: *mut CommandBuffer);

pub type CmdExecuteCommands = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    command_buffer_count: u32,
    command_buffers: *const *mut CommandBuffer,
);

pub type CmdPipelineBarrier = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    dependency_flags: DependencyFlags,
    memory_barrier_count: u32,
    memory_barriers: *const MemoryBarrier,
    buffer_memory_barrier_count: u32,
    buffer_memory_barriers: *const BufferMemoryBarrier,
    image_memory_barrier_count: u32,
    image_memory_barriers: *const ImageMemoryBarrier,
);

pub type CmdNextSubpass =
    extern "system" fn(command_buffer: *mut CommandBuffer, contents: SubpassContents);

pub type CmdPushConstants = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    layout: *mut PipelineLayout,
    stage_flags: ShaderStageFlags,
    offset: u32,
    size: u32,
    values: *const c_void,
);

pub type CmdSetScissor = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    first_scissor: u32,
    scissor_count: u32,
    scissors: *const Rect2D,
);

pub type CmdSetViewport = extern "system" fn(
    command_buffer: *mut CommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    viewports: *const Viewport,
);

pub type CreateBuffer = extern "system" fn(
    device: *mut Device,
    create_info: *const BufferCreateInfo,
    allocator: *const AllocationCallbacks,
    buffer: *mut *mut Buffer,
) -> Result;

pub type CreateBufferView = extern "system" fn(
    device: *mut Device,
    create_info: *const BufferViewCreateInfo,
    allocator: *const AllocationCallbacks,
    view: *mut *mut BufferView,
) -> Result;

pub type CreateCommandPool = extern "system" fn(
    device: *mut Device,
    info: *const CommandPoolCreateInfo,
    allocator: *const AllocationCallbacks,
    command_pool: *mut *mut CommandPool,
) -> Result;

pub type CreateComputePipelines = extern "system" fn(
    device: *mut Device,
    pipeline_cache: *mut PipelineCache,
    create_info_count: u32,
    create_infos: *const ComputePipelineCreateInfo,
    allocator: *const AllocationCallbacks,
    pipelines: *mut *mut Pipeline,
) -> Result;

pub type CreateDescriptorPool = extern "system" fn(
    device: *mut Device,
    create_info: *const DescriptorPoolCreateInfo,
    allocator: *const AllocationCallbacks,
    descriptor_pool: *mut *mut DescriptorPool,
) -> Result;

pub type CreateDescriptorSetLayout = extern "system" fn(
    device: *mut Device,
    create_info: *const DescriptorSetLayoutCreateInfo,
    allocator: *const AllocationCallbacks,
    set_layout: *mut *mut DescriptorSetLayout,
) -> Result;

pub type CreateDevice = extern "system" fn(
    physical_device: *mut PhysicalDevice,
    device_info: *const DeviceCreateInfo,
    allocation: *const AllocationCallbacks,
    device: *mut *mut Device,
) -> Result;

pub type CreateFence = extern "system" fn(
    device: *mut Device,
    info: *const FenceCreateInfo,
    allocator: *const AllocationCallbacks,
    fence: *mut *mut Fence,
) -> Result;

pub type CreateFramebuffer = extern "system" fn(
    device: *mut Device,
    info: *const FramebufferCreateInfo,
    allocator: *const AllocationCallbacks,
    framebuffer: *mut *mut Framebuffer,
) -> Result;

pub type CreateGraphicsPipelines = extern "system" fn(
    device: *mut Device,
    pipeline_cache: *mut PipelineCache,
    create_info_count: u32,
    create_infos: *const GraphicsPipelineCreateInfo,
    allocator: *const AllocationCallbacks,
    pipelines: *mut *mut Pipeline,
) -> Result;

pub type CreateImage = extern "system" fn(
    device: *mut Device,
    info: *const ImageCreateInfo,
    allocator: *const AllocationCallbacks,
    image: *mut *mut Image,
) -> Result;

pub type CreateImageView = extern "system" fn(
    device: *mut Device,
    info: *const ImageViewCreateInfo,
    allocator: *const AllocationCallbacks,
    view: *mut *mut ImageView,
) -> Result;

pub type CreateInstance = extern "system" fn(
    instance_info: *const InstanceCreateInfo,
    allocator: *const AllocationCallbacks,
    instance: *mut *mut Instance,
) -> Result;

pub type CreatePipelineLayout = extern "system" fn(
    device: *mut Device,
    create_info: *const PipelineLayoutCreateInfo,
    allocator: *const AllocationCallbacks,
    pipeline_layout: *mut *mut PipelineLayout,
) -> Result;

pub type CreateRenderPass = extern "system" fn(
    device: *mut Device,
    info: *const RenderPassCreateInfo,
    allocator: *const AllocationCallbacks,
    render_pass: *mut *mut RenderPass,
) -> Result;

pub type CreateSampler = extern "system" fn(
    device: *mut Device,
    create_info: *const SamplerCreateInfo,
    allocator: *const AllocationCallbacks,
    sampler: *mut *mut Sampler,
) -> Result;

pub type CreateSemaphore = extern "system" fn(
    device: *mut Device,
    info: *const SemaphoreCreateInfo,
    allocator: *const AllocationCallbacks,
    semaphore: *mut *mut Semaphore,
) -> Result;

pub type CreateShaderModule = extern "system" fn(
    device: *mut Device,
    create_info: *const ShaderModuleCreateInfo,
    allocator: *const AllocationCallbacks,
    shader_module: *mut *mut ShaderModule,
) -> Result;

pub type CreateSwapchainKHR = extern "system" fn(
    device: *mut Device,
    info: *const SwapchainCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    swapchain: *mut *mut SwapchainKHR,
) -> Result;

#[cfg(target_os = "linux")]
pub type CreateXcbSurfaceKHR = extern "system" fn(
    instance: *mut Instance,
    info: *const XcbSurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut *mut SurfaceKHR,
) -> Result;

pub type DestroyBuffer = extern "system" fn(
    device: *mut Device,
    buffer: *mut Buffer,
    allocator: *const AllocationCallbacks,
);

pub type DestroyBufferView = extern "system" fn(
    device: *mut Device,
    buffer_view: *mut BufferView,
    allocator: *const AllocationCallbacks,
);

pub type DestroyCommandPool = extern "system" fn(
    device: *mut Device,
    command_pool: *mut CommandPool,
    allocator: *const AllocationCallbacks,
);

pub type DestroyDescriptorPool = extern "system" fn(
    device: *mut Device,
    descriptor_pool: *mut DescriptorPool,
    allocation: *const AllocationCallbacks,
);

pub type DestroyDescriptorSetLayout = extern "system" fn(
    device: *mut Device,
    descriptor_set_layout: *mut DescriptorSetLayout,
    allocator: *const AllocationCallbacks,
);

pub type DestroyDevice =
    extern "system" fn(device: *mut Device, allocator: *const AllocationCallbacks);

pub type DestroyFence = extern "system" fn(
    device: *mut Device,
    fence: *mut Fence,
    allocator: *const AllocationCallbacks,
);

pub type DestroyFramebuffer = extern "system" fn(
    device: *mut Device,
    framebuffer: *mut Framebuffer,
    allocator: *const AllocationCallbacks,
);

pub type DestroyImage = extern "system" fn(
    device: *mut Device,
    image: *mut Image,
    allocator: *const AllocationCallbacks,
);

pub type DestroyImageView = extern "system" fn(
    device: *mut Device,
    view: *mut ImageView,
    allocator: *const AllocationCallbacks,
);

pub type DestroyInstance =
    extern "system" fn(instance: *mut Instance, allocator: *const AllocationCallbacks);

pub type DestroyPipeline = extern "system" fn(
    device: *mut Device,
    pipeline: *mut Pipeline,
    allocator: *const AllocationCallbacks,
);

pub type DestroyPipelineLayout = extern "system" fn(
    device: *mut Device,
    pipeline_layout: *mut PipelineLayout,
    allocator: *const AllocationCallbacks,
);

pub type DestroyRenderPass = extern "system" fn(
    device: *mut Device,
    render_pass: *mut RenderPass,
    allocator: *const AllocationCallbacks,
);

pub type DestroySampler = extern "system" fn(
    device: *mut Device,
    sampler: *mut Sampler,
    allocator: *const AllocationCallbacks,
);

pub type DestroySemaphore = extern "system" fn(
    device: *mut Device,
    semaphore: *mut Semaphore,
    allocator: *const AllocationCallbacks,
);

pub type DestroyShaderModule = extern "system" fn(
    device: *mut Device,
    shader_module: *mut ShaderModule,
    allocator: *const AllocationCallbacks,
);

pub type DestroySurfaceKHR = extern "system" fn(
    instance: *mut Instance,
    surface: *mut SurfaceKHR,
    allocator: *const AllocationCallbacks,
);

pub type DestroySwapchainKHR = extern "system" fn(
    device: *mut Device,
    swapchain: *mut SwapchainKHR,
    allocator: *const AllocationCallbacks,
);

pub type DeviceWaitIdle = extern "system" fn(device: *mut Device) -> Result;

pub type EndCommandBuffer = extern "system" fn(command_buffer: *mut CommandBuffer) -> Result;

pub type EnumerateInstanceExtensionProperties = extern "system" fn(
    layer_name: *const c_char,
    property_count: *mut u32,
    properties: *mut ExtensionProperties,
) -> Result;

pub type EnumeratePhysicalDevices = extern "system" fn(
    instance: *mut Instance,
    count: *mut u32,
    devices: *mut *mut PhysicalDevice,
) -> Result;

pub type FlushMappedMemoryRanges = extern "system" fn(
    device: *mut Device,
    memory_range_count: u32,
    memory_ranges: *const MappedMemoryRange,
) -> Result;

pub type Free = extern "system" fn(user_data: *mut c_void, memory: *mut c_void);

pub type FreeCommandBuffers = extern "system" fn(
    device: *mut Device,
    command_pool: *mut CommandPool,
    command_buffer_count: u32,
    command_buffers: *const *mut CommandBuffer,
);

pub type FreeDescriptorSets = extern "system" fn(
    device: *mut Device,
    descriptor_pool: *mut DescriptorPool,
    descriptor_set_count: u32,
    descriptor_sets: *const *mut DescriptorSet,
) -> Result;

pub type FreeMemory = extern "system" fn(
    device: *mut Device,
    memory: *mut DeviceMemory,
    allocator: *const AllocationCallbacks,
);

pub type GetBufferMemoryRequirements = extern "system" fn(
    device: *mut Device,
    buffer: *mut Buffer,
    memory_requirements: *mut MemoryRequirements,
);

pub type GetDeviceMemoryCommitment = extern "system" fn(
    device: *mut Device,
    memory: *mut DeviceMemory,
    committed_memory_in_bytes: *mut DeviceSize,
);

pub type GetDeviceProcAddr =
    extern "system" fn(device: *mut Device, name: *const c_char) -> Option<Void>;

pub type GetDeviceQueue = extern "system" fn(
    device: *mut Device,
    queue_family_index: u32,
    queue_index: u32,
    queue: *mut *mut Queue,
);

pub type GetImageMemoryRequirements = extern "system" fn(
    device: *mut Device,
    image: *mut Image,
    memory_requirements: *mut MemoryRequirements,
);

pub type GetInstanceProcAddr =
    extern "system" fn(instance: *mut Instance, name: *const c_char) -> Option<Void>;

pub type GetPhysicalDeviceFeatures =
    extern "system" fn(physical_device: *mut PhysicalDevice, features: *mut PhysicalDeviceFeatures);

pub type GetPhysicalDeviceFormatProperties = extern "system" fn(
    physical_device: *mut PhysicalDevice,
    format: Format,
    format_properties: *mut FormatProperties,
);

pub type GetPhysicalDeviceImageFormatProperties = extern "system" fn(
    physical_device: *mut PhysicalDevice,
    format: Format,
    itype: ImageType,
    tiling: ImageTiling,
    usage: ImageUsageFlags,
    flags: ImageCreateFlags,
    image_format_properties: *mut ImageFormatProperties,
) -> Result;

pub type GetPhysicalDeviceMemoryProperties = extern "system" fn(
    physical_device: *mut PhysicalDevice,
    memory_properties: *mut PhysicalDeviceMemoryProperties,
);

pub type GetPhysicalDeviceProperties = extern "system" fn(
    physical_device: *mut PhysicalDevice,
    properties: *mut PhysicalDeviceProperties,
);

pub type GetPhysicalDeviceQueueFamilyProperties = extern "system" fn(
    physical_device: *mut PhysicalDevice,
    queue_family_property_count: *mut u32,
    queue_family_properties: *mut QueueFamilyProperties,
);

pub type GetPhysicalDeviceSurfaceCapabilitiesKHR = extern "system" fn(
    physical_device: *mut PhysicalDevice,
    surface: *mut SurfaceKHR,
    surface_capabilities: *mut SurfaceCapabilitiesKHR,
) -> Result;

pub type GetPhysicalDeviceSurfaceFormatsKHR = extern "system" fn(
    physical_device: *mut PhysicalDevice,
    surface: *mut SurfaceKHR,
    surface_format_count: *mut u32,
    surface_formats: *mut SurfaceFormatKHR,
) -> Result;

pub type GetPhysicalDeviceSurfaceSupportKHR = extern "system" fn(
    physical_device: *mut PhysicalDevice,
    queue_family_index: u32,
    surface: *mut SurfaceKHR,
    supported: *mut Bool32,
) -> Result;

pub type GetSwapchainImagesKHR = extern "system" fn(
    device: *mut Device,
    swapchain: *mut SwapchainKHR,
    image_count: *mut u32,
    swapchain_images: *mut *mut Image,
) -> Result;

pub type InternalAllocation = extern "system" fn(
    user_data: *mut c_void,
    size: usize,
    allocation: InternalAllocationType,
    scope: SystemAllocationScope,
);

pub type InternalFree = extern "system" fn(
    user_data: *mut c_void,
    size: usize,
    allocation_type: InternalAllocationType,
    scope: SystemAllocationScope,
);

pub type InvalidateMappedMemoryRanges = extern "system" fn(
    device: *mut Device,
    memory_range_count: u32,
    memory_ranges: *const MappedMemoryRange,
) -> Result;

pub type MapMemory = extern "system" fn(
    device: *mut Device,
    memory: *mut DeviceMemory,
    offset: DeviceSize,
    size: DeviceSize,
    flags: MemoryMapFlags,
    data: *mut *mut c_void,
) -> Result;

pub type QueuePresentKHR =
    extern "system" fn(queue: *mut Queue, present_info: *const PresentInfoKHR) -> Result;

pub type QueueSubmit = extern "system" fn(
    queue: *mut Queue,
    submit_count: u32,
    info: *const SubmitInfo,
    fence: *mut Fence,
) -> Result;

pub type QueueWaitIdle = extern "system" fn(queue: *mut Queue) -> Result;

pub type Reallocation = extern "system" fn(
    user_data: *mut c_void,
    original: *mut c_void,
    size: usize,
    alignment: usize,
    scope: SystemAllocationScope,
) -> *mut c_void;

pub type ResetCommandBuffer =
    extern "system" fn(command_buffer: *mut CommandBuffer, flags: CommandBufferResetFlags);

pub type ResetCommandPool = extern "system" fn(
    device: *mut Device,
    command_pool: *mut CommandPool,
    flags: CommandPoolResetFlags,
) -> Result;

pub type ResetDescriptorPool = extern "system" fn(
    device: *mut Device,
    descriptor_pool: *mut DescriptorPool,
    allocation: *const AllocationCallbacks,
) -> Result;

pub type ResetFences =
    extern "system" fn(device: *mut Device, fence_count: u32, fences: *const *mut Fence) -> Result;

pub type UnmapMemory = extern "system" fn(device: *mut Device, memory: *mut DeviceMemory);

pub type UpdateDescriptorSets = extern "system" fn(
    device: *mut Device,
    descriptor_write_count: u32,
    descriptor_writes: *const WriteDescriptorSet,
    descriptor_copy_count: u32,
    descriptor_copies: *const CopyDescriptorSet,
);

pub type Void = extern "system" fn();

pub type WaitForFences = extern "system" fn(
    device: *mut Device,
    fence_count: u32,
    fences: *const *mut Fence,
    wait_all: Bool32,
    timeout: u64,
) -> Result;
