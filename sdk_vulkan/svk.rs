// SPDX-License-Identifier: None
// Copyright (c) 2026 None

impl VkStructureType {
    pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT: Self = Self(1000128004);
}

pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
pub const VK_UUID_SIZE: u32 = 16;
pub const VK_LUID_SIZE: u32 = 8;
pub const VK_MAX_EXTENSION_NAME_SIZE: u32 = 256;
pub const VK_MAX_DESCRIPTION_SIZE: u32 = 256;
pub const VK_MAX_MEMORY_TYPES: u32 = 32;
pub const VK_MAX_MEMORY_HEAPS: u32 = 16;
pub const VK_LOD_CLAMP_NONE: f32 = 1000.0;
pub const VK_REMAINING_MIP_LEVELS: u32 = !0u32;
pub const VK_REMAINING_ARRAY_LAYERS: u32 = !0u32;
pub const VK_REMAINING_3D_SLICES_EXT: u32 = !0u32;
pub const VK_WHOLE_SIZE: u64 = !0u64;
pub const VK_ATTACHMENT_UNUSED: u32 = !0u32;
pub const VK_TRUE: u32 = 1;
pub const VK_FALSE: u32 = 0;
pub const VK_QUEUE_FAMILY_IGNORED: u32 = !0u32;
pub const VK_QUEUE_FAMILY_EXTERNAL: u32 = !1u32;
pub const VK_QUEUE_FAMILY_FOREIGN_EXT: u32 = !2u32;
pub const VK_SUBPASS_EXTERNAL: u32 = !0u32;
pub const VK_MAX_DEVICE_GROUP_SIZE: u32 = 32;
pub const VK_MAX_DRIVER_NAME_SIZE: u32 = 256;
pub const VK_MAX_DRIVER_INFO_SIZE: u32 = 256;
pub const VK_SHADER_UNUSED_KHR: u32 = !0u32;
pub const VK_MAX_GLOBAL_PRIORITY_SIZE: u32 = 16;
pub const VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: u32 = 32;
pub const VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR: u32 = 32;
pub const VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR: u32 = 7;
pub const VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR: u32 = 3;
pub const VK_SHADER_INDEX_UNUSED_AMDX: u32 = !0u32;
pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV: u32 = !0u32;
pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX: u32 = 128;
pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX: u32 = 128;
pub const VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM: u32 = 128;
pub const VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM: u32 = 3;
pub const VK_COMPUTE_OCCUPANCY_PRIORITY_LOW_NV: f32 = 0.25;
pub const VK_COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV: f32 = 0.50;
pub const VK_COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV: f32 = 0.75;
pub const VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM: u32 = 128;
pub const VK_MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM: u32 = 4;

// Provided by VK_VERSION_1_0
pub const fn VK_API_VERSION_VARIANT(pack : u32) -> u32 {
    return pack >> 29;
}

// Provided by VK_VERSION_1_0
pub const fn VK_API_VERSION_MAJOR(pack : u32) -> u32 {
    return (pack >> 22) & 0x7F;
}

// Provided by VK_VERSION_1_0
pub const fn VK_API_VERSION_MINOR(pack : u32) -> u32 {
    return (pack >> 12) & 0xFF;
}

// Provided by VK_VERSION_1_0
pub const fn VK_API_VERSION_PATCH(pack : u32) -> u32 {
    return pack & 0xFFF;
}

// Provided by VK_VERSION_1_0
pub const fn VK_MAKE_API_VERSION(variant : u32, major : u32, minor : u32, patch : u32) -> u32 {
    return variant << 29u32 | major << 22u32 | minor << 12u32 | patch;
}

pub type VkComponentTypeNV = VkComponentTypeKHR;
pub type VkScopeNV = VkScopeKHR;
pub type VkAccelerationStructureTypeNV = VkAccelerationStructureTypeKHR;



pub type StdVideoH264ProfileIdc = std::ffi::c_void;
pub type StdVideoH264LevelIdc = std::ffi::c_void;
pub type StdVideoH264SequenceParameterSet = std::ffi::c_void;
pub type StdVideoH264PictureParameterSet = std::ffi::c_void;
pub type StdVideoDecodeH264PictureInfo = std::ffi::c_void;
pub type StdVideoDecodeH264ReferenceInfo = std::ffi::c_void;

pub type StdVideoH265ProfileIdc = std::ffi::c_void;
pub type StdVideoH265LevelIdc = std::ffi::c_void;
pub type StdVideoH265VideoParameterSet = std::ffi::c_void;
pub type StdVideoH265SequenceParameterSet = std::ffi::c_void;
pub type StdVideoH265PictureParameterSet = std::ffi::c_void;
pub type StdVideoDecodeH265PictureInfo = std::ffi::c_void;
pub type StdVideoDecodeH265ReferenceInfo = std::ffi::c_void;

pub type StdVideoVP9Profile = std::ffi::c_void;
pub type StdVideoVP9Level = std::ffi::c_void;
pub type StdVideoDecodeVP9PictureInfo = std::ffi::c_void;

pub type StdVideoAV1Profile = std::ffi::c_void;
pub type StdVideoAV1Level = std::ffi::c_void;
pub type StdVideoAV1SequenceHeader = std::ffi::c_void;
pub type StdVideoDecodeAV1PictureInfo = std::ffi::c_void;
pub type StdVideoDecodeAV1ReferenceInfo = std::ffi::c_void;

pub type StdVideoEncodeH264ReferenceInfo = std::ffi::c_void;
pub type StdVideoEncodeH264PictureInfo = std::ffi::c_void;
pub type StdVideoEncodeH264SliceHeader = std::ffi::c_void;

pub type StdVideoEncodeH265PictureInfo = std::ffi::c_void;
pub type StdVideoEncodeH265SliceSegmentHeader = std::ffi::c_void;
pub type StdVideoEncodeH265ReferenceInfo = std::ffi::c_void;

pub type StdVideoEncodeAV1DecoderModelInfo = std::ffi::c_void;
pub type StdVideoEncodeAV1OperatingPointInfo = std::ffi::c_void;
pub type StdVideoEncodeAV1ReferenceInfo = std::ffi::c_void;
pub type StdVideoEncodeAV1PictureInfo = std::ffi::c_void;


pub type Display = std::ffi::c_void;
pub type VisualID = std::ffi::c_void;
pub type Window = std::ffi::c_void;
pub type RROutput = std::ffi::c_void;

pub type wl_display = std::ffi::c_void;
pub type wl_surface = std::ffi::c_void;

pub type ubm_device = std::ffi::c_void;
pub type ubm_surface = std::ffi::c_void;

pub type HINSTANCE = std::ffi::c_void;
pub type HWND = std::ffi::c_void;
pub type HMONITOR = std::ffi::c_void;
pub type HANDLE = std::ffi::c_void;
pub type SECURITY_ATTRIBUTES = std::ffi::c_void;
pub type DWORD = std::ffi::c_void;
pub type LPCWSTR = std::ffi::c_void;

pub type xcb_connection_t = std::ffi::c_void;
pub type xcb_visualid_t = std::ffi::c_void;
pub type xcb_window_t = std::ffi::c_void;

pub type IDirectFB = std::ffi::c_void;
pub type IDirectFBSurface = std::ffi::c_void;

pub type zx_handle_t = std::ffi::c_void;

pub type GgpStreamDescriptor = std::ffi::c_void;
pub type GgpFrameToken = std::ffi::c_void;

pub type _screen_context = std::ffi::c_void;
pub type _screen_window = std::ffi::c_void;
pub type _screen_buffer = std::ffi::c_void;

pub type NvSciSyncAttrList = std::ffi::c_void;
pub type NvSciSyncObj = std::ffi::c_void;
pub type NvSciSyncFence = std::ffi::c_void;

pub type NvSciBufAttrList = std::ffi::c_void;
pub type NvSciBufObj = std::ffi::c_void;





pub type ANativeWindow = *mut std::ffi::c_void; //

pub type AHardwareBuffer = *mut std::ffi::c_void; //

pub type CAMetalLayer = *mut std::ffi::c_void; //

pub type MTLDevice_id = *mut std::ffi::c_void; //

pub type MTLCommandQueue_id = *mut std::ffi::c_void; //

pub type MTLBuffer_id = *mut std::ffi::c_void; //

pub type MTLTexture_id = *mut std::ffi::c_void; //

pub type MTLSharedEvent_id = *mut std::ffi::c_void; //

pub type IOSurfaceRef = *mut std::ffi::c_void; //

pub type VkSampleMask = u32; //

pub type VkBool32 = u32; //

pub type VkFlags = u32; //

pub type VkFlags64 = u64; //

pub type VkDeviceSize = u64; //

pub type VkDeviceAddress = u64; //

pub type OHNativeWindow = *mut std::ffi::c_void; //

pub type OHBufferHandle = *mut std::ffi::c_void; //

pub type OH_NativeBuffer = *mut std::ffi::c_void; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkFramebufferCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkFramebufferCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkQueryPoolCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkQueryPoolCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkRenderPassCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkRenderPassCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkSamplerCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkSamplerCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineLayoutCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineLayoutCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineCacheCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineCacheCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineDepthStencilStateCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineDepthStencilStateCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineDynamicStateCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineDynamicStateCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineColorBlendStateCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineColorBlendStateCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineMultisampleStateCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineMultisampleStateCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineRasterizationStateCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineRasterizationStateCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineViewportStateCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineViewportStateCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineTessellationStateCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineTessellationStateCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineInputAssemblyStateCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineInputAssemblyStateCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineVertexInputStateCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineVertexInputStateCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineShaderStageCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineShaderStageCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDescriptorSetLayoutCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkDescriptorSetLayoutCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkBufferViewCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkBufferViewCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkInstanceCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkInstanceCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDeviceCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkDeviceCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDeviceQueueCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkDeviceQueueCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkQueueFlags(pub VkFlags); //
impl std::ops::BitAnd for VkQueueFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkMemoryPropertyFlags(pub VkFlags); //
impl std::ops::BitAnd for VkMemoryPropertyFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkMemoryHeapFlags(pub VkFlags); //
impl std::ops::BitAnd for VkMemoryHeapFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkAccessFlags(pub VkFlags); //
impl std::ops::BitAnd for VkAccessFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkBufferUsageFlags(pub VkFlags); //
impl std::ops::BitAnd for VkBufferUsageFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkBufferCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkBufferCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkShaderStageFlags(pub VkFlags); //
impl std::ops::BitAnd for VkShaderStageFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkImageUsageFlags(pub VkFlags); //
impl std::ops::BitAnd for VkImageUsageFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkImageCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkImageCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkImageViewCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkImageViewCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkColorComponentFlags(pub VkFlags); //
impl std::ops::BitAnd for VkColorComponentFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkFenceCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkFenceCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkSemaphoreCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkSemaphoreCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkFormatFeatureFlags(pub VkFlags); //
impl std::ops::BitAnd for VkFormatFeatureFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkQueryControlFlags(pub VkFlags); //
impl std::ops::BitAnd for VkQueryControlFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkQueryResultFlags(pub VkFlags); //
impl std::ops::BitAnd for VkQueryResultFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkShaderModuleCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkShaderModuleCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkEventCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkEventCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkCommandPoolCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkCommandPoolCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkCommandPoolResetFlags(pub VkFlags); //
impl std::ops::BitAnd for VkCommandPoolResetFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkCommandBufferResetFlags(pub VkFlags); //
impl std::ops::BitAnd for VkCommandBufferResetFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkCommandBufferUsageFlags(pub VkFlags); //
impl std::ops::BitAnd for VkCommandBufferUsageFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkQueryPipelineStatisticFlags(pub VkFlags); //
impl std::ops::BitAnd for VkQueryPipelineStatisticFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkMemoryMapFlags(pub VkFlags); //
impl std::ops::BitAnd for VkMemoryMapFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkMemoryUnmapFlags(pub VkFlags); //
impl std::ops::BitAnd for VkMemoryUnmapFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkMemoryUnmapFlagsKHR = VkMemoryUnmapFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkImageAspectFlags(pub VkFlags); //
impl std::ops::BitAnd for VkImageAspectFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkSparseMemoryBindFlags(pub VkFlags); //
impl std::ops::BitAnd for VkSparseMemoryBindFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkSparseImageFormatFlags(pub VkFlags); //
impl std::ops::BitAnd for VkSparseImageFormatFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkSubpassDescriptionFlags(pub VkFlags); //
impl std::ops::BitAnd for VkSubpassDescriptionFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineStageFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineStageFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkSampleCountFlags(pub VkFlags); //
impl std::ops::BitAnd for VkSampleCountFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkAttachmentDescriptionFlags(pub VkFlags); //
impl std::ops::BitAnd for VkAttachmentDescriptionFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkStencilFaceFlags(pub VkFlags); //
impl std::ops::BitAnd for VkStencilFaceFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkCullModeFlags(pub VkFlags); //
impl std::ops::BitAnd for VkCullModeFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDescriptorPoolCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkDescriptorPoolCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDescriptorPoolResetFlags(pub VkFlags); //
impl std::ops::BitAnd for VkDescriptorPoolResetFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDependencyFlags(pub VkFlags); //
impl std::ops::BitAnd for VkDependencyFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkSubgroupFeatureFlags(pub VkFlags); //
impl std::ops::BitAnd for VkSubgroupFeatureFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkIndirectCommandsLayoutUsageFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkIndirectCommandsLayoutUsageFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkIndirectStateFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkIndirectStateFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkGeometryFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkGeometryFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkGeometryFlagsNV = VkGeometryFlagsKHR; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkGeometryInstanceFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkGeometryInstanceFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkGeometryInstanceFlagsNV = VkGeometryInstanceFlagsKHR; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkClusterAccelerationStructureGeometryFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkClusterAccelerationStructureGeometryFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkClusterAccelerationStructureClusterFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkClusterAccelerationStructureClusterFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkClusterAccelerationStructureAddressResolutionFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkClusterAccelerationStructureAddressResolutionFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkBuildAccelerationStructureFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkBuildAccelerationStructureFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkBuildAccelerationStructureFlagsNV = VkBuildAccelerationStructureFlagsKHR; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkPrivateDataSlotCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPrivateDataSlotCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkPrivateDataSlotCreateFlagsEXT = VkPrivateDataSlotCreateFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureCreateFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkAccelerationStructureCreateFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDescriptorUpdateTemplateCreateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkDescriptorUpdateTemplateCreateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkDescriptorUpdateTemplateCreateFlagsKHR = VkDescriptorUpdateTemplateCreateFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineCreationFeedbackFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineCreationFeedbackFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkPipelineCreationFeedbackFlagsEXT = VkPipelineCreationFeedbackFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkPerformanceCounterDescriptionFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkPerformanceCounterDescriptionFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkAcquireProfilingLockFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkAcquireProfilingLockFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkSemaphoreWaitFlags(pub VkFlags); //
impl std::ops::BitAnd for VkSemaphoreWaitFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkSemaphoreWaitFlagsKHR = VkSemaphoreWaitFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineCompilerControlFlagsAMD(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineCompilerControlFlagsAMD {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkShaderCorePropertiesFlagsAMD(pub VkFlags); //
impl std::ops::BitAnd for VkShaderCorePropertiesFlagsAMD {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDeviceDiagnosticsConfigFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkDeviceDiagnosticsConfigFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkRefreshObjectFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkRefreshObjectFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkAccessFlags2(pub VkFlags64); //
impl std::ops::BitAnd for VkAccessFlags2 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkAccessFlags2KHR = VkAccessFlags2; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineStageFlags2(pub VkFlags64); //
impl std::ops::BitAnd for VkPipelineStageFlags2 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkPipelineStageFlags2KHR = VkPipelineStageFlags2; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureMotionInfoFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkAccelerationStructureMotionInfoFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkAccelerationStructureMotionInstanceFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkAccelerationStructureMotionInstanceFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkFormatFeatureFlags2(pub VkFlags64); //
impl std::ops::BitAnd for VkFormatFeatureFlags2 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkFormatFeatureFlags2KHR = VkFormatFeatureFlags2; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkFormatFeatureFlags4KHR(pub VkFlags64); //
impl std::ops::BitAnd for VkFormatFeatureFlags4KHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkRenderingFlags(pub VkFlags); //
impl std::ops::BitAnd for VkRenderingFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkMemoryDecompressionMethodFlagsEXT(pub VkFlags64); //
impl std::ops::BitAnd for VkMemoryDecompressionMethodFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkMemoryDecompressionMethodFlagsNV = VkMemoryDecompressionMethodFlagsEXT; //

pub type VkRenderingFlagsKHR = VkRenderingFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkDeviceFaultFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkDeviceFaultFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkBuildMicromapFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkBuildMicromapFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkMicromapCreateFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkMicromapCreateFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkIndirectCommandsLayoutUsageFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkIndirectCommandsLayoutUsageFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkIndirectCommandsInputModeFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkIndirectCommandsInputModeFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDirectDriverLoadingFlagsLUNARG(pub VkFlags); //
impl std::ops::BitAnd for VkDirectDriverLoadingFlagsLUNARG {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineCreateFlags2(pub VkFlags64); //
impl std::ops::BitAnd for VkPipelineCreateFlags2 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkPipelineCreateFlags2KHR = VkPipelineCreateFlags2; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkBufferUsageFlags2(pub VkFlags64); //
impl std::ops::BitAnd for VkBufferUsageFlags2 {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkBufferUsageFlags2KHR = VkBufferUsageFlags2; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkImageUsageFlags2KHR(pub VkFlags64); //
impl std::ops::BitAnd for VkImageUsageFlags2KHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkImageCreateFlags2KHR(pub VkFlags64); //
impl std::ops::BitAnd for VkImageCreateFlags2KHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkAddressCopyFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkAddressCopyFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkTensorCreateFlagsARM(pub VkFlags64); //
impl std::ops::BitAnd for VkTensorCreateFlagsARM {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkTensorUsageFlagsARM(pub VkFlags64); //
impl std::ops::BitAnd for VkTensorUsageFlagsARM {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkTensorViewCreateFlagsARM(pub VkFlags64); //
impl std::ops::BitAnd for VkTensorViewCreateFlagsARM {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDataGraphPipelineSessionCreateFlagsARM(pub VkFlags64); //
impl std::ops::BitAnd for VkDataGraphPipelineSessionCreateFlagsARM {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDataGraphPipelineDispatchFlagsARM(pub VkFlags64); //
impl std::ops::BitAnd for VkDataGraphPipelineDispatchFlagsARM {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeRgbModelConversionFlagsVALVE(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeRgbModelConversionFlagsVALVE {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeRgbRangeCompressionFlagsVALVE(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeRgbRangeCompressionFlagsVALVE {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeRgbChromaOffsetFlagsVALVE(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeRgbChromaOffsetFlagsVALVE {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkSpirvResourceTypeFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkSpirvResourceTypeFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkGpaSqShaderStageFlagsAMD(pub VkFlags); //
impl std::ops::BitAnd for VkGpaSqShaderStageFlagsAMD {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkGpaPerfBlockPropertiesFlagsAMD(pub VkFlags); //
impl std::ops::BitAnd for VkGpaPerfBlockPropertiesFlagsAMD {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceGpaPropertiesFlagsAMD(pub VkFlags); //
impl std::ops::BitAnd for VkPhysicalDeviceGpaPropertiesFlagsAMD {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkAddressCommandFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkAddressCommandFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkCompositeAlphaFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkCompositeAlphaFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDisplayPlaneAlphaFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkDisplayPlaneAlphaFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkSurfaceTransformFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkSurfaceTransformFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkSwapchainCreateFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkSwapchainCreateFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDisplayModeCreateFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkDisplayModeCreateFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDisplaySurfaceCreateFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkDisplaySurfaceCreateFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkAndroidSurfaceCreateFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkAndroidSurfaceCreateFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkViSurfaceCreateFlagsNN(pub VkFlags); //
impl std::ops::BitAnd for VkViSurfaceCreateFlagsNN {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkWaylandSurfaceCreateFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkWaylandSurfaceCreateFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkUbmSurfaceCreateFlagsSEC(pub VkFlags); //
impl std::ops::BitAnd for VkUbmSurfaceCreateFlagsSEC {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkWin32SurfaceCreateFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkWin32SurfaceCreateFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkXlibSurfaceCreateFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkXlibSurfaceCreateFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkXcbSurfaceCreateFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkXcbSurfaceCreateFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDirectFBSurfaceCreateFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkDirectFBSurfaceCreateFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkIOSSurfaceCreateFlagsMVK(pub VkFlags); //
impl std::ops::BitAnd for VkIOSSurfaceCreateFlagsMVK {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkMacOSSurfaceCreateFlagsMVK(pub VkFlags); //
impl std::ops::BitAnd for VkMacOSSurfaceCreateFlagsMVK {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkMetalSurfaceCreateFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkMetalSurfaceCreateFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkImagePipeSurfaceCreateFlagsFUCHSIA(pub VkFlags); //
impl std::ops::BitAnd for VkImagePipeSurfaceCreateFlagsFUCHSIA {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkStreamDescriptorSurfaceCreateFlagsGGP(pub VkFlags); //
impl std::ops::BitAnd for VkStreamDescriptorSurfaceCreateFlagsGGP {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkHeadlessSurfaceCreateFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkHeadlessSurfaceCreateFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkScreenSurfaceCreateFlagsQNX(pub VkFlags); //
impl std::ops::BitAnd for VkScreenSurfaceCreateFlagsQNX {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPeerMemoryFeatureFlags(pub VkFlags); //
impl std::ops::BitAnd for VkPeerMemoryFeatureFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkPeerMemoryFeatureFlagsKHR = VkPeerMemoryFeatureFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkMemoryAllocateFlags(pub VkFlags); //
impl std::ops::BitAnd for VkMemoryAllocateFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkMemoryAllocateFlagsKHR = VkMemoryAllocateFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkDeviceGroupPresentModeFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkDeviceGroupPresentModeFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDebugReportFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkDebugReportFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkCommandPoolTrimFlags(pub VkFlags); //
impl std::ops::BitAnd for VkCommandPoolTrimFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkCommandPoolTrimFlagsKHR = VkCommandPoolTrimFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkExternalMemoryHandleTypeFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkExternalMemoryHandleTypeFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkClusterAccelerationStructureIndexFormatFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkClusterAccelerationStructureIndexFormatFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkExternalMemoryFeatureFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkExternalMemoryFeatureFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkExternalMemoryHandleTypeFlags(pub VkFlags); //
impl std::ops::BitAnd for VkExternalMemoryHandleTypeFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkExternalMemoryHandleTypeFlagsKHR = VkExternalMemoryHandleTypeFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkExternalMemoryFeatureFlags(pub VkFlags); //
impl std::ops::BitAnd for VkExternalMemoryFeatureFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkExternalMemoryFeatureFlagsKHR = VkExternalMemoryFeatureFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkExternalSemaphoreHandleTypeFlags(pub VkFlags); //
impl std::ops::BitAnd for VkExternalSemaphoreHandleTypeFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkExternalSemaphoreHandleTypeFlagsKHR = VkExternalSemaphoreHandleTypeFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkExternalSemaphoreFeatureFlags(pub VkFlags); //
impl std::ops::BitAnd for VkExternalSemaphoreFeatureFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkExternalSemaphoreFeatureFlagsKHR = VkExternalSemaphoreFeatureFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkSemaphoreImportFlags(pub VkFlags); //
impl std::ops::BitAnd for VkSemaphoreImportFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkSemaphoreImportFlagsKHR = VkSemaphoreImportFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkExternalFenceHandleTypeFlags(pub VkFlags); //
impl std::ops::BitAnd for VkExternalFenceHandleTypeFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkExternalFenceHandleTypeFlagsKHR = VkExternalFenceHandleTypeFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkExternalFenceFeatureFlags(pub VkFlags); //
impl std::ops::BitAnd for VkExternalFenceFeatureFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkExternalFenceFeatureFlagsKHR = VkExternalFenceFeatureFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkFenceImportFlags(pub VkFlags); //
impl std::ops::BitAnd for VkFenceImportFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkFenceImportFlagsKHR = VkFenceImportFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkSurfaceCounterFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkSurfaceCounterFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineViewportSwizzleStateCreateFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineViewportSwizzleStateCreateFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineDiscardRectangleStateCreateFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineDiscardRectangleStateCreateFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineCoverageToColorStateCreateFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineCoverageToColorStateCreateFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineCoverageModulationStateCreateFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineCoverageModulationStateCreateFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineCoverageReductionStateCreateFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineCoverageReductionStateCreateFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkValidationCacheCreateFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkValidationCacheCreateFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDebugUtilsMessageSeverityFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkDebugUtilsMessageSeverityFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDebugUtilsMessageTypeFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkDebugUtilsMessageTypeFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDebugUtilsMessengerCreateFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkDebugUtilsMessengerCreateFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDebugUtilsMessengerCallbackDataFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkDebugUtilsMessengerCallbackDataFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDeviceMemoryReportFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkDeviceMemoryReportFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineRasterizationConservativeStateCreateFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineRasterizationConservativeStateCreateFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDescriptorBindingFlags(pub VkFlags); //
impl std::ops::BitAnd for VkDescriptorBindingFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkDescriptorBindingFlagsEXT = VkDescriptorBindingFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkConditionalRenderingFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkConditionalRenderingFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkResolveModeFlags(pub VkFlags); //
impl std::ops::BitAnd for VkResolveModeFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkResolveModeFlagsKHR = VkResolveModeFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineRasterizationStateStreamCreateFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineRasterizationStateStreamCreateFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPipelineRasterizationDepthClipStateCreateFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkPipelineRasterizationDepthClipStateCreateFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkSwapchainImageUsageFlagsANDROID(pub VkFlags); //
impl std::ops::BitAnd for VkSwapchainImageUsageFlagsANDROID {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkToolPurposeFlags(pub VkFlags); //
impl std::ops::BitAnd for VkToolPurposeFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkToolPurposeFlagsEXT = VkToolPurposeFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkSubmitFlags(pub VkFlags); //
impl std::ops::BitAnd for VkSubmitFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkSubmitFlagsKHR = VkSubmitFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkImageFormatConstraintsFlagsFUCHSIA(pub VkFlags); //
impl std::ops::BitAnd for VkImageFormatConstraintsFlagsFUCHSIA {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkHostImageCopyFlags(pub VkFlags); //
impl std::ops::BitAnd for VkHostImageCopyFlags {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkHostImageCopyFlagsEXT = VkHostImageCopyFlags; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkPartitionedAccelerationStructureInstanceFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkPartitionedAccelerationStructureInstanceFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkImageConstraintsInfoFlagsFUCHSIA(pub VkFlags); //
impl std::ops::BitAnd for VkImageConstraintsInfoFlagsFUCHSIA {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkGraphicsPipelineLibraryFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkGraphicsPipelineLibraryFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkImageCompressionFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkImageCompressionFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkImageCompressionFixedRateFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkImageCompressionFixedRateFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkExportMetalObjectTypeFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkExportMetalObjectTypeFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkRenderingAttachmentFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkRenderingAttachmentFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkResolveImageFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkResolveImageFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDeviceAddressBindingFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkDeviceAddressBindingFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkOpticalFlowGridSizeFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkOpticalFlowGridSizeFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkOpticalFlowUsageFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkOpticalFlowUsageFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkOpticalFlowSessionCreateFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkOpticalFlowSessionCreateFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkOpticalFlowExecuteFlagsNV(pub VkFlags); //
impl std::ops::BitAnd for VkOpticalFlowExecuteFlagsNV {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkFrameBoundaryFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkFrameBoundaryFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPresentScalingFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkPresentScalingFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkPresentScalingFlagsEXT = VkPresentScalingFlagsKHR; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkPresentGravityFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkPresentGravityFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

pub type VkPresentGravityFlagsEXT = VkPresentGravityFlagsKHR; //

#[derive(Copy, Clone, PartialEq)]
pub struct VkShaderCreateFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkShaderCreateFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkTileShadingRenderPassFlagsQCOM(pub VkFlags); //
impl std::ops::BitAnd for VkTileShadingRenderPassFlagsQCOM {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPhysicalDeviceSchedulingControlsFlagsARM(pub VkFlags64); //
impl std::ops::BitAnd for VkPhysicalDeviceSchedulingControlsFlagsARM {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkSurfaceCreateFlagsOHOS(pub VkFlags); //
impl std::ops::BitAnd for VkSurfaceCreateFlagsOHOS {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPresentStageFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkPresentStageFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPastPresentationTimingFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkPastPresentationTimingFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPresentTimingInfoFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkPresentTimingInfoFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkSwapchainImageUsageFlagsOHOS(pub VkFlags); //
impl std::ops::BitAnd for VkSwapchainImageUsageFlagsOHOS {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkPerformanceCounterDescriptionFlagsARM(pub VkFlags); //
impl std::ops::BitAnd for VkPerformanceCounterDescriptionFlagsARM {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkShaderInstrumentationValuesFlagsARM(pub VkFlags); //
impl std::ops::BitAnd for VkShaderInstrumentationValuesFlagsARM {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDataGraphTOSAQualityFlagsARM(pub VkFlags); //
impl std::ops::BitAnd for VkDataGraphTOSAQualityFlagsARM {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDataGraphOpticalFlowGridSizeFlagsARM(pub VkFlags); //
impl std::ops::BitAnd for VkDataGraphOpticalFlowGridSizeFlagsARM {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDataGraphOpticalFlowImageUsageFlagsARM(pub VkFlags); //
impl std::ops::BitAnd for VkDataGraphOpticalFlowImageUsageFlagsARM {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDataGraphOpticalFlowCreateFlagsARM(pub VkFlags); //
impl std::ops::BitAnd for VkDataGraphOpticalFlowCreateFlagsARM {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkDataGraphOpticalFlowExecuteFlagsARM(pub VkFlags); //
impl std::ops::BitAnd for VkDataGraphOpticalFlowExecuteFlagsARM {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoCodecOperationFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoCodecOperationFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoCapabilityFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoCapabilityFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoSessionCreateFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoSessionCreateFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoSessionParametersCreateFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoSessionParametersCreateFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoBeginCodingFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoBeginCodingFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEndCodingFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEndCodingFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoCodingControlFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoCodingControlFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoDecodeUsageFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoDecodeUsageFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoDecodeCapabilityFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoDecodeCapabilityFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoDecodeFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoDecodeFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoDecodeH264PictureLayoutFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoDecodeH264PictureLayoutFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeUsageFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeUsageFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeContentFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeContentFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeCapabilityFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeCapabilityFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeFeedbackFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeFeedbackFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodePerPartitionFeedbackFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodePerPartitionFeedbackFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeRateControlFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeRateControlFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeRateControlModeFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeRateControlModeFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeIntraRefreshModeFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeIntraRefreshModeFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoChromaSubsamplingFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoChromaSubsamplingFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoComponentBitDepthFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoComponentBitDepthFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeH264CapabilityFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeH264CapabilityFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeH264StdFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeH264StdFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeH264RateControlFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeH264RateControlFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeH265CapabilityFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeH265CapabilityFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeH265StdFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeH265StdFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeH265RateControlFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeH265RateControlFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeH265CtbSizeFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeH265CtbSizeFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeH265TransformBlockSizeFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeH265TransformBlockSizeFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeAV1CapabilityFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeAV1CapabilityFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeAV1StdFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeAV1StdFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeAV1RateControlFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeAV1RateControlFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkVideoEncodeAV1SuperblockSizeFlagsKHR(pub VkFlags); //
impl std::ops::BitAnd for VkVideoEncodeAV1SuperblockSizeFlagsKHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkAccessFlags3KHR(pub VkFlags64); //
impl std::ops::BitAnd for VkAccessFlags3KHR {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[derive(Copy, Clone, PartialEq)]
pub struct VkCooperativeMatrixFlagsEXT(pub VkFlags); //
impl std::ops::BitAnd for VkCooperativeMatrixFlagsEXT {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        return Self(self.0 & rhs.0);
    }
}

#[repr(C)]
pub struct VkInstance_T {
    private: [u8; 0]
}
pub type VkInstance = *mut VkInstance_T;
#[repr(C)]
pub struct VkPhysicalDevice_T {
    private: [u8; 0]
}
pub type VkPhysicalDevice = *mut VkPhysicalDevice_T;
#[repr(C)]
pub struct VkDevice_T {
    private: [u8; 0]
}
pub type VkDevice = VkDevice_T;
#[repr(C)]
pub struct VkQueue_T {
    private: [u8; 0]
}
pub type VkQueue = VkQueue_T;
#[repr(C)]
pub struct VkCommandBuffer_T {
    private: [u8; 0]
}
pub type VkCommandBuffer = VkCommandBuffer_T;
#[repr(C)]
pub struct VkDeviceMemory_T {
    private: [u8; 0]
}
pub type VkDeviceMemory = VkDeviceMemory_T;
#[repr(C)]
pub struct VkCommandPool_T {
    private: [u8; 0]
}
pub type VkCommandPool = VkCommandPool_T;
#[repr(C)]
pub struct VkBuffer_T {
    private: [u8; 0]
}
pub type VkBuffer = VkBuffer_T;
#[repr(C)]
pub struct VkBufferView_T {
    private: [u8; 0]
}
pub type VkBufferView = VkBufferView_T;
#[repr(C)]
pub struct VkImage_T {
    private: [u8; 0]
}
pub type VkImage = VkImage_T;
#[repr(C)]
pub struct VkImageView_T {
    private: [u8; 0]
}
pub type VkImageView = VkImageView_T;
#[repr(C)]
pub struct VkShaderModule_T {
    private: [u8; 0]
}
pub type VkShaderModule = VkShaderModule_T;
#[repr(C)]
pub struct VkPipeline_T {
    private: [u8; 0]
}
pub type VkPipeline = VkPipeline_T;
#[repr(C)]
pub struct VkPipelineLayout_T {
    private: [u8; 0]
}
pub type VkPipelineLayout = VkPipelineLayout_T;
#[repr(C)]
pub struct VkSampler_T {
    private: [u8; 0]
}
pub type VkSampler = VkSampler_T;
#[repr(C)]
pub struct VkDescriptorSet_T {
    private: [u8; 0]
}
pub type VkDescriptorSet = VkDescriptorSet_T;
#[repr(C)]
pub struct VkDescriptorSetLayout_T {
    private: [u8; 0]
}
pub type VkDescriptorSetLayout = VkDescriptorSetLayout_T;
#[repr(C)]
pub struct VkDescriptorPool_T {
    private: [u8; 0]
}
pub type VkDescriptorPool = VkDescriptorPool_T;
#[repr(C)]
pub struct VkFence_T {
    private: [u8; 0]
}
pub type VkFence = VkFence_T;
#[repr(C)]
pub struct VkSemaphore_T {
    private: [u8; 0]
}
pub type VkSemaphore = VkSemaphore_T;
#[repr(C)]
pub struct VkEvent_T {
    private: [u8; 0]
}
pub type VkEvent = VkEvent_T;
#[repr(C)]
pub struct VkQueryPool_T {
    private: [u8; 0]
}
pub type VkQueryPool = VkQueryPool_T;
#[repr(C)]
pub struct VkFramebuffer_T {
    private: [u8; 0]
}
pub type VkFramebuffer = VkFramebuffer_T;
#[repr(C)]
pub struct VkRenderPass_T {
    private: [u8; 0]
}
pub type VkRenderPass = VkRenderPass_T;
#[repr(C)]
pub struct VkPipelineCache_T {
    private: [u8; 0]
}
pub type VkPipelineCache = VkPipelineCache_T;
#[repr(C)]
pub struct VkPipelineBinaryKHR_T {
    private: [u8; 0]
}
pub type VkPipelineBinaryKHR = VkPipelineBinaryKHR_T;
#[repr(C)]
pub struct VkIndirectCommandsLayoutNV_T {
    private: [u8; 0]
}
pub type VkIndirectCommandsLayoutNV = VkIndirectCommandsLayoutNV_T;
#[repr(C)]
pub struct VkIndirectCommandsLayoutEXT_T {
    private: [u8; 0]
}
pub type VkIndirectCommandsLayoutEXT = VkIndirectCommandsLayoutEXT_T;
#[repr(C)]
pub struct VkIndirectExecutionSetEXT_T {
    private: [u8; 0]
}
pub type VkIndirectExecutionSetEXT = VkIndirectExecutionSetEXT_T;
#[repr(C)]
pub struct VkDescriptorUpdateTemplate_T {
    private: [u8; 0]
}
pub type VkDescriptorUpdateTemplate = VkDescriptorUpdateTemplate_T;
#[repr(C)]
pub struct VkSamplerYcbcrConversion_T {
    private: [u8; 0]
}
pub type VkSamplerYcbcrConversion = VkSamplerYcbcrConversion_T;
#[repr(C)]
pub struct VkValidationCacheEXT_T {
    private: [u8; 0]
}
pub type VkValidationCacheEXT = VkValidationCacheEXT_T;
#[repr(C)]
pub struct VkAccelerationStructureKHR_T {
    private: [u8; 0]
}
pub type VkAccelerationStructureKHR = VkAccelerationStructureKHR_T;
#[repr(C)]
pub struct VkAccelerationStructureNV_T {
    private: [u8; 0]
}
pub type VkAccelerationStructureNV = VkAccelerationStructureNV_T;
#[repr(C)]
pub struct VkPerformanceConfigurationINTEL_T {
    private: [u8; 0]
}
pub type VkPerformanceConfigurationINTEL = VkPerformanceConfigurationINTEL_T;
#[repr(C)]
pub struct VkBufferCollectionFUCHSIA_T {
    private: [u8; 0]
}
pub type VkBufferCollectionFUCHSIA = VkBufferCollectionFUCHSIA_T;
#[repr(C)]
pub struct VkDeferredOperationKHR_T {
    private: [u8; 0]
}
pub type VkDeferredOperationKHR = VkDeferredOperationKHR_T;
#[repr(C)]
pub struct VkPrivateDataSlot_T {
    private: [u8; 0]
}
pub type VkPrivateDataSlot = VkPrivateDataSlot_T;
#[repr(C)]
pub struct VkCuModuleNVX_T {
    private: [u8; 0]
}
pub type VkCuModuleNVX = VkCuModuleNVX_T;
#[repr(C)]
pub struct VkCuFunctionNVX_T {
    private: [u8; 0]
}
pub type VkCuFunctionNVX = VkCuFunctionNVX_T;
#[repr(C)]
pub struct VkOpticalFlowSessionNV_T {
    private: [u8; 0]
}
pub type VkOpticalFlowSessionNV = VkOpticalFlowSessionNV_T;
#[repr(C)]
pub struct VkMicromapEXT_T {
    private: [u8; 0]
}
pub type VkMicromapEXT = VkMicromapEXT_T;
#[repr(C)]
pub struct VkShaderEXT_T {
    private: [u8; 0]
}
pub type VkShaderEXT = VkShaderEXT_T;
#[repr(C)]
pub struct VkTensorARM_T {
    private: [u8; 0]
}
pub type VkTensorARM = VkTensorARM_T;
#[repr(C)]
pub struct VkTensorViewARM_T {
    private: [u8; 0]
}
pub type VkTensorViewARM = VkTensorViewARM_T;
#[repr(C)]
pub struct VkDataGraphPipelineSessionARM_T {
    private: [u8; 0]
}
pub type VkDataGraphPipelineSessionARM = VkDataGraphPipelineSessionARM_T;
#[repr(C)]
pub struct VkShaderInstrumentationARM_T {
    private: [u8; 0]
}
pub type VkShaderInstrumentationARM = VkShaderInstrumentationARM_T;
#[repr(C)]
pub struct VkGpaSessionAMD_T {
    private: [u8; 0]
}
pub type VkGpaSessionAMD = VkGpaSessionAMD_T;
#[repr(C)]
pub struct VkDisplayKHR_T {
    private: [u8; 0]
}
pub type VkDisplayKHR = VkDisplayKHR_T;
#[repr(C)]
pub struct VkDisplayModeKHR_T {
    private: [u8; 0]
}
pub type VkDisplayModeKHR = VkDisplayModeKHR_T;
#[repr(C)]
pub struct VkSurfaceKHR_T {
    private: [u8; 0]
}
pub type VkSurfaceKHR = VkSurfaceKHR_T;
#[repr(C)]
pub struct VkSwapchainKHR_T {
    private: [u8; 0]
}
pub type VkSwapchainKHR = VkSwapchainKHR_T;
#[repr(C)]
pub struct VkDebugReportCallbackEXT_T {
    private: [u8; 0]
}
pub type VkDebugReportCallbackEXT = VkDebugReportCallbackEXT_T;
#[repr(C)]
pub struct VkDebugUtilsMessengerEXT_T {
    private: [u8; 0]
}
pub type VkDebugUtilsMessengerEXT = VkDebugUtilsMessengerEXT_T;
#[repr(C)]
pub struct VkVideoSessionKHR_T {
    private: [u8; 0]
}
pub type VkVideoSessionKHR = VkVideoSessionKHR_T;
#[repr(C)]
pub struct VkVideoSessionParametersKHR_T {
    private: [u8; 0]
}
pub type VkVideoSessionParametersKHR = VkVideoSessionParametersKHR_T;
#[repr(C)]
pub struct VkSemaphoreSciSyncPoolNV_T {
    private: [u8; 0]
}
pub type VkSemaphoreSciSyncPoolNV = VkSemaphoreSciSyncPoolNV_T;
#[repr(C)]
pub struct VkBaseOutStructure {
    pub sType:  VkStructureType,
    pub pNext: *mut  VkBaseOutStructure,
}

#[repr(C)]
pub struct VkBaseInStructure {
    pub sType:  VkStructureType,
    pub pNext: *const  VkBaseInStructure,
}

#[repr(C)]
pub struct VkOffset2D {
    pub x:  i32,
    pub y:  i32,
}

#[repr(C)]
pub struct VkOffset3D {
    pub x:  i32,
    pub y:  i32,
    pub z:  i32,
}

#[repr(C)]
pub struct VkExtent2D {
    pub width:  u32,
    pub height:  u32,
}

#[repr(C)]
pub struct VkExtent3D {
    pub width:  u32,
    pub height:  u32,
    pub depth:  u32,
}

#[repr(C)]
pub struct VkViewport {
    pub x:  f32,
    pub y:  f32,
    pub width:  f32,
    pub height:  f32,
    pub minDepth:  f32,
    pub maxDepth:  f32,
}

#[repr(C)]
pub struct VkRect2D {
    pub offset:  VkOffset2D,
    pub extent:  VkExtent2D,
}

#[repr(C)]
pub struct VkClearRect {
    pub rect:  VkRect2D,
    pub baseArrayLayer:  u32,
    pub layerCount:  u32,
}

#[repr(C)]
pub struct VkComponentMapping {
    pub r:  VkComponentSwizzle,
    pub g:  VkComponentSwizzle,
    pub b:  VkComponentSwizzle,
    pub a:  VkComponentSwizzle,
}

#[repr(C)]
pub struct VkPhysicalDeviceProperties {
    pub apiVersion:  u32,
    pub driverVersion:  u32,
    pub vendorID:  u32,
    pub deviceID:  u32,
    pub deviceType:  VkPhysicalDeviceType,
    pub deviceName:  [i8; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
    pub pipelineCacheUUID:  [u8; VK_UUID_SIZE as usize],
    pub limits:  VkPhysicalDeviceLimits,
    pub sparseProperties:  VkPhysicalDeviceSparseProperties,
}

#[repr(C)]
pub struct VkExtensionProperties {
    pub extensionName:  [i8; VK_MAX_EXTENSION_NAME_SIZE as usize],
    pub specVersion:  u32,
}

#[repr(C)]
pub struct VkLayerProperties {
    pub layerName:  [i8; VK_MAX_EXTENSION_NAME_SIZE as usize],
    pub specVersion:  u32,
    pub implementationVersion:  u32,
    pub description:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
}

#[repr(C)]
pub struct VkApplicationInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pApplicationName: *const  i8,
    pub applicationVersion:  u32,
    pub pEngineName: *const  i8,
    pub engineVersion:  u32,
    pub apiVersion:  u32,
}

#[repr(C)]
pub struct VkAllocationCallbacks {
    pub pUserData: *mut  std::ffi::c_void,
    pub pfnAllocation:  crate::svk_commands::PFN_vkAllocationFunction,
    pub pfnReallocation:  crate::svk_commands::PFN_vkReallocationFunction,
    pub pfnFree:  crate::svk_commands::PFN_vkFreeFunction,
    pub pfnInternalAllocation:  crate::svk_commands::PFN_vkInternalAllocationNotification,
    pub pfnInternalFree:  crate::svk_commands::PFN_vkInternalFreeNotification,
}

#[repr(C)]
pub struct VkDeviceQueueCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkDeviceQueueCreateFlags,
    pub queueFamilyIndex:  u32,
    pub queueCount:  u32,
    pub pQueuePriorities: *const  f32,
}

#[repr(C)]
pub struct VkDeviceCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkDeviceCreateFlags,
    pub queueCreateInfoCount:  u32,
    pub pQueueCreateInfos: *const  VkDeviceQueueCreateInfo,
    pub enabledLayerCount:  u32,
    pub ppEnabledLayerNames: *const  i8,
    pub enabledExtensionCount:  u32,
    pub ppEnabledExtensionNames: *const  i8,
    pub pEnabledFeatures: *const  VkPhysicalDeviceFeatures,
}

#[repr(C)]
pub struct VkInstanceCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkInstanceCreateFlags,
    pub pApplicationInfo: *const  VkApplicationInfo,
    pub enabledLayerCount:  u32,
    pub ppEnabledLayerNames: *const  i8,
    pub enabledExtensionCount:  u32,
    pub ppEnabledExtensionNames: *const  i8,
}

#[repr(C)]
pub struct VkQueueFamilyProperties {
    pub queueFlags:  VkQueueFlags,
    pub queueCount:  u32,
    pub timestampValidBits:  u32,
    pub minImageTransferGranularity:  VkExtent3D,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties {
    pub memoryTypeCount:  u32,
    pub memoryTypes:  [VkMemoryType; VK_MAX_MEMORY_TYPES as usize],
    pub memoryHeapCount:  u32,
    pub memoryHeaps:  [VkMemoryHeap; VK_MAX_MEMORY_HEAPS as usize],
}

#[repr(C)]
pub struct VkMemoryAllocateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub allocationSize:  VkDeviceSize,
    pub memoryTypeIndex:  u32,
}

#[repr(C)]
pub struct VkMemoryRequirements {
    pub size:  VkDeviceSize,
    pub alignment:  VkDeviceSize,
    pub memoryTypeBits:  u32,
}

#[repr(C)]
pub struct VkSparseImageFormatProperties {
    pub aspectMask:  VkImageAspectFlags,
    pub imageGranularity:  VkExtent3D,
    pub flags:  VkSparseImageFormatFlags,
}

#[repr(C)]
pub struct VkSparseImageMemoryRequirements {
    pub formatProperties:  VkSparseImageFormatProperties,
    pub imageMipTailFirstLod:  u32,
    pub imageMipTailSize:  VkDeviceSize,
    pub imageMipTailOffset:  VkDeviceSize,
    pub imageMipTailStride:  VkDeviceSize,
}

#[repr(C)]
pub struct VkMemoryType {
    pub propertyFlags:  VkMemoryPropertyFlags,
    pub heapIndex:  u32,
}

#[repr(C)]
pub struct VkMemoryHeap {
    pub size:  VkDeviceSize,
    pub flags:  VkMemoryHeapFlags,
}

#[repr(C)]
pub struct VkMappedMemoryRange {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub memory:  VkDeviceMemory,
    pub offset:  VkDeviceSize,
    pub size:  VkDeviceSize,
}

#[repr(C)]
pub struct VkFormatProperties {
    pub linearTilingFeatures:  VkFormatFeatureFlags,
    pub optimalTilingFeatures:  VkFormatFeatureFlags,
    pub bufferFeatures:  VkFormatFeatureFlags,
}

#[repr(C)]
pub struct VkImageFormatProperties {
    pub maxExtent:  VkExtent3D,
    pub maxMipLevels:  u32,
    pub maxArrayLayers:  u32,
    pub sampleCounts:  VkSampleCountFlags,
    pub maxResourceSize:  VkDeviceSize,
}

#[repr(C)]
pub struct VkDescriptorBufferInfo {
    pub buffer:  VkBuffer,
    pub offset:  VkDeviceSize,
    pub range:  VkDeviceSize,
}

#[repr(C)]
pub struct VkDescriptorImageInfo {
    pub sampler:  VkSampler,
    pub imageView:  VkImageView,
    pub imageLayout:  VkImageLayout,
}

#[repr(C)]
pub struct VkWriteDescriptorSet {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub dstSet:  VkDescriptorSet,
    pub dstBinding:  u32,
    pub dstArrayElement:  u32,
    pub descriptorCount:  u32,
    pub descriptorType:  VkDescriptorType,
    pub pImageInfo: *const  VkDescriptorImageInfo,
    pub pBufferInfo: *const  VkDescriptorBufferInfo,
    pub pTexelBufferView: *const  VkBufferView,
}

#[repr(C)]
pub struct VkCopyDescriptorSet {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcSet:  VkDescriptorSet,
    pub srcBinding:  u32,
    pub srcArrayElement:  u32,
    pub dstSet:  VkDescriptorSet,
    pub dstBinding:  u32,
    pub dstArrayElement:  u32,
    pub descriptorCount:  u32,
}

#[repr(C)]
pub struct VkBufferUsageFlags2CreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub usage:  VkBufferUsageFlags2,
}

#[repr(C)]
pub struct VkBufferUsageFlags2CreateInfoKHR {
}

#[repr(C)]
pub struct VkBufferCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkBufferCreateFlags,
    pub size:  VkDeviceSize,
    pub usage:  VkBufferUsageFlags,
    pub sharingMode:  VkSharingMode,
    pub queueFamilyIndexCount:  u32,
    pub pQueueFamilyIndices: *const  u32,
}

#[repr(C)]
pub struct VkBufferViewCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkBufferViewCreateFlags,
    pub buffer:  VkBuffer,
    pub format:  VkFormat,
    pub offset:  VkDeviceSize,
    pub range:  VkDeviceSize,
}

#[repr(C)]
pub struct VkImageSubresource {
    pub aspectMask:  VkImageAspectFlags,
    pub mipLevel:  u32,
    pub arrayLayer:  u32,
}

#[repr(C)]
pub struct VkImageSubresourceLayers {
    pub aspectMask:  VkImageAspectFlags,
    pub mipLevel:  u32,
    pub baseArrayLayer:  u32,
    pub layerCount:  u32,
}

#[repr(C)]
pub struct VkImageSubresourceRange {
    pub aspectMask:  VkImageAspectFlags,
    pub baseMipLevel:  u32,
    pub levelCount:  u32,
    pub baseArrayLayer:  u32,
    pub layerCount:  u32,
}

#[repr(C)]
pub struct VkMemoryBarrier {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcAccessMask:  VkAccessFlags,
    pub dstAccessMask:  VkAccessFlags,
}

#[repr(C)]
pub struct VkBufferMemoryBarrier {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcAccessMask:  VkAccessFlags,
    pub dstAccessMask:  VkAccessFlags,
    pub srcQueueFamilyIndex:  u32,
    pub dstQueueFamilyIndex:  u32,
    pub buffer:  VkBuffer,
    pub offset:  VkDeviceSize,
    pub size:  VkDeviceSize,
}

#[repr(C)]
pub struct VkImageMemoryBarrier {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcAccessMask:  VkAccessFlags,
    pub dstAccessMask:  VkAccessFlags,
    pub oldLayout:  VkImageLayout,
    pub newLayout:  VkImageLayout,
    pub srcQueueFamilyIndex:  u32,
    pub dstQueueFamilyIndex:  u32,
    pub image:  VkImage,
    pub subresourceRange:  VkImageSubresourceRange,
}

#[repr(C)]
pub struct VkImageCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkImageCreateFlags,
    pub imageType:  VkImageType,
    pub format:  VkFormat,
    pub extent:  VkExtent3D,
    pub mipLevels:  u32,
    pub arrayLayers:  u32,
    pub samples:  VkSampleCountFlagBits,
    pub tiling:  VkImageTiling,
    pub usage:  VkImageUsageFlags,
    pub sharingMode:  VkSharingMode,
    pub queueFamilyIndexCount:  u32,
    pub pQueueFamilyIndices: *const  u32,
    pub initialLayout:  VkImageLayout,
}

#[repr(C)]
pub struct VkImageCreateFlags2CreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkImageCreateFlags2KHR,
}

#[repr(C)]
pub struct VkImageUsageFlags2CreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub usage:  VkImageUsageFlags2KHR,
}

#[repr(C)]
pub struct VkSubresourceLayout {
    pub offset:  VkDeviceSize,
    pub size:  VkDeviceSize,
    pub rowPitch:  VkDeviceSize,
    pub arrayPitch:  VkDeviceSize,
    pub depthPitch:  VkDeviceSize,
}

#[repr(C)]
pub struct VkImageViewCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkImageViewCreateFlags,
    pub image:  VkImage,
    pub viewType:  VkImageViewType,
    pub format:  VkFormat,
    pub components:  VkComponentMapping,
    pub subresourceRange:  VkImageSubresourceRange,
}

#[repr(C)]
pub struct VkBufferCopy {
    pub srcOffset:  VkDeviceSize,
    pub dstOffset:  VkDeviceSize,
    pub size:  VkDeviceSize,
}

#[repr(C)]
pub struct VkSparseMemoryBind {
    pub resourceOffset:  VkDeviceSize,
    pub size:  VkDeviceSize,
    pub memory:  VkDeviceMemory,
    pub memoryOffset:  VkDeviceSize,
    pub flags:  VkSparseMemoryBindFlags,
}

#[repr(C)]
pub struct VkSparseImageMemoryBind {
    pub subresource:  VkImageSubresource,
    pub offset:  VkOffset3D,
    pub extent:  VkExtent3D,
    pub memory:  VkDeviceMemory,
    pub memoryOffset:  VkDeviceSize,
    pub flags:  VkSparseMemoryBindFlags,
}

#[repr(C)]
pub struct VkSparseBufferMemoryBindInfo {
    pub buffer:  VkBuffer,
    pub bindCount:  u32,
    pub pBinds: *const  VkSparseMemoryBind,
}

#[repr(C)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
    pub image:  VkImage,
    pub bindCount:  u32,
    pub pBinds: *const  VkSparseMemoryBind,
}

#[repr(C)]
pub struct VkSparseImageMemoryBindInfo {
    pub image:  VkImage,
    pub bindCount:  u32,
    pub pBinds: *const  VkSparseImageMemoryBind,
}

#[repr(C)]
pub struct VkBindSparseInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub waitSemaphoreCount:  u32,
    pub pWaitSemaphores: *const  VkSemaphore,
    pub bufferBindCount:  u32,
    pub pBufferBinds: *const  VkSparseBufferMemoryBindInfo,
    pub imageOpaqueBindCount:  u32,
    pub pImageOpaqueBinds: *const  VkSparseImageOpaqueMemoryBindInfo,
    pub imageBindCount:  u32,
    pub pImageBinds: *const  VkSparseImageMemoryBindInfo,
    pub signalSemaphoreCount:  u32,
    pub pSignalSemaphores: *const  VkSemaphore,
}

#[repr(C)]
pub struct VkImageCopy {
    pub srcSubresource:  VkImageSubresourceLayers,
    pub srcOffset:  VkOffset3D,
    pub dstSubresource:  VkImageSubresourceLayers,
    pub dstOffset:  VkOffset3D,
    pub extent:  VkExtent3D,
}

#[repr(C)]
pub struct VkImageBlit {
    pub srcSubresource:  VkImageSubresourceLayers,
    pub srcOffsets:  VkOffset3D,
    pub dstSubresource:  VkImageSubresourceLayers,
    pub dstOffsets:  VkOffset3D,
}

#[repr(C)]
pub struct VkBufferImageCopy {
    pub bufferOffset:  VkDeviceSize,
    pub bufferRowLength:  u32,
    pub bufferImageHeight:  u32,
    pub imageSubresource:  VkImageSubresourceLayers,
    pub imageOffset:  VkOffset3D,
    pub imageExtent:  VkExtent3D,
}

#[repr(C)]
pub struct VkStridedDeviceAddressRangeKHR {
    pub address:  VkDeviceAddress,
    pub size:  VkDeviceSize,
    pub stride:  VkDeviceSize,
}

#[repr(C)]
pub struct VkCopyMemoryIndirectCommandKHR {
    pub srcAddress:  VkDeviceAddress,
    pub dstAddress:  VkDeviceAddress,
    pub size:  VkDeviceSize,
}

#[repr(C)]
pub struct VkCopyMemoryIndirectCommandNV {
}

#[repr(C)]
pub struct VkCopyMemoryIndirectInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcCopyFlags:  VkAddressCopyFlagsKHR,
    pub dstCopyFlags:  VkAddressCopyFlagsKHR,
    pub copyCount:  u32,
    pub copyAddressRange:  VkStridedDeviceAddressRangeKHR,
}

#[repr(C)]
pub struct VkCopyMemoryToImageIndirectCommandKHR {
    pub srcAddress:  VkDeviceAddress,
    pub bufferRowLength:  u32,
    pub bufferImageHeight:  u32,
    pub imageSubresource:  VkImageSubresourceLayers,
    pub imageOffset:  VkOffset3D,
    pub imageExtent:  VkExtent3D,
}

#[repr(C)]
pub struct VkCopyMemoryToImageIndirectCommandNV {
}

#[repr(C)]
pub struct VkCopyMemoryToImageIndirectInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcCopyFlags:  VkAddressCopyFlagsKHR,
    pub copyCount:  u32,
    pub copyAddressRange:  VkStridedDeviceAddressRangeKHR,
    pub dstImage:  VkImage,
    pub dstImageLayout:  VkImageLayout,
    pub pImageSubresources: *const  VkImageSubresourceLayers,
}

#[repr(C)]
pub struct VkImageResolve {
    pub srcSubresource:  VkImageSubresourceLayers,
    pub srcOffset:  VkOffset3D,
    pub dstSubresource:  VkImageSubresourceLayers,
    pub dstOffset:  VkOffset3D,
    pub extent:  VkExtent3D,
}

#[repr(C)]
pub struct VkShaderModuleCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkShaderModuleCreateFlags,
    pub codeSize:  usize,
    pub pCode: *const  u32,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutBinding {
    pub binding:  u32,
    pub descriptorType:  VkDescriptorType,
    pub descriptorCount:  u32,
    pub stageFlags:  VkShaderStageFlags,
    pub pImmutableSamplers: *const  VkSampler,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkDescriptorSetLayoutCreateFlags,
    pub bindingCount:  u32,
    pub pBindings: *const  VkDescriptorSetLayoutBinding,
}

#[repr(C)]
pub struct VkDescriptorPoolSize {
    pub r#type:  VkDescriptorType,
    pub descriptorCount:  u32,
}

#[repr(C)]
pub struct VkDescriptorPoolCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkDescriptorPoolCreateFlags,
    pub maxSets:  u32,
    pub poolSizeCount:  u32,
    pub pPoolSizes: *const  VkDescriptorPoolSize,
}

#[repr(C)]
pub struct VkDescriptorSetAllocateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub descriptorPool:  VkDescriptorPool,
    pub descriptorSetCount:  u32,
    pub pSetLayouts: *const  VkDescriptorSetLayout,
}

#[repr(C)]
pub struct VkSpecializationMapEntry {
    pub constantID:  u32,
    pub offset:  u32,
    pub size:  usize,
}

#[repr(C)]
pub struct VkSpecializationInfo {
    pub mapEntryCount:  u32,
    pub pMapEntries: *const  VkSpecializationMapEntry,
    pub dataSize:  usize,
    pub pData: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPipelineShaderStageCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineShaderStageCreateFlags,
    pub stage:  VkShaderStageFlagBits,
    pub module:  VkShaderModule,
    pub pName: *const  i8,
    //pub pName: *const  i8,
    pub pSpecializationInfo: *const  VkSpecializationInfo,
}

#[repr(C)]
pub struct VkComputePipelineCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineCreateFlags,
    pub stage:  VkPipelineShaderStageCreateInfo,
    pub layout:  VkPipelineLayout,
    pub basePipelineHandle:  VkPipeline,
    pub basePipelineIndex:  i32,
}

#[repr(C)]
pub struct VkComputePipelineIndirectBufferInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub deviceAddress:  VkDeviceAddress,
    pub size:  VkDeviceSize,
    pub pipelineDeviceAddressCaptureReplay:  VkDeviceAddress,
}

#[repr(C)]
pub struct VkPipelineCreateFlags2CreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineCreateFlags2,
}

#[repr(C)]
pub struct VkPipelineCreateFlags2CreateInfoKHR {
}

#[repr(C)]
pub struct VkVertexInputBindingDescription {
    pub binding:  u32,
    pub stride:  u32,
    pub inputRate:  VkVertexInputRate,
}

#[repr(C)]
pub struct VkVertexInputAttributeDescription {
    pub location:  u32,
    pub binding:  u32,
    pub format:  VkFormat,
    pub offset:  u32,
}

#[repr(C)]
pub struct VkPipelineVertexInputStateCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineVertexInputStateCreateFlags,
    pub vertexBindingDescriptionCount:  u32,
    pub pVertexBindingDescriptions: *const  VkVertexInputBindingDescription,
    pub vertexAttributeDescriptionCount:  u32,
    pub pVertexAttributeDescriptions: *const  VkVertexInputAttributeDescription,
}

#[repr(C)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineInputAssemblyStateCreateFlags,
    pub topology:  VkPrimitiveTopology,
    pub primitiveRestartEnable:  VkBool32,
}

#[repr(C)]
pub struct VkPipelineTessellationStateCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineTessellationStateCreateFlags,
    pub patchControlPoints:  u32,
}

#[repr(C)]
pub struct VkPipelineViewportStateCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineViewportStateCreateFlags,
    pub viewportCount:  u32,
    pub pViewports: *const  VkViewport,
    pub scissorCount:  u32,
    pub pScissors: *const  VkRect2D,
}

#[repr(C)]
pub struct VkPipelineRasterizationStateCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineRasterizationStateCreateFlags,
    pub depthClampEnable:  VkBool32,
    pub rasterizerDiscardEnable:  VkBool32,
    pub polygonMode:  VkPolygonMode,
    pub cullMode:  VkCullModeFlags,
    pub frontFace:  VkFrontFace,
    pub depthBiasEnable:  VkBool32,
    pub depthBiasConstantFactor:  f32,
    pub depthBiasClamp:  f32,
    pub depthBiasSlopeFactor:  f32,
    pub lineWidth:  f32,
}

#[repr(C)]
pub struct VkPipelineMultisampleStateCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineMultisampleStateCreateFlags,
    pub rasterizationSamples:  VkSampleCountFlagBits,
    pub sampleShadingEnable:  VkBool32,
    pub minSampleShading:  f32,
    pub pSampleMask: *const  VkSampleMask,
    pub alphaToCoverageEnable:  VkBool32,
    pub alphaToOneEnable:  VkBool32,
}

#[repr(C)]
pub struct VkPipelineColorBlendAttachmentState {
    pub blendEnable:  VkBool32,
    pub srcColorBlendFactor:  VkBlendFactor,
    pub dstColorBlendFactor:  VkBlendFactor,
    pub colorBlendOp:  VkBlendOp,
    pub srcAlphaBlendFactor:  VkBlendFactor,
    pub dstAlphaBlendFactor:  VkBlendFactor,
    pub alphaBlendOp:  VkBlendOp,
    pub colorWriteMask:  VkColorComponentFlags,
}

#[repr(C)]
pub struct VkPipelineColorBlendStateCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineColorBlendStateCreateFlags,
    pub logicOpEnable:  VkBool32,
    pub logicOp:  VkLogicOp,
    pub attachmentCount:  u32,
    pub pAttachments: *const  VkPipelineColorBlendAttachmentState,
    pub blendConstants:  f32,
}

#[repr(C)]
pub struct VkPipelineDynamicStateCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineDynamicStateCreateFlags,
    pub dynamicStateCount:  u32,
    pub pDynamicStates: *const  VkDynamicState,
}

#[repr(C)]
pub struct VkStencilOpState {
    pub failOp:  VkStencilOp,
    pub passOp:  VkStencilOp,
    pub depthFailOp:  VkStencilOp,
    pub compareOp:  VkCompareOp,
    pub compareMask:  u32,
    pub writeMask:  u32,
    pub reference:  u32,
}

#[repr(C)]
pub struct VkPipelineDepthStencilStateCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineDepthStencilStateCreateFlags,
    pub depthTestEnable:  VkBool32,
    pub depthWriteEnable:  VkBool32,
    pub depthCompareOp:  VkCompareOp,
    pub depthBoundsTestEnable:  VkBool32,
    pub stencilTestEnable:  VkBool32,
    pub front:  VkStencilOpState,
    pub back:  VkStencilOpState,
    pub minDepthBounds:  f32,
    pub maxDepthBounds:  f32,
}

#[repr(C)]
pub struct VkGraphicsPipelineCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineCreateFlags,
    pub stageCount:  u32,
    pub pStages: *const  VkPipelineShaderStageCreateInfo,
    //pub pStages: *const  VkPipelineShaderStageCreateInfo,
    pub pVertexInputState: *const  VkPipelineVertexInputStateCreateInfo,
    pub pInputAssemblyState: *const  VkPipelineInputAssemblyStateCreateInfo,
    pub pTessellationState: *const  VkPipelineTessellationStateCreateInfo,
    pub pViewportState: *const  VkPipelineViewportStateCreateInfo,
    pub pRasterizationState: *const  VkPipelineRasterizationStateCreateInfo,
    pub pMultisampleState: *const  VkPipelineMultisampleStateCreateInfo,
    pub pDepthStencilState: *const  VkPipelineDepthStencilStateCreateInfo,
    pub pColorBlendState: *const  VkPipelineColorBlendStateCreateInfo,
    pub pDynamicState: *const  VkPipelineDynamicStateCreateInfo,
    pub layout:  VkPipelineLayout,
    pub renderPass:  VkRenderPass,
    pub subpass:  u32,
    pub basePipelineHandle:  VkPipeline,
    pub basePipelineIndex:  i32,
}

#[repr(C)]
pub struct VkPipelineCacheCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineCacheCreateFlags,
    pub initialDataSize:  usize,
    //pub initialDataSize:  usize,
    pub pInitialData: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPipelineCacheHeaderVersionOne {
    pub headerSize:  u32,
    pub headerVersion:  VkPipelineCacheHeaderVersion,
    pub vendorID:  u32,
    pub deviceID:  u32,
    pub pipelineCacheUUID:  [u8; VK_UUID_SIZE as usize],
}

#[repr(C)]
pub struct VkPipelineCacheStageValidationIndexEntry {
    pub codeSize:  u64,
    pub codeOffset:  u64,
}

#[repr(C)]
pub struct VkPipelineCacheSafetyCriticalIndexEntry {
    pub pipelineIdentifier:  [u8; VK_UUID_SIZE as usize],
    pub pipelineMemorySize:  u64,
    pub jsonSize:  u64,
    pub jsonOffset:  u64,
    pub stageIndexCount:  u32,
    pub stageIndexStride:  u32,
    pub stageIndexOffset:  u64,
}

#[repr(C)]
pub struct VkPipelineCacheHeaderVersionSafetyCriticalOne {
    pub headerVersionOne:  VkPipelineCacheHeaderVersionOne,
    pub validationVersion:  VkPipelineCacheValidationVersion,
    pub implementationData:  u32,
    pub pipelineIndexCount:  u32,
    pub pipelineIndexStride:  u32,
    pub pipelineIndexOffset:  u64,
}

#[repr(C)]
pub struct VkPipelineCacheHeaderVersionDataGraphQCOM {
    pub headerSize:  u32,
    pub headerVersion:  VkPipelineCacheHeaderVersion,
    pub cacheType:  VkDataGraphModelCacheTypeQCOM,
    pub cacheVersion:  u32,
    pub toolchainVersion:  [u32; VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM as usize],
}

#[repr(C)]
pub struct VkPushConstantRange {
    pub stageFlags:  VkShaderStageFlags,
    pub offset:  u32,
    pub size:  u32,
}

#[repr(C)]
pub struct VkPipelineBinaryCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pKeysAndDataInfo: *const  VkPipelineBinaryKeysAndDataKHR,
    pub pipeline:  VkPipeline,
    pub pPipelineCreateInfo: *const  VkPipelineCreateInfoKHR,
}

#[repr(C)]
pub struct VkPipelineBinaryHandlesInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pipelineBinaryCount:  u32,
    pub pPipelineBinaries: *mut  VkPipelineBinaryKHR,
}

#[repr(C)]
pub struct VkPipelineBinaryDataKHR {
    pub dataSize:  usize,
    pub pData: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPipelineBinaryKeysAndDataKHR {
    pub binaryCount:  u32,
    pub pPipelineBinaryKeys: *const  VkPipelineBinaryKeyKHR,
    pub pPipelineBinaryData: *const  VkPipelineBinaryDataKHR,
}

#[repr(C)]
pub struct VkPipelineBinaryKeyKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub keySize:  u32,
    pub key:  [u8; VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR as usize],
}

#[repr(C)]
pub struct VkPipelineBinaryInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub binaryCount:  u32,
    pub pPipelineBinaries: *const  VkPipelineBinaryKHR,
}

#[repr(C)]
pub struct VkReleaseCapturedPipelineDataInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pipeline:  VkPipeline,
}

#[repr(C)]
pub struct VkPipelineBinaryDataInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pipelineBinary:  VkPipelineBinaryKHR,
}

#[repr(C)]
pub struct VkPipelineCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPipelineLayoutCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineLayoutCreateFlags,
    pub setLayoutCount:  u32,
    pub pSetLayouts: *const  VkDescriptorSetLayout,
    pub pushConstantRangeCount:  u32,
    pub pPushConstantRanges: *const  VkPushConstantRange,
}

#[repr(C)]
pub struct VkSamplerCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkSamplerCreateFlags,
    pub magFilter:  VkFilter,
    pub minFilter:  VkFilter,
    pub mipmapMode:  VkSamplerMipmapMode,
    pub addressModeU:  VkSamplerAddressMode,
    pub addressModeV:  VkSamplerAddressMode,
    pub addressModeW:  VkSamplerAddressMode,
    pub mipLodBias:  f32,
    pub anisotropyEnable:  VkBool32,
    pub maxAnisotropy:  f32,
    pub compareEnable:  VkBool32,
    pub compareOp:  VkCompareOp,
    pub minLod:  f32,
    pub maxLod:  f32,
    pub borderColor:  VkBorderColor,
    pub unnormalizedCoordinates:  VkBool32,
}

#[repr(C)]
pub struct VkCommandPoolCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkCommandPoolCreateFlags,
    pub queueFamilyIndex:  u32,
}

#[repr(C)]
pub struct VkCommandBufferAllocateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub commandPool:  VkCommandPool,
    pub level:  VkCommandBufferLevel,
    pub commandBufferCount:  u32,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub renderPass:  VkRenderPass,
    pub subpass:  u32,
    pub framebuffer:  VkFramebuffer,
    pub occlusionQueryEnable:  VkBool32,
    pub queryFlags:  VkQueryControlFlags,
    pub pipelineStatistics:  VkQueryPipelineStatisticFlags,
}

#[repr(C)]
pub struct VkCommandBufferBeginInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkCommandBufferUsageFlags,
    pub pInheritanceInfo: *const  VkCommandBufferInheritanceInfo,
}

#[repr(C)]
pub struct VkRenderPassBeginInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub renderPass:  VkRenderPass,
    pub framebuffer:  VkFramebuffer,
    pub renderArea:  VkRect2D,
    pub clearValueCount:  u32,
    pub pClearValues: *const  VkClearValue,
}

#[repr(C)]
pub struct VkClearColorValue {
    pub float32:  f32,
    pub int32:  i32,
    pub uint32:  u32,
}

#[repr(C)]
pub struct VkClearDepthStencilValue {
    pub depth:  f32,
    pub stencil:  u32,
}

#[repr(C)]
pub struct VkClearValue {
    pub color:  VkClearColorValue,
    pub depthStencil:  VkClearDepthStencilValue,
}

#[repr(C)]
pub struct VkClearAttachment {
    pub aspectMask:  VkImageAspectFlags,
    pub colorAttachment:  u32,
    pub clearValue:  VkClearValue,
}

#[repr(C)]
pub struct VkAttachmentDescription {
    pub flags:  VkAttachmentDescriptionFlags,
    pub format:  VkFormat,
    pub samples:  VkSampleCountFlagBits,
    pub loadOp:  VkAttachmentLoadOp,
    pub storeOp:  VkAttachmentStoreOp,
    pub stencilLoadOp:  VkAttachmentLoadOp,
    pub stencilStoreOp:  VkAttachmentStoreOp,
    pub initialLayout:  VkImageLayout,
    pub finalLayout:  VkImageLayout,
}

#[repr(C)]
pub struct VkAttachmentReference {
    pub attachment:  u32,
    pub layout:  VkImageLayout,
}

#[repr(C)]
pub struct VkSubpassDescription {
    pub flags:  VkSubpassDescriptionFlags,
    pub pipelineBindPoint:  VkPipelineBindPoint,
    pub inputAttachmentCount:  u32,
    pub pInputAttachments: *const  VkAttachmentReference,
    pub colorAttachmentCount:  u32,
    pub pColorAttachments: *const  VkAttachmentReference,
    pub pResolveAttachments: *const  VkAttachmentReference,
    pub pDepthStencilAttachment: *const  VkAttachmentReference,
    pub preserveAttachmentCount:  u32,
    pub pPreserveAttachments: *const  u32,
}

#[repr(C)]
pub struct VkSubpassDependency {
    pub srcSubpass:  u32,
    pub dstSubpass:  u32,
    pub srcStageMask:  VkPipelineStageFlags,
    pub dstStageMask:  VkPipelineStageFlags,
    pub srcAccessMask:  VkAccessFlags,
    pub dstAccessMask:  VkAccessFlags,
    pub dependencyFlags:  VkDependencyFlags,
}

#[repr(C)]
pub struct VkRenderPassCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkRenderPassCreateFlags,
    pub attachmentCount:  u32,
    pub pAttachments: *const  VkAttachmentDescription,
    pub subpassCount:  u32,
    pub pSubpasses: *const  VkSubpassDescription,
    pub dependencyCount:  u32,
    pub pDependencies: *const  VkSubpassDependency,
}

#[repr(C)]
pub struct VkEventCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkEventCreateFlags,
}

#[repr(C)]
pub struct VkFenceCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkFenceCreateFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceFeatures {
    pub robustBufferAccess:  VkBool32,
    pub fullDrawIndexUint32:  VkBool32,
    pub imageCubeArray:  VkBool32,
    pub independentBlend:  VkBool32,
    pub geometryShader:  VkBool32,
    pub tessellationShader:  VkBool32,
    pub sampleRateShading:  VkBool32,
    pub dualSrcBlend:  VkBool32,
    pub logicOp:  VkBool32,
    pub multiDrawIndirect:  VkBool32,
    pub drawIndirectFirstInstance:  VkBool32,
    pub depthClamp:  VkBool32,
    pub depthBiasClamp:  VkBool32,
    pub fillModeNonSolid:  VkBool32,
    pub depthBounds:  VkBool32,
    pub wideLines:  VkBool32,
    pub largePoints:  VkBool32,
    pub alphaToOne:  VkBool32,
    pub multiViewport:  VkBool32,
    pub samplerAnisotropy:  VkBool32,
    pub textureCompressionETC2:  VkBool32,
    pub textureCompressionASTC_LDR:  VkBool32,
    pub textureCompressionBC:  VkBool32,
    pub occlusionQueryPrecise:  VkBool32,
    pub pipelineStatisticsQuery:  VkBool32,
    pub vertexPipelineStoresAndAtomics:  VkBool32,
    pub fragmentStoresAndAtomics:  VkBool32,
    pub shaderTessellationAndGeometryPointSize:  VkBool32,
    pub shaderImageGatherExtended:  VkBool32,
    pub shaderStorageImageExtendedFormats:  VkBool32,
    pub shaderStorageImageMultisample:  VkBool32,
    pub shaderStorageImageReadWithoutFormat:  VkBool32,
    pub shaderStorageImageWriteWithoutFormat:  VkBool32,
    pub shaderUniformBufferArrayDynamicIndexing:  VkBool32,
    pub shaderSampledImageArrayDynamicIndexing:  VkBool32,
    pub shaderStorageBufferArrayDynamicIndexing:  VkBool32,
    pub shaderStorageImageArrayDynamicIndexing:  VkBool32,
    pub shaderClipDistance:  VkBool32,
    pub shaderCullDistance:  VkBool32,
    pub shaderFloat64:  VkBool32,
    pub shaderInt64:  VkBool32,
    pub shaderInt16:  VkBool32,
    pub shaderResourceResidency:  VkBool32,
    pub shaderResourceMinLod:  VkBool32,
    pub sparseBinding:  VkBool32,
    pub sparseResidencyBuffer:  VkBool32,
    pub sparseResidencyImage2D:  VkBool32,
    pub sparseResidencyImage3D:  VkBool32,
    pub sparseResidency2Samples:  VkBool32,
    pub sparseResidency4Samples:  VkBool32,
    pub sparseResidency8Samples:  VkBool32,
    pub sparseResidency16Samples:  VkBool32,
    pub sparseResidencyAliased:  VkBool32,
    pub variableMultisampleRate:  VkBool32,
    pub inheritedQueries:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape:  VkBool32,
    pub residencyStandard2DMultisampleBlockShape:  VkBool32,
    pub residencyStandard3DBlockShape:  VkBool32,
    pub residencyAlignedMipSize:  VkBool32,
    pub residencyNonResidentStrict:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceLimits {
    pub maxImageDimension1D:  u32,
    pub maxImageDimension2D:  u32,
    pub maxImageDimension3D:  u32,
    pub maxImageDimensionCube:  u32,
    pub maxImageArrayLayers:  u32,
    pub maxTexelBufferElements:  u32,
    pub maxUniformBufferRange:  u32,
    pub maxStorageBufferRange:  u32,
    pub maxPushConstantsSize:  u32,
    pub maxMemoryAllocationCount:  u32,
    pub maxSamplerAllocationCount:  u32,
    pub bufferImageGranularity:  VkDeviceSize,
    pub sparseAddressSpaceSize:  VkDeviceSize,
    pub maxBoundDescriptorSets:  u32,
    pub maxPerStageDescriptorSamplers:  u32,
    pub maxPerStageDescriptorUniformBuffers:  u32,
    pub maxPerStageDescriptorStorageBuffers:  u32,
    pub maxPerStageDescriptorSampledImages:  u32,
    pub maxPerStageDescriptorStorageImages:  u32,
    pub maxPerStageDescriptorInputAttachments:  u32,
    pub maxPerStageResources:  u32,
    pub maxDescriptorSetSamplers:  u32,
    pub maxDescriptorSetUniformBuffers:  u32,
    pub maxDescriptorSetUniformBuffersDynamic:  u32,
    pub maxDescriptorSetStorageBuffers:  u32,
    pub maxDescriptorSetStorageBuffersDynamic:  u32,
    pub maxDescriptorSetSampledImages:  u32,
    pub maxDescriptorSetStorageImages:  u32,
    pub maxDescriptorSetInputAttachments:  u32,
    pub maxVertexInputAttributes:  u32,
    pub maxVertexInputBindings:  u32,
    pub maxVertexInputAttributeOffset:  u32,
    pub maxVertexInputBindingStride:  u32,
    pub maxVertexOutputComponents:  u32,
    pub maxTessellationGenerationLevel:  u32,
    pub maxTessellationPatchSize:  u32,
    pub maxTessellationControlPerVertexInputComponents:  u32,
    pub maxTessellationControlPerVertexOutputComponents:  u32,
    pub maxTessellationControlPerPatchOutputComponents:  u32,
    pub maxTessellationControlTotalOutputComponents:  u32,
    pub maxTessellationEvaluationInputComponents:  u32,
    pub maxTessellationEvaluationOutputComponents:  u32,
    pub maxGeometryShaderInvocations:  u32,
    pub maxGeometryInputComponents:  u32,
    pub maxGeometryOutputComponents:  u32,
    pub maxGeometryOutputVertices:  u32,
    pub maxGeometryTotalOutputComponents:  u32,
    pub maxFragmentInputComponents:  u32,
    pub maxFragmentOutputAttachments:  u32,
    pub maxFragmentDualSrcAttachments:  u32,
    pub maxFragmentCombinedOutputResources:  u32,
    pub maxComputeSharedMemorySize:  u32,
    pub maxComputeWorkGroupCount:  u32,
    pub maxComputeWorkGroupInvocations:  u32,
    pub maxComputeWorkGroupSize:  u32,
    pub subPixelPrecisionBits:  u32,
    pub subTexelPrecisionBits:  u32,
    pub mipmapPrecisionBits:  u32,
    pub maxDrawIndexedIndexValue:  u32,
    pub maxDrawIndirectCount:  u32,
    pub maxSamplerLodBias:  f32,
    pub maxSamplerAnisotropy:  f32,
    pub maxViewports:  u32,
    pub maxViewportDimensions:  u32,
    pub viewportBoundsRange:  f32,
    pub viewportSubPixelBits:  u32,
    pub minMemoryMapAlignment:  usize,
    pub minTexelBufferOffsetAlignment:  VkDeviceSize,
    pub minUniformBufferOffsetAlignment:  VkDeviceSize,
    pub minStorageBufferOffsetAlignment:  VkDeviceSize,
    pub minTexelOffset:  i32,
    pub maxTexelOffset:  u32,
    pub minTexelGatherOffset:  i32,
    pub maxTexelGatherOffset:  u32,
    pub minInterpolationOffset:  f32,
    pub maxInterpolationOffset:  f32,
    pub subPixelInterpolationOffsetBits:  u32,
    pub maxFramebufferWidth:  u32,
    pub maxFramebufferHeight:  u32,
    pub maxFramebufferLayers:  u32,
    pub framebufferColorSampleCounts:  VkSampleCountFlags,
    pub framebufferDepthSampleCounts:  VkSampleCountFlags,
    pub framebufferStencilSampleCounts:  VkSampleCountFlags,
    pub framebufferNoAttachmentsSampleCounts:  VkSampleCountFlags,
    pub maxColorAttachments:  u32,
    pub sampledImageColorSampleCounts:  VkSampleCountFlags,
    pub sampledImageIntegerSampleCounts:  VkSampleCountFlags,
    pub sampledImageDepthSampleCounts:  VkSampleCountFlags,
    pub sampledImageStencilSampleCounts:  VkSampleCountFlags,
    pub storageImageSampleCounts:  VkSampleCountFlags,
    pub maxSampleMaskWords:  u32,
    pub timestampComputeAndGraphics:  VkBool32,
    pub timestampPeriod:  f32,
    pub maxClipDistances:  u32,
    pub maxCullDistances:  u32,
    pub maxCombinedClipAndCullDistances:  u32,
    pub discreteQueuePriorities:  u32,
    pub pointSizeRange:  f32,
    pub lineWidthRange:  f32,
    pub pointSizeGranularity:  f32,
    pub lineWidthGranularity:  f32,
    pub strictLines:  VkBool32,
    pub standardSampleLocations:  VkBool32,
    pub optimalBufferCopyOffsetAlignment:  VkDeviceSize,
    pub optimalBufferCopyRowPitchAlignment:  VkDeviceSize,
    pub nonCoherentAtomSize:  VkDeviceSize,
}

#[repr(C)]
pub struct VkSemaphoreCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkSemaphoreCreateFlags,
}

#[repr(C)]
pub struct VkQueryPoolCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkQueryPoolCreateFlags,
    pub queryType:  VkQueryType,
    pub queryCount:  u32,
    pub pipelineStatistics:  VkQueryPipelineStatisticFlags,
}

#[repr(C)]
pub struct VkFramebufferCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkFramebufferCreateFlags,
    pub renderPass:  VkRenderPass,
    pub attachmentCount:  u32,
    pub pAttachments: *const  VkImageView,
    pub width:  u32,
    pub height:  u32,
    pub layers:  u32,
}

#[repr(C)]
pub struct VkDrawIndirectCommand {
    pub vertexCount:  u32,
    pub instanceCount:  u32,
    pub firstVertex:  u32,
    pub firstInstance:  u32,
}

#[repr(C)]
pub struct VkDrawIndexedIndirectCommand {
    pub indexCount:  u32,
    pub instanceCount:  u32,
    pub firstIndex:  u32,
    pub vertexOffset:  i32,
    pub firstInstance:  u32,
}

#[repr(C)]
pub struct VkDispatchIndirectCommand {
    pub x:  u32,
    pub y:  u32,
    pub z:  u32,
}

#[repr(C)]
pub struct VkMultiDrawInfoEXT {
    pub firstVertex:  u32,
    pub vertexCount:  u32,
}

#[repr(C)]
pub struct VkMultiDrawIndexedInfoEXT {
    pub firstIndex:  u32,
    pub indexCount:  u32,
    pub vertexOffset:  i32,
}

#[repr(C)]
pub struct VkSubmitInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub waitSemaphoreCount:  u32,
    pub pWaitSemaphores: *const  VkSemaphore,
    pub pWaitDstStageMask: *const  VkPipelineStageFlags,
    pub commandBufferCount:  u32,
    pub pCommandBuffers: *const  VkCommandBuffer,
    pub signalSemaphoreCount:  u32,
    pub pSignalSemaphores: *const  VkSemaphore,
}

#[repr(C)]
pub struct VkDisplayPropertiesKHR {
    pub display:  VkDisplayKHR,
    pub displayName: *const  i8,
    pub physicalDimensions:  VkExtent2D,
    pub physicalResolution:  VkExtent2D,
    pub supportedTransforms:  VkSurfaceTransformFlagsKHR,
    pub planeReorderPossible:  VkBool32,
    pub persistentContent:  VkBool32,
}

#[repr(C)]
pub struct VkDisplayPlanePropertiesKHR {
    pub currentDisplay:  VkDisplayKHR,
    pub currentStackIndex:  u32,
}

#[repr(C)]
pub struct VkDisplayModeParametersKHR {
    pub visibleRegion:  VkExtent2D,
    pub refreshRate:  u32,
}

#[repr(C)]
pub struct VkDisplayModePropertiesKHR {
    pub displayMode:  VkDisplayModeKHR,
    pub parameters:  VkDisplayModeParametersKHR,
}

#[repr(C)]
pub struct VkDisplayModeCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkDisplayModeCreateFlagsKHR,
    pub parameters:  VkDisplayModeParametersKHR,
}

#[repr(C)]
pub struct VkDisplayPlaneCapabilitiesKHR {
    pub supportedAlpha:  VkDisplayPlaneAlphaFlagsKHR,
    pub minSrcPosition:  VkOffset2D,
    pub maxSrcPosition:  VkOffset2D,
    pub minSrcExtent:  VkExtent2D,
    pub maxSrcExtent:  VkExtent2D,
    pub minDstPosition:  VkOffset2D,
    pub maxDstPosition:  VkOffset2D,
    pub minDstExtent:  VkExtent2D,
    pub maxDstExtent:  VkExtent2D,
}

#[repr(C)]
pub struct VkDisplaySurfaceCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkDisplaySurfaceCreateFlagsKHR,
    pub displayMode:  VkDisplayModeKHR,
    pub planeIndex:  u32,
    pub planeStackIndex:  u32,
    pub transform:  VkSurfaceTransformFlagBitsKHR,
    pub globalAlpha:  f32,
    pub alphaMode:  VkDisplayPlaneAlphaFlagBitsKHR,
    pub imageExtent:  VkExtent2D,
}

#[repr(C)]
pub struct VkDisplaySurfaceStereoCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stereoType:  VkDisplaySurfaceStereoTypeNV,
}

#[repr(C)]
pub struct VkDisplayPresentInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcRect:  VkRect2D,
    pub dstRect:  VkRect2D,
    pub persistent:  VkBool32,
}

#[repr(C)]
pub struct VkSurfaceCapabilitiesKHR {
    pub minImageCount:  u32,
    pub maxImageCount:  u32,
    pub currentExtent:  VkExtent2D,
    pub minImageExtent:  VkExtent2D,
    pub maxImageExtent:  VkExtent2D,
    pub maxImageArrayLayers:  u32,
    pub supportedTransforms:  VkSurfaceTransformFlagsKHR,
    pub currentTransform:  VkSurfaceTransformFlagBitsKHR,
    pub supportedCompositeAlpha:  VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags:  VkImageUsageFlags,
}

#[repr(C)]
pub struct VkAndroidSurfaceCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkAndroidSurfaceCreateFlagsKHR,
    pub window: *mut  ANativeWindow,
}

#[repr(C)]
pub struct VkViSurfaceCreateInfoNN {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkViSurfaceCreateFlagsNN,
    pub window: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkWaylandSurfaceCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkWaylandSurfaceCreateFlagsKHR,
    pub display: *mut  wl_display,
    pub surface: *mut  wl_surface,
}

#[repr(C)]
pub struct VkUbmSurfaceCreateInfoSEC {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkUbmSurfaceCreateFlagsSEC,
    pub device: *mut  ubm_device,
    pub surface: *mut  ubm_surface,
}

#[repr(C)]
pub struct VkWin32SurfaceCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkWin32SurfaceCreateFlagsKHR,
    pub hinstance:  HINSTANCE,
    pub hwnd:  HWND,
}

#[repr(C)]
pub struct VkXlibSurfaceCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkXlibSurfaceCreateFlagsKHR,
    pub dpy: *mut  Display,
    pub window:  Window,
}

#[repr(C)]
pub struct VkXcbSurfaceCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkXcbSurfaceCreateFlagsKHR,
    pub connection: *mut  xcb_connection_t,
    pub window:  xcb_window_t,
}

#[repr(C)]
pub struct VkDirectFBSurfaceCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkDirectFBSurfaceCreateFlagsEXT,
    pub dfb: *mut  IDirectFB,
    pub surface: *mut  IDirectFBSurface,
}

#[repr(C)]
pub struct VkImagePipeSurfaceCreateInfoFUCHSIA {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkImagePipeSurfaceCreateFlagsFUCHSIA,
    pub imagePipeHandle:  zx_handle_t,
}

#[repr(C)]
pub struct VkStreamDescriptorSurfaceCreateInfoGGP {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkStreamDescriptorSurfaceCreateFlagsGGP,
    pub streamDescriptor:  GgpStreamDescriptor,
}

#[repr(C)]
pub struct VkScreenSurfaceCreateInfoQNX {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkScreenSurfaceCreateFlagsQNX,
    pub context: *mut  _screen_context,
    pub window: *mut  _screen_window,
}

#[repr(C)]
pub struct VkSurfaceFormatKHR {
    pub format:  VkFormat,
    pub colorSpace:  VkColorSpaceKHR,
}

#[repr(C)]
pub struct VkSwapchainCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkSwapchainCreateFlagsKHR,
    pub surface:  VkSurfaceKHR,
    pub minImageCount:  u32,
    pub imageFormat:  VkFormat,
    pub imageColorSpace:  VkColorSpaceKHR,
    pub imageExtent:  VkExtent2D,
    pub imageArrayLayers:  u32,
    pub imageUsage:  VkImageUsageFlags,
    pub imageSharingMode:  VkSharingMode,
    pub queueFamilyIndexCount:  u32,
    pub pQueueFamilyIndices: *const  u32,
    pub preTransform:  VkSurfaceTransformFlagBitsKHR,
    pub compositeAlpha:  VkCompositeAlphaFlagBitsKHR,
    pub presentMode:  VkPresentModeKHR,
    pub clipped:  VkBool32,
    pub oldSwapchain:  VkSwapchainKHR,
    //pub oldSwapchain:  VkSwapchainKHR,
}

#[repr(C)]
pub struct VkPresentInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub waitSemaphoreCount:  u32,
    pub pWaitSemaphores: *const  VkSemaphore,
    pub swapchainCount:  u32,
    pub pSwapchains: *const  VkSwapchainKHR,
    pub pImageIndices: *const  u32,
    pub pResults: *mut  VkResult,
}

#[repr(C)]
pub struct VkDebugReportCallbackCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkDebugReportFlagsEXT,
    pub pfnCallback:  crate::svk_commands::PFN_vkDebugReportCallbackEXT,
    pub pUserData: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkValidationFlagsEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub disabledValidationCheckCount:  u32,
    pub pDisabledValidationChecks: *const  VkValidationCheckEXT,
}

#[repr(C)]
pub struct VkValidationFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub enabledValidationFeatureCount:  u32,
    pub pEnabledValidationFeatures: *const  VkValidationFeatureEnableEXT,
    pub disabledValidationFeatureCount:  u32,
    pub pDisabledValidationFeatures: *const  VkValidationFeatureDisableEXT,
}

#[repr(C)]
pub struct VkLayerSettingsCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub settingCount:  u32,
    pub pSettings: *const  VkLayerSettingEXT,
}

#[repr(C)]
pub struct VkLayerSettingEXT {
    pub pLayerName: *const  i8,
    pub pSettingName: *const  i8,
    pub r#type:  VkLayerSettingTypeEXT,
    pub valueCount:  u32,
    pub pValues: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkApplicationParametersEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub vendorID:  u32,
    pub deviceID:  u32,
    pub key:  u32,
    pub value:  u64,
}

#[repr(C)]
pub struct VkPipelineRasterizationStateRasterizationOrderAMD {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub rasterizationOrder:  VkRasterizationOrderAMD,
}

#[repr(C)]
pub struct VkDebugMarkerObjectNameInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub objectType:  VkDebugReportObjectTypeEXT,
    pub object:  u64,
    pub pObjectName: *const  i8,
}

#[repr(C)]
pub struct VkDebugMarkerObjectTagInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub objectType:  VkDebugReportObjectTypeEXT,
    pub object:  u64,
    pub tagName:  u64,
    pub tagSize:  usize,
    pub pTag: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkDebugMarkerMarkerInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pMarkerName: *const  i8,
    pub color:  f32,
}

#[repr(C)]
pub struct VkDedicatedAllocationImageCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub dedicatedAllocation:  VkBool32,
}

#[repr(C)]
pub struct VkDedicatedAllocationBufferCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub dedicatedAllocation:  VkBool32,
}

#[repr(C)]
pub struct VkDedicatedAllocationMemoryAllocateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub image:  VkImage,
    pub buffer:  VkBuffer,
}

#[repr(C)]
pub struct VkExternalImageFormatPropertiesNV {
    pub imageFormatProperties:  VkImageFormatProperties,
    pub externalMemoryFeatures:  VkExternalMemoryFeatureFlagsNV,
    pub exportFromImportedHandleTypes:  VkExternalMemoryHandleTypeFlagsNV,
    pub compatibleHandleTypes:  VkExternalMemoryHandleTypeFlagsNV,
}

#[repr(C)]
pub struct VkExternalMemoryImageCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleTypes:  VkExternalMemoryHandleTypeFlagsNV,
}

#[repr(C)]
pub struct VkExportMemoryAllocateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleTypes:  VkExternalMemoryHandleTypeFlagsNV,
}

#[repr(C)]
pub struct VkImportMemoryWin32HandleInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleType:  VkExternalMemoryHandleTypeFlagsNV,
    pub handle:  HANDLE,
}

#[repr(C)]
pub struct VkExportMemoryWin32HandleInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pAttributes: *const  SECURITY_ATTRIBUTES,
    pub dwAccess:  DWORD,
}

#[repr(C)]
pub struct VkExportMemorySciBufInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pAttributes:  NvSciBufAttrList,
}

#[repr(C)]
pub struct VkImportMemorySciBufInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleType:  VkExternalMemoryHandleTypeFlagBits,
    pub handle:  NvSciBufObj,
}

#[repr(C)]
pub struct VkMemoryGetSciBufInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub memory:  VkDeviceMemory,
    pub handleType:  VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkMemorySciBufPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub memoryTypeBits:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalMemorySciBufFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub sciBufImport:  VkBool32,
    pub sciBufExport:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalSciBufFeaturesNV {
}

#[repr(C)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub acquireCount:  u32,
    pub pAcquireSyncs: *const  VkDeviceMemory,
    pub pAcquireKeys: *const  u64,
    pub pAcquireTimeoutMilliseconds: *const  u32,
    pub releaseCount:  u32,
    pub pReleaseSyncs: *const  VkDeviceMemory,
    pub pReleaseKeys: *const  u64,
}

#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub deviceGeneratedCommands:  VkBool32,
}

#[repr(C)]
pub struct VkPushConstantBankInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub bank:  u32,
}

#[repr(C)]
pub struct VkPhysicalDevicePushConstantBankFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pushConstantBank:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePushConstantBankPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxGraphicsPushConstantBanks:  u32,
    pub maxComputePushConstantBanks:  u32,
    pub maxGraphicsPushDataBanks:  u32,
    pub maxComputePushDataBanks:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub deviceGeneratedCompute:  VkBool32,
    pub deviceGeneratedComputePipelines:  VkBool32,
    pub deviceGeneratedComputeCaptureReplay:  VkBool32,
}

#[repr(C)]
pub struct VkDevicePrivateDataCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub privateDataSlotRequestCount:  u32,
}

#[repr(C)]
pub struct VkDevicePrivateDataCreateInfoEXT {
}

#[repr(C)]
pub struct VkPrivateDataSlotCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPrivateDataSlotCreateFlags,
}

#[repr(C)]
pub struct VkPrivateDataSlotCreateInfoEXT {
}

#[repr(C)]
pub struct VkPhysicalDevicePrivateDataFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub privateData:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePrivateDataFeaturesEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxGraphicsShaderGroupCount:  u32,
    pub maxIndirectSequenceCount:  u32,
    pub maxIndirectCommandsTokenCount:  u32,
    pub maxIndirectCommandsStreamCount:  u32,
    pub maxIndirectCommandsTokenOffset:  u32,
    pub maxIndirectCommandsStreamStride:  u32,
    pub minSequencesCountBufferOffsetAlignment:  u32,
    pub minSequencesIndexBufferOffsetAlignment:  u32,
    pub minIndirectCommandsBufferOffsetAlignment:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceClusterAccelerationStructureFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub clusterAccelerationStructure:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceClusterAccelerationStructurePropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxVerticesPerCluster:  u32,
    pub maxTrianglesPerCluster:  u32,
    pub clusterScratchByteAlignment:  u32,
    pub clusterByteAlignment:  u32,
    pub clusterTemplateByteAlignment:  u32,
    pub clusterBottomLevelByteAlignment:  u32,
    pub clusterTemplateBoundsByteAlignment:  u32,
    pub maxClusterGeometryIndex:  u32,
}

#[repr(C)]
pub struct VkStridedDeviceAddressNV {
    pub startAddress:  VkDeviceAddress,
    pub strideInBytes:  VkDeviceSize,
}

#[repr(C)]
pub struct VkRayTracingPipelineClusterAccelerationStructureCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub allowClusterAccelerationStructure:  VkBool32,
}

#[repr(C)]
pub struct VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV {
    pub geometryIndex:  u32,
    pub reserved:  u32,
    pub geometryFlags:  u32,
}

#[repr(C)]
pub struct VkClusterAccelerationStructureMoveObjectsInfoNV {
    pub srcAccelerationStructure:  VkDeviceAddress,
}

#[repr(C)]
pub struct VkClusterAccelerationStructureBuildClustersBottomLevelInfoNV {
    pub clusterReferencesCount:  u32,
    pub clusterReferencesStride:  u32,
    pub clusterReferences:  VkDeviceAddress,
}

#[repr(C)]
pub struct VkClusterAccelerationStructureGetTemplateIndicesInfoNV {
    pub clusterTemplateAddress:  VkDeviceAddress,
}

#[repr(C)]
pub struct VkClusterAccelerationStructureBuildTriangleClusterInfoNV {
    pub clusterID:  u32,
    pub clusterFlags:  VkClusterAccelerationStructureClusterFlagsNV,
    pub triangleCount:  u32,
    pub vertexCount:  u32,
    pub positionTruncateBitCount:  u32,
    pub indexType:  u32,
    pub opacityMicromapIndexType:  u32,
    pub baseGeometryIndexAndGeometryFlags:  VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV,
    pub indexBufferStride:  u16,
    pub vertexBufferStride:  u16,
    pub geometryIndexAndFlagsBufferStride:  u16,
    pub opacityMicromapIndexBufferStride:  u16,
    pub indexBuffer:  VkDeviceAddress,
    pub vertexBuffer:  VkDeviceAddress,
    pub geometryIndexAndFlagsBuffer:  VkDeviceAddress,
    pub opacityMicromapArray:  VkDeviceAddress,
    pub opacityMicromapIndexBuffer:  VkDeviceAddress,
}

#[repr(C)]
pub struct VkClusterAccelerationStructureBuildTriangleClusterTemplateInfoNV {
    pub clusterID:  u32,
    pub clusterFlags:  VkClusterAccelerationStructureClusterFlagsNV,
    pub triangleCount:  u32,
    pub vertexCount:  u32,
    pub positionTruncateBitCount:  u32,
    pub indexType:  u32,
    pub opacityMicromapIndexType:  u32,
    pub baseGeometryIndexAndGeometryFlags:  VkClusterAccelerationStructureGeometryIndexAndGeometryFlagsNV,
    pub indexBufferStride:  u16,
    pub vertexBufferStride:  u16,
    pub geometryIndexAndFlagsBufferStride:  u16,
    pub opacityMicromapIndexBufferStride:  u16,
    pub indexBuffer:  VkDeviceAddress,
    pub vertexBuffer:  VkDeviceAddress,
    pub geometryIndexAndFlagsBuffer:  VkDeviceAddress,
    pub opacityMicromapArray:  VkDeviceAddress,
    pub opacityMicromapIndexBuffer:  VkDeviceAddress,
    pub instantiationBoundingBoxLimit:  VkDeviceAddress,
}

#[repr(C)]
pub struct VkClusterAccelerationStructureInstantiateClusterInfoNV {
    pub clusterIdOffset:  u32,
    pub geometryIndexOffset:  u32,
    pub reserved:  u32,
    pub clusterTemplateAddress:  VkDeviceAddress,
    pub vertexBuffer:  VkStridedDeviceAddressNV,
}

#[repr(C)]
pub struct VkClusterAccelerationStructureClustersBottomLevelInputNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxTotalClusterCount:  u32,
    pub maxClusterCountPerAccelerationStructure:  u32,
}

#[repr(C)]
pub struct VkClusterAccelerationStructureTriangleClusterInputNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub vertexFormat:  VkFormat,
    pub maxGeometryIndexValue:  u32,
    pub maxClusterUniqueGeometryCount:  u32,
    pub maxClusterTriangleCount:  u32,
    pub maxClusterVertexCount:  u32,
    pub maxTotalTriangleCount:  u32,
    pub maxTotalVertexCount:  u32,
    pub minPositionTruncateBitCount:  u32,
}

#[repr(C)]
pub struct VkClusterAccelerationStructureMoveObjectsInputNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub r#type:  VkClusterAccelerationStructureTypeNV,
    pub noMoveOverlap:  VkBool32,
    pub maxMovedBytes:  VkDeviceSize,
}

#[repr(C)]
pub struct VkClusterAccelerationStructureOpInputNV {
    pub pClustersBottomLevel: *mut  VkClusterAccelerationStructureClustersBottomLevelInputNV,
    pub pTriangleClusters: *mut  VkClusterAccelerationStructureTriangleClusterInputNV,
    pub pMoveObjects: *mut  VkClusterAccelerationStructureMoveObjectsInputNV,
}

#[repr(C)]
pub struct VkClusterAccelerationStructureInputInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxAccelerationStructureCount:  u32,
    pub flags:  VkBuildAccelerationStructureFlagsKHR,
    pub opType:  VkClusterAccelerationStructureOpTypeNV,
    pub opMode:  VkClusterAccelerationStructureOpModeNV,
    pub opInput:  VkClusterAccelerationStructureOpInputNV,
}

#[repr(C)]
pub struct VkClusterAccelerationStructureCommandsInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub input:  VkClusterAccelerationStructureInputInfoNV,
    pub dstImplicitData:  VkDeviceAddress,
    pub scratchData:  VkDeviceAddress,
    pub dstAddressesArray:  VkStridedDeviceAddressRegionKHR,
    pub dstSizesArray:  VkStridedDeviceAddressRegionKHR,
    pub srcInfosArray:  VkStridedDeviceAddressRegionKHR,
    pub srcInfosCount:  VkDeviceAddress,
    pub addressResolutionFlags:  VkClusterAccelerationStructureAddressResolutionFlagsNV,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiDrawPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxMultiDrawCount:  u32,
}

#[repr(C)]
pub struct VkGraphicsShaderGroupCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stageCount:  u32,
    pub pStages: *const  VkPipelineShaderStageCreateInfo,
    pub pVertexInputState: *const  VkPipelineVertexInputStateCreateInfo,
    pub pTessellationState: *const  VkPipelineTessellationStateCreateInfo,
}

#[repr(C)]
pub struct VkGraphicsPipelineShaderGroupsCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub groupCount:  u32,
    pub pGroups: *const  VkGraphicsShaderGroupCreateInfoNV,
    pub pipelineCount:  u32,
    pub pPipelines: *const  VkPipeline,
}

#[repr(C)]
pub struct VkBindShaderGroupIndirectCommandNV {
    pub groupIndex:  u32,
}

#[repr(C)]
pub struct VkBindIndexBufferIndirectCommandNV {
    pub bufferAddress:  VkDeviceAddress,
    pub size:  u32,
    pub indexType:  VkIndexType,
}

#[repr(C)]
pub struct VkBindVertexBufferIndirectCommandNV {
    pub bufferAddress:  VkDeviceAddress,
    pub size:  u32,
    pub stride:  u32,
}

#[repr(C)]
pub struct VkSetStateFlagsIndirectCommandNV {
    pub data:  u32,
}

#[repr(C)]
pub struct VkIndirectCommandsStreamNV {
    pub buffer:  VkBuffer,
    pub offset:  VkDeviceSize,
}

#[repr(C)]
pub struct VkIndirectCommandsLayoutTokenNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub tokenType:  VkIndirectCommandsTokenTypeNV,
    pub stream:  u32,
    pub offset:  u32,
    pub vertexBindingUnit:  u32,
    pub vertexDynamicStride:  VkBool32,
    pub pushconstantPipelineLayout:  VkPipelineLayout,
    pub pushconstantShaderStageFlags:  VkShaderStageFlags,
    pub pushconstantOffset:  u32,
    pub pushconstantSize:  u32,
    pub indirectStateFlags:  VkIndirectStateFlagsNV,
    pub indexTypeCount:  u32,
    pub pIndexTypes: *const  VkIndexType,
    pub pIndexTypeValues: *const  u32,
}

#[repr(C)]
pub struct VkIndirectCommandsLayoutCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkIndirectCommandsLayoutUsageFlagsNV,
    pub pipelineBindPoint:  VkPipelineBindPoint,
    pub tokenCount:  u32,
    pub pTokens: *const  VkIndirectCommandsLayoutTokenNV,
    pub streamCount:  u32,
    pub pStreamStrides: *const  u32,
}

#[repr(C)]
pub struct VkGeneratedCommandsInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pipelineBindPoint:  VkPipelineBindPoint,
    pub pipeline:  VkPipeline,
    pub indirectCommandsLayout:  VkIndirectCommandsLayoutNV,
    pub streamCount:  u32,
    pub pStreams: *const  VkIndirectCommandsStreamNV,
    pub sequencesCount:  u32,
    pub preprocessBuffer:  VkBuffer,
    pub preprocessOffset:  VkDeviceSize,
    pub preprocessSize:  VkDeviceSize,
    pub sequencesCountBuffer:  VkBuffer,
    pub sequencesCountOffset:  VkDeviceSize,
    pub sequencesIndexBuffer:  VkBuffer,
    pub sequencesIndexOffset:  VkDeviceSize,
}

#[repr(C)]
pub struct VkGeneratedCommandsMemoryRequirementsInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pipelineBindPoint:  VkPipelineBindPoint,
    pub pipeline:  VkPipeline,
    pub indirectCommandsLayout:  VkIndirectCommandsLayoutNV,
    pub maxSequencesCount:  u32,
}

#[repr(C)]
pub struct VkPipelineIndirectDeviceAddressInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pipelineBindPoint:  VkPipelineBindPoint,
    pub pipeline:  VkPipeline,
}

#[repr(C)]
pub struct VkBindPipelineIndirectCommandNV {
    pub pipelineAddress:  VkDeviceAddress,
}

#[repr(C)]
pub struct VkPhysicalDeviceFeatures2 {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub features:  VkPhysicalDeviceFeatures,
}

#[repr(C)]
pub struct VkPhysicalDeviceFeatures2KHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceProperties2 {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub properties:  VkPhysicalDeviceProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceProperties2KHR {
}

#[repr(C)]
pub struct VkFormatProperties2 {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub formatProperties:  VkFormatProperties,
}

#[repr(C)]
pub struct VkFormatProperties2KHR {
}

#[repr(C)]
pub struct VkImageFormatProperties2 {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub imageFormatProperties:  VkImageFormatProperties,
}

#[repr(C)]
pub struct VkImageFormatProperties2KHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceImageFormatInfo2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub format:  VkFormat,
    pub r#type:  VkImageType,
    pub tiling:  VkImageTiling,
    pub usage:  VkImageUsageFlags,
    pub flags:  VkImageCreateFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageFormatInfo2KHR {
}

#[repr(C)]
pub struct VkQueueFamilyProperties2 {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub queueFamilyProperties:  VkQueueFamilyProperties,
}

#[repr(C)]
pub struct VkQueueFamilyProperties2KHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties2 {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub memoryProperties:  VkPhysicalDeviceMemoryProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties2KHR {
}

#[repr(C)]
pub struct VkSparseImageFormatProperties2 {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub properties:  VkSparseImageFormatProperties,
}

#[repr(C)]
pub struct VkSparseImageFormatProperties2KHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub format:  VkFormat,
    pub r#type:  VkImageType,
    pub samples:  VkSampleCountFlagBits,
    pub usage:  VkImageUsageFlags,
    pub tiling:  VkImageTiling,
}

#[repr(C)]
pub struct VkPhysicalDeviceSparseImageFormatInfo2KHR {
}

#[repr(C)]
pub struct VkPhysicalDevicePushDescriptorProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxPushDescriptors:  u32,
}

#[repr(C)]
pub struct VkPhysicalDevicePushDescriptorPropertiesKHR {
}

#[repr(C)]
pub struct VkConformanceVersion {
    pub major:  u8,
    pub minor:  u8,
    pub subminor:  u8,
    pub patch:  u8,
}

#[repr(C)]
pub struct VkConformanceVersionKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceDriverProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub driverID:  VkDriverId,
    pub driverName:  [i8; VK_MAX_DRIVER_NAME_SIZE as usize],
    pub driverInfo:  [i8; VK_MAX_DRIVER_INFO_SIZE as usize],
    pub conformanceVersion:  VkConformanceVersion,
}

#[repr(C)]
pub struct VkPhysicalDeviceDriverPropertiesKHR {
}

#[repr(C)]
pub struct VkPresentRegionsKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub swapchainCount:  u32,
    pub pRegions: *const  VkPresentRegionKHR,
}

#[repr(C)]
pub struct VkPresentRegionKHR {
    pub rectangleCount:  u32,
    pub pRectangles: *const  VkRectLayerKHR,
}

#[repr(C)]
pub struct VkRectLayerKHR {
    pub offset:  VkOffset2D,
    pub extent:  VkExtent2D,
    pub layer:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVariablePointersFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub variablePointersStorageBuffer:  VkBool32,
    pub variablePointers:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVariablePointersFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceVariablePointerFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceVariablePointerFeatures {
}

#[repr(C)]
pub struct VkExternalMemoryProperties {
    pub externalMemoryFeatures:  VkExternalMemoryFeatureFlags,
    pub exportFromImportedHandleTypes:  VkExternalMemoryHandleTypeFlags,
    pub compatibleHandleTypes:  VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct VkExternalMemoryPropertiesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalImageFormatInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleType:  VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalImageFormatInfoKHR {
}

#[repr(C)]
pub struct VkExternalImageFormatProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub externalMemoryProperties:  VkExternalMemoryProperties,
}

#[repr(C)]
pub struct VkExternalImageFormatPropertiesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalBufferInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkBufferCreateFlags,
    pub usage:  VkBufferUsageFlags,
    pub handleType:  VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalBufferInfoKHR {
}

#[repr(C)]
pub struct VkExternalBufferProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub externalMemoryProperties:  VkExternalMemoryProperties,
}

#[repr(C)]
pub struct VkExternalBufferPropertiesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceIDProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub deviceUUID:  [u8; VK_UUID_SIZE as usize],
    pub driverUUID:  [u8; VK_UUID_SIZE as usize],
    pub deviceLUID:  [u8; VK_LUID_SIZE as usize],
    pub deviceNodeMask:  u32,
    pub deviceLUIDValid:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceIDPropertiesKHR {
}

#[repr(C)]
pub struct VkExternalMemoryImageCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleTypes:  VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct VkExternalMemoryImageCreateInfoKHR {
}

#[repr(C)]
pub struct VkExternalMemoryBufferCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleTypes:  VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct VkExternalMemoryBufferCreateInfoKHR {
}

#[repr(C)]
pub struct VkExportMemoryAllocateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleTypes:  VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct VkExportMemoryAllocateInfoKHR {
}

#[repr(C)]
pub struct VkImportMemoryWin32HandleInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleType:  VkExternalMemoryHandleTypeFlagBits,
    pub handle:  HANDLE,
    pub name:  LPCWSTR,
}

#[repr(C)]
pub struct VkExportMemoryWin32HandleInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pAttributes: *const  SECURITY_ATTRIBUTES,
    pub dwAccess:  DWORD,
    pub name:  LPCWSTR,
}

#[repr(C)]
pub struct VkImportMemoryZirconHandleInfoFUCHSIA {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleType:  VkExternalMemoryHandleTypeFlagBits,
    pub handle:  zx_handle_t,
}

#[repr(C)]
pub struct VkMemoryZirconHandlePropertiesFUCHSIA {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub memoryTypeBits:  u32,
}

#[repr(C)]
pub struct VkMemoryGetZirconHandleInfoFUCHSIA {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub memory:  VkDeviceMemory,
    pub handleType:  VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkMemoryWin32HandlePropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub memoryTypeBits:  u32,
}

#[repr(C)]
pub struct VkMemoryGetWin32HandleInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub memory:  VkDeviceMemory,
    pub handleType:  VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkImportMemoryFdInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleType:  VkExternalMemoryHandleTypeFlagBits,
    pub fd:  i32,
}

#[repr(C)]
pub struct VkMemoryFdPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub memoryTypeBits:  u32,
}

#[repr(C)]
pub struct VkMemoryGetFdInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub memory:  VkDeviceMemory,
    pub handleType:  VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkWin32KeyedMutexAcquireReleaseInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub acquireCount:  u32,
    pub pAcquireSyncs: *const  VkDeviceMemory,
    pub pAcquireKeys: *const  u64,
    pub pAcquireTimeouts: *const  u32,
    pub releaseCount:  u32,
    pub pReleaseSyncs: *const  VkDeviceMemory,
    pub pReleaseKeys: *const  u64,
}

#[repr(C)]
pub struct VkImportMemoryMetalHandleInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleType:  VkExternalMemoryHandleTypeFlagBits,
    pub handle: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkMemoryMetalHandlePropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub memoryTypeBits:  u32,
}

#[repr(C)]
pub struct VkMemoryGetMetalHandleInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub memory:  VkDeviceMemory,
    pub handleType:  VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalSemaphoreInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleType:  VkExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalSemaphoreInfoKHR {
}

#[repr(C)]
pub struct VkExternalSemaphoreProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub exportFromImportedHandleTypes:  VkExternalSemaphoreHandleTypeFlags,
    pub compatibleHandleTypes:  VkExternalSemaphoreHandleTypeFlags,
    pub externalSemaphoreFeatures:  VkExternalSemaphoreFeatureFlags,
}

#[repr(C)]
pub struct VkExternalSemaphorePropertiesKHR {
}

#[repr(C)]
pub struct VkExportSemaphoreCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleTypes:  VkExternalSemaphoreHandleTypeFlags,
}

#[repr(C)]
pub struct VkExportSemaphoreCreateInfoKHR {
}

#[repr(C)]
pub struct VkImportSemaphoreWin32HandleInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub semaphore:  VkSemaphore,
    pub flags:  VkSemaphoreImportFlags,
    pub handleType:  VkExternalSemaphoreHandleTypeFlagBits,
    pub handle:  HANDLE,
    pub name:  LPCWSTR,
}

#[repr(C)]
pub struct VkExportSemaphoreWin32HandleInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pAttributes: *const  SECURITY_ATTRIBUTES,
    pub dwAccess:  DWORD,
    pub name:  LPCWSTR,
}

#[repr(C)]
pub struct VkD3D12FenceSubmitInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub waitSemaphoreValuesCount:  u32,
    pub pWaitSemaphoreValues: *const  u64,
    pub signalSemaphoreValuesCount:  u32,
    pub pSignalSemaphoreValues: *const  u64,
}

#[repr(C)]
pub struct VkSemaphoreGetWin32HandleInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub semaphore:  VkSemaphore,
    pub handleType:  VkExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkImportSemaphoreFdInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub semaphore:  VkSemaphore,
    pub flags:  VkSemaphoreImportFlags,
    pub handleType:  VkExternalSemaphoreHandleTypeFlagBits,
    pub fd:  i32,
}

#[repr(C)]
pub struct VkSemaphoreGetFdInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub semaphore:  VkSemaphore,
    pub handleType:  VkExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkImportSemaphoreZirconHandleInfoFUCHSIA {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub semaphore:  VkSemaphore,
    pub flags:  VkSemaphoreImportFlags,
    pub handleType:  VkExternalSemaphoreHandleTypeFlagBits,
    pub zirconHandle:  zx_handle_t,
}

#[repr(C)]
pub struct VkSemaphoreGetZirconHandleInfoFUCHSIA {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub semaphore:  VkSemaphore,
    pub handleType:  VkExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalFenceInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleType:  VkExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalFenceInfoKHR {
}

#[repr(C)]
pub struct VkExternalFenceProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub exportFromImportedHandleTypes:  VkExternalFenceHandleTypeFlags,
    pub compatibleHandleTypes:  VkExternalFenceHandleTypeFlags,
    pub externalFenceFeatures:  VkExternalFenceFeatureFlags,
}

#[repr(C)]
pub struct VkExternalFencePropertiesKHR {
}

#[repr(C)]
pub struct VkExportFenceCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleTypes:  VkExternalFenceHandleTypeFlags,
}

#[repr(C)]
pub struct VkExportFenceCreateInfoKHR {
}

#[repr(C)]
pub struct VkImportFenceWin32HandleInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub fence:  VkFence,
    pub flags:  VkFenceImportFlags,
    pub handleType:  VkExternalFenceHandleTypeFlagBits,
    pub handle:  HANDLE,
    pub name:  LPCWSTR,
}

#[repr(C)]
pub struct VkExportFenceWin32HandleInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pAttributes: *const  SECURITY_ATTRIBUTES,
    pub dwAccess:  DWORD,
    pub name:  LPCWSTR,
}

#[repr(C)]
pub struct VkFenceGetWin32HandleInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub fence:  VkFence,
    pub handleType:  VkExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkImportFenceFdInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub fence:  VkFence,
    pub flags:  VkFenceImportFlags,
    pub handleType:  VkExternalFenceHandleTypeFlagBits,
    pub fd:  i32,
}

#[repr(C)]
pub struct VkFenceGetFdInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub fence:  VkFence,
    pub handleType:  VkExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkExportFenceSciSyncInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pAttributes:  NvSciSyncAttrList,
}

#[repr(C)]
pub struct VkImportFenceSciSyncInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub fence:  VkFence,
    pub handleType:  VkExternalFenceHandleTypeFlagBits,
    pub handle: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkFenceGetSciSyncInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub fence:  VkFence,
    pub handleType:  VkExternalFenceHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkExportSemaphoreSciSyncInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pAttributes:  NvSciSyncAttrList,
}

#[repr(C)]
pub struct VkImportSemaphoreSciSyncInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub semaphore:  VkSemaphore,
    pub handleType:  VkExternalSemaphoreHandleTypeFlagBits,
    pub handle: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkSemaphoreGetSciSyncInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub semaphore:  VkSemaphore,
    pub handleType:  VkExternalSemaphoreHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkSciSyncAttributesInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub clientType:  VkSciSyncClientTypeNV,
    pub primitiveType:  VkSciSyncPrimitiveTypeNV,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalSciSyncFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub sciSyncFence:  VkBool32,
    pub sciSyncSemaphore:  VkBool32,
    pub sciSyncImport:  VkBool32,
    pub sciSyncExport:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalSciSync2FeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub sciSyncFence:  VkBool32,
    pub sciSyncSemaphore2:  VkBool32,
    pub sciSyncImport:  VkBool32,
    pub sciSyncExport:  VkBool32,
}

#[repr(C)]
pub struct VkSemaphoreSciSyncPoolCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handle:  NvSciSyncObj,
}

#[repr(C)]
pub struct VkSemaphoreSciSyncCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub semaphorePool:  VkSemaphoreSciSyncPoolNV,
    pub pFence: *const  NvSciSyncFence,
}

#[repr(C)]
pub struct VkDeviceSemaphoreSciSyncPoolReservationCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub semaphoreSciSyncPoolRequestCount:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub multiview:  VkBool32,
    pub multiviewGeometryShader:  VkBool32,
    pub multiviewTessellationShader:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxMultiviewViewCount:  u32,
    pub maxMultiviewInstanceIndex:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPropertiesKHR {
}

#[repr(C)]
pub struct VkRenderPassMultiviewCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub subpassCount:  u32,
    pub pViewMasks: *const  u32,
    pub dependencyCount:  u32,
    pub pViewOffsets: *const  i32,
    pub correlationMaskCount:  u32,
    pub pCorrelationMasks: *const  u32,
}

#[repr(C)]
pub struct VkRenderPassMultiviewCreateInfoKHR {
}

#[repr(C)]
pub struct VkSurfaceCapabilities2EXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub minImageCount:  u32,
    pub maxImageCount:  u32,
    pub currentExtent:  VkExtent2D,
    pub minImageExtent:  VkExtent2D,
    pub maxImageExtent:  VkExtent2D,
    pub maxImageArrayLayers:  u32,
    pub supportedTransforms:  VkSurfaceTransformFlagsKHR,
    pub currentTransform:  VkSurfaceTransformFlagBitsKHR,
    pub supportedCompositeAlpha:  VkCompositeAlphaFlagsKHR,
    pub supportedUsageFlags:  VkImageUsageFlags,
    pub supportedSurfaceCounters:  VkSurfaceCounterFlagsEXT,
}

#[repr(C)]
pub struct VkDisplayPowerInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub powerState:  VkDisplayPowerStateEXT,
}

#[repr(C)]
pub struct VkDeviceEventInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub deviceEvent:  VkDeviceEventTypeEXT,
}

#[repr(C)]
pub struct VkDisplayEventInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub displayEvent:  VkDisplayEventTypeEXT,
}

#[repr(C)]
pub struct VkSwapchainCounterCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub surfaceCounters:  VkSurfaceCounterFlagsEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceGroupProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub physicalDeviceCount:  u32,
    pub physicalDevices:  [VkPhysicalDevice; VK_MAX_DEVICE_GROUP_SIZE as usize],
    pub subsetAllocation:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceGroupPropertiesKHR {
}

#[repr(C)]
pub struct VkMemoryAllocateFlagsInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkMemoryAllocateFlags,
    pub deviceMask:  u32,
}

#[repr(C)]
pub struct VkMemoryAllocateFlagsInfoKHR {
}

#[repr(C)]
pub struct VkBindBufferMemoryInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub buffer:  VkBuffer,
    pub memory:  VkDeviceMemory,
    pub memoryOffset:  VkDeviceSize,
}

#[repr(C)]
pub struct VkBindBufferMemoryInfoKHR {
}

#[repr(C)]
pub struct VkBindBufferMemoryDeviceGroupInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub deviceIndexCount:  u32,
    pub pDeviceIndices: *const  u32,
}

#[repr(C)]
pub struct VkBindBufferMemoryDeviceGroupInfoKHR {
}

#[repr(C)]
pub struct VkBindImageMemoryInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub image:  VkImage,
    pub memory:  VkDeviceMemory,
    pub memoryOffset:  VkDeviceSize,
}

#[repr(C)]
pub struct VkBindImageMemoryInfoKHR {
}

#[repr(C)]
pub struct VkBindImageMemoryDeviceGroupInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub deviceIndexCount:  u32,
    pub pDeviceIndices: *const  u32,
    pub splitInstanceBindRegionCount:  u32,
    pub pSplitInstanceBindRegions: *const  VkRect2D,
}

#[repr(C)]
pub struct VkBindImageMemoryDeviceGroupInfoKHR {
}

#[repr(C)]
pub struct VkDeviceGroupRenderPassBeginInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub deviceMask:  u32,
    pub deviceRenderAreaCount:  u32,
    pub pDeviceRenderAreas: *const  VkRect2D,
}

#[repr(C)]
pub struct VkDeviceGroupRenderPassBeginInfoKHR {
}

#[repr(C)]
pub struct VkDeviceGroupCommandBufferBeginInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub deviceMask:  u32,
}

#[repr(C)]
pub struct VkDeviceGroupCommandBufferBeginInfoKHR {
}

#[repr(C)]
pub struct VkDeviceGroupSubmitInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub waitSemaphoreCount:  u32,
    pub pWaitSemaphoreDeviceIndices: *const  u32,
    pub commandBufferCount:  u32,
    pub pCommandBufferDeviceMasks: *const  u32,
    pub signalSemaphoreCount:  u32,
    pub pSignalSemaphoreDeviceIndices: *const  u32,
}

#[repr(C)]
pub struct VkDeviceGroupSubmitInfoKHR {
}

#[repr(C)]
pub struct VkDeviceGroupBindSparseInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub resourceDeviceIndex:  u32,
    pub memoryDeviceIndex:  u32,
}

#[repr(C)]
pub struct VkDeviceGroupBindSparseInfoKHR {
}

#[repr(C)]
pub struct VkDeviceGroupPresentCapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentMask:  [u32; VK_MAX_DEVICE_GROUP_SIZE as usize],
    pub modes:  VkDeviceGroupPresentModeFlagsKHR,
}

#[repr(C)]
pub struct VkImageSwapchainCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub swapchain:  VkSwapchainKHR,
}

#[repr(C)]
pub struct VkBindImageMemorySwapchainInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub swapchain:  VkSwapchainKHR,
    pub imageIndex:  u32,
}

#[repr(C)]
pub struct VkAcquireNextImageInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub swapchain:  VkSwapchainKHR,
    pub timeout:  u64,
    pub semaphore:  VkSemaphore,
    pub fence:  VkFence,
    pub deviceMask:  u32,
}

#[repr(C)]
pub struct VkDeviceGroupPresentInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub swapchainCount:  u32,
    pub pDeviceMasks: *const  u32,
    pub mode:  VkDeviceGroupPresentModeFlagBitsKHR,
}

#[repr(C)]
pub struct VkDeviceGroupDeviceCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub physicalDeviceCount:  u32,
    pub pPhysicalDevices: *const  VkPhysicalDevice,
}

#[repr(C)]
pub struct VkDeviceGroupDeviceCreateInfoKHR {
}

#[repr(C)]
pub struct VkDeviceGroupSwapchainCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub modes:  VkDeviceGroupPresentModeFlagsKHR,
}

#[repr(C)]
pub struct VkDescriptorUpdateTemplateEntry {
    pub dstBinding:  u32,
    pub dstArrayElement:  u32,
    pub descriptorCount:  u32,
    pub descriptorType:  VkDescriptorType,
    pub offset:  usize,
    pub stride:  usize,
}

#[repr(C)]
pub struct VkDescriptorUpdateTemplateEntryKHR {
}

#[repr(C)]
pub struct VkDescriptorUpdateTemplateCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkDescriptorUpdateTemplateCreateFlags,
    pub descriptorUpdateEntryCount:  u32,
    pub pDescriptorUpdateEntries: *const  VkDescriptorUpdateTemplateEntry,
    pub templateType:  VkDescriptorUpdateTemplateType,
    pub descriptorSetLayout:  VkDescriptorSetLayout,
    pub pipelineBindPoint:  VkPipelineBindPoint,
    pub pipelineLayout:  VkPipelineLayout,
    pub set:  u32,
}

#[repr(C)]
pub struct VkDescriptorUpdateTemplateCreateInfoKHR {
}

#[repr(C)]
pub struct VkXYColorEXT {
    pub x:  f32,
    pub y:  f32,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentIdFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentId:  VkBool32,
}

#[repr(C)]
pub struct VkPresentIdKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub swapchainCount:  u32,
    pub pPresentIds: *const  u64,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentId2FeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentId2:  VkBool32,
}

#[repr(C)]
pub struct VkPresentId2KHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub swapchainCount:  u32,
    pub pPresentIds: *const  u64,
}

#[repr(C)]
pub struct VkPresentWait2InfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub presentId:  u64,
    pub timeout:  u64,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentWaitFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentWait:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentWait2FeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentWait2:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentTimingFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentTiming:  VkBool32,
    pub presentAtAbsoluteTime:  VkBool32,
    pub presentAtRelativeTime:  VkBool32,
}

#[repr(C)]
pub struct VkPresentTimingSurfaceCapabilitiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentTimingSupported:  VkBool32,
    pub presentAtAbsoluteTimeSupported:  VkBool32,
    pub presentAtRelativeTimeSupported:  VkBool32,
    pub presentStageQueries:  VkPresentStageFlagsEXT,
}

#[repr(C)]
pub struct VkSwapchainTimingPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub refreshDuration:  u64,
    pub refreshInterval:  u64,
}

#[repr(C)]
pub struct VkSwapchainTimeDomainPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub timeDomainCount:  u32,
    pub pTimeDomains: *mut  VkTimeDomainKHR,
    pub pTimeDomainIds: *mut  u64,
}

#[repr(C)]
pub struct VkPresentStageTimeEXT {
    pub stage:  VkPresentStageFlagsEXT,
    pub time:  u64,
}

#[repr(C)]
pub struct VkPastPresentationTimingInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPastPresentationTimingFlagsEXT,
    pub swapchain:  VkSwapchainKHR,
}

#[repr(C)]
pub struct VkPastPresentationTimingPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub timingPropertiesCounter:  u64,
    pub timeDomainsCounter:  u64,
    pub presentationTimingCount:  u32,
    pub pPresentationTimings: *mut  VkPastPresentationTimingEXT,
}

#[repr(C)]
pub struct VkPastPresentationTimingEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentId:  u64,
    pub targetTime:  u64,
    pub presentStageCount:  u32,
    pub pPresentStages: *mut  VkPresentStageTimeEXT,
    pub timeDomain:  VkTimeDomainKHR,
    pub timeDomainId:  u64,
    pub reportComplete:  VkBool32,
}

#[repr(C)]
pub struct VkPresentTimingsInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub swapchainCount:  u32,
    pub pTimingInfos: *const  VkPresentTimingInfoEXT,
}

#[repr(C)]
pub struct VkPresentTimingInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPresentTimingInfoFlagsEXT,
    pub targetTime:  u64,
    pub timeDomainId:  u64,
    pub presentStageQueries:  VkPresentStageFlagsEXT,
    pub targetTimeDomainPresentStage:  VkPresentStageFlagsEXT,
}

#[repr(C)]
pub struct VkSwapchainCalibratedTimestampInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub swapchain:  VkSwapchainKHR,
    pub presentStage:  VkPresentStageFlagsEXT,
    pub timeDomainId:  u64,
}

#[repr(C)]
pub struct VkHdrMetadataEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub displayPrimaryRed:  VkXYColorEXT,
    pub displayPrimaryGreen:  VkXYColorEXT,
    pub displayPrimaryBlue:  VkXYColorEXT,
    pub whitePoint:  VkXYColorEXT,
    pub maxLuminance:  f32,
    pub minLuminance:  f32,
    pub maxContentLightLevel:  f32,
    pub maxFrameAverageLightLevel:  f32,
}

#[repr(C)]
pub struct VkHdrVividDynamicMetadataHUAWEI {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub dynamicMetadataSize:  usize,
    pub pDynamicMetadata: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkDisplayNativeHdrSurfaceCapabilitiesAMD {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub localDimmingSupport:  VkBool32,
}

#[repr(C)]
pub struct VkSwapchainDisplayNativeHdrCreateInfoAMD {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub localDimmingEnable:  VkBool32,
}

#[repr(C)]
pub struct VkRefreshCycleDurationGOOGLE {
    pub refreshDuration:  u64,
}

#[repr(C)]
pub struct VkPastPresentationTimingGOOGLE {
    pub presentID:  u32,
    pub desiredPresentTime:  u64,
    pub actualPresentTime:  u64,
    pub earliestPresentTime:  u64,
    pub presentMargin:  u64,
}

#[repr(C)]
pub struct VkPresentTimesInfoGOOGLE {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub swapchainCount:  u32,
    pub pTimes: *const  VkPresentTimeGOOGLE,
}

#[repr(C)]
pub struct VkPresentTimeGOOGLE {
    pub presentID:  u32,
    pub desiredPresentTime:  u64,
}

#[repr(C)]
pub struct VkIOSSurfaceCreateInfoMVK {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkIOSSurfaceCreateFlagsMVK,
    pub pView: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkMacOSSurfaceCreateInfoMVK {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkMacOSSurfaceCreateFlagsMVK,
    pub pView: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkMetalSurfaceCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkMetalSurfaceCreateFlagsEXT,
    pub pLayer: *const  CAMetalLayer,
}

#[repr(C)]
pub struct VkViewportWScalingNV {
    pub xcoeff:  f32,
    pub ycoeff:  f32,
}

#[repr(C)]
pub struct VkPipelineViewportWScalingStateCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub viewportWScalingEnable:  VkBool32,
    pub viewportCount:  u32,
    pub pViewportWScalings: *const  VkViewportWScalingNV,
}

#[repr(C)]
pub struct VkViewportSwizzleNV {
    pub x:  VkViewportCoordinateSwizzleNV,
    pub y:  VkViewportCoordinateSwizzleNV,
    pub z:  VkViewportCoordinateSwizzleNV,
    pub w:  VkViewportCoordinateSwizzleNV,
}

#[repr(C)]
pub struct VkPipelineViewportSwizzleStateCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineViewportSwizzleStateCreateFlagsNV,
    pub viewportCount:  u32,
    pub pViewportSwizzles: *const  VkViewportSwizzleNV,
}

#[repr(C)]
pub struct VkPhysicalDeviceDiscardRectanglePropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxDiscardRectangles:  u32,
}

#[repr(C)]
pub struct VkPipelineDiscardRectangleStateCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineDiscardRectangleStateCreateFlagsEXT,
    pub discardRectangleMode:  VkDiscardRectangleModeEXT,
    pub discardRectangleCount:  u32,
    pub pDiscardRectangles: *const  VkRect2D,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewAttributesPropertiesNVX {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub perViewPositionAllComponents:  VkBool32,
}

#[repr(C)]
pub struct VkInputAttachmentAspectReference {
    pub subpass:  u32,
    pub inputAttachmentIndex:  u32,
    pub aspectMask:  VkImageAspectFlags,
}

#[repr(C)]
pub struct VkInputAttachmentAspectReferenceKHR {
}

#[repr(C)]
pub struct VkRenderPassInputAttachmentAspectCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub aspectReferenceCount:  u32,
    pub pAspectReferences: *const  VkInputAttachmentAspectReference,
}

#[repr(C)]
pub struct VkRenderPassInputAttachmentAspectCreateInfoKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceSurfaceInfo2KHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub surface:  VkSurfaceKHR,
}

#[repr(C)]
pub struct VkSurfaceCapabilities2KHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub surfaceCapabilities:  VkSurfaceCapabilitiesKHR,
}

#[repr(C)]
pub struct VkSurfaceFormat2KHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub surfaceFormat:  VkSurfaceFormatKHR,
}

#[repr(C)]
pub struct VkDisplayProperties2KHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub displayProperties:  VkDisplayPropertiesKHR,
}

#[repr(C)]
pub struct VkDisplayPlaneProperties2KHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub displayPlaneProperties:  VkDisplayPlanePropertiesKHR,
}

#[repr(C)]
pub struct VkDisplayModeProperties2KHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub displayModeProperties:  VkDisplayModePropertiesKHR,
}

#[repr(C)]
pub struct VkDisplayModeStereoPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub hdmi3DSupported:  VkBool32,
}

#[repr(C)]
pub struct VkDisplayPlaneInfo2KHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub mode:  VkDisplayModeKHR,
    pub planeIndex:  u32,
}

#[repr(C)]
pub struct VkDisplayPlaneCapabilities2KHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub capabilities:  VkDisplayPlaneCapabilitiesKHR,
}

#[repr(C)]
pub struct VkSharedPresentSurfaceCapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub sharedPresentSupportedUsageFlags:  VkImageUsageFlags,
}

#[repr(C)]
pub struct VkSwapchainFlagsSurfaceCapabilitiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub swapchainSupportedFlags:  VkSwapchainCreateFlagsKHR,
}

#[repr(C)]
pub struct VkSharedPresentSurfaceCapabilities2KHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub sharedPresentSupportedUsageFlags:  VkImageUsageFlags2KHR,
}

#[repr(C)]
pub struct VkPhysicalDevice16BitStorageFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub storageBuffer16BitAccess:  VkBool32,
    pub uniformAndStorageBuffer16BitAccess:  VkBool32,
    pub storagePushConstant16:  VkBool32,
    pub storageInputOutput16:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevice16BitStorageFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceSubgroupProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub subgroupSize:  u32,
    pub supportedStages:  VkShaderStageFlags,
    pub supportedOperations:  VkSubgroupFeatureFlags,
    pub quadOperationsInAllStages:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderSubgroupExtendedTypes:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupExtendedTypesFeaturesKHR {
}

#[repr(C)]
pub struct VkBufferMemoryRequirementsInfo2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub buffer:  VkBuffer,
}

#[repr(C)]
pub struct VkBufferMemoryRequirementsInfo2KHR {
}

#[repr(C)]
pub struct VkDeviceBufferMemoryRequirements {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pCreateInfo: *const  VkBufferCreateInfo,
}

#[repr(C)]
pub struct VkDeviceBufferMemoryRequirementsKHR {
}

#[repr(C)]
pub struct VkImageMemoryRequirementsInfo2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub image:  VkImage,
}

#[repr(C)]
pub struct VkImageMemoryRequirementsInfo2KHR {
}

#[repr(C)]
pub struct VkImageSparseMemoryRequirementsInfo2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub image:  VkImage,
}

#[repr(C)]
pub struct VkImageSparseMemoryRequirementsInfo2KHR {
}

#[repr(C)]
pub struct VkDeviceImageMemoryRequirements {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pCreateInfo: *const  VkImageCreateInfo,
    pub planeAspect:  VkImageAspectFlagBits,
}

#[repr(C)]
pub struct VkDeviceImageMemoryRequirementsKHR {
}

#[repr(C)]
pub struct VkMemoryRequirements2 {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub memoryRequirements:  VkMemoryRequirements,
}

#[repr(C)]
pub struct VkMemoryRequirements2KHR {
}

#[repr(C)]
pub struct VkSparseImageMemoryRequirements2 {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub memoryRequirements:  VkSparseImageMemoryRequirements,
}

#[repr(C)]
pub struct VkSparseImageMemoryRequirements2KHR {
}

#[repr(C)]
pub struct VkPhysicalDevicePointClippingProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pointClippingBehavior:  VkPointClippingBehavior,
}

#[repr(C)]
pub struct VkPhysicalDevicePointClippingPropertiesKHR {
}

#[repr(C)]
pub struct VkMemoryDedicatedRequirements {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub prefersDedicatedAllocation:  VkBool32,
    pub requiresDedicatedAllocation:  VkBool32,
}

#[repr(C)]
pub struct VkMemoryDedicatedRequirementsKHR {
}

#[repr(C)]
pub struct VkMemoryDedicatedAllocateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub image:  VkImage,
    pub buffer:  VkBuffer,
}

#[repr(C)]
pub struct VkMemoryDedicatedAllocateInfoKHR {
}

#[repr(C)]
pub struct VkImageViewUsageCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub usage:  VkImageUsageFlags,
}

#[repr(C)]
pub struct VkImageViewUsage2CreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub usage:  VkImageUsageFlags2KHR,
}

#[repr(C)]
pub struct VkImageViewSlicedCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub sliceOffset:  u32,
    pub sliceCount:  u32,
}

#[repr(C)]
pub struct VkImageViewUsageCreateInfoKHR {
}

#[repr(C)]
pub struct VkPipelineTessellationDomainOriginStateCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub domainOrigin:  VkTessellationDomainOrigin,
}

#[repr(C)]
pub struct VkPipelineTessellationDomainOriginStateCreateInfoKHR {
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub conversion:  VkSamplerYcbcrConversion,
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionInfoKHR {
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub format:  VkFormat,
    pub ycbcrModel:  VkSamplerYcbcrModelConversion,
    pub ycbcrRange:  VkSamplerYcbcrRange,
    pub components:  VkComponentMapping,
    pub xChromaOffset:  VkChromaLocation,
    pub yChromaOffset:  VkChromaLocation,
    pub chromaFilter:  VkFilter,
    pub forceExplicitReconstruction:  VkBool32,
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionCreateInfoKHR {
}

#[repr(C)]
pub struct VkBindImagePlaneMemoryInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub planeAspect:  VkImageAspectFlagBits,
}

#[repr(C)]
pub struct VkBindImagePlaneMemoryInfoKHR {
}

#[repr(C)]
pub struct VkImagePlaneMemoryRequirementsInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub planeAspect:  VkImageAspectFlagBits,
}

#[repr(C)]
pub struct VkImagePlaneMemoryRequirementsInfoKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub samplerYcbcrConversion:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSamplerYcbcrConversionFeaturesKHR {
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionImageFormatProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub combinedImageSamplerDescriptorCount:  u32,
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionImageFormatPropertiesKHR {
}

#[repr(C)]
pub struct VkTextureLODGatherFormatPropertiesAMD {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub supportsTextureGatherLODBiasAMD:  VkBool32,
}

#[repr(C)]
pub struct VkConditionalRenderingBeginInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub buffer:  VkBuffer,
    pub offset:  VkDeviceSize,
    pub flags:  VkConditionalRenderingFlagsEXT,
}

#[repr(C)]
pub struct VkProtectedSubmitInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub protectedSubmit:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceProtectedMemoryFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub protectedMemory:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceProtectedMemoryProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub protectedNoFault:  VkBool32,
}

#[repr(C)]
pub struct VkDeviceQueueInfo2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkDeviceQueueCreateFlags,
    pub queueFamilyIndex:  u32,
    pub queueIndex:  u32,
}

#[repr(C)]
pub struct VkPipelineCoverageToColorStateCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineCoverageToColorStateCreateFlagsNV,
    pub coverageToColorEnable:  VkBool32,
    pub coverageToColorLocation:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub filterMinmaxSingleComponentFormats:  VkBool32,
    pub filterMinmaxImageComponentMapping:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSamplerFilterMinmaxPropertiesEXT {
}

#[repr(C)]
pub struct VkSampleLocationEXT {
    pub x:  f32,
    pub y:  f32,
}

#[repr(C)]
pub struct VkSampleLocationsInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub sampleLocationsPerPixel:  VkSampleCountFlagBits,
    pub sampleLocationGridSize:  VkExtent2D,
    pub sampleLocationsCount:  u32,
    pub pSampleLocations: *const  VkSampleLocationEXT,
}

#[repr(C)]
pub struct VkAttachmentSampleLocationsEXT {
    pub attachmentIndex:  u32,
    pub sampleLocationsInfo:  VkSampleLocationsInfoEXT,
}

#[repr(C)]
pub struct VkSubpassSampleLocationsEXT {
    pub subpassIndex:  u32,
    pub sampleLocationsInfo:  VkSampleLocationsInfoEXT,
}

#[repr(C)]
pub struct VkRenderPassSampleLocationsBeginInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub attachmentInitialSampleLocationsCount:  u32,
    pub pAttachmentInitialSampleLocations: *const  VkAttachmentSampleLocationsEXT,
    pub postSubpassSampleLocationsCount:  u32,
    pub pPostSubpassSampleLocations: *const  VkSubpassSampleLocationsEXT,
}

#[repr(C)]
pub struct VkPipelineSampleLocationsStateCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub sampleLocationsEnable:  VkBool32,
    pub sampleLocationsInfo:  VkSampleLocationsInfoEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceSampleLocationsPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub sampleLocationSampleCounts:  VkSampleCountFlags,
    pub maxSampleLocationGridSize:  VkExtent2D,
    pub sampleLocationCoordinateRange:  f32,
    pub sampleLocationSubPixelBits:  u32,
    pub variableSampleLocations:  VkBool32,
}

#[repr(C)]
pub struct VkMultisamplePropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxSampleLocationGridSize:  VkExtent2D,
}

#[repr(C)]
pub struct VkSamplerReductionModeCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub reductionMode:  VkSamplerReductionMode,
}

#[repr(C)]
pub struct VkSamplerReductionModeCreateInfoEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceBlendOperationAdvancedFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub advancedBlendCoherentOperations:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiDrawFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub multiDraw:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceBlendOperationAdvancedPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub advancedBlendMaxColorAttachments:  u32,
    pub advancedBlendIndependentBlend:  VkBool32,
    pub advancedBlendNonPremultipliedSrcColor:  VkBool32,
    pub advancedBlendNonPremultipliedDstColor:  VkBool32,
    pub advancedBlendCorrelatedOverlap:  VkBool32,
    pub advancedBlendAllOperations:  VkBool32,
}

#[repr(C)]
pub struct VkPipelineColorBlendAdvancedStateCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcPremultiplied:  VkBool32,
    pub dstPremultiplied:  VkBool32,
    pub blendOverlap:  VkBlendOverlapEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceInlineUniformBlockFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub inlineUniformBlock:  VkBool32,
    pub descriptorBindingInlineUniformBlockUpdateAfterBind:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceInlineUniformBlockFeaturesEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceInlineUniformBlockProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxInlineUniformBlockSize:  u32,
    pub maxPerStageDescriptorInlineUniformBlocks:  u32,
    pub maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks:  u32,
    pub maxDescriptorSetInlineUniformBlocks:  u32,
    pub maxDescriptorSetUpdateAfterBindInlineUniformBlocks:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceInlineUniformBlockPropertiesEXT {
}

#[repr(C)]
pub struct VkWriteDescriptorSetInlineUniformBlock {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub dataSize:  u32,
    pub pData: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkWriteDescriptorSetInlineUniformBlockEXT {
}

#[repr(C)]
pub struct VkDescriptorPoolInlineUniformBlockCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub maxInlineUniformBlockBindings:  u32,
}

#[repr(C)]
pub struct VkDescriptorPoolInlineUniformBlockCreateInfoEXT {
}

#[repr(C)]
pub struct VkPipelineCoverageModulationStateCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineCoverageModulationStateCreateFlagsNV,
    pub coverageModulationMode:  VkCoverageModulationModeNV,
    pub coverageModulationTableEnable:  VkBool32,
    pub coverageModulationTableCount:  u32,
    pub pCoverageModulationTable: *const  f32,
}

#[repr(C)]
pub struct VkImageFormatListCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub viewFormatCount:  u32,
    pub pViewFormats: *const  VkFormat,
}

#[repr(C)]
pub struct VkImageFormatListCreateInfoKHR {
}

#[repr(C)]
pub struct VkValidationCacheCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkValidationCacheCreateFlagsEXT,
    pub initialDataSize:  usize,
    pub pInitialData: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkShaderModuleValidationCacheCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub validationCache:  VkValidationCacheEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance3Properties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxPerSetDescriptors:  u32,
    pub maxMemoryAllocationSize:  VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance3PropertiesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance4Features {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maintenance4:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance4FeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance4Properties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxBufferSize:  VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance4PropertiesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance5Features {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maintenance5:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance5FeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance5Properties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub earlyFragmentMultisampleCoverageAfterSampleCounting:  VkBool32,
    pub earlyFragmentSampleMaskTestBeforeSampleCounting:  VkBool32,
    pub depthStencilSwizzleOneSupport:  VkBool32,
    pub polygonModePointSize:  VkBool32,
    pub nonStrictSinglePixelWideLinesUseParallelogram:  VkBool32,
    pub nonStrictWideLinesUseParallelogram:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance5PropertiesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance6Features {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maintenance6:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance6FeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance6Properties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub blockTexelViewCompatibleMultipleLayers:  VkBool32,
    pub maxCombinedImageSamplerDescriptorCount:  u32,
    pub fragmentShadingRateClampCombinerInputs:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance6PropertiesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance7FeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maintenance7:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance7PropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub robustFragmentShadingRateAttachmentAccess:  VkBool32,
    pub separateDepthStencilAttachmentAccess:  VkBool32,
    pub maxDescriptorSetTotalUniformBuffersDynamic:  u32,
    pub maxDescriptorSetTotalStorageBuffersDynamic:  u32,
    pub maxDescriptorSetTotalBuffersDynamic:  u32,
    pub maxDescriptorSetUpdateAfterBindTotalUniformBuffersDynamic:  u32,
    pub maxDescriptorSetUpdateAfterBindTotalStorageBuffersDynamic:  u32,
    pub maxDescriptorSetUpdateAfterBindTotalBuffersDynamic:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceLayeredApiPropertiesListKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub layeredApiCount:  u32,
    pub pLayeredApis: *mut  VkPhysicalDeviceLayeredApiPropertiesKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceLayeredApiPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub vendorID:  u32,
    pub deviceID:  u32,
    pub layeredAPI:  VkPhysicalDeviceLayeredApiKHR,
    pub deviceName:  [i8; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
}

#[repr(C)]
pub struct VkPhysicalDeviceLayeredApiVulkanPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub properties:  VkPhysicalDeviceProperties2,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance8FeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maintenance8:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance9FeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maintenance9:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance9PropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub image2DViewOf3DSparse:  VkBool32,
    pub defaultVertexAttributeValue:  VkDefaultVertexAttributeValueKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance11FeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maintenance11:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance10PropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub rgba4OpaqueBlackSwizzled:  VkBool32,
    pub resolveSrgbFormatAppliesTransferFunction:  VkBool32,
    pub resolveSrgbFormatSupportsTransferFunctionControl:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMaintenance10FeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maintenance10:  VkBool32,
}

#[repr(C)]
pub struct VkQueueFamilyOwnershipTransferPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub optimalImageTransferToQueueFamilies:  u32,
}

#[repr(C)]
pub struct VkQueueFamilyOptimalImageTransferGranularityPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub optimalImageTransferGranularity:  VkExtent3D,
}

#[repr(C)]
pub struct VkRenderingAreaInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub viewMask:  u32,
    pub colorAttachmentCount:  u32,
    pub pColorAttachmentFormats: *const  VkFormat,
    pub depthAttachmentFormat:  VkFormat,
    pub stencilAttachmentFormat:  VkFormat,
}

#[repr(C)]
pub struct VkRenderingAreaInfoKHR {
}

#[repr(C)]
pub struct VkDescriptorSetLayoutSupport {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub supported:  VkBool32,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutSupportKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderDrawParametersFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderDrawParameters:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderDrawParameterFeatures {
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderFloat16Int8Features {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderFloat16:  VkBool32,
    pub shaderInt8:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderFloat16Int8FeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceFloat16Int8FeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceFloatControlsProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub denormBehaviorIndependence:  VkShaderFloatControlsIndependence,
    pub roundingModeIndependence:  VkShaderFloatControlsIndependence,
    pub shaderSignedZeroInfNanPreserveFloat16:  VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat32:  VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat64:  VkBool32,
    pub shaderDenormPreserveFloat16:  VkBool32,
    pub shaderDenormPreserveFloat32:  VkBool32,
    pub shaderDenormPreserveFloat64:  VkBool32,
    pub shaderDenormFlushToZeroFloat16:  VkBool32,
    pub shaderDenormFlushToZeroFloat32:  VkBool32,
    pub shaderDenormFlushToZeroFloat64:  VkBool32,
    pub shaderRoundingModeRTEFloat16:  VkBool32,
    pub shaderRoundingModeRTEFloat32:  VkBool32,
    pub shaderRoundingModeRTEFloat64:  VkBool32,
    pub shaderRoundingModeRTZFloat16:  VkBool32,
    pub shaderRoundingModeRTZFloat32:  VkBool32,
    pub shaderRoundingModeRTZFloat64:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFloatControlsPropertiesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceHostQueryResetFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub hostQueryReset:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceHostQueryResetFeaturesEXT {
}

#[repr(C)]
pub struct VkNativeBufferUsage2ANDROID {
    pub consumer:  u64,
    pub producer:  u64,
}

#[repr(C)]
pub struct VkNativeBufferANDROID {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handle: *const  std::ffi::c_void,
    pub stride:  i32,
    pub format:  i32,
    pub usage:  i32,
    pub usage2:  VkNativeBufferUsage2ANDROID,
}

#[repr(C)]
pub struct VkSwapchainImageCreateInfoANDROID {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub usage:  VkSwapchainImageUsageFlagsANDROID,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentationPropertiesANDROID {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub sharedImage:  VkBool32,
}

#[repr(C)]
pub struct VkShaderResourceUsageAMD {
    pub numUsedVgprs:  u32,
    pub numUsedSgprs:  u32,
    pub ldsSizePerLocalWorkGroup:  u32,
    pub ldsUsageSizeInBytes:  usize,
    pub scratchMemUsageInBytes:  usize,
}

#[repr(C)]
pub struct VkShaderStatisticsInfoAMD {
    pub shaderStageMask:  VkShaderStageFlags,
    pub resourceUsage:  VkShaderResourceUsageAMD,
    pub numPhysicalVgprs:  u32,
    pub numPhysicalSgprs:  u32,
    pub numAvailableVgprs:  u32,
    pub numAvailableSgprs:  u32,
    pub computeWorkGroupSize:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceElapsedTimerQueryFeaturesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub elapsedTimerQuery:  VkBool32,
}

#[repr(C)]
pub struct VkDeviceQueueGlobalPriorityCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub globalPriority:  VkQueueGlobalPriority,
}

#[repr(C)]
pub struct VkDeviceQueueGlobalPriorityCreateInfoKHR {
}

#[repr(C)]
pub struct VkDeviceQueueGlobalPriorityCreateInfoEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceGlobalPriorityQueryFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub globalPriorityQuery:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceGlobalPriorityQueryFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceGlobalPriorityQueryFeaturesEXT {
}

#[repr(C)]
pub struct VkQueueFamilyGlobalPriorityProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub priorityCount:  u32,
    pub priorities:  [VkQueueGlobalPriority; VK_MAX_GLOBAL_PRIORITY_SIZE as usize],
}

#[repr(C)]
pub struct VkQueueFamilyGlobalPriorityPropertiesKHR {
}

#[repr(C)]
pub struct VkQueueFamilyGlobalPriorityPropertiesEXT {
}

#[repr(C)]
pub struct VkDebugUtilsObjectNameInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub objectType:  VkObjectType,
    pub objectHandle:  u64,
    pub pObjectName: *const  i8,
}

#[repr(C)]
pub struct VkDebugUtilsObjectTagInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub objectType:  VkObjectType,
    pub objectHandle:  u64,
    pub tagName:  u64,
    pub tagSize:  usize,
    pub pTag: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkDebugUtilsLabelEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pLabelName: *const  i8,
    pub color:  f32,
}

#[repr(C)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkDebugUtilsMessengerCreateFlagsEXT,
    pub messageSeverity:  VkDebugUtilsMessageSeverityFlagsEXT,
    pub messageType:  VkDebugUtilsMessageTypeFlagsEXT,
    pub pfnUserCallback:  crate::svk_commands::PFN_vkDebugUtilsMessengerCallbackEXT,
    pub pUserData: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkDebugUtilsMessengerCallbackDataFlagsEXT,
    pub pMessageIdName: *const  i8,
    pub messageIdNumber:  i32,
    pub pMessage: *const  i8,
    pub queueLabelCount:  u32,
    pub pQueueLabels: *const  VkDebugUtilsLabelEXT,
    pub cmdBufLabelCount:  u32,
    pub pCmdBufLabels: *const  VkDebugUtilsLabelEXT,
    pub objectCount:  u32,
    pub pObjects: *const  VkDebugUtilsObjectNameInfoEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceDeviceMemoryReportFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub deviceMemoryReport:  VkBool32,
}

#[repr(C)]
pub struct VkDeviceDeviceMemoryReportCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkDeviceMemoryReportFlagsEXT,
    //pub pfnUserCallback:  crate::svk_commands::PFN_vkDeviceMemoryReportCallbackEXT,
    pub pUserData: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkDeviceMemoryReportCallbackDataEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkDeviceMemoryReportFlagsEXT,
    pub r#type:  VkDeviceMemoryReportEventTypeEXT,
    pub memoryObjectId:  u64,
    pub size:  VkDeviceSize,
    pub objectType:  VkObjectType,
    pub objectHandle:  u64,
    pub heapIndex:  u32,
}

#[repr(C)]
pub struct VkImportMemoryHostPointerInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleType:  VkExternalMemoryHandleTypeFlagBits,
    pub pHostPointer: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkMemoryHostPointerPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub memoryTypeBits:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalMemoryHostPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub minImportedHostPointerAlignment:  VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceConservativeRasterizationPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub primitiveOverestimationSize:  f32,
    pub maxExtraPrimitiveOverestimationSize:  f32,
    pub extraPrimitiveOverestimationSizeGranularity:  f32,
    pub primitiveUnderestimation:  VkBool32,
    pub conservativePointAndLineRasterization:  VkBool32,
    pub degenerateTrianglesRasterized:  VkBool32,
    pub degenerateLinesRasterized:  VkBool32,
    pub fullyCoveredFragmentShaderInputVariable:  VkBool32,
    pub conservativeRasterizationPostDepthCoverage:  VkBool32,
}

#[repr(C)]
pub struct VkCalibratedTimestampInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub timeDomain:  VkTimeDomainKHR,
}

#[repr(C)]
pub struct VkCalibratedTimestampInfoEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderCorePropertiesAMD {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderEngineCount:  u32,
    pub shaderArraysPerEngineCount:  u32,
    pub computeUnitsPerShaderArray:  u32,
    pub simdPerComputeUnit:  u32,
    pub wavefrontsPerSimd:  u32,
    pub wavefrontSize:  u32,
    pub sgprsPerSimd:  u32,
    pub minSgprAllocation:  u32,
    pub maxSgprAllocation:  u32,
    pub sgprAllocationGranularity:  u32,
    pub vgprsPerSimd:  u32,
    pub minVgprAllocation:  u32,
    pub maxVgprAllocation:  u32,
    pub vgprAllocationGranularity:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreProperties2AMD {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderCoreFeatures:  VkShaderCorePropertiesFlagsAMD,
    pub activeComputeUnitCount:  u32,
}

#[repr(C)]
pub struct VkPipelineRasterizationConservativeStateCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineRasterizationConservativeStateCreateFlagsEXT,
    pub conservativeRasterizationMode:  VkConservativeRasterizationModeEXT,
    pub extraPrimitiveOverestimationSize:  f32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderInputAttachmentArrayDynamicIndexing:  VkBool32,
    pub shaderUniformTexelBufferArrayDynamicIndexing:  VkBool32,
    pub shaderStorageTexelBufferArrayDynamicIndexing:  VkBool32,
    pub shaderUniformBufferArrayNonUniformIndexing:  VkBool32,
    pub shaderSampledImageArrayNonUniformIndexing:  VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexing:  VkBool32,
    pub shaderStorageImageArrayNonUniformIndexing:  VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexing:  VkBool32,
    pub shaderUniformTexelBufferArrayNonUniformIndexing:  VkBool32,
    pub shaderStorageTexelBufferArrayNonUniformIndexing:  VkBool32,
    pub descriptorBindingUniformBufferUpdateAfterBind:  VkBool32,
    pub descriptorBindingSampledImageUpdateAfterBind:  VkBool32,
    pub descriptorBindingStorageImageUpdateAfterBind:  VkBool32,
    pub descriptorBindingStorageBufferUpdateAfterBind:  VkBool32,
    pub descriptorBindingUniformTexelBufferUpdateAfterBind:  VkBool32,
    pub descriptorBindingStorageTexelBufferUpdateAfterBind:  VkBool32,
    pub descriptorBindingUpdateUnusedWhilePending:  VkBool32,
    pub descriptorBindingPartiallyBound:  VkBool32,
    pub descriptorBindingVariableDescriptorCount:  VkBool32,
    pub runtimeDescriptorArray:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingFeaturesEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxUpdateAfterBindDescriptorsInAllPools:  u32,
    pub shaderUniformBufferArrayNonUniformIndexingNative:  VkBool32,
    pub shaderSampledImageArrayNonUniformIndexingNative:  VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexingNative:  VkBool32,
    pub shaderStorageImageArrayNonUniformIndexingNative:  VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexingNative:  VkBool32,
    pub robustBufferAccessUpdateAfterBind:  VkBool32,
    pub quadDivergentImplicitLod:  VkBool32,
    pub maxPerStageDescriptorUpdateAfterBindSamplers:  u32,
    pub maxPerStageDescriptorUpdateAfterBindUniformBuffers:  u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageBuffers:  u32,
    pub maxPerStageDescriptorUpdateAfterBindSampledImages:  u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageImages:  u32,
    pub maxPerStageDescriptorUpdateAfterBindInputAttachments:  u32,
    pub maxPerStageUpdateAfterBindResources:  u32,
    pub maxDescriptorSetUpdateAfterBindSamplers:  u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffers:  u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic:  u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffers:  u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic:  u32,
    pub maxDescriptorSetUpdateAfterBindSampledImages:  u32,
    pub maxDescriptorSetUpdateAfterBindStorageImages:  u32,
    pub maxDescriptorSetUpdateAfterBindInputAttachments:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorIndexingPropertiesEXT {
}

#[repr(C)]
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub bindingCount:  u32,
    pub pBindingFlags: *const  VkDescriptorBindingFlags,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutBindingFlagsCreateInfoEXT {
}

#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub descriptorSetCount:  u32,
    pub pDescriptorCounts: *const  u32,
}

#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountAllocateInfoEXT {
}

#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupport {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxVariableDescriptorCount:  u32,
}

#[repr(C)]
pub struct VkDescriptorSetVariableDescriptorCountLayoutSupportEXT {
}

#[repr(C)]
pub struct VkAttachmentDescription2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkAttachmentDescriptionFlags,
    pub format:  VkFormat,
    pub samples:  VkSampleCountFlagBits,
    pub loadOp:  VkAttachmentLoadOp,
    pub storeOp:  VkAttachmentStoreOp,
    pub stencilLoadOp:  VkAttachmentLoadOp,
    pub stencilStoreOp:  VkAttachmentStoreOp,
    pub initialLayout:  VkImageLayout,
    pub finalLayout:  VkImageLayout,
}

#[repr(C)]
pub struct VkAttachmentDescription2KHR {
}

#[repr(C)]
pub struct VkAttachmentReference2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub attachment:  u32,
    pub layout:  VkImageLayout,
    pub aspectMask:  VkImageAspectFlags,
}

#[repr(C)]
pub struct VkAttachmentReference2KHR {
}

#[repr(C)]
pub struct VkSubpassDescription2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkSubpassDescriptionFlags,
    pub pipelineBindPoint:  VkPipelineBindPoint,
    pub viewMask:  u32,
    pub inputAttachmentCount:  u32,
    pub pInputAttachments: *const  VkAttachmentReference2,
    pub colorAttachmentCount:  u32,
    pub pColorAttachments: *const  VkAttachmentReference2,
    pub pResolveAttachments: *const  VkAttachmentReference2,
    pub pDepthStencilAttachment: *const  VkAttachmentReference2,
    pub preserveAttachmentCount:  u32,
    pub pPreserveAttachments: *const  u32,
}

#[repr(C)]
pub struct VkSubpassDescription2KHR {
}

#[repr(C)]
pub struct VkSubpassDependency2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcSubpass:  u32,
    pub dstSubpass:  u32,
    pub srcStageMask:  VkPipelineStageFlags,
    pub dstStageMask:  VkPipelineStageFlags,
    pub srcAccessMask:  VkAccessFlags,
    pub dstAccessMask:  VkAccessFlags,
    pub dependencyFlags:  VkDependencyFlags,
    pub viewOffset:  i32,
}

#[repr(C)]
pub struct VkSubpassDependency2KHR {
}

#[repr(C)]
pub struct VkRenderPassCreateInfo2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkRenderPassCreateFlags,
    pub attachmentCount:  u32,
    pub pAttachments: *const  VkAttachmentDescription2,
    pub subpassCount:  u32,
    pub pSubpasses: *const  VkSubpassDescription2,
    pub dependencyCount:  u32,
    pub pDependencies: *const  VkSubpassDependency2,
    pub correlatedViewMaskCount:  u32,
    pub pCorrelatedViewMasks: *const  u32,
}

#[repr(C)]
pub struct VkRenderPassCreateInfo2KHR {
}

#[repr(C)]
pub struct VkSubpassBeginInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub contents:  VkSubpassContents,
}

#[repr(C)]
pub struct VkSubpassBeginInfoKHR {
}

#[repr(C)]
pub struct VkSubpassEndInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkSubpassEndInfoKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceTimelineSemaphoreFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub timelineSemaphore:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceTimelineSemaphoreFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceTimelineSemaphoreProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxTimelineSemaphoreValueDifference:  u64,
}

#[repr(C)]
pub struct VkPhysicalDeviceTimelineSemaphorePropertiesKHR {
}

#[repr(C)]
pub struct VkSemaphoreTypeCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub semaphoreType:  VkSemaphoreType,
    pub initialValue:  u64,
}

#[repr(C)]
pub struct VkSemaphoreTypeCreateInfoKHR {
}

#[repr(C)]
pub struct VkTimelineSemaphoreSubmitInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub waitSemaphoreValueCount:  u32,
    pub pWaitSemaphoreValues: *const  u64,
    pub signalSemaphoreValueCount:  u32,
    pub pSignalSemaphoreValues: *const  u64,
}

#[repr(C)]
pub struct VkTimelineSemaphoreSubmitInfoKHR {
}

#[repr(C)]
pub struct VkSemaphoreWaitInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkSemaphoreWaitFlags,
    pub semaphoreCount:  u32,
    pub pSemaphores: *const  VkSemaphore,
    pub pValues: *const  u64,
}

#[repr(C)]
pub struct VkSemaphoreWaitInfoKHR {
}

#[repr(C)]
pub struct VkSemaphoreSignalInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub semaphore:  VkSemaphore,
    pub value:  u64,
}

#[repr(C)]
pub struct VkSemaphoreSignalInfoKHR {
}

#[repr(C)]
pub struct VkVertexInputBindingDivisorDescription {
    pub binding:  u32,
    pub divisor:  u32,
}

#[repr(C)]
pub struct VkVertexInputBindingDivisorDescriptionKHR {
}

#[repr(C)]
pub struct VkVertexInputBindingDivisorDescriptionEXT {
}

#[repr(C)]
pub struct VkPipelineVertexInputDivisorStateCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub vertexBindingDivisorCount:  u32,
    pub pVertexBindingDivisors: *const  VkVertexInputBindingDivisorDescription,
}

#[repr(C)]
pub struct VkPipelineVertexInputDivisorStateCreateInfoKHR {
}

#[repr(C)]
pub struct VkPipelineVertexInputDivisorStateCreateInfoEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxVertexAttribDivisor:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeDivisorProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxVertexAttribDivisor:  u32,
    pub supportsNonZeroFirstInstance:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeDivisorPropertiesKHR {
}

#[repr(C)]
pub struct VkPhysicalDevicePCIBusInfoPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pciDomain:  u32,
    pub pciBus:  u32,
    pub pciDevice:  u32,
    pub pciFunction:  u32,
}

#[repr(C)]
pub struct VkImportAndroidHardwareBufferInfoANDROID {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub buffer: *mut  AHardwareBuffer,
}

#[repr(C)]
pub struct VkAndroidHardwareBufferUsageANDROID {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub androidHardwareBufferUsage:  u64,
}

#[repr(C)]
pub struct VkAndroidHardwareBufferPropertiesANDROID {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub allocationSize:  VkDeviceSize,
    pub memoryTypeBits:  u32,
}

#[repr(C)]
pub struct VkMemoryGetAndroidHardwareBufferInfoANDROID {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub memory:  VkDeviceMemory,
}

#[repr(C)]
pub struct VkAndroidHardwareBufferFormatPropertiesANDROID {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub format:  VkFormat,
    pub externalFormat:  u64,
    pub formatFeatures:  VkFormatFeatureFlags,
    pub samplerYcbcrConversionComponents:  VkComponentMapping,
    pub suggestedYcbcrModel:  VkSamplerYcbcrModelConversion,
    pub suggestedYcbcrRange:  VkSamplerYcbcrRange,
    pub suggestedXChromaOffset:  VkChromaLocation,
    pub suggestedYChromaOffset:  VkChromaLocation,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceConditionalRenderingInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub conditionalRenderingEnable:  VkBool32,
}

#[repr(C)]
pub struct VkExternalFormatANDROID {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub externalFormat:  u64,
}

#[repr(C)]
pub struct VkPhysicalDevice8BitStorageFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub storageBuffer8BitAccess:  VkBool32,
    pub uniformAndStorageBuffer8BitAccess:  VkBool32,
    pub storagePushConstant8:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevice8BitStorageFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceConditionalRenderingFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub conditionalRendering:  VkBool32,
    pub inheritedConditionalRendering:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkanMemoryModelFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub vulkanMemoryModel:  VkBool32,
    pub vulkanMemoryModelDeviceScope:  VkBool32,
    pub vulkanMemoryModelAvailabilityVisibilityChains:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkanMemoryModelFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicInt64Features {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderBufferInt64Atomics:  VkBool32,
    pub shaderSharedInt64Atomics:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicInt64FeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicFloatFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderBufferFloat32Atomics:  VkBool32,
    pub shaderBufferFloat32AtomicAdd:  VkBool32,
    pub shaderBufferFloat64Atomics:  VkBool32,
    pub shaderBufferFloat64AtomicAdd:  VkBool32,
    pub shaderSharedFloat32Atomics:  VkBool32,
    pub shaderSharedFloat32AtomicAdd:  VkBool32,
    pub shaderSharedFloat64Atomics:  VkBool32,
    pub shaderSharedFloat64AtomicAdd:  VkBool32,
    pub shaderImageFloat32Atomics:  VkBool32,
    pub shaderImageFloat32AtomicAdd:  VkBool32,
    pub sparseImageFloat32Atomics:  VkBool32,
    pub sparseImageFloat32AtomicAdd:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicFloat2FeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderBufferFloat16Atomics:  VkBool32,
    pub shaderBufferFloat16AtomicAdd:  VkBool32,
    pub shaderBufferFloat16AtomicMinMax:  VkBool32,
    pub shaderBufferFloat32AtomicMinMax:  VkBool32,
    pub shaderBufferFloat64AtomicMinMax:  VkBool32,
    pub shaderSharedFloat16Atomics:  VkBool32,
    pub shaderSharedFloat16AtomicAdd:  VkBool32,
    pub shaderSharedFloat16AtomicMinMax:  VkBool32,
    pub shaderSharedFloat32AtomicMinMax:  VkBool32,
    pub shaderSharedFloat64AtomicMinMax:  VkBool32,
    pub shaderImageFloat32AtomicMinMax:  VkBool32,
    pub sparseImageFloat32AtomicMinMax:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeDivisorFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub vertexAttributeInstanceRateDivisor:  VkBool32,
    pub vertexAttributeInstanceRateZeroDivisor:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeDivisorFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeDivisorFeaturesEXT {
}

#[repr(C)]
pub struct VkQueueFamilyCheckpointPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub checkpointExecutionStageMask:  VkPipelineStageFlags,
}

#[repr(C)]
pub struct VkCheckpointDataNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub stage:  VkPipelineStageFlagBits,
    pub pCheckpointMarker: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthStencilResolveProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub supportedDepthResolveModes:  VkResolveModeFlags,
    pub supportedStencilResolveModes:  VkResolveModeFlags,
    pub independentResolveNone:  VkBool32,
    pub independentResolve:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthStencilResolvePropertiesKHR {
}

#[repr(C)]
pub struct VkSubpassDescriptionDepthStencilResolve {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub depthResolveMode:  VkResolveModeFlagBits,
    pub stencilResolveMode:  VkResolveModeFlagBits,
    pub pDepthStencilResolveAttachment: *const  VkAttachmentReference2,
}

#[repr(C)]
pub struct VkSubpassDescriptionDepthStencilResolveKHR {
}

#[repr(C)]
pub struct VkImageViewASTCDecodeModeEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub decodeMode:  VkFormat,
}

#[repr(C)]
pub struct VkPhysicalDeviceASTCDecodeFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub decodeModeSharedExponent:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceTransformFeedbackFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub transformFeedback:  VkBool32,
    pub geometryStreams:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceTransformFeedbackPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxTransformFeedbackStreams:  u32,
    pub maxTransformFeedbackBuffers:  u32,
    pub maxTransformFeedbackBufferSize:  VkDeviceSize,
    pub maxTransformFeedbackStreamDataSize:  u32,
    pub maxTransformFeedbackBufferDataSize:  u32,
    pub maxTransformFeedbackBufferDataStride:  u32,
    pub transformFeedbackQueries:  VkBool32,
    pub transformFeedbackStreamsLinesTriangles:  VkBool32,
    pub transformFeedbackRasterizationStreamSelect:  VkBool32,
    pub transformFeedbackDraw:  VkBool32,
}

#[repr(C)]
pub struct VkPipelineRasterizationStateStreamCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineRasterizationStateStreamCreateFlagsEXT,
    pub rasterizationStream:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRepresentativeFragmentTestFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub representativeFragmentTest:  VkBool32,
}

#[repr(C)]
pub struct VkPipelineRepresentativeFragmentTestStateCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub representativeFragmentTestEnable:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExclusiveScissorFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub exclusiveScissor:  VkBool32,
}

#[repr(C)]
pub struct VkPipelineViewportExclusiveScissorStateCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub exclusiveScissorCount:  u32,
    pub pExclusiveScissors: *const  VkRect2D,
}

#[repr(C)]
pub struct VkPhysicalDeviceCornerSampledImageFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub cornerSampledImage:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceComputeShaderDerivativesFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub computeDerivativeGroupQuads:  VkBool32,
    pub computeDerivativeGroupLinear:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceComputeShaderDerivativesFeaturesNV {
}

#[repr(C)]
pub struct VkPhysicalDeviceComputeShaderDerivativesPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub meshAndTaskShaderDerivatives:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderBarycentricFeaturesNV {
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderImageFootprintFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub imageFootprint:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub dedicatedAllocationImageAliasing:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCopyMemoryIndirectFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub indirectMemoryCopy:  VkBool32,
    pub indirectMemoryToImageCopy:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCopyMemoryIndirectFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub indirectCopy:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCopyMemoryIndirectPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub supportedQueues:  VkQueueFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceCopyMemoryIndirectPropertiesNV {
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryDecompressionFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub memoryDecompression:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryDecompressionFeaturesNV {
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryDecompressionPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub decompressionMethods:  VkMemoryDecompressionMethodFlagsEXT,
    pub maxDecompressionIndirectCount:  u64,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryDecompressionPropertiesNV {
}

#[repr(C)]
pub struct VkShadingRatePaletteNV {
    pub shadingRatePaletteEntryCount:  u32,
    pub pShadingRatePaletteEntries: *const  VkShadingRatePaletteEntryNV,
}

#[repr(C)]
pub struct VkPipelineViewportShadingRateImageStateCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub shadingRateImageEnable:  VkBool32,
    pub viewportCount:  u32,
    pub pShadingRatePalettes: *const  VkShadingRatePaletteNV,
}

#[repr(C)]
pub struct VkPhysicalDeviceShadingRateImageFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shadingRateImage:  VkBool32,
    pub shadingRateCoarseSampleOrder:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShadingRateImagePropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shadingRateTexelSize:  VkExtent2D,
    pub shadingRatePaletteSize:  u32,
    pub shadingRateMaxCoarseSamples:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceInvocationMaskFeaturesHUAWEI {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub invocationMask:  VkBool32,
}

#[repr(C)]
pub struct VkCoarseSampleLocationNV {
    pub pixelX:  u32,
    pub pixelY:  u32,
    pub sample:  u32,
}

#[repr(C)]
pub struct VkCoarseSampleOrderCustomNV {
    pub shadingRate:  VkShadingRatePaletteEntryNV,
    pub sampleCount:  u32,
    pub sampleLocationCount:  u32,
    pub pSampleLocations: *const  VkCoarseSampleLocationNV,
}

#[repr(C)]
pub struct VkPipelineViewportCoarseSampleOrderStateCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub sampleOrderType:  VkCoarseSampleOrderTypeNV,
    pub customSampleOrderCount:  u32,
    pub pCustomSampleOrders: *const  VkCoarseSampleOrderCustomNV,
}

#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub taskShader:  VkBool32,
    pub meshShader:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxDrawMeshTasksCount:  u32,
    pub maxTaskWorkGroupInvocations:  u32,
    pub maxTaskWorkGroupSize:  u32,
    pub maxTaskTotalMemorySize:  u32,
    pub maxTaskOutputCount:  u32,
    pub maxMeshWorkGroupInvocations:  u32,
    pub maxMeshWorkGroupSize:  u32,
    pub maxMeshTotalMemorySize:  u32,
    pub maxMeshOutputVertices:  u32,
    pub maxMeshOutputPrimitives:  u32,
    pub maxMeshMultiviewViewCount:  u32,
    pub meshOutputPerVertexGranularity:  u32,
    pub meshOutputPerPrimitiveGranularity:  u32,
}

#[repr(C)]
pub struct VkDrawMeshTasksIndirectCommandNV {
    pub taskCount:  u32,
    pub firstTask:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub taskShader:  VkBool32,
    pub meshShader:  VkBool32,
    pub multiviewMeshShader:  VkBool32,
    pub primitiveFragmentShadingRateMeshShader:  VkBool32,
    pub meshShaderQueries:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMeshShaderPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxTaskWorkGroupTotalCount:  u32,
    pub maxTaskWorkGroupCount:  u32,
    pub maxTaskWorkGroupInvocations:  u32,
    pub maxTaskWorkGroupSize:  u32,
    pub maxTaskPayloadSize:  u32,
    pub maxTaskSharedMemorySize:  u32,
    pub maxTaskPayloadAndSharedMemorySize:  u32,
    pub maxMeshWorkGroupTotalCount:  u32,
    pub maxMeshWorkGroupCount:  u32,
    pub maxMeshWorkGroupInvocations:  u32,
    pub maxMeshWorkGroupSize:  u32,
    pub maxMeshSharedMemorySize:  u32,
    pub maxMeshPayloadAndSharedMemorySize:  u32,
    pub maxMeshOutputMemorySize:  u32,
    pub maxMeshPayloadAndOutputMemorySize:  u32,
    pub maxMeshOutputComponents:  u32,
    pub maxMeshOutputVertices:  u32,
    pub maxMeshOutputPrimitives:  u32,
    pub maxMeshOutputLayers:  u32,
    pub maxMeshMultiviewViewCount:  u32,
    pub meshOutputPerVertexGranularity:  u32,
    pub meshOutputPerPrimitiveGranularity:  u32,
    pub maxPreferredTaskWorkGroupInvocations:  u32,
    pub maxPreferredMeshWorkGroupInvocations:  u32,
    pub prefersLocalInvocationVertexOutput:  VkBool32,
    pub prefersLocalInvocationPrimitiveOutput:  VkBool32,
    pub prefersCompactVertexOutput:  VkBool32,
    pub prefersCompactPrimitiveOutput:  VkBool32,
}

#[repr(C)]
pub struct VkDrawMeshTasksIndirectCommandEXT {
    pub groupCountX:  u32,
    pub groupCountY:  u32,
    pub groupCountZ:  u32,
}

#[repr(C)]
pub struct VkRayTracingShaderGroupCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub r#type:  VkRayTracingShaderGroupTypeKHR,
    pub generalShader:  u32,
    pub closestHitShader:  u32,
    pub anyHitShader:  u32,
    pub intersectionShader:  u32,
}

#[repr(C)]
pub struct VkRayTracingShaderGroupCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub r#type:  VkRayTracingShaderGroupTypeKHR,
    pub generalShader:  u32,
    pub closestHitShader:  u32,
    pub anyHitShader:  u32,
    pub intersectionShader:  u32,
    pub pShaderGroupCaptureReplayHandle: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkRayTracingPipelineCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineCreateFlags,
    pub stageCount:  u32,
    pub pStages: *const  VkPipelineShaderStageCreateInfo,
    pub groupCount:  u32,
    pub pGroups: *const  VkRayTracingShaderGroupCreateInfoNV,
    pub maxRecursionDepth:  u32,
    pub layout:  VkPipelineLayout,
    pub basePipelineHandle:  VkPipeline,
    pub basePipelineIndex:  i32,
}

#[repr(C)]
pub struct VkRayTracingPipelineCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineCreateFlags,
    pub stageCount:  u32,
    pub pStages: *const  VkPipelineShaderStageCreateInfo,
    pub groupCount:  u32,
    pub pGroups: *const  VkRayTracingShaderGroupCreateInfoKHR,
    pub maxPipelineRayRecursionDepth:  u32,
    pub pLibraryInfo: *const  VkPipelineLibraryCreateInfoKHR,
    pub pLibraryInterface: *const  VkRayTracingPipelineInterfaceCreateInfoKHR,
    pub pDynamicState: *const  VkPipelineDynamicStateCreateInfo,
    pub layout:  VkPipelineLayout,
    pub basePipelineHandle:  VkPipeline,
    pub basePipelineIndex:  i32,
}

#[repr(C)]
pub struct VkGeometryTrianglesNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub vertexData:  VkBuffer,
    pub vertexOffset:  VkDeviceSize,
    pub vertexCount:  u32,
    pub vertexStride:  VkDeviceSize,
    pub vertexFormat:  VkFormat,
    pub indexData:  VkBuffer,
    pub indexOffset:  VkDeviceSize,
    pub indexCount:  u32,
    pub indexType:  VkIndexType,
    pub transformData:  VkBuffer,
    pub transformOffset:  VkDeviceSize,
}

#[repr(C)]
pub struct VkGeometryAABBNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub aabbData:  VkBuffer,
    pub numAABBs:  u32,
    pub stride:  u32,
    pub offset:  VkDeviceSize,
}

#[repr(C)]
pub struct VkGeometryDataNV {
    pub triangles:  VkGeometryTrianglesNV,
    pub aabbs:  VkGeometryAABBNV,
}

#[repr(C)]
pub struct VkGeometryNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub geometryType:  VkGeometryTypeKHR,
    pub geometry:  VkGeometryDataNV,
    pub flags:  VkGeometryFlagsKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub r#type:  VkAccelerationStructureTypeNV,
    pub flags:  VkBuildAccelerationStructureFlagsKHR,
    pub instanceCount:  u32,
    pub geometryCount:  u32,
    pub pGeometries: *const  VkGeometryNV,
}

#[repr(C)]
pub struct VkAccelerationStructureCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub compactedSize:  VkDeviceSize,
    pub info:  VkAccelerationStructureInfoNV,
}

#[repr(C)]
pub struct VkBindAccelerationStructureMemoryInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub accelerationStructure:  VkAccelerationStructureNV,
    pub memory:  VkDeviceMemory,
    pub memoryOffset:  VkDeviceSize,
    pub deviceIndexCount:  u32,
    pub pDeviceIndices: *const  u32,
}

#[repr(C)]
pub struct VkWriteDescriptorSetAccelerationStructureKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub accelerationStructureCount:  u32,
    pub pAccelerationStructures: *const  VkAccelerationStructureKHR,
}

#[repr(C)]
pub struct VkWriteDescriptorSetAccelerationStructureNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub accelerationStructureCount:  u32,
    pub pAccelerationStructures: *const  VkAccelerationStructureNV,
}

#[repr(C)]
pub struct VkAccelerationStructureMemoryRequirementsInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub r#type:  VkAccelerationStructureMemoryRequirementsTypeNV,
    pub accelerationStructure:  VkAccelerationStructureNV,
}

#[repr(C)]
pub struct VkPhysicalDeviceAccelerationStructureFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub accelerationStructure:  VkBool32,
    pub accelerationStructureCaptureReplay:  VkBool32,
    pub accelerationStructureIndirectBuild:  VkBool32,
    pub accelerationStructureHostCommands:  VkBool32,
    pub descriptorBindingAccelerationStructureUpdateAfterBind:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPipelineFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub rayTracingPipeline:  VkBool32,
    pub rayTracingPipelineShaderGroupHandleCaptureReplay:  VkBool32,
    pub rayTracingPipelineShaderGroupHandleCaptureReplayMixed:  VkBool32,
    pub rayTracingPipelineTraceRaysIndirect:  VkBool32,
    pub rayTraversalPrimitiveCulling:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayQueryFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub rayQuery:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceAccelerationStructurePropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxGeometryCount:  u64,
    pub maxInstanceCount:  u64,
    pub maxPrimitiveCount:  u64,
    pub maxPerStageDescriptorAccelerationStructures:  u32,
    pub maxPerStageDescriptorUpdateAfterBindAccelerationStructures:  u32,
    pub maxDescriptorSetAccelerationStructures:  u32,
    pub maxDescriptorSetUpdateAfterBindAccelerationStructures:  u32,
    pub minAccelerationStructureScratchOffsetAlignment:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPipelinePropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderGroupHandleSize:  u32,
    pub maxRayRecursionDepth:  u32,
    pub maxShaderGroupStride:  u32,
    pub shaderGroupBaseAlignment:  u32,
    pub shaderGroupHandleCaptureReplaySize:  u32,
    pub maxRayDispatchInvocationCount:  u32,
    pub shaderGroupHandleAlignment:  u32,
    pub maxRayHitAttributeSize:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderGroupHandleSize:  u32,
    pub maxRecursionDepth:  u32,
    pub maxShaderGroupStride:  u32,
    pub shaderGroupBaseAlignment:  u32,
    pub maxGeometryCount:  u64,
    pub maxInstanceCount:  u64,
    pub maxTriangleCount:  u64,
    pub maxDescriptorSetAccelerationStructures:  u32,
}

#[repr(C)]
pub struct VkStridedDeviceAddressRegionKHR {
    pub deviceAddress:  VkDeviceAddress,
    pub stride:  VkDeviceSize,
    pub size:  VkDeviceSize,
}

#[repr(C)]
pub struct VkTraceRaysIndirectCommandKHR {
    pub width:  u32,
    pub height:  u32,
    pub depth:  u32,
}

#[repr(C)]
pub struct VkTraceRaysIndirectCommand2KHR {
    pub raygenShaderRecordAddress:  VkDeviceAddress,
    pub raygenShaderRecordSize:  VkDeviceSize,
    pub missShaderBindingTableAddress:  VkDeviceAddress,
    pub missShaderBindingTableSize:  VkDeviceSize,
    pub missShaderBindingTableStride:  VkDeviceSize,
    pub hitShaderBindingTableAddress:  VkDeviceAddress,
    pub hitShaderBindingTableSize:  VkDeviceSize,
    pub hitShaderBindingTableStride:  VkDeviceSize,
    pub callableShaderBindingTableAddress:  VkDeviceAddress,
    pub callableShaderBindingTableSize:  VkDeviceSize,
    pub callableShaderBindingTableStride:  VkDeviceSize,
    pub width:  u32,
    pub height:  u32,
    pub depth:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingMaintenance1FeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub rayTracingMaintenance1:  VkBool32,
    pub rayTracingPipelineTraceRaysIndirect2:  VkBool32,
}

#[repr(C)]
pub struct VkDrmFormatModifierPropertiesListEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub drmFormatModifierCount:  u32,
    pub pDrmFormatModifierProperties: *mut  VkDrmFormatModifierPropertiesEXT,
}

#[repr(C)]
pub struct VkDrmFormatModifierPropertiesEXT {
    pub drmFormatModifier:  u64,
    pub drmFormatModifierPlaneCount:  u32,
    pub drmFormatModifierTilingFeatures:  VkFormatFeatureFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageDrmFormatModifierInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub drmFormatModifier:  u64,
    pub sharingMode:  VkSharingMode,
    pub queueFamilyIndexCount:  u32,
    pub pQueueFamilyIndices: *const  u32,
}

#[repr(C)]
pub struct VkImageDrmFormatModifierListCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub drmFormatModifierCount:  u32,
    pub pDrmFormatModifiers: *const  u64,
}

#[repr(C)]
pub struct VkImageDrmFormatModifierExplicitCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub drmFormatModifier:  u64,
    pub drmFormatModifierPlaneCount:  u32,
    pub pPlaneLayouts: *const  VkSubresourceLayout,
}

#[repr(C)]
pub struct VkImageDrmFormatModifierPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub drmFormatModifier:  u64,
}

#[repr(C)]
pub struct VkImageStencilUsageCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stencilUsage:  VkImageUsageFlags,
}

#[repr(C)]
pub struct VkImageStencilUsageCreateInfoEXT {
}

#[repr(C)]
pub struct VkImageStencilUsage2CreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub stencilUsage:  VkImageUsageFlags2KHR,
}

#[repr(C)]
pub struct VkDeviceMemoryOverallocationCreateInfoAMD {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub overallocationBehavior:  VkMemoryOverallocationBehaviorAMD,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub fragmentDensityMap:  VkBool32,
    pub fragmentDensityMapDynamic:  VkBool32,
    pub fragmentDensityMapNonSubsampledImages:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMap2FeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub fragmentDensityMapDeferred:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub fragmentDensityMapOffset:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetFeaturesQCOM {
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub minFragmentDensityTexelSize:  VkExtent2D,
    pub maxFragmentDensityTexelSize:  VkExtent2D,
    pub fragmentDensityInvocations:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMap2PropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub subsampledLoads:  VkBool32,
    pub subsampledCoarseReconstructionEarlyAccess:  VkBool32,
    pub maxSubsampledArrayLayers:  u32,
    pub maxDescriptorSetSubsampledSamplers:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub fragmentDensityOffsetGranularity:  VkExtent2D,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapOffsetPropertiesQCOM {
}

#[repr(C)]
pub struct VkRenderPassFragmentDensityMapCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub fragmentDensityMapAttachment:  VkAttachmentReference,
}

#[repr(C)]
pub struct VkRenderPassFragmentDensityMapOffsetEndInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub fragmentDensityOffsetCount:  u32,
    pub pFragmentDensityOffsets: *const  VkOffset2D,
}

#[repr(C)]
pub struct VkSubpassFragmentDensityMapOffsetEndInfoQCOM {
}

#[repr(C)]
pub struct VkPhysicalDeviceScalarBlockLayoutFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub scalarBlockLayout:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceScalarBlockLayoutFeaturesEXT {
}

#[repr(C)]
pub struct VkSurfaceProtectedCapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub supportsProtected:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub uniformBufferStandardLayout:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceUniformBufferStandardLayoutFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthClipEnableFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub depthClipEnable:  VkBool32,
}

#[repr(C)]
pub struct VkPipelineRasterizationDepthClipStateCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineRasterizationDepthClipStateCreateFlagsEXT,
    pub depthClipEnable:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryBudgetPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub heapBudget:  [VkDeviceSize; VK_MAX_MEMORY_HEAPS as usize],
    pub heapUsage:  [VkDeviceSize; VK_MAX_MEMORY_HEAPS as usize],
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryPriorityFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub memoryPriority:  VkBool32,
}

#[repr(C)]
pub struct VkMemoryPriorityAllocateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub priority:  f32,
}

#[repr(C)]
pub struct VkPhysicalDevicePageableDeviceLocalMemoryFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pageableDeviceLocalMemory:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub bufferDeviceAddress:  VkBool32,
    pub bufferDeviceAddressCaptureReplay:  VkBool32,
    pub bufferDeviceAddressMultiDevice:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceBufferDeviceAddressFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub bufferDeviceAddress:  VkBool32,
    pub bufferDeviceAddressCaptureReplay:  VkBool32,
    pub bufferDeviceAddressMultiDevice:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceBufferAddressFeaturesEXT {
}

#[repr(C)]
pub struct VkBufferDeviceAddressInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub buffer:  VkBuffer,
}

#[repr(C)]
pub struct VkBufferDeviceAddressInfoKHR {
}

#[repr(C)]
pub struct VkBufferDeviceAddressInfoEXT {
}

#[repr(C)]
pub struct VkBufferOpaqueCaptureAddressCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub opaqueCaptureAddress:  u64,
}

#[repr(C)]
pub struct VkBufferOpaqueCaptureAddressCreateInfoKHR {
}

#[repr(C)]
pub struct VkBufferDeviceAddressCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub deviceAddress:  VkDeviceAddress,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageViewImageFormatInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub imageViewType:  VkImageViewType,
}

#[repr(C)]
pub struct VkFilterCubicImageViewImageFormatPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub filterCubic:  VkBool32,
    pub filterCubicMinmax:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImagelessFramebufferFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub imagelessFramebuffer:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImagelessFramebufferFeaturesKHR {
}

#[repr(C)]
pub struct VkFramebufferAttachmentsCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub attachmentImageInfoCount:  u32,
    pub pAttachmentImageInfos: *const  VkFramebufferAttachmentImageInfo,
}

#[repr(C)]
pub struct VkFramebufferAttachmentsCreateInfoKHR {
}

#[repr(C)]
pub struct VkFramebufferAttachmentImageInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkImageCreateFlags,
    pub usage:  VkImageUsageFlags,
    pub width:  u32,
    pub height:  u32,
    pub layerCount:  u32,
    pub viewFormatCount:  u32,
    pub pViewFormats: *const  VkFormat,
}

#[repr(C)]
pub struct VkFramebufferAttachmentImageInfoKHR {
}

#[repr(C)]
pub struct VkRenderPassAttachmentBeginInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub attachmentCount:  u32,
    pub pAttachments: *const  VkImageView,
}

#[repr(C)]
pub struct VkRenderPassAttachmentBeginInfoKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceTextureCompressionASTCHDRFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub textureCompressionASTC_HDR:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceTextureCompressionASTCHDRFeaturesEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub cooperativeMatrix:  VkBool32,
    pub cooperativeMatrixRobustBufferAccess:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub cooperativeMatrixSupportedStages:  VkShaderStageFlags,
}

#[repr(C)]
pub struct VkCooperativeMatrixPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub MSize:  u32,
    pub NSize:  u32,
    pub KSize:  u32,
    pub AType:  VkComponentTypeNV,
    pub BType:  VkComponentTypeNV,
    pub CType:  VkComponentTypeNV,
    pub DType:  VkComponentTypeNV,
    pub scope:  VkScopeNV,
}

#[repr(C)]
pub struct VkPhysicalDeviceYcbcrImageArraysFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub ycbcrImageArrays:  VkBool32,
}

#[repr(C)]
pub struct VkImageViewHandleInfoNVX {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub imageView:  VkImageView,
    pub descriptorType:  VkDescriptorType,
    pub sampler:  VkSampler,
}

#[repr(C)]
pub struct VkImageViewAddressPropertiesNVX {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub deviceAddress:  VkDeviceAddress,
    pub size:  VkDeviceSize,
}

#[repr(C)]
pub struct VkPresentFrameTokenGGP {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub frameToken:  GgpFrameToken,
}

#[repr(C)]
pub struct VkPipelineCreationFeedback {
    pub flags:  VkPipelineCreationFeedbackFlags,
    pub duration:  u64,
}

#[repr(C)]
pub struct VkPipelineCreationFeedbackEXT {
}

#[repr(C)]
pub struct VkPipelineCreationFeedbackCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pPipelineCreationFeedback: *mut  VkPipelineCreationFeedback,
    pub pipelineStageCreationFeedbackCount:  u32,
    pub pPipelineStageCreationFeedbacks: *mut  VkPipelineCreationFeedback,
}

#[repr(C)]
pub struct VkPipelineCreationFeedbackCreateInfoEXT {
}

#[repr(C)]
pub struct VkSurfaceFullScreenExclusiveInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub fullScreenExclusive:  VkFullScreenExclusiveEXT,
}

#[repr(C)]
pub struct VkSurfaceFullScreenExclusiveWin32InfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub hmonitor:  HMONITOR,
}

#[repr(C)]
pub struct VkSurfaceCapabilitiesFullScreenExclusiveEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub fullScreenExclusiveSupported:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentBarrierFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentBarrier:  VkBool32,
}

#[repr(C)]
pub struct VkSurfaceCapabilitiesPresentBarrierNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentBarrierSupported:  VkBool32,
}

#[repr(C)]
pub struct VkSwapchainPresentBarrierCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentBarrierEnable:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePerformanceQueryFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub performanceCounterQueryPools:  VkBool32,
    pub performanceCounterMultipleQueryPools:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePerformanceQueryPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub allowCommandBufferQueryCopies:  VkBool32,
}

#[repr(C)]
pub struct VkPerformanceCounterKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub unit:  VkPerformanceCounterUnitKHR,
    pub scope:  VkPerformanceCounterScopeKHR,
    pub storage:  VkPerformanceCounterStorageKHR,
    pub uuid:  [u8; VK_UUID_SIZE as usize],
}

#[repr(C)]
pub struct VkPerformanceCounterDescriptionKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkPerformanceCounterDescriptionFlagsKHR,
    pub name:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
    pub category:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
    pub description:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
}

#[repr(C)]
pub struct VkQueryPoolPerformanceCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub queueFamilyIndex:  u32,
    pub counterIndexCount:  u32,
    pub pCounterIndices: *const  u32,
}

#[repr(C)]
pub struct VkPerformanceCounterResultKHR {
    pub int32:  i32,
    pub int64:  i64,
    pub uint32:  u32,
    pub uint64:  u64,
    pub float32:  f32,
    pub float64:  f64,
}

#[repr(C)]
pub struct VkAcquireProfilingLockInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkAcquireProfilingLockFlagsKHR,
    pub timeout:  u64,
}

#[repr(C)]
pub struct VkPerformanceQuerySubmitInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub counterPassIndex:  u32,
}

#[repr(C)]
pub struct VkPerformanceQueryReservationInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub maxPerformanceQueriesPerPool:  u32,
}

#[repr(C)]
pub struct VkHeadlessSurfaceCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkHeadlessSurfaceCreateFlagsEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceCoverageReductionModeFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub coverageReductionMode:  VkBool32,
}

#[repr(C)]
pub struct VkPipelineCoverageReductionStateCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineCoverageReductionStateCreateFlagsNV,
    pub coverageReductionMode:  VkCoverageReductionModeNV,
}

#[repr(C)]
pub struct VkFramebufferMixedSamplesCombinationNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub coverageReductionMode:  VkCoverageReductionModeNV,
    pub rasterizationSamples:  VkSampleCountFlagBits,
    pub depthStencilSamples:  VkSampleCountFlags,
    pub colorSamples:  VkSampleCountFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerFunctions2FeaturesINTEL {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderIntegerFunctions2:  VkBool32,
}

#[repr(C)]
pub struct VkPerformanceValueDataINTEL {
    pub value32:  u32,
    pub value64:  u64,
    pub valueFloat:  f32,
    pub valueBool:  VkBool32,
    pub valueString: *const  i8,
}

#[repr(C)]
pub struct VkPerformanceValueINTEL {
    pub r#type:  VkPerformanceValueTypeINTEL,
    pub data:  VkPerformanceValueDataINTEL,
}

#[repr(C)]
pub struct VkInitializePerformanceApiInfoINTEL {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pUserData: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkQueryPoolPerformanceQueryCreateInfoINTEL {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub performanceCountersSampling:  VkQueryPoolSamplingModeINTEL,
}

#[repr(C)]
pub struct VkQueryPoolCreateInfoINTEL {
}

#[repr(C)]
pub struct VkPerformanceMarkerInfoINTEL {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub marker:  u64,
}

#[repr(C)]
pub struct VkPerformanceStreamMarkerInfoINTEL {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub marker:  u32,
}

#[repr(C)]
pub struct VkPerformanceOverrideInfoINTEL {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub r#type:  VkPerformanceOverrideTypeINTEL,
    pub enable:  VkBool32,
    pub parameter:  u64,
}

#[repr(C)]
pub struct VkPerformanceConfigurationAcquireInfoINTEL {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub r#type:  VkPerformanceConfigurationTypeINTEL,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderClockFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderSubgroupClock:  VkBool32,
    pub shaderDeviceClock:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceIndexTypeUint8Features {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub indexTypeUint8:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceIndexTypeUint8FeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceIndexTypeUint8FeaturesEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSMBuiltinsPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderSMCount:  u32,
    pub shaderWarpsPerSM:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSMBuiltinsFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderSMBuiltins:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderInterlockFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub fragmentShaderSampleInterlock:  VkBool32,
    pub fragmentShaderPixelInterlock:  VkBool32,
    pub fragmentShaderShadingRateInterlock:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub separateDepthStencilLayouts:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSeparateDepthStencilLayoutsFeaturesKHR {
}

#[repr(C)]
pub struct VkAttachmentReferenceStencilLayout {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub stencilLayout:  VkImageLayout,
}

#[repr(C)]
pub struct VkPhysicalDevicePrimitiveTopologyListRestartFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub primitiveTopologyListRestart:  VkBool32,
    pub primitiveTopologyPatchListRestart:  VkBool32,
}

#[repr(C)]
pub struct VkAttachmentReferenceStencilLayoutKHR {
}

#[repr(C)]
pub struct VkAttachmentDescriptionStencilLayout {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub stencilInitialLayout:  VkImageLayout,
    pub stencilFinalLayout:  VkImageLayout,
}

#[repr(C)]
pub struct VkAttachmentDescriptionStencilLayoutKHR {
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineExecutablePropertiesFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pipelineExecutableInfo:  VkBool32,
}

#[repr(C)]
pub struct VkPipelineInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pipeline:  VkPipeline,
}

#[repr(C)]
pub struct VkPipelineInfoEXT {
}

#[repr(C)]
pub struct VkPipelineExecutablePropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub stages:  VkShaderStageFlags,
    pub name:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
    pub description:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
    pub subgroupSize:  u32,
}

#[repr(C)]
pub struct VkPipelineExecutableInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pipeline:  VkPipeline,
    pub executableIndex:  u32,
}

#[repr(C)]
pub struct VkPipelineExecutableStatisticValueKHR {
    pub b32:  VkBool32,
    pub i64:  i64,
    pub u64:  u64,
    pub f64:  f64,
}

#[repr(C)]
pub struct VkPipelineExecutableStatisticKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub name:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
    pub description:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
    pub format:  VkPipelineExecutableStatisticFormatKHR,
    pub value:  VkPipelineExecutableStatisticValueKHR,
}

#[repr(C)]
pub struct VkPipelineExecutableInternalRepresentationKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub name:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
    pub description:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
    pub isText:  VkBool32,
    pub dataSize:  usize,
    pub pData: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderDemoteToHelperInvocationFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderDemoteToHelperInvocation:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderDemoteToHelperInvocationFeaturesEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceTexelBufferAlignmentFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub texelBufferAlignment:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceTexelBufferAlignmentProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub storageTexelBufferOffsetAlignmentBytes:  VkDeviceSize,
    pub storageTexelBufferOffsetSingleTexelAlignment:  VkBool32,
    pub uniformTexelBufferOffsetAlignmentBytes:  VkDeviceSize,
    pub uniformTexelBufferOffsetSingleTexelAlignment:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceTexelBufferAlignmentPropertiesEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceSubgroupSizeControlFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub subgroupSizeControl:  VkBool32,
    pub computeFullSubgroups:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubgroupSizeControlFeaturesEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceSubgroupSizeControlProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub minSubgroupSize:  u32,
    pub maxSubgroupSize:  u32,
    pub maxComputeWorkgroupSubgroups:  u32,
    pub requiredSubgroupSizeStages:  VkShaderStageFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubgroupSizeControlPropertiesEXT {
}

#[repr(C)]
pub struct VkPipelineShaderStageRequiredSubgroupSizeCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub requiredSubgroupSize:  u32,
}

#[repr(C)]
pub struct VkPipelineShaderStageRequiredSubgroupSizeCreateInfoEXT {
}

#[repr(C)]
pub struct VkShaderRequiredSubgroupSizeCreateInfoEXT {
}

#[repr(C)]
pub struct VkSubpassShadingPipelineCreateInfoHUAWEI {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub renderPass:  VkRenderPass,
    pub subpass:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubpassShadingPropertiesHUAWEI {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxSubpassShadingWorkgroupSizeAspectRatio:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceClusterCullingShaderPropertiesHUAWEI {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxWorkGroupCount:  u32,
    pub maxWorkGroupSize:  u32,
    pub maxOutputClusterCount:  u32,
    pub indirectBufferOffsetAlignment:  VkDeviceSize,
}

#[repr(C)]
pub struct VkMemoryOpaqueCaptureAddressAllocateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub opaqueCaptureAddress:  u64,
}

#[repr(C)]
pub struct VkMemoryOpaqueCaptureAddressAllocateInfoKHR {
}

#[repr(C)]
pub struct VkDeviceMemoryOpaqueCaptureAddressInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub memory:  VkDeviceMemory,
}

#[repr(C)]
pub struct VkDeviceMemoryOpaqueCaptureAddressInfoKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceLineRasterizationFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub rectangularLines:  VkBool32,
    pub bresenhamLines:  VkBool32,
    pub smoothLines:  VkBool32,
    pub stippledRectangularLines:  VkBool32,
    pub stippledBresenhamLines:  VkBool32,
    pub stippledSmoothLines:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceLineRasterizationFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceLineRasterizationFeaturesEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceLineRasterizationProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub lineSubPixelPrecisionBits:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceLineRasterizationPropertiesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceLineRasterizationPropertiesEXT {
}

#[repr(C)]
pub struct VkPipelineRasterizationLineStateCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub lineRasterizationMode:  VkLineRasterizationMode,
    pub stippledLineEnable:  VkBool32,
    pub lineStippleFactor:  u32,
    pub lineStipplePattern:  u16,
}

#[repr(C)]
pub struct VkPipelineRasterizationLineStateCreateInfoKHR {
}

#[repr(C)]
pub struct VkPipelineRasterizationLineStateCreateInfoEXT {
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineCreationCacheControlFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pipelineCreationCacheControl:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineCreationCacheControlFeaturesEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan11Features {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub storageBuffer16BitAccess:  VkBool32,
    pub uniformAndStorageBuffer16BitAccess:  VkBool32,
    pub storagePushConstant16:  VkBool32,
    pub storageInputOutput16:  VkBool32,
    pub multiview:  VkBool32,
    pub multiviewGeometryShader:  VkBool32,
    pub multiviewTessellationShader:  VkBool32,
    pub variablePointersStorageBuffer:  VkBool32,
    pub variablePointers:  VkBool32,
    pub protectedMemory:  VkBool32,
    pub samplerYcbcrConversion:  VkBool32,
    pub shaderDrawParameters:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan11Properties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub deviceUUID:  [u8; VK_UUID_SIZE as usize],
    pub driverUUID:  [u8; VK_UUID_SIZE as usize],
    pub deviceLUID:  [u8; VK_LUID_SIZE as usize],
    pub deviceNodeMask:  u32,
    pub deviceLUIDValid:  VkBool32,
    pub subgroupSize:  u32,
    pub subgroupSupportedStages:  VkShaderStageFlags,
    pub subgroupSupportedOperations:  VkSubgroupFeatureFlags,
    pub subgroupQuadOperationsInAllStages:  VkBool32,
    pub pointClippingBehavior:  VkPointClippingBehavior,
    pub maxMultiviewViewCount:  u32,
    pub maxMultiviewInstanceIndex:  u32,
    pub protectedNoFault:  VkBool32,
    pub maxPerSetDescriptors:  u32,
    pub maxMemoryAllocationSize:  VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan12Features {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub samplerMirrorClampToEdge:  VkBool32,
    pub drawIndirectCount:  VkBool32,
    pub storageBuffer8BitAccess:  VkBool32,
    pub uniformAndStorageBuffer8BitAccess:  VkBool32,
    pub storagePushConstant8:  VkBool32,
    pub shaderBufferInt64Atomics:  VkBool32,
    pub shaderSharedInt64Atomics:  VkBool32,
    pub shaderFloat16:  VkBool32,
    pub shaderInt8:  VkBool32,
    pub descriptorIndexing:  VkBool32,
    pub shaderInputAttachmentArrayDynamicIndexing:  VkBool32,
    pub shaderUniformTexelBufferArrayDynamicIndexing:  VkBool32,
    pub shaderStorageTexelBufferArrayDynamicIndexing:  VkBool32,
    pub shaderUniformBufferArrayNonUniformIndexing:  VkBool32,
    pub shaderSampledImageArrayNonUniformIndexing:  VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexing:  VkBool32,
    pub shaderStorageImageArrayNonUniformIndexing:  VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexing:  VkBool32,
    pub shaderUniformTexelBufferArrayNonUniformIndexing:  VkBool32,
    pub shaderStorageTexelBufferArrayNonUniformIndexing:  VkBool32,
    pub descriptorBindingUniformBufferUpdateAfterBind:  VkBool32,
    pub descriptorBindingSampledImageUpdateAfterBind:  VkBool32,
    pub descriptorBindingStorageImageUpdateAfterBind:  VkBool32,
    pub descriptorBindingStorageBufferUpdateAfterBind:  VkBool32,
    pub descriptorBindingUniformTexelBufferUpdateAfterBind:  VkBool32,
    pub descriptorBindingStorageTexelBufferUpdateAfterBind:  VkBool32,
    pub descriptorBindingUpdateUnusedWhilePending:  VkBool32,
    pub descriptorBindingPartiallyBound:  VkBool32,
    pub descriptorBindingVariableDescriptorCount:  VkBool32,
    pub runtimeDescriptorArray:  VkBool32,
    pub samplerFilterMinmax:  VkBool32,
    pub scalarBlockLayout:  VkBool32,
    pub imagelessFramebuffer:  VkBool32,
    pub uniformBufferStandardLayout:  VkBool32,
    pub shaderSubgroupExtendedTypes:  VkBool32,
    pub separateDepthStencilLayouts:  VkBool32,
    pub hostQueryReset:  VkBool32,
    pub timelineSemaphore:  VkBool32,
    pub bufferDeviceAddress:  VkBool32,
    pub bufferDeviceAddressCaptureReplay:  VkBool32,
    pub bufferDeviceAddressMultiDevice:  VkBool32,
    pub vulkanMemoryModel:  VkBool32,
    pub vulkanMemoryModelDeviceScope:  VkBool32,
    pub vulkanMemoryModelAvailabilityVisibilityChains:  VkBool32,
    pub shaderOutputViewportIndex:  VkBool32,
    pub shaderOutputLayer:  VkBool32,
    pub subgroupBroadcastDynamicId:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan12Properties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub driverID:  VkDriverId,
    pub driverName:  [i8; VK_MAX_DRIVER_NAME_SIZE as usize],
    pub driverInfo:  [i8; VK_MAX_DRIVER_INFO_SIZE as usize],
    pub conformanceVersion:  VkConformanceVersion,
    pub denormBehaviorIndependence:  VkShaderFloatControlsIndependence,
    pub roundingModeIndependence:  VkShaderFloatControlsIndependence,
    pub shaderSignedZeroInfNanPreserveFloat16:  VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat32:  VkBool32,
    pub shaderSignedZeroInfNanPreserveFloat64:  VkBool32,
    pub shaderDenormPreserveFloat16:  VkBool32,
    pub shaderDenormPreserveFloat32:  VkBool32,
    pub shaderDenormPreserveFloat64:  VkBool32,
    pub shaderDenormFlushToZeroFloat16:  VkBool32,
    pub shaderDenormFlushToZeroFloat32:  VkBool32,
    pub shaderDenormFlushToZeroFloat64:  VkBool32,
    pub shaderRoundingModeRTEFloat16:  VkBool32,
    pub shaderRoundingModeRTEFloat32:  VkBool32,
    pub shaderRoundingModeRTEFloat64:  VkBool32,
    pub shaderRoundingModeRTZFloat16:  VkBool32,
    pub shaderRoundingModeRTZFloat32:  VkBool32,
    pub shaderRoundingModeRTZFloat64:  VkBool32,
    pub maxUpdateAfterBindDescriptorsInAllPools:  u32,
    pub shaderUniformBufferArrayNonUniformIndexingNative:  VkBool32,
    pub shaderSampledImageArrayNonUniformIndexingNative:  VkBool32,
    pub shaderStorageBufferArrayNonUniformIndexingNative:  VkBool32,
    pub shaderStorageImageArrayNonUniformIndexingNative:  VkBool32,
    pub shaderInputAttachmentArrayNonUniformIndexingNative:  VkBool32,
    pub robustBufferAccessUpdateAfterBind:  VkBool32,
    pub quadDivergentImplicitLod:  VkBool32,
    pub maxPerStageDescriptorUpdateAfterBindSamplers:  u32,
    pub maxPerStageDescriptorUpdateAfterBindUniformBuffers:  u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageBuffers:  u32,
    pub maxPerStageDescriptorUpdateAfterBindSampledImages:  u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageImages:  u32,
    pub maxPerStageDescriptorUpdateAfterBindInputAttachments:  u32,
    pub maxPerStageUpdateAfterBindResources:  u32,
    pub maxDescriptorSetUpdateAfterBindSamplers:  u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffers:  u32,
    pub maxDescriptorSetUpdateAfterBindUniformBuffersDynamic:  u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffers:  u32,
    pub maxDescriptorSetUpdateAfterBindStorageBuffersDynamic:  u32,
    pub maxDescriptorSetUpdateAfterBindSampledImages:  u32,
    pub maxDescriptorSetUpdateAfterBindStorageImages:  u32,
    pub maxDescriptorSetUpdateAfterBindInputAttachments:  u32,
    pub supportedDepthResolveModes:  VkResolveModeFlags,
    pub supportedStencilResolveModes:  VkResolveModeFlags,
    pub independentResolveNone:  VkBool32,
    pub independentResolve:  VkBool32,
    pub filterMinmaxSingleComponentFormats:  VkBool32,
    pub filterMinmaxImageComponentMapping:  VkBool32,
    pub maxTimelineSemaphoreValueDifference:  u64,
    pub framebufferIntegerColorSampleCounts:  VkSampleCountFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan13Features {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub robustImageAccess:  VkBool32,
    pub inlineUniformBlock:  VkBool32,
    pub descriptorBindingInlineUniformBlockUpdateAfterBind:  VkBool32,
    pub pipelineCreationCacheControl:  VkBool32,
    pub privateData:  VkBool32,
    pub shaderDemoteToHelperInvocation:  VkBool32,
    pub shaderTerminateInvocation:  VkBool32,
    pub subgroupSizeControl:  VkBool32,
    pub computeFullSubgroups:  VkBool32,
    pub synchronization2:  VkBool32,
    pub textureCompressionASTC_HDR:  VkBool32,
    pub shaderZeroInitializeWorkgroupMemory:  VkBool32,
    pub dynamicRendering:  VkBool32,
    pub shaderIntegerDotProduct:  VkBool32,
    pub maintenance4:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan13Properties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub minSubgroupSize:  u32,
    pub maxSubgroupSize:  u32,
    pub maxComputeWorkgroupSubgroups:  u32,
    pub requiredSubgroupSizeStages:  VkShaderStageFlags,
    pub maxInlineUniformBlockSize:  u32,
    pub maxPerStageDescriptorInlineUniformBlocks:  u32,
    pub maxPerStageDescriptorUpdateAfterBindInlineUniformBlocks:  u32,
    pub maxDescriptorSetInlineUniformBlocks:  u32,
    pub maxDescriptorSetUpdateAfterBindInlineUniformBlocks:  u32,
    pub maxInlineUniformTotalSize:  u32,
    pub integerDotProduct8BitUnsignedAccelerated:  VkBool32,
    pub integerDotProduct8BitSignedAccelerated:  VkBool32,
    pub integerDotProduct8BitMixedSignednessAccelerated:  VkBool32,
    pub integerDotProduct4x8BitPackedUnsignedAccelerated:  VkBool32,
    pub integerDotProduct4x8BitPackedSignedAccelerated:  VkBool32,
    pub integerDotProduct4x8BitPackedMixedSignednessAccelerated:  VkBool32,
    pub integerDotProduct16BitUnsignedAccelerated:  VkBool32,
    pub integerDotProduct16BitSignedAccelerated:  VkBool32,
    pub integerDotProduct16BitMixedSignednessAccelerated:  VkBool32,
    pub integerDotProduct32BitUnsignedAccelerated:  VkBool32,
    pub integerDotProduct32BitSignedAccelerated:  VkBool32,
    pub integerDotProduct32BitMixedSignednessAccelerated:  VkBool32,
    pub integerDotProduct64BitUnsignedAccelerated:  VkBool32,
    pub integerDotProduct64BitSignedAccelerated:  VkBool32,
    pub integerDotProduct64BitMixedSignednessAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating8BitUnsignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating8BitSignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating16BitUnsignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating16BitSignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating32BitUnsignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating32BitSignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating64BitUnsignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating64BitSignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated:  VkBool32,
    pub storageTexelBufferOffsetAlignmentBytes:  VkDeviceSize,
    pub storageTexelBufferOffsetSingleTexelAlignment:  VkBool32,
    pub uniformTexelBufferOffsetAlignmentBytes:  VkDeviceSize,
    pub uniformTexelBufferOffsetSingleTexelAlignment:  VkBool32,
    pub maxBufferSize:  VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan14Features {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub globalPriorityQuery:  VkBool32,
    pub shaderSubgroupRotate:  VkBool32,
    pub shaderSubgroupRotateClustered:  VkBool32,
    pub shaderFloatControls2:  VkBool32,
    pub shaderExpectAssume:  VkBool32,
    pub rectangularLines:  VkBool32,
    pub bresenhamLines:  VkBool32,
    pub smoothLines:  VkBool32,
    pub stippledRectangularLines:  VkBool32,
    pub stippledBresenhamLines:  VkBool32,
    pub stippledSmoothLines:  VkBool32,
    pub vertexAttributeInstanceRateDivisor:  VkBool32,
    pub vertexAttributeInstanceRateZeroDivisor:  VkBool32,
    pub indexTypeUint8:  VkBool32,
    pub dynamicRenderingLocalRead:  VkBool32,
    pub maintenance5:  VkBool32,
    pub maintenance6:  VkBool32,
    pub pipelineProtectedAccess:  VkBool32,
    pub pipelineRobustness:  VkBool32,
    pub hostImageCopy:  VkBool32,
    pub pushDescriptor:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkan14Properties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub lineSubPixelPrecisionBits:  u32,
    pub maxVertexAttribDivisor:  u32,
    pub supportsNonZeroFirstInstance:  VkBool32,
    pub maxPushDescriptors:  u32,
    pub dynamicRenderingLocalReadDepthStencilAttachments:  VkBool32,
    pub dynamicRenderingLocalReadMultisampledAttachments:  VkBool32,
    pub earlyFragmentMultisampleCoverageAfterSampleCounting:  VkBool32,
    pub earlyFragmentSampleMaskTestBeforeSampleCounting:  VkBool32,
    pub depthStencilSwizzleOneSupport:  VkBool32,
    pub polygonModePointSize:  VkBool32,
    pub nonStrictSinglePixelWideLinesUseParallelogram:  VkBool32,
    pub nonStrictWideLinesUseParallelogram:  VkBool32,
    pub blockTexelViewCompatibleMultipleLayers:  VkBool32,
    pub maxCombinedImageSamplerDescriptorCount:  u32,
    pub fragmentShadingRateClampCombinerInputs:  VkBool32,
    pub defaultRobustnessStorageBuffers:  VkPipelineRobustnessBufferBehavior,
    pub defaultRobustnessUniformBuffers:  VkPipelineRobustnessBufferBehavior,
    pub defaultRobustnessVertexInputs:  VkPipelineRobustnessBufferBehavior,
    pub defaultRobustnessImages:  VkPipelineRobustnessImageBehavior,
    pub copySrcLayoutCount:  u32,
    pub pCopySrcLayouts: *mut  VkImageLayout,
    pub copyDstLayoutCount:  u32,
    pub pCopyDstLayouts: *mut  VkImageLayout,
    pub optimalTilingLayoutUUID:  [u8; VK_UUID_SIZE as usize],
    pub identicalMemoryTypeRequirements:  VkBool32,
}

#[repr(C)]
pub struct VkPipelineCompilerControlCreateInfoAMD {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub compilerControlFlags:  VkPipelineCompilerControlFlagsAMD,
}

#[repr(C)]
pub struct VkPhysicalDeviceCoherentMemoryFeaturesAMD {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub deviceCoherentMemory:  VkBool32,
}

#[repr(C)]
pub struct VkGpaPerfBlockPropertiesAMD {
    pub blockType:  VkGpaPerfBlockAMD,
    pub flags:  VkGpaPerfBlockPropertiesFlagsAMD,
    pub instanceCount:  u32,
    pub maxEventID:  u32,
    pub maxGlobalOnlyCounters:  u32,
    pub maxGlobalSharedCounters:  u32,
    pub maxStreamingCounters:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceGpaFeaturesAMD {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub perfCounters:  VkBool32,
    pub streamingPerfCounters:  VkBool32,
    pub sqThreadTracing:  VkBool32,
    pub clockModes:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceGpaPropertiesAMD {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkPhysicalDeviceGpaPropertiesFlagsAMD,
    pub maxSqttSeBufferSize:  VkDeviceSize,
    pub shaderEngineCount:  u32,
    pub perfBlockCount:  u32,
    pub pPerfBlocks: *mut  VkGpaPerfBlockPropertiesAMD,
}

#[repr(C)]
pub struct VkPhysicalDeviceGpaProperties2AMD {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub revisionId:  u32,
}

#[repr(C)]
pub struct VkGpaPerfCounterAMD {
    pub blockType:  VkGpaPerfBlockAMD,
    pub blockInstance:  u32,
    pub eventID:  u32,
}

#[repr(C)]
pub struct VkGpaSampleBeginInfoAMD {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub sampleType:  VkGpaSampleTypeAMD,
    pub sampleInternalOperations:  VkBool32,
    pub cacheFlushOnCounterCollection:  VkBool32,
    pub sqShaderMaskEnable:  VkBool32,
    pub sqShaderMask:  VkGpaSqShaderStageFlagsAMD,
    pub perfCounterCount:  u32,
    pub pPerfCounters: *const  VkGpaPerfCounterAMD,
    pub streamingPerfTraceSampleInterval:  u32,
    pub perfCounterDeviceMemoryLimit:  VkDeviceSize,
    pub sqThreadTraceEnable:  VkBool32,
    pub sqThreadTraceSuppressInstructionTokens:  VkBool32,
    pub sqThreadTraceDeviceMemoryLimit:  VkDeviceSize,
    pub timingPreSample:  VkPipelineStageFlags,
    pub timingPostSample:  VkPipelineStageFlags,
}

#[repr(C)]
pub struct VkGpaDeviceClockModeInfoAMD {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub clockMode:  VkGpaDeviceClockModeAMD,
    pub memoryClockRatioToPeak:  f32,
    pub engineClockRatioToPeak:  f32,
}

#[repr(C)]
pub struct VkGpaDeviceGetClockInfoAMD {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub memoryClockRatioToPeak:  f32,
    pub engineClockRatioToPeak:  f32,
    pub memoryClockFrequency:  u32,
    pub engineClockFrequency:  u32,
}

#[repr(C)]
pub struct VkGpaSessionCreateInfoAMD {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub secondaryCopySource:  VkGpaSessionAMD,
}

#[repr(C)]
pub struct VkFaultData {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub faultLevel:  VkFaultLevel,
    pub faultType:  VkFaultType,
}

#[repr(C)]
pub struct VkFaultCallbackInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub faultCount:  u32,
    pub pFaults: *mut  VkFaultData,
    //pub pfnFaultCallback:  crate::svk_commands::PFN_vkFaultCallbackFunction,
}

#[repr(C)]
pub struct VkPhysicalDeviceToolProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub name:  [i8; VK_MAX_EXTENSION_NAME_SIZE as usize],
    pub version:  [i8; VK_MAX_EXTENSION_NAME_SIZE as usize],
    pub purposes:  VkToolPurposeFlags,
    pub description:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
    pub layer:  [i8; VK_MAX_EXTENSION_NAME_SIZE as usize],
}

#[repr(C)]
pub struct VkPhysicalDeviceToolPropertiesEXT {
}

#[repr(C)]
pub struct VkSamplerCustomBorderColorCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub customBorderColor:  VkClearColorValue,
    pub format:  VkFormat,
}

#[repr(C)]
pub struct VkPhysicalDeviceCustomBorderColorPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxCustomBorderColorSamplers:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCustomBorderColorFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub customBorderColors:  VkBool32,
    pub customBorderColorWithoutFormat:  VkBool32,
}

#[repr(C)]
pub struct VkSamplerBorderColorComponentMappingCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub components:  VkComponentMapping,
    pub srgb:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceBorderColorSwizzleFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub borderColorSwizzle:  VkBool32,
    pub borderColorSwizzleFromImage:  VkBool32,
}

#[repr(C)]
pub struct VkDeviceOrHostAddressKHR {
    pub deviceAddress:  VkDeviceAddress,
    pub hostAddress: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkDeviceOrHostAddressConstKHR {
    pub deviceAddress:  VkDeviceAddress,
    pub hostAddress: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkDeviceOrHostAddressConstAMDX {
    pub deviceAddress:  VkDeviceAddress,
    pub hostAddress: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryTrianglesDataKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub vertexFormat:  VkFormat,
    pub vertexData:  VkDeviceOrHostAddressConstKHR,
    pub vertexStride:  VkDeviceSize,
    pub maxVertex:  u32,
    pub indexType:  VkIndexType,
    pub indexData:  VkDeviceOrHostAddressConstKHR,
    pub transformData:  VkDeviceOrHostAddressConstKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryAabbsDataKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub data:  VkDeviceOrHostAddressConstKHR,
    pub stride:  VkDeviceSize,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryInstancesDataKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub arrayOfPointers:  VkBool32,
    pub data:  VkDeviceOrHostAddressConstKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryLinearSweptSpheresDataNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub vertexFormat:  VkFormat,
    pub vertexData:  VkDeviceOrHostAddressConstKHR,
    pub vertexStride:  VkDeviceSize,
    pub radiusFormat:  VkFormat,
    pub radiusData:  VkDeviceOrHostAddressConstKHR,
    pub radiusStride:  VkDeviceSize,
    pub indexType:  VkIndexType,
    pub indexData:  VkDeviceOrHostAddressConstKHR,
    pub indexStride:  VkDeviceSize,
    pub indexingMode:  VkRayTracingLssIndexingModeNV,
    pub endCapsMode:  VkRayTracingLssPrimitiveEndCapsModeNV,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometrySpheresDataNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub vertexFormat:  VkFormat,
    pub vertexData:  VkDeviceOrHostAddressConstKHR,
    pub vertexStride:  VkDeviceSize,
    pub radiusFormat:  VkFormat,
    pub radiusData:  VkDeviceOrHostAddressConstKHR,
    pub radiusStride:  VkDeviceSize,
    pub indexType:  VkIndexType,
    pub indexData:  VkDeviceOrHostAddressConstKHR,
    pub indexStride:  VkDeviceSize,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryDataKHR {
    pub triangles:  VkAccelerationStructureGeometryTrianglesDataKHR,
    pub aabbs:  VkAccelerationStructureGeometryAabbsDataKHR,
    pub instances:  VkAccelerationStructureGeometryInstancesDataKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub geometryType:  VkGeometryTypeKHR,
    pub geometry:  VkAccelerationStructureGeometryDataKHR,
    pub flags:  VkGeometryFlagsKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureBuildGeometryInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub r#type:  VkAccelerationStructureTypeKHR,
    pub flags:  VkBuildAccelerationStructureFlagsKHR,
    pub mode:  VkBuildAccelerationStructureModeKHR,
    pub srcAccelerationStructure:  VkAccelerationStructureKHR,
    pub dstAccelerationStructure:  VkAccelerationStructureKHR,
    pub geometryCount:  u32,
    pub pGeometries: *const  VkAccelerationStructureGeometryKHR,
    pub ppGeometries: *const  VkAccelerationStructureGeometryKHR,
    pub scratchData:  VkDeviceOrHostAddressKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureBuildRangeInfoKHR {
    pub primitiveCount:  u32,
    pub primitiveOffset:  u32,
    pub firstVertex:  u32,
    pub transformOffset:  u32,
}

#[repr(C)]
pub struct VkAccelerationStructureCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub createFlags:  VkAccelerationStructureCreateFlagsKHR,
    pub buffer:  VkBuffer,
    pub offset:  VkDeviceSize,
    pub size:  VkDeviceSize,
    pub r#type:  VkAccelerationStructureTypeKHR,
    pub deviceAddress:  VkDeviceAddress,
}

#[repr(C)]
pub struct VkAabbPositionsKHR {
    pub minX:  f32,
    pub minY:  f32,
    pub minZ:  f32,
    pub maxX:  f32,
    pub maxY:  f32,
    pub maxZ:  f32,
}

#[repr(C)]
pub struct VkAabbPositionsNV {
}

#[repr(C)]
pub struct VkTransformMatrixKHR {
    pub matrix:  f32,
}

#[repr(C)]
pub struct VkTransformMatrixNV {
}

#[repr(C)]
pub struct VkAccelerationStructureInstanceKHR {
    pub transform:  VkTransformMatrixKHR,
    pub instanceCustomIndex:  u32,
    pub mask:  u32,
    pub instanceShaderBindingTableRecordOffset:  u32,
    pub flags:  VkGeometryInstanceFlagsKHR,
    pub accelerationStructureReference:  u64,
}

#[repr(C)]
pub struct VkAccelerationStructureInstanceNV {
}

#[repr(C)]
pub struct VkAccelerationStructureDeviceAddressInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub accelerationStructure:  VkAccelerationStructureKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureVersionInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pVersionData: *const  u8,
}

#[repr(C)]
pub struct VkCopyAccelerationStructureInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub src:  VkAccelerationStructureKHR,
    pub dst:  VkAccelerationStructureKHR,
    pub mode:  VkCopyAccelerationStructureModeKHR,
}

#[repr(C)]
pub struct VkCopyAccelerationStructureToMemoryInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub src:  VkAccelerationStructureKHR,
    pub dst:  VkDeviceOrHostAddressKHR,
    pub mode:  VkCopyAccelerationStructureModeKHR,
}

#[repr(C)]
pub struct VkCopyMemoryToAccelerationStructureInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub src:  VkDeviceOrHostAddressConstKHR,
    pub dst:  VkAccelerationStructureKHR,
    pub mode:  VkCopyAccelerationStructureModeKHR,
}

#[repr(C)]
pub struct VkRayTracingPipelineInterfaceCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub maxPipelineRayPayloadSize:  u32,
    pub maxPipelineRayHitAttributeSize:  u32,
}

#[repr(C)]
pub struct VkPipelineLibraryCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub libraryCount:  u32,
    pub pLibraries: *const  VkPipeline,
}

#[repr(C)]
pub struct VkRefreshObjectKHR {
    pub objectType:  VkObjectType,
    pub objectHandle:  u64,
    pub flags:  VkRefreshObjectFlagsKHR,
}

#[repr(C)]
pub struct VkRefreshObjectListKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub objectCount:  u32,
    pub pObjects: *const  VkRefreshObjectKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicStateFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub extendedDynamicState:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState2FeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub extendedDynamicState2:  VkBool32,
    pub extendedDynamicState2LogicOp:  VkBool32,
    pub extendedDynamicState2PatchControlPoints:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState3FeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub extendedDynamicState3TessellationDomainOrigin:  VkBool32,
    pub extendedDynamicState3DepthClampEnable:  VkBool32,
    pub extendedDynamicState3PolygonMode:  VkBool32,
    pub extendedDynamicState3RasterizationSamples:  VkBool32,
    pub extendedDynamicState3SampleMask:  VkBool32,
    pub extendedDynamicState3AlphaToCoverageEnable:  VkBool32,
    pub extendedDynamicState3AlphaToOneEnable:  VkBool32,
    pub extendedDynamicState3LogicOpEnable:  VkBool32,
    pub extendedDynamicState3ColorBlendEnable:  VkBool32,
    pub extendedDynamicState3ColorBlendEquation:  VkBool32,
    pub extendedDynamicState3ColorWriteMask:  VkBool32,
    pub extendedDynamicState3RasterizationStream:  VkBool32,
    pub extendedDynamicState3ConservativeRasterizationMode:  VkBool32,
    pub extendedDynamicState3ExtraPrimitiveOverestimationSize:  VkBool32,
    pub extendedDynamicState3DepthClipEnable:  VkBool32,
    pub extendedDynamicState3SampleLocationsEnable:  VkBool32,
    pub extendedDynamicState3ColorBlendAdvanced:  VkBool32,
    pub extendedDynamicState3ProvokingVertexMode:  VkBool32,
    pub extendedDynamicState3LineRasterizationMode:  VkBool32,
    pub extendedDynamicState3LineStippleEnable:  VkBool32,
    pub extendedDynamicState3DepthClipNegativeOneToOne:  VkBool32,
    pub extendedDynamicState3ViewportWScalingEnable:  VkBool32,
    pub extendedDynamicState3ViewportSwizzle:  VkBool32,
    pub extendedDynamicState3CoverageToColorEnable:  VkBool32,
    pub extendedDynamicState3CoverageToColorLocation:  VkBool32,
    pub extendedDynamicState3CoverageModulationMode:  VkBool32,
    pub extendedDynamicState3CoverageModulationTableEnable:  VkBool32,
    pub extendedDynamicState3CoverageModulationTable:  VkBool32,
    pub extendedDynamicState3CoverageReductionMode:  VkBool32,
    pub extendedDynamicState3RepresentativeFragmentTestEnable:  VkBool32,
    pub extendedDynamicState3ShadingRateImageEnable:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedDynamicState3PropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub dynamicPrimitiveTopologyUnrestricted:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedFlagsFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub extendedFlags:  VkBool32,
}

#[repr(C)]
pub struct VkColorBlendEquationEXT {
    pub srcColorBlendFactor:  VkBlendFactor,
    pub dstColorBlendFactor:  VkBlendFactor,
    pub colorBlendOp:  VkBlendOp,
    pub srcAlphaBlendFactor:  VkBlendFactor,
    pub dstAlphaBlendFactor:  VkBlendFactor,
    pub alphaBlendOp:  VkBlendOp,
}

#[repr(C)]
pub struct VkColorBlendAdvancedEXT {
    pub advancedBlendOp:  VkBlendOp,
    pub srcPremultiplied:  VkBool32,
    pub dstPremultiplied:  VkBool32,
    pub blendOverlap:  VkBlendOverlapEXT,
    pub clampResults:  VkBool32,
}

#[repr(C)]
pub struct VkRenderPassTransformBeginInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub transform:  VkSurfaceTransformFlagBitsKHR,
}

#[repr(C)]
pub struct VkCopyCommandTransformInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub transform:  VkSurfaceTransformFlagBitsKHR,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceRenderPassTransformInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub transform:  VkSurfaceTransformFlagBitsKHR,
    pub renderArea:  VkRect2D,
}

#[repr(C)]
pub struct VkPhysicalDevicePartitionedAccelerationStructureFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub partitionedAccelerationStructure:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePartitionedAccelerationStructurePropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxPartitionCount:  u32,
}

#[repr(C)]
pub struct VkBuildPartitionedAccelerationStructureIndirectCommandNV {
    pub opType:  VkPartitionedAccelerationStructureOpTypeNV,
    pub argCount:  u32,
    pub argData:  VkStridedDeviceAddressNV,
}

#[repr(C)]
pub struct VkPartitionedAccelerationStructureFlagsNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub enablePartitionTranslation:  VkBool32,
}

#[repr(C)]
pub struct VkPartitionedAccelerationStructureWriteInstanceDataNV {
    pub transform:  VkTransformMatrixKHR,
    pub explicitAABB:  f32,
    pub instanceID:  u32,
    pub instanceMask:  u32,
    pub instanceContributionToHitGroupIndex:  u32,
    pub instanceFlags:  VkPartitionedAccelerationStructureInstanceFlagsNV,
    pub instanceIndex:  u32,
    pub partitionIndex:  u32,
    pub accelerationStructure:  VkDeviceAddress,
}

#[repr(C)]
pub struct VkPartitionedAccelerationStructureUpdateInstanceDataNV {
    pub instanceIndex:  u32,
    pub instanceContributionToHitGroupIndex:  u32,
    pub accelerationStructure:  VkDeviceAddress,
}

#[repr(C)]
pub struct VkPartitionedAccelerationStructureWritePartitionTranslationDataNV {
    pub partitionIndex:  u32,
    pub partitionTranslation:  f32,
}

#[repr(C)]
pub struct VkWriteDescriptorSetPartitionedAccelerationStructureNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub accelerationStructureCount:  u32,
    pub pAccelerationStructures: *const  VkDeviceAddress,
}

#[repr(C)]
pub struct VkPartitionedAccelerationStructureInstancesInputNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkBuildAccelerationStructureFlagsKHR,
    pub instanceCount:  u32,
    pub maxInstancePerPartitionCount:  u32,
    pub partitionCount:  u32,
    pub maxInstanceInGlobalPartitionCount:  u32,
}

#[repr(C)]
pub struct VkBuildPartitionedAccelerationStructureInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub input:  VkPartitionedAccelerationStructureInstancesInputNV,
    pub srcAccelerationStructureData:  VkDeviceAddress,
    pub dstAccelerationStructureData:  VkDeviceAddress,
    pub scratchData:  VkDeviceAddress,
    pub srcInfos:  VkDeviceAddress,
    pub srcInfosCount:  VkDeviceAddress,
}

#[repr(C)]
pub struct VkPhysicalDeviceDiagnosticsConfigFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub diagnosticsConfig:  VkBool32,
}

#[repr(C)]
pub struct VkDeviceDiagnosticsConfigCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkDeviceDiagnosticsConfigFlagsNV,
}

#[repr(C)]
pub struct VkPipelineOfflineCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pipelineIdentifier:  [u8; VK_UUID_SIZE as usize],
    pub matchControl:  VkPipelineMatchControl,
    pub poolEntrySize:  VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderZeroInitializeWorkgroupMemory:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceZeroInitializeWorkgroupMemoryFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderSubgroupUniformControlFlow:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRobustness2FeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub robustBufferAccess2:  VkBool32,
    pub robustImageAccess2:  VkBool32,
    pub nullDescriptor:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRobustness2FeaturesEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceRobustness2PropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub robustStorageBufferAccessSizeAlignment:  VkDeviceSize,
    pub robustUniformBufferAccessSizeAlignment:  VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceRobustness2PropertiesEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceImageRobustnessFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub robustImageAccess:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageRobustnessFeaturesEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub workgroupMemoryExplicitLayout:  VkBool32,
    pub workgroupMemoryExplicitLayoutScalarBlockLayout:  VkBool32,
    pub workgroupMemoryExplicitLayout8BitAccess:  VkBool32,
    pub workgroupMemoryExplicitLayout16BitAccess:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePortabilitySubsetFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub constantAlphaColorBlendFactors:  VkBool32,
    pub events:  VkBool32,
    pub imageViewFormatReinterpretation:  VkBool32,
    pub imageViewFormatSwizzle:  VkBool32,
    pub imageView2DOn3DImage:  VkBool32,
    pub multisampleArrayImage:  VkBool32,
    pub mutableComparisonSamplers:  VkBool32,
    pub pointPolygons:  VkBool32,
    pub samplerMipLodBias:  VkBool32,
    pub separateStencilMaskRef:  VkBool32,
    pub shaderSampleRateInterpolationFunctions:  VkBool32,
    pub tessellationIsolines:  VkBool32,
    pub tessellationPointMode:  VkBool32,
    pub triangleFans:  VkBool32,
    pub vertexAttributeAccessBeyondStride:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePortabilitySubsetPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub minVertexInputBindingStrideAlignment:  u32,
}

#[repr(C)]
pub struct VkPhysicalDevice4444FormatsFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub formatA4R4G4B4:  VkBool32,
    pub formatA4B4G4R4:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubpassShadingFeaturesHUAWEI {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub subpassShading:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceClusterCullingShaderFeaturesHUAWEI {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub clustercullingShader:  VkBool32,
    pub multiviewClusterCullingShader:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub clusterShadingRate:  VkBool32,
}

#[repr(C)]
pub struct VkBufferCopy2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcOffset:  VkDeviceSize,
    pub dstOffset:  VkDeviceSize,
    pub size:  VkDeviceSize,
}

#[repr(C)]
pub struct VkBufferCopy2KHR {
}

#[repr(C)]
pub struct VkImageCopy2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcSubresource:  VkImageSubresourceLayers,
    pub srcOffset:  VkOffset3D,
    pub dstSubresource:  VkImageSubresourceLayers,
    pub dstOffset:  VkOffset3D,
    pub extent:  VkExtent3D,
}

#[repr(C)]
pub struct VkImageCopy2KHR {
}

#[repr(C)]
pub struct VkImageBlit2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcSubresource:  VkImageSubresourceLayers,
    pub srcOffsets:  VkOffset3D,
    pub dstSubresource:  VkImageSubresourceLayers,
    pub dstOffsets:  VkOffset3D,
}

#[repr(C)]
pub struct VkImageBlit2KHR {
}

#[repr(C)]
pub struct VkBufferImageCopy2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub bufferOffset:  VkDeviceSize,
    pub bufferRowLength:  u32,
    pub bufferImageHeight:  u32,
    pub imageSubresource:  VkImageSubresourceLayers,
    pub imageOffset:  VkOffset3D,
    pub imageExtent:  VkExtent3D,
}

#[repr(C)]
pub struct VkBufferImageCopy2KHR {
}

#[repr(C)]
pub struct VkImageResolve2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcSubresource:  VkImageSubresourceLayers,
    pub srcOffset:  VkOffset3D,
    pub dstSubresource:  VkImageSubresourceLayers,
    pub dstOffset:  VkOffset3D,
    pub extent:  VkExtent3D,
}

#[repr(C)]
pub struct VkImageResolve2KHR {
}

#[repr(C)]
pub struct VkCopyBufferInfo2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcBuffer:  VkBuffer,
    pub dstBuffer:  VkBuffer,
    pub regionCount:  u32,
    pub pRegions: *const  VkBufferCopy2,
}

#[repr(C)]
pub struct VkCopyBufferInfo2KHR {
}

#[repr(C)]
pub struct VkCopyImageInfo2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcImage:  VkImage,
    pub srcImageLayout:  VkImageLayout,
    pub dstImage:  VkImage,
    pub dstImageLayout:  VkImageLayout,
    pub regionCount:  u32,
    pub pRegions: *const  VkImageCopy2,
}

#[repr(C)]
pub struct VkCopyImageInfo2KHR {
}

#[repr(C)]
pub struct VkBlitImageInfo2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcImage:  VkImage,
    pub srcImageLayout:  VkImageLayout,
    pub dstImage:  VkImage,
    pub dstImageLayout:  VkImageLayout,
    pub regionCount:  u32,
    pub pRegions: *const  VkImageBlit2,
    pub filter:  VkFilter,
}

#[repr(C)]
pub struct VkBlitImageInfo2KHR {
}

#[repr(C)]
pub struct VkCopyBufferToImageInfo2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcBuffer:  VkBuffer,
    pub dstImage:  VkImage,
    pub dstImageLayout:  VkImageLayout,
    pub regionCount:  u32,
    pub pRegions: *const  VkBufferImageCopy2,
}

#[repr(C)]
pub struct VkCopyBufferToImageInfo2KHR {
}

#[repr(C)]
pub struct VkCopyImageToBufferInfo2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcImage:  VkImage,
    pub srcImageLayout:  VkImageLayout,
    pub dstBuffer:  VkBuffer,
    pub regionCount:  u32,
    pub pRegions: *const  VkBufferImageCopy2,
}

#[repr(C)]
pub struct VkCopyImageToBufferInfo2KHR {
}

#[repr(C)]
pub struct VkResolveImageInfo2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcImage:  VkImage,
    pub srcImageLayout:  VkImageLayout,
    pub dstImage:  VkImage,
    pub dstImageLayout:  VkImageLayout,
    pub regionCount:  u32,
    pub pRegions: *const  VkImageResolve2,
}

#[repr(C)]
pub struct VkResolveImageInfo2KHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderImageAtomicInt64FeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderImageInt64Atomics:  VkBool32,
    pub sparseImageInt64Atomics:  VkBool32,
}

#[repr(C)]
pub struct VkFragmentShadingRateAttachmentInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pFragmentShadingRateAttachment: *const  VkAttachmentReference2,
    pub shadingRateAttachmentTexelSize:  VkExtent2D,
}

#[repr(C)]
pub struct VkPipelineFragmentShadingRateStateCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub fragmentSize:  VkExtent2D,
    pub combinerOps:  VkFragmentShadingRateCombinerOpKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pipelineFragmentShadingRate:  VkBool32,
    pub primitiveFragmentShadingRate:  VkBool32,
    pub attachmentFragmentShadingRate:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRatePropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub minFragmentShadingRateAttachmentTexelSize:  VkExtent2D,
    pub maxFragmentShadingRateAttachmentTexelSize:  VkExtent2D,
    pub maxFragmentShadingRateAttachmentTexelSizeAspectRatio:  u32,
    pub primitiveFragmentShadingRateWithMultipleViewports:  VkBool32,
    pub layeredShadingRateAttachments:  VkBool32,
    pub fragmentShadingRateNonTrivialCombinerOps:  VkBool32,
    pub maxFragmentSize:  VkExtent2D,
    pub maxFragmentSizeAspectRatio:  u32,
    pub maxFragmentShadingRateCoverageSamples:  u32,
    pub maxFragmentShadingRateRasterizationSamples:  VkSampleCountFlagBits,
    pub fragmentShadingRateWithShaderDepthStencilWrites:  VkBool32,
    pub fragmentShadingRateWithSampleMask:  VkBool32,
    pub fragmentShadingRateWithShaderSampleMask:  VkBool32,
    pub fragmentShadingRateWithConservativeRasterization:  VkBool32,
    pub fragmentShadingRateWithFragmentShaderInterlock:  VkBool32,
    pub fragmentShadingRateWithCustomSampleLocations:  VkBool32,
    pub fragmentShadingRateStrictMultiplyCombiner:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub sampleCounts:  VkSampleCountFlags,
    pub fragmentSize:  VkExtent2D,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderTerminateInvocationFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderTerminateInvocation:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderTerminateInvocationFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateEnumsFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub fragmentShadingRateEnums:  VkBool32,
    pub supersampleFragmentShadingRates:  VkBool32,
    pub noInvocationFragmentShadingRates:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShadingRateEnumsPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxFragmentShadingRateInvocationCount:  VkSampleCountFlagBits,
}

#[repr(C)]
pub struct VkPipelineFragmentShadingRateEnumStateCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub shadingRateType:  VkFragmentShadingRateTypeNV,
    pub shadingRate:  VkFragmentShadingRateNV,
    pub combinerOps:  VkFragmentShadingRateCombinerOpKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureBuildSizesInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub accelerationStructureSize:  VkDeviceSize,
    pub updateScratchSize:  VkDeviceSize,
    pub buildScratchSize:  VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceImage2DViewOf3DFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub image2DViewOf3D:  VkBool32,
    pub sampler2DViewOf3D:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageSlicedViewOf3DFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub imageSlicedViewOf3D:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub attachmentFeedbackLoopDynamicState:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceLegacyVertexAttributesFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub legacyVertexAttributes:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceLegacyVertexAttributesPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub nativeUnalignedPerformance:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMutableDescriptorTypeFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub mutableDescriptorType:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMutableDescriptorTypeFeaturesVALVE {
}

#[repr(C)]
pub struct VkMutableDescriptorTypeListEXT {
    pub descriptorTypeCount:  u32,
    pub pDescriptorTypes: *const  VkDescriptorType,
}

#[repr(C)]
pub struct VkMutableDescriptorTypeListVALVE {
}

#[repr(C)]
pub struct VkMutableDescriptorTypeCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub mutableDescriptorTypeListCount:  u32,
    pub pMutableDescriptorTypeLists: *const  VkMutableDescriptorTypeListEXT,
}

#[repr(C)]
pub struct VkMutableDescriptorTypeCreateInfoVALVE {
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthClipControlFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub depthClipControl:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub zeroInitializeDeviceMemory:  VkBool32,
}

#[repr(C)]
pub struct VkBeginCustomResolveInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceCustomResolveFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub customResolve:  VkBool32,
}

#[repr(C)]
pub struct VkCustomResolveCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub customResolve:  VkBool32,
    pub colorAttachmentCount:  u32,
    pub pColorAttachmentFormats: *const  VkFormat,
    pub depthAttachmentFormat:  VkFormat,
    pub stencilAttachmentFormat:  VkFormat,
}

#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub deviceGeneratedCommands:  VkBool32,
    pub dynamicGeneratedPipelineLayout:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDeviceGeneratedCommandsPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxIndirectPipelineCount:  u32,
    pub maxIndirectShaderObjectCount:  u32,
    pub maxIndirectSequenceCount:  u32,
    pub maxIndirectCommandsTokenCount:  u32,
    pub maxIndirectCommandsTokenOffset:  u32,
    pub maxIndirectCommandsIndirectStride:  u32,
    pub supportedIndirectCommandsInputModes:  VkIndirectCommandsInputModeFlagsEXT,
    pub supportedIndirectCommandsShaderStages:  VkShaderStageFlags,
    pub supportedIndirectCommandsShaderStagesPipelineBinding:  VkShaderStageFlags,
    pub supportedIndirectCommandsShaderStagesShaderBinding:  VkShaderStageFlags,
    pub deviceGeneratedCommandsTransformFeedback:  VkBool32,
    pub deviceGeneratedCommandsMultiDrawIndirectCount:  VkBool32,
}

#[repr(C)]
pub struct VkGeneratedCommandsPipelineInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pipeline:  VkPipeline,
}

#[repr(C)]
pub struct VkGeneratedCommandsShaderInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderCount:  u32,
    pub pShaders: *const  VkShaderEXT,
}

#[repr(C)]
pub struct VkGeneratedCommandsMemoryRequirementsInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub indirectExecutionSet:  VkIndirectExecutionSetEXT,
    pub indirectCommandsLayout:  VkIndirectCommandsLayoutEXT,
    pub maxSequenceCount:  u32,
    pub maxDrawCount:  u32,
}

#[repr(C)]
pub struct VkIndirectExecutionSetPipelineInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub initialPipeline:  VkPipeline,
    pub maxPipelineCount:  u32,
}

#[repr(C)]
pub struct VkIndirectExecutionSetShaderLayoutInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub setLayoutCount:  u32,
    pub pSetLayouts: *const  VkDescriptorSetLayout,
}

#[repr(C)]
pub struct VkIndirectExecutionSetShaderInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub shaderCount:  u32,
    pub pInitialShaders: *const  VkShaderEXT,
    pub pSetLayoutInfos: *const  VkIndirectExecutionSetShaderLayoutInfoEXT,
    pub maxShaderCount:  u32,
    pub pushConstantRangeCount:  u32,
    pub pPushConstantRanges: *const  VkPushConstantRange,
}

#[repr(C)]
pub struct VkIndirectExecutionSetInfoEXT {
    pub pPipelineInfo: *const  VkIndirectExecutionSetPipelineInfoEXT,
    pub pShaderInfo: *const  VkIndirectExecutionSetShaderInfoEXT,
}

#[repr(C)]
pub struct VkIndirectExecutionSetCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub r#type:  VkIndirectExecutionSetInfoTypeEXT,
    pub info:  VkIndirectExecutionSetInfoEXT,
}

#[repr(C)]
pub struct VkGeneratedCommandsInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub shaderStages:  VkShaderStageFlags,
    pub indirectExecutionSet:  VkIndirectExecutionSetEXT,
    pub indirectCommandsLayout:  VkIndirectCommandsLayoutEXT,
    pub indirectAddress:  VkDeviceAddress,
    pub indirectAddressSize:  VkDeviceSize,
    pub preprocessAddress:  VkDeviceAddress,
    pub preprocessSize:  VkDeviceSize,
    pub maxSequenceCount:  u32,
    pub sequenceCountAddress:  VkDeviceAddress,
    pub maxDrawCount:  u32,
}

#[repr(C)]
pub struct VkWriteIndirectExecutionSetPipelineEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub index:  u32,
    pub pipeline:  VkPipeline,
}

#[repr(C)]
pub struct VkWriteIndirectExecutionSetShaderEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub index:  u32,
    pub shader:  VkShaderEXT,
}

#[repr(C)]
pub struct VkIndirectCommandsLayoutCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkIndirectCommandsLayoutUsageFlagsEXT,
    pub shaderStages:  VkShaderStageFlags,
    pub indirectStride:  u32,
    pub pipelineLayout:  VkPipelineLayout,
    pub tokenCount:  u32,
    pub pTokens: *const  VkIndirectCommandsLayoutTokenEXT,
}

#[repr(C)]
pub struct VkIndirectCommandsLayoutTokenEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub r#type:  VkIndirectCommandsTokenTypeEXT,
    pub data:  VkIndirectCommandsTokenDataEXT,
    pub offset:  u32,
}

#[repr(C)]
pub struct VkDrawIndirectCountIndirectCommandEXT {
    pub bufferAddress:  VkDeviceAddress,
    pub stride:  u32,
    pub commandCount:  u32,
}

#[repr(C)]
pub struct VkIndirectCommandsVertexBufferTokenEXT {
    pub vertexBindingUnit:  u32,
}

#[repr(C)]
pub struct VkBindVertexBufferIndirectCommandEXT {
    pub bufferAddress:  VkDeviceAddress,
    pub size:  u32,
    pub stride:  u32,
}

#[repr(C)]
pub struct VkIndirectCommandsIndexBufferTokenEXT {
    pub mode:  VkIndirectCommandsInputModeFlagBitsEXT,
}

#[repr(C)]
pub struct VkBindIndexBufferIndirectCommandEXT {
    pub bufferAddress:  VkDeviceAddress,
    pub size:  u32,
    pub indexType:  VkIndexType,
}

#[repr(C)]
pub struct VkIndirectCommandsPushConstantTokenEXT {
    pub updateRange:  VkPushConstantRange,
}

#[repr(C)]
pub struct VkIndirectCommandsExecutionSetTokenEXT {
    pub r#type:  VkIndirectExecutionSetInfoTypeEXT,
    pub shaderStages:  VkShaderStageFlags,
}

#[repr(C)]
pub struct VkIndirectCommandsTokenDataEXT {
    pub pPushConstant: *const  VkIndirectCommandsPushConstantTokenEXT,
    pub pVertexBuffer: *const  VkIndirectCommandsVertexBufferTokenEXT,
    pub pIndexBuffer: *const  VkIndirectCommandsIndexBufferTokenEXT,
    pub pExecutionSet: *const  VkIndirectCommandsExecutionSetTokenEXT,
}

#[repr(C)]
pub struct VkPipelineViewportDepthClipControlCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub negativeOneToOne:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthClampControlFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub depthClampControl:  VkBool32,
}

#[repr(C)]
pub struct VkPipelineViewportDepthClampControlCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub depthClampMode:  VkDepthClampModeEXT,
    pub pDepthClampRange: *const  VkDepthClampRangeEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceVertexInputDynamicStateFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub vertexInputDynamicState:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalMemoryRDMAFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub externalMemoryRDMA:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderRelaxedExtendedInstruction:  VkBool32,
}

#[repr(C)]
pub struct VkVertexInputBindingDescription2EXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub binding:  u32,
    pub stride:  u32,
    pub inputRate:  VkVertexInputRate,
    pub divisor:  u32,
}

#[repr(C)]
pub struct VkVertexInputAttributeDescription2EXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub location:  u32,
    pub binding:  u32,
    pub format:  VkFormat,
    pub offset:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceColorWriteEnableFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub colorWriteEnable:  VkBool32,
}

#[repr(C)]
pub struct VkPipelineColorWriteCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub attachmentCount:  u32,
    pub pColorWriteEnables: *const  VkBool32,
}

#[repr(C)]
pub struct VkMemoryBarrier2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcStageMask:  VkPipelineStageFlags2,
    pub srcAccessMask:  VkAccessFlags2,
    pub dstStageMask:  VkPipelineStageFlags2,
    pub dstAccessMask:  VkAccessFlags2,
}

#[repr(C)]
pub struct VkMemoryBarrier2KHR {
}

#[repr(C)]
pub struct VkImageMemoryBarrier2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcStageMask:  VkPipelineStageFlags2,
    pub srcAccessMask:  VkAccessFlags2,
    pub dstStageMask:  VkPipelineStageFlags2,
    pub dstAccessMask:  VkAccessFlags2,
    pub oldLayout:  VkImageLayout,
    pub newLayout:  VkImageLayout,
    pub srcQueueFamilyIndex:  u32,
    pub dstQueueFamilyIndex:  u32,
    pub image:  VkImage,
    pub subresourceRange:  VkImageSubresourceRange,
}

#[repr(C)]
pub struct VkImageMemoryBarrier2KHR {
}

#[repr(C)]
pub struct VkBufferMemoryBarrier2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcStageMask:  VkPipelineStageFlags2,
    pub srcAccessMask:  VkAccessFlags2,
    pub dstStageMask:  VkPipelineStageFlags2,
    pub dstAccessMask:  VkAccessFlags2,
    pub srcQueueFamilyIndex:  u32,
    pub dstQueueFamilyIndex:  u32,
    pub buffer:  VkBuffer,
    pub offset:  VkDeviceSize,
    pub size:  VkDeviceSize,
}

#[repr(C)]
pub struct VkBufferMemoryBarrier2KHR {
}

#[repr(C)]
pub struct VkMemoryBarrierAccessFlags3KHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcAccessMask3:  VkAccessFlags3KHR,
    pub dstAccessMask3:  VkAccessFlags3KHR,
}

#[repr(C)]
pub struct VkDependencyInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub dependencyFlags:  VkDependencyFlags,
    pub memoryBarrierCount:  u32,
    pub pMemoryBarriers: *const  VkMemoryBarrier2,
    pub bufferMemoryBarrierCount:  u32,
    pub pBufferMemoryBarriers: *const  VkBufferMemoryBarrier2,
    pub imageMemoryBarrierCount:  u32,
    pub pImageMemoryBarriers: *const  VkImageMemoryBarrier2,
}

#[repr(C)]
pub struct VkDependencyInfoKHR {
}

#[repr(C)]
pub struct VkSemaphoreSubmitInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub semaphore:  VkSemaphore,
    pub value:  u64,
    pub stageMask:  VkPipelineStageFlags2,
    pub deviceIndex:  u32,
}

#[repr(C)]
pub struct VkSemaphoreSubmitInfoKHR {
}

#[repr(C)]
pub struct VkCommandBufferSubmitInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub commandBuffer:  VkCommandBuffer,
    pub deviceMask:  u32,
}

#[repr(C)]
pub struct VkCommandBufferSubmitInfoKHR {
}

#[repr(C)]
pub struct VkSubmitInfo2 {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkSubmitFlags,
    pub waitSemaphoreInfoCount:  u32,
    pub pWaitSemaphoreInfos: *const  VkSemaphoreSubmitInfo,
    pub commandBufferInfoCount:  u32,
    pub pCommandBufferInfos: *const  VkCommandBufferSubmitInfo,
    pub signalSemaphoreInfoCount:  u32,
    pub pSignalSemaphoreInfos: *const  VkSemaphoreSubmitInfo,
}

#[repr(C)]
pub struct VkSubmitInfo2KHR {
}

#[repr(C)]
pub struct VkQueueFamilyCheckpointProperties2NV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub checkpointExecutionStageMask:  VkPipelineStageFlags2,
}

#[repr(C)]
pub struct VkCheckpointData2NV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub stage:  VkPipelineStageFlags2,
    pub pCheckpointMarker: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceSynchronization2Features {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub synchronization2:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSynchronization2FeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceUnifiedImageLayoutsFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub unifiedImageLayouts:  VkBool32,
    pub unifiedImageLayoutsVideo:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceHostImageCopyFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub hostImageCopy:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceHostImageCopyFeaturesEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceHostImageCopyProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub copySrcLayoutCount:  u32,
    pub pCopySrcLayouts: *mut  VkImageLayout,
    pub copyDstLayoutCount:  u32,
    pub pCopyDstLayouts: *mut  VkImageLayout,
    pub optimalTilingLayoutUUID:  [u8; VK_UUID_SIZE as usize],
    pub identicalMemoryTypeRequirements:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceHostImageCopyPropertiesEXT {
}

#[repr(C)]
pub struct VkMemoryToImageCopy {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pHostPointer: *const  std::ffi::c_void,
    pub memoryRowLength:  u32,
    pub memoryImageHeight:  u32,
    pub imageSubresource:  VkImageSubresourceLayers,
    pub imageOffset:  VkOffset3D,
    pub imageExtent:  VkExtent3D,
}

#[repr(C)]
pub struct VkMemoryToImageCopyEXT {
}

#[repr(C)]
pub struct VkImageToMemoryCopy {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pHostPointer: *mut  std::ffi::c_void,
    pub memoryRowLength:  u32,
    pub memoryImageHeight:  u32,
    pub imageSubresource:  VkImageSubresourceLayers,
    pub imageOffset:  VkOffset3D,
    pub imageExtent:  VkExtent3D,
}

#[repr(C)]
pub struct VkImageToMemoryCopyEXT {
}

#[repr(C)]
pub struct VkCopyMemoryToImageInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkHostImageCopyFlags,
    pub dstImage:  VkImage,
    pub dstImageLayout:  VkImageLayout,
    pub regionCount:  u32,
    pub pRegions: *const  VkMemoryToImageCopy,
}

#[repr(C)]
pub struct VkCopyMemoryToImageInfoEXT {
}

#[repr(C)]
pub struct VkCopyImageToMemoryInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkHostImageCopyFlags,
    pub srcImage:  VkImage,
    pub srcImageLayout:  VkImageLayout,
    pub regionCount:  u32,
    pub pRegions: *const  VkImageToMemoryCopy,
}

#[repr(C)]
pub struct VkCopyImageToMemoryInfoEXT {
}

#[repr(C)]
pub struct VkCopyImageToImageInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkHostImageCopyFlags,
    pub srcImage:  VkImage,
    pub srcImageLayout:  VkImageLayout,
    pub dstImage:  VkImage,
    pub dstImageLayout:  VkImageLayout,
    pub regionCount:  u32,
    pub pRegions: *const  VkImageCopy2,
}

#[repr(C)]
pub struct VkCopyImageToImageInfoEXT {
}

#[repr(C)]
pub struct VkHostImageLayoutTransitionInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub image:  VkImage,
    pub oldLayout:  VkImageLayout,
    pub newLayout:  VkImageLayout,
    pub subresourceRange:  VkImageSubresourceRange,
}

#[repr(C)]
pub struct VkHostImageLayoutTransitionInfoEXT {
}

#[repr(C)]
pub struct VkSubresourceHostMemcpySize {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub size:  VkDeviceSize,
}

#[repr(C)]
pub struct VkSubresourceHostMemcpySizeEXT {
}

#[repr(C)]
pub struct VkHostImageCopyDevicePerformanceQuery {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub optimalDeviceAccess:  VkBool32,
    pub identicalMemoryLayout:  VkBool32,
}

#[repr(C)]
pub struct VkHostImageCopyDevicePerformanceQueryEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkanSC10Properties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub deviceNoDynamicHostAllocations:  VkBool32,
    pub deviceDestroyFreesMemory:  VkBool32,
    pub commandPoolMultipleCommandBuffersRecording:  VkBool32,
    pub commandPoolResetCommandBuffer:  VkBool32,
    pub commandBufferSimultaneousUse:  VkBool32,
    pub secondaryCommandBufferNullOrImagelessFramebuffer:  VkBool32,
    pub recycleDescriptorSetMemory:  VkBool32,
    pub recyclePipelineMemory:  VkBool32,
    pub maxRenderPassSubpasses:  u32,
    pub maxRenderPassDependencies:  u32,
    pub maxSubpassInputAttachments:  u32,
    pub maxSubpassPreserveAttachments:  u32,
    pub maxFramebufferAttachments:  u32,
    pub maxDescriptorSetLayoutBindings:  u32,
    pub maxQueryFaultCount:  u32,
    pub maxCallbackFaultCount:  u32,
    pub maxCommandPoolCommandBuffers:  u32,
    pub maxCommandBufferSize:  VkDeviceSize,
}

#[repr(C)]
pub struct VkPipelinePoolSize {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub poolEntrySize:  VkDeviceSize,
    pub poolEntryCount:  u32,
}

#[repr(C)]
pub struct VkDeviceObjectReservationCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pipelineCacheCreateInfoCount:  u32,
    pub pPipelineCacheCreateInfos: *const  VkPipelineCacheCreateInfo,
    pub pipelinePoolSizeCount:  u32,
    pub pPipelinePoolSizes: *const  VkPipelinePoolSize,
    pub semaphoreRequestCount:  u32,
    pub commandBufferRequestCount:  u32,
    pub fenceRequestCount:  u32,
    pub deviceMemoryRequestCount:  u32,
    pub bufferRequestCount:  u32,
    pub imageRequestCount:  u32,
    pub eventRequestCount:  u32,
    pub queryPoolRequestCount:  u32,
    pub bufferViewRequestCount:  u32,
    pub imageViewRequestCount:  u32,
    pub layeredImageViewRequestCount:  u32,
    pub pipelineCacheRequestCount:  u32,
    pub pipelineLayoutRequestCount:  u32,
    pub renderPassRequestCount:  u32,
    pub graphicsPipelineRequestCount:  u32,
    pub computePipelineRequestCount:  u32,
    pub descriptorSetLayoutRequestCount:  u32,
    pub samplerRequestCount:  u32,
    pub descriptorPoolRequestCount:  u32,
    pub descriptorSetRequestCount:  u32,
    pub framebufferRequestCount:  u32,
    pub commandPoolRequestCount:  u32,
    pub samplerYcbcrConversionRequestCount:  u32,
    pub surfaceRequestCount:  u32,
    pub swapchainRequestCount:  u32,
    pub displayModeRequestCount:  u32,
    pub subpassDescriptionRequestCount:  u32,
    pub attachmentDescriptionRequestCount:  u32,
    pub descriptorSetLayoutBindingRequestCount:  u32,
    pub descriptorSetLayoutBindingLimit:  u32,
    pub maxImageViewMipLevels:  u32,
    pub maxImageViewArrayLayers:  u32,
    pub maxLayeredImageViewMipLevels:  u32,
    pub maxOcclusionQueriesPerPool:  u32,
    pub maxPipelineStatisticsQueriesPerPool:  u32,
    pub maxTimestampQueriesPerPool:  u32,
    pub maxImmutableSamplersPerDescriptorSetLayout:  u32,
}

#[repr(C)]
pub struct VkCommandPoolMemoryReservationCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub commandPoolReservedSize:  VkDeviceSize,
    pub commandPoolMaxCommandBuffers:  u32,
}

#[repr(C)]
pub struct VkCommandPoolMemoryConsumption {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub commandPoolAllocated:  VkDeviceSize,
    pub commandPoolReservedSize:  VkDeviceSize,
    pub commandBufferAllocated:  VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceVulkanSC10Features {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderAtomicInstructions:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePrimitivesGeneratedQueryFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub primitivesGeneratedQuery:  VkBool32,
    pub primitivesGeneratedQueryWithRasterizerDiscard:  VkBool32,
    pub primitivesGeneratedQueryWithNonZeroStreams:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceLegacyDitheringFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub legacyDithering:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub multisampledRenderToSingleSampled:  VkBool32,
}

#[repr(C)]
pub struct VkSurfaceCapabilitiesPresentId2KHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentId2Supported:  VkBool32,
}

#[repr(C)]
pub struct VkSurfaceCapabilitiesPresentWait2KHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentWait2Supported:  VkBool32,
}

#[repr(C)]
pub struct VkSubpassResolvePerformanceQueryEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub optimal:  VkBool32,
}

#[repr(C)]
pub struct VkMultisampledRenderToSingleSampledInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub multisampledRenderToSingleSampledEnable:  VkBool32,
    pub rasterizationSamples:  VkSampleCountFlagBits,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultisampledRenderToSwapchainFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub multisampledRenderToSwapchain:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineProtectedAccessFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pipelineProtectedAccess:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineProtectedAccessFeaturesEXT {
}

#[repr(C)]
pub struct VkQueueFamilyVideoPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub videoCodecOperations:  VkVideoCodecOperationFlagsKHR,
}

#[repr(C)]
pub struct VkQueueFamilyQueryResultStatusPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub queryResultStatusSupport:  VkBool32,
}

#[repr(C)]
pub struct VkVideoProfileListInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub profileCount:  u32,
    pub pProfiles: *const  VkVideoProfileInfoKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceVideoFormatInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub imageUsage:  VkImageUsageFlags,
}

#[repr(C)]
pub struct VkVideoFormatPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub format:  VkFormat,
    pub componentMapping:  VkComponentMapping,
    pub imageCreateFlags:  VkImageCreateFlags,
    pub imageType:  VkImageType,
    pub imageTiling:  VkImageTiling,
    pub imageUsageFlags:  VkImageUsageFlags,
}

#[repr(C)]
pub struct VkVideoEncodeQuantizationMapCapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxQuantizationMapExtent:  VkExtent2D,
}

#[repr(C)]
pub struct VkVideoEncodeH264QuantizationMapCapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub minQpDelta:  i32,
    pub maxQpDelta:  i32,
}

#[repr(C)]
pub struct VkVideoEncodeH265QuantizationMapCapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub minQpDelta:  i32,
    pub maxQpDelta:  i32,
}

#[repr(C)]
pub struct VkVideoEncodeAV1QuantizationMapCapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub minQIndexDelta:  i32,
    pub maxQIndexDelta:  i32,
}

#[repr(C)]
pub struct VkVideoFormatQuantizationMapPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub quantizationMapTexelSize:  VkExtent2D,
}

#[repr(C)]
pub struct VkVideoFormatH265QuantizationMapPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub compatibleCtbSizes:  VkVideoEncodeH265CtbSizeFlagsKHR,
}

#[repr(C)]
pub struct VkVideoFormatAV1QuantizationMapPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub compatibleSuperblockSizes:  VkVideoEncodeAV1SuperblockSizeFlagsKHR,
}

#[repr(C)]
pub struct VkVideoProfileInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub videoCodecOperation:  VkVideoCodecOperationFlagBitsKHR,
    pub chromaSubsampling:  VkVideoChromaSubsamplingFlagsKHR,
    pub lumaBitDepth:  VkVideoComponentBitDepthFlagsKHR,
    pub chromaBitDepth:  VkVideoComponentBitDepthFlagsKHR,
}

#[repr(C)]
pub struct VkVideoCapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkVideoCapabilityFlagsKHR,
    pub minBitstreamBufferOffsetAlignment:  VkDeviceSize,
    pub minBitstreamBufferSizeAlignment:  VkDeviceSize,
    pub pictureAccessGranularity:  VkExtent2D,
    pub minCodedExtent:  VkExtent2D,
    pub maxCodedExtent:  VkExtent2D,
    pub maxDpbSlots:  u32,
    pub maxActiveReferencePictures:  u32,
    pub stdHeaderVersion:  VkExtensionProperties,
}

#[repr(C)]
pub struct VkVideoSessionMemoryRequirementsKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub memoryBindIndex:  u32,
    pub memoryRequirements:  VkMemoryRequirements,
}

#[repr(C)]
pub struct VkBindVideoSessionMemoryInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub memoryBindIndex:  u32,
    pub memory:  VkDeviceMemory,
    pub memoryOffset:  VkDeviceSize,
    pub memorySize:  VkDeviceSize,
}

#[repr(C)]
pub struct VkVideoPictureResourceInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub codedOffset:  VkOffset2D,
    pub codedExtent:  VkExtent2D,
    pub baseArrayLayer:  u32,
    pub imageViewBinding:  VkImageView,
}

#[repr(C)]
pub struct VkVideoReferenceSlotInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub slotIndex:  i32,
    pub pPictureResource: *const  VkVideoPictureResourceInfoKHR,
}

#[repr(C)]
pub struct VkVideoDecodeCapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkVideoDecodeCapabilityFlagsKHR,
}

#[repr(C)]
pub struct VkVideoDecodeUsageInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub videoUsageHints:  VkVideoDecodeUsageFlagsKHR,
}

#[repr(C)]
pub struct VkVideoDecodeInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkVideoDecodeFlagsKHR,
    pub srcBuffer:  VkBuffer,
    pub srcBufferOffset:  VkDeviceSize,
    pub srcBufferRange:  VkDeviceSize,
    pub dstPictureResource:  VkVideoPictureResourceInfoKHR,
    pub pSetupReferenceSlot: *const  VkVideoReferenceSlotInfoKHR,
    pub referenceSlotCount:  u32,
    pub pReferenceSlots: *const  VkVideoReferenceSlotInfoKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceVideoMaintenance1FeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub videoMaintenance1:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVideoMaintenance2FeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub videoMaintenance2:  VkBool32,
}

#[repr(C)]
pub struct VkVideoInlineQueryInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub queryPool:  VkQueryPool,
    pub firstQuery:  u32,
    pub queryCount:  u32,
}

#[repr(C)]
pub struct VkVideoDecodeH264ProfileInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stdProfileIdc:  StdVideoH264ProfileIdc,
    pub pictureLayout:  VkVideoDecodeH264PictureLayoutFlagBitsKHR,
}

#[repr(C)]
pub struct VkVideoDecodeH264CapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxLevelIdc:  StdVideoH264LevelIdc,
    pub fieldOffsetGranularity:  VkOffset2D,
}

#[repr(C)]
pub struct VkVideoDecodeH264SessionParametersAddInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stdSPSCount:  u32,
    pub pStdSPSs: *const  StdVideoH264SequenceParameterSet,
    pub stdPPSCount:  u32,
    pub pStdPPSs: *const  StdVideoH264PictureParameterSet,
}

#[repr(C)]
pub struct VkVideoDecodeH264SessionParametersCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub maxStdSPSCount:  u32,
    pub maxStdPPSCount:  u32,
    pub pParametersAddInfo: *const  VkVideoDecodeH264SessionParametersAddInfoKHR,
}

#[repr(C)]
pub struct VkVideoDecodeH264InlineSessionParametersInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pStdSPS: *const  StdVideoH264SequenceParameterSet,
    pub pStdPPS: *const  StdVideoH264PictureParameterSet,
}

#[repr(C)]
pub struct VkVideoDecodeH264PictureInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pStdPictureInfo: *const  StdVideoDecodeH264PictureInfo,
    pub sliceCount:  u32,
    pub pSliceOffsets: *const  u32,
}

#[repr(C)]
pub struct VkVideoDecodeH264DpbSlotInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pStdReferenceInfo: *const  StdVideoDecodeH264ReferenceInfo,
}

#[repr(C)]
pub struct VkVideoDecodeH265ProfileInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stdProfileIdc:  StdVideoH265ProfileIdc,
}

#[repr(C)]
pub struct VkVideoDecodeH265CapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxLevelIdc:  StdVideoH265LevelIdc,
}

#[repr(C)]
pub struct VkVideoDecodeH265SessionParametersAddInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stdVPSCount:  u32,
    pub pStdVPSs: *const  StdVideoH265VideoParameterSet,
    pub stdSPSCount:  u32,
    pub pStdSPSs: *const  StdVideoH265SequenceParameterSet,
    pub stdPPSCount:  u32,
    pub pStdPPSs: *const  StdVideoH265PictureParameterSet,
}

#[repr(C)]
pub struct VkVideoDecodeH265SessionParametersCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub maxStdVPSCount:  u32,
    pub maxStdSPSCount:  u32,
    pub maxStdPPSCount:  u32,
    pub pParametersAddInfo: *const  VkVideoDecodeH265SessionParametersAddInfoKHR,
}

#[repr(C)]
pub struct VkVideoDecodeH265InlineSessionParametersInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pStdVPS: *const  StdVideoH265VideoParameterSet,
    pub pStdSPS: *const  StdVideoH265SequenceParameterSet,
    pub pStdPPS: *const  StdVideoH265PictureParameterSet,
}

#[repr(C)]
pub struct VkVideoDecodeH265PictureInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pStdPictureInfo: *const  StdVideoDecodeH265PictureInfo,
    pub sliceSegmentCount:  u32,
    pub pSliceSegmentOffsets: *const  u32,
}

#[repr(C)]
pub struct VkVideoDecodeH265DpbSlotInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pStdReferenceInfo: *const  StdVideoDecodeH265ReferenceInfo,
}

#[repr(C)]
pub struct VkPhysicalDeviceVideoDecodeVP9FeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub videoDecodeVP9:  VkBool32,
}

#[repr(C)]
pub struct VkVideoDecodeVP9ProfileInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stdProfile:  StdVideoVP9Profile,
}

#[repr(C)]
pub struct VkVideoDecodeVP9CapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxLevel:  StdVideoVP9Level,
}

#[repr(C)]
pub struct VkVideoDecodeVP9PictureInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pStdPictureInfo: *const  StdVideoDecodeVP9PictureInfo,
    pub referenceNameSlotIndices:  [i32; VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR as usize],
    pub uncompressedHeaderOffset:  u32,
    pub compressedHeaderOffset:  u32,
    pub tilesOffset:  u32,
}

#[repr(C)]
pub struct VkVideoDecodeAV1ProfileInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stdProfile:  StdVideoAV1Profile,
    pub filmGrainSupport:  VkBool32,
}

#[repr(C)]
pub struct VkVideoDecodeAV1CapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxLevel:  StdVideoAV1Level,
}

#[repr(C)]
pub struct VkVideoDecodeAV1SessionParametersCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pStdSequenceHeader: *const  StdVideoAV1SequenceHeader,
}

#[repr(C)]
pub struct VkVideoDecodeAV1InlineSessionParametersInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pStdSequenceHeader: *const  StdVideoAV1SequenceHeader,
}

#[repr(C)]
pub struct VkVideoDecodeAV1PictureInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pStdPictureInfo: *const  StdVideoDecodeAV1PictureInfo,
    pub referenceNameSlotIndices:  [i32; VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR as usize],
    pub frameHeaderOffset:  u32,
    pub tileCount:  u32,
    pub pTileOffsets: *const  u32,
    pub pTileSizes: *const  u32,
}

#[repr(C)]
pub struct VkVideoDecodeAV1DpbSlotInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pStdReferenceInfo: *const  StdVideoDecodeAV1ReferenceInfo,
}

#[repr(C)]
pub struct VkVideoSessionCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub queueFamilyIndex:  u32,
    pub flags:  VkVideoSessionCreateFlagsKHR,
    pub pVideoProfile: *const  VkVideoProfileInfoKHR,
    pub pictureFormat:  VkFormat,
    pub maxCodedExtent:  VkExtent2D,
    pub referencePictureFormat:  VkFormat,
    pub maxDpbSlots:  u32,
    pub maxActiveReferencePictures:  u32,
    pub pStdHeaderVersion: *const  VkExtensionProperties,
}

#[repr(C)]
pub struct VkVideoSessionParametersCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkVideoSessionParametersCreateFlagsKHR,
    pub videoSessionParametersTemplate:  VkVideoSessionParametersKHR,
    pub videoSession:  VkVideoSessionKHR,
}

#[repr(C)]
pub struct VkVideoSessionParametersUpdateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub updateSequenceCount:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeSessionParametersGetInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub videoSessionParameters:  VkVideoSessionParametersKHR,
}

#[repr(C)]
pub struct VkVideoEncodeSessionParametersFeedbackInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub hasOverrides:  VkBool32,
}

#[repr(C)]
pub struct VkVideoBeginCodingInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkVideoBeginCodingFlagsKHR,
    pub videoSession:  VkVideoSessionKHR,
    pub videoSessionParameters:  VkVideoSessionParametersKHR,
    pub referenceSlotCount:  u32,
    pub pReferenceSlots: *const  VkVideoReferenceSlotInfoKHR,
}

#[repr(C)]
pub struct VkVideoEndCodingInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkVideoEndCodingFlagsKHR,
}

#[repr(C)]
pub struct VkVideoCodingControlInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkVideoCodingControlFlagsKHR,
}

#[repr(C)]
pub struct VkVideoEncodeUsageInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub videoUsageHints:  VkVideoEncodeUsageFlagsKHR,
    pub videoContentHints:  VkVideoEncodeContentFlagsKHR,
    pub tuningMode:  VkVideoEncodeTuningModeKHR,
}

#[repr(C)]
pub struct VkVideoEncodeInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkVideoEncodeFlagsKHR,
    pub dstBuffer:  VkBuffer,
    pub dstBufferOffset:  VkDeviceSize,
    pub dstBufferRange:  VkDeviceSize,
    pub srcPictureResource:  VkVideoPictureResourceInfoKHR,
    pub pSetupReferenceSlot: *const  VkVideoReferenceSlotInfoKHR,
    pub referenceSlotCount:  u32,
    pub pReferenceSlots: *const  VkVideoReferenceSlotInfoKHR,
    pub precedingExternallyEncodedBytes:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeQuantizationMapInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub quantizationMap:  VkImageView,
    pub quantizationMapExtent:  VkExtent2D,
}

#[repr(C)]
pub struct VkVideoEncodeQuantizationMapSessionParametersCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub quantizationMapTexelSize:  VkExtent2D,
}

#[repr(C)]
pub struct VkPhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub videoEncodeQuantizationMap:  VkBool32,
}

#[repr(C)]
pub struct VkQueryPoolVideoEncodeFeedbackCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub encodeFeedbackFlags:  VkVideoEncodeFeedbackFlagsKHR,
}

#[repr(C)]
pub struct VkVideoEncodeQualityLevelInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub qualityLevel:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVideoEncodeQualityLevelInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pVideoProfile: *const  VkVideoProfileInfoKHR,
    pub qualityLevel:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeQualityLevelPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub preferredRateControlMode:  VkVideoEncodeRateControlModeFlagBitsKHR,
    pub preferredRateControlLayerCount:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeRateControlInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkVideoEncodeRateControlFlagsKHR,
    pub rateControlMode:  VkVideoEncodeRateControlModeFlagBitsKHR,
    pub layerCount:  u32,
    pub pLayers: *const  VkVideoEncodeRateControlLayerInfoKHR,
    pub virtualBufferSizeInMs:  u32,
    pub initialVirtualBufferSizeInMs:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeRateControlLayerInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub averageBitrate:  u64,
    pub maxBitrate:  u64,
    pub frameRateNumerator:  u32,
    pub frameRateDenominator:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeCapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkVideoEncodeCapabilityFlagsKHR,
    pub rateControlModes:  VkVideoEncodeRateControlModeFlagsKHR,
    pub maxRateControlLayers:  u32,
    pub maxBitrate:  u64,
    pub maxQualityLevels:  u32,
    pub encodeInputPictureGranularity:  VkExtent2D,
    pub supportedEncodeFeedbackFlags:  VkVideoEncodeFeedbackFlagsKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceVideoEncodeFeedback2FeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub videoEncodeFeedback2:  VkBool32,
}

#[repr(C)]
pub struct VkVideoEncodeFeedback2CapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxPerPartitionFeedbackEntries:  u32,
    pub supportedPerPartitionEncodeFeedbackFlags:  VkVideoEncodePerPartitionFeedbackFlagsKHR,
}

#[repr(C)]
pub struct VkQueryPoolVideoEncodePerPartitionFeedbackCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub maxPerPartitionFeedbackEntries:  u32,
    pub perPartitionEncodeFeedbackFlags:  VkVideoEncodePerPartitionFeedbackFlagsKHR,
}

#[repr(C)]
pub struct VkVideoEncodeH264CapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkVideoEncodeH264CapabilityFlagsKHR,
    pub maxLevelIdc:  StdVideoH264LevelIdc,
    pub maxSliceCount:  u32,
    pub maxPPictureL0ReferenceCount:  u32,
    pub maxBPictureL0ReferenceCount:  u32,
    pub maxL1ReferenceCount:  u32,
    pub maxTemporalLayerCount:  u32,
    pub expectDyadicTemporalLayerPattern:  VkBool32,
    pub minQp:  i32,
    pub maxQp:  i32,
    pub prefersGopRemainingFrames:  VkBool32,
    pub requiresGopRemainingFrames:  VkBool32,
    pub stdSyntaxFlags:  VkVideoEncodeH264StdFlagsKHR,
}

#[repr(C)]
pub struct VkVideoEncodeH264QualityLevelPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub preferredRateControlFlags:  VkVideoEncodeH264RateControlFlagsKHR,
    pub preferredGopFrameCount:  u32,
    pub preferredIdrPeriod:  u32,
    pub preferredConsecutiveBFrameCount:  u32,
    pub preferredTemporalLayerCount:  u32,
    pub preferredConstantQp:  VkVideoEncodeH264QpKHR,
    pub preferredMaxL0ReferenceCount:  u32,
    pub preferredMaxL1ReferenceCount:  u32,
    pub preferredStdEntropyCodingModeFlag:  VkBool32,
}

#[repr(C)]
pub struct VkVideoEncodeH264SessionCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub useMaxLevelIdc:  VkBool32,
    pub maxLevelIdc:  StdVideoH264LevelIdc,
}

#[repr(C)]
pub struct VkVideoEncodeH264SessionParametersAddInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stdSPSCount:  u32,
    pub pStdSPSs: *const  StdVideoH264SequenceParameterSet,
    pub stdPPSCount:  u32,
    pub pStdPPSs: *const  StdVideoH264PictureParameterSet,
}

#[repr(C)]
pub struct VkVideoEncodeH264SessionParametersCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub maxStdSPSCount:  u32,
    pub maxStdPPSCount:  u32,
    pub pParametersAddInfo: *const  VkVideoEncodeH264SessionParametersAddInfoKHR,
}

#[repr(C)]
pub struct VkVideoEncodeH264SessionParametersGetInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub writeStdSPS:  VkBool32,
    pub writeStdPPS:  VkBool32,
    pub stdSPSId:  u32,
    pub stdPPSId:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeH264SessionParametersFeedbackInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub hasStdSPSOverrides:  VkBool32,
    pub hasStdPPSOverrides:  VkBool32,
}

#[repr(C)]
pub struct VkVideoEncodeH264DpbSlotInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pStdReferenceInfo: *const  StdVideoEncodeH264ReferenceInfo,
}

#[repr(C)]
pub struct VkVideoEncodeH264PictureInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub naluSliceEntryCount:  u32,
    pub pNaluSliceEntries: *const  VkVideoEncodeH264NaluSliceInfoKHR,
    pub pStdPictureInfo: *const  StdVideoEncodeH264PictureInfo,
    pub generatePrefixNalu:  VkBool32,
}

#[repr(C)]
pub struct VkVideoEncodeH264ProfileInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stdProfileIdc:  StdVideoH264ProfileIdc,
}

#[repr(C)]
pub struct VkVideoEncodeH264NaluSliceInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub constantQp:  i32,
    pub pStdSliceHeader: *const  StdVideoEncodeH264SliceHeader,
}

#[repr(C)]
pub struct VkVideoEncodeH264RateControlInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkVideoEncodeH264RateControlFlagsKHR,
    pub gopFrameCount:  u32,
    pub idrPeriod:  u32,
    pub consecutiveBFrameCount:  u32,
    pub temporalLayerCount:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeH264QpKHR {
    pub qpI:  i32,
    pub qpP:  i32,
    pub qpB:  i32,
}

#[repr(C)]
pub struct VkVideoEncodeH264FrameSizeKHR {
    pub frameISize:  u32,
    pub framePSize:  u32,
    pub frameBSize:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeH264GopRemainingFrameInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub useGopRemainingFrames:  VkBool32,
    pub gopRemainingI:  u32,
    pub gopRemainingP:  u32,
    pub gopRemainingB:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeH264RateControlLayerInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub useMinQp:  VkBool32,
    pub minQp:  VkVideoEncodeH264QpKHR,
    pub useMaxQp:  VkBool32,
    pub maxQp:  VkVideoEncodeH264QpKHR,
    pub useMaxFrameSize:  VkBool32,
    pub maxFrameSize:  VkVideoEncodeH264FrameSizeKHR,
}

#[repr(C)]
pub struct VkVideoEncodeH265CapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkVideoEncodeH265CapabilityFlagsKHR,
    pub maxLevelIdc:  StdVideoH265LevelIdc,
    pub maxSliceSegmentCount:  u32,
    pub maxTiles:  VkExtent2D,
    pub ctbSizes:  VkVideoEncodeH265CtbSizeFlagsKHR,
    pub transformBlockSizes:  VkVideoEncodeH265TransformBlockSizeFlagsKHR,
    pub maxPPictureL0ReferenceCount:  u32,
    pub maxBPictureL0ReferenceCount:  u32,
    pub maxL1ReferenceCount:  u32,
    pub maxSubLayerCount:  u32,
    pub expectDyadicTemporalSubLayerPattern:  VkBool32,
    pub minQp:  i32,
    pub maxQp:  i32,
    pub prefersGopRemainingFrames:  VkBool32,
    pub requiresGopRemainingFrames:  VkBool32,
    pub stdSyntaxFlags:  VkVideoEncodeH265StdFlagsKHR,
}

#[repr(C)]
pub struct VkVideoEncodeH265QualityLevelPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub preferredRateControlFlags:  VkVideoEncodeH265RateControlFlagsKHR,
    pub preferredGopFrameCount:  u32,
    pub preferredIdrPeriod:  u32,
    pub preferredConsecutiveBFrameCount:  u32,
    pub preferredSubLayerCount:  u32,
    pub preferredConstantQp:  VkVideoEncodeH265QpKHR,
    pub preferredMaxL0ReferenceCount:  u32,
    pub preferredMaxL1ReferenceCount:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeH265SessionCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub useMaxLevelIdc:  VkBool32,
    pub maxLevelIdc:  StdVideoH265LevelIdc,
}

#[repr(C)]
pub struct VkVideoEncodeH265SessionParametersAddInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stdVPSCount:  u32,
    pub pStdVPSs: *const  StdVideoH265VideoParameterSet,
    pub stdSPSCount:  u32,
    pub pStdSPSs: *const  StdVideoH265SequenceParameterSet,
    pub stdPPSCount:  u32,
    pub pStdPPSs: *const  StdVideoH265PictureParameterSet,
}

#[repr(C)]
pub struct VkVideoEncodeH265SessionParametersCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub maxStdVPSCount:  u32,
    pub maxStdSPSCount:  u32,
    pub maxStdPPSCount:  u32,
    pub pParametersAddInfo: *const  VkVideoEncodeH265SessionParametersAddInfoKHR,
}

#[repr(C)]
pub struct VkVideoEncodeH265SessionParametersGetInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub writeStdVPS:  VkBool32,
    pub writeStdSPS:  VkBool32,
    pub writeStdPPS:  VkBool32,
    pub stdVPSId:  u32,
    pub stdSPSId:  u32,
    pub stdPPSId:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeH265SessionParametersFeedbackInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub hasStdVPSOverrides:  VkBool32,
    pub hasStdSPSOverrides:  VkBool32,
    pub hasStdPPSOverrides:  VkBool32,
}

#[repr(C)]
pub struct VkVideoEncodeH265PictureInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub naluSliceSegmentEntryCount:  u32,
    pub pNaluSliceSegmentEntries: *const  VkVideoEncodeH265NaluSliceSegmentInfoKHR,
    pub pStdPictureInfo: *const  StdVideoEncodeH265PictureInfo,
}

#[repr(C)]
pub struct VkVideoEncodeH265NaluSliceSegmentInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub constantQp:  i32,
    pub pStdSliceSegmentHeader: *const  StdVideoEncodeH265SliceSegmentHeader,
}

#[repr(C)]
pub struct VkVideoEncodeH265RateControlInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkVideoEncodeH265RateControlFlagsKHR,
    pub gopFrameCount:  u32,
    pub idrPeriod:  u32,
    pub consecutiveBFrameCount:  u32,
    pub subLayerCount:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeH265QpKHR {
    pub qpI:  i32,
    pub qpP:  i32,
    pub qpB:  i32,
}

#[repr(C)]
pub struct VkVideoEncodeH265FrameSizeKHR {
    pub frameISize:  u32,
    pub framePSize:  u32,
    pub frameBSize:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeH265GopRemainingFrameInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub useGopRemainingFrames:  VkBool32,
    pub gopRemainingI:  u32,
    pub gopRemainingP:  u32,
    pub gopRemainingB:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeH265RateControlLayerInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub useMinQp:  VkBool32,
    pub minQp:  VkVideoEncodeH265QpKHR,
    pub useMaxQp:  VkBool32,
    pub maxQp:  VkVideoEncodeH265QpKHR,
    pub useMaxFrameSize:  VkBool32,
    pub maxFrameSize:  VkVideoEncodeH265FrameSizeKHR,
}

#[repr(C)]
pub struct VkVideoEncodeH265ProfileInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stdProfileIdc:  StdVideoH265ProfileIdc,
}

#[repr(C)]
pub struct VkVideoEncodeH265DpbSlotInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pStdReferenceInfo: *const  StdVideoEncodeH265ReferenceInfo,
}

#[repr(C)]
pub struct VkVideoEncodeAV1CapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkVideoEncodeAV1CapabilityFlagsKHR,
    pub maxLevel:  StdVideoAV1Level,
    pub codedPictureAlignment:  VkExtent2D,
    pub maxTiles:  VkExtent2D,
    pub minTileSize:  VkExtent2D,
    pub maxTileSize:  VkExtent2D,
    pub superblockSizes:  VkVideoEncodeAV1SuperblockSizeFlagsKHR,
    pub maxSingleReferenceCount:  u32,
    pub singleReferenceNameMask:  u32,
    pub maxUnidirectionalCompoundReferenceCount:  u32,
    pub maxUnidirectionalCompoundGroup1ReferenceCount:  u32,
    pub unidirectionalCompoundReferenceNameMask:  u32,
    pub maxBidirectionalCompoundReferenceCount:  u32,
    pub maxBidirectionalCompoundGroup1ReferenceCount:  u32,
    pub maxBidirectionalCompoundGroup2ReferenceCount:  u32,
    pub bidirectionalCompoundReferenceNameMask:  u32,
    pub maxTemporalLayerCount:  u32,
    pub maxSpatialLayerCount:  u32,
    pub maxOperatingPoints:  u32,
    pub minQIndex:  u32,
    pub maxQIndex:  u32,
    pub prefersGopRemainingFrames:  VkBool32,
    pub requiresGopRemainingFrames:  VkBool32,
    pub stdSyntaxFlags:  VkVideoEncodeAV1StdFlagsKHR,
}

#[repr(C)]
pub struct VkVideoEncodeAV1QualityLevelPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub preferredRateControlFlags:  VkVideoEncodeAV1RateControlFlagsKHR,
    pub preferredGopFrameCount:  u32,
    pub preferredKeyFramePeriod:  u32,
    pub preferredConsecutiveBipredictiveFrameCount:  u32,
    pub preferredTemporalLayerCount:  u32,
    pub preferredConstantQIndex:  VkVideoEncodeAV1QIndexKHR,
    pub preferredMaxSingleReferenceCount:  u32,
    pub preferredSingleReferenceNameMask:  u32,
    pub preferredMaxUnidirectionalCompoundReferenceCount:  u32,
    pub preferredMaxUnidirectionalCompoundGroup1ReferenceCount:  u32,
    pub preferredUnidirectionalCompoundReferenceNameMask:  u32,
    pub preferredMaxBidirectionalCompoundReferenceCount:  u32,
    pub preferredMaxBidirectionalCompoundGroup1ReferenceCount:  u32,
    pub preferredMaxBidirectionalCompoundGroup2ReferenceCount:  u32,
    pub preferredBidirectionalCompoundReferenceNameMask:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVideoEncodeAV1FeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub videoEncodeAV1:  VkBool32,
}

#[repr(C)]
pub struct VkVideoEncodeAV1SessionCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub useMaxLevel:  VkBool32,
    pub maxLevel:  StdVideoAV1Level,
}

#[repr(C)]
pub struct VkVideoEncodeAV1SessionParametersCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pStdSequenceHeader: *const  StdVideoAV1SequenceHeader,
    pub pStdDecoderModelInfo: *const  StdVideoEncodeAV1DecoderModelInfo,
    pub stdOperatingPointCount:  u32,
    pub pStdOperatingPoints: *const  StdVideoEncodeAV1OperatingPointInfo,
}

#[repr(C)]
pub struct VkVideoEncodeAV1DpbSlotInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pStdReferenceInfo: *const  StdVideoEncodeAV1ReferenceInfo,
}

#[repr(C)]
pub struct VkVideoEncodeAV1PictureInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub predictionMode:  VkVideoEncodeAV1PredictionModeKHR,
    pub rateControlGroup:  VkVideoEncodeAV1RateControlGroupKHR,
    pub constantQIndex:  u32,
    pub pStdPictureInfo: *const  StdVideoEncodeAV1PictureInfo,
    pub referenceNameSlotIndices:  [i32; VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR as usize],
    pub primaryReferenceCdfOnly:  VkBool32,
    pub generateObuExtensionHeader:  VkBool32,
}

#[repr(C)]
pub struct VkVideoEncodeAV1ProfileInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stdProfile:  StdVideoAV1Profile,
}

#[repr(C)]
pub struct VkVideoEncodeAV1RateControlInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkVideoEncodeAV1RateControlFlagsKHR,
    pub gopFrameCount:  u32,
    pub keyFramePeriod:  u32,
    pub consecutiveBipredictiveFrameCount:  u32,
    pub temporalLayerCount:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeAV1QIndexKHR {
    pub intraQIndex:  u32,
    pub predictiveQIndex:  u32,
    pub bipredictiveQIndex:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeAV1FrameSizeKHR {
    pub intraFrameSize:  u32,
    pub predictiveFrameSize:  u32,
    pub bipredictiveFrameSize:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeAV1GopRemainingFrameInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub useGopRemainingFrames:  VkBool32,
    pub gopRemainingIntra:  u32,
    pub gopRemainingPredictive:  u32,
    pub gopRemainingBipredictive:  u32,
}

#[repr(C)]
pub struct VkVideoEncodeAV1RateControlLayerInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub useMinQIndex:  VkBool32,
    pub minQIndex:  VkVideoEncodeAV1QIndexKHR,
    pub useMaxQIndex:  VkBool32,
    pub maxQIndex:  VkVideoEncodeAV1QIndexKHR,
    pub useMaxFrameSize:  VkBool32,
    pub maxFrameSize:  VkVideoEncodeAV1FrameSizeKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceInheritedViewportScissorFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub inheritedViewportScissor2D:  VkBool32,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceViewportScissorInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub viewportScissor2D:  VkBool32,
    pub viewportDepthCount:  u32,
    pub pViewportDepths: *const  VkViewport,
}

#[repr(C)]
pub struct VkPhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub ycbcr2plane444Formats:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceProvokingVertexFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub provokingVertexLast:  VkBool32,
    pub transformFeedbackPreservesProvokingVertex:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceProvokingVertexPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub provokingVertexModePerPipeline:  VkBool32,
    pub transformFeedbackPreservesTriangleFanProvokingVertex:  VkBool32,
}

#[repr(C)]
pub struct VkPipelineRasterizationProvokingVertexStateCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub provokingVertexMode:  VkProvokingVertexModeEXT,
}

#[repr(C)]
pub struct VkVideoEncodeIntraRefreshCapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub intraRefreshModes:  VkVideoEncodeIntraRefreshModeFlagsKHR,
    pub maxIntraRefreshCycleDuration:  u32,
    pub maxIntraRefreshActiveReferencePictures:  u32,
    pub partitionIndependentIntraRefreshRegions:  VkBool32,
    pub nonRectangularIntraRefreshRegions:  VkBool32,
}

#[repr(C)]
pub struct VkVideoEncodeSessionIntraRefreshCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub intraRefreshMode:  VkVideoEncodeIntraRefreshModeFlagBitsKHR,
}

#[repr(C)]
pub struct VkVideoEncodeIntraRefreshInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub intraRefreshCycleDuration:  u32,
    pub intraRefreshIndex:  u32,
}

#[repr(C)]
pub struct VkVideoReferenceIntraRefreshInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub dirtyIntraRefreshRegions:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub videoEncodeIntraRefresh:  VkBool32,
}

#[repr(C)]
pub struct VkCuModuleCreateInfoNVX {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub dataSize:  usize,
    pub pData: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkCuModuleTexturingModeCreateInfoNVX {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub use64bitTexturing:  VkBool32,
}

#[repr(C)]
pub struct VkCuFunctionCreateInfoNVX {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub module:  VkCuModuleNVX,
    pub pName: *const  i8,
}

#[repr(C)]
pub struct VkCuLaunchInfoNVX {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub function:  VkCuFunctionNVX,
    pub gridDimX:  u32,
    pub gridDimY:  u32,
    pub gridDimZ:  u32,
    pub blockDimX:  u32,
    pub blockDimY:  u32,
    pub blockDimZ:  u32,
    pub sharedMemBytes:  u32,
    pub paramCount:  usize,
    pub pParams: *const  std::ffi::c_void,
    pub extraCount:  usize,
    pub pExtras: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub descriptorBuffer:  VkBool32,
    pub descriptorBufferCaptureReplay:  VkBool32,
    pub descriptorBufferImageLayoutIgnored:  VkBool32,
    pub descriptorBufferPushDescriptors:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub combinedImageSamplerDescriptorSingleArray:  VkBool32,
    pub bufferlessPushDescriptors:  VkBool32,
    pub allowSamplerImageViewPostSubmitCreation:  VkBool32,
    pub descriptorBufferOffsetAlignment:  VkDeviceSize,
    pub maxDescriptorBufferBindings:  u32,
    pub maxResourceDescriptorBufferBindings:  u32,
    pub maxSamplerDescriptorBufferBindings:  u32,
    pub maxEmbeddedImmutableSamplerBindings:  u32,
    pub maxEmbeddedImmutableSamplers:  u32,
    pub bufferCaptureReplayDescriptorDataSize:  usize,
    pub imageCaptureReplayDescriptorDataSize:  usize,
    pub imageViewCaptureReplayDescriptorDataSize:  usize,
    pub samplerCaptureReplayDescriptorDataSize:  usize,
    pub accelerationStructureCaptureReplayDescriptorDataSize:  usize,
    pub samplerDescriptorSize:  usize,
    pub combinedImageSamplerDescriptorSize:  usize,
    pub sampledImageDescriptorSize:  usize,
    pub storageImageDescriptorSize:  usize,
    pub uniformTexelBufferDescriptorSize:  usize,
    pub robustUniformTexelBufferDescriptorSize:  usize,
    pub storageTexelBufferDescriptorSize:  usize,
    pub robustStorageTexelBufferDescriptorSize:  usize,
    pub uniformBufferDescriptorSize:  usize,
    pub robustUniformBufferDescriptorSize:  usize,
    pub storageBufferDescriptorSize:  usize,
    pub robustStorageBufferDescriptorSize:  usize,
    pub inputAttachmentDescriptorSize:  usize,
    pub accelerationStructureDescriptorSize:  usize,
    pub maxSamplerDescriptorBufferRange:  VkDeviceSize,
    pub maxResourceDescriptorBufferRange:  VkDeviceSize,
    pub samplerDescriptorBufferAddressSpaceSize:  VkDeviceSize,
    pub resourceDescriptorBufferAddressSpaceSize:  VkDeviceSize,
    pub descriptorBufferAddressSpaceSize:  VkDeviceSize,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferDensityMapPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub combinedImageSamplerDensityMapDescriptorSize:  usize,
}

#[repr(C)]
pub struct VkDescriptorAddressInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub address:  VkDeviceAddress,
    pub range:  VkDeviceSize,
    pub format:  VkFormat,
}

#[repr(C)]
pub struct VkDescriptorBufferBindingInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub address:  VkDeviceAddress,
    pub usage:  VkBufferUsageFlags,
}

#[repr(C)]
pub struct VkDescriptorBufferBindingPushDescriptorBufferHandleEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub buffer:  VkBuffer,
}

#[repr(C)]
pub struct VkDescriptorDataEXT {
    pub pSampler: *const  VkSampler,
    pub pCombinedImageSampler: *const  VkDescriptorImageInfo,
    pub pInputAttachmentImage: *const  VkDescriptorImageInfo,
    pub pSampledImage: *const  VkDescriptorImageInfo,
    pub pStorageImage: *const  VkDescriptorImageInfo,
    pub pUniformTexelBuffer: *const  VkDescriptorAddressInfoEXT,
    pub pStorageTexelBuffer: *const  VkDescriptorAddressInfoEXT,
    pub pUniformBuffer: *const  VkDescriptorAddressInfoEXT,
    pub pStorageBuffer: *const  VkDescriptorAddressInfoEXT,
    pub accelerationStructure:  VkDeviceAddress,
}

#[repr(C)]
pub struct VkDescriptorGetInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub r#type:  VkDescriptorType,
    pub data:  VkDescriptorDataEXT,
}

#[repr(C)]
pub struct VkBufferCaptureDescriptorDataInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub buffer:  VkBuffer,
}

#[repr(C)]
pub struct VkImageCaptureDescriptorDataInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub image:  VkImage,
}

#[repr(C)]
pub struct VkImageViewCaptureDescriptorDataInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub imageView:  VkImageView,
}

#[repr(C)]
pub struct VkSamplerCaptureDescriptorDataInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub sampler:  VkSampler,
}

#[repr(C)]
pub struct VkAccelerationStructureCaptureDescriptorDataInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub accelerationStructure:  VkAccelerationStructureKHR,
    pub accelerationStructureNV:  VkAccelerationStructureNV,
}

#[repr(C)]
pub struct VkOpaqueCaptureDescriptorDataCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub opaqueCaptureDescriptorData: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerDotProductFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderIntegerDotProduct:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerDotProductFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerDotProductProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub integerDotProduct8BitUnsignedAccelerated:  VkBool32,
    pub integerDotProduct8BitSignedAccelerated:  VkBool32,
    pub integerDotProduct8BitMixedSignednessAccelerated:  VkBool32,
    pub integerDotProduct4x8BitPackedUnsignedAccelerated:  VkBool32,
    pub integerDotProduct4x8BitPackedSignedAccelerated:  VkBool32,
    pub integerDotProduct4x8BitPackedMixedSignednessAccelerated:  VkBool32,
    pub integerDotProduct16BitUnsignedAccelerated:  VkBool32,
    pub integerDotProduct16BitSignedAccelerated:  VkBool32,
    pub integerDotProduct16BitMixedSignednessAccelerated:  VkBool32,
    pub integerDotProduct32BitUnsignedAccelerated:  VkBool32,
    pub integerDotProduct32BitSignedAccelerated:  VkBool32,
    pub integerDotProduct32BitMixedSignednessAccelerated:  VkBool32,
    pub integerDotProduct64BitUnsignedAccelerated:  VkBool32,
    pub integerDotProduct64BitSignedAccelerated:  VkBool32,
    pub integerDotProduct64BitMixedSignednessAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating8BitUnsignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating8BitSignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating8BitMixedSignednessAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedUnsignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedSignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating4x8BitPackedMixedSignednessAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating16BitUnsignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating16BitSignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating16BitMixedSignednessAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating32BitUnsignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating32BitSignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating32BitMixedSignednessAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating64BitUnsignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating64BitSignedAccelerated:  VkBool32,
    pub integerDotProductAccumulatingSaturating64BitMixedSignednessAccelerated:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderIntegerDotProductPropertiesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceDrmPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub hasPrimary:  VkBool32,
    pub hasRender:  VkBool32,
    pub primaryMajor:  i64,
    pub primaryMinor:  i64,
    pub renderMajor:  i64,
    pub renderMinor:  i64,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderBarycentricFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub fragmentShaderBarycentric:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentShaderBarycentricPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub triStripVertexOrderIndependentOfProvokingVertex:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderFmaFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderFmaFloat16:  VkBool32,
    pub shaderFmaFloat32:  VkBool32,
    pub shaderFmaFloat64:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingMotionBlurFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub rayTracingMotionBlur:  VkBool32,
    pub rayTracingMotionBlurPipelineTraceRaysIndirect:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingValidationFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub rayTracingValidation:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub spheres:  VkBool32,
    pub linearSweptSpheres:  VkBool32,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryMotionTrianglesDataNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub vertexData:  VkDeviceOrHostAddressConstKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureMotionInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub maxInstances:  u32,
    pub flags:  VkAccelerationStructureMotionInfoFlagsNV,
}

#[repr(C)]
pub struct VkSRTDataNV {
    pub sx:  f32,
    pub a:  f32,
    pub b:  f32,
    pub pvx:  f32,
    pub sy:  f32,
    pub c:  f32,
    pub pvy:  f32,
    pub sz:  f32,
    pub pvz:  f32,
    pub qx:  f32,
    pub qy:  f32,
    pub qz:  f32,
    pub qw:  f32,
    pub tx:  f32,
    pub ty:  f32,
    pub tz:  f32,
}

#[repr(C)]
pub struct VkAccelerationStructureSRTMotionInstanceNV {
    pub transformT0:  VkSRTDataNV,
    pub transformT1:  VkSRTDataNV,
    pub instanceCustomIndex:  u32,
    pub mask:  u32,
    pub instanceShaderBindingTableRecordOffset:  u32,
    pub flags:  VkGeometryInstanceFlagsKHR,
    pub accelerationStructureReference:  u64,
}

#[repr(C)]
pub struct VkAccelerationStructureMatrixMotionInstanceNV {
    pub transformT0:  VkTransformMatrixKHR,
    pub transformT1:  VkTransformMatrixKHR,
    pub instanceCustomIndex:  u32,
    pub mask:  u32,
    pub instanceShaderBindingTableRecordOffset:  u32,
    pub flags:  VkGeometryInstanceFlagsKHR,
    pub accelerationStructureReference:  u64,
}

#[repr(C)]
pub struct VkAccelerationStructureMotionInstanceDataNV {
    pub staticInstance:  VkAccelerationStructureInstanceKHR,
    pub matrixMotionInstance:  VkAccelerationStructureMatrixMotionInstanceNV,
    pub srtMotionInstance:  VkAccelerationStructureSRTMotionInstanceNV,
}

#[repr(C)]
pub struct VkAccelerationStructureMotionInstanceNV {
    pub r#type:  VkAccelerationStructureMotionInstanceTypeNV,
    pub flags:  VkAccelerationStructureMotionInstanceFlagsNV,
    pub data:  VkAccelerationStructureMotionInstanceDataNV,
}

pub type VkRemoteAddressNV = *mut std::ffi::c_void; //

#[repr(C)]
pub struct VkMemoryGetRemoteAddressInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub memory:  VkDeviceMemory,
    pub handleType:  VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkImportMemoryBufferCollectionFUCHSIA {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub collection:  VkBufferCollectionFUCHSIA,
    pub index:  u32,
}

#[repr(C)]
pub struct VkBufferCollectionImageCreateInfoFUCHSIA {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub collection:  VkBufferCollectionFUCHSIA,
    pub index:  u32,
}

#[repr(C)]
pub struct VkBufferCollectionBufferCreateInfoFUCHSIA {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub collection:  VkBufferCollectionFUCHSIA,
    pub index:  u32,
}

#[repr(C)]
pub struct VkBufferCollectionCreateInfoFUCHSIA {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub collectionToken:  zx_handle_t,
}

#[repr(C)]
pub struct VkBufferCollectionPropertiesFUCHSIA {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub memoryTypeBits:  u32,
    pub bufferCount:  u32,
    pub createInfoIndex:  u32,
    pub sysmemPixelFormat:  u64,
    pub formatFeatures:  VkFormatFeatureFlags,
    pub sysmemColorSpaceIndex:  VkSysmemColorSpaceFUCHSIA,
    pub samplerYcbcrConversionComponents:  VkComponentMapping,
    pub suggestedYcbcrModel:  VkSamplerYcbcrModelConversion,
    pub suggestedYcbcrRange:  VkSamplerYcbcrRange,
    pub suggestedXChromaOffset:  VkChromaLocation,
    pub suggestedYChromaOffset:  VkChromaLocation,
}

#[repr(C)]
pub struct VkBufferConstraintsInfoFUCHSIA {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub createInfo:  VkBufferCreateInfo,
    pub requiredFormatFeatures:  VkFormatFeatureFlags,
    pub bufferCollectionConstraints:  VkBufferCollectionConstraintsInfoFUCHSIA,
}

#[repr(C)]
pub struct VkSysmemColorSpaceFUCHSIA {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub colorSpace:  u32,
}

#[repr(C)]
pub struct VkImageFormatConstraintsInfoFUCHSIA {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub imageCreateInfo:  VkImageCreateInfo,
    pub requiredFormatFeatures:  VkFormatFeatureFlags,
    pub flags:  VkImageFormatConstraintsFlagsFUCHSIA,
    pub sysmemPixelFormat:  u64,
    pub colorSpaceCount:  u32,
    pub pColorSpaces: *const  VkSysmemColorSpaceFUCHSIA,
}

#[repr(C)]
pub struct VkImageConstraintsInfoFUCHSIA {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub formatConstraintsCount:  u32,
    pub pFormatConstraints: *const  VkImageFormatConstraintsInfoFUCHSIA,
    pub bufferCollectionConstraints:  VkBufferCollectionConstraintsInfoFUCHSIA,
    pub flags:  VkImageConstraintsInfoFlagsFUCHSIA,
}

#[repr(C)]
pub struct VkBufferCollectionConstraintsInfoFUCHSIA {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub minBufferCount:  u32,
    pub maxBufferCount:  u32,
    pub minBufferCountForCamping:  u32,
    pub minBufferCountForDedicatedSlack:  u32,
    pub minBufferCountForSharedSlack:  u32,
}

#[repr(C)]
pub struct VkCudaModuleNV {
    private: [u8; 0]
}
#[repr(C)]
pub struct VkCudaFunctionNV {
    private: [u8; 0]
}
#[repr(C)]
pub struct VkCudaModuleCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub dataSize:  usize,
    pub pData: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkCudaFunctionCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub module:  VkCudaModuleNV,
    pub pName: *const  i8,
}

#[repr(C)]
pub struct VkCudaLaunchInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub function:  VkCudaFunctionNV,
    pub gridDimX:  u32,
    pub gridDimY:  u32,
    pub gridDimZ:  u32,
    pub blockDimX:  u32,
    pub blockDimY:  u32,
    pub blockDimZ:  u32,
    pub sharedMemBytes:  u32,
    pub paramCount:  usize,
    pub pParams: *const  std::ffi::c_void,
    pub extraCount:  usize,
    pub pExtras: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceRGBA10X6FormatsFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub formatRgba10x6WithoutYCbCrSampler:  VkBool32,
}

#[repr(C)]
pub struct VkFormatProperties3 {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub linearTilingFeatures:  VkFormatFeatureFlags2,
    pub optimalTilingFeatures:  VkFormatFeatureFlags2,
    pub bufferFeatures:  VkFormatFeatureFlags2,
}

#[repr(C)]
pub struct VkFormatProperties3KHR {
}

#[repr(C)]
pub struct VkFormatProperties4KHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub linearTilingFeatures:  VkFormatFeatureFlags4KHR,
    pub optimalTilingFeatures:  VkFormatFeatureFlags4KHR,
    pub bufferFeatures:  VkFormatFeatureFlags4KHR,
}

#[repr(C)]
pub struct VkDrmFormatModifierPropertiesList2EXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub drmFormatModifierCount:  u32,
    pub pDrmFormatModifierProperties: *mut  VkDrmFormatModifierProperties2EXT,
}

#[repr(C)]
pub struct VkDrmFormatModifierProperties2EXT {
    pub drmFormatModifier:  u64,
    pub drmFormatModifierPlaneCount:  u32,
    pub drmFormatModifierTilingFeatures:  VkFormatFeatureFlags2,
}

#[repr(C)]
pub struct VkAndroidHardwareBufferFormatProperties2ANDROID {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub format:  VkFormat,
    pub externalFormat:  u64,
    pub formatFeatures:  VkFormatFeatureFlags2,
    pub samplerYcbcrConversionComponents:  VkComponentMapping,
    pub suggestedYcbcrModel:  VkSamplerYcbcrModelConversion,
    pub suggestedYcbcrRange:  VkSamplerYcbcrRange,
    pub suggestedXChromaOffset:  VkChromaLocation,
    pub suggestedYChromaOffset:  VkChromaLocation,
}

#[repr(C)]
pub struct VkPipelineRenderingCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub viewMask:  u32,
    pub colorAttachmentCount:  u32,
    pub pColorAttachmentFormats: *const  VkFormat,
    pub depthAttachmentFormat:  VkFormat,
    pub stencilAttachmentFormat:  VkFormat,
}

#[repr(C)]
pub struct VkPipelineRenderingCreateInfoKHR {
}

#[repr(C)]
pub struct VkRenderingInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkRenderingFlags,
    pub renderArea:  VkRect2D,
    pub layerCount:  u32,
    pub viewMask:  u32,
    pub colorAttachmentCount:  u32,
    pub pColorAttachments: *const  VkRenderingAttachmentInfo,
    pub pDepthAttachment: *const  VkRenderingAttachmentInfo,
    pub pStencilAttachment: *const  VkRenderingAttachmentInfo,
}

#[repr(C)]
pub struct VkRenderingInfoKHR {
}

#[repr(C)]
pub struct VkRenderingEndInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkRenderingEndInfoEXT {
}

#[repr(C)]
pub struct VkRenderingAttachmentInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub imageView:  VkImageView,
    pub imageLayout:  VkImageLayout,
    pub resolveMode:  VkResolveModeFlagBits,
    pub resolveImageView:  VkImageView,
    pub resolveImageLayout:  VkImageLayout,
    pub loadOp:  VkAttachmentLoadOp,
    pub storeOp:  VkAttachmentStoreOp,
    pub clearValue:  VkClearValue,
}

#[repr(C)]
pub struct VkRenderingAttachmentInfoKHR {
}

#[repr(C)]
pub struct VkRenderingFragmentShadingRateAttachmentInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub imageView:  VkImageView,
    pub imageLayout:  VkImageLayout,
    pub shadingRateAttachmentTexelSize:  VkExtent2D,
}

#[repr(C)]
pub struct VkRenderingFragmentDensityMapAttachmentInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub imageView:  VkImageView,
    pub imageLayout:  VkImageLayout,
}

#[repr(C)]
pub struct VkPhysicalDeviceDynamicRenderingFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub dynamicRendering:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDynamicRenderingFeaturesKHR {
}

#[repr(C)]
pub struct VkCommandBufferInheritanceRenderingInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkRenderingFlags,
    pub viewMask:  u32,
    pub colorAttachmentCount:  u32,
    //pub colorAttachmentCount:  u32,
    pub pColorAttachmentFormats: *const  VkFormat,
    pub depthAttachmentFormat:  VkFormat,
    pub stencilAttachmentFormat:  VkFormat,
    pub rasterizationSamples:  VkSampleCountFlagBits,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceRenderingInfoKHR {
}

#[repr(C)]
pub struct VkAttachmentSampleCountInfoAMD {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub colorAttachmentCount:  u32,
    pub pColorAttachmentSamples: *const  VkSampleCountFlagBits,
    pub depthStencilAttachmentSamples:  VkSampleCountFlagBits,
}

#[repr(C)]
pub struct VkAttachmentSampleCountInfoNV {
}

#[repr(C)]
pub struct VkMultiviewPerViewAttributesInfoNVX {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub perViewAttributes:  VkBool32,
    pub perViewAttributesPositionXOnly:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageViewMinLodFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub minLod:  VkBool32,
}

#[repr(C)]
pub struct VkImageViewMinLodCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub minLod:  f32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub rasterizationOrderColorAttachmentAccess:  VkBool32,
    pub rasterizationOrderDepthAttachmentAccess:  VkBool32,
    pub rasterizationOrderStencilAttachmentAccess:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRasterizationOrderAttachmentAccessFeaturesARM {
}

#[repr(C)]
pub struct VkPhysicalDeviceLinearColorAttachmentFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub linearColorAttachment:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceGraphicsPipelineLibraryFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub graphicsPipelineLibrary:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineBinaryFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pipelineBinaries:  VkBool32,
}

#[repr(C)]
pub struct VkDevicePipelineBinaryInternalCacheControlKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub disableInternalCache:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineBinaryPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pipelineBinaryInternalCache:  VkBool32,
    pub pipelineBinaryInternalCacheControl:  VkBool32,
    pub pipelineBinaryPrefersInternalCache:  VkBool32,
    pub pipelineBinaryPrecompiledInternalCache:  VkBool32,
    pub pipelineBinaryCompressedData:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceGraphicsPipelineLibraryPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub graphicsPipelineLibraryFastLinking:  VkBool32,
    pub graphicsPipelineLibraryIndependentInterpolationDecoration:  VkBool32,
}

#[repr(C)]
pub struct VkGraphicsPipelineLibraryCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkGraphicsPipelineLibraryFlagsEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceDataGraphNeuralAcceleratorStatisticsFeaturesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub dataGraphNeuralAcceleratorStatistics:  VkBool32,
}

#[repr(C)]
pub struct VkDataGraphPipelineNeuralStatisticsCreateInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub allowNeuralStatistics:  VkBool32,
}

#[repr(C)]
pub struct VkDataGraphPipelineSessionNeuralStatisticsCreateInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub mode:  VkNeuralAcceleratorStatisticsModeARM,
}

#[repr(C)]
pub struct VkTensorExplicitTilingFormatPropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub brick16TilingTensorFeatures:  VkFormatFeatureFlags2,
    pub brick8TilingTensorFeatures:  VkFormatFeatureFlags2,
    pub brick4TilingTensorFeatures:  VkFormatFeatureFlags2,
    pub blockUTilingTensorFeatures:  VkFormatFeatureFlags2,
    pub blockU64kTilingTensorFeatures:  VkFormatFeatureFlags2,
}

#[repr(C)]
pub struct VkTensorRollingBackingCreateInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub wraps:  [u32; VK_MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM as usize],
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorSetHostMappingFeaturesVALVE {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub descriptorSetHostMapping:  VkBool32,
}

#[repr(C)]
pub struct VkDescriptorSetBindingReferenceVALVE {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub descriptorSetLayout:  VkDescriptorSetLayout,
    pub binding:  u32,
}

#[repr(C)]
pub struct VkDescriptorSetLayoutHostMappingInfoVALVE {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub descriptorOffset:  usize,
    pub descriptorSize:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceNestedCommandBufferFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub nestedCommandBuffer:  VkBool32,
    pub nestedCommandBufferRendering:  VkBool32,
    pub nestedCommandBufferSimultaneousUse:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceNestedCommandBufferPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxCommandBufferNestingLevel:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderModuleIdentifierFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderModuleIdentifier:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderModuleIdentifierPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderModuleIdentifierAlgorithmUUID:  [u8; VK_UUID_SIZE as usize],
}

#[repr(C)]
pub struct VkPipelineShaderStageModuleIdentifierCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub identifierSize:  u32,
    pub pIdentifier: *const  u8,
}

#[repr(C)]
pub struct VkShaderModuleIdentifierEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub identifierSize:  u32,
    pub identifier:  [u8; VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT as usize],
}

#[repr(C)]
pub struct VkImageCompressionControlEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkImageCompressionFlagsEXT,
    pub compressionControlPlaneCount:  u32,
    pub pFixedRateFlags: *mut  VkImageCompressionFixedRateFlagsEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageCompressionControlFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub imageCompressionControl:  VkBool32,
}

#[repr(C)]
pub struct VkImageCompressionPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub imageCompressionFlags:  VkImageCompressionFlagsEXT,
    pub imageCompressionFixedRateFlags:  VkImageCompressionFixedRateFlagsEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageCompressionControlSwapchainFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub imageCompressionControlSwapchain:  VkBool32,
}

#[repr(C)]
pub struct VkImageSubresource2 {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub imageSubresource:  VkImageSubresource,
}

#[repr(C)]
pub struct VkImageSubresource2KHR {
}

#[repr(C)]
pub struct VkImageSubresource2EXT {
}

#[repr(C)]
pub struct VkSubresourceLayout2 {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub subresourceLayout:  VkSubresourceLayout,
}

#[repr(C)]
pub struct VkSubresourceLayout2KHR {
}

#[repr(C)]
pub struct VkSubresourceLayout2EXT {
}

#[repr(C)]
pub struct VkRenderPassCreationControlEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub disallowMerging:  VkBool32,
}

#[repr(C)]
pub struct VkRenderPassCreationFeedbackInfoEXT {
    pub postMergeSubpassCount:  u32,
}

#[repr(C)]
pub struct VkRenderPassCreationFeedbackCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pRenderPassFeedback: *mut  VkRenderPassCreationFeedbackInfoEXT,
}

#[repr(C)]
pub struct VkRenderPassSubpassFeedbackInfoEXT {
    pub subpassMergeStatus:  VkSubpassMergeStatusEXT,
    pub description:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
    pub postMergeIndex:  u32,
}

#[repr(C)]
pub struct VkRenderPassSubpassFeedbackCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pSubpassFeedback: *mut  VkRenderPassSubpassFeedbackInfoEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceSubpassMergeFeedbackFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub subpassMergeFeedback:  VkBool32,
}

#[repr(C)]
pub struct VkMicromapBuildInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub r#type:  VkMicromapTypeEXT,
    pub flags:  VkBuildMicromapFlagsEXT,
    pub mode:  VkBuildMicromapModeEXT,
    pub dstMicromap:  VkMicromapEXT,
    pub usageCountsCount:  u32,
    pub pUsageCounts: *const  VkMicromapUsageEXT,
    pub ppUsageCounts: *const  VkMicromapUsageEXT,
    pub data:  VkDeviceOrHostAddressConstKHR,
    pub scratchData:  VkDeviceOrHostAddressKHR,
    pub triangleArray:  VkDeviceOrHostAddressConstKHR,
    pub triangleArrayStride:  VkDeviceSize,
}

#[repr(C)]
pub struct VkAccelerationStructureGeometryMicromapDataKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub usageCountsCount:  u32,
    pub pUsageCounts: *const  VkMicromapUsageKHR,
    pub ppUsageCounts: *const  VkMicromapUsageKHR,
    pub data:  VkDeviceAddress,
    pub triangleArray:  VkDeviceAddress,
    pub triangleArrayStride:  VkDeviceSize,
}

#[repr(C)]
pub struct VkMicromapCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub createFlags:  VkMicromapCreateFlagsEXT,
    pub buffer:  VkBuffer,
    pub offset:  VkDeviceSize,
    pub size:  VkDeviceSize,
    pub r#type:  VkMicromapTypeEXT,
    pub deviceAddress:  VkDeviceAddress,
}

#[repr(C)]
pub struct VkMicromapVersionInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pVersionData: *const  u8,
}

#[repr(C)]
pub struct VkCopyMicromapInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub src:  VkMicromapEXT,
    pub dst:  VkMicromapEXT,
    pub mode:  VkCopyMicromapModeEXT,
}

#[repr(C)]
pub struct VkCopyMicromapToMemoryInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub src:  VkMicromapEXT,
    pub dst:  VkDeviceOrHostAddressKHR,
    pub mode:  VkCopyMicromapModeEXT,
}

#[repr(C)]
pub struct VkCopyMemoryToMicromapInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub src:  VkDeviceOrHostAddressConstKHR,
    pub dst:  VkMicromapEXT,
    pub mode:  VkCopyMicromapModeEXT,
}

#[repr(C)]
pub struct VkMicromapBuildSizesInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub micromapSize:  VkDeviceSize,
    pub buildScratchSize:  VkDeviceSize,
    pub discardable:  VkBool32,
}

#[repr(C)]
pub struct VkMicromapUsageKHR {
    pub count:  u32,
    pub subdivisionLevel:  u32,
    pub format:  VkOpacityMicromapFormatKHR,
}

#[repr(C)]
pub struct VkMicromapUsageEXT {
    pub count:  u32,
    pub subdivisionLevel:  u32,
    pub format:  u32,
}

#[repr(C)]
pub struct VkMicromapTriangleKHR {
    pub dataOffset:  u32,
    pub subdivisionLevel:  u16,
    pub format:  u16,
}

#[repr(C)]
pub struct VkMicromapTriangleEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceOpacityMicromapFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub micromap:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceOpacityMicromapFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub micromap:  VkBool32,
    pub micromapCaptureReplay:  VkBool32,
    pub micromapHostCommands:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceOpacityMicromapPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxOpacity2StateSubdivisionLevel:  u32,
    pub maxOpacity4StateSubdivisionLevel:  u32,
    pub maxOpacityLossy4StateSubdivisionLevel:  u32,
    pub maxMicromapTriangles:  u64,
}

#[repr(C)]
pub struct VkPhysicalDeviceOpacityMicromapPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxOpacity2StateSubdivisionLevel:  u32,
    pub maxOpacity4StateSubdivisionLevel:  u32,
}

#[repr(C)]
pub struct VkAccelerationStructureTrianglesOpacityMicromapKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub indexType:  VkIndexType,
    pub indexBuffer:  VkDeviceAddress,
    pub indexStride:  VkDeviceSize,
    pub baseTriangle:  u32,
    pub micromap:  VkAccelerationStructureKHR,
}

#[repr(C)]
pub struct VkAccelerationStructureTrianglesOpacityMicromapEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub indexType:  VkIndexType,
    pub indexBuffer:  VkDeviceOrHostAddressConstKHR,
    pub indexStride:  VkDeviceSize,
    pub baseTriangle:  u32,
    pub usageCountsCount:  u32,
    pub pUsageCounts: *const  VkMicromapUsageEXT,
    pub ppUsageCounts: *const  VkMicromapUsageEXT,
    pub micromap:  VkMicromapEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceDisplacementMicromapFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub displacementMicromap:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDisplacementMicromapPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxDisplacementMicromapSubdivisionLevel:  u32,
}

#[repr(C)]
pub struct VkAccelerationStructureTrianglesDisplacementMicromapNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub displacementBiasAndScaleFormat:  VkFormat,
    pub displacementVectorFormat:  VkFormat,
    pub displacementBiasAndScaleBuffer:  VkDeviceOrHostAddressConstKHR,
    pub displacementBiasAndScaleStride:  VkDeviceSize,
    pub displacementVectorBuffer:  VkDeviceOrHostAddressConstKHR,
    pub displacementVectorStride:  VkDeviceSize,
    pub displacedMicromapPrimitiveFlags:  VkDeviceOrHostAddressConstKHR,
    pub displacedMicromapPrimitiveFlagsStride:  VkDeviceSize,
    pub indexType:  VkIndexType,
    pub indexBuffer:  VkDeviceOrHostAddressConstKHR,
    pub indexStride:  VkDeviceSize,
    pub baseTriangle:  u32,
    pub usageCountsCount:  u32,
    pub pUsageCounts: *const  VkMicromapUsageEXT,
    pub ppUsageCounts: *const  VkMicromapUsageEXT,
    pub micromap:  VkMicromapEXT,
}

#[repr(C)]
pub struct VkPipelinePropertiesIdentifierEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pipelineIdentifier:  [u8; VK_UUID_SIZE as usize],
}

#[repr(C)]
pub struct VkPhysicalDevicePipelinePropertiesFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pipelinePropertiesIdentifier:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderEarlyAndLateFragmentTests:  VkBool32,
}

#[repr(C)]
pub struct VkExternalMemoryAcquireUnmodifiedEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub acquireUnmodifiedMemory:  VkBool32,
}

#[repr(C)]
pub struct VkExportMetalObjectCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub exportObjectType:  VkExportMetalObjectTypeFlagBitsEXT,
}

#[repr(C)]
pub struct VkExportMetalObjectsInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkExportMetalDeviceInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub mtlDevice:  MTLDevice_id,
}

#[repr(C)]
pub struct VkExportMetalCommandQueueInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub queue:  VkQueue,
    pub mtlCommandQueue:  MTLCommandQueue_id,
}

#[repr(C)]
pub struct VkExportMetalBufferInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub memory:  VkDeviceMemory,
    pub mtlBuffer:  MTLBuffer_id,
}

#[repr(C)]
pub struct VkImportMetalBufferInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub mtlBuffer:  MTLBuffer_id,
}

#[repr(C)]
pub struct VkExportMetalTextureInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub image:  VkImage,
    pub imageView:  VkImageView,
    pub bufferView:  VkBufferView,
    pub plane:  VkImageAspectFlagBits,
    pub mtlTexture:  MTLTexture_id,
}

#[repr(C)]
pub struct VkImportMetalTextureInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub plane:  VkImageAspectFlagBits,
    pub mtlTexture:  MTLTexture_id,
}

#[repr(C)]
pub struct VkExportMetalIOSurfaceInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub image:  VkImage,
    pub ioSurface:  IOSurfaceRef,
}

#[repr(C)]
pub struct VkImportMetalIOSurfaceInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub ioSurface:  IOSurfaceRef,
}

#[repr(C)]
pub struct VkExportMetalSharedEventInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub semaphore:  VkSemaphore,
    pub event:  VkEvent,
    pub mtlSharedEvent:  MTLSharedEvent_id,
}

#[repr(C)]
pub struct VkImportMetalSharedEventInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub mtlSharedEvent:  MTLSharedEvent_id,
}

#[repr(C)]
pub struct VkPhysicalDeviceNonSeamlessCubeMapFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub nonSeamlessCubeMap:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineRobustnessFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pipelineRobustness:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineRobustnessFeaturesEXT {
}

#[repr(C)]
pub struct VkPipelineRobustnessCreateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub storageBuffers:  VkPipelineRobustnessBufferBehavior,
    pub uniformBuffers:  VkPipelineRobustnessBufferBehavior,
    pub vertexInputs:  VkPipelineRobustnessBufferBehavior,
    pub images:  VkPipelineRobustnessImageBehavior,
}

#[repr(C)]
pub struct VkPipelineRobustnessCreateInfoEXT {
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineRobustnessProperties {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub defaultRobustnessStorageBuffers:  VkPipelineRobustnessBufferBehavior,
    pub defaultRobustnessUniformBuffers:  VkPipelineRobustnessBufferBehavior,
    pub defaultRobustnessVertexInputs:  VkPipelineRobustnessBufferBehavior,
    pub defaultRobustnessImages:  VkPipelineRobustnessImageBehavior,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineRobustnessPropertiesEXT {
}

#[repr(C)]
pub struct VkImageViewSampleWeightCreateInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub filterCenter:  VkOffset2D,
    pub filterSize:  VkExtent2D,
    pub numPhases:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderMultipleWaitQueuesFeaturesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderMultipleWaitQueues:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderMultipleWaitQueuesPropertiesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxShaderWaitQueues:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageProcessingFeaturesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub textureSampleWeighted:  VkBool32,
    pub textureBoxFilter:  VkBool32,
    pub textureBlockMatch:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageProcessingPropertiesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxWeightFilterPhases:  u32,
    pub maxWeightFilterDimension:  VkExtent2D,
    pub maxBlockMatchRegion:  VkExtent2D,
    pub maxBoxFilterBlockSize:  VkExtent2D,
}

#[repr(C)]
pub struct VkPhysicalDeviceTilePropertiesFeaturesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub tileProperties:  VkBool32,
}

#[repr(C)]
pub struct VkTilePropertiesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub tileSize:  VkExtent3D,
    pub apronSize:  VkExtent2D,
    pub origin:  VkOffset2D,
}

#[repr(C)]
pub struct VkTileMemoryBindInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub memory:  VkDeviceMemory,
}

#[repr(C)]
pub struct VkPhysicalDeviceAmigoProfilingFeaturesSEC {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub amigoProfiling:  VkBool32,
}

#[repr(C)]
pub struct VkAmigoProfilingSubmitInfoSEC {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub firstDrawTimestamp:  u64,
    pub swapBufferTimestamp:  u64,
}

#[repr(C)]
pub struct VkPhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub attachmentFeedbackLoopLayout:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthClampZeroOneFeaturesEXT {
}

#[repr(C)]
pub struct VkAttachmentFeedbackLoopInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub feedbackLoopEnable:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceAddressBindingReportFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub reportAddressBinding:  VkBool32,
}

#[repr(C)]
pub struct VkRenderingAttachmentFlagsInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkRenderingAttachmentFlagsKHR,
}

#[repr(C)]
pub struct VkResolveImageModeInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkResolveImageFlagsKHR,
    pub resolveMode:  VkResolveModeFlagBits,
    pub stencilResolveMode:  VkResolveModeFlagBits,
}

#[repr(C)]
pub struct VkDeviceAddressBindingCallbackDataEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkDeviceAddressBindingFlagsEXT,
    pub baseAddress:  VkDeviceAddress,
    pub size:  VkDeviceSize,
    pub bindingType:  VkDeviceAddressBindingTypeEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceOpticalFlowFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub opticalFlow:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceOpticalFlowPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub supportedOutputGridSizes:  VkOpticalFlowGridSizeFlagsNV,
    pub supportedHintGridSizes:  VkOpticalFlowGridSizeFlagsNV,
    pub hintSupported:  VkBool32,
    pub costSupported:  VkBool32,
    pub bidirectionalFlowSupported:  VkBool32,
    pub globalFlowSupported:  VkBool32,
    pub minWidth:  u32,
    pub minHeight:  u32,
    pub maxWidth:  u32,
    pub maxHeight:  u32,
    pub maxNumRegionsOfInterest:  u32,
}

#[repr(C)]
pub struct VkOpticalFlowImageFormatInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub usage:  VkOpticalFlowUsageFlagsNV,
}

#[repr(C)]
pub struct VkOpticalFlowImageFormatPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub format:  VkFormat,
}

#[repr(C)]
pub struct VkOpticalFlowSessionCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub width:  u32,
    pub height:  u32,
    pub imageFormat:  VkFormat,
    pub flowVectorFormat:  VkFormat,
    pub costFormat:  VkFormat,
    pub outputGridSize:  VkOpticalFlowGridSizeFlagsNV,
    pub hintGridSize:  VkOpticalFlowGridSizeFlagsNV,
    pub performanceLevel:  VkOpticalFlowPerformanceLevelNV,
    pub flags:  VkOpticalFlowSessionCreateFlagsNV,
}

#[repr(C)]
pub struct VkOpticalFlowSessionCreatePrivateDataInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub id:  u32,
    pub size:  u32,
    pub pPrivateData: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkOpticalFlowExecuteInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkOpticalFlowExecuteFlagsNV,
    pub regionCount:  u32,
    pub pRegions: *const  VkRect2D,
}

#[repr(C)]
pub struct VkPhysicalDeviceFaultFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub deviceFault:  VkBool32,
    pub deviceFaultVendorBinary:  VkBool32,
}

#[repr(C)]
pub struct VkDeviceFaultAddressInfoKHR {
    pub addressType:  VkDeviceFaultAddressTypeKHR,
    pub reportedAddress:  VkDeviceAddress,
    pub addressPrecision:  VkDeviceSize,
}

#[repr(C)]
pub struct VkDeviceFaultAddressInfoEXT {
}

#[repr(C)]
pub struct VkDeviceFaultVendorInfoKHR {
    pub description:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
    pub vendorFaultCode:  u64,
    pub vendorFaultData:  u64,
}

#[repr(C)]
pub struct VkDeviceFaultVendorInfoEXT {
}

#[repr(C)]
pub struct VkDeviceFaultInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkDeviceFaultFlagsKHR,
    pub groupId:  u64,
    pub description:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
    pub faultAddressInfo:  VkDeviceFaultAddressInfoKHR,
    pub instructionAddressInfo:  VkDeviceFaultAddressInfoKHR,
    pub vendorInfo:  VkDeviceFaultVendorInfoKHR,
}

#[repr(C)]
pub struct VkDeviceFaultDebugInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub vendorBinarySize:  u32,
    pub pVendorBinaryData: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkDeviceFaultCountsEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub addressInfoCount:  u32,
    pub vendorInfoCount:  u32,
    pub vendorBinarySize:  VkDeviceSize,
}

#[repr(C)]
pub struct VkDeviceFaultInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub description:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
    pub pAddressInfos: *mut  VkDeviceFaultAddressInfoKHR,
    pub pVendorInfos: *mut  VkDeviceFaultVendorInfoKHR,
    pub pVendorBinaryData: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkDeviceFaultVendorBinaryHeaderVersionOneKHR {
    pub headerSize:  u32,
    pub headerVersion:  VkDeviceFaultVendorBinaryHeaderVersionKHR,
    pub vendorID:  u32,
    pub deviceID:  u32,
    pub driverVersion:  u32,
    pub pipelineCacheUUID:  [u8; VK_UUID_SIZE as usize],
    pub applicationNameOffset:  u32,
    pub applicationVersion:  u32,
    pub engineNameOffset:  u32,
    pub engineVersion:  u32,
    pub apiVersion:  u32,
}

#[repr(C)]
pub struct VkDeviceFaultVendorBinaryHeaderVersionOneEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceFaultFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub deviceFault:  VkBool32,
    pub deviceFaultVendorBinary:  VkBool32,
    pub deviceFaultReportMasked:  VkBool32,
    pub deviceFaultDeviceLostOnMasked:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFaultPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxDeviceFaultCount:  u32,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pipelineLibraryGroupHandles:  VkBool32,
}

#[repr(C)]
pub struct VkDepthBiasInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub depthBiasConstantFactor:  f32,
    pub depthBiasClamp:  f32,
    pub depthBiasSlopeFactor:  f32,
}

#[repr(C)]
pub struct VkDepthBiasRepresentationInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub depthBiasRepresentation:  VkDepthBiasRepresentationEXT,
    pub depthBiasExact:  VkBool32,
}

#[repr(C)]
pub struct VkDecompressMemoryRegionNV {
    pub srcAddress:  VkDeviceAddress,
    pub dstAddress:  VkDeviceAddress,
    pub compressedSize:  VkDeviceSize,
    pub decompressedSize:  VkDeviceSize,
    pub decompressionMethod:  VkMemoryDecompressionMethodFlagsEXT,
}

#[repr(C)]
pub struct VkDecompressMemoryRegionEXT {
    pub srcAddress:  VkDeviceAddress,
    pub dstAddress:  VkDeviceAddress,
    pub compressedSize:  VkDeviceSize,
    pub decompressedSize:  VkDeviceSize,
}

#[repr(C)]
pub struct VkDecompressMemoryInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub decompressionMethod:  VkMemoryDecompressionMethodFlagsEXT,
    pub regionCount:  u32,
    pub pRegions: *const  VkDecompressMemoryRegionEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreBuiltinsPropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderCoreMask:  u64,
    pub shaderCoreCount:  u32,
    pub shaderWarpsPerCore:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderCoreBuiltinsFeaturesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderCoreBuiltins:  VkBool32,
}

#[repr(C)]
pub struct VkFrameBoundaryEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkFrameBoundaryFlagsEXT,
    pub frameID:  u64,
    pub imageCount:  u32,
    pub pImages: *const  VkImage,
    pub bufferCount:  u32,
    pub pBuffers: *const  VkBuffer,
    pub tagName:  u64,
    pub tagSize:  usize,
    pub pTag: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceFrameBoundaryFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub frameBoundary:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub dynamicRenderingUnusedAttachments:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceInternallySynchronizedQueuesFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub internallySynchronizedQueues:  VkBool32,
}

#[repr(C)]
pub struct VkSurfacePresentModeKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentMode:  VkPresentModeKHR,
}

#[repr(C)]
pub struct VkSurfacePresentModeEXT {
}

#[repr(C)]
pub struct VkSurfacePresentScalingCapabilitiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub supportedPresentScaling:  VkPresentScalingFlagsKHR,
    pub supportedPresentGravityX:  VkPresentGravityFlagsKHR,
    pub supportedPresentGravityY:  VkPresentGravityFlagsKHR,
    pub minScaledImageExtent:  VkExtent2D,
    pub maxScaledImageExtent:  VkExtent2D,
}

#[repr(C)]
pub struct VkSurfacePresentScalingCapabilitiesEXT {
}

#[repr(C)]
pub struct VkSurfacePresentModeCompatibilityKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentModeCount:  u32,
    pub pPresentModes: *mut  VkPresentModeKHR,
}

#[repr(C)]
pub struct VkSurfacePresentModeCompatibilityEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceSwapchainMaintenance1FeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub swapchainMaintenance1:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSwapchainMaintenance1FeaturesEXT {
}

#[repr(C)]
pub struct VkSwapchainPresentFenceInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub swapchainCount:  u32,
    pub pFences: *const  VkFence,
}

#[repr(C)]
pub struct VkSwapchainPresentFenceInfoEXT {
}

#[repr(C)]
pub struct VkSwapchainPresentModesCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub presentModeCount:  u32,
    pub pPresentModes: *const  VkPresentModeKHR,
}

#[repr(C)]
pub struct VkSwapchainPresentModesCreateInfoEXT {
}

#[repr(C)]
pub struct VkSwapchainPresentModeInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub swapchainCount:  u32,
    pub pPresentModes: *const  VkPresentModeKHR,
}

#[repr(C)]
pub struct VkSwapchainPresentModeInfoEXT {
}

#[repr(C)]
pub struct VkSwapchainPresentScalingCreateInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub scalingBehavior:  VkPresentScalingFlagsKHR,
    pub presentGravityX:  VkPresentGravityFlagsKHR,
    pub presentGravityY:  VkPresentGravityFlagsKHR,
}

#[repr(C)]
pub struct VkSwapchainPresentScalingCreateInfoEXT {
}

#[repr(C)]
pub struct VkReleaseSwapchainImagesInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub swapchain:  VkSwapchainKHR,
    pub imageIndexCount:  u32,
    pub pImageIndices: *const  u32,
}

#[repr(C)]
pub struct VkReleaseSwapchainImagesInfoEXT {
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthBiasControlFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub depthBiasControl:  VkBool32,
    pub leastRepresentableValueForceUnormRepresentation:  VkBool32,
    pub floatRepresentation:  VkBool32,
    pub depthBiasExact:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub rayTracingInvocationReorder:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub rayTracingInvocationReorder:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub rayTracingInvocationReorderReorderingHint:  VkRayTracingInvocationReorderModeEXT,
    pub maxShaderBindingTableRecordIndex:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingInvocationReorderPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub rayTracingInvocationReorderReorderingHint:  VkRayTracingInvocationReorderModeEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedSparseAddressSpaceFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub extendedSparseAddressSpace:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExtendedSparseAddressSpacePropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub extendedSparseAddressSpaceSize:  VkDeviceSize,
    pub extendedSparseImageUsageFlags:  VkImageUsageFlags,
    pub extendedSparseBufferUsageFlags:  VkBufferUsageFlags,
}

#[repr(C)]
pub struct VkDirectDriverLoadingInfoLUNARG {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkDirectDriverLoadingFlagsLUNARG,
    //pub pfnGetInstanceProcAddr:  crate::svk_commands::PFN_vkGetInstanceProcAddrLUNARG,
}

#[repr(C)]
pub struct VkDirectDriverLoadingListLUNARG {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub mode:  VkDirectDriverLoadingModeLUNARG,
    pub driverCount:  u32,
    pub pDrivers: *const  VkDirectDriverLoadingInfoLUNARG,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub multiviewPerViewViewports:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRayTracingPositionFetchFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub rayTracingPositionFetch:  VkBool32,
}

#[repr(C)]
pub struct VkDeviceImageSubresourceInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pCreateInfo: *const  VkImageCreateInfo,
    pub pSubresource: *const  VkImageSubresource2,
}

#[repr(C)]
pub struct VkDeviceImageSubresourceInfoKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderCorePropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pixelRate:  u32,
    pub texelRate:  u32,
    pub fmaRate:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub multiviewPerViewRenderAreas:  VkBool32,
}

#[repr(C)]
pub struct VkMultiviewPerViewRenderAreasRenderPassBeginInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub perViewRenderAreaCount:  u32,
    pub pPerViewRenderAreas: *const  VkRect2D,
}

#[repr(C)]
pub struct VkQueryLowLatencySupportNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pQueriedLowLatencyData: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkMemoryMapInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkMemoryMapFlags,
    pub memory:  VkDeviceMemory,
    pub offset:  VkDeviceSize,
    pub size:  VkDeviceSize,
}

#[repr(C)]
pub struct VkMemoryMapInfoKHR {
}

#[repr(C)]
pub struct VkMemoryUnmapInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkMemoryUnmapFlags,
    pub memory:  VkDeviceMemory,
}

#[repr(C)]
pub struct VkMemoryUnmapInfoKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderObjectFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderObject:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderObjectPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderBinaryUUID:  [u8; VK_UUID_SIZE as usize],
    pub shaderBinaryVersion:  u32,
}

#[repr(C)]
pub struct VkShaderCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkShaderCreateFlagsEXT,
    pub stage:  VkShaderStageFlagBits,
    pub nextStage:  VkShaderStageFlags,
    pub codeType:  VkShaderCodeTypeEXT,
    pub codeSize:  usize,
    pub pCode: *const  std::ffi::c_void,
    pub pName: *const  i8,
    pub setLayoutCount:  u32,
    pub pSetLayouts: *const  VkDescriptorSetLayout,
    pub pushConstantRangeCount:  u32,
    pub pPushConstantRanges: *const  VkPushConstantRange,
    pub pSpecializationInfo: *const  VkSpecializationInfo,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderTileImageFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderTileImageColorReadAccess:  VkBool32,
    pub shaderTileImageDepthReadAccess:  VkBool32,
    pub shaderTileImageStencilReadAccess:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderTileImagePropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderTileImageCoherentReadAccelerated:  VkBool32,
    pub shaderTileImageReadSampleFromPixelRateInvocation:  VkBool32,
    pub shaderTileImageReadFromHelperInvocation:  VkBool32,
}

#[repr(C)]
pub struct VkImportScreenBufferInfoQNX {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub buffer: *mut  _screen_buffer,
}

#[repr(C)]
pub struct VkScreenBufferPropertiesQNX {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub allocationSize:  VkDeviceSize,
    pub memoryTypeBits:  u32,
}

#[repr(C)]
pub struct VkScreenBufferFormatPropertiesQNX {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub format:  VkFormat,
    pub externalFormat:  u64,
    pub screenUsage:  u64,
    pub formatFeatures:  VkFormatFeatureFlags,
    pub samplerYcbcrConversionComponents:  VkComponentMapping,
    pub suggestedYcbcrModel:  VkSamplerYcbcrModelConversion,
    pub suggestedYcbcrRange:  VkSamplerYcbcrRange,
    pub suggestedXChromaOffset:  VkChromaLocation,
    pub suggestedYChromaOffset:  VkChromaLocation,
}

#[repr(C)]
pub struct VkExternalFormatQNX {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub externalFormat:  u64,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalMemoryScreenBufferFeaturesQNX {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub screenBufferImport:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub cooperativeMatrix:  VkBool32,
    pub cooperativeMatrixRobustBufferAccess:  VkBool32,
}

#[repr(C)]
pub struct VkCooperativeMatrixPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub MSize:  u32,
    pub NSize:  u32,
    pub KSize:  u32,
    pub AType:  VkComponentTypeKHR,
    pub BType:  VkComponentTypeKHR,
    pub CType:  VkComponentTypeKHR,
    pub ResultType:  VkComponentTypeKHR,
    pub saturatingAccumulation:  VkBool32,
    pub scope:  VkScopeKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixInfo2EXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub scope:  VkScopeKHR,
    pub invocations:  u32,
    pub subgroupSize:  u32,
    pub flags:  VkCooperativeMatrixFlagsEXT,
}

#[repr(C)]
pub struct VkCooperativeMatrixProperties2EXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub MGranularity:  u32,
    pub NGranularity:  u32,
    pub KGranularity:  u32,
    pub AType:  VkComponentTypeKHR,
    pub BType:  VkComponentTypeKHR,
    pub CType:  VkComponentTypeKHR,
    pub ResultType:  VkComponentTypeKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub cooperativeMatrixSupportedStages:  VkShaderStageFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixConversionFeaturesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub cooperativeMatrixConversion:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderEnqueuePropertiesAMDX {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxExecutionGraphDepth:  u32,
    pub maxExecutionGraphShaderOutputNodes:  u32,
    pub maxExecutionGraphShaderPayloadSize:  u32,
    pub maxExecutionGraphShaderPayloadCount:  u32,
    pub executionGraphDispatchAddressAlignment:  u32,
    pub maxExecutionGraphWorkgroupCount:  u32,
    pub maxExecutionGraphWorkgroups:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderEnqueueFeaturesAMDX {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderEnqueue:  VkBool32,
    pub shaderMeshEnqueue:  VkBool32,
}

#[repr(C)]
pub struct VkExecutionGraphPipelineCreateInfoAMDX {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineCreateFlags,
    pub stageCount:  u32,
    pub pStages: *const  VkPipelineShaderStageCreateInfo,
    pub pLibraryInfo: *const  VkPipelineLibraryCreateInfoKHR,
    pub layout:  VkPipelineLayout,
    pub basePipelineHandle:  VkPipeline,
    pub basePipelineIndex:  i32,
}

#[repr(C)]
pub struct VkPipelineShaderStageNodeCreateInfoAMDX {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pName: *const  i8,
    pub index:  u32,
}

#[repr(C)]
pub struct VkExecutionGraphPipelineScratchSizeAMDX {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub minSize:  VkDeviceSize,
    pub maxSize:  VkDeviceSize,
    pub sizeGranularity:  VkDeviceSize,
}

#[repr(C)]
pub struct VkDispatchGraphInfoAMDX {
    pub nodeIndex:  u32,
    pub payloadCount:  u32,
    pub payloads:  VkDeviceOrHostAddressConstAMDX,
    pub payloadStride:  u64,
}

#[repr(C)]
pub struct VkDispatchGraphCountInfoAMDX {
    pub count:  u32,
    pub infos:  VkDeviceOrHostAddressConstAMDX,
    pub stride:  u64,
}

#[repr(C)]
pub struct VkPhysicalDeviceAntiLagFeaturesAMD {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub antiLag:  VkBool32,
}

#[repr(C)]
pub struct VkAntiLagDataAMD {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub mode:  VkAntiLagModeAMD,
    pub maxFPS:  u32,
    pub pPresentationInfo: *const  VkAntiLagPresentationInfoAMD,
}

#[repr(C)]
pub struct VkAntiLagPresentationInfoAMD {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub stage:  VkAntiLagStageAMD,
    pub frameIndex:  u64,
}

#[repr(C)]
pub struct VkBindMemoryStatus {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pResult: *mut  VkResult,
}

#[repr(C)]
pub struct VkPhysicalDeviceTileMemoryHeapFeaturesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub tileMemoryHeap:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceTileMemoryHeapPropertiesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub queueSubmitBoundary:  VkBool32,
    pub tileBufferTransfers:  VkBool32,
}

#[repr(C)]
pub struct VkTileMemorySizeInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub size:  VkDeviceSize,
}

#[repr(C)]
pub struct VkTileMemoryRequirementsQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub size:  VkDeviceSize,
    pub alignment:  VkDeviceSize,
}

#[repr(C)]
pub struct VkBindMemoryStatusKHR {
}

#[repr(C)]
pub struct VkBindDescriptorSetsInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stageFlags:  VkShaderStageFlags,
    pub layout:  VkPipelineLayout,
    pub firstSet:  u32,
    pub descriptorSetCount:  u32,
    pub pDescriptorSets: *const  VkDescriptorSet,
    pub dynamicOffsetCount:  u32,
    pub pDynamicOffsets: *const  u32,
}

#[repr(C)]
pub struct VkBindDescriptorSetsInfoKHR {
}

#[repr(C)]
pub struct VkPushConstantsInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub layout:  VkPipelineLayout,
    pub stageFlags:  VkShaderStageFlags,
    pub offset:  u32,
    pub size:  u32,
    pub pValues: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPushConstantsInfoKHR {
}

#[repr(C)]
pub struct VkPushDescriptorSetInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stageFlags:  VkShaderStageFlags,
    pub layout:  VkPipelineLayout,
    pub set:  u32,
    pub descriptorWriteCount:  u32,
    pub pDescriptorWrites: *const  VkWriteDescriptorSet,
}

#[repr(C)]
pub struct VkPushDescriptorSetInfoKHR {
}

#[repr(C)]
pub struct VkPushDescriptorSetWithTemplateInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub descriptorUpdateTemplate:  VkDescriptorUpdateTemplate,
    pub layout:  VkPipelineLayout,
    pub set:  u32,
    pub pData: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPushDescriptorSetWithTemplateInfoKHR {
}

#[repr(C)]
pub struct VkSetDescriptorBufferOffsetsInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stageFlags:  VkShaderStageFlags,
    pub layout:  VkPipelineLayout,
    pub firstSet:  u32,
    pub setCount:  u32,
    pub pBufferIndices: *const  u32,
    pub pOffsets: *const  VkDeviceSize,
}

#[repr(C)]
pub struct VkBindDescriptorBufferEmbeddedSamplersInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stageFlags:  VkShaderStageFlags,
    pub layout:  VkPipelineLayout,
    pub set:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCubicClampFeaturesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub cubicRangeClamp:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceYcbcrDegammaFeaturesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub ycbcrDegamma:  VkBool32,
}

#[repr(C)]
pub struct VkSamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub enableYDegamma:  VkBool32,
    pub enableCbCrDegamma:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCubicWeightsFeaturesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub selectableCubicWeights:  VkBool32,
}

#[repr(C)]
pub struct VkSamplerCubicWeightsCreateInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub cubicWeights:  VkCubicFilterWeightsQCOM,
}

#[repr(C)]
pub struct VkBlitImageCubicWeightsInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub cubicWeights:  VkCubicFilterWeightsQCOM,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageProcessing2FeaturesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub textureBlockMatch2:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageProcessing2PropertiesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxBlockMatchWindow:  VkExtent2D,
}

#[repr(C)]
pub struct VkSamplerBlockMatchWindowCreateInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub windowExtent:  VkExtent2D,
    pub windowCompareMode:  VkBlockMatchWindowCompareModeQCOM,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageProcessing3FeaturesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub imageGatherLinear:  VkBool32,
    pub imageGatherExtendedModes:  VkBool32,
    pub blockMatchExtendedClampToEdge:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorPoolOverallocationFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub descriptorPoolOverallocation:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceLayeredDriverPropertiesMSFT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub underlyingAPI:  VkLayeredDriverUnderlyingApiMSFT,
}

#[repr(C)]
pub struct VkPhysicalDevicePerStageDescriptorSetFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub perStageDescriptorSet:  VkBool32,
    pub dynamicPipelineLayout:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalFormatResolveFeaturesANDROID {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub externalFormatResolve:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalFormatResolvePropertiesANDROID {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub nullColorAttachmentWithExternalFormatResolve:  VkBool32,
    pub externalFormatResolveChromaOffsetX:  VkChromaLocation,
    pub externalFormatResolveChromaOffsetY:  VkChromaLocation,
}

#[repr(C)]
pub struct VkAndroidHardwareBufferFormatResolvePropertiesANDROID {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub colorAttachmentFormat:  VkFormat,
}

#[repr(C)]
pub struct VkLatencySleepModeInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub lowLatencyMode:  VkBool32,
    pub lowLatencyBoost:  VkBool32,
    pub minimumIntervalUs:  u32,
}

#[repr(C)]
pub struct VkLatencySleepInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub signalSemaphore:  VkSemaphore,
    pub value:  u64,
}

#[repr(C)]
pub struct VkSetLatencyMarkerInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub presentID:  u64,
    pub marker:  VkLatencyMarkerNV,
}

#[repr(C)]
pub struct VkGetLatencyMarkerInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub timingCount:  u32,
    pub pTimings: *mut  VkLatencyTimingsFrameReportNV,
}

#[repr(C)]
pub struct VkLatencyTimingsFrameReportNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentID:  u64,
    pub inputSampleTimeUs:  u64,
    pub simStartTimeUs:  u64,
    pub simEndTimeUs:  u64,
    pub renderSubmitStartTimeUs:  u64,
    pub renderSubmitEndTimeUs:  u64,
    pub presentStartTimeUs:  u64,
    pub presentEndTimeUs:  u64,
    pub driverStartTimeUs:  u64,
    pub driverEndTimeUs:  u64,
    pub osRenderQueueStartTimeUs:  u64,
    pub osRenderQueueEndTimeUs:  u64,
    pub gpuRenderStartTimeUs:  u64,
    pub gpuRenderEndTimeUs:  u64,
}

#[repr(C)]
pub struct VkOutOfBandQueueTypeInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub queueType:  VkOutOfBandQueueTypeNV,
}

#[repr(C)]
pub struct VkLatencySubmissionPresentIdNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub presentID:  u64,
}

#[repr(C)]
pub struct VkSwapchainLatencyCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub latencyModeEnable:  VkBool32,
}

#[repr(C)]
pub struct VkLatencySurfaceCapabilitiesNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub presentModeCount:  u32,
    pub pPresentModes: *mut  VkPresentModeKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceCudaKernelLaunchFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub cudaKernelLaunchFeatures:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCudaKernelLaunchPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub computeCapabilityMinor:  u32,
    pub computeCapabilityMajor:  u32,
}

#[repr(C)]
pub struct VkDeviceQueueShaderCoreControlCreateInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderCoreCount:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSchedulingControlsFeaturesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub schedulingControls:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceSchedulingControlsPropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub schedulingControlsFlags:  VkPhysicalDeviceSchedulingControlsFlagsARM,
}

#[repr(C)]
pub struct VkPhysicalDeviceSchedulingControlsDispatchParametersPropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub schedulingControlsMaxWarpsCount:  u32,
    pub schedulingControlsMaxQueuedBatchesCount:  u32,
    pub schedulingControlsMaxWorkGroupBatchSize:  u32,
}

#[repr(C)]
pub struct VkDispatchParametersARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub workGroupBatchSize:  u32,
    pub maxQueuedWorkGroupBatches:  u32,
    pub maxWarpsPerShaderCore:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRelaxedLineRasterizationFeaturesIMG {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub relaxedLineRasterization:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRenderPassStripedFeaturesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub renderPassStriped:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRenderPassStripedPropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub renderPassStripeGranularity:  VkExtent2D,
    pub maxRenderPassStripes:  u32,
}

#[repr(C)]
pub struct VkRenderPassStripeInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stripeArea:  VkRect2D,
}

#[repr(C)]
pub struct VkRenderPassStripeBeginInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stripeInfoCount:  u32,
    pub pStripeInfos: *const  VkRenderPassStripeInfoARM,
}

#[repr(C)]
pub struct VkRenderPassStripeSubmitInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stripeSemaphoreInfoCount:  u32,
    pub pStripeSemaphoreInfos: *const  VkSemaphoreSubmitInfo,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineOpacityMicromapFeaturesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pipelineOpacityMicromap:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderMaximalReconvergenceFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderMaximalReconvergence:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupRotateFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderSubgroupRotate:  VkBool32,
    pub shaderSubgroupRotateClustered:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupRotateFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderExpectAssumeFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderExpectAssume:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderExpectAssumeFeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderFloatControls2Features {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderFloatControls2:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderFloatControls2FeaturesKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceDynamicRenderingLocalReadFeatures {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub dynamicRenderingLocalRead:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDynamicRenderingLocalReadFeaturesKHR {
}

#[repr(C)]
pub struct VkRenderingAttachmentLocationInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub colorAttachmentCount:  u32,
    pub pColorAttachmentLocations: *const  u32,
}

#[repr(C)]
pub struct VkRenderingAttachmentLocationInfoKHR {
}

#[repr(C)]
pub struct VkRenderingInputAttachmentIndexInfo {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub colorAttachmentCount:  u32,
    pub pColorAttachmentInputIndices: *const  u32,
    pub pDepthInputAttachmentIndex: *const  u32,
    pub pStencilInputAttachmentIndex: *const  u32,
}

#[repr(C)]
pub struct VkRenderingInputAttachmentIndexInfoKHR {
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderQuadControlFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderQuadControl:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderAtomicFloat16VectorFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderFloat16VectorAtomics:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMapMemoryPlacedFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub memoryMapPlaced:  VkBool32,
    pub memoryMapRangePlaced:  VkBool32,
    pub memoryUnmapReserve:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceMapMemoryPlacedPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub minPlacedMemoryMapAlignment:  VkDeviceSize,
}

#[repr(C)]
pub struct VkMemoryMapPlacedInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pPlacedAddress: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderBfloat16FeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderBFloat16Type:  VkBool32,
    pub shaderBFloat16DotProduct:  VkBool32,
    pub shaderBFloat16CooperativeMatrix:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceRawAccessChainsFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderRawAccessChains:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCommandBufferInheritanceFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub commandBufferInheritance:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageAlignmentControlFeaturesMESA {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub imageAlignmentControl:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageAlignmentControlPropertiesMESA {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub supportedImageAlignmentMask:  u32,
}

#[repr(C)]
pub struct VkImageAlignmentControlCreateInfoMESA {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub maximumRequestedAlignment:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderReplicatedCompositesFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderReplicatedComposites:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentModeFifoLatestReadyFeaturesEXT {
}

#[repr(C)]
pub struct VkPhysicalDevicePresentModeFifoLatestReadyFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentModeFifoLatestReady:  VkBool32,
}

#[repr(C)]
pub struct VkDepthClampRangeEXT {
    pub minDepthClamp:  f32,
    pub maxDepthClamp:  f32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrix2FeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub cooperativeMatrixWorkgroupScope:  VkBool32,
    pub cooperativeMatrixFlexibleDimensions:  VkBool32,
    pub cooperativeMatrixReductions:  VkBool32,
    pub cooperativeMatrixConversions:  VkBool32,
    pub cooperativeMatrixPerElementOperations:  VkBool32,
    pub cooperativeMatrixTensorAddressing:  VkBool32,
    pub cooperativeMatrixBlockLoads:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrix2PropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub cooperativeMatrixWorkgroupScopeMaxWorkgroupSize:  u32,
    pub cooperativeMatrixFlexibleDimensionsMaxDimension:  u32,
    pub cooperativeMatrixWorkgroupScopeReservedSharedMemory:  u32,
}

#[repr(C)]
pub struct VkCooperativeMatrixFlexibleDimensionsPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub MGranularity:  u32,
    pub NGranularity:  u32,
    pub KGranularity:  u32,
    pub AType:  VkComponentTypeKHR,
    pub BType:  VkComponentTypeKHR,
    pub CType:  VkComponentTypeKHR,
    pub ResultType:  VkComponentTypeKHR,
    pub saturatingAccumulation:  VkBool32,
    pub scope:  VkScopeKHR,
    pub workgroupInvocations:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixDecodeVectorFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub cooperativeMatrixDecodeVector:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceHdrVividFeaturesHUAWEI {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub hdrVivid:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVertexAttributeRobustnessFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub vertexAttributeRobustness:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDenseGeometryFormatFeaturesAMDX {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub denseGeometryFormat:  VkBool32,
}

#[repr(C)]
pub struct VkAccelerationStructureDenseGeometryFormatTrianglesDataAMDX {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub compressedData:  VkDeviceOrHostAddressConstKHR,
    pub dataSize:  VkDeviceSize,
    pub numTriangles:  u32,
    pub numVertices:  u32,
    pub maxPrimitiveIndex:  u32,
    pub maxGeometryIndex:  u32,
    pub format:  VkCompressedTriangleFormatAMDX,
}

#[repr(C)]
pub struct VkPhysicalDeviceDepthClampZeroOneFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub depthClampZeroOne:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeVectorFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub cooperativeVector:  VkBool32,
    pub cooperativeVectorTraining:  VkBool32,
}

#[repr(C)]
pub struct VkCooperativeVectorPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub inputType:  VkComponentTypeKHR,
    pub inputInterpretation:  VkComponentTypeKHR,
    pub matrixInterpretation:  VkComponentTypeKHR,
    pub biasInterpretation:  VkComponentTypeKHR,
    pub resultType:  VkComponentTypeKHR,
    pub transpose:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeVectorPropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub cooperativeVectorSupportedStages:  VkShaderStageFlags,
    pub cooperativeVectorTrainingFloat16Accumulation:  VkBool32,
    pub cooperativeVectorTrainingFloat32Accumulation:  VkBool32,
    pub maxCooperativeVectorComponents:  u32,
}

#[repr(C)]
pub struct VkConvertCooperativeVectorMatrixInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcSize:  usize,
    pub srcData:  VkDeviceOrHostAddressConstKHR,
    pub pDstSize: *mut  usize,
    pub dstData:  VkDeviceOrHostAddressKHR,
    pub srcComponentType:  VkComponentTypeKHR,
    pub dstComponentType:  VkComponentTypeKHR,
    pub numRows:  u32,
    pub numColumns:  u32,
    pub srcLayout:  VkCooperativeVectorMatrixLayoutNV,
    pub srcStride:  usize,
    pub dstLayout:  VkCooperativeVectorMatrixLayoutNV,
    pub dstStride:  usize,
}

#[repr(C)]
pub struct VkPhysicalDeviceTileShadingFeaturesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub tileShading:  VkBool32,
    pub tileShadingFragmentStage:  VkBool32,
    pub tileShadingColorAttachments:  VkBool32,
    pub tileShadingDepthAttachments:  VkBool32,
    pub tileShadingStencilAttachments:  VkBool32,
    pub tileShadingInputAttachments:  VkBool32,
    pub tileShadingSampledAttachments:  VkBool32,
    pub tileShadingPerTileDraw:  VkBool32,
    pub tileShadingPerTileDispatch:  VkBool32,
    pub tileShadingDispatchTile:  VkBool32,
    pub tileShadingApron:  VkBool32,
    pub tileShadingAnisotropicApron:  VkBool32,
    pub tileShadingAtomicOps:  VkBool32,
    pub tileShadingImageProcessing:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceTileShadingPropertiesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxApronSize:  u32,
    pub preferNonCoherent:  VkBool32,
    pub tileGranularity:  VkExtent2D,
    pub maxTileShadingRate:  VkExtent2D,
}

#[repr(C)]
pub struct VkRenderPassTileShadingCreateInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkTileShadingRenderPassFlagsQCOM,
    pub tileApronSize:  VkExtent2D,
}

#[repr(C)]
pub struct VkPerTileBeginInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPerTileEndInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkDispatchTileInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxFragmentDensityMapLayers:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub fragmentDensityMapLayered:  VkBool32,
}

#[repr(C)]
pub struct VkPipelineFragmentDensityMapLayeredCreateInfoVALVE {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub maxFragmentDensityMapLayers:  u32,
}

#[repr(C)]
pub struct VkSetPresentConfigNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub numFramesPerBatch:  u32,
    pub presentConfigFeedback:  u32,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentMeteringFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub presentMetering:  VkBool32,
}

#[repr(C)]
pub struct VkExternalComputeQueueDeviceCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub reservedExternalQueues:  u32,
}

#[repr(C)]
pub struct VkExternalComputeQueueCreateInfoNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub preferredQueue:  VkQueue,
}

#[repr(C)]
pub struct VkExternalComputeQueueDataParamsNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub deviceIndex:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalComputeQueuePropertiesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub externalDataSize:  u32,
    pub maxExternalQueues:  u32,
}

#[repr(C)]
pub struct VkExternalComputeQueueNV {
    private: [u8; 0]
}
#[repr(C)]
pub struct VkPhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderUniformBufferUnsizedArray:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderMixedFloatDotProductFeaturesVALVE {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderMixedFloatDotProductFloat16AccFloat32:  VkBool32,
    pub shaderMixedFloatDotProductFloat16AccFloat16:  VkBool32,
    pub shaderMixedFloatDotProductBFloat16Acc:  VkBool32,
    pub shaderMixedFloatDotProductFloat8AccFloat32:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePrimitiveRestartIndexFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub primitiveRestartIndex:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceFormatPackFeaturesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub formatPack:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceThrottleHintFeaturesSEC {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub throttleHint:  VkBool32,
}

#[repr(C)]
pub struct VkThrottleHintSubmitInfoSEC {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub throttleHint:  VkThrottleHintTypeSEC,
}

#[repr(C)]
pub struct VkTensorDescriptionARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub tiling:  VkTensorTilingARM,
    pub format:  VkFormat,
    pub dimensionCount:  u32,
    pub pDimensions: *const  i64,
    pub pStrides: *const  i64,
    pub usage:  VkTensorUsageFlagsARM,
}

#[repr(C)]
pub struct VkTensorCreateInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkTensorCreateFlagsARM,
    pub pDescription: *const  VkTensorDescriptionARM,
    pub sharingMode:  VkSharingMode,
    pub queueFamilyIndexCount:  u32,
    pub pQueueFamilyIndices: *const  u32,
}

#[repr(C)]
pub struct VkTensorViewCreateInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkTensorViewCreateFlagsARM,
    pub tensor:  VkTensorARM,
    pub format:  VkFormat,
}

#[repr(C)]
pub struct VkTensorMemoryRequirementsInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub tensor:  VkTensorARM,
}

#[repr(C)]
pub struct VkBindTensorMemoryInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub tensor:  VkTensorARM,
    pub memory:  VkDeviceMemory,
    pub memoryOffset:  VkDeviceSize,
}

#[repr(C)]
pub struct VkWriteDescriptorSetTensorARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub tensorViewCount:  u32,
    pub pTensorViews: *const  VkTensorViewARM,
}

#[repr(C)]
pub struct VkTensorFormatPropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub optimalTilingTensorFeatures:  VkFormatFeatureFlags2,
    pub linearTilingTensorFeatures:  VkFormatFeatureFlags2,
}

#[repr(C)]
pub struct VkPhysicalDeviceTensorPropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxTensorDimensionCount:  u32,
    pub maxTensorElements:  u64,
    pub maxPerDimensionTensorElements:  u64,
    pub maxTensorStride:  i64,
    pub maxTensorSize:  u64,
    pub maxTensorShaderAccessArrayLength:  u32,
    pub maxTensorShaderAccessSize:  u32,
    pub maxDescriptorSetStorageTensors:  u32,
    pub maxPerStageDescriptorSetStorageTensors:  u32,
    pub maxDescriptorSetUpdateAfterBindStorageTensors:  u32,
    pub maxPerStageDescriptorUpdateAfterBindStorageTensors:  u32,
    pub shaderStorageTensorArrayNonUniformIndexingNative:  VkBool32,
    pub shaderTensorSupportedStages:  VkShaderStageFlags,
}

#[repr(C)]
pub struct VkTensorMemoryBarrierARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcStageMask:  VkPipelineStageFlags2,
    pub srcAccessMask:  VkAccessFlags2,
    pub dstStageMask:  VkPipelineStageFlags2,
    pub dstAccessMask:  VkAccessFlags2,
    pub srcQueueFamilyIndex:  u32,
    pub dstQueueFamilyIndex:  u32,
    pub tensor:  VkTensorARM,
}

#[repr(C)]
pub struct VkTensorDependencyInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub tensorMemoryBarrierCount:  u32,
    pub pTensorMemoryBarriers: *const  VkTensorMemoryBarrierARM,
}

#[repr(C)]
pub struct VkPhysicalDeviceTensorFeaturesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub tensorNonPacked:  VkBool32,
    pub shaderTensorAccess:  VkBool32,
    pub shaderStorageTensorArrayDynamicIndexing:  VkBool32,
    pub shaderStorageTensorArrayNonUniformIndexing:  VkBool32,
    pub descriptorBindingStorageTensorUpdateAfterBind:  VkBool32,
    pub tensors:  VkBool32,
}

#[repr(C)]
pub struct VkDeviceTensorMemoryRequirementsARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pCreateInfo: *const  VkTensorCreateInfoARM,
}

#[repr(C)]
pub struct VkCopyTensorInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcTensor:  VkTensorARM,
    pub dstTensor:  VkTensorARM,
    pub regionCount:  u32,
    pub pRegions: *const  VkTensorCopyARM,
}

#[repr(C)]
pub struct VkTensorCopyARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub dimensionCount:  u32,
    pub pSrcOffset: *const  u64,
    pub pDstOffset: *const  u64,
    pub pExtent: *const  u64,
}

#[repr(C)]
pub struct VkMemoryDedicatedAllocateInfoTensorARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub tensor:  VkTensorARM,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferTensorPropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub tensorCaptureReplayDescriptorDataSize:  usize,
    pub tensorViewCaptureReplayDescriptorDataSize:  usize,
    pub tensorDescriptorSize:  usize,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorBufferTensorFeaturesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub descriptorBufferTensorDescriptors:  VkBool32,
}

#[repr(C)]
pub struct VkTensorCaptureDescriptorDataInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub tensor:  VkTensorARM,
}

#[repr(C)]
pub struct VkTensorViewCaptureDescriptorDataInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub tensorView:  VkTensorViewARM,
}

#[repr(C)]
pub struct VkDescriptorGetTensorInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub tensorView:  VkTensorViewARM,
}

#[repr(C)]
pub struct VkFrameBoundaryTensorsARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub tensorCount:  u32,
    pub pTensors: *const  VkTensorARM,
}

#[repr(C)]
pub struct VkPhysicalDeviceExternalTensorInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkTensorCreateFlagsARM,
    pub pDescription: *const  VkTensorDescriptionARM,
    pub handleType:  VkExternalMemoryHandleTypeFlagBits,
}

#[repr(C)]
pub struct VkExternalTensorPropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub externalMemoryProperties:  VkExternalMemoryProperties,
}

#[repr(C)]
pub struct VkExternalMemoryTensorCreateInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handleTypes:  VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderFloat8FeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderFloat8:  VkBool32,
    pub shaderFloat8CooperativeMatrix:  VkBool32,
}

#[repr(C)]
pub struct VkSurfaceCreateInfoOHOS {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkSurfaceCreateFlagsOHOS,
    pub window: *mut  OHNativeWindow,
}

#[repr(C)]
pub struct VkPhysicalDeviceDataGraphFeaturesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub dataGraph:  VkBool32,
    pub dataGraphUpdateAfterBind:  VkBool32,
    pub dataGraphSpecializationConstants:  VkBool32,
    pub dataGraphDescriptorBuffer:  VkBool32,
    pub dataGraphShaderModule:  VkBool32,
}

#[repr(C)]
pub struct VkDataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub dimension:  u32,
    pub zeroCount:  u32,
    pub groupSize:  u32,
}

#[repr(C)]
pub struct VkDataGraphPipelineConstantARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub id:  u32,
    pub pConstantData: *const  std::ffi::c_void,
}

#[repr(C)]
pub struct VkDataGraphPipelineResourceInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub descriptorSet:  u32,
    pub binding:  u32,
    pub arrayElement:  u32,
}

#[repr(C)]
pub struct VkDataGraphPipelineResourceInfoImageLayoutARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub layout:  VkImageLayout,
}

#[repr(C)]
pub struct VkDataGraphPipelineCompilerControlCreateInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pVendorOptions: *const  i8,
}

#[repr(C)]
pub struct VkDataGraphPipelineCreateInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkPipelineCreateFlags2,
    pub layout:  VkPipelineLayout,
    pub resourceInfoCount:  u32,
    pub pResourceInfos: *const  VkDataGraphPipelineResourceInfoARM,
}

#[repr(C)]
pub struct VkDataGraphPipelineShaderModuleCreateInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub module:  VkShaderModule,
    pub pName: *const  i8,
    pub pSpecializationInfo: *const  VkSpecializationInfo,
    pub constantCount:  u32,
    pub pConstants: *const  VkDataGraphPipelineConstantARM,
}

#[repr(C)]
pub struct VkDataGraphPipelineSessionCreateInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub flags:  VkDataGraphPipelineSessionCreateFlagsARM,
    pub dataGraphPipeline:  VkPipeline,
}

#[repr(C)]
pub struct VkDataGraphPipelineSessionBindPointRequirementsInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub session:  VkDataGraphPipelineSessionARM,
}

#[repr(C)]
pub struct VkDataGraphPipelineSessionBindPointRequirementARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub bindPoint:  VkDataGraphPipelineSessionBindPointARM,
    pub bindPointType:  VkDataGraphPipelineSessionBindPointTypeARM,
    pub numObjects:  u32,
}

#[repr(C)]
pub struct VkDataGraphPipelineSessionMemoryRequirementsInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub session:  VkDataGraphPipelineSessionARM,
    pub bindPoint:  VkDataGraphPipelineSessionBindPointARM,
    pub objectIndex:  u32,
}

#[repr(C)]
pub struct VkBindDataGraphPipelineSessionMemoryInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub session:  VkDataGraphPipelineSessionARM,
    pub bindPoint:  VkDataGraphPipelineSessionBindPointARM,
    pub objectIndex:  u32,
    pub memory:  VkDeviceMemory,
    pub memoryOffset:  VkDeviceSize,
}

#[repr(C)]
pub struct VkDataGraphPipelineInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub dataGraphPipeline:  VkPipeline,
}

#[repr(C)]
pub struct VkDataGraphPipelinePropertyQueryResultARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub property:  VkDataGraphPipelinePropertyARM,
    pub isText:  VkBool32,
    pub dataSize:  usize,
    pub pData: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkDataGraphPipelineIdentifierCreateInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub identifierSize:  u32,
    pub pIdentifier: *const  u8,
}

#[repr(C)]
pub struct VkDataGraphPipelineDispatchInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkDataGraphPipelineDispatchFlagsARM,
}

#[repr(C)]
pub struct VkPhysicalDeviceDataGraphProcessingEngineARM {
    pub r#type:  VkPhysicalDeviceDataGraphProcessingEngineTypeARM,
    pub isForeign:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDataGraphOperationSupportARM {
    pub operationType:  VkPhysicalDeviceDataGraphOperationTypeARM,
    pub name:  [i8; VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM as usize],
    pub version:  u32,
}

#[repr(C)]
pub struct VkQueueFamilyDataGraphPropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub engine:  VkPhysicalDeviceDataGraphProcessingEngineARM,
    pub operation:  VkPhysicalDeviceDataGraphOperationSupportARM,
}

#[repr(C)]
pub struct VkPhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub queueFamilyIndex:  u32,
    pub engineType:  VkPhysicalDeviceDataGraphProcessingEngineTypeARM,
}

#[repr(C)]
pub struct VkQueueFamilyDataGraphProcessingEnginePropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub foreignSemaphoreHandleTypes:  VkExternalSemaphoreHandleTypeFlags,
    pub foreignMemoryHandleTypes:  VkExternalMemoryHandleTypeFlags,
}

#[repr(C)]
pub struct VkDataGraphProcessingEngineCreateInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub processingEngineCount:  u32,
    pub pProcessingEngines: *mut  VkPhysicalDeviceDataGraphProcessingEngineARM,
}

#[repr(C)]
pub struct VkPhysicalDevicePipelineCacheIncrementalModeFeaturesSEC {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub pipelineCacheIncrementalMode:  VkBool32,
}

#[repr(C)]
pub struct VkDataGraphPipelineBuiltinModelCreateInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pOperation: *const  VkPhysicalDeviceDataGraphOperationSupportARM,
}

#[repr(C)]
pub struct VkPhysicalDeviceDataGraphModelFeaturesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub dataGraphModel:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderUntypedPointersFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderUntypedPointers:  VkBool32,
}

#[repr(C)]
pub struct VkNativeBufferOHOS {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub handle: *mut  OHBufferHandle,
}

#[repr(C)]
pub struct VkSwapchainImageCreateInfoOHOS {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub usage:  VkSwapchainImageUsageFlagsOHOS,
}

#[repr(C)]
pub struct VkPhysicalDevicePresentationPropertiesOHOS {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub sharedImage:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub videoEncodeRgbConversion:  VkBool32,
}

#[repr(C)]
pub struct VkVideoEncodeRgbConversionCapabilitiesVALVE {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub rgbModels:  VkVideoEncodeRgbModelConversionFlagsVALVE,
    pub rgbRanges:  VkVideoEncodeRgbRangeCompressionFlagsVALVE,
    pub xChromaOffsets:  VkVideoEncodeRgbChromaOffsetFlagsVALVE,
    pub yChromaOffsets:  VkVideoEncodeRgbChromaOffsetFlagsVALVE,
}

#[repr(C)]
pub struct VkVideoEncodeProfileRgbConversionInfoVALVE {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub performEncodeRgbConversion:  VkBool32,
}

#[repr(C)]
pub struct VkVideoEncodeSessionRgbConversionCreateInfoVALVE {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub rgbModel:  VkVideoEncodeRgbModelConversionFlagBitsVALVE,
    pub rgbRange:  VkVideoEncodeRgbRangeCompressionFlagBitsVALVE,
    pub xChromaOffset:  VkVideoEncodeRgbChromaOffsetFlagBitsVALVE,
    pub yChromaOffset:  VkVideoEncodeRgbChromaOffsetFlagBitsVALVE,
}

#[repr(C)]
pub struct VkPhysicalDeviceShader64BitIndexingFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shader64BitIndexing:  VkBool32,
}

#[repr(C)]
pub struct VkNativeBufferUsageOHOS {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub OHOSNativeBufferUsage:  u64,
}

#[repr(C)]
pub struct VkNativeBufferPropertiesOHOS {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub allocationSize:  VkDeviceSize,
    pub memoryTypeBits:  u32,
}

#[repr(C)]
pub struct VkNativeBufferFormatPropertiesOHOS {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub format:  VkFormat,
    pub externalFormat:  u64,
    pub formatFeatures:  VkFormatFeatureFlags,
    pub samplerYcbcrConversionComponents:  VkComponentMapping,
    pub suggestedYcbcrModel:  VkSamplerYcbcrModelConversion,
    pub suggestedYcbcrRange:  VkSamplerYcbcrRange,
    pub suggestedXChromaOffset:  VkChromaLocation,
    pub suggestedYChromaOffset:  VkChromaLocation,
}

#[repr(C)]
pub struct VkImportNativeBufferInfoOHOS {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub buffer: *mut  OH_NativeBuffer,
}

#[repr(C)]
pub struct VkMemoryGetNativeBufferInfoOHOS {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub memory:  VkDeviceMemory,
}

#[repr(C)]
pub struct VkExternalFormatOHOS {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub externalFormat:  u64,
}

#[repr(C)]
pub struct VkPerfHintInfoQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub r#type:  VkPerfHintTypeQCOM,
    pub scale:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceQueuePerfHintFeaturesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub queuePerfHint:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceQueuePerfHintPropertiesQCOM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub supportedQueues:  VkQueueFlags,
}

#[repr(C)]
pub struct VkPhysicalDevicePerformanceCountersByRegionFeaturesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub performanceCountersByRegion:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDevicePerformanceCountersByRegionPropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxPerRegionPerformanceCounters:  u32,
    pub performanceCounterRegionSize:  VkExtent2D,
    pub rowStrideAlignment:  u32,
    pub regionAlignment:  u32,
    pub identityTransformOrder:  VkBool32,
}

#[repr(C)]
pub struct VkPerformanceCounterARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub counterID:  u32,
}

#[repr(C)]
pub struct VkPerformanceCounterDescriptionARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkPerformanceCounterDescriptionFlagsARM,
    pub name:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
}

#[repr(C)]
pub struct VkRenderPassPerformanceCountersByRegionBeginInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub counterAddressCount:  u32,
    pub pCounterAddresses: *const  VkDeviceAddress,
    pub serializeRegions:  VkBool32,
    pub counterIndexCount:  u32,
    pub pCounterIndices: *mut  u32,
}

#[repr(C)]
pub struct VkComputeOccupancyPriorityParametersNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub occupancyPriority:  f32,
    pub occupancyThrottling:  f32,
}

#[repr(C)]
pub struct VkPhysicalDeviceComputeOccupancyPriorityFeaturesNV {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub computeOccupancyPriority:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderLongVectorFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub longVector:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderLongVectorPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxVectorComponents:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceTextureCompressionASTC3DFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub textureCompressionASTC_3D:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSubgroupPartitionedFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderSubgroupPartitioned:  VkBool32,
}

#[repr(C)]
pub struct VkHostAddressRangeEXT {
    pub address: *mut  std::ffi::c_void,
    pub size:  usize,
}

#[repr(C)]
pub struct VkHostAddressRangeConstEXT {
    pub address: *const  std::ffi::c_void,
    pub size:  usize,
}

#[repr(C)]
pub struct VkTexelBufferDescriptorInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub format:  VkFormat,
    pub addressRange:  VkDeviceAddressRangeEXT,
}

#[repr(C)]
pub struct VkImageDescriptorInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pView: *const  VkImageViewCreateInfo,
    pub layout:  VkImageLayout,
}

#[repr(C)]
pub struct VkResourceDescriptorDataEXT {
    pub pImage: *const  VkImageDescriptorInfoEXT,
    pub pTexelBuffer: *const  VkTexelBufferDescriptorInfoEXT,
    pub pAddressRange: *const  VkDeviceAddressRangeEXT,
    pub pTensorARM: *const  VkTensorViewCreateInfoARM,
}

#[repr(C)]
pub struct VkResourceDescriptorInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub r#type:  VkDescriptorType,
    pub data:  VkResourceDescriptorDataEXT,
}

#[repr(C)]
pub struct VkBindHeapInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub heapRange:  VkDeviceAddressRangeEXT,
    pub reservedRangeOffset:  VkDeviceSize,
    pub reservedRangeSize:  VkDeviceSize,
}

#[repr(C)]
pub struct VkPushDataInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub offset:  u32,
    pub data:  VkHostAddressRangeConstEXT,
}

#[repr(C)]
pub struct VkDescriptorMappingSourceConstantOffsetEXT {
    pub heapOffset:  u32,
    pub heapArrayStride:  u32,
    pub pEmbeddedSampler: *const  VkSamplerCreateInfo,
    pub samplerHeapOffset:  u32,
    pub samplerHeapArrayStride:  u32,
}

#[repr(C)]
pub struct VkDescriptorMappingSourcePushIndexEXT {
    pub heapOffset:  u32,
    pub pushOffset:  u32,
    pub heapIndexStride:  u32,
    pub heapArrayStride:  u32,
    pub pEmbeddedSampler: *const  VkSamplerCreateInfo,
    pub useCombinedImageSamplerIndex:  VkBool32,
    pub samplerHeapOffset:  u32,
    pub samplerPushOffset:  u32,
    pub samplerHeapIndexStride:  u32,
    pub samplerHeapArrayStride:  u32,
}

#[repr(C)]
pub struct VkDescriptorMappingSourceIndirectIndexEXT {
    pub heapOffset:  u32,
    pub pushOffset:  u32,
    pub addressOffset:  u32,
    pub heapIndexStride:  u32,
    pub heapArrayStride:  u32,
    pub pEmbeddedSampler: *const  VkSamplerCreateInfo,
    pub useCombinedImageSamplerIndex:  VkBool32,
    pub samplerHeapOffset:  u32,
    pub samplerPushOffset:  u32,
    pub samplerAddressOffset:  u32,
    pub samplerHeapIndexStride:  u32,
    pub samplerHeapArrayStride:  u32,
}

#[repr(C)]
pub struct VkDescriptorMappingSourceIndirectIndexArrayEXT {
    pub heapOffset:  u32,
    pub pushOffset:  u32,
    pub addressOffset:  u32,
    pub heapIndexStride:  u32,
    pub pEmbeddedSampler: *const  VkSamplerCreateInfo,
    pub useCombinedImageSamplerIndex:  VkBool32,
    pub samplerHeapOffset:  u32,
    pub samplerPushOffset:  u32,
    pub samplerAddressOffset:  u32,
    pub samplerHeapIndexStride:  u32,
}

#[repr(C)]
pub struct VkDescriptorMappingSourceHeapDataEXT {
    pub heapOffset:  u32,
    pub pushOffset:  u32,
}

#[repr(C)]
pub struct VkDescriptorMappingSourceShaderRecordIndexEXT {
    pub heapOffset:  u32,
    pub shaderRecordOffset:  u32,
    pub heapIndexStride:  u32,
    pub heapArrayStride:  u32,
    pub pEmbeddedSampler: *const  VkSamplerCreateInfo,
    pub useCombinedImageSamplerIndex:  VkBool32,
    pub samplerHeapOffset:  u32,
    pub samplerShaderRecordOffset:  u32,
    pub samplerHeapIndexStride:  u32,
    pub samplerHeapArrayStride:  u32,
}

#[repr(C)]
pub struct VkDescriptorMappingSourceIndirectAddressEXT {
    pub pushOffset:  u32,
    pub addressOffset:  u32,
}

#[repr(C)]
pub struct VkDescriptorMappingSourceDataEXT {
    pub constantOffset:  VkDescriptorMappingSourceConstantOffsetEXT,
    pub pushIndex:  VkDescriptorMappingSourcePushIndexEXT,
    pub indirectIndex:  VkDescriptorMappingSourceIndirectIndexEXT,
    pub indirectIndexArray:  VkDescriptorMappingSourceIndirectIndexArrayEXT,
    pub heapData:  VkDescriptorMappingSourceHeapDataEXT,
    pub pushDataOffset:  u32,
    pub pushAddressOffset:  u32,
    pub indirectAddress:  VkDescriptorMappingSourceIndirectAddressEXT,
    pub shaderRecordIndex:  VkDescriptorMappingSourceShaderRecordIndexEXT,
    pub shaderRecordDataOffset:  u32,
    pub shaderRecordAddressOffset:  u32,
}

#[repr(C)]
pub struct VkDescriptorSetAndBindingMappingEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub descriptorSet:  u32,
    pub firstBinding:  u32,
    pub bindingCount:  u32,
    pub resourceMask:  VkSpirvResourceTypeFlagsEXT,
    pub source:  VkDescriptorMappingSourceEXT,
    pub sourceData:  VkDescriptorMappingSourceDataEXT,
}

#[repr(C)]
pub struct VkShaderDescriptorSetAndBindingMappingInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub mappingCount:  u32,
    pub pMappings: *const  VkDescriptorSetAndBindingMappingEXT,
}

#[repr(C)]
pub struct VkSamplerCustomBorderColorIndexCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub index:  u32,
}

#[repr(C)]
pub struct VkOpaqueCaptureDataCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pData: *const  VkHostAddressRangeConstEXT,
}

#[repr(C)]
pub struct VkIndirectCommandsLayoutPushDataTokenNV {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pushDataOffset:  u32,
    pub pushDataSize:  u32,
}

#[repr(C)]
pub struct VkSubsampledImageFormatPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub subsampledImageDescriptorCount:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSplitBarrierFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderSplitBarrier:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderSplitBarrierPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub splitBarrierReservedSharedMemory:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorHeapFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub descriptorHeap:  VkBool32,
    pub descriptorHeapCaptureReplay:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorHeapPropertiesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub samplerHeapAlignment:  VkDeviceSize,
    pub resourceHeapAlignment:  VkDeviceSize,
    pub maxSamplerHeapSize:  VkDeviceSize,
    pub maxResourceHeapSize:  VkDeviceSize,
    pub minSamplerHeapReservedRange:  VkDeviceSize,
    pub minSamplerHeapReservedRangeWithEmbedded:  VkDeviceSize,
    pub minResourceHeapReservedRange:  VkDeviceSize,
    pub samplerDescriptorSize:  VkDeviceSize,
    pub imageDescriptorSize:  VkDeviceSize,
    pub bufferDescriptorSize:  VkDeviceSize,
    pub samplerDescriptorAlignment:  VkDeviceSize,
    pub imageDescriptorAlignment:  VkDeviceSize,
    pub bufferDescriptorAlignment:  VkDeviceSize,
    pub maxPushDataSize:  VkDeviceSize,
    pub imageCaptureReplayOpaqueDataSize:  usize,
    pub maxDescriptorHeapEmbeddedSamplers:  u32,
    pub samplerYcbcrConversionCount:  u32,
    pub sparseDescriptorHeaps:  VkBool32,
    pub protectedDescriptorHeaps:  VkBool32,
}

#[repr(C)]
pub struct VkCommandBufferInheritanceDescriptorHeapInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub pSamplerHeapBindInfo: *const  VkBindHeapInfoEXT,
    pub pResourceHeapBindInfo: *const  VkBindHeapInfoEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceDescriptorHeapTensorPropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub tensorDescriptorSize:  VkDeviceSize,
    pub tensorDescriptorAlignment:  VkDeviceSize,
    pub tensorCaptureReplayOpaqueDataSize:  usize,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderInstrumentationFeaturesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderInstrumentation:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderInstrumentationPropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub numMetrics:  u32,
    pub perBasicBlockGranularity:  VkBool32,
}

#[repr(C)]
pub struct VkShaderInstrumentationCreateInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkShaderInstrumentationMetricDescriptionARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub name:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
    pub description:  [i8; VK_MAX_DESCRIPTION_SIZE as usize],
}

#[repr(C)]
pub struct VkShaderInstrumentationMetricDataHeaderARM {
    pub resultIndex:  u32,
    pub resultSubIndex:  u32,
    pub stages:  VkShaderStageFlags,
    pub basicBlockIndex:  u32,
}

#[repr(C)]
pub struct VkDeviceAddressRangeKHR {
    pub address:  VkDeviceAddress,
    pub size:  VkDeviceSize,
}

#[repr(C)]
pub struct VkDeviceAddressRangeEXT {
}

#[repr(C)]
pub struct VkDeviceMemoryCopyKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcRange:  VkDeviceAddressRangeKHR,
    pub srcFlags:  VkAddressCommandFlagsKHR,
    pub dstRange:  VkDeviceAddressRangeKHR,
    pub dstFlags:  VkAddressCommandFlagsKHR,
}

#[repr(C)]
pub struct VkCopyDeviceMemoryInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub regionCount:  u32,
    pub pRegions: *const  VkDeviceMemoryCopyKHR,
}

#[repr(C)]
pub struct VkDeviceMemoryImageCopyKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub addressRange:  VkDeviceAddressRangeKHR,
    pub addressFlags:  VkAddressCommandFlagsKHR,
    pub addressRowLength:  u32,
    pub addressImageHeight:  u32,
    pub imageSubresource:  VkImageSubresourceLayers,
    pub imageLayout:  VkImageLayout,
    pub imageOffset:  VkOffset3D,
    pub imageExtent:  VkExtent3D,
}

#[repr(C)]
pub struct VkCopyDeviceMemoryImageInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub image:  VkImage,
    pub regionCount:  u32,
    pub pRegions: *const  VkDeviceMemoryImageCopyKHR,
}

#[repr(C)]
pub struct VkMemoryRangeBarriersInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub memoryRangeBarrierCount:  u32,
    pub pMemoryRangeBarriers: *const  VkMemoryRangeBarrierKHR,
}

#[repr(C)]
pub struct VkMemoryRangeBarrierKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub srcStageMask:  VkPipelineStageFlags2,
    pub srcAccessMask:  VkAccessFlags2,
    pub dstStageMask:  VkPipelineStageFlags2,
    pub dstAccessMask:  VkAccessFlags2,
    pub srcQueueFamilyIndex:  u32,
    pub dstQueueFamilyIndex:  u32,
    pub addressRange:  VkDeviceAddressRangeKHR,
    pub addressFlags:  VkAddressCommandFlagsKHR,
}

#[repr(C)]
pub struct VkPhysicalDeviceDeviceAddressCommandsFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub deviceAddressCommands:  VkBool32,
}

#[repr(C)]
pub struct VkConditionalRenderingBeginInfo2EXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub addressRange:  VkDeviceAddressRangeKHR,
    pub addressFlags:  VkAddressCommandFlagsKHR,
    pub flags:  VkConditionalRenderingFlagsEXT,
}

#[repr(C)]
pub struct VkAccelerationStructureCreateInfo2KHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub createFlags:  VkAccelerationStructureCreateFlagsKHR,
    pub addressRange:  VkDeviceAddressRangeKHR,
    pub addressFlags:  VkAddressCommandFlagsKHR,
    pub r#type:  VkAccelerationStructureTypeKHR,
}

#[repr(C)]
pub struct VkBindIndexBuffer3InfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub addressRange:  VkDeviceAddressRangeKHR,
    pub addressFlags:  VkAddressCommandFlagsKHR,
    pub indexType:  VkIndexType,
}

#[repr(C)]
pub struct VkBindVertexBuffer3InfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub setStride:  VkBool32,
    pub addressRange:  VkStridedDeviceAddressRangeKHR,
    pub addressFlags:  VkAddressCommandFlagsKHR,
}

#[repr(C)]
pub struct VkDrawIndirect2InfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub addressRange:  VkStridedDeviceAddressRangeKHR,
    pub addressFlags:  VkAddressCommandFlagsKHR,
    pub drawCount:  u32,
}

#[repr(C)]
pub struct VkDrawIndirectCount2InfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub addressRange:  VkStridedDeviceAddressRangeKHR,
    pub addressFlags:  VkAddressCommandFlagsKHR,
    pub countAddressRange:  VkDeviceAddressRangeKHR,
    pub countAddressFlags:  VkAddressCommandFlagsKHR,
    pub maxDrawCount:  u32,
}

#[repr(C)]
pub struct VkDispatchIndirect2InfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub addressRange:  VkDeviceAddressRangeKHR,
    pub addressFlags:  VkAddressCommandFlagsKHR,
}

#[repr(C)]
pub struct VkBindTransformFeedbackBuffer2InfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub addressRange:  VkDeviceAddressRangeKHR,
    pub addressFlags:  VkAddressCommandFlagsKHR,
}

#[repr(C)]
pub struct VkMemoryMarkerInfoAMD {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub stage:  VkPipelineStageFlags2KHR,
    pub dstRange:  VkDeviceAddressRangeKHR,
    pub dstFlags:  VkAddressCommandFlagsKHR,
    pub marker:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderConstantDataFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderConstantData:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderAbortFeaturesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderAbort:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderAbortPropertiesKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub maxShaderAbortMessageSize:  u64,
}

#[repr(C)]
pub struct VkDeviceFaultShaderAbortMessageInfoKHR {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub messageDataSize:  u64,
    pub pMessageData: *mut  std::ffi::c_void,
}

#[repr(C)]
pub struct VkDataGraphTOSANameQualityARM {
    pub name:  [i8; VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM as usize],
    pub qualityFlags:  VkDataGraphTOSAQualityFlagsARM,
}

#[repr(C)]
pub struct VkQueueFamilyDataGraphTOSAPropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub profileCount:  u32,
    pub pProfiles: *const  VkDataGraphTOSANameQualityARM,
    pub extensionCount:  u32,
    pub pExtensions: *const  VkDataGraphTOSANameQualityARM,
    pub level:  VkDataGraphTOSALevelARM,
}

#[repr(C)]
pub struct VkDataGraphPipelineSingleNodeConnectionARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub set:  u32,
    pub binding:  u32,
    pub connection:  VkDataGraphPipelineNodeConnectionTypeARM,
}

#[repr(C)]
pub struct VkPhysicalDeviceDataGraphOpticalFlowFeaturesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub dataGraphOpticalFlow:  VkBool32,
}

#[repr(C)]
pub struct VkQueueFamilyDataGraphOpticalFlowPropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub supportedOutputGridSizes:  VkDataGraphOpticalFlowGridSizeFlagsARM,
    pub supportedHintGridSizes:  VkDataGraphOpticalFlowGridSizeFlagsARM,
    pub hintSupported:  VkBool32,
    pub costSupported:  VkBool32,
    pub minWidth:  u32,
    pub minHeight:  u32,
    pub maxWidth:  u32,
    pub maxHeight:  u32,
}

#[repr(C)]
pub struct VkDataGraphOpticalFlowImageFormatInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub usage:  VkDataGraphOpticalFlowImageUsageFlagsARM,
}

#[repr(C)]
pub struct VkDataGraphOpticalFlowImageFormatPropertiesARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub format:  VkFormat,
}

#[repr(C)]
pub struct VkDataGraphPipelineSingleNodeCreateInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub nodeType:  VkDataGraphPipelineNodeTypeARM,
    pub connectionCount:  u32,
    pub pConnections: *const  VkDataGraphPipelineSingleNodeConnectionARM,
}

#[repr(C)]
pub struct VkDataGraphPipelineOpticalFlowCreateInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub width:  u32,
    pub height:  u32,
    pub imageFormat:  VkFormat,
    pub flowVectorFormat:  VkFormat,
    pub costFormat:  VkFormat,
    pub outputGridSize:  VkDataGraphOpticalFlowGridSizeFlagsARM,
    pub hintGridSize:  VkDataGraphOpticalFlowGridSizeFlagsARM,
    pub performanceLevel:  VkDataGraphOpticalFlowPerformanceLevelARM,
    pub flags:  VkDataGraphOpticalFlowCreateFlagsARM,
}

#[repr(C)]
pub struct VkDataGraphPipelineOpticalFlowDispatchInfoARM {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub flags:  VkDataGraphOpticalFlowExecuteFlagsARM,
    pub meanFlowL1NormHint:  u32,
}

#[repr(C)]
pub struct VkPhysicalDeviceImageTilingControlFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub imageTilingControl:  VkBool32,
}

#[repr(C)]
pub struct VkImageTilingControlCreateInfoEXT {
    pub sType:  VkStructureType,
    pub pNext: *const  std::ffi::c_void,
    pub tilingControl:  VkImageTilingControlEXT,
}

#[repr(C)]
pub struct VkPhysicalDeviceShaderOCPMicroscalingTypesFeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub shaderFloat4:  VkBool32,
    pub shaderFloat6:  VkBool32,
    pub shaderFloat8UnsignedE8M0:  VkBool32,
    pub shaderMXInt8:  VkBool32,
}

#[repr(C)]
pub struct VkPhysicalDeviceCooperativeMatrixMaintenance1FeaturesEXT {
    pub sType:  VkStructureType,
    pub pNext: *mut  std::ffi::c_void,
    pub cooperativeMatrixProperties2:  VkBool32,
    pub cooperativeMatrixReductions:  VkBool32,
    pub cooperativeMatrixConversions:  VkBool32,
    pub cooperativeMatrixPerElementOperations:  VkBool32,
    pub cooperativeMatrixGetCoordinate:  VkBool32,
}

pub struct VkImageLayout(i32); //
impl VkImageLayout {
    pub const VK_IMAGE_LAYOUT_UNDEFINED: Self = Self(0);
    pub const VK_IMAGE_LAYOUT_GENERAL: Self = Self(1);
    pub const VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL: Self = Self(2);
    pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL: Self = Self(3);
    pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL: Self = Self(4);
    pub const VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL: Self = Self(5);
    pub const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL: Self = Self(6);
    pub const VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL: Self = Self(7);
    pub const VK_IMAGE_LAYOUT_PREINITIALIZED: Self = Self(8);
}

pub struct VkAttachmentLoadOp(i32); //
impl VkAttachmentLoadOp {
    pub const VK_ATTACHMENT_LOAD_OP_LOAD: Self = Self(0);
    pub const VK_ATTACHMENT_LOAD_OP_CLEAR: Self = Self(1);
    pub const VK_ATTACHMENT_LOAD_OP_DONT_CARE: Self = Self(2);
}

pub struct VkAttachmentStoreOp(i32); //
impl VkAttachmentStoreOp {
    pub const VK_ATTACHMENT_STORE_OP_STORE: Self = Self(0);
    pub const VK_ATTACHMENT_STORE_OP_DONT_CARE: Self = Self(1);
}

pub struct VkImageType(i32); //
impl VkImageType {
    pub const VK_IMAGE_TYPE_1D: Self = Self(0);
    pub const VK_IMAGE_TYPE_2D: Self = Self(1);
    pub const VK_IMAGE_TYPE_3D: Self = Self(2);
}

pub struct VkImageTiling(i32); //
impl VkImageTiling {
    pub const VK_IMAGE_TILING_OPTIMAL: Self = Self(0);
    pub const VK_IMAGE_TILING_LINEAR: Self = Self(1);
}

pub struct VkImageViewType(i32); //
impl VkImageViewType {
    pub const VK_IMAGE_VIEW_TYPE_1D: Self = Self(0);
    pub const VK_IMAGE_VIEW_TYPE_2D: Self = Self(1);
    pub const VK_IMAGE_VIEW_TYPE_3D: Self = Self(2);
    pub const VK_IMAGE_VIEW_TYPE_CUBE: Self = Self(3);
    pub const VK_IMAGE_VIEW_TYPE_1D_ARRAY: Self = Self(4);
    pub const VK_IMAGE_VIEW_TYPE_2D_ARRAY: Self = Self(5);
    pub const VK_IMAGE_VIEW_TYPE_CUBE_ARRAY: Self = Self(6);
}

pub struct VkCommandBufferLevel(i32); //
impl VkCommandBufferLevel {
    pub const VK_COMMAND_BUFFER_LEVEL_PRIMARY: Self = Self(0);
    pub const VK_COMMAND_BUFFER_LEVEL_SECONDARY: Self = Self(1);
}

pub struct VkComponentSwizzle(i32); //
impl VkComponentSwizzle {
    pub const VK_COMPONENT_SWIZZLE_IDENTITY: Self = Self(0);
    pub const VK_COMPONENT_SWIZZLE_ZERO: Self = Self(1);
    pub const VK_COMPONENT_SWIZZLE_ONE: Self = Self(2);
    pub const VK_COMPONENT_SWIZZLE_R: Self = Self(3);
    pub const VK_COMPONENT_SWIZZLE_G: Self = Self(4);
    pub const VK_COMPONENT_SWIZZLE_B: Self = Self(5);
    pub const VK_COMPONENT_SWIZZLE_A: Self = Self(6);
}

pub struct VkDescriptorType(i32); //
impl VkDescriptorType {
    pub const VK_DESCRIPTOR_TYPE_SAMPLER: Self = Self(0);
    pub const VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: Self = Self(1);
    pub const VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE: Self = Self(2);
    pub const VK_DESCRIPTOR_TYPE_STORAGE_IMAGE: Self = Self(3);
    pub const VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER: Self = Self(4);
    pub const VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER: Self = Self(5);
    pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER: Self = Self(6);
    pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER: Self = Self(7);
    pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC: Self = Self(8);
    pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC: Self = Self(9);
    pub const VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT: Self = Self(10);
}

pub struct VkQueryType(i32); //
impl VkQueryType {
    pub const VK_QUERY_TYPE_OCCLUSION: Self = Self(0);
    pub const VK_QUERY_TYPE_PIPELINE_STATISTICS: Self = Self(1);
    pub const VK_QUERY_TYPE_TIMESTAMP: Self = Self(2);
}

pub struct VkBorderColor(i32); //
impl VkBorderColor {
    pub const VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK: Self = Self(0);
    pub const VK_BORDER_COLOR_INT_TRANSPARENT_BLACK: Self = Self(1);
    pub const VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK: Self = Self(2);
    pub const VK_BORDER_COLOR_INT_OPAQUE_BLACK: Self = Self(3);
    pub const VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE: Self = Self(4);
    pub const VK_BORDER_COLOR_INT_OPAQUE_WHITE: Self = Self(5);
}

pub struct VkPipelineBindPoint(i32); //
impl VkPipelineBindPoint {
    pub const VK_PIPELINE_BIND_POINT_GRAPHICS: Self = Self(0);
    pub const VK_PIPELINE_BIND_POINT_COMPUTE: Self = Self(1);
}

pub struct VkPipelineCacheHeaderVersion(i32); //
impl VkPipelineCacheHeaderVersion {
    pub const VK_PIPELINE_CACHE_HEADER_VERSION_ONE: Self = Self(1);
}

pub type VkPipelineCacheCreateFlagBits = VkPipelineCacheCreateFlags; //
impl VkPipelineCacheCreateFlagBits {
}

pub struct VkPrimitiveTopology(i32); //
impl VkPrimitiveTopology {
    pub const VK_PRIMITIVE_TOPOLOGY_POINT_LIST: Self = Self(0);
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST: Self = Self(1);
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP: Self = Self(2);
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: Self = Self(3);
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP: Self = Self(4);
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN: Self = Self(5);
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY: Self = Self(6);
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY: Self = Self(7);
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY: Self = Self(8);
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY: Self = Self(9);
    pub const VK_PRIMITIVE_TOPOLOGY_PATCH_LIST: Self = Self(10);
}

pub struct VkSharingMode(i32); //
impl VkSharingMode {
    pub const VK_SHARING_MODE_EXCLUSIVE: Self = Self(0);
    pub const VK_SHARING_MODE_CONCURRENT: Self = Self(1);
}

pub struct VkIndexType(i32); //
impl VkIndexType {
    pub const VK_INDEX_TYPE_UINT16: Self = Self(0);
    pub const VK_INDEX_TYPE_UINT32: Self = Self(1);
}

pub struct VkFilter(i32); //
impl VkFilter {
    pub const VK_FILTER_NEAREST: Self = Self(0);
    pub const VK_FILTER_LINEAR: Self = Self(1);
}

pub struct VkSamplerMipmapMode(i32); //
impl VkSamplerMipmapMode {
    pub const VK_SAMPLER_MIPMAP_MODE_NEAREST: Self = Self(0);
    pub const VK_SAMPLER_MIPMAP_MODE_LINEAR: Self = Self(1);
}

pub struct VkSamplerAddressMode(i32); //
impl VkSamplerAddressMode {
    pub const VK_SAMPLER_ADDRESS_MODE_REPEAT: Self = Self(0);
    pub const VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT: Self = Self(1);
    pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE: Self = Self(2);
    pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER: Self = Self(3);
}

pub struct VkCompareOp(i32); //
impl VkCompareOp {
    pub const VK_COMPARE_OP_NEVER: Self = Self(0);
    pub const VK_COMPARE_OP_LESS: Self = Self(1);
    pub const VK_COMPARE_OP_EQUAL: Self = Self(2);
    pub const VK_COMPARE_OP_LESS_OR_EQUAL: Self = Self(3);
    pub const VK_COMPARE_OP_GREATER: Self = Self(4);
    pub const VK_COMPARE_OP_NOT_EQUAL: Self = Self(5);
    pub const VK_COMPARE_OP_GREATER_OR_EQUAL: Self = Self(6);
    pub const VK_COMPARE_OP_ALWAYS: Self = Self(7);
}

pub struct VkPolygonMode(i32); //
impl VkPolygonMode {
    pub const VK_POLYGON_MODE_FILL: Self = Self(0);
    pub const VK_POLYGON_MODE_LINE: Self = Self(1);
    pub const VK_POLYGON_MODE_POINT: Self = Self(2);
}

pub struct VkFrontFace(i32); //
impl VkFrontFace {
    pub const VK_FRONT_FACE_COUNTER_CLOCKWISE: Self = Self(0);
    pub const VK_FRONT_FACE_CLOCKWISE: Self = Self(1);
}

pub struct VkBlendFactor(i32); //
impl VkBlendFactor {
    pub const VK_BLEND_FACTOR_ZERO: Self = Self(0);
    pub const VK_BLEND_FACTOR_ONE: Self = Self(1);
    pub const VK_BLEND_FACTOR_SRC_COLOR: Self = Self(2);
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR: Self = Self(3);
    pub const VK_BLEND_FACTOR_DST_COLOR: Self = Self(4);
    pub const VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR: Self = Self(5);
    pub const VK_BLEND_FACTOR_SRC_ALPHA: Self = Self(6);
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: Self = Self(7);
    pub const VK_BLEND_FACTOR_DST_ALPHA: Self = Self(8);
    pub const VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA: Self = Self(9);
    pub const VK_BLEND_FACTOR_CONSTANT_COLOR: Self = Self(10);
    pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: Self = Self(11);
    pub const VK_BLEND_FACTOR_CONSTANT_ALPHA: Self = Self(12);
    pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: Self = Self(13);
    pub const VK_BLEND_FACTOR_SRC_ALPHA_SATURATE: Self = Self(14);
    pub const VK_BLEND_FACTOR_SRC1_COLOR: Self = Self(15);
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR: Self = Self(16);
    pub const VK_BLEND_FACTOR_SRC1_ALPHA: Self = Self(17);
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA: Self = Self(18);
}

pub struct VkBlendOp(i32); //
impl VkBlendOp {
    pub const VK_BLEND_OP_ADD: Self = Self(0);
    pub const VK_BLEND_OP_SUBTRACT: Self = Self(1);
    pub const VK_BLEND_OP_REVERSE_SUBTRACT: Self = Self(2);
    pub const VK_BLEND_OP_MIN: Self = Self(3);
    pub const VK_BLEND_OP_MAX: Self = Self(4);
}

pub struct VkStencilOp(i32); //
impl VkStencilOp {
    pub const VK_STENCIL_OP_KEEP: Self = Self(0);
    pub const VK_STENCIL_OP_ZERO: Self = Self(1);
    pub const VK_STENCIL_OP_REPLACE: Self = Self(2);
    pub const VK_STENCIL_OP_INCREMENT_AND_CLAMP: Self = Self(3);
    pub const VK_STENCIL_OP_DECREMENT_AND_CLAMP: Self = Self(4);
    pub const VK_STENCIL_OP_INVERT: Self = Self(5);
    pub const VK_STENCIL_OP_INCREMENT_AND_WRAP: Self = Self(6);
    pub const VK_STENCIL_OP_DECREMENT_AND_WRAP: Self = Self(7);
}

pub struct VkLogicOp(i32); //
impl VkLogicOp {
    pub const VK_LOGIC_OP_CLEAR: Self = Self(0);
    pub const VK_LOGIC_OP_AND: Self = Self(1);
    pub const VK_LOGIC_OP_AND_REVERSE: Self = Self(2);
    pub const VK_LOGIC_OP_COPY: Self = Self(3);
    pub const VK_LOGIC_OP_AND_INVERTED: Self = Self(4);
    pub const VK_LOGIC_OP_NO_OP: Self = Self(5);
    pub const VK_LOGIC_OP_XOR: Self = Self(6);
    pub const VK_LOGIC_OP_OR: Self = Self(7);
    pub const VK_LOGIC_OP_NOR: Self = Self(8);
    pub const VK_LOGIC_OP_EQUIVALENT: Self = Self(9);
    pub const VK_LOGIC_OP_INVERT: Self = Self(10);
    pub const VK_LOGIC_OP_OR_REVERSE: Self = Self(11);
    pub const VK_LOGIC_OP_COPY_INVERTED: Self = Self(12);
    pub const VK_LOGIC_OP_OR_INVERTED: Self = Self(13);
    pub const VK_LOGIC_OP_NAND: Self = Self(14);
    pub const VK_LOGIC_OP_SET: Self = Self(15);
}

pub struct VkInternalAllocationType(i32); //
impl VkInternalAllocationType {
    pub const VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE: Self = Self(0);
}

pub struct VkSystemAllocationScope(i32); //
impl VkSystemAllocationScope {
    pub const VK_SYSTEM_ALLOCATION_SCOPE_COMMAND: Self = Self(0);
    pub const VK_SYSTEM_ALLOCATION_SCOPE_OBJECT: Self = Self(1);
    pub const VK_SYSTEM_ALLOCATION_SCOPE_CACHE: Self = Self(2);
    pub const VK_SYSTEM_ALLOCATION_SCOPE_DEVICE: Self = Self(3);
    pub const VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE: Self = Self(4);
}

pub struct VkPhysicalDeviceType(i32); //
impl VkPhysicalDeviceType {
    pub const VK_PHYSICAL_DEVICE_TYPE_OTHER: Self = Self(0);
    pub const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: Self = Self(1);
    pub const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: Self = Self(2);
    pub const VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU: Self = Self(3);
    pub const VK_PHYSICAL_DEVICE_TYPE_CPU: Self = Self(4);
}

pub struct VkVertexInputRate(i32); //
impl VkVertexInputRate {
    pub const VK_VERTEX_INPUT_RATE_VERTEX: Self = Self(0);
    pub const VK_VERTEX_INPUT_RATE_INSTANCE: Self = Self(1);
}

pub struct VkFormat(i32); //Vulkan format definitions
impl VkFormat {
    pub const VK_FORMAT_UNDEFINED: Self = Self(0);
    pub const VK_FORMAT_R4G4_UNORM_PACK8: Self = Self(1);
    pub const VK_FORMAT_R4G4B4A4_UNORM_PACK16: Self = Self(2);
    pub const VK_FORMAT_B4G4R4A4_UNORM_PACK16: Self = Self(3);
    pub const VK_FORMAT_R5G6B5_UNORM_PACK16: Self = Self(4);
    pub const VK_FORMAT_B5G6R5_UNORM_PACK16: Self = Self(5);
    pub const VK_FORMAT_R5G5B5A1_UNORM_PACK16: Self = Self(6);
    pub const VK_FORMAT_B5G5R5A1_UNORM_PACK16: Self = Self(7);
    pub const VK_FORMAT_A1R5G5B5_UNORM_PACK16: Self = Self(8);
    pub const VK_FORMAT_R8_UNORM: Self = Self(9);
    pub const VK_FORMAT_R8_SNORM: Self = Self(10);
    pub const VK_FORMAT_R8_USCALED: Self = Self(11);
    pub const VK_FORMAT_R8_SSCALED: Self = Self(12);
    pub const VK_FORMAT_R8_UINT: Self = Self(13);
    pub const VK_FORMAT_R8_SINT: Self = Self(14);
    pub const VK_FORMAT_R8_SRGB: Self = Self(15);
    pub const VK_FORMAT_R8G8_UNORM: Self = Self(16);
    pub const VK_FORMAT_R8G8_SNORM: Self = Self(17);
    pub const VK_FORMAT_R8G8_USCALED: Self = Self(18);
    pub const VK_FORMAT_R8G8_SSCALED: Self = Self(19);
    pub const VK_FORMAT_R8G8_UINT: Self = Self(20);
    pub const VK_FORMAT_R8G8_SINT: Self = Self(21);
    pub const VK_FORMAT_R8G8_SRGB: Self = Self(22);
    pub const VK_FORMAT_R8G8B8_UNORM: Self = Self(23);
    pub const VK_FORMAT_R8G8B8_SNORM: Self = Self(24);
    pub const VK_FORMAT_R8G8B8_USCALED: Self = Self(25);
    pub const VK_FORMAT_R8G8B8_SSCALED: Self = Self(26);
    pub const VK_FORMAT_R8G8B8_UINT: Self = Self(27);
    pub const VK_FORMAT_R8G8B8_SINT: Self = Self(28);
    pub const VK_FORMAT_R8G8B8_SRGB: Self = Self(29);
    pub const VK_FORMAT_B8G8R8_UNORM: Self = Self(30);
    pub const VK_FORMAT_B8G8R8_SNORM: Self = Self(31);
    pub const VK_FORMAT_B8G8R8_USCALED: Self = Self(32);
    pub const VK_FORMAT_B8G8R8_SSCALED: Self = Self(33);
    pub const VK_FORMAT_B8G8R8_UINT: Self = Self(34);
    pub const VK_FORMAT_B8G8R8_SINT: Self = Self(35);
    pub const VK_FORMAT_B8G8R8_SRGB: Self = Self(36);
    pub const VK_FORMAT_R8G8B8A8_UNORM: Self = Self(37);
    pub const VK_FORMAT_R8G8B8A8_SNORM: Self = Self(38);
    pub const VK_FORMAT_R8G8B8A8_USCALED: Self = Self(39);
    pub const VK_FORMAT_R8G8B8A8_SSCALED: Self = Self(40);
    pub const VK_FORMAT_R8G8B8A8_UINT: Self = Self(41);
    pub const VK_FORMAT_R8G8B8A8_SINT: Self = Self(42);
    pub const VK_FORMAT_R8G8B8A8_SRGB: Self = Self(43);
    pub const VK_FORMAT_B8G8R8A8_UNORM: Self = Self(44);
    pub const VK_FORMAT_B8G8R8A8_SNORM: Self = Self(45);
    pub const VK_FORMAT_B8G8R8A8_USCALED: Self = Self(46);
    pub const VK_FORMAT_B8G8R8A8_SSCALED: Self = Self(47);
    pub const VK_FORMAT_B8G8R8A8_UINT: Self = Self(48);
    pub const VK_FORMAT_B8G8R8A8_SINT: Self = Self(49);
    pub const VK_FORMAT_B8G8R8A8_SRGB: Self = Self(50);
    pub const VK_FORMAT_A8B8G8R8_UNORM_PACK32: Self = Self(51);
    pub const VK_FORMAT_A8B8G8R8_SNORM_PACK32: Self = Self(52);
    pub const VK_FORMAT_A8B8G8R8_USCALED_PACK32: Self = Self(53);
    pub const VK_FORMAT_A8B8G8R8_SSCALED_PACK32: Self = Self(54);
    pub const VK_FORMAT_A8B8G8R8_UINT_PACK32: Self = Self(55);
    pub const VK_FORMAT_A8B8G8R8_SINT_PACK32: Self = Self(56);
    pub const VK_FORMAT_A8B8G8R8_SRGB_PACK32: Self = Self(57);
    pub const VK_FORMAT_A2R10G10B10_UNORM_PACK32: Self = Self(58);
    pub const VK_FORMAT_A2R10G10B10_SNORM_PACK32: Self = Self(59);
    pub const VK_FORMAT_A2R10G10B10_USCALED_PACK32: Self = Self(60);
    pub const VK_FORMAT_A2R10G10B10_SSCALED_PACK32: Self = Self(61);
    pub const VK_FORMAT_A2R10G10B10_UINT_PACK32: Self = Self(62);
    pub const VK_FORMAT_A2R10G10B10_SINT_PACK32: Self = Self(63);
    pub const VK_FORMAT_A2B10G10R10_UNORM_PACK32: Self = Self(64);
    pub const VK_FORMAT_A2B10G10R10_SNORM_PACK32: Self = Self(65);
    pub const VK_FORMAT_A2B10G10R10_USCALED_PACK32: Self = Self(66);
    pub const VK_FORMAT_A2B10G10R10_SSCALED_PACK32: Self = Self(67);
    pub const VK_FORMAT_A2B10G10R10_UINT_PACK32: Self = Self(68);
    pub const VK_FORMAT_A2B10G10R10_SINT_PACK32: Self = Self(69);
    pub const VK_FORMAT_R16_UNORM: Self = Self(70);
    pub const VK_FORMAT_R16_SNORM: Self = Self(71);
    pub const VK_FORMAT_R16_USCALED: Self = Self(72);
    pub const VK_FORMAT_R16_SSCALED: Self = Self(73);
    pub const VK_FORMAT_R16_UINT: Self = Self(74);
    pub const VK_FORMAT_R16_SINT: Self = Self(75);
    pub const VK_FORMAT_R16_SFLOAT: Self = Self(76);
    pub const VK_FORMAT_R16G16_UNORM: Self = Self(77);
    pub const VK_FORMAT_R16G16_SNORM: Self = Self(78);
    pub const VK_FORMAT_R16G16_USCALED: Self = Self(79);
    pub const VK_FORMAT_R16G16_SSCALED: Self = Self(80);
    pub const VK_FORMAT_R16G16_UINT: Self = Self(81);
    pub const VK_FORMAT_R16G16_SINT: Self = Self(82);
    pub const VK_FORMAT_R16G16_SFLOAT: Self = Self(83);
    pub const VK_FORMAT_R16G16B16_UNORM: Self = Self(84);
    pub const VK_FORMAT_R16G16B16_SNORM: Self = Self(85);
    pub const VK_FORMAT_R16G16B16_USCALED: Self = Self(86);
    pub const VK_FORMAT_R16G16B16_SSCALED: Self = Self(87);
    pub const VK_FORMAT_R16G16B16_UINT: Self = Self(88);
    pub const VK_FORMAT_R16G16B16_SINT: Self = Self(89);
    pub const VK_FORMAT_R16G16B16_SFLOAT: Self = Self(90);
    pub const VK_FORMAT_R16G16B16A16_UNORM: Self = Self(91);
    pub const VK_FORMAT_R16G16B16A16_SNORM: Self = Self(92);
    pub const VK_FORMAT_R16G16B16A16_USCALED: Self = Self(93);
    pub const VK_FORMAT_R16G16B16A16_SSCALED: Self = Self(94);
    pub const VK_FORMAT_R16G16B16A16_UINT: Self = Self(95);
    pub const VK_FORMAT_R16G16B16A16_SINT: Self = Self(96);
    pub const VK_FORMAT_R16G16B16A16_SFLOAT: Self = Self(97);
    pub const VK_FORMAT_R32_UINT: Self = Self(98);
    pub const VK_FORMAT_R32_SINT: Self = Self(99);
    pub const VK_FORMAT_R32_SFLOAT: Self = Self(100);
    pub const VK_FORMAT_R32G32_UINT: Self = Self(101);
    pub const VK_FORMAT_R32G32_SINT: Self = Self(102);
    pub const VK_FORMAT_R32G32_SFLOAT: Self = Self(103);
    pub const VK_FORMAT_R32G32B32_UINT: Self = Self(104);
    pub const VK_FORMAT_R32G32B32_SINT: Self = Self(105);
    pub const VK_FORMAT_R32G32B32_SFLOAT: Self = Self(106);
    pub const VK_FORMAT_R32G32B32A32_UINT: Self = Self(107);
    pub const VK_FORMAT_R32G32B32A32_SINT: Self = Self(108);
    pub const VK_FORMAT_R32G32B32A32_SFLOAT: Self = Self(109);
    pub const VK_FORMAT_R64_UINT: Self = Self(110);
    pub const VK_FORMAT_R64_SINT: Self = Self(111);
    pub const VK_FORMAT_R64_SFLOAT: Self = Self(112);
    pub const VK_FORMAT_R64G64_UINT: Self = Self(113);
    pub const VK_FORMAT_R64G64_SINT: Self = Self(114);
    pub const VK_FORMAT_R64G64_SFLOAT: Self = Self(115);
    pub const VK_FORMAT_R64G64B64_UINT: Self = Self(116);
    pub const VK_FORMAT_R64G64B64_SINT: Self = Self(117);
    pub const VK_FORMAT_R64G64B64_SFLOAT: Self = Self(118);
    pub const VK_FORMAT_R64G64B64A64_UINT: Self = Self(119);
    pub const VK_FORMAT_R64G64B64A64_SINT: Self = Self(120);
    pub const VK_FORMAT_R64G64B64A64_SFLOAT: Self = Self(121);
    pub const VK_FORMAT_B10G11R11_UFLOAT_PACK32: Self = Self(122);
    pub const VK_FORMAT_E5B9G9R9_UFLOAT_PACK32: Self = Self(123);
    pub const VK_FORMAT_D16_UNORM: Self = Self(124);
    pub const VK_FORMAT_X8_D24_UNORM_PACK32: Self = Self(125);
    pub const VK_FORMAT_D32_SFLOAT: Self = Self(126);
    pub const VK_FORMAT_S8_UINT: Self = Self(127);
    pub const VK_FORMAT_D16_UNORM_S8_UINT: Self = Self(128);
    pub const VK_FORMAT_D24_UNORM_S8_UINT: Self = Self(129);
    pub const VK_FORMAT_D32_SFLOAT_S8_UINT: Self = Self(130);
    pub const VK_FORMAT_BC1_RGB_UNORM_BLOCK: Self = Self(131);
    pub const VK_FORMAT_BC1_RGB_SRGB_BLOCK: Self = Self(132);
    pub const VK_FORMAT_BC1_RGBA_UNORM_BLOCK: Self = Self(133);
    pub const VK_FORMAT_BC1_RGBA_SRGB_BLOCK: Self = Self(134);
    pub const VK_FORMAT_BC2_UNORM_BLOCK: Self = Self(135);
    pub const VK_FORMAT_BC2_SRGB_BLOCK: Self = Self(136);
    pub const VK_FORMAT_BC3_UNORM_BLOCK: Self = Self(137);
    pub const VK_FORMAT_BC3_SRGB_BLOCK: Self = Self(138);
    pub const VK_FORMAT_BC4_UNORM_BLOCK: Self = Self(139);
    pub const VK_FORMAT_BC4_SNORM_BLOCK: Self = Self(140);
    pub const VK_FORMAT_BC5_UNORM_BLOCK: Self = Self(141);
    pub const VK_FORMAT_BC5_SNORM_BLOCK: Self = Self(142);
    pub const VK_FORMAT_BC6H_UFLOAT_BLOCK: Self = Self(143);
    pub const VK_FORMAT_BC6H_SFLOAT_BLOCK: Self = Self(144);
    pub const VK_FORMAT_BC7_UNORM_BLOCK: Self = Self(145);
    pub const VK_FORMAT_BC7_SRGB_BLOCK: Self = Self(146);
    pub const VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK: Self = Self(147);
    pub const VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK: Self = Self(148);
    pub const VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK: Self = Self(149);
    pub const VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK: Self = Self(150);
    pub const VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK: Self = Self(151);
    pub const VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK: Self = Self(152);
    pub const VK_FORMAT_EAC_R11_UNORM_BLOCK: Self = Self(153);
    pub const VK_FORMAT_EAC_R11_SNORM_BLOCK: Self = Self(154);
    pub const VK_FORMAT_EAC_R11G11_UNORM_BLOCK: Self = Self(155);
    pub const VK_FORMAT_EAC_R11G11_SNORM_BLOCK: Self = Self(156);
    pub const VK_FORMAT_ASTC_4x4_UNORM_BLOCK: Self = Self(157);
    pub const VK_FORMAT_ASTC_4x4_SRGB_BLOCK: Self = Self(158);
    pub const VK_FORMAT_ASTC_5x4_UNORM_BLOCK: Self = Self(159);
    pub const VK_FORMAT_ASTC_5x4_SRGB_BLOCK: Self = Self(160);
    pub const VK_FORMAT_ASTC_5x5_UNORM_BLOCK: Self = Self(161);
    pub const VK_FORMAT_ASTC_5x5_SRGB_BLOCK: Self = Self(162);
    pub const VK_FORMAT_ASTC_6x5_UNORM_BLOCK: Self = Self(163);
    pub const VK_FORMAT_ASTC_6x5_SRGB_BLOCK: Self = Self(164);
    pub const VK_FORMAT_ASTC_6x6_UNORM_BLOCK: Self = Self(165);
    pub const VK_FORMAT_ASTC_6x6_SRGB_BLOCK: Self = Self(166);
    pub const VK_FORMAT_ASTC_8x5_UNORM_BLOCK: Self = Self(167);
    pub const VK_FORMAT_ASTC_8x5_SRGB_BLOCK: Self = Self(168);
    pub const VK_FORMAT_ASTC_8x6_UNORM_BLOCK: Self = Self(169);
    pub const VK_FORMAT_ASTC_8x6_SRGB_BLOCK: Self = Self(170);
    pub const VK_FORMAT_ASTC_8x8_UNORM_BLOCK: Self = Self(171);
    pub const VK_FORMAT_ASTC_8x8_SRGB_BLOCK: Self = Self(172);
    pub const VK_FORMAT_ASTC_10x5_UNORM_BLOCK: Self = Self(173);
    pub const VK_FORMAT_ASTC_10x5_SRGB_BLOCK: Self = Self(174);
    pub const VK_FORMAT_ASTC_10x6_UNORM_BLOCK: Self = Self(175);
    pub const VK_FORMAT_ASTC_10x6_SRGB_BLOCK: Self = Self(176);
    pub const VK_FORMAT_ASTC_10x8_UNORM_BLOCK: Self = Self(177);
    pub const VK_FORMAT_ASTC_10x8_SRGB_BLOCK: Self = Self(178);
    pub const VK_FORMAT_ASTC_10x10_UNORM_BLOCK: Self = Self(179);
    pub const VK_FORMAT_ASTC_10x10_SRGB_BLOCK: Self = Self(180);
    pub const VK_FORMAT_ASTC_12x10_UNORM_BLOCK: Self = Self(181);
    pub const VK_FORMAT_ASTC_12x10_SRGB_BLOCK: Self = Self(182);
    pub const VK_FORMAT_ASTC_12x12_UNORM_BLOCK: Self = Self(183);
    pub const VK_FORMAT_ASTC_12x12_SRGB_BLOCK: Self = Self(184);
}

pub struct VkStructureType(i32); //Structure type enumerant
impl VkStructureType {
    pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: Self = Self(0);
    pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: Self = Self(1);
    pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: Self = Self(2);
    pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: Self = Self(3);
    pub const VK_STRUCTURE_TYPE_SUBMIT_INFO: Self = Self(4);
    pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: Self = Self(5);
    pub const VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE: Self = Self(6);
    pub const VK_STRUCTURE_TYPE_BIND_SPARSE_INFO: Self = Self(7);
    pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO: Self = Self(8);
    pub const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO: Self = Self(9);
    pub const VK_STRUCTURE_TYPE_EVENT_CREATE_INFO: Self = Self(10);
    pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO: Self = Self(11);
    pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO: Self = Self(12);
    pub const VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO: Self = Self(13);
    pub const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO: Self = Self(14);
    pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO: Self = Self(15);
    pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: Self = Self(16);
    pub const VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO: Self = Self(17);
    pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO: Self = Self(18);
    pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: Self = Self(19);
    pub const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: Self = Self(20);
    pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO: Self = Self(21);
    pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO: Self = Self(22);
    pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO: Self = Self(23);
    pub const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: Self = Self(24);
    pub const VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: Self = Self(25);
    pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: Self = Self(26);
    pub const VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO: Self = Self(27);
    pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO: Self = Self(28);
    pub const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO: Self = Self(29);
    pub const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO: Self = Self(30);
    pub const VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO: Self = Self(31);
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO: Self = Self(32);
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: Self = Self(33);
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO: Self = Self(34);
    pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET: Self = Self(35);
    pub const VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET: Self = Self(36);
    pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO: Self = Self(37);
    pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO: Self = Self(38);
    pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: Self = Self(39);
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: Self = Self(40);
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO: Self = Self(41);
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: Self = Self(42);
    pub const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO: Self = Self(43);
    pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER: Self = Self(44);
    pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER: Self = Self(45);
    pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER: Self = Self(46);
    pub const VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO: Self = Self(47);
    pub const VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO: Self = Self(48);
}

pub struct VkSubpassContents(i32); //
impl VkSubpassContents {
    pub const VK_SUBPASS_CONTENTS_INLINE: Self = Self(0);
    pub const VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS: Self = Self(1);
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct VkResult(i32); //API result codes
impl VkResult {
    pub const VK_SUCCESS: Self = Self(0);
    pub const VK_NOT_READY: Self = Self(1);
    pub const VK_TIMEOUT: Self = Self(2);
    pub const VK_EVENT_SET: Self = Self(3);
    pub const VK_EVENT_RESET: Self = Self(4);
    pub const VK_INCOMPLETE: Self = Self(5);
    pub const VK_ERROR_OUT_OF_HOST_MEMORY: Self = Self(-1);
    pub const VK_ERROR_OUT_OF_DEVICE_MEMORY: Self = Self(-2);
    pub const VK_ERROR_INITIALIZATION_FAILED: Self = Self(-3);
    pub const VK_ERROR_DEVICE_LOST: Self = Self(-4);
    pub const VK_ERROR_MEMORY_MAP_FAILED: Self = Self(-5);
    pub const VK_ERROR_LAYER_NOT_PRESENT: Self = Self(-6);
    pub const VK_ERROR_EXTENSION_NOT_PRESENT: Self = Self(-7);
    pub const VK_ERROR_FEATURE_NOT_PRESENT: Self = Self(-8);
    pub const VK_ERROR_INCOMPATIBLE_DRIVER: Self = Self(-9);
    pub const VK_ERROR_TOO_MANY_OBJECTS: Self = Self(-10);
    pub const VK_ERROR_FORMAT_NOT_SUPPORTED: Self = Self(-11);
    pub const VK_ERROR_FRAGMENTED_POOL: Self = Self(-12);
    pub const VK_ERROR_UNKNOWN: Self = Self(-13);
}

pub struct VkDynamicState(i32); //
impl VkDynamicState {
    pub const VK_DYNAMIC_STATE_VIEWPORT: Self = Self(0);
    pub const VK_DYNAMIC_STATE_SCISSOR: Self = Self(1);
    pub const VK_DYNAMIC_STATE_LINE_WIDTH: Self = Self(2);
    pub const VK_DYNAMIC_STATE_DEPTH_BIAS: Self = Self(3);
    pub const VK_DYNAMIC_STATE_BLEND_CONSTANTS: Self = Self(4);
    pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS: Self = Self(5);
    pub const VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK: Self = Self(6);
    pub const VK_DYNAMIC_STATE_STENCIL_WRITE_MASK: Self = Self(7);
    pub const VK_DYNAMIC_STATE_STENCIL_REFERENCE: Self = Self(8);
}

pub struct VkDescriptorUpdateTemplateType(i32); //
impl VkDescriptorUpdateTemplateType {
    pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET: Self = Self(0);
}

pub struct VkObjectType(i32); //Enums to track objects of various types - also see objtypeenum attributes on type tags
impl VkObjectType {
    pub const VK_OBJECT_TYPE_UNKNOWN: Self = Self(0);
    pub const VK_OBJECT_TYPE_INSTANCE: Self = Self(1);
    pub const VK_OBJECT_TYPE_PHYSICAL_DEVICE: Self = Self(2);
    pub const VK_OBJECT_TYPE_DEVICE: Self = Self(3);
    pub const VK_OBJECT_TYPE_QUEUE: Self = Self(4);
    pub const VK_OBJECT_TYPE_SEMAPHORE: Self = Self(5);
    pub const VK_OBJECT_TYPE_COMMAND_BUFFER: Self = Self(6);
    pub const VK_OBJECT_TYPE_FENCE: Self = Self(7);
    pub const VK_OBJECT_TYPE_DEVICE_MEMORY: Self = Self(8);
    pub const VK_OBJECT_TYPE_BUFFER: Self = Self(9);
    pub const VK_OBJECT_TYPE_IMAGE: Self = Self(10);
    pub const VK_OBJECT_TYPE_EVENT: Self = Self(11);
    pub const VK_OBJECT_TYPE_QUERY_POOL: Self = Self(12);
    pub const VK_OBJECT_TYPE_BUFFER_VIEW: Self = Self(13);
    pub const VK_OBJECT_TYPE_IMAGE_VIEW: Self = Self(14);
    pub const VK_OBJECT_TYPE_SHADER_MODULE: Self = Self(15);
    pub const VK_OBJECT_TYPE_PIPELINE_CACHE: Self = Self(16);
    pub const VK_OBJECT_TYPE_PIPELINE_LAYOUT: Self = Self(17);
    pub const VK_OBJECT_TYPE_RENDER_PASS: Self = Self(18);
    pub const VK_OBJECT_TYPE_PIPELINE: Self = Self(19);
    pub const VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT: Self = Self(20);
    pub const VK_OBJECT_TYPE_SAMPLER: Self = Self(21);
    pub const VK_OBJECT_TYPE_DESCRIPTOR_POOL: Self = Self(22);
    pub const VK_OBJECT_TYPE_DESCRIPTOR_SET: Self = Self(23);
    pub const VK_OBJECT_TYPE_FRAMEBUFFER: Self = Self(24);
    pub const VK_OBJECT_TYPE_COMMAND_POOL: Self = Self(25);
}

pub struct VkRayTracingInvocationReorderModeEXT(i32); //
impl VkRayTracingInvocationReorderModeEXT {
    pub const VK_RAY_TRACING_INVOCATION_REORDER_MODE_NONE_EXT: Self = Self(0);
    pub const VK_RAY_TRACING_INVOCATION_REORDER_MODE_REORDER_EXT: Self = Self(1);
}

pub struct VkRayTracingLssIndexingModeNV(i32); //
impl VkRayTracingLssIndexingModeNV {
    pub const VK_RAY_TRACING_LSS_INDEXING_MODE_LIST_NV: Self = Self(0);
    pub const VK_RAY_TRACING_LSS_INDEXING_MODE_SUCCESSIVE_NV: Self = Self(1);
}

pub struct VkRayTracingLssPrimitiveEndCapsModeNV(i32); //
impl VkRayTracingLssPrimitiveEndCapsModeNV {
    pub const VK_RAY_TRACING_LSS_PRIMITIVE_END_CAPS_MODE_NONE_NV: Self = Self(0);
    pub const VK_RAY_TRACING_LSS_PRIMITIVE_END_CAPS_MODE_CHAINED_NV: Self = Self(1);
}

pub struct VkDirectDriverLoadingModeLUNARG(i32); //
impl VkDirectDriverLoadingModeLUNARG {
    pub const VK_DIRECT_DRIVER_LOADING_MODE_EXCLUSIVE_LUNARG: Self = Self(0);
    pub const VK_DIRECT_DRIVER_LOADING_MODE_INCLUSIVE_LUNARG: Self = Self(1);
}

pub struct VkAntiLagModeAMD(i32); //
impl VkAntiLagModeAMD {
    pub const VK_ANTI_LAG_MODE_DRIVER_CONTROL_AMD: Self = Self(0);
    pub const VK_ANTI_LAG_MODE_ON_AMD: Self = Self(1);
    pub const VK_ANTI_LAG_MODE_OFF_AMD: Self = Self(2);
}

pub struct VkAntiLagStageAMD(i32); //
impl VkAntiLagStageAMD {
    pub const VK_ANTI_LAG_STAGE_INPUT_AMD: Self = Self(0);
    pub const VK_ANTI_LAG_STAGE_PRESENT_AMD: Self = Self(1);
}

pub type VkQueueFlagBits = VkQueueFlags; //
impl VkQueueFlagBits {
    pub const VK_QUEUE_GRAPHICS_BIT: VkQueueFlags = VkQueueFlags(1);
    pub const VK_QUEUE_COMPUTE_BIT: VkQueueFlags = VkQueueFlags(2);
    pub const VK_QUEUE_TRANSFER_BIT: VkQueueFlags = VkQueueFlags(4);
    pub const VK_QUEUE_SPARSE_BINDING_BIT: VkQueueFlags = VkQueueFlags(8);
}

pub type VkCullModeFlagBits = VkCullModeFlags; //
impl VkCullModeFlagBits {
    pub const VK_CULL_MODE_NONE: VkCullModeFlags = VkCullModeFlags(0);
    pub const VK_CULL_MODE_FRONT_BIT: VkCullModeFlags = VkCullModeFlags(1);
    pub const VK_CULL_MODE_BACK_BIT: VkCullModeFlags = VkCullModeFlags(2);
    pub const VK_CULL_MODE_FRONT_AND_BACK: VkCullModeFlags = VkCullModeFlags(0);
}

pub type VkRenderPassCreateFlagBits = VkRenderPassCreateFlags; //
impl VkRenderPassCreateFlagBits {
}

pub type VkDeviceQueueCreateFlagBits = VkDeviceQueueCreateFlags; //
impl VkDeviceQueueCreateFlagBits {
}

pub type VkMemoryPropertyFlagBits = VkMemoryPropertyFlags; //
impl VkMemoryPropertyFlagBits {
    pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT: VkMemoryPropertyFlags = VkMemoryPropertyFlags(1);
    pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT: VkMemoryPropertyFlags = VkMemoryPropertyFlags(2);
    pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT: VkMemoryPropertyFlags = VkMemoryPropertyFlags(4);
    pub const VK_MEMORY_PROPERTY_HOST_CACHED_BIT: VkMemoryPropertyFlags = VkMemoryPropertyFlags(8);
    pub const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT: VkMemoryPropertyFlags = VkMemoryPropertyFlags(16);
}

pub type VkMemoryHeapFlagBits = VkMemoryHeapFlags; //
impl VkMemoryHeapFlagBits {
    pub const VK_MEMORY_HEAP_DEVICE_LOCAL_BIT: VkMemoryHeapFlags = VkMemoryHeapFlags(1);
}

pub type VkAccessFlagBits = VkAccessFlags; //
impl VkAccessFlagBits {
    pub const VK_ACCESS_INDIRECT_COMMAND_READ_BIT: VkAccessFlags = VkAccessFlags(1);
    pub const VK_ACCESS_INDEX_READ_BIT: VkAccessFlags = VkAccessFlags(2);
    pub const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlags = VkAccessFlags(4);
    pub const VK_ACCESS_UNIFORM_READ_BIT: VkAccessFlags = VkAccessFlags(8);
    pub const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT: VkAccessFlags = VkAccessFlags(16);
    pub const VK_ACCESS_SHADER_READ_BIT: VkAccessFlags = VkAccessFlags(32);
    pub const VK_ACCESS_SHADER_WRITE_BIT: VkAccessFlags = VkAccessFlags(64);
    pub const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT: VkAccessFlags = VkAccessFlags(128);
    pub const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlags = VkAccessFlags(256);
    pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlags = VkAccessFlags(512);
    pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlags = VkAccessFlags(1024);
    pub const VK_ACCESS_TRANSFER_READ_BIT: VkAccessFlags = VkAccessFlags(2048);
    pub const VK_ACCESS_TRANSFER_WRITE_BIT: VkAccessFlags = VkAccessFlags(4096);
    pub const VK_ACCESS_HOST_READ_BIT: VkAccessFlags = VkAccessFlags(8192);
    pub const VK_ACCESS_HOST_WRITE_BIT: VkAccessFlags = VkAccessFlags(16384);
    pub const VK_ACCESS_MEMORY_READ_BIT: VkAccessFlags = VkAccessFlags(32768);
    pub const VK_ACCESS_MEMORY_WRITE_BIT: VkAccessFlags = VkAccessFlags(65536);
}

pub type VkBufferUsageFlagBits = VkBufferUsageFlags; //
impl VkBufferUsageFlagBits {
    pub const VK_BUFFER_USAGE_TRANSFER_SRC_BIT: VkBufferUsageFlags = VkBufferUsageFlags(1);
    pub const VK_BUFFER_USAGE_TRANSFER_DST_BIT: VkBufferUsageFlags = VkBufferUsageFlags(2);
    pub const VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT: VkBufferUsageFlags = VkBufferUsageFlags(4);
    pub const VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT: VkBufferUsageFlags = VkBufferUsageFlags(8);
    pub const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT: VkBufferUsageFlags = VkBufferUsageFlags(16);
    pub const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT: VkBufferUsageFlags = VkBufferUsageFlags(32);
    pub const VK_BUFFER_USAGE_INDEX_BUFFER_BIT: VkBufferUsageFlags = VkBufferUsageFlags(64);
    pub const VK_BUFFER_USAGE_VERTEX_BUFFER_BIT: VkBufferUsageFlags = VkBufferUsageFlags(128);
    pub const VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT: VkBufferUsageFlags = VkBufferUsageFlags(256);
}

pub type VkBufferUsageFlagBits2 = VkBufferUsageFlags2; //
impl VkBufferUsageFlagBits2 {
    pub const VK_BUFFER_USAGE_2_TRANSFER_SRC_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(1);
    pub const VK_BUFFER_USAGE_2_TRANSFER_DST_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(2);
    pub const VK_BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(4);
    pub const VK_BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(8);
    pub const VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(16);
    pub const VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(32);
    pub const VK_BUFFER_USAGE_2_INDEX_BUFFER_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(64);
    pub const VK_BUFFER_USAGE_2_VERTEX_BUFFER_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(128);
    pub const VK_BUFFER_USAGE_2_INDIRECT_BUFFER_BIT: VkBufferUsageFlags2 = VkBufferUsageFlags2(256);
}

pub type VkBufferCreateFlagBits = VkBufferCreateFlags; //
impl VkBufferCreateFlagBits {
    pub const VK_BUFFER_CREATE_SPARSE_BINDING_BIT: VkBufferCreateFlags = VkBufferCreateFlags(1);
    pub const VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT: VkBufferCreateFlags = VkBufferCreateFlags(2);
    pub const VK_BUFFER_CREATE_SPARSE_ALIASED_BIT: VkBufferCreateFlags = VkBufferCreateFlags(4);
}

pub type VkShaderStageFlagBits = VkShaderStageFlags; //
impl VkShaderStageFlagBits {
    pub const VK_SHADER_STAGE_VERTEX_BIT: VkShaderStageFlags = VkShaderStageFlags(1);
    pub const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT: VkShaderStageFlags = VkShaderStageFlags(2);
    pub const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT: VkShaderStageFlags = VkShaderStageFlags(4);
    pub const VK_SHADER_STAGE_GEOMETRY_BIT: VkShaderStageFlags = VkShaderStageFlags(8);
    pub const VK_SHADER_STAGE_FRAGMENT_BIT: VkShaderStageFlags = VkShaderStageFlags(16);
    pub const VK_SHADER_STAGE_COMPUTE_BIT: VkShaderStageFlags = VkShaderStageFlags(32);
    pub const VK_SHADER_STAGE_ALL_GRAPHICS: VkShaderStageFlags = VkShaderStageFlags(0);
    pub const VK_SHADER_STAGE_ALL: VkShaderStageFlags = VkShaderStageFlags(0);
}

pub type VkImageUsageFlagBits = VkImageUsageFlags; //
impl VkImageUsageFlagBits {
    pub const VK_IMAGE_USAGE_TRANSFER_SRC_BIT: VkImageUsageFlags = VkImageUsageFlags(1);
    pub const VK_IMAGE_USAGE_TRANSFER_DST_BIT: VkImageUsageFlags = VkImageUsageFlags(2);
    pub const VK_IMAGE_USAGE_SAMPLED_BIT: VkImageUsageFlags = VkImageUsageFlags(4);
    pub const VK_IMAGE_USAGE_STORAGE_BIT: VkImageUsageFlags = VkImageUsageFlags(8);
    pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT: VkImageUsageFlags = VkImageUsageFlags(16);
    pub const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT: VkImageUsageFlags = VkImageUsageFlags(32);
    pub const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT: VkImageUsageFlags = VkImageUsageFlags(64);
    pub const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT: VkImageUsageFlags = VkImageUsageFlags(128);
}

pub type VkImageUsageFlagBits2KHR = VkImageUsageFlags2KHR; //
impl VkImageUsageFlagBits2KHR {
    pub const VK_IMAGE_USAGE_2_TRANSFER_SRC_BIT_KHR: VkImageUsageFlags2KHR = VkImageUsageFlags2KHR(1);
    pub const VK_IMAGE_USAGE_2_TRANSFER_DST_BIT_KHR: VkImageUsageFlags2KHR = VkImageUsageFlags2KHR(2);
    pub const VK_IMAGE_USAGE_2_SAMPLED_BIT_KHR: VkImageUsageFlags2KHR = VkImageUsageFlags2KHR(4);
    pub const VK_IMAGE_USAGE_2_STORAGE_BIT_KHR: VkImageUsageFlags2KHR = VkImageUsageFlags2KHR(8);
    pub const VK_IMAGE_USAGE_2_COLOR_ATTACHMENT_BIT_KHR: VkImageUsageFlags2KHR = VkImageUsageFlags2KHR(16);
    pub const VK_IMAGE_USAGE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR: VkImageUsageFlags2KHR = VkImageUsageFlags2KHR(32);
    pub const VK_IMAGE_USAGE_2_TRANSIENT_ATTACHMENT_BIT_KHR: VkImageUsageFlags2KHR = VkImageUsageFlags2KHR(64);
    pub const VK_IMAGE_USAGE_2_INPUT_ATTACHMENT_BIT_KHR: VkImageUsageFlags2KHR = VkImageUsageFlags2KHR(128);
}

pub type VkImageCreateFlagBits = VkImageCreateFlags; //
impl VkImageCreateFlagBits {
    pub const VK_IMAGE_CREATE_SPARSE_BINDING_BIT: VkImageCreateFlags = VkImageCreateFlags(1);
    pub const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT: VkImageCreateFlags = VkImageCreateFlags(2);
    pub const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT: VkImageCreateFlags = VkImageCreateFlags(4);
    pub const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT: VkImageCreateFlags = VkImageCreateFlags(8);
    pub const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT: VkImageCreateFlags = VkImageCreateFlags(16);
}

pub type VkImageCreateFlagBits2KHR = VkImageCreateFlags2KHR; //
impl VkImageCreateFlagBits2KHR {
    pub const VK_IMAGE_CREATE_2_SPARSE_BINDING_BIT_KHR: VkImageCreateFlags2KHR = VkImageCreateFlags2KHR(1);
    pub const VK_IMAGE_CREATE_2_SPARSE_RESIDENCY_BIT_KHR: VkImageCreateFlags2KHR = VkImageCreateFlags2KHR(2);
    pub const VK_IMAGE_CREATE_2_SPARSE_ALIASED_BIT_KHR: VkImageCreateFlags2KHR = VkImageCreateFlags2KHR(4);
    pub const VK_IMAGE_CREATE_2_MUTABLE_FORMAT_BIT_KHR: VkImageCreateFlags2KHR = VkImageCreateFlags2KHR(8);
    pub const VK_IMAGE_CREATE_2_CUBE_COMPATIBLE_BIT_KHR: VkImageCreateFlags2KHR = VkImageCreateFlags2KHR(16);
}

pub type VkImageViewCreateFlagBits = VkImageViewCreateFlags; //
impl VkImageViewCreateFlagBits {
}

pub type VkSamplerCreateFlagBits = VkSamplerCreateFlags; //
impl VkSamplerCreateFlagBits {
}

pub type VkPipelineCreateFlagBits = VkPipelineCreateFlags; //
impl VkPipelineCreateFlagBits {
    pub const VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT: VkPipelineCreateFlags = VkPipelineCreateFlags(1);
    pub const VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT: VkPipelineCreateFlags = VkPipelineCreateFlags(2);
    pub const VK_PIPELINE_CREATE_DERIVATIVE_BIT: VkPipelineCreateFlags = VkPipelineCreateFlags(4);
}

pub type VkPipelineCreateFlagBits2 = VkPipelineCreateFlags2; //
impl VkPipelineCreateFlagBits2 {
    pub const VK_PIPELINE_CREATE_2_DISABLE_OPTIMIZATION_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(1);
    pub const VK_PIPELINE_CREATE_2_ALLOW_DERIVATIVES_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(2);
    pub const VK_PIPELINE_CREATE_2_DERIVATIVE_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(4);
    pub const VK_PIPELINE_CREATE_2_VIEW_INDEX_FROM_DEVICE_INDEX_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(8);
    pub const VK_PIPELINE_CREATE_2_DISPATCH_BASE_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(16);
    pub const VK_PIPELINE_CREATE_2_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(256);
    pub const VK_PIPELINE_CREATE_2_EARLY_RETURN_ON_FAILURE_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(512);
    pub const VK_PIPELINE_CREATE_2_NO_PROTECTED_ACCESS_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(134217728);
    pub const VK_PIPELINE_CREATE_2_PROTECTED_ACCESS_ONLY_BIT: VkPipelineCreateFlags2 = VkPipelineCreateFlags2(1073741824);
}

pub type VkPipelineShaderStageCreateFlagBits = VkPipelineShaderStageCreateFlags; //
impl VkPipelineShaderStageCreateFlagBits {
}

pub type VkColorComponentFlagBits = VkColorComponentFlags; //
impl VkColorComponentFlagBits {
    pub const VK_COLOR_COMPONENT_R_BIT: VkColorComponentFlags = VkColorComponentFlags(1);
    pub const VK_COLOR_COMPONENT_G_BIT: VkColorComponentFlags = VkColorComponentFlags(2);
    pub const VK_COLOR_COMPONENT_B_BIT: VkColorComponentFlags = VkColorComponentFlags(4);
    pub const VK_COLOR_COMPONENT_A_BIT: VkColorComponentFlags = VkColorComponentFlags(8);
}

pub type VkFenceCreateFlagBits = VkFenceCreateFlags; //
impl VkFenceCreateFlagBits {
    pub const VK_FENCE_CREATE_SIGNALED_BIT: VkFenceCreateFlags = VkFenceCreateFlags(1);
}

pub type VkSemaphoreCreateFlagBits = VkSemaphoreCreateFlags; //
impl VkSemaphoreCreateFlagBits {
}

pub type VkFormatFeatureFlagBits = VkFormatFeatureFlags; //
impl VkFormatFeatureFlagBits {
    pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(1);
    pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(2);
    pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(4);
    pub const VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(8);
    pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(16);
    pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(32);
    pub const VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(64);
    pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(128);
    pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(256);
    pub const VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(512);
    pub const VK_FORMAT_FEATURE_BLIT_SRC_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(1024);
    pub const VK_FORMAT_FEATURE_BLIT_DST_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(2048);
    pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlags = VkFormatFeatureFlags(4096);
}

pub type VkQueryControlFlagBits = VkQueryControlFlags; //
impl VkQueryControlFlagBits {
    pub const VK_QUERY_CONTROL_PRECISE_BIT: VkQueryControlFlags = VkQueryControlFlags(1);
}

pub type VkQueryResultFlagBits = VkQueryResultFlags; //
impl VkQueryResultFlagBits {
    pub const VK_QUERY_RESULT_64_BIT: VkQueryResultFlags = VkQueryResultFlags(1);
    pub const VK_QUERY_RESULT_WAIT_BIT: VkQueryResultFlags = VkQueryResultFlags(2);
    pub const VK_QUERY_RESULT_WITH_AVAILABILITY_BIT: VkQueryResultFlags = VkQueryResultFlags(4);
    pub const VK_QUERY_RESULT_PARTIAL_BIT: VkQueryResultFlags = VkQueryResultFlags(8);
}

pub type VkCommandBufferUsageFlagBits = VkCommandBufferUsageFlags; //
impl VkCommandBufferUsageFlagBits {
    pub const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT: VkCommandBufferUsageFlags = VkCommandBufferUsageFlags(1);
    pub const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT: VkCommandBufferUsageFlags = VkCommandBufferUsageFlags(2);
    pub const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT: VkCommandBufferUsageFlags = VkCommandBufferUsageFlags(4);
}

pub type VkQueryPipelineStatisticFlagBits = VkQueryPipelineStatisticFlags; //
impl VkQueryPipelineStatisticFlagBits {
    pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(1);
    pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(2);
    pub const VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(4);
    pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(8);
    pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(16);
    pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(32);
    pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(64);
    pub const VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(128);
    pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(256);
    pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(512);
    pub const VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT: VkQueryPipelineStatisticFlags = VkQueryPipelineStatisticFlags(1024);
}

pub type VkMemoryMapFlagBits = VkMemoryMapFlags; //
impl VkMemoryMapFlagBits {
}

pub type VkImageAspectFlagBits = VkImageAspectFlags; //
impl VkImageAspectFlagBits {
    pub const VK_IMAGE_ASPECT_COLOR_BIT: VkImageAspectFlags = VkImageAspectFlags(1);
    pub const VK_IMAGE_ASPECT_DEPTH_BIT: VkImageAspectFlags = VkImageAspectFlags(2);
    pub const VK_IMAGE_ASPECT_STENCIL_BIT: VkImageAspectFlags = VkImageAspectFlags(4);
    pub const VK_IMAGE_ASPECT_METADATA_BIT: VkImageAspectFlags = VkImageAspectFlags(8);
}

pub type VkSparseImageFormatFlagBits = VkSparseImageFormatFlags; //
impl VkSparseImageFormatFlagBits {
    pub const VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT: VkSparseImageFormatFlags = VkSparseImageFormatFlags(1);
    pub const VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT: VkSparseImageFormatFlags = VkSparseImageFormatFlags(2);
    pub const VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT: VkSparseImageFormatFlags = VkSparseImageFormatFlags(4);
}

pub type VkSparseMemoryBindFlagBits = VkSparseMemoryBindFlags; //
impl VkSparseMemoryBindFlagBits {
    pub const VK_SPARSE_MEMORY_BIND_METADATA_BIT: VkSparseMemoryBindFlags = VkSparseMemoryBindFlags(1);
}

pub type VkPipelineStageFlagBits = VkPipelineStageFlags; //
impl VkPipelineStageFlagBits {
    pub const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT: VkPipelineStageFlags = VkPipelineStageFlags(1);
    pub const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT: VkPipelineStageFlags = VkPipelineStageFlags(2);
    pub const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT: VkPipelineStageFlags = VkPipelineStageFlags(4);
    pub const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT: VkPipelineStageFlags = VkPipelineStageFlags(8);
    pub const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlags = VkPipelineStageFlags(16);
    pub const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlags = VkPipelineStageFlags(32);
    pub const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT: VkPipelineStageFlags = VkPipelineStageFlags(64);
    pub const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT: VkPipelineStageFlags = VkPipelineStageFlags(128);
    pub const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlags = VkPipelineStageFlags(256);
    pub const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlags = VkPipelineStageFlags(512);
    pub const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlags = VkPipelineStageFlags(1024);
    pub const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT: VkPipelineStageFlags = VkPipelineStageFlags(2048);
    pub const VK_PIPELINE_STAGE_TRANSFER_BIT: VkPipelineStageFlags = VkPipelineStageFlags(4096);
    pub const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlags = VkPipelineStageFlags(8192);
    pub const VK_PIPELINE_STAGE_HOST_BIT: VkPipelineStageFlags = VkPipelineStageFlags(16384);
    pub const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT: VkPipelineStageFlags = VkPipelineStageFlags(32768);
    pub const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT: VkPipelineStageFlags = VkPipelineStageFlags(65536);
}

pub type VkCommandPoolCreateFlagBits = VkCommandPoolCreateFlags; //
impl VkCommandPoolCreateFlagBits {
    pub const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT: VkCommandPoolCreateFlags = VkCommandPoolCreateFlags(1);
    pub const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT: VkCommandPoolCreateFlags = VkCommandPoolCreateFlags(2);
}

pub type VkCommandPoolResetFlagBits = VkCommandPoolResetFlags; //
impl VkCommandPoolResetFlagBits {
    pub const VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT: VkCommandPoolResetFlags = VkCommandPoolResetFlags(1);
}

pub type VkCommandBufferResetFlagBits = VkCommandBufferResetFlags; //
impl VkCommandBufferResetFlagBits {
    pub const VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT: VkCommandBufferResetFlags = VkCommandBufferResetFlags(1);
}

pub type VkSampleCountFlagBits = VkSampleCountFlags; //
impl VkSampleCountFlagBits {
    pub const VK_SAMPLE_COUNT_1_BIT: VkSampleCountFlags = VkSampleCountFlags(1);
    pub const VK_SAMPLE_COUNT_2_BIT: VkSampleCountFlags = VkSampleCountFlags(2);
    pub const VK_SAMPLE_COUNT_4_BIT: VkSampleCountFlags = VkSampleCountFlags(4);
    pub const VK_SAMPLE_COUNT_8_BIT: VkSampleCountFlags = VkSampleCountFlags(8);
    pub const VK_SAMPLE_COUNT_16_BIT: VkSampleCountFlags = VkSampleCountFlags(16);
    pub const VK_SAMPLE_COUNT_32_BIT: VkSampleCountFlags = VkSampleCountFlags(32);
    pub const VK_SAMPLE_COUNT_64_BIT: VkSampleCountFlags = VkSampleCountFlags(64);
}

pub type VkAttachmentDescriptionFlagBits = VkAttachmentDescriptionFlags; //
impl VkAttachmentDescriptionFlagBits {
    pub const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT: VkAttachmentDescriptionFlags = VkAttachmentDescriptionFlags(1);
}

pub type VkStencilFaceFlagBits = VkStencilFaceFlags; //
impl VkStencilFaceFlagBits {
    pub const VK_STENCIL_FACE_FRONT_BIT: VkStencilFaceFlags = VkStencilFaceFlags(1);
    pub const VK_STENCIL_FACE_BACK_BIT: VkStencilFaceFlags = VkStencilFaceFlags(2);
    pub const VK_STENCIL_FACE_FRONT_AND_BACK: VkStencilFaceFlags = VkStencilFaceFlags(0);
    pub const VK_STENCIL_FRONT_AND_BACK: VkStencilFaceFlags = Self::VK_STENCIL_FACE_FRONT_AND_BACK;
}

pub type VkDescriptorPoolCreateFlagBits = VkDescriptorPoolCreateFlags; //
impl VkDescriptorPoolCreateFlagBits {
    pub const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT: VkDescriptorPoolCreateFlags = VkDescriptorPoolCreateFlags(1);
}

pub type VkDependencyFlagBits = VkDependencyFlags; //
impl VkDependencyFlagBits {
    pub const VK_DEPENDENCY_BY_REGION_BIT: VkDependencyFlags = VkDependencyFlags(1);
}

pub struct VkSemaphoreType(i32); //
impl VkSemaphoreType {
    pub const VK_SEMAPHORE_TYPE_BINARY: Self = Self(0);
    pub const VK_SEMAPHORE_TYPE_TIMELINE: Self = Self(1);
}

pub type VkSemaphoreWaitFlagBits = VkSemaphoreWaitFlags; //
impl VkSemaphoreWaitFlagBits {
    pub const VK_SEMAPHORE_WAIT_ANY_BIT: VkSemaphoreWaitFlags = VkSemaphoreWaitFlags(1);
}

pub struct VkPresentModeKHR(i32); //
impl VkPresentModeKHR {
    pub const VK_PRESENT_MODE_IMMEDIATE_KHR: Self = Self(0);
    pub const VK_PRESENT_MODE_MAILBOX_KHR: Self = Self(1);
    pub const VK_PRESENT_MODE_FIFO_KHR: Self = Self(2);
    pub const VK_PRESENT_MODE_FIFO_RELAXED_KHR: Self = Self(3);
}

pub struct VkColorSpaceKHR(i32); //
impl VkColorSpaceKHR {
    pub const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR: Self = Self(0);
    pub const VK_COLORSPACE_SRGB_NONLINEAR_KHR: Self = Self::VK_COLOR_SPACE_SRGB_NONLINEAR_KHR;
}

pub type VkDisplayPlaneAlphaFlagBitsKHR = VkDisplayPlaneAlphaFlagsKHR; //
impl VkDisplayPlaneAlphaFlagBitsKHR {
    pub const VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR: VkDisplayPlaneAlphaFlagsKHR = VkDisplayPlaneAlphaFlagsKHR(1);
    pub const VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR: VkDisplayPlaneAlphaFlagsKHR = VkDisplayPlaneAlphaFlagsKHR(2);
    pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR: VkDisplayPlaneAlphaFlagsKHR = VkDisplayPlaneAlphaFlagsKHR(4);
    pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR: VkDisplayPlaneAlphaFlagsKHR = VkDisplayPlaneAlphaFlagsKHR(8);
}

pub type VkCompositeAlphaFlagBitsKHR = VkCompositeAlphaFlagsKHR; //
impl VkCompositeAlphaFlagBitsKHR {
    pub const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR: VkCompositeAlphaFlagsKHR = VkCompositeAlphaFlagsKHR(1);
    pub const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagsKHR = VkCompositeAlphaFlagsKHR(2);
    pub const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR: VkCompositeAlphaFlagsKHR = VkCompositeAlphaFlagsKHR(4);
    pub const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR: VkCompositeAlphaFlagsKHR = VkCompositeAlphaFlagsKHR(8);
}

pub type VkSurfaceTransformFlagBitsKHR = VkSurfaceTransformFlagsKHR; //
impl VkSurfaceTransformFlagBitsKHR {
    pub const VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(1);
    pub const VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(2);
    pub const VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(4);
    pub const VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(8);
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(16);
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(32);
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(64);
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(128);
    pub const VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR: VkSurfaceTransformFlagsKHR = VkSurfaceTransformFlagsKHR(256);
}

pub struct VkDisplaySurfaceStereoTypeNV(i32); //
impl VkDisplaySurfaceStereoTypeNV {
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_NONE_NV: Self = Self(0);
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_ONBOARD_DIN_NV: Self = Self(1);
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_HDMI_3D_NV: Self = Self(2);
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_INBAND_DISPLAYPORT_NV: Self = Self(3);
}

pub type VkSwapchainImageUsageFlagBitsANDROID = VkSwapchainImageUsageFlagsANDROID; //
impl VkSwapchainImageUsageFlagBitsANDROID {
    pub const VK_SWAPCHAIN_IMAGE_USAGE_SHARED_BIT_ANDROID: VkSwapchainImageUsageFlagsANDROID = VkSwapchainImageUsageFlagsANDROID(1);
}

pub struct VkTimeDomainKHR(i32); //
impl VkTimeDomainKHR {
    pub const VK_TIME_DOMAIN_DEVICE_KHR: Self = Self(0);
    pub const VK_TIME_DOMAIN_CLOCK_MONOTONIC_KHR: Self = Self(1);
    pub const VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_KHR: Self = Self(2);
    pub const VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_KHR: Self = Self(3);
}

pub type VkDebugReportFlagBitsEXT = VkDebugReportFlagsEXT; //
impl VkDebugReportFlagBitsEXT {
    pub const VK_DEBUG_REPORT_INFORMATION_BIT_EXT: VkDebugReportFlagsEXT = VkDebugReportFlagsEXT(1);
    pub const VK_DEBUG_REPORT_WARNING_BIT_EXT: VkDebugReportFlagsEXT = VkDebugReportFlagsEXT(2);
    pub const VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT: VkDebugReportFlagsEXT = VkDebugReportFlagsEXT(4);
    pub const VK_DEBUG_REPORT_ERROR_BIT_EXT: VkDebugReportFlagsEXT = VkDebugReportFlagsEXT(8);
    pub const VK_DEBUG_REPORT_DEBUG_BIT_EXT: VkDebugReportFlagsEXT = VkDebugReportFlagsEXT(16);
}

pub struct VkDebugReportObjectTypeEXT(i32); //
impl VkDebugReportObjectTypeEXT {
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT: Self = Self(0);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT: Self = Self(1);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT: Self = Self(2);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT: Self = Self(3);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT: Self = Self(4);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT: Self = Self(5);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT: Self = Self(6);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT: Self = Self(7);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT: Self = Self(8);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT: Self = Self(9);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT: Self = Self(10);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT: Self = Self(11);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT: Self = Self(12);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT: Self = Self(13);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT: Self = Self(14);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT: Self = Self(15);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT: Self = Self(16);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT: Self = Self(17);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT: Self = Self(18);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT: Self = Self(19);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT: Self = Self(20);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT: Self = Self(21);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT: Self = Self(22);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT: Self = Self(23);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT: Self = Self(24);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT: Self = Self(25);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT: Self = Self(26);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT: Self = Self(27);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT: Self = Self(28);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT: Self = Self::VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT: Self = Self(29);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT: Self = Self(30);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT: Self = Self(33);
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT: Self = Self::VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT;
}

pub struct VkDeviceMemoryReportEventTypeEXT(i32); //
impl VkDeviceMemoryReportEventTypeEXT {
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT: Self = Self(0);
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT: Self = Self(1);
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT: Self = Self(2);
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT: Self = Self(3);
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT: Self = Self(4);
}

pub struct VkRasterizationOrderAMD(i32); //
impl VkRasterizationOrderAMD {
    pub const VK_RASTERIZATION_ORDER_STRICT_AMD: Self = Self(0);
    pub const VK_RASTERIZATION_ORDER_RELAXED_AMD: Self = Self(1);
}

pub type VkExternalMemoryHandleTypeFlagBitsNV = VkExternalMemoryHandleTypeFlagsNV; //
impl VkExternalMemoryHandleTypeFlagBitsNV {
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV: VkExternalMemoryHandleTypeFlagsNV = VkExternalMemoryHandleTypeFlagsNV(1);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV: VkExternalMemoryHandleTypeFlagsNV = VkExternalMemoryHandleTypeFlagsNV(2);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV: VkExternalMemoryHandleTypeFlagsNV = VkExternalMemoryHandleTypeFlagsNV(4);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV: VkExternalMemoryHandleTypeFlagsNV = VkExternalMemoryHandleTypeFlagsNV(8);
}

pub type VkExternalMemoryFeatureFlagBitsNV = VkExternalMemoryFeatureFlagsNV; //
impl VkExternalMemoryFeatureFlagBitsNV {
    pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV: VkExternalMemoryFeatureFlagsNV = VkExternalMemoryFeatureFlagsNV(1);
    pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV: VkExternalMemoryFeatureFlagsNV = VkExternalMemoryFeatureFlagsNV(2);
    pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV: VkExternalMemoryFeatureFlagsNV = VkExternalMemoryFeatureFlagsNV(4);
}

pub type VkClusterAccelerationStructureIndexFormatFlagBitsNV = VkClusterAccelerationStructureIndexFormatFlagsNV; //
impl VkClusterAccelerationStructureIndexFormatFlagBitsNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_8BIT_NV: VkClusterAccelerationStructureIndexFormatFlagsNV = VkClusterAccelerationStructureIndexFormatFlagsNV(1);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_16BIT_NV: VkClusterAccelerationStructureIndexFormatFlagsNV = VkClusterAccelerationStructureIndexFormatFlagsNV(2);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_32BIT_NV: VkClusterAccelerationStructureIndexFormatFlagsNV = VkClusterAccelerationStructureIndexFormatFlagsNV(4);
}

pub struct VkClusterAccelerationStructureTypeNV(i32); //
impl VkClusterAccelerationStructureTypeNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_TYPE_CLUSTERS_BOTTOM_LEVEL_NV: Self = Self(0);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_TYPE_TRIANGLE_CLUSTER_NV: Self = Self(1);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_TYPE_TRIANGLE_CLUSTER_TEMPLATE_NV: Self = Self(2);
}

pub struct VkClusterAccelerationStructureOpTypeNV(i32); //
impl VkClusterAccelerationStructureOpTypeNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_MOVE_OBJECTS_NV: Self = Self(0);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_CLUSTERS_BOTTOM_LEVEL_NV: Self = Self(1);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_TRIANGLE_CLUSTER_NV: Self = Self(2);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_TRIANGLE_CLUSTER_TEMPLATE_NV: Self = Self(3);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_INSTANTIATE_TRIANGLE_CLUSTER_NV: Self = Self(4);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_GET_CLUSTER_TEMPLATE_INDICES_NV: Self = Self(5);
}

pub struct VkClusterAccelerationStructureOpModeNV(i32); //
impl VkClusterAccelerationStructureOpModeNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_IMPLICIT_DESTINATIONS_NV: Self = Self(0);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_EXPLICIT_DESTINATIONS_NV: Self = Self(1);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_COMPUTE_SIZES_NV: Self = Self(2);
}

pub type VkClusterAccelerationStructureClusterFlagBitsNV = VkClusterAccelerationStructureClusterFlagsNV; //
impl VkClusterAccelerationStructureClusterFlagBitsNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_CLUSTER_ALLOW_DISABLE_OPACITY_MICROMAPS_NV: VkClusterAccelerationStructureClusterFlagsNV = VkClusterAccelerationStructureClusterFlagsNV(1);
}

pub type VkClusterAccelerationStructureGeometryFlagBitsNV = VkClusterAccelerationStructureGeometryFlagsNV; //
impl VkClusterAccelerationStructureGeometryFlagBitsNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_CULL_DISABLE_BIT_NV: VkClusterAccelerationStructureGeometryFlagsNV = VkClusterAccelerationStructureGeometryFlagsNV(1);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_NO_DUPLICATE_ANYHIT_INVOCATION_BIT_NV: VkClusterAccelerationStructureGeometryFlagsNV = VkClusterAccelerationStructureGeometryFlagsNV(2);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_OPAQUE_BIT_NV: VkClusterAccelerationStructureGeometryFlagsNV = VkClusterAccelerationStructureGeometryFlagsNV(4);
}

pub type VkClusterAccelerationStructureAddressResolutionFlagBitsNV = VkClusterAccelerationStructureAddressResolutionFlagsNV; //
impl VkClusterAccelerationStructureAddressResolutionFlagBitsNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_NONE_NV: VkClusterAccelerationStructureAddressResolutionFlagsNV = VkClusterAccelerationStructureAddressResolutionFlagsNV(0);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_IMPLICIT_DATA_BIT_NV: VkClusterAccelerationStructureAddressResolutionFlagsNV = VkClusterAccelerationStructureAddressResolutionFlagsNV(1);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SCRATCH_DATA_BIT_NV: VkClusterAccelerationStructureAddressResolutionFlagsNV = VkClusterAccelerationStructureAddressResolutionFlagsNV(2);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_ADDRESS_ARRAY_BIT_NV: VkClusterAccelerationStructureAddressResolutionFlagsNV = VkClusterAccelerationStructureAddressResolutionFlagsNV(4);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_SIZES_ARRAY_BIT_NV: VkClusterAccelerationStructureAddressResolutionFlagsNV = VkClusterAccelerationStructureAddressResolutionFlagsNV(8);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_ARRAY_BIT_NV: VkClusterAccelerationStructureAddressResolutionFlagsNV = VkClusterAccelerationStructureAddressResolutionFlagsNV(16);
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_COUNT_BIT_NV: VkClusterAccelerationStructureAddressResolutionFlagsNV = VkClusterAccelerationStructureAddressResolutionFlagsNV(32);
}

pub struct VkValidationCheckEXT(i32); //
impl VkValidationCheckEXT {
    pub const VK_VALIDATION_CHECK_ALL_EXT: Self = Self(0);
    pub const VK_VALIDATION_CHECK_SHADERS_EXT: Self = Self(1);
}

pub struct VkValidationFeatureEnableEXT(i32); //
impl VkValidationFeatureEnableEXT {
    pub const VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT: Self = Self(0);
    pub const VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT: Self = Self(1);
    pub const VK_VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT: Self = Self(2);
    pub const VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT: Self = Self(3);
    pub const VK_VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT: Self = Self(4);
}

pub struct VkValidationFeatureDisableEXT(i32); //
impl VkValidationFeatureDisableEXT {
    pub const VK_VALIDATION_FEATURE_DISABLE_ALL_EXT: Self = Self(0);
    pub const VK_VALIDATION_FEATURE_DISABLE_SHADERS_EXT: Self = Self(1);
    pub const VK_VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT: Self = Self(2);
    pub const VK_VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT: Self = Self(3);
    pub const VK_VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT: Self = Self(4);
    pub const VK_VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT: Self = Self(5);
    pub const VK_VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT: Self = Self(6);
    pub const VK_VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE_EXT: Self = Self(7);
}

pub struct VkLayerSettingTypeEXT(i32); //
impl VkLayerSettingTypeEXT {
    pub const VK_LAYER_SETTING_TYPE_BOOL32_EXT: Self = Self(0);
    pub const VK_LAYER_SETTING_TYPE_INT32_EXT: Self = Self(1);
    pub const VK_LAYER_SETTING_TYPE_INT64_EXT: Self = Self(2);
    pub const VK_LAYER_SETTING_TYPE_UINT32_EXT: Self = Self(3);
    pub const VK_LAYER_SETTING_TYPE_UINT64_EXT: Self = Self(4);
    pub const VK_LAYER_SETTING_TYPE_FLOAT32_EXT: Self = Self(5);
    pub const VK_LAYER_SETTING_TYPE_FLOAT64_EXT: Self = Self(6);
    pub const VK_LAYER_SETTING_TYPE_STRING_EXT: Self = Self(7);
}

pub type VkSubgroupFeatureFlagBits = VkSubgroupFeatureFlags; //
impl VkSubgroupFeatureFlagBits {
    pub const VK_SUBGROUP_FEATURE_BASIC_BIT: VkSubgroupFeatureFlags = VkSubgroupFeatureFlags(1);
    pub const VK_SUBGROUP_FEATURE_VOTE_BIT: VkSubgroupFeatureFlags = VkSubgroupFeatureFlags(2);
    pub const VK_SUBGROUP_FEATURE_ARITHMETIC_BIT: VkSubgroupFeatureFlags = VkSubgroupFeatureFlags(4);
    pub const VK_SUBGROUP_FEATURE_BALLOT_BIT: VkSubgroupFeatureFlags = VkSubgroupFeatureFlags(8);
    pub const VK_SUBGROUP_FEATURE_SHUFFLE_BIT: VkSubgroupFeatureFlags = VkSubgroupFeatureFlags(16);
    pub const VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT: VkSubgroupFeatureFlags = VkSubgroupFeatureFlags(32);
    pub const VK_SUBGROUP_FEATURE_CLUSTERED_BIT: VkSubgroupFeatureFlags = VkSubgroupFeatureFlags(64);
    pub const VK_SUBGROUP_FEATURE_QUAD_BIT: VkSubgroupFeatureFlags = VkSubgroupFeatureFlags(128);
}

pub type VkIndirectCommandsLayoutUsageFlagBitsNV = VkIndirectCommandsLayoutUsageFlagsNV; //
impl VkIndirectCommandsLayoutUsageFlagBitsNV {
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV: VkIndirectCommandsLayoutUsageFlagsNV = VkIndirectCommandsLayoutUsageFlagsNV(1);
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV: VkIndirectCommandsLayoutUsageFlagsNV = VkIndirectCommandsLayoutUsageFlagsNV(2);
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV: VkIndirectCommandsLayoutUsageFlagsNV = VkIndirectCommandsLayoutUsageFlagsNV(4);
}

pub type VkIndirectStateFlagBitsNV = VkIndirectStateFlagsNV; //
impl VkIndirectStateFlagBitsNV {
    pub const VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV: VkIndirectStateFlagsNV = VkIndirectStateFlagsNV(1);
}

pub struct VkIndirectCommandsTokenTypeNV(i32); //
impl VkIndirectCommandsTokenTypeNV {
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV: Self = Self(0);
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV: Self = Self(1);
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV: Self = Self(2);
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV: Self = Self(3);
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV: Self = Self(4);
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV: Self = Self(5);
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV: Self = Self(6);
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV: Self = Self(7);
}

pub type VkPrivateDataSlotCreateFlagBits = VkPrivateDataSlotCreateFlags; //
impl VkPrivateDataSlotCreateFlagBits {
}

pub type VkDescriptorSetLayoutCreateFlagBits = VkDescriptorSetLayoutCreateFlags; //
impl VkDescriptorSetLayoutCreateFlagBits {
}

pub type VkExternalMemoryHandleTypeFlagBits = VkExternalMemoryHandleTypeFlags; //
impl VkExternalMemoryHandleTypeFlagBits {
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlags(1);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlags(2);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlags(4);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT: VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlags(8);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT: VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlags(16);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT: VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlags(32);
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT: VkExternalMemoryHandleTypeFlags = VkExternalMemoryHandleTypeFlags(64);
}

pub type VkExternalMemoryFeatureFlagBits = VkExternalMemoryFeatureFlags; //
impl VkExternalMemoryFeatureFlagBits {
    pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT: VkExternalMemoryFeatureFlags = VkExternalMemoryFeatureFlags(1);
    pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT: VkExternalMemoryFeatureFlags = VkExternalMemoryFeatureFlags(2);
    pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT: VkExternalMemoryFeatureFlags = VkExternalMemoryFeatureFlags(4);
}

pub type VkExternalSemaphoreHandleTypeFlagBits = VkExternalSemaphoreHandleTypeFlags; //
impl VkExternalSemaphoreHandleTypeFlagBits {
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalSemaphoreHandleTypeFlags = VkExternalSemaphoreHandleTypeFlags(1);
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalSemaphoreHandleTypeFlags = VkExternalSemaphoreHandleTypeFlags(2);
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalSemaphoreHandleTypeFlags = VkExternalSemaphoreHandleTypeFlags(4);
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT: VkExternalSemaphoreHandleTypeFlags = VkExternalSemaphoreHandleTypeFlags(8);
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT: VkExternalSemaphoreHandleTypeFlags = Self::VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT;
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT: VkExternalSemaphoreHandleTypeFlags = VkExternalSemaphoreHandleTypeFlags(16);
}

pub type VkExternalSemaphoreFeatureFlagBits = VkExternalSemaphoreFeatureFlags; //
impl VkExternalSemaphoreFeatureFlagBits {
    pub const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT: VkExternalSemaphoreFeatureFlags = VkExternalSemaphoreFeatureFlags(1);
    pub const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT: VkExternalSemaphoreFeatureFlags = VkExternalSemaphoreFeatureFlags(2);
}

pub type VkSemaphoreImportFlagBits = VkSemaphoreImportFlags; //
impl VkSemaphoreImportFlagBits {
    pub const VK_SEMAPHORE_IMPORT_TEMPORARY_BIT: VkSemaphoreImportFlags = VkSemaphoreImportFlags(1);
}

pub type VkExternalFenceHandleTypeFlagBits = VkExternalFenceHandleTypeFlags; //
impl VkExternalFenceHandleTypeFlagBits {
    pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT: VkExternalFenceHandleTypeFlags = VkExternalFenceHandleTypeFlags(1);
    pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT: VkExternalFenceHandleTypeFlags = VkExternalFenceHandleTypeFlags(2);
    pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT: VkExternalFenceHandleTypeFlags = VkExternalFenceHandleTypeFlags(4);
    pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT: VkExternalFenceHandleTypeFlags = VkExternalFenceHandleTypeFlags(8);
}

pub type VkExternalFenceFeatureFlagBits = VkExternalFenceFeatureFlags; //
impl VkExternalFenceFeatureFlagBits {
    pub const VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT: VkExternalFenceFeatureFlags = VkExternalFenceFeatureFlags(1);
    pub const VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT: VkExternalFenceFeatureFlags = VkExternalFenceFeatureFlags(2);
}

pub type VkFenceImportFlagBits = VkFenceImportFlags; //
impl VkFenceImportFlagBits {
    pub const VK_FENCE_IMPORT_TEMPORARY_BIT: VkFenceImportFlags = VkFenceImportFlags(1);
}

pub type VkSurfaceCounterFlagBitsEXT = VkSurfaceCounterFlagsEXT; //
impl VkSurfaceCounterFlagBitsEXT {
    pub const VK_SURFACE_COUNTER_VBLANK_BIT_EXT: VkSurfaceCounterFlagsEXT = VkSurfaceCounterFlagsEXT(1);
    pub const VK_SURFACE_COUNTER_VBLANK_EXT: VkSurfaceCounterFlagsEXT = Self::VK_SURFACE_COUNTER_VBLANK_BIT_EXT;
}

pub struct VkDisplayPowerStateEXT(i32); //
impl VkDisplayPowerStateEXT {
    pub const VK_DISPLAY_POWER_STATE_OFF_EXT: Self = Self(0);
    pub const VK_DISPLAY_POWER_STATE_SUSPEND_EXT: Self = Self(1);
    pub const VK_DISPLAY_POWER_STATE_ON_EXT: Self = Self(2);
}

pub struct VkDeviceEventTypeEXT(i32); //
impl VkDeviceEventTypeEXT {
    pub const VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT: Self = Self(0);
}

pub struct VkDisplayEventTypeEXT(i32); //
impl VkDisplayEventTypeEXT {
    pub const VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT: Self = Self(0);
}

pub type VkPeerMemoryFeatureFlagBits = VkPeerMemoryFeatureFlags; //
impl VkPeerMemoryFeatureFlagBits {
    pub const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT: VkPeerMemoryFeatureFlags = VkPeerMemoryFeatureFlags(1);
    pub const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT: VkPeerMemoryFeatureFlags = VkPeerMemoryFeatureFlags(2);
    pub const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT: VkPeerMemoryFeatureFlags = VkPeerMemoryFeatureFlags(4);
    pub const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT: VkPeerMemoryFeatureFlags = VkPeerMemoryFeatureFlags(8);
}

pub type VkMemoryAllocateFlagBits = VkMemoryAllocateFlags; //
impl VkMemoryAllocateFlagBits {
    pub const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT: VkMemoryAllocateFlags = VkMemoryAllocateFlags(1);
}

pub type VkDeviceGroupPresentModeFlagBitsKHR = VkDeviceGroupPresentModeFlagsKHR; //
impl VkDeviceGroupPresentModeFlagBitsKHR {
    pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR: VkDeviceGroupPresentModeFlagsKHR = VkDeviceGroupPresentModeFlagsKHR(1);
    pub const VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR: VkDeviceGroupPresentModeFlagsKHR = VkDeviceGroupPresentModeFlagsKHR(2);
    pub const VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR: VkDeviceGroupPresentModeFlagsKHR = VkDeviceGroupPresentModeFlagsKHR(4);
    pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR: VkDeviceGroupPresentModeFlagsKHR = VkDeviceGroupPresentModeFlagsKHR(8);
}

pub type VkSwapchainCreateFlagBitsKHR = VkSwapchainCreateFlagsKHR; //
impl VkSwapchainCreateFlagBitsKHR {
}

pub struct VkViewportCoordinateSwizzleNV(i32); //
impl VkViewportCoordinateSwizzleNV {
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV: Self = Self(0);
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV: Self = Self(1);
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV: Self = Self(2);
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV: Self = Self(3);
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV: Self = Self(4);
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV: Self = Self(5);
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV: Self = Self(6);
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV: Self = Self(7);
}

pub struct VkDiscardRectangleModeEXT(i32); //
impl VkDiscardRectangleModeEXT {
    pub const VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT: Self = Self(0);
    pub const VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT: Self = Self(1);
}

pub type VkSubpassDescriptionFlagBits = VkSubpassDescriptionFlags; //
impl VkSubpassDescriptionFlagBits {
}

pub struct VkPointClippingBehavior(i32); //
impl VkPointClippingBehavior {
    pub const VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES: Self = Self(0);
    pub const VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY: Self = Self(1);
}

pub struct VkSamplerReductionMode(i32); //
impl VkSamplerReductionMode {
    pub const VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE: Self = Self(0);
    pub const VK_SAMPLER_REDUCTION_MODE_MIN: Self = Self(1);
    pub const VK_SAMPLER_REDUCTION_MODE_MAX: Self = Self(2);
}

pub struct VkTessellationDomainOrigin(i32); //
impl VkTessellationDomainOrigin {
    pub const VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT: Self = Self(0);
    pub const VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT: Self = Self(1);
}

pub struct VkSamplerYcbcrModelConversion(i32); //
impl VkSamplerYcbcrModelConversion {
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY: Self = Self(0);
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY: Self = Self(1);
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709: Self = Self(2);
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601: Self = Self(3);
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020: Self = Self(4);
}

pub struct VkSamplerYcbcrRange(i32); //
impl VkSamplerYcbcrRange {
    pub const VK_SAMPLER_YCBCR_RANGE_ITU_FULL: Self = Self(0);
    pub const VK_SAMPLER_YCBCR_RANGE_ITU_NARROW: Self = Self(1);
}

pub struct VkChromaLocation(i32); //
impl VkChromaLocation {
    pub const VK_CHROMA_LOCATION_COSITED_EVEN: Self = Self(0);
    pub const VK_CHROMA_LOCATION_MIDPOINT: Self = Self(1);
}

pub struct VkBlendOverlapEXT(i32); //
impl VkBlendOverlapEXT {
    pub const VK_BLEND_OVERLAP_UNCORRELATED_EXT: Self = Self(0);
    pub const VK_BLEND_OVERLAP_DISJOINT_EXT: Self = Self(1);
    pub const VK_BLEND_OVERLAP_CONJOINT_EXT: Self = Self(2);
}

pub struct VkCoverageModulationModeNV(i32); //
impl VkCoverageModulationModeNV {
    pub const VK_COVERAGE_MODULATION_MODE_NONE_NV: Self = Self(0);
    pub const VK_COVERAGE_MODULATION_MODE_RGB_NV: Self = Self(1);
    pub const VK_COVERAGE_MODULATION_MODE_ALPHA_NV: Self = Self(2);
    pub const VK_COVERAGE_MODULATION_MODE_RGBA_NV: Self = Self(3);
}

pub struct VkCoverageReductionModeNV(i32); //
impl VkCoverageReductionModeNV {
    pub const VK_COVERAGE_REDUCTION_MODE_MERGE_NV: Self = Self(0);
    pub const VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV: Self = Self(1);
}

pub struct VkValidationCacheHeaderVersionEXT(i32); //
impl VkValidationCacheHeaderVersionEXT {
    pub const VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT: Self = Self(1);
}

pub struct VkShaderInfoTypeAMD(i32); //
impl VkShaderInfoTypeAMD {
    pub const VK_SHADER_INFO_TYPE_STATISTICS_AMD: Self = Self(0);
    pub const VK_SHADER_INFO_TYPE_BINARY_AMD: Self = Self(1);
    pub const VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD: Self = Self(2);
}

pub struct VkQueueGlobalPriority(i32); //
impl VkQueueGlobalPriority {
    pub const VK_QUEUE_GLOBAL_PRIORITY_LOW: Self = Self(128);
    pub const VK_QUEUE_GLOBAL_PRIORITY_MEDIUM: Self = Self(256);
    pub const VK_QUEUE_GLOBAL_PRIORITY_HIGH: Self = Self(512);
    pub const VK_QUEUE_GLOBAL_PRIORITY_REALTIME: Self = Self(1024);
}

pub type VkDebugUtilsMessageSeverityFlagBitsEXT = VkDebugUtilsMessageSeverityFlagsEXT; //
impl VkDebugUtilsMessageSeverityFlagBitsEXT {
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT: VkDebugUtilsMessageSeverityFlagsEXT = VkDebugUtilsMessageSeverityFlagsEXT(1);
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT: VkDebugUtilsMessageSeverityFlagsEXT = VkDebugUtilsMessageSeverityFlagsEXT(16);
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT: VkDebugUtilsMessageSeverityFlagsEXT = VkDebugUtilsMessageSeverityFlagsEXT(256);
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT: VkDebugUtilsMessageSeverityFlagsEXT = VkDebugUtilsMessageSeverityFlagsEXT(4096);
}

pub type VkDebugUtilsMessageTypeFlagBitsEXT = VkDebugUtilsMessageTypeFlagsEXT; //
impl VkDebugUtilsMessageTypeFlagBitsEXT {
    pub const VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT: VkDebugUtilsMessageTypeFlagsEXT = VkDebugUtilsMessageTypeFlagsEXT(1);
    pub const VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT: VkDebugUtilsMessageTypeFlagsEXT = VkDebugUtilsMessageTypeFlagsEXT(2);
    pub const VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT: VkDebugUtilsMessageTypeFlagsEXT = VkDebugUtilsMessageTypeFlagsEXT(4);
}

pub struct VkConservativeRasterizationModeEXT(i32); //
impl VkConservativeRasterizationModeEXT {
    pub const VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT: Self = Self(0);
    pub const VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT: Self = Self(1);
    pub const VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT: Self = Self(2);
}

pub type VkDescriptorBindingFlagBits = VkDescriptorBindingFlags; //
impl VkDescriptorBindingFlagBits {
    pub const VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT: VkDescriptorBindingFlags = VkDescriptorBindingFlags(1);
    pub const VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT: VkDescriptorBindingFlags = VkDescriptorBindingFlags(2);
    pub const VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT: VkDescriptorBindingFlags = VkDescriptorBindingFlags(4);
    pub const VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT: VkDescriptorBindingFlags = VkDescriptorBindingFlags(8);
}

pub struct VkVendorId(i32); //
impl VkVendorId {
    pub const VK_VENDOR_ID_KHRONOS: Self = Self(0x10000);
    pub const VK_VENDOR_ID_VIV: Self = Self(0x10001);
    pub const VK_VENDOR_ID_VSI: Self = Self(0x10002);
    pub const VK_VENDOR_ID_KAZAN: Self = Self(0x10003);
    pub const VK_VENDOR_ID_CODEPLAY: Self = Self(0x10004);
    pub const VK_VENDOR_ID_MESA: Self = Self(0x10005);
    pub const VK_VENDOR_ID_POCL: Self = Self(0x10006);
    pub const VK_VENDOR_ID_MOBILEYE: Self = Self(0x10007);
    pub const VK_VENDOR_ID_APE: Self = Self(0x10008);
}

pub struct VkDriverId(i32); //
impl VkDriverId {
    pub const VK_DRIVER_ID_AMD_PROPRIETARY: Self = Self(1);
    pub const VK_DRIVER_ID_AMD_OPEN_SOURCE: Self = Self(2);
    pub const VK_DRIVER_ID_MESA_RADV: Self = Self(3);
    pub const VK_DRIVER_ID_NVIDIA_PROPRIETARY: Self = Self(4);
    pub const VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS: Self = Self(5);
    pub const VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA: Self = Self(6);
    pub const VK_DRIVER_ID_IMAGINATION_PROPRIETARY: Self = Self(7);
    pub const VK_DRIVER_ID_QUALCOMM_PROPRIETARY: Self = Self(8);
    pub const VK_DRIVER_ID_ARM_PROPRIETARY: Self = Self(9);
    pub const VK_DRIVER_ID_GOOGLE_SWIFTSHADER: Self = Self(10);
    pub const VK_DRIVER_ID_GGP_PROPRIETARY: Self = Self(11);
    pub const VK_DRIVER_ID_BROADCOM_PROPRIETARY: Self = Self(12);
    pub const VK_DRIVER_ID_MESA_LLVMPIPE: Self = Self(13);
    pub const VK_DRIVER_ID_MOLTENVK: Self = Self(14);
    pub const VK_DRIVER_ID_COREAVI_PROPRIETARY: Self = Self(15);
    pub const VK_DRIVER_ID_JUICE_PROPRIETARY: Self = Self(16);
    pub const VK_DRIVER_ID_VERISILICON_PROPRIETARY: Self = Self(17);
    pub const VK_DRIVER_ID_MESA_TURNIP: Self = Self(18);
    pub const VK_DRIVER_ID_MESA_V3DV: Self = Self(19);
    pub const VK_DRIVER_ID_MESA_PANVK: Self = Self(20);
    pub const VK_DRIVER_ID_SAMSUNG_PROPRIETARY: Self = Self(21);
    pub const VK_DRIVER_ID_MESA_VENUS: Self = Self(22);
    pub const VK_DRIVER_ID_MESA_DOZEN: Self = Self(23);
    pub const VK_DRIVER_ID_MESA_NVK: Self = Self(24);
    pub const VK_DRIVER_ID_IMAGINATION_OPEN_SOURCE_MESA: Self = Self(25);
    pub const VK_DRIVER_ID_MESA_HONEYKRISP: Self = Self(26);
    pub const VK_DRIVER_ID_VULKAN_SC_EMULATION_ON_VULKAN: Self = Self(27);
    pub const VK_DRIVER_ID_MESA_KOSMICKRISP: Self = Self(28);
    pub const VK_DRIVER_ID_MESA_GFXSTREAM: Self = Self(29);
    pub const VK_DRIVER_ID_APE_SOFT: Self = Self(30);
    pub const VK_DRIVER_ID_RESERVED_31: Self = Self(31);
}

pub type VkConditionalRenderingFlagBitsEXT = VkConditionalRenderingFlagsEXT; //
impl VkConditionalRenderingFlagBitsEXT {
    pub const VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT: VkConditionalRenderingFlagsEXT = VkConditionalRenderingFlagsEXT(1);
}

pub type VkResolveModeFlagBits = VkResolveModeFlags; //
impl VkResolveModeFlagBits {
    pub const VK_RESOLVE_MODE_NONE: VkResolveModeFlags = VkResolveModeFlags(0);
    pub const VK_RESOLVE_MODE_SAMPLE_ZERO_BIT: VkResolveModeFlags = VkResolveModeFlags(1);
    pub const VK_RESOLVE_MODE_AVERAGE_BIT: VkResolveModeFlags = VkResolveModeFlags(2);
    pub const VK_RESOLVE_MODE_MIN_BIT: VkResolveModeFlags = VkResolveModeFlags(4);
    pub const VK_RESOLVE_MODE_MAX_BIT: VkResolveModeFlags = VkResolveModeFlags(8);
}

pub struct VkShadingRatePaletteEntryNV(i32); //
impl VkShadingRatePaletteEntryNV {
    pub const VK_SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS_NV: Self = Self(0);
    pub const VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV: Self = Self(1);
    pub const VK_SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL_NV: Self = Self(2);
    pub const VK_SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL_NV: Self = Self(3);
    pub const VK_SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL_NV: Self = Self(4);
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL_NV: Self = Self(5);
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS_NV: Self = Self(6);
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS_NV: Self = Self(7);
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS_NV: Self = Self(8);
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS_NV: Self = Self(9);
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS_NV: Self = Self(10);
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS_NV: Self = Self(11);
}

pub struct VkCoarseSampleOrderTypeNV(i32); //
impl VkCoarseSampleOrderTypeNV {
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV: Self = Self(0);
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV: Self = Self(1);
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV: Self = Self(2);
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV: Self = Self(3);
}

pub type VkGeometryInstanceFlagBitsKHR = VkGeometryInstanceFlagsKHR; //
impl VkGeometryInstanceFlagBitsKHR {
    pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR: VkGeometryInstanceFlagsKHR = VkGeometryInstanceFlagsKHR(1);
    pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR: VkGeometryInstanceFlagsKHR = VkGeometryInstanceFlagsKHR(2);
    pub const VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR: VkGeometryInstanceFlagsKHR = VkGeometryInstanceFlagsKHR(4);
    pub const VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR: VkGeometryInstanceFlagsKHR = VkGeometryInstanceFlagsKHR(8);
    pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR: VkGeometryInstanceFlagsKHR = Self::VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR;
}

pub type VkGeometryFlagBitsKHR = VkGeometryFlagsKHR; //
impl VkGeometryFlagBitsKHR {
    pub const VK_GEOMETRY_OPAQUE_BIT_KHR: VkGeometryFlagsKHR = VkGeometryFlagsKHR(1);
    pub const VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR: VkGeometryFlagsKHR = VkGeometryFlagsKHR(2);
}

pub type VkBuildAccelerationStructureFlagBitsKHR = VkBuildAccelerationStructureFlagsKHR; //
impl VkBuildAccelerationStructureFlagBitsKHR {
    pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR: VkBuildAccelerationStructureFlagsKHR = VkBuildAccelerationStructureFlagsKHR(1);
    pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR: VkBuildAccelerationStructureFlagsKHR = VkBuildAccelerationStructureFlagsKHR(2);
    pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR: VkBuildAccelerationStructureFlagsKHR = VkBuildAccelerationStructureFlagsKHR(4);
    pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR: VkBuildAccelerationStructureFlagsKHR = VkBuildAccelerationStructureFlagsKHR(8);
    pub const VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR: VkBuildAccelerationStructureFlagsKHR = VkBuildAccelerationStructureFlagsKHR(16);
}

pub type VkAccelerationStructureCreateFlagBitsKHR = VkAccelerationStructureCreateFlagsKHR; //
impl VkAccelerationStructureCreateFlagBitsKHR {
    pub const VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR: VkAccelerationStructureCreateFlagsKHR = VkAccelerationStructureCreateFlagsKHR(1);
}

pub struct VkCopyAccelerationStructureModeKHR(i32); //
impl VkCopyAccelerationStructureModeKHR {
    pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR: Self = Self(0);
    pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR: Self = Self(1);
}

pub struct VkBuildAccelerationStructureModeKHR(i32); //
impl VkBuildAccelerationStructureModeKHR {
    pub const VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR: Self = Self(0);
    pub const VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR: Self = Self(1);
}

pub struct VkAccelerationStructureTypeKHR(i32); //
impl VkAccelerationStructureTypeKHR {
    pub const VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR: Self = Self(0);
    pub const VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR: Self = Self(1);
    pub const VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR: Self = Self(2);
}

pub struct VkGeometryTypeKHR(i32); //
impl VkGeometryTypeKHR {
    pub const VK_GEOMETRY_TYPE_TRIANGLES_KHR: Self = Self(0);
    pub const VK_GEOMETRY_TYPE_AABBS_KHR: Self = Self(1);
    pub const VK_GEOMETRY_TYPE_INSTANCES_KHR: Self = Self(2);
}

pub struct VkAccelerationStructureMemoryRequirementsTypeNV(i32); //
impl VkAccelerationStructureMemoryRequirementsTypeNV {
    pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV: Self = Self(0);
    pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV: Self = Self(1);
    pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV: Self = Self(2);
}

pub struct VkAccelerationStructureBuildTypeKHR(i32); //
impl VkAccelerationStructureBuildTypeKHR {
    pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR: Self = Self(0);
    pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR: Self = Self(1);
    pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR: Self = Self(2);
}

pub struct VkRayTracingShaderGroupTypeKHR(i32); //
impl VkRayTracingShaderGroupTypeKHR {
    pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR: Self = Self(0);
    pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR: Self = Self(1);
    pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR: Self = Self(2);
}

pub struct VkAccelerationStructureCompatibilityKHR(i32); //
impl VkAccelerationStructureCompatibilityKHR {
    pub const VK_ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR: Self = Self(0);
    pub const VK_ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR: Self = Self(1);
}

pub struct VkShaderGroupShaderKHR(i32); //
impl VkShaderGroupShaderKHR {
    pub const VK_SHADER_GROUP_SHADER_GENERAL_KHR: Self = Self(0);
    pub const VK_SHADER_GROUP_SHADER_CLOSEST_HIT_KHR: Self = Self(1);
    pub const VK_SHADER_GROUP_SHADER_ANY_HIT_KHR: Self = Self(2);
    pub const VK_SHADER_GROUP_SHADER_INTERSECTION_KHR: Self = Self(3);
}

pub struct VkMemoryOverallocationBehaviorAMD(i32); //
impl VkMemoryOverallocationBehaviorAMD {
    pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD: Self = Self(0);
    pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD: Self = Self(1);
    pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD: Self = Self(2);
}

pub type VkFramebufferCreateFlagBits = VkFramebufferCreateFlags; //
impl VkFramebufferCreateFlagBits {
}

pub type VkQueryPoolCreateFlagBits = VkQueryPoolCreateFlags; //
impl VkQueryPoolCreateFlagBits {
}

pub type VkDeviceDiagnosticsConfigFlagBitsNV = VkDeviceDiagnosticsConfigFlagsNV; //
impl VkDeviceDiagnosticsConfigFlagBitsNV {
    pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV: VkDeviceDiagnosticsConfigFlagsNV = VkDeviceDiagnosticsConfigFlagsNV(1);
    pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV: VkDeviceDiagnosticsConfigFlagsNV = VkDeviceDiagnosticsConfigFlagsNV(2);
    pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV: VkDeviceDiagnosticsConfigFlagsNV = VkDeviceDiagnosticsConfigFlagsNV(4);
    pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_ERROR_REPORTING_BIT_NV: VkDeviceDiagnosticsConfigFlagsNV = VkDeviceDiagnosticsConfigFlagsNV(8);
}

pub type VkPipelineCreationFeedbackFlagBits = VkPipelineCreationFeedbackFlags; //
impl VkPipelineCreationFeedbackFlagBits {
    pub const VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT: VkPipelineCreationFeedbackFlags = VkPipelineCreationFeedbackFlags(1);
    pub const VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT: VkPipelineCreationFeedbackFlags = VkPipelineCreationFeedbackFlags(2);
    pub const VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT: VkPipelineCreationFeedbackFlags = VkPipelineCreationFeedbackFlags(4);
}

pub struct VkFullScreenExclusiveEXT(i32); //
impl VkFullScreenExclusiveEXT {
    pub const VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT: Self = Self(0);
    pub const VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT: Self = Self(1);
    pub const VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT: Self = Self(2);
    pub const VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT: Self = Self(3);
}

pub struct VkPerformanceCounterScopeKHR(i32); //
impl VkPerformanceCounterScopeKHR {
    pub const VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR: Self = Self(0);
    pub const VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR: Self = Self(1);
    pub const VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR: Self = Self(2);
    pub const VK_QUERY_SCOPE_COMMAND_BUFFER_KHR: Self = Self::VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR;
    pub const VK_QUERY_SCOPE_RENDER_PASS_KHR: Self = Self::VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR;
    pub const VK_QUERY_SCOPE_COMMAND_KHR: Self = Self::VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR;
}

pub type VkMemoryDecompressionMethodFlagBitsEXT = VkMemoryDecompressionMethodFlagsEXT; //
impl VkMemoryDecompressionMethodFlagBitsEXT {
    pub const VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_EXT: VkMemoryDecompressionMethodFlagsEXT = VkMemoryDecompressionMethodFlagsEXT(1);
    pub const VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_NV: VkMemoryDecompressionMethodFlagsEXT = Self::VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_EXT;
}

pub struct VkPerformanceCounterUnitKHR(i32); //
impl VkPerformanceCounterUnitKHR {
    pub const VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR: Self = Self(0);
    pub const VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR: Self = Self(1);
    pub const VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR: Self = Self(2);
    pub const VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR: Self = Self(3);
    pub const VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR: Self = Self(4);
    pub const VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR: Self = Self(5);
    pub const VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR: Self = Self(6);
    pub const VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR: Self = Self(7);
    pub const VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR: Self = Self(8);
    pub const VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR: Self = Self(9);
    pub const VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR: Self = Self(10);
}

pub struct VkPerformanceCounterStorageKHR(i32); //
impl VkPerformanceCounterStorageKHR {
    pub const VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR: Self = Self(0);
    pub const VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR: Self = Self(1);
    pub const VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR: Self = Self(2);
    pub const VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR: Self = Self(3);
    pub const VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR: Self = Self(4);
    pub const VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR: Self = Self(5);
}

pub type VkPerformanceCounterDescriptionFlagBitsKHR = VkPerformanceCounterDescriptionFlagsKHR; //
impl VkPerformanceCounterDescriptionFlagBitsKHR {
    pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR: VkPerformanceCounterDescriptionFlagsKHR = VkPerformanceCounterDescriptionFlagsKHR(1);
    pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR: VkPerformanceCounterDescriptionFlagsKHR = Self::VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR;
    pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR: VkPerformanceCounterDescriptionFlagsKHR = VkPerformanceCounterDescriptionFlagsKHR(2);
    pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR: VkPerformanceCounterDescriptionFlagsKHR = Self::VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR;
}

pub type VkAcquireProfilingLockFlagBitsKHR = VkAcquireProfilingLockFlagsKHR; //
impl VkAcquireProfilingLockFlagBitsKHR {
}

pub type VkShaderCorePropertiesFlagBitsAMD = VkShaderCorePropertiesFlagsAMD; //
impl VkShaderCorePropertiesFlagBitsAMD {
}

pub type VkRefreshObjectFlagBitsKHR = VkRefreshObjectFlagsKHR; //
impl VkRefreshObjectFlagBitsKHR {
}

pub struct VkPerformanceConfigurationTypeINTEL(i32); //
impl VkPerformanceConfigurationTypeINTEL {
    pub const VK_PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: Self = Self(0);
}

pub struct VkQueryPoolSamplingModeINTEL(i32); //
impl VkQueryPoolSamplingModeINTEL {
    pub const VK_QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL: Self = Self(0);
}

pub struct VkPerformanceOverrideTypeINTEL(i32); //
impl VkPerformanceOverrideTypeINTEL {
    pub const VK_PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL: Self = Self(0);
    pub const VK_PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL: Self = Self(1);
}

pub struct VkPerformanceParameterTypeINTEL(i32); //
impl VkPerformanceParameterTypeINTEL {
    pub const VK_PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL: Self = Self(0);
    pub const VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL: Self = Self(1);
}

pub struct VkPerformanceValueTypeINTEL(i32); //
impl VkPerformanceValueTypeINTEL {
    pub const VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL: Self = Self(0);
    pub const VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL: Self = Self(1);
    pub const VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL: Self = Self(2);
    pub const VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL: Self = Self(3);
    pub const VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL: Self = Self(4);
}

pub struct VkShaderFloatControlsIndependence(i32); //
impl VkShaderFloatControlsIndependence {
    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY: Self = Self(0);
    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL: Self = Self(1);
    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE: Self = Self(2);
}

pub struct VkPipelineExecutableStatisticFormatKHR(i32); //
impl VkPipelineExecutableStatisticFormatKHR {
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR: Self = Self(0);
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR: Self = Self(1);
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR: Self = Self(2);
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR: Self = Self(3);
}

pub struct VkLineRasterizationMode(i32); //
impl VkLineRasterizationMode {
    pub const VK_LINE_RASTERIZATION_MODE_DEFAULT: Self = Self(0);
    pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR: Self = Self(1);
    pub const VK_LINE_RASTERIZATION_MODE_BRESENHAM: Self = Self(2);
    pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH: Self = Self(3);
}

pub type VkShaderModuleCreateFlagBits = VkShaderModuleCreateFlags; //
impl VkShaderModuleCreateFlagBits {
}

pub type VkPipelineCompilerControlFlagBitsAMD = VkPipelineCompilerControlFlagsAMD; //
impl VkPipelineCompilerControlFlagBitsAMD {
}

pub struct VkFaultLevel(i32); //
impl VkFaultLevel {
    pub const VK_FAULT_LEVEL_UNASSIGNED: Self = Self(0);
    pub const VK_FAULT_LEVEL_CRITICAL: Self = Self(1);
    pub const VK_FAULT_LEVEL_RECOVERABLE: Self = Self(2);
    pub const VK_FAULT_LEVEL_WARNING: Self = Self(3);
}

pub struct VkFaultType(i32); //
impl VkFaultType {
    pub const VK_FAULT_TYPE_INVALID: Self = Self(0);
    pub const VK_FAULT_TYPE_UNASSIGNED: Self = Self(1);
    pub const VK_FAULT_TYPE_IMPLEMENTATION: Self = Self(2);
    pub const VK_FAULT_TYPE_SYSTEM: Self = Self(3);
    pub const VK_FAULT_TYPE_PHYSICAL_DEVICE: Self = Self(4);
    pub const VK_FAULT_TYPE_COMMAND_BUFFER_FULL: Self = Self(5);
    pub const VK_FAULT_TYPE_INVALID_API_USAGE: Self = Self(6);
}

pub struct VkFaultQueryBehavior(i32); //
impl VkFaultQueryBehavior {
    pub const VK_FAULT_QUERY_BEHAVIOR_GET_AND_CLEAR_ALL_FAULTS: Self = Self(0);
}

pub type VkToolPurposeFlagBits = VkToolPurposeFlags; //
impl VkToolPurposeFlagBits {
    pub const VK_TOOL_PURPOSE_VALIDATION_BIT: VkToolPurposeFlags = VkToolPurposeFlags(1);
    pub const VK_TOOL_PURPOSE_PROFILING_BIT: VkToolPurposeFlags = VkToolPurposeFlags(2);
    pub const VK_TOOL_PURPOSE_TRACING_BIT: VkToolPurposeFlags = VkToolPurposeFlags(4);
    pub const VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT: VkToolPurposeFlags = VkToolPurposeFlags(8);
    pub const VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT: VkToolPurposeFlags = VkToolPurposeFlags(16);
}

pub struct VkPipelineMatchControl(i32); //
impl VkPipelineMatchControl {
    pub const VK_PIPELINE_MATCH_CONTROL_APPLICATION_UUID_EXACT_MATCH: Self = Self(0);
}

pub struct VkFragmentShadingRateCombinerOpKHR(i32); //
impl VkFragmentShadingRateCombinerOpKHR {
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR: Self = Self(0);
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR: Self = Self(1);
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR: Self = Self(2);
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR: Self = Self(3);
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR: Self = Self(4);
}

pub struct VkFragmentShadingRateNV(i32); //
impl VkFragmentShadingRateNV {
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV: Self = Self(0);
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV: Self = Self(1);
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV: Self = Self(4);
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV: Self = Self(5);
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV: Self = Self(6);
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV: Self = Self(9);
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV: Self = Self(10);
    pub const VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV: Self = Self(11);
    pub const VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV: Self = Self(12);
    pub const VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV: Self = Self(13);
    pub const VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV: Self = Self(14);
    pub const VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV: Self = Self(15);
}

pub struct VkFragmentShadingRateTypeNV(i32); //
impl VkFragmentShadingRateTypeNV {
    pub const VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV: Self = Self(0);
    pub const VK_FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV: Self = Self(1);
}

pub struct VkSubpassMergeStatusEXT(i32); //
impl VkSubpassMergeStatusEXT {
    pub const VK_SUBPASS_MERGE_STATUS_MERGED_EXT: Self = Self(0);
    pub const VK_SUBPASS_MERGE_STATUS_DISALLOWED_EXT: Self = Self(1);
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SIDE_EFFECTS_EXT: Self = Self(2);
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SAMPLES_MISMATCH_EXT: Self = Self(3);
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_VIEWS_MISMATCH_EXT: Self = Self(4);
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_ALIASING_EXT: Self = Self(5);
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_DEPENDENCIES_EXT: Self = Self(6);
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT: Self = Self(7);
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT: Self = Self(8);
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_INSUFFICIENT_STORAGE_EXT: Self = Self(9);
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_DEPTH_STENCIL_COUNT_EXT: Self = Self(10);
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT: Self = Self(11);
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SINGLE_SUBPASS_EXT: Self = Self(12);
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_UNSPECIFIED_EXT: Self = Self(13);
}

pub type VkAccessFlagBits2 = VkAccessFlags2; //
impl VkAccessFlagBits2 {
    pub const VK_ACCESS_2_NONE: VkAccessFlags2 = VkAccessFlags2(0);
    pub const VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT: VkAccessFlags2 = VkAccessFlags2(1);
    pub const VK_ACCESS_2_INDEX_READ_BIT: VkAccessFlags2 = VkAccessFlags2(2);
    pub const VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT: VkAccessFlags2 = VkAccessFlags2(4);
    pub const VK_ACCESS_2_UNIFORM_READ_BIT: VkAccessFlags2 = VkAccessFlags2(8);
    pub const VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT: VkAccessFlags2 = VkAccessFlags2(16);
    pub const VK_ACCESS_2_SHADER_READ_BIT: VkAccessFlags2 = VkAccessFlags2(32);
    pub const VK_ACCESS_2_SHADER_WRITE_BIT: VkAccessFlags2 = VkAccessFlags2(64);
    pub const VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT: VkAccessFlags2 = VkAccessFlags2(128);
    pub const VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT: VkAccessFlags2 = VkAccessFlags2(256);
    pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT: VkAccessFlags2 = VkAccessFlags2(512);
    pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT: VkAccessFlags2 = VkAccessFlags2(1024);
    pub const VK_ACCESS_2_TRANSFER_READ_BIT: VkAccessFlags2 = VkAccessFlags2(2048);
    pub const VK_ACCESS_2_TRANSFER_WRITE_BIT: VkAccessFlags2 = VkAccessFlags2(4096);
    pub const VK_ACCESS_2_HOST_READ_BIT: VkAccessFlags2 = VkAccessFlags2(8192);
    pub const VK_ACCESS_2_HOST_WRITE_BIT: VkAccessFlags2 = VkAccessFlags2(16384);
    pub const VK_ACCESS_2_MEMORY_READ_BIT: VkAccessFlags2 = VkAccessFlags2(32768);
    pub const VK_ACCESS_2_MEMORY_WRITE_BIT: VkAccessFlags2 = VkAccessFlags2(65536);
    pub const VK_ACCESS_2_SHADER_SAMPLED_READ_BIT: VkAccessFlags2 = VkAccessFlags2(4294967296);
    pub const VK_ACCESS_2_SHADER_STORAGE_READ_BIT: VkAccessFlags2 = VkAccessFlags2(8589934592);
    pub const VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT: VkAccessFlags2 = VkAccessFlags2(17179869184);
}

pub type VkPipelineStageFlagBits2 = VkPipelineStageFlags2; //
impl VkPipelineStageFlagBits2 {
    pub const VK_PIPELINE_STAGE_2_NONE: VkPipelineStageFlags2 = VkPipelineStageFlags2(0);
    pub const VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(1);
    pub const VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(2);
    pub const VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(4);
    pub const VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(8);
    pub const VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(16);
    pub const VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(32);
    pub const VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(64);
    pub const VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(128);
    pub const VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(256);
    pub const VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(512);
    pub const VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(1024);
    pub const VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(2048);
    pub const VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(4096);
    pub const VK_PIPELINE_STAGE_2_TRANSFER_BIT: VkPipelineStageFlags2 = Self::VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT;
    pub const VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(8192);
    pub const VK_PIPELINE_STAGE_2_HOST_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(16384);
    pub const VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(32768);
    pub const VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(65536);
    pub const VK_PIPELINE_STAGE_2_COPY_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(4294967296);
    pub const VK_PIPELINE_STAGE_2_RESOLVE_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(8589934592);
    pub const VK_PIPELINE_STAGE_2_BLIT_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(17179869184);
    pub const VK_PIPELINE_STAGE_2_CLEAR_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(34359738368);
    pub const VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(68719476736);
    pub const VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(137438953472);
    pub const VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT: VkPipelineStageFlags2 = VkPipelineStageFlags2(274877906944);
}

pub type VkSubmitFlagBits = VkSubmitFlags; //
impl VkSubmitFlagBits {
    pub const VK_SUBMIT_PROTECTED_BIT: VkSubmitFlags = VkSubmitFlags(1);
}

pub type VkEventCreateFlagBits = VkEventCreateFlags; //
impl VkEventCreateFlagBits {
}

pub type VkPipelineLayoutCreateFlagBits = VkPipelineLayoutCreateFlags; //
impl VkPipelineLayoutCreateFlagBits {
}

pub struct VkSciSyncClientTypeNV(i32); //
impl VkSciSyncClientTypeNV {
    pub const VK_SCI_SYNC_CLIENT_TYPE_SIGNALER_NV: Self = Self(0);
    pub const VK_SCI_SYNC_CLIENT_TYPE_WAITER_NV: Self = Self(1);
    pub const VK_SCI_SYNC_CLIENT_TYPE_SIGNALER_WAITER_NV: Self = Self(2);
}

pub struct VkSciSyncPrimitiveTypeNV(i32); //
impl VkSciSyncPrimitiveTypeNV {
    pub const VK_SCI_SYNC_PRIMITIVE_TYPE_FENCE_NV: Self = Self(0);
    pub const VK_SCI_SYNC_PRIMITIVE_TYPE_SEMAPHORE_NV: Self = Self(1);
}

pub struct VkProvokingVertexModeEXT(i32); //
impl VkProvokingVertexModeEXT {
    pub const VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT: Self = Self(0);
    pub const VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT: Self = Self(1);
}

pub struct VkPipelineCacheValidationVersion(i32); //
impl VkPipelineCacheValidationVersion {
    pub const VK_PIPELINE_CACHE_VALIDATION_VERSION_SAFETY_CRITICAL_ONE: Self = Self(1);
}

pub struct VkAccelerationStructureMotionInstanceTypeNV(i32); //
impl VkAccelerationStructureMotionInstanceTypeNV {
    pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV: Self = Self(0);
    pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV: Self = Self(1);
    pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV: Self = Self(2);
}

pub type VkPipelineColorBlendStateCreateFlagBits = VkPipelineColorBlendStateCreateFlags; //
impl VkPipelineColorBlendStateCreateFlagBits {
}

pub type VkPipelineDepthStencilStateCreateFlagBits = VkPipelineDepthStencilStateCreateFlags; //
impl VkPipelineDepthStencilStateCreateFlagBits {
}

pub type VkGraphicsPipelineLibraryFlagBitsEXT = VkGraphicsPipelineLibraryFlagsEXT; //
impl VkGraphicsPipelineLibraryFlagBitsEXT {
    pub const VK_GRAPHICS_PIPELINE_LIBRARY_VERTEX_INPUT_INTERFACE_BIT_EXT: VkGraphicsPipelineLibraryFlagsEXT = VkGraphicsPipelineLibraryFlagsEXT(1);
    pub const VK_GRAPHICS_PIPELINE_LIBRARY_PRE_RASTERIZATION_SHADERS_BIT_EXT: VkGraphicsPipelineLibraryFlagsEXT = VkGraphicsPipelineLibraryFlagsEXT(2);
    pub const VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_SHADER_BIT_EXT: VkGraphicsPipelineLibraryFlagsEXT = VkGraphicsPipelineLibraryFlagsEXT(4);
    pub const VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_OUTPUT_INTERFACE_BIT_EXT: VkGraphicsPipelineLibraryFlagsEXT = VkGraphicsPipelineLibraryFlagsEXT(8);
}

pub type VkRenderingAttachmentFlagBitsKHR = VkRenderingAttachmentFlagsKHR; //
impl VkRenderingAttachmentFlagBitsKHR {
}

pub type VkResolveImageFlagBitsKHR = VkResolveImageFlagsKHR; //
impl VkResolveImageFlagBitsKHR {
}

pub type VkDeviceAddressBindingFlagBitsEXT = VkDeviceAddressBindingFlagsEXT; //
impl VkDeviceAddressBindingFlagBitsEXT {
    pub const VK_DEVICE_ADDRESS_BINDING_INTERNAL_OBJECT_BIT_EXT: VkDeviceAddressBindingFlagsEXT = VkDeviceAddressBindingFlagsEXT(1);
}

pub struct VkDeviceAddressBindingTypeEXT(i32); //
impl VkDeviceAddressBindingTypeEXT {
    pub const VK_DEVICE_ADDRESS_BINDING_TYPE_BIND_EXT: Self = Self(0);
    pub const VK_DEVICE_ADDRESS_BINDING_TYPE_UNBIND_EXT: Self = Self(1);
}

pub type VkFrameBoundaryFlagBitsEXT = VkFrameBoundaryFlagsEXT; //
impl VkFrameBoundaryFlagBitsEXT {
    pub const VK_FRAME_BOUNDARY_FRAME_END_BIT_EXT: VkFrameBoundaryFlagsEXT = VkFrameBoundaryFlagsEXT(1);
}

pub type VkPresentScalingFlagBitsKHR = VkPresentScalingFlagsKHR; //
impl VkPresentScalingFlagBitsKHR {
    pub const VK_PRESENT_SCALING_ONE_TO_ONE_BIT_KHR: VkPresentScalingFlagsKHR = VkPresentScalingFlagsKHR(1);
    pub const VK_PRESENT_SCALING_ONE_TO_ONE_BIT_EXT: VkPresentScalingFlagsKHR = Self::VK_PRESENT_SCALING_ONE_TO_ONE_BIT_KHR;
    pub const VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_KHR: VkPresentScalingFlagsKHR = VkPresentScalingFlagsKHR(2);
    pub const VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_EXT: VkPresentScalingFlagsKHR = Self::VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_KHR;
    pub const VK_PRESENT_SCALING_STRETCH_BIT_KHR: VkPresentScalingFlagsKHR = VkPresentScalingFlagsKHR(4);
    pub const VK_PRESENT_SCALING_STRETCH_BIT_EXT: VkPresentScalingFlagsKHR = Self::VK_PRESENT_SCALING_STRETCH_BIT_KHR;
}

pub type VkPresentGravityFlagBitsKHR = VkPresentGravityFlagsKHR; //
impl VkPresentGravityFlagBitsKHR {
    pub const VK_PRESENT_GRAVITY_MIN_BIT_KHR: VkPresentGravityFlagsKHR = VkPresentGravityFlagsKHR(1);
    pub const VK_PRESENT_GRAVITY_MIN_BIT_EXT: VkPresentGravityFlagsKHR = Self::VK_PRESENT_GRAVITY_MIN_BIT_KHR;
    pub const VK_PRESENT_GRAVITY_MAX_BIT_KHR: VkPresentGravityFlagsKHR = VkPresentGravityFlagsKHR(2);
    pub const VK_PRESENT_GRAVITY_MAX_BIT_EXT: VkPresentGravityFlagsKHR = Self::VK_PRESENT_GRAVITY_MAX_BIT_KHR;
    pub const VK_PRESENT_GRAVITY_CENTERED_BIT_KHR: VkPresentGravityFlagsKHR = VkPresentGravityFlagsKHR(4);
    pub const VK_PRESENT_GRAVITY_CENTERED_BIT_EXT: VkPresentGravityFlagsKHR = Self::VK_PRESENT_GRAVITY_CENTERED_BIT_KHR;
}

pub type VkPhysicalDeviceSchedulingControlsFlagBitsARM = VkPhysicalDeviceSchedulingControlsFlagsARM; //
impl VkPhysicalDeviceSchedulingControlsFlagBitsARM {
    pub const VK_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_SHADER_CORE_COUNT_ARM: VkPhysicalDeviceSchedulingControlsFlagsARM = VkPhysicalDeviceSchedulingControlsFlagsARM(1);
    pub const VK_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_ARM: VkPhysicalDeviceSchedulingControlsFlagsARM = VkPhysicalDeviceSchedulingControlsFlagsARM(2);
}

pub type VkPresentStageFlagBitsEXT = VkPresentStageFlagsEXT; //
impl VkPresentStageFlagBitsEXT {
    pub const VK_PRESENT_STAGE_QUEUE_OPERATIONS_END_BIT_EXT: VkPresentStageFlagsEXT = VkPresentStageFlagsEXT(1);
    pub const VK_PRESENT_STAGE_REQUEST_DEQUEUED_BIT_EXT: VkPresentStageFlagsEXT = VkPresentStageFlagsEXT(2);
    pub const VK_PRESENT_STAGE_IMAGE_FIRST_PIXEL_OUT_BIT_EXT: VkPresentStageFlagsEXT = VkPresentStageFlagsEXT(4);
    pub const VK_PRESENT_STAGE_IMAGE_FIRST_PIXEL_VISIBLE_BIT_EXT: VkPresentStageFlagsEXT = VkPresentStageFlagsEXT(8);
}

pub type VkPastPresentationTimingFlagBitsEXT = VkPastPresentationTimingFlagsEXT; //
impl VkPastPresentationTimingFlagBitsEXT {
    pub const VK_PAST_PRESENTATION_TIMING_ALLOW_PARTIAL_RESULTS_BIT_EXT: VkPastPresentationTimingFlagsEXT = VkPastPresentationTimingFlagsEXT(1);
    pub const VK_PAST_PRESENTATION_TIMING_ALLOW_OUT_OF_ORDER_RESULTS_BIT_EXT: VkPastPresentationTimingFlagsEXT = VkPastPresentationTimingFlagsEXT(2);
}

pub type VkPresentTimingInfoFlagBitsEXT = VkPresentTimingInfoFlagsEXT; //
impl VkPresentTimingInfoFlagBitsEXT {
    pub const VK_PRESENT_TIMING_INFO_PRESENT_AT_RELATIVE_TIME_BIT_EXT: VkPresentTimingInfoFlagsEXT = VkPresentTimingInfoFlagsEXT(1);
    pub const VK_PRESENT_TIMING_INFO_PRESENT_AT_NEAREST_REFRESH_CYCLE_BIT_EXT: VkPresentTimingInfoFlagsEXT = VkPresentTimingInfoFlagsEXT(2);
}

pub type VkVideoCodecOperationFlagBitsKHR = VkVideoCodecOperationFlagsKHR; //
impl VkVideoCodecOperationFlagBitsKHR {
    pub const VK_VIDEO_CODEC_OPERATION_NONE_KHR: VkVideoCodecOperationFlagsKHR = VkVideoCodecOperationFlagsKHR(0);
}

pub type VkVideoChromaSubsamplingFlagBitsKHR = VkVideoChromaSubsamplingFlagsKHR; //Vulkan video chroma subsampling definitions
impl VkVideoChromaSubsamplingFlagBitsKHR {
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_INVALID_KHR: VkVideoChromaSubsamplingFlagsKHR = VkVideoChromaSubsamplingFlagsKHR(0);
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_MONOCHROME_BIT_KHR: VkVideoChromaSubsamplingFlagsKHR = VkVideoChromaSubsamplingFlagsKHR(1);
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_420_BIT_KHR: VkVideoChromaSubsamplingFlagsKHR = VkVideoChromaSubsamplingFlagsKHR(2);
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_422_BIT_KHR: VkVideoChromaSubsamplingFlagsKHR = VkVideoChromaSubsamplingFlagsKHR(4);
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_444_BIT_KHR: VkVideoChromaSubsamplingFlagsKHR = VkVideoChromaSubsamplingFlagsKHR(8);
}

pub type VkVideoComponentBitDepthFlagBitsKHR = VkVideoComponentBitDepthFlagsKHR; //Vulkan video component bit depth definitions
impl VkVideoComponentBitDepthFlagBitsKHR {
    pub const VK_VIDEO_COMPONENT_BIT_DEPTH_INVALID_KHR: VkVideoComponentBitDepthFlagsKHR = VkVideoComponentBitDepthFlagsKHR(0);
    pub const VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR: VkVideoComponentBitDepthFlagsKHR = VkVideoComponentBitDepthFlagsKHR(1);
    pub const VK_VIDEO_COMPONENT_BIT_DEPTH_10_BIT_KHR: VkVideoComponentBitDepthFlagsKHR = VkVideoComponentBitDepthFlagsKHR(4);
    pub const VK_VIDEO_COMPONENT_BIT_DEPTH_12_BIT_KHR: VkVideoComponentBitDepthFlagsKHR = VkVideoComponentBitDepthFlagsKHR(16);
}

pub type VkVideoCapabilityFlagBitsKHR = VkVideoCapabilityFlagsKHR; //
impl VkVideoCapabilityFlagBitsKHR {
    pub const VK_VIDEO_CAPABILITY_PROTECTED_CONTENT_BIT_KHR: VkVideoCapabilityFlagsKHR = VkVideoCapabilityFlagsKHR(1);
    pub const VK_VIDEO_CAPABILITY_SEPARATE_REFERENCE_IMAGES_BIT_KHR: VkVideoCapabilityFlagsKHR = VkVideoCapabilityFlagsKHR(2);
}

pub type VkVideoSessionCreateFlagBitsKHR = VkVideoSessionCreateFlagsKHR; //
impl VkVideoSessionCreateFlagBitsKHR {
    pub const VK_VIDEO_SESSION_CREATE_PROTECTED_CONTENT_BIT_KHR: VkVideoSessionCreateFlagsKHR = VkVideoSessionCreateFlagsKHR(1);
}

pub type VkVideoSessionParametersCreateFlagBitsKHR = VkVideoSessionParametersCreateFlagsKHR; //
impl VkVideoSessionParametersCreateFlagBitsKHR {
}

pub type VkVideoDecodeH264PictureLayoutFlagBitsKHR = VkVideoDecodeH264PictureLayoutFlagsKHR; //
impl VkVideoDecodeH264PictureLayoutFlagBitsKHR {
    pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_KHR: VkVideoDecodeH264PictureLayoutFlagsKHR = VkVideoDecodeH264PictureLayoutFlagsKHR(0);
    pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_BIT_KHR: VkVideoDecodeH264PictureLayoutFlagsKHR = VkVideoDecodeH264PictureLayoutFlagsKHR(1);
    pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_BIT_KHR: VkVideoDecodeH264PictureLayoutFlagsKHR = VkVideoDecodeH264PictureLayoutFlagsKHR(2);
}

pub type VkVideoCodingControlFlagBitsKHR = VkVideoCodingControlFlagsKHR; //
impl VkVideoCodingControlFlagBitsKHR {
    pub const VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR: VkVideoCodingControlFlagsKHR = VkVideoCodingControlFlagsKHR(1);
}

pub struct VkQueryResultStatusKHR(i32); //
impl VkQueryResultStatusKHR {
    pub const VK_QUERY_RESULT_STATUS_ERROR_KHR: Self = Self(-1);
    pub const VK_QUERY_RESULT_STATUS_NOT_READY_KHR: Self = Self(0);
    pub const VK_QUERY_RESULT_STATUS_COMPLETE_KHR: Self = Self(1);
}

pub type VkVideoDecodeUsageFlagBitsKHR = VkVideoDecodeUsageFlagsKHR; //
impl VkVideoDecodeUsageFlagBitsKHR {
    pub const VK_VIDEO_DECODE_USAGE_DEFAULT_KHR: VkVideoDecodeUsageFlagsKHR = VkVideoDecodeUsageFlagsKHR(0);
    pub const VK_VIDEO_DECODE_USAGE_TRANSCODING_BIT_KHR: VkVideoDecodeUsageFlagsKHR = VkVideoDecodeUsageFlagsKHR(1);
    pub const VK_VIDEO_DECODE_USAGE_OFFLINE_BIT_KHR: VkVideoDecodeUsageFlagsKHR = VkVideoDecodeUsageFlagsKHR(2);
    pub const VK_VIDEO_DECODE_USAGE_STREAMING_BIT_KHR: VkVideoDecodeUsageFlagsKHR = VkVideoDecodeUsageFlagsKHR(4);
}

pub type VkVideoDecodeCapabilityFlagBitsKHR = VkVideoDecodeCapabilityFlagsKHR; //
impl VkVideoDecodeCapabilityFlagBitsKHR {
    pub const VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR: VkVideoDecodeCapabilityFlagsKHR = VkVideoDecodeCapabilityFlagsKHR(1);
    pub const VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR: VkVideoDecodeCapabilityFlagsKHR = VkVideoDecodeCapabilityFlagsKHR(2);
}

pub type VkVideoEncodeFlagBitsKHR = VkVideoEncodeFlagsKHR; //
impl VkVideoEncodeFlagBitsKHR {
}

pub type VkCooperativeMatrixFlagBitsEXT = VkCooperativeMatrixFlagsEXT; //
impl VkCooperativeMatrixFlagBitsEXT {
}

pub type VkVideoEncodeUsageFlagBitsKHR = VkVideoEncodeUsageFlagsKHR; //
impl VkVideoEncodeUsageFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_USAGE_DEFAULT_KHR: VkVideoEncodeUsageFlagsKHR = VkVideoEncodeUsageFlagsKHR(0);
    pub const VK_VIDEO_ENCODE_USAGE_TRANSCODING_BIT_KHR: VkVideoEncodeUsageFlagsKHR = VkVideoEncodeUsageFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_USAGE_STREAMING_BIT_KHR: VkVideoEncodeUsageFlagsKHR = VkVideoEncodeUsageFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_USAGE_RECORDING_BIT_KHR: VkVideoEncodeUsageFlagsKHR = VkVideoEncodeUsageFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_USAGE_CONFERENCING_BIT_KHR: VkVideoEncodeUsageFlagsKHR = VkVideoEncodeUsageFlagsKHR(8);
}

pub type VkVideoEncodeContentFlagBitsKHR = VkVideoEncodeContentFlagsKHR; //
impl VkVideoEncodeContentFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_CONTENT_DEFAULT_KHR: VkVideoEncodeContentFlagsKHR = VkVideoEncodeContentFlagsKHR(0);
    pub const VK_VIDEO_ENCODE_CONTENT_CAMERA_BIT_KHR: VkVideoEncodeContentFlagsKHR = VkVideoEncodeContentFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_CONTENT_DESKTOP_BIT_KHR: VkVideoEncodeContentFlagsKHR = VkVideoEncodeContentFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_CONTENT_RENDERED_BIT_KHR: VkVideoEncodeContentFlagsKHR = VkVideoEncodeContentFlagsKHR(4);
}

pub struct VkVideoEncodeTuningModeKHR(i32); //
impl VkVideoEncodeTuningModeKHR {
    pub const VK_VIDEO_ENCODE_TUNING_MODE_DEFAULT_KHR: Self = Self(0);
    pub const VK_VIDEO_ENCODE_TUNING_MODE_HIGH_QUALITY_KHR: Self = Self(1);
    pub const VK_VIDEO_ENCODE_TUNING_MODE_LOW_LATENCY_KHR: Self = Self(2);
    pub const VK_VIDEO_ENCODE_TUNING_MODE_ULTRA_LOW_LATENCY_KHR: Self = Self(3);
    pub const VK_VIDEO_ENCODE_TUNING_MODE_LOSSLESS_KHR: Self = Self(4);
}

pub type VkVideoEncodeCapabilityFlagBitsKHR = VkVideoEncodeCapabilityFlagsKHR; //
impl VkVideoEncodeCapabilityFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_CAPABILITY_PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR: VkVideoEncodeCapabilityFlagsKHR = VkVideoEncodeCapabilityFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_CAPABILITY_INSUFFICIENT_BITSTREAM_BUFFER_RANGE_DETECTION_BIT_KHR: VkVideoEncodeCapabilityFlagsKHR = VkVideoEncodeCapabilityFlagsKHR(2);
}

pub type VkVideoEncodeFeedbackFlagBitsKHR = VkVideoEncodeFeedbackFlagsKHR; //
impl VkVideoEncodeFeedbackFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_BUFFER_OFFSET_BIT_KHR: VkVideoEncodeFeedbackFlagsKHR = VkVideoEncodeFeedbackFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_BYTES_WRITTEN_BIT_KHR: VkVideoEncodeFeedbackFlagsKHR = VkVideoEncodeFeedbackFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_HAS_OVERRIDES_BIT_KHR: VkVideoEncodeFeedbackFlagsKHR = VkVideoEncodeFeedbackFlagsKHR(4);
}

pub type VkVideoEncodePerPartitionFeedbackFlagBitsKHR = VkVideoEncodePerPartitionFeedbackFlagsKHR; //
impl VkVideoEncodePerPartitionFeedbackFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_STATUS_BIT_KHR: VkVideoEncodePerPartitionFeedbackFlagsKHR = VkVideoEncodePerPartitionFeedbackFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_BITSTREAM_BUFFER_OFFSET_BIT_KHR: VkVideoEncodePerPartitionFeedbackFlagsKHR = VkVideoEncodePerPartitionFeedbackFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_BITSTREAM_BYTES_WRITTEN_BIT_KHR: VkVideoEncodePerPartitionFeedbackFlagsKHR = VkVideoEncodePerPartitionFeedbackFlagsKHR(4);
}

pub type VkVideoEncodeRateControlModeFlagBitsKHR = VkVideoEncodeRateControlModeFlagsKHR; //
impl VkVideoEncodeRateControlModeFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_DEFAULT_KHR: VkVideoEncodeRateControlModeFlagsKHR = VkVideoEncodeRateControlModeFlagsKHR(0);
    pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_DISABLED_BIT_KHR: VkVideoEncodeRateControlModeFlagsKHR = VkVideoEncodeRateControlModeFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_CBR_BIT_KHR: VkVideoEncodeRateControlModeFlagsKHR = VkVideoEncodeRateControlModeFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR: VkVideoEncodeRateControlModeFlagsKHR = VkVideoEncodeRateControlModeFlagsKHR(4);
}

pub type VkVideoEncodeIntraRefreshModeFlagBitsKHR = VkVideoEncodeIntraRefreshModeFlagsKHR; //
impl VkVideoEncodeIntraRefreshModeFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_NONE_KHR: VkVideoEncodeIntraRefreshModeFlagsKHR = VkVideoEncodeIntraRefreshModeFlagsKHR(0);
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_PER_PICTURE_PARTITION_BIT_KHR: VkVideoEncodeIntraRefreshModeFlagsKHR = VkVideoEncodeIntraRefreshModeFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_BASED_BIT_KHR: VkVideoEncodeIntraRefreshModeFlagsKHR = VkVideoEncodeIntraRefreshModeFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_ROW_BASED_BIT_KHR: VkVideoEncodeIntraRefreshModeFlagsKHR = VkVideoEncodeIntraRefreshModeFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_COLUMN_BASED_BIT_KHR: VkVideoEncodeIntraRefreshModeFlagsKHR = VkVideoEncodeIntraRefreshModeFlagsKHR(8);
}

pub type VkVideoEncodeH264CapabilityFlagBitsKHR = VkVideoEncodeH264CapabilityFlagsKHR; //
impl VkVideoEncodeH264CapabilityFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_HRD_COMPLIANCE_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_ROW_UNALIGNED_SLICE_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(8);
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(16);
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(32);
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(64);
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_PER_SLICE_CONSTANT_QP_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(128);
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_GENERATE_PREFIX_NALU_BIT_KHR: VkVideoEncodeH264CapabilityFlagsKHR = VkVideoEncodeH264CapabilityFlagsKHR(256);
}

pub type VkVideoEncodeH264StdFlagBitsKHR = VkVideoEncodeH264StdFlagsKHR; //
impl VkVideoEncodeH264StdFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H264_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_H264_STD_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_H264_STD_SCALING_MATRIX_PRESENT_FLAG_SET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_H264_STD_CHROMA_QP_INDEX_OFFSET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(8);
    pub const VK_VIDEO_ENCODE_H264_STD_SECOND_CHROMA_QP_INDEX_OFFSET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(16);
    pub const VK_VIDEO_ENCODE_H264_STD_PIC_INIT_QP_MINUS26_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(32);
    pub const VK_VIDEO_ENCODE_H264_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(64);
    pub const VK_VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_EXPLICIT_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(128);
    pub const VK_VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_IMPLICIT_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(256);
    pub const VK_VIDEO_ENCODE_H264_STD_TRANSFORM_8X8_MODE_FLAG_SET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(512);
    pub const VK_VIDEO_ENCODE_H264_STD_DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(1024);
    pub const VK_VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_UNSET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(2048);
    pub const VK_VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_SET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(4096);
    pub const VK_VIDEO_ENCODE_H264_STD_DIRECT_8X8_INFERENCE_FLAG_UNSET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(8192);
    pub const VK_VIDEO_ENCODE_H264_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(16384);
    pub const VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_DISABLED_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(32768);
    pub const VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_ENABLED_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(65536);
    pub const VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_PARTIAL_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(131072);
    pub const VK_VIDEO_ENCODE_H264_STD_SLICE_QP_DELTA_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(524288);
    pub const VK_VIDEO_ENCODE_H264_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR: VkVideoEncodeH264StdFlagsKHR = VkVideoEncodeH264StdFlagsKHR(1048576);
}

pub type VkVideoEncodeH264RateControlFlagBitsKHR = VkVideoEncodeH264RateControlFlagsKHR; //
impl VkVideoEncodeH264RateControlFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR: VkVideoEncodeH264RateControlFlagsKHR = VkVideoEncodeH264RateControlFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_REGULAR_GOP_BIT_KHR: VkVideoEncodeH264RateControlFlagsKHR = VkVideoEncodeH264RateControlFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR: VkVideoEncodeH264RateControlFlagsKHR = VkVideoEncodeH264RateControlFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR: VkVideoEncodeH264RateControlFlagsKHR = VkVideoEncodeH264RateControlFlagsKHR(8);
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR: VkVideoEncodeH264RateControlFlagsKHR = VkVideoEncodeH264RateControlFlagsKHR(16);
}

pub type VkHostImageCopyFlagBits = VkHostImageCopyFlags; //
impl VkHostImageCopyFlagBits {
    pub const VK_HOST_IMAGE_COPY_MEMCPY_BIT: VkHostImageCopyFlags = VkHostImageCopyFlags(1);
    pub const VK_HOST_IMAGE_COPY_MEMCPY: VkHostImageCopyFlags = Self::VK_HOST_IMAGE_COPY_MEMCPY_BIT;
}

pub struct VkPartitionedAccelerationStructureOpTypeNV(i32); //
impl VkPartitionedAccelerationStructureOpTypeNV {
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_WRITE_INSTANCE_NV: Self = Self(0);
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_UPDATE_INSTANCE_NV: Self = Self(1);
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_WRITE_PARTITION_TRANSLATION_NV: Self = Self(2);
}

pub type VkPartitionedAccelerationStructureInstanceFlagBitsNV = VkPartitionedAccelerationStructureInstanceFlagsNV; //
impl VkPartitionedAccelerationStructureInstanceFlagBitsNV {
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FACING_CULL_DISABLE_BIT_NV: VkPartitionedAccelerationStructureInstanceFlagsNV = VkPartitionedAccelerationStructureInstanceFlagsNV(1);
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FLIP_FACING_BIT_NV: VkPartitionedAccelerationStructureInstanceFlagsNV = VkPartitionedAccelerationStructureInstanceFlagsNV(2);
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_OPAQUE_BIT_NV: VkPartitionedAccelerationStructureInstanceFlagsNV = VkPartitionedAccelerationStructureInstanceFlagsNV(4);
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_NO_OPAQUE_BIT_NV: VkPartitionedAccelerationStructureInstanceFlagsNV = VkPartitionedAccelerationStructureInstanceFlagsNV(8);
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV: VkPartitionedAccelerationStructureInstanceFlagsNV = VkPartitionedAccelerationStructureInstanceFlagsNV(16);
}

pub type VkImageFormatConstraintsFlagBitsFUCHSIA = VkImageFormatConstraintsFlagsFUCHSIA; //
impl VkImageFormatConstraintsFlagBitsFUCHSIA {
}

pub type VkImageConstraintsInfoFlagBitsFUCHSIA = VkImageConstraintsInfoFlagsFUCHSIA; //
impl VkImageConstraintsInfoFlagBitsFUCHSIA {
    pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA: VkImageConstraintsInfoFlagsFUCHSIA = VkImageConstraintsInfoFlagsFUCHSIA(1);
    pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA: VkImageConstraintsInfoFlagsFUCHSIA = VkImageConstraintsInfoFlagsFUCHSIA(2);
    pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA: VkImageConstraintsInfoFlagsFUCHSIA = VkImageConstraintsInfoFlagsFUCHSIA(4);
    pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA: VkImageConstraintsInfoFlagsFUCHSIA = VkImageConstraintsInfoFlagsFUCHSIA(8);
    pub const VK_IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA: VkImageConstraintsInfoFlagsFUCHSIA = VkImageConstraintsInfoFlagsFUCHSIA(16);
}

pub type VkFormatFeatureFlagBits2 = VkFormatFeatureFlags2; //
impl VkFormatFeatureFlagBits2 {
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(1);
    pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(2);
    pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(4);
    pub const VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(8);
    pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(16);
    pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(32);
    pub const VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(64);
    pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(128);
    pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(256);
    pub const VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(512);
    pub const VK_FORMAT_FEATURE_2_BLIT_SRC_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(1024);
    pub const VK_FORMAT_FEATURE_2_BLIT_DST_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(2048);
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(4096);
    pub const VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(16384);
    pub const VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(32768);
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(65536);
    pub const VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(131072);
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(262144);
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(524288);
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(1048576);
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(2097152);
    pub const VK_FORMAT_FEATURE_2_DISJOINT_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(4194304);
    pub const VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(8388608);
    pub const VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(2147483648);
    pub const VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(4294967296);
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT: VkFormatFeatureFlags2 = VkFormatFeatureFlags2(8589934592);
}

pub type VkFormatFeatureFlagBits4KHR = VkFormatFeatureFlags4KHR; //
impl VkFormatFeatureFlagBits4KHR {
}

pub type VkRenderingFlagBits = VkRenderingFlags; //
impl VkRenderingFlagBits {
    pub const VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT: VkRenderingFlags = VkRenderingFlags(1);
    pub const VK_RENDERING_SUSPENDING_BIT: VkRenderingFlags = VkRenderingFlags(2);
    pub const VK_RENDERING_RESUMING_BIT: VkRenderingFlags = VkRenderingFlags(4);
}

pub type VkVideoEncodeH265CapabilityFlagBitsKHR = VkVideoEncodeH265CapabilityFlagsKHR; //
impl VkVideoEncodeH265CapabilityFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_HRD_COMPLIANCE_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_ROW_UNALIGNED_SLICE_SEGMENT_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_SLICE_SEGMENT_TYPE_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(8);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(16);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(32);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(64);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_PER_SLICE_SEGMENT_CONSTANT_QP_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(128);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILES_PER_SLICE_SEGMENT_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(256);
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_SLICE_SEGMENTS_PER_TILE_BIT_KHR: VkVideoEncodeH265CapabilityFlagsKHR = VkVideoEncodeH265CapabilityFlagsKHR(512);
}

pub type VkVideoEncodeH265StdFlagBitsKHR = VkVideoEncodeH265StdFlagsKHR; //
impl VkVideoEncodeH265StdFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H265_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_H265_STD_SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_H265_STD_SCALING_LIST_DATA_PRESENT_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_H265_STD_PCM_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(8);
    pub const VK_VIDEO_ENCODE_H265_STD_SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(16);
    pub const VK_VIDEO_ENCODE_H265_STD_INIT_QP_MINUS26_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(32);
    pub const VK_VIDEO_ENCODE_H265_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(64);
    pub const VK_VIDEO_ENCODE_H265_STD_WEIGHTED_BIPRED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(128);
    pub const VK_VIDEO_ENCODE_H265_STD_LOG2_PARALLEL_MERGE_LEVEL_MINUS2_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(256);
    pub const VK_VIDEO_ENCODE_H265_STD_SIGN_DATA_HIDING_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(512);
    pub const VK_VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(1024);
    pub const VK_VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_UNSET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(2048);
    pub const VK_VIDEO_ENCODE_H265_STD_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(4096);
    pub const VK_VIDEO_ENCODE_H265_STD_TRANSQUANT_BYPASS_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(8192);
    pub const VK_VIDEO_ENCODE_H265_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(16384);
    pub const VK_VIDEO_ENCODE_H265_STD_ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(32768);
    pub const VK_VIDEO_ENCODE_H265_STD_DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(65536);
    pub const VK_VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(131072);
    pub const VK_VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENT_FLAG_SET_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(262144);
    pub const VK_VIDEO_ENCODE_H265_STD_SLICE_QP_DELTA_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(524288);
    pub const VK_VIDEO_ENCODE_H265_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR: VkVideoEncodeH265StdFlagsKHR = VkVideoEncodeH265StdFlagsKHR(1048576);
}

pub type VkVideoEncodeH265RateControlFlagBitsKHR = VkVideoEncodeH265RateControlFlagsKHR; //
impl VkVideoEncodeH265RateControlFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR: VkVideoEncodeH265RateControlFlagsKHR = VkVideoEncodeH265RateControlFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_REGULAR_GOP_BIT_KHR: VkVideoEncodeH265RateControlFlagsKHR = VkVideoEncodeH265RateControlFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR: VkVideoEncodeH265RateControlFlagsKHR = VkVideoEncodeH265RateControlFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR: VkVideoEncodeH265RateControlFlagsKHR = VkVideoEncodeH265RateControlFlagsKHR(8);
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_TEMPORAL_SUB_LAYER_PATTERN_DYADIC_BIT_KHR: VkVideoEncodeH265RateControlFlagsKHR = VkVideoEncodeH265RateControlFlagsKHR(16);
}

pub type VkVideoEncodeH265CtbSizeFlagBitsKHR = VkVideoEncodeH265CtbSizeFlagsKHR; //
impl VkVideoEncodeH265CtbSizeFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_16_BIT_KHR: VkVideoEncodeH265CtbSizeFlagsKHR = VkVideoEncodeH265CtbSizeFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_32_BIT_KHR: VkVideoEncodeH265CtbSizeFlagsKHR = VkVideoEncodeH265CtbSizeFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_64_BIT_KHR: VkVideoEncodeH265CtbSizeFlagsKHR = VkVideoEncodeH265CtbSizeFlagsKHR(4);
}

pub type VkVideoEncodeH265TransformBlockSizeFlagBitsKHR = VkVideoEncodeH265TransformBlockSizeFlagsKHR; //
impl VkVideoEncodeH265TransformBlockSizeFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_4_BIT_KHR: VkVideoEncodeH265TransformBlockSizeFlagsKHR = VkVideoEncodeH265TransformBlockSizeFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_8_BIT_KHR: VkVideoEncodeH265TransformBlockSizeFlagsKHR = VkVideoEncodeH265TransformBlockSizeFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_16_BIT_KHR: VkVideoEncodeH265TransformBlockSizeFlagsKHR = VkVideoEncodeH265TransformBlockSizeFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_32_BIT_KHR: VkVideoEncodeH265TransformBlockSizeFlagsKHR = VkVideoEncodeH265TransformBlockSizeFlagsKHR(8);
}

pub type VkVideoEncodeAV1CapabilityFlagBitsKHR = VkVideoEncodeAV1CapabilityFlagsKHR; //
impl VkVideoEncodeAV1CapabilityFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_BIT_KHR: VkVideoEncodeAV1CapabilityFlagsKHR = VkVideoEncodeAV1CapabilityFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_GENERATE_OBU_EXTENSION_HEADER_BIT_KHR: VkVideoEncodeAV1CapabilityFlagsKHR = VkVideoEncodeAV1CapabilityFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_PRIMARY_REFERENCE_CDF_ONLY_BIT_KHR: VkVideoEncodeAV1CapabilityFlagsKHR = VkVideoEncodeAV1CapabilityFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_FRAME_SIZE_OVERRIDE_BIT_KHR: VkVideoEncodeAV1CapabilityFlagsKHR = VkVideoEncodeAV1CapabilityFlagsKHR(8);
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_MOTION_VECTOR_SCALING_BIT_KHR: VkVideoEncodeAV1CapabilityFlagsKHR = VkVideoEncodeAV1CapabilityFlagsKHR(16);
}

pub type VkVideoEncodeAV1StdFlagBitsKHR = VkVideoEncodeAV1StdFlagsKHR; //
impl VkVideoEncodeAV1StdFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_AV1_STD_UNIFORM_TILE_SPACING_FLAG_SET_BIT_KHR: VkVideoEncodeAV1StdFlagsKHR = VkVideoEncodeAV1StdFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_AV1_STD_SKIP_MODE_PRESENT_UNSET_BIT_KHR: VkVideoEncodeAV1StdFlagsKHR = VkVideoEncodeAV1StdFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_AV1_STD_PRIMARY_REF_FRAME_BIT_KHR: VkVideoEncodeAV1StdFlagsKHR = VkVideoEncodeAV1StdFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_AV1_STD_DELTA_Q_BIT_KHR: VkVideoEncodeAV1StdFlagsKHR = VkVideoEncodeAV1StdFlagsKHR(8);
}

pub type VkVideoEncodeAV1RateControlFlagBitsKHR = VkVideoEncodeAV1RateControlFlagsKHR; //
impl VkVideoEncodeAV1RateControlFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REGULAR_GOP_BIT_KHR: VkVideoEncodeAV1RateControlFlagsKHR = VkVideoEncodeAV1RateControlFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR: VkVideoEncodeAV1RateControlFlagsKHR = VkVideoEncodeAV1RateControlFlagsKHR(2);
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR: VkVideoEncodeAV1RateControlFlagsKHR = VkVideoEncodeAV1RateControlFlagsKHR(4);
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR: VkVideoEncodeAV1RateControlFlagsKHR = VkVideoEncodeAV1RateControlFlagsKHR(8);
}

pub type VkVideoEncodeAV1SuperblockSizeFlagBitsKHR = VkVideoEncodeAV1SuperblockSizeFlagsKHR; //
impl VkVideoEncodeAV1SuperblockSizeFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_64_BIT_KHR: VkVideoEncodeAV1SuperblockSizeFlagsKHR = VkVideoEncodeAV1SuperblockSizeFlagsKHR(1);
    pub const VK_VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_128_BIT_KHR: VkVideoEncodeAV1SuperblockSizeFlagsKHR = VkVideoEncodeAV1SuperblockSizeFlagsKHR(2);
}

pub struct VkVideoEncodeAV1PredictionModeKHR(i32); //
impl VkVideoEncodeAV1PredictionModeKHR {
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_INTRA_ONLY_KHR: Self = Self(0);
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_SINGLE_REFERENCE_KHR: Self = Self(1);
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_UNIDIRECTIONAL_COMPOUND_KHR: Self = Self(2);
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_BIDIRECTIONAL_COMPOUND_KHR: Self = Self(3);
}

pub struct VkVideoEncodeAV1RateControlGroupKHR(i32); //
impl VkVideoEncodeAV1RateControlGroupKHR {
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_INTRA_KHR: Self = Self(0);
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_PREDICTIVE_KHR: Self = Self(1);
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_BIPREDICTIVE_KHR: Self = Self(2);
}

pub type VkExportMetalObjectTypeFlagBitsEXT = VkExportMetalObjectTypeFlagsEXT; //
impl VkExportMetalObjectTypeFlagBitsEXT {
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_DEVICE_BIT_EXT: VkExportMetalObjectTypeFlagsEXT = VkExportMetalObjectTypeFlagsEXT(1);
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_COMMAND_QUEUE_BIT_EXT: VkExportMetalObjectTypeFlagsEXT = VkExportMetalObjectTypeFlagsEXT(2);
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_BUFFER_BIT_EXT: VkExportMetalObjectTypeFlagsEXT = VkExportMetalObjectTypeFlagsEXT(4);
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_TEXTURE_BIT_EXT: VkExportMetalObjectTypeFlagsEXT = VkExportMetalObjectTypeFlagsEXT(8);
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_IOSURFACE_BIT_EXT: VkExportMetalObjectTypeFlagsEXT = VkExportMetalObjectTypeFlagsEXT(16);
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_SHARED_EVENT_BIT_EXT: VkExportMetalObjectTypeFlagsEXT = VkExportMetalObjectTypeFlagsEXT(32);
}

pub type VkInstanceCreateFlagBits = VkInstanceCreateFlags; //
impl VkInstanceCreateFlagBits {
}

pub type VkImageCompressionFlagBitsEXT = VkImageCompressionFlagsEXT; //
impl VkImageCompressionFlagBitsEXT {
    pub const VK_IMAGE_COMPRESSION_DEFAULT_EXT: VkImageCompressionFlagsEXT = VkImageCompressionFlagsEXT(0);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_DEFAULT_EXT: VkImageCompressionFlagsEXT = VkImageCompressionFlagsEXT(1);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_EXPLICIT_EXT: VkImageCompressionFlagsEXT = VkImageCompressionFlagsEXT(2);
    pub const VK_IMAGE_COMPRESSION_DISABLED_EXT: VkImageCompressionFlagsEXT = VkImageCompressionFlagsEXT(4);
}

pub type VkImageCompressionFixedRateFlagBitsEXT = VkImageCompressionFixedRateFlagsEXT; //
impl VkImageCompressionFixedRateFlagBitsEXT {
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_NONE_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(0);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_1BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(1);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_2BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(2);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_3BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(4);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_4BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(8);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_5BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(16);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_6BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(32);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_7BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(64);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_8BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(128);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_9BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(256);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_10BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(512);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_11BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(1024);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_12BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(2048);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_13BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(4096);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_14BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(8192);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_15BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(16384);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_16BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(32768);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_17BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(65536);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_18BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(131072);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_19BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(262144);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_20BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(524288);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_21BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(1048576);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_22BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(2097152);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_23BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(4194304);
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_24BPC_BIT_EXT: VkImageCompressionFixedRateFlagsEXT = VkImageCompressionFixedRateFlagsEXT(8388608);
}

pub struct VkPipelineRobustnessBufferBehavior(i32); //
impl VkPipelineRobustnessBufferBehavior {
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DEVICE_DEFAULT: Self = Self(0);
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DISABLED: Self = Self(1);
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS: Self = Self(2);
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_2: Self = Self(3);
}

pub struct VkPipelineRobustnessImageBehavior(i32); //
impl VkPipelineRobustnessImageBehavior {
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DEVICE_DEFAULT: Self = Self(0);
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DISABLED: Self = Self(1);
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS: Self = Self(2);
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_2: Self = Self(3);
}

pub type VkOpticalFlowGridSizeFlagBitsNV = VkOpticalFlowGridSizeFlagsNV; //
impl VkOpticalFlowGridSizeFlagBitsNV {
    pub const VK_OPTICAL_FLOW_GRID_SIZE_UNKNOWN_NV: VkOpticalFlowGridSizeFlagsNV = VkOpticalFlowGridSizeFlagsNV(0);
    pub const VK_OPTICAL_FLOW_GRID_SIZE_1X1_BIT_NV: VkOpticalFlowGridSizeFlagsNV = VkOpticalFlowGridSizeFlagsNV(1);
    pub const VK_OPTICAL_FLOW_GRID_SIZE_2X2_BIT_NV: VkOpticalFlowGridSizeFlagsNV = VkOpticalFlowGridSizeFlagsNV(2);
    pub const VK_OPTICAL_FLOW_GRID_SIZE_4X4_BIT_NV: VkOpticalFlowGridSizeFlagsNV = VkOpticalFlowGridSizeFlagsNV(4);
    pub const VK_OPTICAL_FLOW_GRID_SIZE_8X8_BIT_NV: VkOpticalFlowGridSizeFlagsNV = VkOpticalFlowGridSizeFlagsNV(8);
}

pub type VkOpticalFlowUsageFlagBitsNV = VkOpticalFlowUsageFlagsNV; //
impl VkOpticalFlowUsageFlagBitsNV {
    pub const VK_OPTICAL_FLOW_USAGE_UNKNOWN_NV: VkOpticalFlowUsageFlagsNV = VkOpticalFlowUsageFlagsNV(0);
    pub const VK_OPTICAL_FLOW_USAGE_INPUT_BIT_NV: VkOpticalFlowUsageFlagsNV = VkOpticalFlowUsageFlagsNV(1);
    pub const VK_OPTICAL_FLOW_USAGE_OUTPUT_BIT_NV: VkOpticalFlowUsageFlagsNV = VkOpticalFlowUsageFlagsNV(2);
    pub const VK_OPTICAL_FLOW_USAGE_HINT_BIT_NV: VkOpticalFlowUsageFlagsNV = VkOpticalFlowUsageFlagsNV(4);
    pub const VK_OPTICAL_FLOW_USAGE_COST_BIT_NV: VkOpticalFlowUsageFlagsNV = VkOpticalFlowUsageFlagsNV(8);
    pub const VK_OPTICAL_FLOW_USAGE_GLOBAL_FLOW_BIT_NV: VkOpticalFlowUsageFlagsNV = VkOpticalFlowUsageFlagsNV(16);
}

pub struct VkOpticalFlowPerformanceLevelNV(i32); //
impl VkOpticalFlowPerformanceLevelNV {
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_UNKNOWN_NV: Self = Self(0);
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_SLOW_NV: Self = Self(1);
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_MEDIUM_NV: Self = Self(2);
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_FAST_NV: Self = Self(3);
}

pub struct VkOpticalFlowSessionBindingPointNV(i32); //
impl VkOpticalFlowSessionBindingPointNV {
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_UNKNOWN_NV: Self = Self(0);
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_INPUT_NV: Self = Self(1);
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_REFERENCE_NV: Self = Self(2);
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_HINT_NV: Self = Self(3);
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_FLOW_VECTOR_NV: Self = Self(4);
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_FLOW_VECTOR_NV: Self = Self(5);
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_COST_NV: Self = Self(6);
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_COST_NV: Self = Self(7);
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_GLOBAL_FLOW_NV: Self = Self(8);
}

pub type VkOpticalFlowSessionCreateFlagBitsNV = VkOpticalFlowSessionCreateFlagsNV; //
impl VkOpticalFlowSessionCreateFlagBitsNV {
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_HINT_BIT_NV: VkOpticalFlowSessionCreateFlagsNV = VkOpticalFlowSessionCreateFlagsNV(1);
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_COST_BIT_NV: VkOpticalFlowSessionCreateFlagsNV = VkOpticalFlowSessionCreateFlagsNV(2);
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_GLOBAL_FLOW_BIT_NV: VkOpticalFlowSessionCreateFlagsNV = VkOpticalFlowSessionCreateFlagsNV(4);
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_ALLOW_REGIONS_BIT_NV: VkOpticalFlowSessionCreateFlagsNV = VkOpticalFlowSessionCreateFlagsNV(8);
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_BOTH_DIRECTIONS_BIT_NV: VkOpticalFlowSessionCreateFlagsNV = VkOpticalFlowSessionCreateFlagsNV(16);
}

pub type VkOpticalFlowExecuteFlagBitsNV = VkOpticalFlowExecuteFlagsNV; //
impl VkOpticalFlowExecuteFlagBitsNV {
    pub const VK_OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_NV: VkOpticalFlowExecuteFlagsNV = VkOpticalFlowExecuteFlagsNV(1);
}

pub struct VkMicromapTypeEXT(i32); //
impl VkMicromapTypeEXT {
    pub const VK_MICROMAP_TYPE_OPACITY_MICROMAP_EXT: Self = Self(0);
}

pub type VkBuildMicromapFlagBitsEXT = VkBuildMicromapFlagsEXT; //
impl VkBuildMicromapFlagBitsEXT {
    pub const VK_BUILD_MICROMAP_PREFER_FAST_TRACE_BIT_EXT: VkBuildMicromapFlagsEXT = VkBuildMicromapFlagsEXT(1);
    pub const VK_BUILD_MICROMAP_PREFER_FAST_BUILD_BIT_EXT: VkBuildMicromapFlagsEXT = VkBuildMicromapFlagsEXT(2);
    pub const VK_BUILD_MICROMAP_ALLOW_COMPACTION_BIT_EXT: VkBuildMicromapFlagsEXT = VkBuildMicromapFlagsEXT(4);
}

pub type VkMicromapCreateFlagBitsEXT = VkMicromapCreateFlagsEXT; //
impl VkMicromapCreateFlagBitsEXT {
    pub const VK_MICROMAP_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT: VkMicromapCreateFlagsEXT = VkMicromapCreateFlagsEXT(1);
}

pub struct VkCopyMicromapModeEXT(i32); //
impl VkCopyMicromapModeEXT {
    pub const VK_COPY_MICROMAP_MODE_CLONE_EXT: Self = Self(0);
    pub const VK_COPY_MICROMAP_MODE_SERIALIZE_EXT: Self = Self(1);
    pub const VK_COPY_MICROMAP_MODE_DESERIALIZE_EXT: Self = Self(2);
    pub const VK_COPY_MICROMAP_MODE_COMPACT_EXT: Self = Self(3);
}

pub struct VkBuildMicromapModeEXT(i32); //
impl VkBuildMicromapModeEXT {
    pub const VK_BUILD_MICROMAP_MODE_BUILD_EXT: Self = Self(0);
}

pub struct VkOpacityMicromapFormatKHR(i32); //
impl VkOpacityMicromapFormatKHR {
    pub const VK_OPACITY_MICROMAP_FORMAT_2_STATE_KHR: Self = Self(1);
    pub const VK_OPACITY_MICROMAP_FORMAT_4_STATE_KHR: Self = Self(2);
}

pub struct VkOpacityMicromapSpecialIndexKHR(i32); //
impl VkOpacityMicromapSpecialIndexKHR {
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_TRANSPARENT_KHR: Self = Self(-1);
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_OPAQUE_KHR: Self = Self(-2);
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_TRANSPARENT_KHR: Self = Self(-3);
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_OPAQUE_KHR: Self = Self(-4);
}

pub struct VkAccelerationStructureSerializedBlockTypeKHR(i32); //
impl VkAccelerationStructureSerializedBlockTypeKHR {
    pub const VK_ACCELERATION_STRUCTURE_SERIALIZED_BLOCK_TYPE_OPACITY_MICROMAP_KHR: Self = Self(0);
}

pub struct VkDepthBiasRepresentationEXT(i32); //
impl VkDepthBiasRepresentationEXT {
    pub const VK_DEPTH_BIAS_REPRESENTATION_LEAST_REPRESENTABLE_VALUE_FORMAT_EXT: Self = Self(0);
    pub const VK_DEPTH_BIAS_REPRESENTATION_LEAST_REPRESENTABLE_VALUE_FORCE_UNORM_EXT: Self = Self(1);
    pub const VK_DEPTH_BIAS_REPRESENTATION_FLOAT_EXT: Self = Self(2);
}

pub type VkDeviceFaultFlagBitsKHR = VkDeviceFaultFlagsKHR; //
impl VkDeviceFaultFlagBitsKHR {
    pub const VK_DEVICE_FAULT_FLAG_DEVICE_LOST_KHR: VkDeviceFaultFlagsKHR = VkDeviceFaultFlagsKHR(1);
    pub const VK_DEVICE_FAULT_FLAG_MEMORY_ADDRESS_KHR: VkDeviceFaultFlagsKHR = VkDeviceFaultFlagsKHR(2);
    pub const VK_DEVICE_FAULT_FLAG_INSTRUCTION_ADDRESS_KHR: VkDeviceFaultFlagsKHR = VkDeviceFaultFlagsKHR(4);
    pub const VK_DEVICE_FAULT_FLAG_VENDOR_KHR: VkDeviceFaultFlagsKHR = VkDeviceFaultFlagsKHR(8);
    pub const VK_DEVICE_FAULT_FLAG_WATCHDOG_TIMEOUT_KHR: VkDeviceFaultFlagsKHR = VkDeviceFaultFlagsKHR(16);
    pub const VK_DEVICE_FAULT_FLAG_OVERFLOW_KHR: VkDeviceFaultFlagsKHR = VkDeviceFaultFlagsKHR(32);
}

pub struct VkDeviceFaultAddressTypeKHR(i32); //
impl VkDeviceFaultAddressTypeKHR {
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_NONE_KHR: Self = Self(0);
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_READ_INVALID_KHR: Self = Self(1);
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_WRITE_INVALID_KHR: Self = Self(2);
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_EXECUTE_INVALID_KHR: Self = Self(3);
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_UNKNOWN_KHR: Self = Self(4);
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_INVALID_KHR: Self = Self(5);
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_FAULT_KHR: Self = Self(6);
}

pub struct VkDeviceFaultVendorBinaryHeaderVersionKHR(i32); //
impl VkDeviceFaultVendorBinaryHeaderVersionKHR {
    pub const VK_DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_ONE_KHR: Self = Self(1);
    pub const VK_DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_ONE_EXT: Self = Self::VK_DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_ONE_KHR;
}

pub type VkIndirectCommandsLayoutUsageFlagBitsEXT = VkIndirectCommandsLayoutUsageFlagsEXT; //
impl VkIndirectCommandsLayoutUsageFlagBitsEXT {
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_EXT: VkIndirectCommandsLayoutUsageFlagsEXT = VkIndirectCommandsLayoutUsageFlagsEXT(1);
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_EXT: VkIndirectCommandsLayoutUsageFlagsEXT = VkIndirectCommandsLayoutUsageFlagsEXT(2);
}

pub struct VkIndirectExecutionSetInfoTypeEXT(i32); //
impl VkIndirectExecutionSetInfoTypeEXT {
    pub const VK_INDIRECT_EXECUTION_SET_INFO_TYPE_PIPELINES_EXT: Self = Self(0);
    pub const VK_INDIRECT_EXECUTION_SET_INFO_TYPE_SHADER_OBJECTS_EXT: Self = Self(1);
}

pub type VkIndirectCommandsInputModeFlagBitsEXT = VkIndirectCommandsInputModeFlagsEXT; //
impl VkIndirectCommandsInputModeFlagBitsEXT {
    pub const VK_INDIRECT_COMMANDS_INPUT_MODE_VULKAN_INDEX_BUFFER_EXT: VkIndirectCommandsInputModeFlagsEXT = VkIndirectCommandsInputModeFlagsEXT(1);
    pub const VK_INDIRECT_COMMANDS_INPUT_MODE_DXGI_INDEX_BUFFER_EXT: VkIndirectCommandsInputModeFlagsEXT = VkIndirectCommandsInputModeFlagsEXT(2);
}

pub struct VkIndirectCommandsTokenTypeEXT(i32); //
impl VkIndirectCommandsTokenTypeEXT {
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_EXECUTION_SET_EXT: Self = Self(0);
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_EXT: Self = Self(1);
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_SEQUENCE_INDEX_EXT: Self = Self(2);
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_EXT: Self = Self(3);
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_EXT: Self = Self(4);
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_EXT: Self = Self(5);
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_EXT: Self = Self(6);
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_COUNT_EXT: Self = Self(7);
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_COUNT_EXT: Self = Self(8);
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DISPATCH_EXT: Self = Self(9);
}

pub struct VkDisplacementMicromapFormatNV(i32); //
impl VkDisplacementMicromapFormatNV {
    pub const VK_DISPLACEMENT_MICROMAP_FORMAT_64_TRIANGLES_64_BYTES_NV: Self = Self(1);
    pub const VK_DISPLACEMENT_MICROMAP_FORMAT_256_TRIANGLES_128_BYTES_NV: Self = Self(2);
    pub const VK_DISPLACEMENT_MICROMAP_FORMAT_1024_TRIANGLES_128_BYTES_NV: Self = Self(3);
}

pub type VkShaderCreateFlagBitsEXT = VkShaderCreateFlagsEXT; //
impl VkShaderCreateFlagBitsEXT {
    pub const VK_SHADER_CREATE_LINK_STAGE_BIT_EXT: VkShaderCreateFlagsEXT = VkShaderCreateFlagsEXT(1);
}

pub struct VkShaderCodeTypeEXT(i32); //
impl VkShaderCodeTypeEXT {
    pub const VK_SHADER_CODE_TYPE_BINARY_EXT: Self = Self(0);
    pub const VK_SHADER_CODE_TYPE_SPIRV_EXT: Self = Self(1);
}

pub struct VkScopeKHR(i32); //
impl VkScopeKHR {
    pub const VK_SCOPE_DEVICE_KHR: Self = Self(1);
    pub const VK_SCOPE_WORKGROUP_KHR: Self = Self(2);
    pub const VK_SCOPE_SUBGROUP_KHR: Self = Self(3);
    pub const VK_SCOPE_QUEUE_FAMILY_KHR: Self = Self(5);
}

pub struct VkComponentTypeKHR(i32); //
impl VkComponentTypeKHR {
    pub const VK_COMPONENT_TYPE_FLOAT16_KHR: Self = Self(0);
    pub const VK_COMPONENT_TYPE_FLOAT32_KHR: Self = Self(1);
    pub const VK_COMPONENT_TYPE_FLOAT64_KHR: Self = Self(2);
    pub const VK_COMPONENT_TYPE_SINT8_KHR: Self = Self(3);
    pub const VK_COMPONENT_TYPE_SINT16_KHR: Self = Self(4);
    pub const VK_COMPONENT_TYPE_SINT32_KHR: Self = Self(5);
    pub const VK_COMPONENT_TYPE_SINT64_KHR: Self = Self(6);
    pub const VK_COMPONENT_TYPE_UINT8_KHR: Self = Self(7);
    pub const VK_COMPONENT_TYPE_UINT16_KHR: Self = Self(8);
    pub const VK_COMPONENT_TYPE_UINT32_KHR: Self = Self(9);
    pub const VK_COMPONENT_TYPE_UINT64_KHR: Self = Self(10);
}

pub struct VkCubicFilterWeightsQCOM(i32); //
impl VkCubicFilterWeightsQCOM {
    pub const VK_CUBIC_FILTER_WEIGHTS_CATMULL_ROM_QCOM: Self = Self(0);
    pub const VK_CUBIC_FILTER_WEIGHTS_ZERO_TANGENT_CARDINAL_QCOM: Self = Self(1);
    pub const VK_CUBIC_FILTER_WEIGHTS_B_SPLINE_QCOM: Self = Self(2);
    pub const VK_CUBIC_FILTER_WEIGHTS_MITCHELL_NETRAVALI_QCOM: Self = Self(3);
}

pub struct VkBlockMatchWindowCompareModeQCOM(i32); //
impl VkBlockMatchWindowCompareModeQCOM {
    pub const VK_BLOCK_MATCH_WINDOW_COMPARE_MODE_MIN_QCOM: Self = Self(0);
    pub const VK_BLOCK_MATCH_WINDOW_COMPARE_MODE_MAX_QCOM: Self = Self(1);
}

pub struct VkPhysicalDeviceLayeredApiKHR(i32); //
impl VkPhysicalDeviceLayeredApiKHR {
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_VULKAN_KHR: Self = Self(0);
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_D3D12_KHR: Self = Self(1);
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_METAL_KHR: Self = Self(2);
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_OPENGL_KHR: Self = Self(3);
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_OPENGLES_KHR: Self = Self(4);
}

pub struct VkLayeredDriverUnderlyingApiMSFT(i32); //
impl VkLayeredDriverUnderlyingApiMSFT {
    pub const VK_LAYERED_DRIVER_UNDERLYING_API_NONE_MSFT: Self = Self(0);
    pub const VK_LAYERED_DRIVER_UNDERLYING_API_D3D12_MSFT: Self = Self(1);
}

pub struct VkLatencyMarkerNV(i32); //
impl VkLatencyMarkerNV {
    pub const VK_LATENCY_MARKER_SIMULATION_START_NV: Self = Self(0);
    pub const VK_LATENCY_MARKER_SIMULATION_END_NV: Self = Self(1);
    pub const VK_LATENCY_MARKER_RENDERSUBMIT_START_NV: Self = Self(2);
    pub const VK_LATENCY_MARKER_RENDERSUBMIT_END_NV: Self = Self(3);
    pub const VK_LATENCY_MARKER_PRESENT_START_NV: Self = Self(4);
    pub const VK_LATENCY_MARKER_PRESENT_END_NV: Self = Self(5);
    pub const VK_LATENCY_MARKER_INPUT_SAMPLE_NV: Self = Self(6);
    pub const VK_LATENCY_MARKER_TRIGGER_FLASH_NV: Self = Self(7);
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_RENDERSUBMIT_START_NV: Self = Self(8);
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_RENDERSUBMIT_END_NV: Self = Self(9);
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_PRESENT_START_NV: Self = Self(10);
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_PRESENT_END_NV: Self = Self(11);
}

pub struct VkOutOfBandQueueTypeNV(i32); //
impl VkOutOfBandQueueTypeNV {
    pub const VK_OUT_OF_BAND_QUEUE_TYPE_RENDER_NV: Self = Self(0);
    pub const VK_OUT_OF_BAND_QUEUE_TYPE_PRESENT_NV: Self = Self(1);
}

pub type VkMemoryUnmapFlagBits = VkMemoryUnmapFlags; //
impl VkMemoryUnmapFlagBits {
}

pub struct VkCompressedTriangleFormatAMDX(i32); //
impl VkCompressedTriangleFormatAMDX {
    pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_AMDX: Self = Self(0);
}

pub type VkWaylandSurfaceCreateFlagBitsKHR = VkWaylandSurfaceCreateFlagsKHR; //
impl VkWaylandSurfaceCreateFlagBitsKHR {
}

pub struct VkDepthClampModeEXT(i32); //
impl VkDepthClampModeEXT {
    pub const VK_DEPTH_CLAMP_MODE_VIEWPORT_RANGE_EXT: Self = Self(0);
    pub const VK_DEPTH_CLAMP_MODE_USER_DEFINED_RANGE_EXT: Self = Self(1);
}

pub type VkAccessFlagBits3KHR = VkAccessFlags3KHR; //
impl VkAccessFlagBits3KHR {
    pub const VK_ACCESS_3_NONE_KHR: VkAccessFlags3KHR = VkAccessFlags3KHR(0);
}

pub type VkTileShadingRenderPassFlagBitsQCOM = VkTileShadingRenderPassFlagsQCOM; //
impl VkTileShadingRenderPassFlagBitsQCOM {
    pub const VK_TILE_SHADING_RENDER_PASS_ENABLE_BIT_QCOM: VkTileShadingRenderPassFlagsQCOM = VkTileShadingRenderPassFlagsQCOM(1);
    pub const VK_TILE_SHADING_RENDER_PASS_PER_TILE_EXECUTION_BIT_QCOM: VkTileShadingRenderPassFlagsQCOM = VkTileShadingRenderPassFlagsQCOM(2);
}

pub struct VkCooperativeVectorMatrixLayoutNV(i32); //
impl VkCooperativeVectorMatrixLayoutNV {
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_ROW_MAJOR_NV: Self = Self(0);
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_COLUMN_MAJOR_NV: Self = Self(1);
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_INFERENCING_OPTIMAL_NV: Self = Self(2);
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_TRAINING_OPTIMAL_NV: Self = Self(3);
}

pub type VkAddressCopyFlagBitsKHR = VkAddressCopyFlagsKHR; //
impl VkAddressCopyFlagBitsKHR {
    pub const VK_ADDRESS_COPY_DEVICE_LOCAL_BIT_KHR: VkAddressCopyFlagsKHR = VkAddressCopyFlagsKHR(1);
    pub const VK_ADDRESS_COPY_SPARSE_BIT_KHR: VkAddressCopyFlagsKHR = VkAddressCopyFlagsKHR(2);
    pub const VK_ADDRESS_COPY_PROTECTED_BIT_KHR: VkAddressCopyFlagsKHR = VkAddressCopyFlagsKHR(4);
}

pub type VkTensorCreateFlagBitsARM = VkTensorCreateFlagsARM; //
impl VkTensorCreateFlagBitsARM {
    pub const VK_TENSOR_CREATE_MUTABLE_FORMAT_BIT_ARM: VkTensorCreateFlagsARM = VkTensorCreateFlagsARM(1);
    pub const VK_TENSOR_CREATE_PROTECTED_BIT_ARM: VkTensorCreateFlagsARM = VkTensorCreateFlagsARM(2);
}

pub type VkTensorUsageFlagBitsARM = VkTensorUsageFlagsARM; //
impl VkTensorUsageFlagBitsARM {
    pub const VK_TENSOR_USAGE_SHADER_BIT_ARM: VkTensorUsageFlagsARM = VkTensorUsageFlagsARM(2);
    pub const VK_TENSOR_USAGE_TRANSFER_SRC_BIT_ARM: VkTensorUsageFlagsARM = VkTensorUsageFlagsARM(4);
    pub const VK_TENSOR_USAGE_TRANSFER_DST_BIT_ARM: VkTensorUsageFlagsARM = VkTensorUsageFlagsARM(8);
    pub const VK_TENSOR_USAGE_IMAGE_ALIASING_BIT_ARM: VkTensorUsageFlagsARM = VkTensorUsageFlagsARM(16);
}

pub struct VkTensorTilingARM(i32); //
impl VkTensorTilingARM {
    pub const VK_TENSOR_TILING_OPTIMAL_ARM: Self = Self(0);
    pub const VK_TENSOR_TILING_LINEAR_ARM: Self = Self(1);
}

pub type VkTensorViewCreateFlagBitsARM = VkTensorViewCreateFlagsARM; //
impl VkTensorViewCreateFlagBitsARM {
}

pub struct VkDefaultVertexAttributeValueKHR(i32); //
impl VkDefaultVertexAttributeValueKHR {
    pub const VK_DEFAULT_VERTEX_ATTRIBUTE_VALUE_ZERO_ZERO_ZERO_ZERO_KHR: Self = Self(0);
    pub const VK_DEFAULT_VERTEX_ATTRIBUTE_VALUE_ZERO_ZERO_ZERO_ONE_KHR: Self = Self(1);
}

pub type VkDataGraphPipelineSessionCreateFlagBitsARM = VkDataGraphPipelineSessionCreateFlagsARM; //
impl VkDataGraphPipelineSessionCreateFlagBitsARM {
    pub const VK_DATA_GRAPH_PIPELINE_SESSION_CREATE_PROTECTED_BIT_ARM: VkDataGraphPipelineSessionCreateFlagsARM = VkDataGraphPipelineSessionCreateFlagsARM(1);
}

pub struct VkDataGraphPipelineSessionBindPointARM(i32); //
impl VkDataGraphPipelineSessionBindPointARM {
    pub const VK_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_TRANSIENT_ARM: Self = Self(0);
}

pub struct VkDataGraphPipelineSessionBindPointTypeARM(i32); //
impl VkDataGraphPipelineSessionBindPointTypeARM {
    pub const VK_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_TYPE_MEMORY_ARM: Self = Self(0);
}

pub struct VkDataGraphPipelinePropertyARM(i32); //
impl VkDataGraphPipelinePropertyARM {
    pub const VK_DATA_GRAPH_PIPELINE_PROPERTY_CREATION_LOG_ARM: Self = Self(0);
    pub const VK_DATA_GRAPH_PIPELINE_PROPERTY_IDENTIFIER_ARM: Self = Self(1);
}

pub type VkDataGraphPipelineDispatchFlagBitsARM = VkDataGraphPipelineDispatchFlagsARM; //
impl VkDataGraphPipelineDispatchFlagBitsARM {
}

pub struct VkPhysicalDeviceDataGraphProcessingEngineTypeARM(i32); //
impl VkPhysicalDeviceDataGraphProcessingEngineTypeARM {
    pub const VK_PHYSICAL_DEVICE_DATA_GRAPH_PROCESSING_ENGINE_TYPE_DEFAULT_ARM: Self = Self(0);
}

pub struct VkPhysicalDeviceDataGraphOperationTypeARM(i32); //
impl VkPhysicalDeviceDataGraphOperationTypeARM {
    pub const VK_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_TYPE_SPIRV_EXTENDED_INSTRUCTION_SET_ARM: Self = Self(0);
}

pub struct VkDataGraphModelCacheTypeQCOM(i32); //
impl VkDataGraphModelCacheTypeQCOM {
    pub const VK_DATA_GRAPH_MODEL_CACHE_TYPE_GENERIC_BINARY_QCOM: Self = Self(0);
}

pub struct VkPerfHintTypeQCOM(i32); //
impl VkPerfHintTypeQCOM {
    pub const VK_PERF_HINT_TYPE_DEFAULT_QCOM: Self = Self(0);
    pub const VK_PERF_HINT_TYPE_FREQUENCY_MIN_QCOM: Self = Self(1);
    pub const VK_PERF_HINT_TYPE_FREQUENCY_MAX_QCOM: Self = Self(2);
    pub const VK_PERF_HINT_TYPE_FREQUENCY_SCALED_QCOM: Self = Self(3);
}

pub struct VkThrottleHintTypeSEC(i32); //
impl VkThrottleHintTypeSEC {
    pub const VK_THROTTLE_HINT_TYPE_DEFAULT_SEC: Self = Self(0);
    pub const VK_THROTTLE_HINT_TYPE_LOW_SEC: Self = Self(1);
    pub const VK_THROTTLE_HINT_TYPE_HIGH_SEC: Self = Self(2);
}

pub type VkVideoEncodeRgbModelConversionFlagBitsVALVE = VkVideoEncodeRgbModelConversionFlagsVALVE; //
impl VkVideoEncodeRgbModelConversionFlagBitsVALVE {
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_RGB_IDENTITY_BIT_VALVE: VkVideoEncodeRgbModelConversionFlagsVALVE = VkVideoEncodeRgbModelConversionFlagsVALVE(1);
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_IDENTITY_BIT_VALVE: VkVideoEncodeRgbModelConversionFlagsVALVE = VkVideoEncodeRgbModelConversionFlagsVALVE(2);
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_709_BIT_VALVE: VkVideoEncodeRgbModelConversionFlagsVALVE = VkVideoEncodeRgbModelConversionFlagsVALVE(4);
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_601_BIT_VALVE: VkVideoEncodeRgbModelConversionFlagsVALVE = VkVideoEncodeRgbModelConversionFlagsVALVE(8);
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_2020_BIT_VALVE: VkVideoEncodeRgbModelConversionFlagsVALVE = VkVideoEncodeRgbModelConversionFlagsVALVE(16);
}

pub type VkVideoEncodeRgbRangeCompressionFlagBitsVALVE = VkVideoEncodeRgbRangeCompressionFlagsVALVE; //
impl VkVideoEncodeRgbRangeCompressionFlagBitsVALVE {
    pub const VK_VIDEO_ENCODE_RGB_RANGE_COMPRESSION_FULL_RANGE_BIT_VALVE: VkVideoEncodeRgbRangeCompressionFlagsVALVE = VkVideoEncodeRgbRangeCompressionFlagsVALVE(1);
    pub const VK_VIDEO_ENCODE_RGB_RANGE_COMPRESSION_NARROW_RANGE_BIT_VALVE: VkVideoEncodeRgbRangeCompressionFlagsVALVE = VkVideoEncodeRgbRangeCompressionFlagsVALVE(2);
}

pub type VkVideoEncodeRgbChromaOffsetFlagBitsVALVE = VkVideoEncodeRgbChromaOffsetFlagsVALVE; //
impl VkVideoEncodeRgbChromaOffsetFlagBitsVALVE {
    pub const VK_VIDEO_ENCODE_RGB_CHROMA_OFFSET_COSITED_EVEN_BIT_VALVE: VkVideoEncodeRgbChromaOffsetFlagsVALVE = VkVideoEncodeRgbChromaOffsetFlagsVALVE(1);
    pub const VK_VIDEO_ENCODE_RGB_CHROMA_OFFSET_MIDPOINT_BIT_VALVE: VkVideoEncodeRgbChromaOffsetFlagsVALVE = VkVideoEncodeRgbChromaOffsetFlagsVALVE(2);
}

pub type VkSwapchainImageUsageFlagBitsOHOS = VkSwapchainImageUsageFlagsOHOS; //
impl VkSwapchainImageUsageFlagBitsOHOS {
    pub const VK_SWAPCHAIN_IMAGE_USAGE_SHARED_BIT_OHOS: VkSwapchainImageUsageFlagsOHOS = VkSwapchainImageUsageFlagsOHOS(1);
}

pub struct VkDescriptorMappingSourceEXT(i32); //
impl VkDescriptorMappingSourceEXT {
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_CONSTANT_OFFSET_EXT: Self = Self(0);
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_PUSH_INDEX_EXT: Self = Self(1);
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_INDIRECT_INDEX_EXT: Self = Self(2);
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_INDIRECT_INDEX_ARRAY_EXT: Self = Self(3);
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_RESOURCE_HEAP_DATA_EXT: Self = Self(4);
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_PUSH_DATA_EXT: Self = Self(5);
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_PUSH_ADDRESS_EXT: Self = Self(6);
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_INDIRECT_ADDRESS_EXT: Self = Self(7);
}

pub type VkSpirvResourceTypeFlagBitsEXT = VkSpirvResourceTypeFlagsEXT; //
impl VkSpirvResourceTypeFlagBitsEXT {
    pub const VK_SPIRV_RESOURCE_TYPE_ALL_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(0);
    pub const VK_SPIRV_RESOURCE_TYPE_SAMPLER_BIT_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(1);
    pub const VK_SPIRV_RESOURCE_TYPE_SAMPLED_IMAGE_BIT_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(2);
    pub const VK_SPIRV_RESOURCE_TYPE_READ_ONLY_IMAGE_BIT_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(4);
    pub const VK_SPIRV_RESOURCE_TYPE_READ_WRITE_IMAGE_BIT_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(8);
    pub const VK_SPIRV_RESOURCE_TYPE_COMBINED_SAMPLED_IMAGE_BIT_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(16);
    pub const VK_SPIRV_RESOURCE_TYPE_UNIFORM_BUFFER_BIT_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(32);
    pub const VK_SPIRV_RESOURCE_TYPE_READ_ONLY_STORAGE_BUFFER_BIT_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(64);
    pub const VK_SPIRV_RESOURCE_TYPE_READ_WRITE_STORAGE_BUFFER_BIT_EXT: VkSpirvResourceTypeFlagsEXT = VkSpirvResourceTypeFlagsEXT(128);
}

pub type VkGpaSqShaderStageFlagBitsAMD = VkGpaSqShaderStageFlagsAMD; //
impl VkGpaSqShaderStageFlagBitsAMD {
    pub const VK_GPA_SQ_SHADER_STAGE_PS_BIT_AMD: VkGpaSqShaderStageFlagsAMD = VkGpaSqShaderStageFlagsAMD(1);
    pub const VK_GPA_SQ_SHADER_STAGE_VS_BIT_AMD: VkGpaSqShaderStageFlagsAMD = VkGpaSqShaderStageFlagsAMD(2);
    pub const VK_GPA_SQ_SHADER_STAGE_GS_BIT_AMD: VkGpaSqShaderStageFlagsAMD = VkGpaSqShaderStageFlagsAMD(4);
    pub const VK_GPA_SQ_SHADER_STAGE_ES_BIT_AMD: VkGpaSqShaderStageFlagsAMD = VkGpaSqShaderStageFlagsAMD(8);
    pub const VK_GPA_SQ_SHADER_STAGE_HS_BIT_AMD: VkGpaSqShaderStageFlagsAMD = VkGpaSqShaderStageFlagsAMD(16);
    pub const VK_GPA_SQ_SHADER_STAGE_LS_BIT_AMD: VkGpaSqShaderStageFlagsAMD = VkGpaSqShaderStageFlagsAMD(32);
    pub const VK_GPA_SQ_SHADER_STAGE_CS_BIT_AMD: VkGpaSqShaderStageFlagsAMD = VkGpaSqShaderStageFlagsAMD(64);
}

pub struct VkGpaPerfBlockAMD(i32); //
impl VkGpaPerfBlockAMD {
    pub const VK_GPA_PERF_BLOCK_CPF_AMD: Self = Self(0);
    pub const VK_GPA_PERF_BLOCK_IA_AMD: Self = Self(1);
    pub const VK_GPA_PERF_BLOCK_VGT_AMD: Self = Self(2);
    pub const VK_GPA_PERF_BLOCK_PA_AMD: Self = Self(3);
    pub const VK_GPA_PERF_BLOCK_SC_AMD: Self = Self(4);
    pub const VK_GPA_PERF_BLOCK_SPI_AMD: Self = Self(5);
    pub const VK_GPA_PERF_BLOCK_SQ_AMD: Self = Self(6);
    pub const VK_GPA_PERF_BLOCK_SX_AMD: Self = Self(7);
    pub const VK_GPA_PERF_BLOCK_TA_AMD: Self = Self(8);
    pub const VK_GPA_PERF_BLOCK_TD_AMD: Self = Self(9);
    pub const VK_GPA_PERF_BLOCK_TCP_AMD: Self = Self(10);
    pub const VK_GPA_PERF_BLOCK_TCC_AMD: Self = Self(11);
    pub const VK_GPA_PERF_BLOCK_TCA_AMD: Self = Self(12);
    pub const VK_GPA_PERF_BLOCK_DB_AMD: Self = Self(13);
    pub const VK_GPA_PERF_BLOCK_CB_AMD: Self = Self(14);
    pub const VK_GPA_PERF_BLOCK_GDS_AMD: Self = Self(15);
    pub const VK_GPA_PERF_BLOCK_SRBM_AMD: Self = Self(16);
    pub const VK_GPA_PERF_BLOCK_GRBM_AMD: Self = Self(17);
    pub const VK_GPA_PERF_BLOCK_GRBM_SE_AMD: Self = Self(18);
    pub const VK_GPA_PERF_BLOCK_RLC_AMD: Self = Self(19);
    pub const VK_GPA_PERF_BLOCK_DMA_AMD: Self = Self(20);
    pub const VK_GPA_PERF_BLOCK_MC_AMD: Self = Self(21);
    pub const VK_GPA_PERF_BLOCK_CPG_AMD: Self = Self(22);
    pub const VK_GPA_PERF_BLOCK_CPC_AMD: Self = Self(23);
    pub const VK_GPA_PERF_BLOCK_WD_AMD: Self = Self(24);
    pub const VK_GPA_PERF_BLOCK_TCS_AMD: Self = Self(25);
    pub const VK_GPA_PERF_BLOCK_ATC_AMD: Self = Self(26);
    pub const VK_GPA_PERF_BLOCK_ATC_L2_AMD: Self = Self(27);
    pub const VK_GPA_PERF_BLOCK_MC_VM_L2_AMD: Self = Self(28);
    pub const VK_GPA_PERF_BLOCK_EA_AMD: Self = Self(29);
    pub const VK_GPA_PERF_BLOCK_RPB_AMD: Self = Self(30);
    pub const VK_GPA_PERF_BLOCK_RMI_AMD: Self = Self(31);
    pub const VK_GPA_PERF_BLOCK_UMCCH_AMD: Self = Self(32);
    pub const VK_GPA_PERF_BLOCK_GE_AMD: Self = Self(33);
    pub const VK_GPA_PERF_BLOCK_GL1A_AMD: Self = Self(34);
    pub const VK_GPA_PERF_BLOCK_GL1C_AMD: Self = Self(35);
    pub const VK_GPA_PERF_BLOCK_GL1CG_AMD: Self = Self(36);
    pub const VK_GPA_PERF_BLOCK_GL2A_AMD: Self = Self(37);
    pub const VK_GPA_PERF_BLOCK_GL2C_AMD: Self = Self(38);
    pub const VK_GPA_PERF_BLOCK_CHA_AMD: Self = Self(39);
    pub const VK_GPA_PERF_BLOCK_CHC_AMD: Self = Self(40);
    pub const VK_GPA_PERF_BLOCK_CHCG_AMD: Self = Self(41);
    pub const VK_GPA_PERF_BLOCK_GUS_AMD: Self = Self(42);
    pub const VK_GPA_PERF_BLOCK_GCR_AMD: Self = Self(43);
    pub const VK_GPA_PERF_BLOCK_PH_AMD: Self = Self(44);
    pub const VK_GPA_PERF_BLOCK_UTCL1_AMD: Self = Self(45);
    pub const VK_GPA_PERF_BLOCK_GE1_AMD: Self = Self::VK_GPA_PERF_BLOCK_GE_AMD;
    pub const VK_GPA_PERF_BLOCK_GE_DIST_AMD: Self = Self(46);
    pub const VK_GPA_PERF_BLOCK_GE_SE_AMD: Self = Self(47);
    pub const VK_GPA_PERF_BLOCK_DF_MALL_AMD: Self = Self(48);
    pub const VK_GPA_PERF_BLOCK_SQ_WGP_AMD: Self = Self(49);
    pub const VK_GPA_PERF_BLOCK_PC_AMD: Self = Self(50);
    pub const VK_GPA_PERF_BLOCK_GL1XA_AMD: Self = Self(51);
    pub const VK_GPA_PERF_BLOCK_GL1XC_AMD: Self = Self(52);
    pub const VK_GPA_PERF_BLOCK_WGS_AMD: Self = Self(53);
    pub const VK_GPA_PERF_BLOCK_EACPWD_AMD: Self = Self(54);
    pub const VK_GPA_PERF_BLOCK_EASE_AMD: Self = Self(55);
    pub const VK_GPA_PERF_BLOCK_RLCUSER_AMD: Self = Self(56);
    pub const VK_GPA_PERF_BLOCK_RLCLOCAL_AMD: Self = Self::VK_GPA_PERF_BLOCK_RLCUSER_AMD;
}

pub struct VkGpaSampleTypeAMD(i32); //
impl VkGpaSampleTypeAMD {
    pub const VK_GPA_SAMPLE_TYPE_CUMULATIVE_AMD: Self = Self(0);
    pub const VK_GPA_SAMPLE_TYPE_TRACE_AMD: Self = Self(1);
    pub const VK_GPA_SAMPLE_TYPE_TIMING_AMD: Self = Self(2);
}

pub struct VkGpaDeviceClockModeAMD(i32); //
impl VkGpaDeviceClockModeAMD {
    pub const VK_GPA_DEVICE_CLOCK_MODE_DEFAULT_AMD: Self = Self(0);
    pub const VK_GPA_DEVICE_CLOCK_MODE_QUERY_AMD: Self = Self(1);
    pub const VK_GPA_DEVICE_CLOCK_MODE_PROFILING_AMD: Self = Self(2);
    pub const VK_GPA_DEVICE_CLOCK_MODE_MIN_MEMORY_AMD: Self = Self(3);
    pub const VK_GPA_DEVICE_CLOCK_MODE_MIN_ENGINE_AMD: Self = Self(4);
    pub const VK_GPA_DEVICE_CLOCK_MODE_PEAK_AMD: Self = Self(5);
}

pub type VkAddressCommandFlagBitsKHR = VkAddressCommandFlagsKHR; //
impl VkAddressCommandFlagBitsKHR {
    pub const VK_ADDRESS_COMMAND_PROTECTED_BIT_KHR: VkAddressCommandFlagsKHR = VkAddressCommandFlagsKHR(1);
    pub const VK_ADDRESS_COMMAND_FULLY_BOUND_BIT_KHR: VkAddressCommandFlagsKHR = VkAddressCommandFlagsKHR(2);
    pub const VK_ADDRESS_COMMAND_STORAGE_BUFFER_USAGE_BIT_KHR: VkAddressCommandFlagsKHR = VkAddressCommandFlagsKHR(4);
    pub const VK_ADDRESS_COMMAND_UNKNOWN_STORAGE_BUFFER_USAGE_BIT_KHR: VkAddressCommandFlagsKHR = VkAddressCommandFlagsKHR(8);
}

pub type VkDataGraphTOSAQualityFlagBitsARM = VkDataGraphTOSAQualityFlagsARM; //
impl VkDataGraphTOSAQualityFlagBitsARM {
    pub const VK_DATA_GRAPH_TOSA_QUALITY_ACCELERATED_ARM: VkDataGraphTOSAQualityFlagsARM = VkDataGraphTOSAQualityFlagsARM(1);
    pub const VK_DATA_GRAPH_TOSA_QUALITY_CONFORMANT_ARM: VkDataGraphTOSAQualityFlagsARM = VkDataGraphTOSAQualityFlagsARM(2);
    pub const VK_DATA_GRAPH_TOSA_QUALITY_EXPERIMENTAL_ARM: VkDataGraphTOSAQualityFlagsARM = VkDataGraphTOSAQualityFlagsARM(4);
    pub const VK_DATA_GRAPH_TOSA_QUALITY_DEPRECATED_ARM: VkDataGraphTOSAQualityFlagsARM = VkDataGraphTOSAQualityFlagsARM(8);
}

pub struct VkDataGraphTOSALevelARM(i32); //
impl VkDataGraphTOSALevelARM {
    pub const VK_DATA_GRAPH_TOSA_LEVEL_NONE_ARM: Self = Self(0);
    pub const VK_DATA_GRAPH_TOSA_LEVEL_8K_ARM: Self = Self(1);
}

pub type VkDataGraphOpticalFlowGridSizeFlagBitsARM = VkDataGraphOpticalFlowGridSizeFlagsARM; //
impl VkDataGraphOpticalFlowGridSizeFlagBitsARM {
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_UNKNOWN_ARM: VkDataGraphOpticalFlowGridSizeFlagsARM = VkDataGraphOpticalFlowGridSizeFlagsARM(0);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_1X1_BIT_ARM: VkDataGraphOpticalFlowGridSizeFlagsARM = VkDataGraphOpticalFlowGridSizeFlagsARM(1);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_2X2_BIT_ARM: VkDataGraphOpticalFlowGridSizeFlagsARM = VkDataGraphOpticalFlowGridSizeFlagsARM(2);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_4X4_BIT_ARM: VkDataGraphOpticalFlowGridSizeFlagsARM = VkDataGraphOpticalFlowGridSizeFlagsARM(4);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_8X8_BIT_ARM: VkDataGraphOpticalFlowGridSizeFlagsARM = VkDataGraphOpticalFlowGridSizeFlagsARM(8);
}

pub type VkDataGraphOpticalFlowImageUsageFlagBitsARM = VkDataGraphOpticalFlowImageUsageFlagsARM; //
impl VkDataGraphOpticalFlowImageUsageFlagBitsARM {
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_UNKNOWN_ARM: VkDataGraphOpticalFlowImageUsageFlagsARM = VkDataGraphOpticalFlowImageUsageFlagsARM(0);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_INPUT_BIT_ARM: VkDataGraphOpticalFlowImageUsageFlagsARM = VkDataGraphOpticalFlowImageUsageFlagsARM(1);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_OUTPUT_BIT_ARM: VkDataGraphOpticalFlowImageUsageFlagsARM = VkDataGraphOpticalFlowImageUsageFlagsARM(2);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_HINT_BIT_ARM: VkDataGraphOpticalFlowImageUsageFlagsARM = VkDataGraphOpticalFlowImageUsageFlagsARM(4);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_COST_BIT_ARM: VkDataGraphOpticalFlowImageUsageFlagsARM = VkDataGraphOpticalFlowImageUsageFlagsARM(8);
}

pub struct VkDataGraphOpticalFlowPerformanceLevelARM(i32); //
impl VkDataGraphOpticalFlowPerformanceLevelARM {
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_UNKNOWN_ARM: Self = Self(0);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_SLOW_ARM: Self = Self(1);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_MEDIUM_ARM: Self = Self(2);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_FAST_ARM: Self = Self(3);
}

pub struct VkDataGraphPipelineNodeConnectionTypeARM(i32); //
impl VkDataGraphPipelineNodeConnectionTypeARM {
}

pub struct VkDataGraphPipelineNodeTypeARM(i32); //
impl VkDataGraphPipelineNodeTypeARM {
}

pub type VkDataGraphOpticalFlowCreateFlagBitsARM = VkDataGraphOpticalFlowCreateFlagsARM; //
impl VkDataGraphOpticalFlowCreateFlagBitsARM {
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_ENABLE_HINT_BIT_ARM: VkDataGraphOpticalFlowCreateFlagsARM = VkDataGraphOpticalFlowCreateFlagsARM(1);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_ENABLE_COST_BIT_ARM: VkDataGraphOpticalFlowCreateFlagsARM = VkDataGraphOpticalFlowCreateFlagsARM(2);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_RESERVED_30_BIT_ARM: VkDataGraphOpticalFlowCreateFlagsARM = VkDataGraphOpticalFlowCreateFlagsARM(1073741824);
}

pub type VkDataGraphOpticalFlowExecuteFlagBitsARM = VkDataGraphOpticalFlowExecuteFlagsARM; //
impl VkDataGraphOpticalFlowExecuteFlagBitsARM {
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_ARM: VkDataGraphOpticalFlowExecuteFlagsARM = VkDataGraphOpticalFlowExecuteFlagsARM(1);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_INPUT_UNCHANGED_BIT_ARM: VkDataGraphOpticalFlowExecuteFlagsARM = VkDataGraphOpticalFlowExecuteFlagsARM(2);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_REFERENCE_UNCHANGED_BIT_ARM: VkDataGraphOpticalFlowExecuteFlagsARM = VkDataGraphOpticalFlowExecuteFlagsARM(4);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_INPUT_IS_PREVIOUS_REFERENCE_BIT_ARM: VkDataGraphOpticalFlowExecuteFlagsARM = VkDataGraphOpticalFlowExecuteFlagsARM(8);
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_REFERENCE_IS_PREVIOUS_INPUT_BIT_ARM: VkDataGraphOpticalFlowExecuteFlagsARM = VkDataGraphOpticalFlowExecuteFlagsARM(16);
}

pub struct VkNeuralAcceleratorStatisticsModeARM(i32); //
impl VkNeuralAcceleratorStatisticsModeARM {
    pub const VK_NEURAL_ACCELERATOR_STATISTICS_MODE_DISABLED_ARM: Self = Self(0);
    pub const VK_NEURAL_ACCELERATOR_STATISTICS_MODE_STATISTICS0_ARM: Self = Self(1);
    pub const VK_NEURAL_ACCELERATOR_STATISTICS_MODE_STATISTICS1_ARM: Self = Self(2);
}

pub struct VkImageTilingControlEXT(i32); //
impl VkImageTilingControlEXT {
    pub const VK_IMAGE_TILING_CONTROL_DEFAULT_EXT: Self = Self(0);
    pub const VK_IMAGE_TILING_CONTROL_MIN_SIZE_EXT: Self = Self(1);
    pub const VK_IMAGE_TILING_CONTROL_MAX_PERFORMANCE_EXT: Self = Self(2);
}



























