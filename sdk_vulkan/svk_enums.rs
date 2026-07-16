// SPDX-License-Identifier: None
// Copyright (c) 2026 None
pub type VkImageLayout = i32;
pub mod VkImageLayoutValue {
    use crate::VkImageLayout;

    pub const VK_IMAGE_LAYOUT_UNDEFINED : VkImageLayout = 0;
    pub const VK_IMAGE_LAYOUT_GENERAL : VkImageLayout = 1;
    pub const VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL : VkImageLayout = 2;
    pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL : VkImageLayout = 3;
    pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL : VkImageLayout = 4;
    pub const VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL : VkImageLayout = 5;
    pub const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL : VkImageLayout = 6;
    pub const VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL : VkImageLayout = 7;
    pub const VK_IMAGE_LAYOUT_PREINITIALIZED : VkImageLayout = 8;
}

pub type VkAttachmentLoadOp = i32;
pub mod VkAttachmentLoadOpValue {
    use crate::VkAttachmentLoadOp;

    pub const VK_ATTACHMENT_LOAD_OP_LOAD : VkAttachmentLoadOp = 0;
    pub const VK_ATTACHMENT_LOAD_OP_CLEAR : VkAttachmentLoadOp = 1;
    pub const VK_ATTACHMENT_LOAD_OP_DONT_CARE : VkAttachmentLoadOp = 2;
}

pub type VkAttachmentStoreOp = i32;
pub mod VkAttachmentStoreOpValue {
    use crate::VkAttachmentStoreOp;

    pub const VK_ATTACHMENT_STORE_OP_STORE : VkAttachmentStoreOp = 0;
    pub const VK_ATTACHMENT_STORE_OP_DONT_CARE : VkAttachmentStoreOp = 1;
}

pub type VkImageType = i32;
pub mod VkImageTypeValue {
    use crate::VkImageType;

    pub const VK_IMAGE_TYPE_1D : VkImageType = 0;
    pub const VK_IMAGE_TYPE_2D : VkImageType = 1;
    pub const VK_IMAGE_TYPE_3D : VkImageType = 2;
}

pub type VkImageTiling = i32;
pub mod VkImageTilingValue {
    use crate::VkImageTiling;

    pub const VK_IMAGE_TILING_OPTIMAL : VkImageTiling = 0;
    pub const VK_IMAGE_TILING_LINEAR : VkImageTiling = 1;
}

pub type VkImageViewType = i32;
pub mod VkImageViewTypeValue {
    use crate::VkImageViewType;

    pub const VK_IMAGE_VIEW_TYPE_1D : VkImageViewType = 0;
    pub const VK_IMAGE_VIEW_TYPE_2D : VkImageViewType = 1;
    pub const VK_IMAGE_VIEW_TYPE_3D : VkImageViewType = 2;
    pub const VK_IMAGE_VIEW_TYPE_CUBE : VkImageViewType = 3;
    pub const VK_IMAGE_VIEW_TYPE_1D_ARRAY : VkImageViewType = 4;
    pub const VK_IMAGE_VIEW_TYPE_2D_ARRAY : VkImageViewType = 5;
    pub const VK_IMAGE_VIEW_TYPE_CUBE_ARRAY : VkImageViewType = 6;
}

pub type VkCommandBufferLevel = i32;
pub mod VkCommandBufferLevelValue {
    use crate::VkCommandBufferLevel;

    pub const VK_COMMAND_BUFFER_LEVEL_PRIMARY : VkCommandBufferLevel = 0;
    pub const VK_COMMAND_BUFFER_LEVEL_SECONDARY : VkCommandBufferLevel = 1;
}

pub type VkComponentSwizzle = i32;
pub mod VkComponentSwizzleValue {
    use crate::VkComponentSwizzle;

    pub const VK_COMPONENT_SWIZZLE_IDENTITY : VkComponentSwizzle = 0;
    pub const VK_COMPONENT_SWIZZLE_ZERO : VkComponentSwizzle = 1;
    pub const VK_COMPONENT_SWIZZLE_ONE : VkComponentSwizzle = 2;
    pub const VK_COMPONENT_SWIZZLE_R : VkComponentSwizzle = 3;
    pub const VK_COMPONENT_SWIZZLE_G : VkComponentSwizzle = 4;
    pub const VK_COMPONENT_SWIZZLE_B : VkComponentSwizzle = 5;
    pub const VK_COMPONENT_SWIZZLE_A : VkComponentSwizzle = 6;
}

pub type VkDescriptorType = i32;
pub mod VkDescriptorTypeValue {
    use crate::VkDescriptorType;

    pub const VK_DESCRIPTOR_TYPE_SAMPLER : VkDescriptorType = 0;
    pub const VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER : VkDescriptorType = 1;
    pub const VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE : VkDescriptorType = 2;
    pub const VK_DESCRIPTOR_TYPE_STORAGE_IMAGE : VkDescriptorType = 3;
    pub const VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER : VkDescriptorType = 4;
    pub const VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER : VkDescriptorType = 5;
    pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER : VkDescriptorType = 6;
    pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER : VkDescriptorType = 7;
    pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC : VkDescriptorType = 8;
    pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC : VkDescriptorType = 9;
    pub const VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT : VkDescriptorType = 10;
}

pub type VkQueryType = i32;
pub mod VkQueryTypeValue {
    use crate::VkQueryType;

    pub const VK_QUERY_TYPE_OCCLUSION : VkQueryType = 0;
    pub const VK_QUERY_TYPE_PIPELINE_STATISTICS : VkQueryType = 1;
    pub const VK_QUERY_TYPE_TIMESTAMP : VkQueryType = 2;
}

pub type VkBorderColor = i32;
pub mod VkBorderColorValue {
    use crate::VkBorderColor;

    pub const VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK : VkBorderColor = 0;
    pub const VK_BORDER_COLOR_INT_TRANSPARENT_BLACK : VkBorderColor = 1;
    pub const VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK : VkBorderColor = 2;
    pub const VK_BORDER_COLOR_INT_OPAQUE_BLACK : VkBorderColor = 3;
    pub const VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE : VkBorderColor = 4;
    pub const VK_BORDER_COLOR_INT_OPAQUE_WHITE : VkBorderColor = 5;
}

pub type VkPipelineBindPoint = i32;
pub mod VkPipelineBindPointValue {
    use crate::VkPipelineBindPoint;

    pub const VK_PIPELINE_BIND_POINT_GRAPHICS : VkPipelineBindPoint = 0;
    pub const VK_PIPELINE_BIND_POINT_COMPUTE : VkPipelineBindPoint = 1;
}

pub type VkPipelineCacheHeaderVersion = i32;
pub mod VkPipelineCacheHeaderVersionValue {
    use crate::VkPipelineCacheHeaderVersion;

    pub const VK_PIPELINE_CACHE_HEADER_VERSION_ONE : VkPipelineCacheHeaderVersion = 1;
}

pub type VkPipelineCacheCreateFlagBits = u32;
pub mod VkPipelineCacheCreateFlagBitsValue {
    use crate::VkPipelineCacheCreateFlagBits;

}

pub type VkPrimitiveTopology = i32;
pub mod VkPrimitiveTopologyValue {
    use crate::VkPrimitiveTopology;

    pub const VK_PRIMITIVE_TOPOLOGY_POINT_LIST : VkPrimitiveTopology = 0;
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST : VkPrimitiveTopology = 1;
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP : VkPrimitiveTopology = 2;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST : VkPrimitiveTopology = 3;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP : VkPrimitiveTopology = 4;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN : VkPrimitiveTopology = 5;
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY : VkPrimitiveTopology = 6;
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY : VkPrimitiveTopology = 7;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY : VkPrimitiveTopology = 8;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY : VkPrimitiveTopology = 9;
    pub const VK_PRIMITIVE_TOPOLOGY_PATCH_LIST : VkPrimitiveTopology = 10;
}

pub type VkSharingMode = i32;
pub mod VkSharingModeValue {
    use crate::VkSharingMode;

    pub const VK_SHARING_MODE_EXCLUSIVE : VkSharingMode = 0;
    pub const VK_SHARING_MODE_CONCURRENT : VkSharingMode = 1;
}

pub type VkIndexType = i32;
pub mod VkIndexTypeValue {
    use crate::VkIndexType;

    pub const VK_INDEX_TYPE_UINT16 : VkIndexType = 0;
    pub const VK_INDEX_TYPE_UINT32 : VkIndexType = 1;
}

pub type VkFilter = i32;
pub mod VkFilterValue {
    use crate::VkFilter;

    pub const VK_FILTER_NEAREST : VkFilter = 0;
    pub const VK_FILTER_LINEAR : VkFilter = 1;
}

pub type VkSamplerMipmapMode = i32;
pub mod VkSamplerMipmapModeValue {
    use crate::VkSamplerMipmapMode;

    pub const VK_SAMPLER_MIPMAP_MODE_NEAREST : VkSamplerMipmapMode = 0;
    pub const VK_SAMPLER_MIPMAP_MODE_LINEAR : VkSamplerMipmapMode = 1;
}

pub type VkSamplerAddressMode = i32;
pub mod VkSamplerAddressModeValue {
    use crate::VkSamplerAddressMode;

    pub const VK_SAMPLER_ADDRESS_MODE_REPEAT : VkSamplerAddressMode = 0;
    pub const VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT : VkSamplerAddressMode = 1;
    pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE : VkSamplerAddressMode = 2;
    pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER : VkSamplerAddressMode = 3;
}

pub type VkCompareOp = i32;
pub mod VkCompareOpValue {
    use crate::VkCompareOp;

    pub const VK_COMPARE_OP_NEVER : VkCompareOp = 0;
    pub const VK_COMPARE_OP_LESS : VkCompareOp = 1;
    pub const VK_COMPARE_OP_EQUAL : VkCompareOp = 2;
    pub const VK_COMPARE_OP_LESS_OR_EQUAL : VkCompareOp = 3;
    pub const VK_COMPARE_OP_GREATER : VkCompareOp = 4;
    pub const VK_COMPARE_OP_NOT_EQUAL : VkCompareOp = 5;
    pub const VK_COMPARE_OP_GREATER_OR_EQUAL : VkCompareOp = 6;
    pub const VK_COMPARE_OP_ALWAYS : VkCompareOp = 7;
}

pub type VkPolygonMode = i32;
pub mod VkPolygonModeValue {
    use crate::VkPolygonMode;

    pub const VK_POLYGON_MODE_FILL : VkPolygonMode = 0;
    pub const VK_POLYGON_MODE_LINE : VkPolygonMode = 1;
    pub const VK_POLYGON_MODE_POINT : VkPolygonMode = 2;
}

pub type VkFrontFace = i32;
pub mod VkFrontFaceValue {
    use crate::VkFrontFace;

    pub const VK_FRONT_FACE_COUNTER_CLOCKWISE : VkFrontFace = 0;
    pub const VK_FRONT_FACE_CLOCKWISE : VkFrontFace = 1;
}

pub type VkBlendFactor = i32;
pub mod VkBlendFactorValue {
    use crate::VkBlendFactor;

    pub const VK_BLEND_FACTOR_ZERO : VkBlendFactor = 0;
    pub const VK_BLEND_FACTOR_ONE : VkBlendFactor = 1;
    pub const VK_BLEND_FACTOR_SRC_COLOR : VkBlendFactor = 2;
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR : VkBlendFactor = 3;
    pub const VK_BLEND_FACTOR_DST_COLOR : VkBlendFactor = 4;
    pub const VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR : VkBlendFactor = 5;
    pub const VK_BLEND_FACTOR_SRC_ALPHA : VkBlendFactor = 6;
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA : VkBlendFactor = 7;
    pub const VK_BLEND_FACTOR_DST_ALPHA : VkBlendFactor = 8;
    pub const VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA : VkBlendFactor = 9;
    pub const VK_BLEND_FACTOR_CONSTANT_COLOR : VkBlendFactor = 10;
    pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR : VkBlendFactor = 11;
    pub const VK_BLEND_FACTOR_CONSTANT_ALPHA : VkBlendFactor = 12;
    pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA : VkBlendFactor = 13;
    pub const VK_BLEND_FACTOR_SRC_ALPHA_SATURATE : VkBlendFactor = 14;
    pub const VK_BLEND_FACTOR_SRC1_COLOR : VkBlendFactor = 15;
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR : VkBlendFactor = 16;
    pub const VK_BLEND_FACTOR_SRC1_ALPHA : VkBlendFactor = 17;
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA : VkBlendFactor = 18;
}

pub type VkBlendOp = i32;
pub mod VkBlendOpValue {
    use crate::VkBlendOp;

    pub const VK_BLEND_OP_ADD : VkBlendOp = 0;
    pub const VK_BLEND_OP_SUBTRACT : VkBlendOp = 1;
    pub const VK_BLEND_OP_REVERSE_SUBTRACT : VkBlendOp = 2;
    pub const VK_BLEND_OP_MIN : VkBlendOp = 3;
    pub const VK_BLEND_OP_MAX : VkBlendOp = 4;
}

pub type VkStencilOp = i32;
pub mod VkStencilOpValue {
    use crate::VkStencilOp;

    pub const VK_STENCIL_OP_KEEP : VkStencilOp = 0;
    pub const VK_STENCIL_OP_ZERO : VkStencilOp = 1;
    pub const VK_STENCIL_OP_REPLACE : VkStencilOp = 2;
    pub const VK_STENCIL_OP_INCREMENT_AND_CLAMP : VkStencilOp = 3;
    pub const VK_STENCIL_OP_DECREMENT_AND_CLAMP : VkStencilOp = 4;
    pub const VK_STENCIL_OP_INVERT : VkStencilOp = 5;
    pub const VK_STENCIL_OP_INCREMENT_AND_WRAP : VkStencilOp = 6;
    pub const VK_STENCIL_OP_DECREMENT_AND_WRAP : VkStencilOp = 7;
}

pub type VkLogicOp = i32;
pub mod VkLogicOpValue {
    use crate::VkLogicOp;

    pub const VK_LOGIC_OP_CLEAR : VkLogicOp = 0;
    pub const VK_LOGIC_OP_AND : VkLogicOp = 1;
    pub const VK_LOGIC_OP_AND_REVERSE : VkLogicOp = 2;
    pub const VK_LOGIC_OP_COPY : VkLogicOp = 3;
    pub const VK_LOGIC_OP_AND_INVERTED : VkLogicOp = 4;
    pub const VK_LOGIC_OP_NO_OP : VkLogicOp = 5;
    pub const VK_LOGIC_OP_XOR : VkLogicOp = 6;
    pub const VK_LOGIC_OP_OR : VkLogicOp = 7;
    pub const VK_LOGIC_OP_NOR : VkLogicOp = 8;
    pub const VK_LOGIC_OP_EQUIVALENT : VkLogicOp = 9;
    pub const VK_LOGIC_OP_INVERT : VkLogicOp = 10;
    pub const VK_LOGIC_OP_OR_REVERSE : VkLogicOp = 11;
    pub const VK_LOGIC_OP_COPY_INVERTED : VkLogicOp = 12;
    pub const VK_LOGIC_OP_OR_INVERTED : VkLogicOp = 13;
    pub const VK_LOGIC_OP_NAND : VkLogicOp = 14;
    pub const VK_LOGIC_OP_SET : VkLogicOp = 15;
}

pub type VkInternalAllocationType = i32;
pub mod VkInternalAllocationTypeValue {
    use crate::VkInternalAllocationType;

    pub const VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE : VkInternalAllocationType = 0;
}

pub type VkSystemAllocationScope = i32;
pub mod VkSystemAllocationScopeValue {
    use crate::VkSystemAllocationScope;

    pub const VK_SYSTEM_ALLOCATION_SCOPE_COMMAND : VkSystemAllocationScope = 0;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_OBJECT : VkSystemAllocationScope = 1;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_CACHE : VkSystemAllocationScope = 2;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_DEVICE : VkSystemAllocationScope = 3;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE : VkSystemAllocationScope = 4;
}

pub type VkPhysicalDeviceType = i32;
pub mod VkPhysicalDeviceTypeValue {
    use crate::VkPhysicalDeviceType;

    pub const VK_PHYSICAL_DEVICE_TYPE_OTHER : VkPhysicalDeviceType = 0;
    pub const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU : VkPhysicalDeviceType = 1;
    pub const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU : VkPhysicalDeviceType = 2;
    pub const VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU : VkPhysicalDeviceType = 3;
    pub const VK_PHYSICAL_DEVICE_TYPE_CPU : VkPhysicalDeviceType = 4;
}

pub type VkVertexInputRate = i32;
pub mod VkVertexInputRateValue {
    use crate::VkVertexInputRate;

    pub const VK_VERTEX_INPUT_RATE_VERTEX : VkVertexInputRate = 0;
    pub const VK_VERTEX_INPUT_RATE_INSTANCE : VkVertexInputRate = 1;
}

pub type VkFormat = i32;
pub mod VkFormatValue {
    use crate::VkFormat;

    pub const VK_FORMAT_UNDEFINED : VkFormat = 0;
    pub const VK_FORMAT_R4G4_UNORM_PACK8 : VkFormat = 1;
    pub const VK_FORMAT_R4G4B4A4_UNORM_PACK16 : VkFormat = 2;
    pub const VK_FORMAT_B4G4R4A4_UNORM_PACK16 : VkFormat = 3;
    pub const VK_FORMAT_R5G6B5_UNORM_PACK16 : VkFormat = 4;
    pub const VK_FORMAT_B5G6R5_UNORM_PACK16 : VkFormat = 5;
    pub const VK_FORMAT_R5G5B5A1_UNORM_PACK16 : VkFormat = 6;
    pub const VK_FORMAT_B5G5R5A1_UNORM_PACK16 : VkFormat = 7;
    pub const VK_FORMAT_A1R5G5B5_UNORM_PACK16 : VkFormat = 8;
    pub const VK_FORMAT_R8_UNORM : VkFormat = 9;
    pub const VK_FORMAT_R8_SNORM : VkFormat = 10;
    pub const VK_FORMAT_R8_USCALED : VkFormat = 11;
    pub const VK_FORMAT_R8_SSCALED : VkFormat = 12;
    pub const VK_FORMAT_R8_UINT : VkFormat = 13;
    pub const VK_FORMAT_R8_SINT : VkFormat = 14;
    pub const VK_FORMAT_R8_SRGB : VkFormat = 15;
    pub const VK_FORMAT_R8G8_UNORM : VkFormat = 16;
    pub const VK_FORMAT_R8G8_SNORM : VkFormat = 17;
    pub const VK_FORMAT_R8G8_USCALED : VkFormat = 18;
    pub const VK_FORMAT_R8G8_SSCALED : VkFormat = 19;
    pub const VK_FORMAT_R8G8_UINT : VkFormat = 20;
    pub const VK_FORMAT_R8G8_SINT : VkFormat = 21;
    pub const VK_FORMAT_R8G8_SRGB : VkFormat = 22;
    pub const VK_FORMAT_R8G8B8_UNORM : VkFormat = 23;
    pub const VK_FORMAT_R8G8B8_SNORM : VkFormat = 24;
    pub const VK_FORMAT_R8G8B8_USCALED : VkFormat = 25;
    pub const VK_FORMAT_R8G8B8_SSCALED : VkFormat = 26;
    pub const VK_FORMAT_R8G8B8_UINT : VkFormat = 27;
    pub const VK_FORMAT_R8G8B8_SINT : VkFormat = 28;
    pub const VK_FORMAT_R8G8B8_SRGB : VkFormat = 29;
    pub const VK_FORMAT_B8G8R8_UNORM : VkFormat = 30;
    pub const VK_FORMAT_B8G8R8_SNORM : VkFormat = 31;
    pub const VK_FORMAT_B8G8R8_USCALED : VkFormat = 32;
    pub const VK_FORMAT_B8G8R8_SSCALED : VkFormat = 33;
    pub const VK_FORMAT_B8G8R8_UINT : VkFormat = 34;
    pub const VK_FORMAT_B8G8R8_SINT : VkFormat = 35;
    pub const VK_FORMAT_B8G8R8_SRGB : VkFormat = 36;
    pub const VK_FORMAT_R8G8B8A8_UNORM : VkFormat = 37;
    pub const VK_FORMAT_R8G8B8A8_SNORM : VkFormat = 38;
    pub const VK_FORMAT_R8G8B8A8_USCALED : VkFormat = 39;
    pub const VK_FORMAT_R8G8B8A8_SSCALED : VkFormat = 40;
    pub const VK_FORMAT_R8G8B8A8_UINT : VkFormat = 41;
    pub const VK_FORMAT_R8G8B8A8_SINT : VkFormat = 42;
    pub const VK_FORMAT_R8G8B8A8_SRGB : VkFormat = 43;
    pub const VK_FORMAT_B8G8R8A8_UNORM : VkFormat = 44;
    pub const VK_FORMAT_B8G8R8A8_SNORM : VkFormat = 45;
    pub const VK_FORMAT_B8G8R8A8_USCALED : VkFormat = 46;
    pub const VK_FORMAT_B8G8R8A8_SSCALED : VkFormat = 47;
    pub const VK_FORMAT_B8G8R8A8_UINT : VkFormat = 48;
    pub const VK_FORMAT_B8G8R8A8_SINT : VkFormat = 49;
    pub const VK_FORMAT_B8G8R8A8_SRGB : VkFormat = 50;
    pub const VK_FORMAT_A8B8G8R8_UNORM_PACK32 : VkFormat = 51;
    pub const VK_FORMAT_A8B8G8R8_SNORM_PACK32 : VkFormat = 52;
    pub const VK_FORMAT_A8B8G8R8_USCALED_PACK32 : VkFormat = 53;
    pub const VK_FORMAT_A8B8G8R8_SSCALED_PACK32 : VkFormat = 54;
    pub const VK_FORMAT_A8B8G8R8_UINT_PACK32 : VkFormat = 55;
    pub const VK_FORMAT_A8B8G8R8_SINT_PACK32 : VkFormat = 56;
    pub const VK_FORMAT_A8B8G8R8_SRGB_PACK32 : VkFormat = 57;
    pub const VK_FORMAT_A2R10G10B10_UNORM_PACK32 : VkFormat = 58;
    pub const VK_FORMAT_A2R10G10B10_SNORM_PACK32 : VkFormat = 59;
    pub const VK_FORMAT_A2R10G10B10_USCALED_PACK32 : VkFormat = 60;
    pub const VK_FORMAT_A2R10G10B10_SSCALED_PACK32 : VkFormat = 61;
    pub const VK_FORMAT_A2R10G10B10_UINT_PACK32 : VkFormat = 62;
    pub const VK_FORMAT_A2R10G10B10_SINT_PACK32 : VkFormat = 63;
    pub const VK_FORMAT_A2B10G10R10_UNORM_PACK32 : VkFormat = 64;
    pub const VK_FORMAT_A2B10G10R10_SNORM_PACK32 : VkFormat = 65;
    pub const VK_FORMAT_A2B10G10R10_USCALED_PACK32 : VkFormat = 66;
    pub const VK_FORMAT_A2B10G10R10_SSCALED_PACK32 : VkFormat = 67;
    pub const VK_FORMAT_A2B10G10R10_UINT_PACK32 : VkFormat = 68;
    pub const VK_FORMAT_A2B10G10R10_SINT_PACK32 : VkFormat = 69;
    pub const VK_FORMAT_R16_UNORM : VkFormat = 70;
    pub const VK_FORMAT_R16_SNORM : VkFormat = 71;
    pub const VK_FORMAT_R16_USCALED : VkFormat = 72;
    pub const VK_FORMAT_R16_SSCALED : VkFormat = 73;
    pub const VK_FORMAT_R16_UINT : VkFormat = 74;
    pub const VK_FORMAT_R16_SINT : VkFormat = 75;
    pub const VK_FORMAT_R16_SFLOAT : VkFormat = 76;
    pub const VK_FORMAT_R16G16_UNORM : VkFormat = 77;
    pub const VK_FORMAT_R16G16_SNORM : VkFormat = 78;
    pub const VK_FORMAT_R16G16_USCALED : VkFormat = 79;
    pub const VK_FORMAT_R16G16_SSCALED : VkFormat = 80;
    pub const VK_FORMAT_R16G16_UINT : VkFormat = 81;
    pub const VK_FORMAT_R16G16_SINT : VkFormat = 82;
    pub const VK_FORMAT_R16G16_SFLOAT : VkFormat = 83;
    pub const VK_FORMAT_R16G16B16_UNORM : VkFormat = 84;
    pub const VK_FORMAT_R16G16B16_SNORM : VkFormat = 85;
    pub const VK_FORMAT_R16G16B16_USCALED : VkFormat = 86;
    pub const VK_FORMAT_R16G16B16_SSCALED : VkFormat = 87;
    pub const VK_FORMAT_R16G16B16_UINT : VkFormat = 88;
    pub const VK_FORMAT_R16G16B16_SINT : VkFormat = 89;
    pub const VK_FORMAT_R16G16B16_SFLOAT : VkFormat = 90;
    pub const VK_FORMAT_R16G16B16A16_UNORM : VkFormat = 91;
    pub const VK_FORMAT_R16G16B16A16_SNORM : VkFormat = 92;
    pub const VK_FORMAT_R16G16B16A16_USCALED : VkFormat = 93;
    pub const VK_FORMAT_R16G16B16A16_SSCALED : VkFormat = 94;
    pub const VK_FORMAT_R16G16B16A16_UINT : VkFormat = 95;
    pub const VK_FORMAT_R16G16B16A16_SINT : VkFormat = 96;
    pub const VK_FORMAT_R16G16B16A16_SFLOAT : VkFormat = 97;
    pub const VK_FORMAT_R32_UINT : VkFormat = 98;
    pub const VK_FORMAT_R32_SINT : VkFormat = 99;
    pub const VK_FORMAT_R32_SFLOAT : VkFormat = 100;
    pub const VK_FORMAT_R32G32_UINT : VkFormat = 101;
    pub const VK_FORMAT_R32G32_SINT : VkFormat = 102;
    pub const VK_FORMAT_R32G32_SFLOAT : VkFormat = 103;
    pub const VK_FORMAT_R32G32B32_UINT : VkFormat = 104;
    pub const VK_FORMAT_R32G32B32_SINT : VkFormat = 105;
    pub const VK_FORMAT_R32G32B32_SFLOAT : VkFormat = 106;
    pub const VK_FORMAT_R32G32B32A32_UINT : VkFormat = 107;
    pub const VK_FORMAT_R32G32B32A32_SINT : VkFormat = 108;
    pub const VK_FORMAT_R32G32B32A32_SFLOAT : VkFormat = 109;
    pub const VK_FORMAT_R64_UINT : VkFormat = 110;
    pub const VK_FORMAT_R64_SINT : VkFormat = 111;
    pub const VK_FORMAT_R64_SFLOAT : VkFormat = 112;
    pub const VK_FORMAT_R64G64_UINT : VkFormat = 113;
    pub const VK_FORMAT_R64G64_SINT : VkFormat = 114;
    pub const VK_FORMAT_R64G64_SFLOAT : VkFormat = 115;
    pub const VK_FORMAT_R64G64B64_UINT : VkFormat = 116;
    pub const VK_FORMAT_R64G64B64_SINT : VkFormat = 117;
    pub const VK_FORMAT_R64G64B64_SFLOAT : VkFormat = 118;
    pub const VK_FORMAT_R64G64B64A64_UINT : VkFormat = 119;
    pub const VK_FORMAT_R64G64B64A64_SINT : VkFormat = 120;
    pub const VK_FORMAT_R64G64B64A64_SFLOAT : VkFormat = 121;
    pub const VK_FORMAT_B10G11R11_UFLOAT_PACK32 : VkFormat = 122;
    pub const VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 : VkFormat = 123;
    pub const VK_FORMAT_D16_UNORM : VkFormat = 124;
    pub const VK_FORMAT_X8_D24_UNORM_PACK32 : VkFormat = 125;
    pub const VK_FORMAT_D32_SFLOAT : VkFormat = 126;
    pub const VK_FORMAT_S8_UINT : VkFormat = 127;
    pub const VK_FORMAT_D16_UNORM_S8_UINT : VkFormat = 128;
    pub const VK_FORMAT_D24_UNORM_S8_UINT : VkFormat = 129;
    pub const VK_FORMAT_D32_SFLOAT_S8_UINT : VkFormat = 130;
    pub const VK_FORMAT_BC1_RGB_UNORM_BLOCK : VkFormat = 131;
    pub const VK_FORMAT_BC1_RGB_SRGB_BLOCK : VkFormat = 132;
    pub const VK_FORMAT_BC1_RGBA_UNORM_BLOCK : VkFormat = 133;
    pub const VK_FORMAT_BC1_RGBA_SRGB_BLOCK : VkFormat = 134;
    pub const VK_FORMAT_BC2_UNORM_BLOCK : VkFormat = 135;
    pub const VK_FORMAT_BC2_SRGB_BLOCK : VkFormat = 136;
    pub const VK_FORMAT_BC3_UNORM_BLOCK : VkFormat = 137;
    pub const VK_FORMAT_BC3_SRGB_BLOCK : VkFormat = 138;
    pub const VK_FORMAT_BC4_UNORM_BLOCK : VkFormat = 139;
    pub const VK_FORMAT_BC4_SNORM_BLOCK : VkFormat = 140;
    pub const VK_FORMAT_BC5_UNORM_BLOCK : VkFormat = 141;
    pub const VK_FORMAT_BC5_SNORM_BLOCK : VkFormat = 142;
    pub const VK_FORMAT_BC6H_UFLOAT_BLOCK : VkFormat = 143;
    pub const VK_FORMAT_BC6H_SFLOAT_BLOCK : VkFormat = 144;
    pub const VK_FORMAT_BC7_UNORM_BLOCK : VkFormat = 145;
    pub const VK_FORMAT_BC7_SRGB_BLOCK : VkFormat = 146;
    pub const VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK : VkFormat = 147;
    pub const VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK : VkFormat = 148;
    pub const VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK : VkFormat = 149;
    pub const VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK : VkFormat = 150;
    pub const VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK : VkFormat = 151;
    pub const VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK : VkFormat = 152;
    pub const VK_FORMAT_EAC_R11_UNORM_BLOCK : VkFormat = 153;
    pub const VK_FORMAT_EAC_R11_SNORM_BLOCK : VkFormat = 154;
    pub const VK_FORMAT_EAC_R11G11_UNORM_BLOCK : VkFormat = 155;
    pub const VK_FORMAT_EAC_R11G11_SNORM_BLOCK : VkFormat = 156;
    pub const VK_FORMAT_ASTC_4x4_UNORM_BLOCK : VkFormat = 157;
    pub const VK_FORMAT_ASTC_4x4_SRGB_BLOCK : VkFormat = 158;
    pub const VK_FORMAT_ASTC_5x4_UNORM_BLOCK : VkFormat = 159;
    pub const VK_FORMAT_ASTC_5x4_SRGB_BLOCK : VkFormat = 160;
    pub const VK_FORMAT_ASTC_5x5_UNORM_BLOCK : VkFormat = 161;
    pub const VK_FORMAT_ASTC_5x5_SRGB_BLOCK : VkFormat = 162;
    pub const VK_FORMAT_ASTC_6x5_UNORM_BLOCK : VkFormat = 163;
    pub const VK_FORMAT_ASTC_6x5_SRGB_BLOCK : VkFormat = 164;
    pub const VK_FORMAT_ASTC_6x6_UNORM_BLOCK : VkFormat = 165;
    pub const VK_FORMAT_ASTC_6x6_SRGB_BLOCK : VkFormat = 166;
    pub const VK_FORMAT_ASTC_8x5_UNORM_BLOCK : VkFormat = 167;
    pub const VK_FORMAT_ASTC_8x5_SRGB_BLOCK : VkFormat = 168;
    pub const VK_FORMAT_ASTC_8x6_UNORM_BLOCK : VkFormat = 169;
    pub const VK_FORMAT_ASTC_8x6_SRGB_BLOCK : VkFormat = 170;
    pub const VK_FORMAT_ASTC_8x8_UNORM_BLOCK : VkFormat = 171;
    pub const VK_FORMAT_ASTC_8x8_SRGB_BLOCK : VkFormat = 172;
    pub const VK_FORMAT_ASTC_10x5_UNORM_BLOCK : VkFormat = 173;
    pub const VK_FORMAT_ASTC_10x5_SRGB_BLOCK : VkFormat = 174;
    pub const VK_FORMAT_ASTC_10x6_UNORM_BLOCK : VkFormat = 175;
    pub const VK_FORMAT_ASTC_10x6_SRGB_BLOCK : VkFormat = 176;
    pub const VK_FORMAT_ASTC_10x8_UNORM_BLOCK : VkFormat = 177;
    pub const VK_FORMAT_ASTC_10x8_SRGB_BLOCK : VkFormat = 178;
    pub const VK_FORMAT_ASTC_10x10_UNORM_BLOCK : VkFormat = 179;
    pub const VK_FORMAT_ASTC_10x10_SRGB_BLOCK : VkFormat = 180;
    pub const VK_FORMAT_ASTC_12x10_UNORM_BLOCK : VkFormat = 181;
    pub const VK_FORMAT_ASTC_12x10_SRGB_BLOCK : VkFormat = 182;
    pub const VK_FORMAT_ASTC_12x12_UNORM_BLOCK : VkFormat = 183;
    pub const VK_FORMAT_ASTC_12x12_SRGB_BLOCK : VkFormat = 184;
}

pub type VkStructureType = i32;
pub mod VkStructureTypeValue {
    use crate::VkStructureType;

    pub const VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT : VkStructureType = 1000128004;

    pub const VK_STRUCTURE_TYPE_APPLICATION_INFO : VkStructureType = 0;
    pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO : VkStructureType = 1;
    pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO : VkStructureType = 2;
    pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO : VkStructureType = 3;
    pub const VK_STRUCTURE_TYPE_SUBMIT_INFO : VkStructureType = 4;
    pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO : VkStructureType = 5;
    pub const VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE : VkStructureType = 6;
    pub const VK_STRUCTURE_TYPE_BIND_SPARSE_INFO : VkStructureType = 7;
    pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO : VkStructureType = 8;
    pub const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO : VkStructureType = 9;
    pub const VK_STRUCTURE_TYPE_EVENT_CREATE_INFO : VkStructureType = 10;
    pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO : VkStructureType = 11;
    pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO : VkStructureType = 12;
    pub const VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO : VkStructureType = 13;
    pub const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO : VkStructureType = 14;
    pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO : VkStructureType = 15;
    pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO : VkStructureType = 16;
    pub const VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO : VkStructureType = 17;
    pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO : VkStructureType = 18;
    pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO : VkStructureType = 19;
    pub const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO : VkStructureType = 20;
    pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO : VkStructureType = 21;
    pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO : VkStructureType = 22;
    pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO : VkStructureType = 23;
    pub const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO : VkStructureType = 24;
    pub const VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO : VkStructureType = 25;
    pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO : VkStructureType = 26;
    pub const VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO : VkStructureType = 27;
    pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO : VkStructureType = 28;
    pub const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO : VkStructureType = 29;
    pub const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO : VkStructureType = 30;
    pub const VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO : VkStructureType = 31;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO : VkStructureType = 32;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO : VkStructureType = 33;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO : VkStructureType = 34;
    pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET : VkStructureType = 35;
    pub const VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET : VkStructureType = 36;
    pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO : VkStructureType = 37;
    pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO : VkStructureType = 38;
    pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO : VkStructureType = 39;
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO : VkStructureType = 40;
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO : VkStructureType = 41;
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO : VkStructureType = 42;
    pub const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO : VkStructureType = 43;
    pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER : VkStructureType = 44;
    pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER : VkStructureType = 45;
    pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER : VkStructureType = 46;
    pub const VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO : VkStructureType = 47;
    pub const VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO : VkStructureType = 48;
}

pub type VkSubpassContents = i32;
pub mod VkSubpassContentsValue {
    use crate::VkSubpassContents;

    pub const VK_SUBPASS_CONTENTS_INLINE : VkSubpassContents = 0;
    pub const VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS : VkSubpassContents = 1;
}

pub type VkResult = i32;
pub mod VkResultValue {
    use crate::VkResult;

    pub const VK_SUCCESS : VkResult = 0;
    pub const VK_NOT_READY : VkResult = 1;
    pub const VK_TIMEOUT : VkResult = 2;
    pub const VK_EVENT_SET : VkResult = 3;
    pub const VK_EVENT_RESET : VkResult = 4;
    pub const VK_INCOMPLETE : VkResult = 5;
    pub const VK_ERROR_OUT_OF_HOST_MEMORY : VkResult = -1;
    pub const VK_ERROR_OUT_OF_DEVICE_MEMORY : VkResult = -2;
    pub const VK_ERROR_INITIALIZATION_FAILED : VkResult = -3;
    pub const VK_ERROR_DEVICE_LOST : VkResult = -4;
    pub const VK_ERROR_MEMORY_MAP_FAILED : VkResult = -5;
    pub const VK_ERROR_LAYER_NOT_PRESENT : VkResult = -6;
    pub const VK_ERROR_EXTENSION_NOT_PRESENT : VkResult = -7;
    pub const VK_ERROR_FEATURE_NOT_PRESENT : VkResult = -8;
    pub const VK_ERROR_INCOMPATIBLE_DRIVER : VkResult = -9;
    pub const VK_ERROR_TOO_MANY_OBJECTS : VkResult = -10;
    pub const VK_ERROR_FORMAT_NOT_SUPPORTED : VkResult = -11;
    pub const VK_ERROR_FRAGMENTED_POOL : VkResult = -12;
    pub const VK_ERROR_UNKNOWN : VkResult = -13;
}

pub type VkDynamicState = i32;
pub mod VkDynamicStateValue {
    use crate::VkDynamicState;

    pub const VK_DYNAMIC_STATE_VIEWPORT : VkDynamicState = 0;
    pub const VK_DYNAMIC_STATE_SCISSOR : VkDynamicState = 1;
    pub const VK_DYNAMIC_STATE_LINE_WIDTH : VkDynamicState = 2;
    pub const VK_DYNAMIC_STATE_DEPTH_BIAS : VkDynamicState = 3;
    pub const VK_DYNAMIC_STATE_BLEND_CONSTANTS : VkDynamicState = 4;
    pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS : VkDynamicState = 5;
    pub const VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK : VkDynamicState = 6;
    pub const VK_DYNAMIC_STATE_STENCIL_WRITE_MASK : VkDynamicState = 7;
    pub const VK_DYNAMIC_STATE_STENCIL_REFERENCE : VkDynamicState = 8;
}

pub type VkDescriptorUpdateTemplateType = i32;
pub mod VkDescriptorUpdateTemplateTypeValue {
    use crate::VkDescriptorUpdateTemplateType;

    pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET : VkDescriptorUpdateTemplateType = 0;
}

pub type VkObjectType = i32;
pub mod VkObjectTypeValue {
    use crate::VkObjectType;

    pub const VK_OBJECT_TYPE_UNKNOWN : VkObjectType = 0;
    pub const VK_OBJECT_TYPE_INSTANCE : VkObjectType = 1;
    pub const VK_OBJECT_TYPE_PHYSICAL_DEVICE : VkObjectType = 2;
    pub const VK_OBJECT_TYPE_DEVICE : VkObjectType = 3;
    pub const VK_OBJECT_TYPE_QUEUE : VkObjectType = 4;
    pub const VK_OBJECT_TYPE_SEMAPHORE : VkObjectType = 5;
    pub const VK_OBJECT_TYPE_COMMAND_BUFFER : VkObjectType = 6;
    pub const VK_OBJECT_TYPE_FENCE : VkObjectType = 7;
    pub const VK_OBJECT_TYPE_DEVICE_MEMORY : VkObjectType = 8;
    pub const VK_OBJECT_TYPE_BUFFER : VkObjectType = 9;
    pub const VK_OBJECT_TYPE_IMAGE : VkObjectType = 10;
    pub const VK_OBJECT_TYPE_EVENT : VkObjectType = 11;
    pub const VK_OBJECT_TYPE_QUERY_POOL : VkObjectType = 12;
    pub const VK_OBJECT_TYPE_BUFFER_VIEW : VkObjectType = 13;
    pub const VK_OBJECT_TYPE_IMAGE_VIEW : VkObjectType = 14;
    pub const VK_OBJECT_TYPE_SHADER_MODULE : VkObjectType = 15;
    pub const VK_OBJECT_TYPE_PIPELINE_CACHE : VkObjectType = 16;
    pub const VK_OBJECT_TYPE_PIPELINE_LAYOUT : VkObjectType = 17;
    pub const VK_OBJECT_TYPE_RENDER_PASS : VkObjectType = 18;
    pub const VK_OBJECT_TYPE_PIPELINE : VkObjectType = 19;
    pub const VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT : VkObjectType = 20;
    pub const VK_OBJECT_TYPE_SAMPLER : VkObjectType = 21;
    pub const VK_OBJECT_TYPE_DESCRIPTOR_POOL : VkObjectType = 22;
    pub const VK_OBJECT_TYPE_DESCRIPTOR_SET : VkObjectType = 23;
    pub const VK_OBJECT_TYPE_FRAMEBUFFER : VkObjectType = 24;
    pub const VK_OBJECT_TYPE_COMMAND_POOL : VkObjectType = 25;
}

pub type VkRayTracingInvocationReorderModeEXT = i32;
pub mod VkRayTracingInvocationReorderModeEXTValue {
    use crate::VkRayTracingInvocationReorderModeEXT;

    pub const VK_RAY_TRACING_INVOCATION_REORDER_MODE_NONE_EXT : VkRayTracingInvocationReorderModeEXT = 0;
    pub const VK_RAY_TRACING_INVOCATION_REORDER_MODE_REORDER_EXT : VkRayTracingInvocationReorderModeEXT = 1;
}

pub type VkRayTracingLssIndexingModeNV = i32;
pub mod VkRayTracingLssIndexingModeNVValue {
    use crate::VkRayTracingLssIndexingModeNV;

    pub const VK_RAY_TRACING_LSS_INDEXING_MODE_LIST_NV : VkRayTracingLssIndexingModeNV = 0;
    pub const VK_RAY_TRACING_LSS_INDEXING_MODE_SUCCESSIVE_NV : VkRayTracingLssIndexingModeNV = 1;
}

pub type VkRayTracingLssPrimitiveEndCapsModeNV = i32;
pub mod VkRayTracingLssPrimitiveEndCapsModeNVValue {
    use crate::VkRayTracingLssPrimitiveEndCapsModeNV;

    pub const VK_RAY_TRACING_LSS_PRIMITIVE_END_CAPS_MODE_NONE_NV : VkRayTracingLssPrimitiveEndCapsModeNV = 0;
    pub const VK_RAY_TRACING_LSS_PRIMITIVE_END_CAPS_MODE_CHAINED_NV : VkRayTracingLssPrimitiveEndCapsModeNV = 1;
}

pub type VkDirectDriverLoadingModeLUNARG = i32;
pub mod VkDirectDriverLoadingModeLUNARGValue {
    use crate::VkDirectDriverLoadingModeLUNARG;

    pub const VK_DIRECT_DRIVER_LOADING_MODE_EXCLUSIVE_LUNARG : VkDirectDriverLoadingModeLUNARG = 0;
    pub const VK_DIRECT_DRIVER_LOADING_MODE_INCLUSIVE_LUNARG : VkDirectDriverLoadingModeLUNARG = 1;
}

pub type VkAntiLagModeAMD = i32;
pub mod VkAntiLagModeAMDValue {
    use crate::VkAntiLagModeAMD;

    pub const VK_ANTI_LAG_MODE_DRIVER_CONTROL_AMD : VkAntiLagModeAMD = 0;
    pub const VK_ANTI_LAG_MODE_ON_AMD : VkAntiLagModeAMD = 1;
    pub const VK_ANTI_LAG_MODE_OFF_AMD : VkAntiLagModeAMD = 2;
}

pub type VkAntiLagStageAMD = i32;
pub mod VkAntiLagStageAMDValue {
    use crate::VkAntiLagStageAMD;

    pub const VK_ANTI_LAG_STAGE_INPUT_AMD : VkAntiLagStageAMD = 0;
    pub const VK_ANTI_LAG_STAGE_PRESENT_AMD : VkAntiLagStageAMD = 1;
}

pub type VkQueueFlagBits = u32;
pub mod VkQueueFlagBitsValue {
    use crate::VkQueueFlagBits;

    pub const VK_QUEUE_GRAPHICS_BIT : VkQueueFlagBits = 1;
    pub const VK_QUEUE_COMPUTE_BIT : VkQueueFlagBits = 2;
    pub const VK_QUEUE_TRANSFER_BIT : VkQueueFlagBits = 4;
    pub const VK_QUEUE_SPARSE_BINDING_BIT : VkQueueFlagBits = 8;
}

pub type VkCullModeFlagBits = u32;
pub mod VkCullModeFlagBitsValue {
    use crate::VkCullModeFlagBits;

    pub const VK_CULL_MODE_NONE : VkCullModeFlagBits = 0;
    pub const VK_CULL_MODE_FRONT_BIT : VkCullModeFlagBits = 1;
    pub const VK_CULL_MODE_BACK_BIT : VkCullModeFlagBits = 2;
    pub const VK_CULL_MODE_FRONT_AND_BACK : VkCullModeFlagBits = 0x00000003;
}

pub type VkRenderPassCreateFlagBits = u32;
pub mod VkRenderPassCreateFlagBitsValue {
    use crate::VkRenderPassCreateFlagBits;

}

pub type VkDeviceQueueCreateFlagBits = u32;
pub mod VkDeviceQueueCreateFlagBitsValue {
    use crate::VkDeviceQueueCreateFlagBits;

}

pub type VkMemoryPropertyFlagBits = u32;
pub mod VkMemoryPropertyFlagBitsValue {
    use crate::VkMemoryPropertyFlagBits;

    pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT : VkMemoryPropertyFlagBits = 1;
    pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT : VkMemoryPropertyFlagBits = 2;
    pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT : VkMemoryPropertyFlagBits = 4;
    pub const VK_MEMORY_PROPERTY_HOST_CACHED_BIT : VkMemoryPropertyFlagBits = 8;
    pub const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT : VkMemoryPropertyFlagBits = 16;
}

pub type VkMemoryHeapFlagBits = u32;
pub mod VkMemoryHeapFlagBitsValue {
    use crate::VkMemoryHeapFlagBits;

    pub const VK_MEMORY_HEAP_DEVICE_LOCAL_BIT : VkMemoryHeapFlagBits = 1;
}

pub type VkAccessFlagBits = u32;
pub mod VkAccessFlagBitsValue {
    use crate::VkAccessFlagBits;

    pub const VK_ACCESS_INDIRECT_COMMAND_READ_BIT : VkAccessFlagBits = 1;
    pub const VK_ACCESS_INDEX_READ_BIT : VkAccessFlagBits = 2;
    pub const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT : VkAccessFlagBits = 4;
    pub const VK_ACCESS_UNIFORM_READ_BIT : VkAccessFlagBits = 8;
    pub const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT : VkAccessFlagBits = 16;
    pub const VK_ACCESS_SHADER_READ_BIT : VkAccessFlagBits = 32;
    pub const VK_ACCESS_SHADER_WRITE_BIT : VkAccessFlagBits = 64;
    pub const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT : VkAccessFlagBits = 128;
    pub const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT : VkAccessFlagBits = 256;
    pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT : VkAccessFlagBits = 512;
    pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT : VkAccessFlagBits = 1024;
    pub const VK_ACCESS_TRANSFER_READ_BIT : VkAccessFlagBits = 2048;
    pub const VK_ACCESS_TRANSFER_WRITE_BIT : VkAccessFlagBits = 4096;
    pub const VK_ACCESS_HOST_READ_BIT : VkAccessFlagBits = 8192;
    pub const VK_ACCESS_HOST_WRITE_BIT : VkAccessFlagBits = 16384;
    pub const VK_ACCESS_MEMORY_READ_BIT : VkAccessFlagBits = 32768;
    pub const VK_ACCESS_MEMORY_WRITE_BIT : VkAccessFlagBits = 65536;
}

pub type VkBufferUsageFlagBits = u32;
pub mod VkBufferUsageFlagBitsValue {
    use crate::VkBufferUsageFlagBits;

    pub const VK_BUFFER_USAGE_TRANSFER_SRC_BIT : VkBufferUsageFlagBits = 1;
    pub const VK_BUFFER_USAGE_TRANSFER_DST_BIT : VkBufferUsageFlagBits = 2;
    pub const VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT : VkBufferUsageFlagBits = 4;
    pub const VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT : VkBufferUsageFlagBits = 8;
    pub const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT : VkBufferUsageFlagBits = 16;
    pub const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT : VkBufferUsageFlagBits = 32;
    pub const VK_BUFFER_USAGE_INDEX_BUFFER_BIT : VkBufferUsageFlagBits = 64;
    pub const VK_BUFFER_USAGE_VERTEX_BUFFER_BIT : VkBufferUsageFlagBits = 128;
    pub const VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT : VkBufferUsageFlagBits = 256;
}

pub type VkBufferUsageFlagBits2 = u64;
pub mod VkBufferUsageFlagBits2Value {
    use crate::VkBufferUsageFlagBits2;

}

pub type VkBufferCreateFlagBits = u32;
pub mod VkBufferCreateFlagBitsValue {
    use crate::VkBufferCreateFlagBits;

    pub const VK_BUFFER_CREATE_SPARSE_BINDING_BIT : VkBufferCreateFlagBits = 1;
    pub const VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT : VkBufferCreateFlagBits = 2;
    pub const VK_BUFFER_CREATE_SPARSE_ALIASED_BIT : VkBufferCreateFlagBits = 4;
}

pub type VkShaderStageFlagBits = u32;
pub mod VkShaderStageFlagBitsValue {
    use crate::VkShaderStageFlagBits;

    pub const VK_SHADER_STAGE_VERTEX_BIT : VkShaderStageFlagBits = 1;
    pub const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT : VkShaderStageFlagBits = 2;
    pub const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT : VkShaderStageFlagBits = 4;
    pub const VK_SHADER_STAGE_GEOMETRY_BIT : VkShaderStageFlagBits = 8;
    pub const VK_SHADER_STAGE_FRAGMENT_BIT : VkShaderStageFlagBits = 16;
    pub const VK_SHADER_STAGE_COMPUTE_BIT : VkShaderStageFlagBits = 32;
    pub const VK_SHADER_STAGE_ALL_GRAPHICS : VkShaderStageFlagBits = 0x0000001F;
    pub const VK_SHADER_STAGE_ALL : VkShaderStageFlagBits = 0x7FFFFFFF;
}

pub type VkImageUsageFlagBits = u32;
pub mod VkImageUsageFlagBitsValue {
    use crate::VkImageUsageFlagBits;

    pub const VK_IMAGE_USAGE_TRANSFER_SRC_BIT : VkImageUsageFlagBits = 1;
    pub const VK_IMAGE_USAGE_TRANSFER_DST_BIT : VkImageUsageFlagBits = 2;
    pub const VK_IMAGE_USAGE_SAMPLED_BIT : VkImageUsageFlagBits = 4;
    pub const VK_IMAGE_USAGE_STORAGE_BIT : VkImageUsageFlagBits = 8;
    pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT : VkImageUsageFlagBits = 16;
    pub const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT : VkImageUsageFlagBits = 32;
    pub const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT : VkImageUsageFlagBits = 64;
    pub const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT : VkImageUsageFlagBits = 128;
}

pub type VkImageUsageFlagBits2KHR = u64;
pub mod VkImageUsageFlagBits2KHRValue {
    use crate::VkImageUsageFlagBits2KHR;

}

pub type VkImageCreateFlagBits = u32;
pub mod VkImageCreateFlagBitsValue {
    use crate::VkImageCreateFlagBits;

    pub const VK_IMAGE_CREATE_SPARSE_BINDING_BIT : VkImageCreateFlagBits = 1;
    pub const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT : VkImageCreateFlagBits = 2;
    pub const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT : VkImageCreateFlagBits = 4;
    pub const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT : VkImageCreateFlagBits = 8;
    pub const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT : VkImageCreateFlagBits = 16;
}

pub type VkImageCreateFlagBits2KHR = u64;
pub mod VkImageCreateFlagBits2KHRValue {
    use crate::VkImageCreateFlagBits2KHR;

}

pub type VkImageViewCreateFlagBits = u32;
pub mod VkImageViewCreateFlagBitsValue {
    use crate::VkImageViewCreateFlagBits;

}

pub type VkSamplerCreateFlagBits = u32;
pub mod VkSamplerCreateFlagBitsValue {
    use crate::VkSamplerCreateFlagBits;

}

pub type VkPipelineCreateFlagBits = u32;
pub mod VkPipelineCreateFlagBitsValue {
    use crate::VkPipelineCreateFlagBits;

    pub const VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT : VkPipelineCreateFlagBits = 1;
    pub const VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT : VkPipelineCreateFlagBits = 2;
    pub const VK_PIPELINE_CREATE_DERIVATIVE_BIT : VkPipelineCreateFlagBits = 4;
}

pub type VkPipelineCreateFlagBits2 = u64;
pub mod VkPipelineCreateFlagBits2Value {
    use crate::VkPipelineCreateFlagBits2;

}

pub type VkPipelineShaderStageCreateFlagBits = u32;
pub mod VkPipelineShaderStageCreateFlagBitsValue {
    use crate::VkPipelineShaderStageCreateFlagBits;

}

pub type VkColorComponentFlagBits = u32;
pub mod VkColorComponentFlagBitsValue {
    use crate::VkColorComponentFlagBits;

    pub const VK_COLOR_COMPONENT_R_BIT : VkColorComponentFlagBits = 1;
    pub const VK_COLOR_COMPONENT_G_BIT : VkColorComponentFlagBits = 2;
    pub const VK_COLOR_COMPONENT_B_BIT : VkColorComponentFlagBits = 4;
    pub const VK_COLOR_COMPONENT_A_BIT : VkColorComponentFlagBits = 8;
}

pub type VkFenceCreateFlagBits = u32;
pub mod VkFenceCreateFlagBitsValue {
    use crate::VkFenceCreateFlagBits;

    pub const VK_FENCE_CREATE_SIGNALED_BIT : VkFenceCreateFlagBits = 1;
}

pub type VkSemaphoreCreateFlagBits = u32;
pub mod VkSemaphoreCreateFlagBitsValue {
    use crate::VkSemaphoreCreateFlagBits;

}

pub type VkFormatFeatureFlagBits = u32;
pub mod VkFormatFeatureFlagBitsValue {
    use crate::VkFormatFeatureFlagBits;

    pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT : VkFormatFeatureFlagBits = 1;
    pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT : VkFormatFeatureFlagBits = 2;
    pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT : VkFormatFeatureFlagBits = 4;
    pub const VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT : VkFormatFeatureFlagBits = 8;
    pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT : VkFormatFeatureFlagBits = 16;
    pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT : VkFormatFeatureFlagBits = 32;
    pub const VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT : VkFormatFeatureFlagBits = 64;
    pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT : VkFormatFeatureFlagBits = 128;
    pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT : VkFormatFeatureFlagBits = 256;
    pub const VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT : VkFormatFeatureFlagBits = 512;
    pub const VK_FORMAT_FEATURE_BLIT_SRC_BIT : VkFormatFeatureFlagBits = 1024;
    pub const VK_FORMAT_FEATURE_BLIT_DST_BIT : VkFormatFeatureFlagBits = 2048;
    pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT : VkFormatFeatureFlagBits = 4096;
}

pub type VkQueryControlFlagBits = u32;
pub mod VkQueryControlFlagBitsValue {
    use crate::VkQueryControlFlagBits;

    pub const VK_QUERY_CONTROL_PRECISE_BIT : VkQueryControlFlagBits = 1;
}

pub type VkQueryResultFlagBits = u32;
pub mod VkQueryResultFlagBitsValue {
    use crate::VkQueryResultFlagBits;

    pub const VK_QUERY_RESULT_64_BIT : VkQueryResultFlagBits = 1;
    pub const VK_QUERY_RESULT_WAIT_BIT : VkQueryResultFlagBits = 2;
    pub const VK_QUERY_RESULT_WITH_AVAILABILITY_BIT : VkQueryResultFlagBits = 4;
    pub const VK_QUERY_RESULT_PARTIAL_BIT : VkQueryResultFlagBits = 8;
}

pub type VkCommandBufferUsageFlagBits = u32;
pub mod VkCommandBufferUsageFlagBitsValue {
    use crate::VkCommandBufferUsageFlagBits;

    pub const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT : VkCommandBufferUsageFlagBits = 1;
    pub const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT : VkCommandBufferUsageFlagBits = 2;
    pub const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT : VkCommandBufferUsageFlagBits = 4;
}

pub type VkQueryPipelineStatisticFlagBits = u32;
pub mod VkQueryPipelineStatisticFlagBitsValue {
    use crate::VkQueryPipelineStatisticFlagBits;

    pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT : VkQueryPipelineStatisticFlagBits = 1;
    pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT : VkQueryPipelineStatisticFlagBits = 2;
    pub const VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT : VkQueryPipelineStatisticFlagBits = 4;
    pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT : VkQueryPipelineStatisticFlagBits = 8;
    pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT : VkQueryPipelineStatisticFlagBits = 16;
    pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT : VkQueryPipelineStatisticFlagBits = 32;
    pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT : VkQueryPipelineStatisticFlagBits = 64;
    pub const VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT : VkQueryPipelineStatisticFlagBits = 128;
    pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT : VkQueryPipelineStatisticFlagBits = 256;
    pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT : VkQueryPipelineStatisticFlagBits = 512;
    pub const VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT : VkQueryPipelineStatisticFlagBits = 1024;
}

pub type VkMemoryMapFlagBits = u32;
pub mod VkMemoryMapFlagBitsValue {
    use crate::VkMemoryMapFlagBits;

}

pub type VkImageAspectFlagBits = u32;
pub mod VkImageAspectFlagBitsValue {
    use crate::VkImageAspectFlagBits;

    pub const VK_IMAGE_ASPECT_COLOR_BIT : VkImageAspectFlagBits = 1;
    pub const VK_IMAGE_ASPECT_DEPTH_BIT : VkImageAspectFlagBits = 2;
    pub const VK_IMAGE_ASPECT_STENCIL_BIT : VkImageAspectFlagBits = 4;
    pub const VK_IMAGE_ASPECT_METADATA_BIT : VkImageAspectFlagBits = 8;
}

pub type VkSparseImageFormatFlagBits = u32;
pub mod VkSparseImageFormatFlagBitsValue {
    use crate::VkSparseImageFormatFlagBits;

    pub const VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT : VkSparseImageFormatFlagBits = 1;
    pub const VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT : VkSparseImageFormatFlagBits = 2;
    pub const VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT : VkSparseImageFormatFlagBits = 4;
}

pub type VkSparseMemoryBindFlagBits = u32;
pub mod VkSparseMemoryBindFlagBitsValue {
    use crate::VkSparseMemoryBindFlagBits;

    pub const VK_SPARSE_MEMORY_BIND_METADATA_BIT : VkSparseMemoryBindFlagBits = 1;
}

pub type VkPipelineStageFlagBits = u32;
pub mod VkPipelineStageFlagBitsValue {
    use crate::VkPipelineStageFlagBits;

    pub const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT : VkPipelineStageFlagBits = 1;
    pub const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT : VkPipelineStageFlagBits = 2;
    pub const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT : VkPipelineStageFlagBits = 4;
    pub const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT : VkPipelineStageFlagBits = 8;
    pub const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT : VkPipelineStageFlagBits = 16;
    pub const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT : VkPipelineStageFlagBits = 32;
    pub const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT : VkPipelineStageFlagBits = 64;
    pub const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT : VkPipelineStageFlagBits = 128;
    pub const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT : VkPipelineStageFlagBits = 256;
    pub const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT : VkPipelineStageFlagBits = 512;
    pub const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT : VkPipelineStageFlagBits = 1024;
    pub const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT : VkPipelineStageFlagBits = 2048;
    pub const VK_PIPELINE_STAGE_TRANSFER_BIT : VkPipelineStageFlagBits = 4096;
    pub const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT : VkPipelineStageFlagBits = 8192;
    pub const VK_PIPELINE_STAGE_HOST_BIT : VkPipelineStageFlagBits = 16384;
    pub const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT : VkPipelineStageFlagBits = 32768;
    pub const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT : VkPipelineStageFlagBits = 65536;
}

pub type VkCommandPoolCreateFlagBits = u32;
pub mod VkCommandPoolCreateFlagBitsValue {
    use crate::VkCommandPoolCreateFlagBits;

    pub const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT : VkCommandPoolCreateFlagBits = 1;
    pub const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT : VkCommandPoolCreateFlagBits = 2;
}

pub type VkCommandPoolResetFlagBits = u32;
pub mod VkCommandPoolResetFlagBitsValue {
    use crate::VkCommandPoolResetFlagBits;

    pub const VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT : VkCommandPoolResetFlagBits = 1;
}

pub type VkCommandBufferResetFlagBits = u32;
pub mod VkCommandBufferResetFlagBitsValue {
    use crate::VkCommandBufferResetFlagBits;

    pub const VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT : VkCommandBufferResetFlagBits = 1;
}

pub type VkSampleCountFlagBits = u32;
pub mod VkSampleCountFlagBitsValue {
    use crate::VkSampleCountFlagBits;

    pub const VK_SAMPLE_COUNT_1_BIT : VkSampleCountFlagBits = 1;
    pub const VK_SAMPLE_COUNT_2_BIT : VkSampleCountFlagBits = 2;
    pub const VK_SAMPLE_COUNT_4_BIT : VkSampleCountFlagBits = 4;
    pub const VK_SAMPLE_COUNT_8_BIT : VkSampleCountFlagBits = 8;
    pub const VK_SAMPLE_COUNT_16_BIT : VkSampleCountFlagBits = 16;
    pub const VK_SAMPLE_COUNT_32_BIT : VkSampleCountFlagBits = 32;
    pub const VK_SAMPLE_COUNT_64_BIT : VkSampleCountFlagBits = 64;
}

pub type VkAttachmentDescriptionFlagBits = u32;
pub mod VkAttachmentDescriptionFlagBitsValue {
    use crate::VkAttachmentDescriptionFlagBits;

    pub const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT : VkAttachmentDescriptionFlagBits = 1;
}

pub type VkStencilFaceFlagBits = u32;
pub mod VkStencilFaceFlagBitsValue {
    use crate::VkStencilFaceFlagBits;

    pub const VK_STENCIL_FACE_FRONT_BIT : VkStencilFaceFlagBits = 1;
    pub const VK_STENCIL_FACE_BACK_BIT : VkStencilFaceFlagBits = 2;
    pub const VK_STENCIL_FACE_FRONT_AND_BACK : VkStencilFaceFlagBits = 0x00000003;
}

pub type VkDescriptorPoolCreateFlagBits = u32;
pub mod VkDescriptorPoolCreateFlagBitsValue {
    use crate::VkDescriptorPoolCreateFlagBits;

    pub const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT : VkDescriptorPoolCreateFlagBits = 1;
}

pub type VkDependencyFlagBits = u32;
pub mod VkDependencyFlagBitsValue {
    use crate::VkDependencyFlagBits;

    pub const VK_DEPENDENCY_BY_REGION_BIT : VkDependencyFlagBits = 1;
}

pub type VkSemaphoreType = i32;
pub mod VkSemaphoreTypeValue {
    use crate::VkSemaphoreType;

    pub const VK_SEMAPHORE_TYPE_BINARY : VkSemaphoreType = 0;
    pub const VK_SEMAPHORE_TYPE_TIMELINE : VkSemaphoreType = 1;
}

pub type VkSemaphoreWaitFlagBits = u32;
pub mod VkSemaphoreWaitFlagBitsValue {
    use crate::VkSemaphoreWaitFlagBits;

    pub const VK_SEMAPHORE_WAIT_ANY_BIT : VkSemaphoreWaitFlagBits = 1;
}

pub type VkPresentModeKHR = i32;
pub mod VkPresentModeKHRValue {
    use crate::VkPresentModeKHR;

    pub const VK_PRESENT_MODE_IMMEDIATE_KHR : VkPresentModeKHR = 0;
    pub const VK_PRESENT_MODE_MAILBOX_KHR : VkPresentModeKHR = 1;
    pub const VK_PRESENT_MODE_FIFO_KHR : VkPresentModeKHR = 2;
    pub const VK_PRESENT_MODE_FIFO_RELAXED_KHR : VkPresentModeKHR = 3;
}

pub type VkColorSpaceKHR = i32;
pub mod VkColorSpaceKHRValue {
    use crate::VkColorSpaceKHR;

    pub const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR : VkColorSpaceKHR = 0;
}

pub type VkDisplayPlaneAlphaFlagBitsKHR = u32;
pub mod VkDisplayPlaneAlphaFlagBitsKHRValue {
    use crate::VkDisplayPlaneAlphaFlagBitsKHR;

    pub const VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR : VkDisplayPlaneAlphaFlagBitsKHR = 1;
    pub const VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR : VkDisplayPlaneAlphaFlagBitsKHR = 2;
    pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR : VkDisplayPlaneAlphaFlagBitsKHR = 4;
    pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR : VkDisplayPlaneAlphaFlagBitsKHR = 8;
}

pub type VkCompositeAlphaFlagBitsKHR = u32;
pub mod VkCompositeAlphaFlagBitsKHRValue {
    use crate::VkCompositeAlphaFlagBitsKHR;

    pub const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR : VkCompositeAlphaFlagBitsKHR = 1;
    pub const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR : VkCompositeAlphaFlagBitsKHR = 2;
    pub const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR : VkCompositeAlphaFlagBitsKHR = 4;
    pub const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR : VkCompositeAlphaFlagBitsKHR = 8;
}

pub type VkSurfaceTransformFlagBitsKHR = u32;
pub mod VkSurfaceTransformFlagBitsKHRValue {
    use crate::VkSurfaceTransformFlagBitsKHR;

    pub const VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR : VkSurfaceTransformFlagBitsKHR = 1;
    pub const VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR : VkSurfaceTransformFlagBitsKHR = 2;
    pub const VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR : VkSurfaceTransformFlagBitsKHR = 4;
    pub const VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR : VkSurfaceTransformFlagBitsKHR = 8;
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR : VkSurfaceTransformFlagBitsKHR = 16;
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR : VkSurfaceTransformFlagBitsKHR = 32;
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR : VkSurfaceTransformFlagBitsKHR = 64;
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR : VkSurfaceTransformFlagBitsKHR = 128;
    pub const VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR : VkSurfaceTransformFlagBitsKHR = 256;
}

pub type VkDisplaySurfaceStereoTypeNV = i32;
pub mod VkDisplaySurfaceStereoTypeNVValue {
    use crate::VkDisplaySurfaceStereoTypeNV;

    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_NONE_NV : VkDisplaySurfaceStereoTypeNV = 0;
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_ONBOARD_DIN_NV : VkDisplaySurfaceStereoTypeNV = 1;
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_HDMI_3D_NV : VkDisplaySurfaceStereoTypeNV = 2;
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_INBAND_DISPLAYPORT_NV : VkDisplaySurfaceStereoTypeNV = 3;
}

pub type VkSwapchainImageUsageFlagBitsANDROID = u32;
pub mod VkSwapchainImageUsageFlagBitsANDROIDValue {
    use crate::VkSwapchainImageUsageFlagBitsANDROID;

    pub const VK_SWAPCHAIN_IMAGE_USAGE_SHARED_BIT_ANDROID : VkSwapchainImageUsageFlagBitsANDROID = 1;
}

pub type VkTimeDomainKHR = i32;
pub mod VkTimeDomainKHRValue {
    use crate::VkTimeDomainKHR;

    pub const VK_TIME_DOMAIN_DEVICE_KHR : VkTimeDomainKHR = 0;
    pub const VK_TIME_DOMAIN_CLOCK_MONOTONIC_KHR : VkTimeDomainKHR = 1;
    pub const VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_KHR : VkTimeDomainKHR = 2;
    pub const VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_KHR : VkTimeDomainKHR = 3;
}

pub type VkDebugReportFlagBitsEXT = u32;
pub mod VkDebugReportFlagBitsEXTValue {
    use crate::VkDebugReportFlagBitsEXT;

    pub const VK_DEBUG_REPORT_INFORMATION_BIT_EXT : VkDebugReportFlagBitsEXT = 1;
    pub const VK_DEBUG_REPORT_WARNING_BIT_EXT : VkDebugReportFlagBitsEXT = 2;
    pub const VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT : VkDebugReportFlagBitsEXT = 4;
    pub const VK_DEBUG_REPORT_ERROR_BIT_EXT : VkDebugReportFlagBitsEXT = 8;
    pub const VK_DEBUG_REPORT_DEBUG_BIT_EXT : VkDebugReportFlagBitsEXT = 16;
}

pub type VkDebugReportObjectTypeEXT = i32;
pub mod VkDebugReportObjectTypeEXTValue {
    use crate::VkDebugReportObjectTypeEXT;

    pub const VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT : VkDebugReportObjectTypeEXT = 0;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT : VkDebugReportObjectTypeEXT = 1;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT : VkDebugReportObjectTypeEXT = 2;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT : VkDebugReportObjectTypeEXT = 3;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT : VkDebugReportObjectTypeEXT = 4;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT : VkDebugReportObjectTypeEXT = 5;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT : VkDebugReportObjectTypeEXT = 6;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT : VkDebugReportObjectTypeEXT = 7;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT : VkDebugReportObjectTypeEXT = 8;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT : VkDebugReportObjectTypeEXT = 9;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT : VkDebugReportObjectTypeEXT = 10;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT : VkDebugReportObjectTypeEXT = 11;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT : VkDebugReportObjectTypeEXT = 12;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT : VkDebugReportObjectTypeEXT = 13;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT : VkDebugReportObjectTypeEXT = 14;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT : VkDebugReportObjectTypeEXT = 15;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT : VkDebugReportObjectTypeEXT = 16;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT : VkDebugReportObjectTypeEXT = 17;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT : VkDebugReportObjectTypeEXT = 18;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT : VkDebugReportObjectTypeEXT = 19;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT : VkDebugReportObjectTypeEXT = 20;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT : VkDebugReportObjectTypeEXT = 21;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT : VkDebugReportObjectTypeEXT = 22;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT : VkDebugReportObjectTypeEXT = 23;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT : VkDebugReportObjectTypeEXT = 24;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT : VkDebugReportObjectTypeEXT = 25;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT : VkDebugReportObjectTypeEXT = 26;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT : VkDebugReportObjectTypeEXT = 27;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT : VkDebugReportObjectTypeEXT = 28;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT : VkDebugReportObjectTypeEXT = 29;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT : VkDebugReportObjectTypeEXT = 30;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT : VkDebugReportObjectTypeEXT = 33;
}

pub type VkDeviceMemoryReportEventTypeEXT = i32;
pub mod VkDeviceMemoryReportEventTypeEXTValue {
    use crate::VkDeviceMemoryReportEventTypeEXT;

    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT : VkDeviceMemoryReportEventTypeEXT = 0;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT : VkDeviceMemoryReportEventTypeEXT = 1;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT : VkDeviceMemoryReportEventTypeEXT = 2;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT : VkDeviceMemoryReportEventTypeEXT = 3;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT : VkDeviceMemoryReportEventTypeEXT = 4;
}

pub type VkRasterizationOrderAMD = i32;
pub mod VkRasterizationOrderAMDValue {
    use crate::VkRasterizationOrderAMD;

    pub const VK_RASTERIZATION_ORDER_STRICT_AMD : VkRasterizationOrderAMD = 0;
    pub const VK_RASTERIZATION_ORDER_RELAXED_AMD : VkRasterizationOrderAMD = 1;
}

pub type VkExternalMemoryHandleTypeFlagBitsNV = u32;
pub mod VkExternalMemoryHandleTypeFlagBitsNVValue {
    use crate::VkExternalMemoryHandleTypeFlagBitsNV;

    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV : VkExternalMemoryHandleTypeFlagBitsNV = 1;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV : VkExternalMemoryHandleTypeFlagBitsNV = 2;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV : VkExternalMemoryHandleTypeFlagBitsNV = 4;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV : VkExternalMemoryHandleTypeFlagBitsNV = 8;
}

pub type VkExternalMemoryFeatureFlagBitsNV = u32;
pub mod VkExternalMemoryFeatureFlagBitsNVValue {
    use crate::VkExternalMemoryFeatureFlagBitsNV;

    pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV : VkExternalMemoryFeatureFlagBitsNV = 1;
    pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV : VkExternalMemoryFeatureFlagBitsNV = 2;
    pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV : VkExternalMemoryFeatureFlagBitsNV = 4;
}

pub type VkClusterAccelerationStructureIndexFormatFlagBitsNV = u32;
pub mod VkClusterAccelerationStructureIndexFormatFlagBitsNVValue {
    use crate::VkClusterAccelerationStructureIndexFormatFlagBitsNV;

    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_8BIT_NV : VkClusterAccelerationStructureIndexFormatFlagBitsNV = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_16BIT_NV : VkClusterAccelerationStructureIndexFormatFlagBitsNV = 2;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_32BIT_NV : VkClusterAccelerationStructureIndexFormatFlagBitsNV = 4;
}

pub type VkClusterAccelerationStructureTypeNV = i32;
pub mod VkClusterAccelerationStructureTypeNVValue {
    use crate::VkClusterAccelerationStructureTypeNV;

    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_TYPE_CLUSTERS_BOTTOM_LEVEL_NV : VkClusterAccelerationStructureTypeNV = 0;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_TYPE_TRIANGLE_CLUSTER_NV : VkClusterAccelerationStructureTypeNV = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_TYPE_TRIANGLE_CLUSTER_TEMPLATE_NV : VkClusterAccelerationStructureTypeNV = 2;
}

pub type VkClusterAccelerationStructureOpTypeNV = i32;
pub mod VkClusterAccelerationStructureOpTypeNVValue {
    use crate::VkClusterAccelerationStructureOpTypeNV;

    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_MOVE_OBJECTS_NV : VkClusterAccelerationStructureOpTypeNV = 0;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_CLUSTERS_BOTTOM_LEVEL_NV : VkClusterAccelerationStructureOpTypeNV = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_TRIANGLE_CLUSTER_NV : VkClusterAccelerationStructureOpTypeNV = 2;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_TRIANGLE_CLUSTER_TEMPLATE_NV : VkClusterAccelerationStructureOpTypeNV = 3;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_INSTANTIATE_TRIANGLE_CLUSTER_NV : VkClusterAccelerationStructureOpTypeNV = 4;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_GET_CLUSTER_TEMPLATE_INDICES_NV : VkClusterAccelerationStructureOpTypeNV = 5;
}

pub type VkClusterAccelerationStructureOpModeNV = i32;
pub mod VkClusterAccelerationStructureOpModeNVValue {
    use crate::VkClusterAccelerationStructureOpModeNV;

    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_IMPLICIT_DESTINATIONS_NV : VkClusterAccelerationStructureOpModeNV = 0;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_EXPLICIT_DESTINATIONS_NV : VkClusterAccelerationStructureOpModeNV = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_COMPUTE_SIZES_NV : VkClusterAccelerationStructureOpModeNV = 2;
}

pub type VkClusterAccelerationStructureClusterFlagBitsNV = u32;
pub mod VkClusterAccelerationStructureClusterFlagBitsNVValue {
    use crate::VkClusterAccelerationStructureClusterFlagBitsNV;

    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_CLUSTER_ALLOW_DISABLE_OPACITY_MICROMAPS_NV : VkClusterAccelerationStructureClusterFlagBitsNV = 1;
}

pub type VkClusterAccelerationStructureGeometryFlagBitsNV = u32;
pub mod VkClusterAccelerationStructureGeometryFlagBitsNVValue {
    use crate::VkClusterAccelerationStructureGeometryFlagBitsNV;

    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_CULL_DISABLE_BIT_NV : VkClusterAccelerationStructureGeometryFlagBitsNV = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_NO_DUPLICATE_ANYHIT_INVOCATION_BIT_NV : VkClusterAccelerationStructureGeometryFlagBitsNV = 2;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_OPAQUE_BIT_NV : VkClusterAccelerationStructureGeometryFlagBitsNV = 4;
}

pub type VkClusterAccelerationStructureAddressResolutionFlagBitsNV = u32;
pub mod VkClusterAccelerationStructureAddressResolutionFlagBitsNVValue {
    use crate::VkClusterAccelerationStructureAddressResolutionFlagBitsNV;

    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_NONE_NV : VkClusterAccelerationStructureAddressResolutionFlagBitsNV = 0;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_IMPLICIT_DATA_BIT_NV : VkClusterAccelerationStructureAddressResolutionFlagBitsNV = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SCRATCH_DATA_BIT_NV : VkClusterAccelerationStructureAddressResolutionFlagBitsNV = 2;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_ADDRESS_ARRAY_BIT_NV : VkClusterAccelerationStructureAddressResolutionFlagBitsNV = 4;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_SIZES_ARRAY_BIT_NV : VkClusterAccelerationStructureAddressResolutionFlagBitsNV = 8;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_ARRAY_BIT_NV : VkClusterAccelerationStructureAddressResolutionFlagBitsNV = 16;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_COUNT_BIT_NV : VkClusterAccelerationStructureAddressResolutionFlagBitsNV = 32;
}

pub type VkValidationCheckEXT = i32;
pub mod VkValidationCheckEXTValue {
    use crate::VkValidationCheckEXT;

    pub const VK_VALIDATION_CHECK_ALL_EXT : VkValidationCheckEXT = 0;
    pub const VK_VALIDATION_CHECK_SHADERS_EXT : VkValidationCheckEXT = 1;
}

pub type VkValidationFeatureEnableEXT = i32;
pub mod VkValidationFeatureEnableEXTValue {
    use crate::VkValidationFeatureEnableEXT;

    pub const VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT : VkValidationFeatureEnableEXT = 0;
    pub const VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT : VkValidationFeatureEnableEXT = 1;
    pub const VK_VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT : VkValidationFeatureEnableEXT = 2;
    pub const VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT : VkValidationFeatureEnableEXT = 3;
    pub const VK_VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT : VkValidationFeatureEnableEXT = 4;
}

pub type VkValidationFeatureDisableEXT = i32;
pub mod VkValidationFeatureDisableEXTValue {
    use crate::VkValidationFeatureDisableEXT;

    pub const VK_VALIDATION_FEATURE_DISABLE_ALL_EXT : VkValidationFeatureDisableEXT = 0;
    pub const VK_VALIDATION_FEATURE_DISABLE_SHADERS_EXT : VkValidationFeatureDisableEXT = 1;
    pub const VK_VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT : VkValidationFeatureDisableEXT = 2;
    pub const VK_VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT : VkValidationFeatureDisableEXT = 3;
    pub const VK_VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT : VkValidationFeatureDisableEXT = 4;
    pub const VK_VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT : VkValidationFeatureDisableEXT = 5;
    pub const VK_VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT : VkValidationFeatureDisableEXT = 6;
    pub const VK_VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE_EXT : VkValidationFeatureDisableEXT = 7;
}

pub type VkLayerSettingTypeEXT = i32;
pub mod VkLayerSettingTypeEXTValue {
    use crate::VkLayerSettingTypeEXT;

    pub const VK_LAYER_SETTING_TYPE_BOOL32_EXT : VkLayerSettingTypeEXT = 0;
    pub const VK_LAYER_SETTING_TYPE_INT32_EXT : VkLayerSettingTypeEXT = 1;
    pub const VK_LAYER_SETTING_TYPE_INT64_EXT : VkLayerSettingTypeEXT = 2;
    pub const VK_LAYER_SETTING_TYPE_UINT32_EXT : VkLayerSettingTypeEXT = 3;
    pub const VK_LAYER_SETTING_TYPE_UINT64_EXT : VkLayerSettingTypeEXT = 4;
    pub const VK_LAYER_SETTING_TYPE_FLOAT32_EXT : VkLayerSettingTypeEXT = 5;
    pub const VK_LAYER_SETTING_TYPE_FLOAT64_EXT : VkLayerSettingTypeEXT = 6;
    pub const VK_LAYER_SETTING_TYPE_STRING_EXT : VkLayerSettingTypeEXT = 7;
}

pub type VkSubgroupFeatureFlagBits = u32;
pub mod VkSubgroupFeatureFlagBitsValue {
    use crate::VkSubgroupFeatureFlagBits;

    pub const VK_SUBGROUP_FEATURE_BASIC_BIT : VkSubgroupFeatureFlagBits = 1;
    pub const VK_SUBGROUP_FEATURE_VOTE_BIT : VkSubgroupFeatureFlagBits = 2;
    pub const VK_SUBGROUP_FEATURE_ARITHMETIC_BIT : VkSubgroupFeatureFlagBits = 4;
    pub const VK_SUBGROUP_FEATURE_BALLOT_BIT : VkSubgroupFeatureFlagBits = 8;
    pub const VK_SUBGROUP_FEATURE_SHUFFLE_BIT : VkSubgroupFeatureFlagBits = 16;
    pub const VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT : VkSubgroupFeatureFlagBits = 32;
    pub const VK_SUBGROUP_FEATURE_CLUSTERED_BIT : VkSubgroupFeatureFlagBits = 64;
    pub const VK_SUBGROUP_FEATURE_QUAD_BIT : VkSubgroupFeatureFlagBits = 128;
}

pub type VkIndirectCommandsLayoutUsageFlagBitsNV = u32;
pub mod VkIndirectCommandsLayoutUsageFlagBitsNVValue {
    use crate::VkIndirectCommandsLayoutUsageFlagBitsNV;

    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV : VkIndirectCommandsLayoutUsageFlagBitsNV = 1;
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV : VkIndirectCommandsLayoutUsageFlagBitsNV = 2;
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV : VkIndirectCommandsLayoutUsageFlagBitsNV = 4;
}

pub type VkIndirectStateFlagBitsNV = u32;
pub mod VkIndirectStateFlagBitsNVValue {
    use crate::VkIndirectStateFlagBitsNV;

    pub const VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV : VkIndirectStateFlagBitsNV = 1;
}

pub type VkIndirectCommandsTokenTypeNV = i32;
pub mod VkIndirectCommandsTokenTypeNVValue {
    use crate::VkIndirectCommandsTokenTypeNV;

    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV : VkIndirectCommandsTokenTypeNV = 0;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV : VkIndirectCommandsTokenTypeNV = 1;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV : VkIndirectCommandsTokenTypeNV = 2;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV : VkIndirectCommandsTokenTypeNV = 3;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV : VkIndirectCommandsTokenTypeNV = 4;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV : VkIndirectCommandsTokenTypeNV = 5;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV : VkIndirectCommandsTokenTypeNV = 6;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV : VkIndirectCommandsTokenTypeNV = 7;
}

pub type VkPrivateDataSlotCreateFlagBits = u32;
pub mod VkPrivateDataSlotCreateFlagBitsValue {
    use crate::VkPrivateDataSlotCreateFlagBits;

}

pub type VkDescriptorSetLayoutCreateFlagBits = u32;
pub mod VkDescriptorSetLayoutCreateFlagBitsValue {
    use crate::VkDescriptorSetLayoutCreateFlagBits;

}

pub type VkExternalMemoryHandleTypeFlagBits = u32;
pub mod VkExternalMemoryHandleTypeFlagBitsValue {
    use crate::VkExternalMemoryHandleTypeFlagBits;

    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT : VkExternalMemoryHandleTypeFlagBits = 1;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT : VkExternalMemoryHandleTypeFlagBits = 2;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT : VkExternalMemoryHandleTypeFlagBits = 4;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT : VkExternalMemoryHandleTypeFlagBits = 8;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT : VkExternalMemoryHandleTypeFlagBits = 16;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT : VkExternalMemoryHandleTypeFlagBits = 32;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT : VkExternalMemoryHandleTypeFlagBits = 64;
}

pub type VkExternalMemoryFeatureFlagBits = u32;
pub mod VkExternalMemoryFeatureFlagBitsValue {
    use crate::VkExternalMemoryFeatureFlagBits;

    pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT : VkExternalMemoryFeatureFlagBits = 1;
    pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT : VkExternalMemoryFeatureFlagBits = 2;
    pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT : VkExternalMemoryFeatureFlagBits = 4;
}

pub type VkExternalSemaphoreHandleTypeFlagBits = u32;
pub mod VkExternalSemaphoreHandleTypeFlagBitsValue {
    use crate::VkExternalSemaphoreHandleTypeFlagBits;

    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT : VkExternalSemaphoreHandleTypeFlagBits = 1;
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT : VkExternalSemaphoreHandleTypeFlagBits = 2;
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT : VkExternalSemaphoreHandleTypeFlagBits = 4;
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT : VkExternalSemaphoreHandleTypeFlagBits = 8;
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT : VkExternalSemaphoreHandleTypeFlagBits = 16;
}

pub type VkExternalSemaphoreFeatureFlagBits = u32;
pub mod VkExternalSemaphoreFeatureFlagBitsValue {
    use crate::VkExternalSemaphoreFeatureFlagBits;

    pub const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT : VkExternalSemaphoreFeatureFlagBits = 1;
    pub const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT : VkExternalSemaphoreFeatureFlagBits = 2;
}

pub type VkSemaphoreImportFlagBits = u32;
pub mod VkSemaphoreImportFlagBitsValue {
    use crate::VkSemaphoreImportFlagBits;

    pub const VK_SEMAPHORE_IMPORT_TEMPORARY_BIT : VkSemaphoreImportFlagBits = 1;
}

pub type VkExternalFenceHandleTypeFlagBits = u32;
pub mod VkExternalFenceHandleTypeFlagBitsValue {
    use crate::VkExternalFenceHandleTypeFlagBits;

    pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT : VkExternalFenceHandleTypeFlagBits = 1;
    pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT : VkExternalFenceHandleTypeFlagBits = 2;
    pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT : VkExternalFenceHandleTypeFlagBits = 4;
    pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT : VkExternalFenceHandleTypeFlagBits = 8;
}

pub type VkExternalFenceFeatureFlagBits = u32;
pub mod VkExternalFenceFeatureFlagBitsValue {
    use crate::VkExternalFenceFeatureFlagBits;

    pub const VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT : VkExternalFenceFeatureFlagBits = 1;
    pub const VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT : VkExternalFenceFeatureFlagBits = 2;
}

pub type VkFenceImportFlagBits = u32;
pub mod VkFenceImportFlagBitsValue {
    use crate::VkFenceImportFlagBits;

    pub const VK_FENCE_IMPORT_TEMPORARY_BIT : VkFenceImportFlagBits = 1;
}

pub type VkSurfaceCounterFlagBitsEXT = u32;
pub mod VkSurfaceCounterFlagBitsEXTValue {
    use crate::VkSurfaceCounterFlagBitsEXT;

    pub const VK_SURFACE_COUNTER_VBLANK_BIT_EXT : VkSurfaceCounterFlagBitsEXT = 1;
}

pub type VkDisplayPowerStateEXT = i32;
pub mod VkDisplayPowerStateEXTValue {
    use crate::VkDisplayPowerStateEXT;

    pub const VK_DISPLAY_POWER_STATE_OFF_EXT : VkDisplayPowerStateEXT = 0;
    pub const VK_DISPLAY_POWER_STATE_SUSPEND_EXT : VkDisplayPowerStateEXT = 1;
    pub const VK_DISPLAY_POWER_STATE_ON_EXT : VkDisplayPowerStateEXT = 2;
}

pub type VkDeviceEventTypeEXT = i32;
pub mod VkDeviceEventTypeEXTValue {
    use crate::VkDeviceEventTypeEXT;

    pub const VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT : VkDeviceEventTypeEXT = 0;
}

pub type VkDisplayEventTypeEXT = i32;
pub mod VkDisplayEventTypeEXTValue {
    use crate::VkDisplayEventTypeEXT;

    pub const VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT : VkDisplayEventTypeEXT = 0;
}

pub type VkPeerMemoryFeatureFlagBits = u32;
pub mod VkPeerMemoryFeatureFlagBitsValue {
    use crate::VkPeerMemoryFeatureFlagBits;

    pub const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT : VkPeerMemoryFeatureFlagBits = 1;
    pub const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT : VkPeerMemoryFeatureFlagBits = 2;
    pub const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT : VkPeerMemoryFeatureFlagBits = 4;
    pub const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT : VkPeerMemoryFeatureFlagBits = 8;
}

pub type VkMemoryAllocateFlagBits = u32;
pub mod VkMemoryAllocateFlagBitsValue {
    use crate::VkMemoryAllocateFlagBits;

    pub const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT : VkMemoryAllocateFlagBits = 1;
}

pub type VkDeviceGroupPresentModeFlagBitsKHR = u32;
pub mod VkDeviceGroupPresentModeFlagBitsKHRValue {
    use crate::VkDeviceGroupPresentModeFlagBitsKHR;

    pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR : VkDeviceGroupPresentModeFlagBitsKHR = 1;
    pub const VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR : VkDeviceGroupPresentModeFlagBitsKHR = 2;
    pub const VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR : VkDeviceGroupPresentModeFlagBitsKHR = 4;
    pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR : VkDeviceGroupPresentModeFlagBitsKHR = 8;
}

pub type VkSwapchainCreateFlagBitsKHR = u32;
pub mod VkSwapchainCreateFlagBitsKHRValue {
    use crate::VkSwapchainCreateFlagBitsKHR;

}

pub type VkViewportCoordinateSwizzleNV = i32;
pub mod VkViewportCoordinateSwizzleNVValue {
    use crate::VkViewportCoordinateSwizzleNV;

    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV : VkViewportCoordinateSwizzleNV = 0;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV : VkViewportCoordinateSwizzleNV = 1;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV : VkViewportCoordinateSwizzleNV = 2;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV : VkViewportCoordinateSwizzleNV = 3;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV : VkViewportCoordinateSwizzleNV = 4;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV : VkViewportCoordinateSwizzleNV = 5;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV : VkViewportCoordinateSwizzleNV = 6;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV : VkViewportCoordinateSwizzleNV = 7;
}

pub type VkDiscardRectangleModeEXT = i32;
pub mod VkDiscardRectangleModeEXTValue {
    use crate::VkDiscardRectangleModeEXT;

    pub const VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT : VkDiscardRectangleModeEXT = 0;
    pub const VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT : VkDiscardRectangleModeEXT = 1;
}

pub type VkSubpassDescriptionFlagBits = u32;
pub mod VkSubpassDescriptionFlagBitsValue {
    use crate::VkSubpassDescriptionFlagBits;

}

pub type VkPointClippingBehavior = i32;
pub mod VkPointClippingBehaviorValue {
    use crate::VkPointClippingBehavior;

    pub const VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES : VkPointClippingBehavior = 0;
    pub const VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY : VkPointClippingBehavior = 1;
}

pub type VkSamplerReductionMode = i32;
pub mod VkSamplerReductionModeValue {
    use crate::VkSamplerReductionMode;

    pub const VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE : VkSamplerReductionMode = 0;
    pub const VK_SAMPLER_REDUCTION_MODE_MIN : VkSamplerReductionMode = 1;
    pub const VK_SAMPLER_REDUCTION_MODE_MAX : VkSamplerReductionMode = 2;
}

pub type VkTessellationDomainOrigin = i32;
pub mod VkTessellationDomainOriginValue {
    use crate::VkTessellationDomainOrigin;

    pub const VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT : VkTessellationDomainOrigin = 0;
    pub const VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT : VkTessellationDomainOrigin = 1;
}

pub type VkSamplerYcbcrModelConversion = i32;
pub mod VkSamplerYcbcrModelConversionValue {
    use crate::VkSamplerYcbcrModelConversion;

    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY : VkSamplerYcbcrModelConversion = 0;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY : VkSamplerYcbcrModelConversion = 1;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709 : VkSamplerYcbcrModelConversion = 2;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601 : VkSamplerYcbcrModelConversion = 3;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020 : VkSamplerYcbcrModelConversion = 4;
}

pub type VkSamplerYcbcrRange = i32;
pub mod VkSamplerYcbcrRangeValue {
    use crate::VkSamplerYcbcrRange;

    pub const VK_SAMPLER_YCBCR_RANGE_ITU_FULL : VkSamplerYcbcrRange = 0;
    pub const VK_SAMPLER_YCBCR_RANGE_ITU_NARROW : VkSamplerYcbcrRange = 1;
}

pub type VkChromaLocation = i32;
pub mod VkChromaLocationValue {
    use crate::VkChromaLocation;

    pub const VK_CHROMA_LOCATION_COSITED_EVEN : VkChromaLocation = 0;
    pub const VK_CHROMA_LOCATION_MIDPOINT : VkChromaLocation = 1;
}

pub type VkBlendOverlapEXT = i32;
pub mod VkBlendOverlapEXTValue {
    use crate::VkBlendOverlapEXT;

    pub const VK_BLEND_OVERLAP_UNCORRELATED_EXT : VkBlendOverlapEXT = 0;
    pub const VK_BLEND_OVERLAP_DISJOINT_EXT : VkBlendOverlapEXT = 1;
    pub const VK_BLEND_OVERLAP_CONJOINT_EXT : VkBlendOverlapEXT = 2;
}

pub type VkCoverageModulationModeNV = i32;
pub mod VkCoverageModulationModeNVValue {
    use crate::VkCoverageModulationModeNV;

    pub const VK_COVERAGE_MODULATION_MODE_NONE_NV : VkCoverageModulationModeNV = 0;
    pub const VK_COVERAGE_MODULATION_MODE_RGB_NV : VkCoverageModulationModeNV = 1;
    pub const VK_COVERAGE_MODULATION_MODE_ALPHA_NV : VkCoverageModulationModeNV = 2;
    pub const VK_COVERAGE_MODULATION_MODE_RGBA_NV : VkCoverageModulationModeNV = 3;
}

pub type VkCoverageReductionModeNV = i32;
pub mod VkCoverageReductionModeNVValue {
    use crate::VkCoverageReductionModeNV;

    pub const VK_COVERAGE_REDUCTION_MODE_MERGE_NV : VkCoverageReductionModeNV = 0;
    pub const VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV : VkCoverageReductionModeNV = 1;
}

pub type VkValidationCacheHeaderVersionEXT = i32;
pub mod VkValidationCacheHeaderVersionEXTValue {
    use crate::VkValidationCacheHeaderVersionEXT;

    pub const VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT : VkValidationCacheHeaderVersionEXT = 1;
}

pub type VkShaderInfoTypeAMD = i32;
pub mod VkShaderInfoTypeAMDValue {
    use crate::VkShaderInfoTypeAMD;

    pub const VK_SHADER_INFO_TYPE_STATISTICS_AMD : VkShaderInfoTypeAMD = 0;
    pub const VK_SHADER_INFO_TYPE_BINARY_AMD : VkShaderInfoTypeAMD = 1;
    pub const VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD : VkShaderInfoTypeAMD = 2;
}

pub type VkQueueGlobalPriority = i32;
pub mod VkQueueGlobalPriorityValue {
    use crate::VkQueueGlobalPriority;

    pub const VK_QUEUE_GLOBAL_PRIORITY_LOW : VkQueueGlobalPriority = 128;
    pub const VK_QUEUE_GLOBAL_PRIORITY_MEDIUM : VkQueueGlobalPriority = 256;
    pub const VK_QUEUE_GLOBAL_PRIORITY_HIGH : VkQueueGlobalPriority = 512;
    pub const VK_QUEUE_GLOBAL_PRIORITY_REALTIME : VkQueueGlobalPriority = 1024;
}

pub type VkDebugUtilsMessageSeverityFlagBitsEXT = u32;
pub mod VkDebugUtilsMessageSeverityFlagBitsEXTValue {
    use crate::VkDebugUtilsMessageSeverityFlagBitsEXT;

    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT : VkDebugUtilsMessageSeverityFlagBitsEXT = 1;
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT : VkDebugUtilsMessageSeverityFlagBitsEXT = 16;
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT : VkDebugUtilsMessageSeverityFlagBitsEXT = 256;
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT : VkDebugUtilsMessageSeverityFlagBitsEXT = 4096;
}

pub type VkDebugUtilsMessageTypeFlagBitsEXT = u32;
pub mod VkDebugUtilsMessageTypeFlagBitsEXTValue {
    use crate::VkDebugUtilsMessageTypeFlagBitsEXT;

    pub const VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT : VkDebugUtilsMessageTypeFlagBitsEXT = 1;
    pub const VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT : VkDebugUtilsMessageTypeFlagBitsEXT = 2;
    pub const VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT : VkDebugUtilsMessageTypeFlagBitsEXT = 4;
}

pub type VkConservativeRasterizationModeEXT = i32;
pub mod VkConservativeRasterizationModeEXTValue {
    use crate::VkConservativeRasterizationModeEXT;

    pub const VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT : VkConservativeRasterizationModeEXT = 0;
    pub const VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT : VkConservativeRasterizationModeEXT = 1;
    pub const VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT : VkConservativeRasterizationModeEXT = 2;
}

pub type VkDescriptorBindingFlagBits = u32;
pub mod VkDescriptorBindingFlagBitsValue {
    use crate::VkDescriptorBindingFlagBits;

    pub const VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT : VkDescriptorBindingFlagBits = 1;
    pub const VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT : VkDescriptorBindingFlagBits = 2;
    pub const VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT : VkDescriptorBindingFlagBits = 4;
    pub const VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT : VkDescriptorBindingFlagBits = 8;
}

pub type VkVendorId = i32;
pub mod VkVendorIdValue {
    use crate::VkVendorId;

    pub const VK_VENDOR_ID_KHRONOS : VkVendorId = 0x10000;
    pub const VK_VENDOR_ID_VIV : VkVendorId = 0x10001;
    pub const VK_VENDOR_ID_VSI : VkVendorId = 0x10002;
    pub const VK_VENDOR_ID_KAZAN : VkVendorId = 0x10003;
    pub const VK_VENDOR_ID_CODEPLAY : VkVendorId = 0x10004;
    pub const VK_VENDOR_ID_MESA : VkVendorId = 0x10005;
    pub const VK_VENDOR_ID_POCL : VkVendorId = 0x10006;
    pub const VK_VENDOR_ID_MOBILEYE : VkVendorId = 0x10007;
    pub const VK_VENDOR_ID_APE : VkVendorId = 0x10008;
}

pub type VkDriverId = i32;
pub mod VkDriverIdValue {
    use crate::VkDriverId;

    pub const VK_DRIVER_ID_AMD_PROPRIETARY : VkDriverId = 1;
    pub const VK_DRIVER_ID_AMD_OPEN_SOURCE : VkDriverId = 2;
    pub const VK_DRIVER_ID_MESA_RADV : VkDriverId = 3;
    pub const VK_DRIVER_ID_NVIDIA_PROPRIETARY : VkDriverId = 4;
    pub const VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS : VkDriverId = 5;
    pub const VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA : VkDriverId = 6;
    pub const VK_DRIVER_ID_IMAGINATION_PROPRIETARY : VkDriverId = 7;
    pub const VK_DRIVER_ID_QUALCOMM_PROPRIETARY : VkDriverId = 8;
    pub const VK_DRIVER_ID_ARM_PROPRIETARY : VkDriverId = 9;
    pub const VK_DRIVER_ID_GOOGLE_SWIFTSHADER : VkDriverId = 10;
    pub const VK_DRIVER_ID_GGP_PROPRIETARY : VkDriverId = 11;
    pub const VK_DRIVER_ID_BROADCOM_PROPRIETARY : VkDriverId = 12;
    pub const VK_DRIVER_ID_MESA_LLVMPIPE : VkDriverId = 13;
    pub const VK_DRIVER_ID_MOLTENVK : VkDriverId = 14;
    pub const VK_DRIVER_ID_COREAVI_PROPRIETARY : VkDriverId = 15;
    pub const VK_DRIVER_ID_JUICE_PROPRIETARY : VkDriverId = 16;
    pub const VK_DRIVER_ID_VERISILICON_PROPRIETARY : VkDriverId = 17;
    pub const VK_DRIVER_ID_MESA_TURNIP : VkDriverId = 18;
    pub const VK_DRIVER_ID_MESA_V3DV : VkDriverId = 19;
    pub const VK_DRIVER_ID_MESA_PANVK : VkDriverId = 20;
    pub const VK_DRIVER_ID_SAMSUNG_PROPRIETARY : VkDriverId = 21;
    pub const VK_DRIVER_ID_MESA_VENUS : VkDriverId = 22;
    pub const VK_DRIVER_ID_MESA_DOZEN : VkDriverId = 23;
    pub const VK_DRIVER_ID_MESA_NVK : VkDriverId = 24;
    pub const VK_DRIVER_ID_IMAGINATION_OPEN_SOURCE_MESA : VkDriverId = 25;
    pub const VK_DRIVER_ID_MESA_HONEYKRISP : VkDriverId = 26;
    pub const VK_DRIVER_ID_VULKAN_SC_EMULATION_ON_VULKAN : VkDriverId = 27;
    pub const VK_DRIVER_ID_MESA_KOSMICKRISP : VkDriverId = 28;
    pub const VK_DRIVER_ID_MESA_GFXSTREAM : VkDriverId = 29;
    pub const VK_DRIVER_ID_APE_SOFT : VkDriverId = 30;
}

pub type VkConditionalRenderingFlagBitsEXT = u32;
pub mod VkConditionalRenderingFlagBitsEXTValue {
    use crate::VkConditionalRenderingFlagBitsEXT;

    pub const VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT : VkConditionalRenderingFlagBitsEXT = 1;
}

pub type VkResolveModeFlagBits = u32;
pub mod VkResolveModeFlagBitsValue {
    use crate::VkResolveModeFlagBits;

    pub const VK_RESOLVE_MODE_NONE : VkResolveModeFlagBits = 0;
    pub const VK_RESOLVE_MODE_SAMPLE_ZERO_BIT : VkResolveModeFlagBits = 1;
    pub const VK_RESOLVE_MODE_AVERAGE_BIT : VkResolveModeFlagBits = 2;
    pub const VK_RESOLVE_MODE_MIN_BIT : VkResolveModeFlagBits = 4;
    pub const VK_RESOLVE_MODE_MAX_BIT : VkResolveModeFlagBits = 8;
}

pub type VkShadingRatePaletteEntryNV = i32;
pub mod VkShadingRatePaletteEntryNVValue {
    use crate::VkShadingRatePaletteEntryNV;

    pub const VK_SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS_NV : VkShadingRatePaletteEntryNV = 0;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV : VkShadingRatePaletteEntryNV = 1;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL_NV : VkShadingRatePaletteEntryNV = 2;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL_NV : VkShadingRatePaletteEntryNV = 3;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL_NV : VkShadingRatePaletteEntryNV = 4;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL_NV : VkShadingRatePaletteEntryNV = 5;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS_NV : VkShadingRatePaletteEntryNV = 6;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS_NV : VkShadingRatePaletteEntryNV = 7;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS_NV : VkShadingRatePaletteEntryNV = 8;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS_NV : VkShadingRatePaletteEntryNV = 9;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS_NV : VkShadingRatePaletteEntryNV = 10;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS_NV : VkShadingRatePaletteEntryNV = 11;
}

pub type VkCoarseSampleOrderTypeNV = i32;
pub mod VkCoarseSampleOrderTypeNVValue {
    use crate::VkCoarseSampleOrderTypeNV;

    pub const VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV : VkCoarseSampleOrderTypeNV = 0;
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV : VkCoarseSampleOrderTypeNV = 1;
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV : VkCoarseSampleOrderTypeNV = 2;
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV : VkCoarseSampleOrderTypeNV = 3;
}

pub type VkGeometryInstanceFlagBitsKHR = u32;
pub mod VkGeometryInstanceFlagBitsKHRValue {
    use crate::VkGeometryInstanceFlagBitsKHR;

    pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR : VkGeometryInstanceFlagBitsKHR = 1;
    pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR : VkGeometryInstanceFlagBitsKHR = 2;
    pub const VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR : VkGeometryInstanceFlagBitsKHR = 4;
    pub const VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR : VkGeometryInstanceFlagBitsKHR = 8;
}

pub type VkGeometryFlagBitsKHR = u32;
pub mod VkGeometryFlagBitsKHRValue {
    use crate::VkGeometryFlagBitsKHR;

    pub const VK_GEOMETRY_OPAQUE_BIT_KHR : VkGeometryFlagBitsKHR = 1;
    pub const VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR : VkGeometryFlagBitsKHR = 2;
}

pub type VkBuildAccelerationStructureFlagBitsKHR = u32;
pub mod VkBuildAccelerationStructureFlagBitsKHRValue {
    use crate::VkBuildAccelerationStructureFlagBitsKHR;

    pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR : VkBuildAccelerationStructureFlagBitsKHR = 1;
    pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR : VkBuildAccelerationStructureFlagBitsKHR = 2;
    pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR : VkBuildAccelerationStructureFlagBitsKHR = 4;
    pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR : VkBuildAccelerationStructureFlagBitsKHR = 8;
    pub const VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR : VkBuildAccelerationStructureFlagBitsKHR = 16;
}

pub type VkAccelerationStructureCreateFlagBitsKHR = u32;
pub mod VkAccelerationStructureCreateFlagBitsKHRValue {
    use crate::VkAccelerationStructureCreateFlagBitsKHR;

    pub const VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR : VkAccelerationStructureCreateFlagBitsKHR = 1;
}

pub type VkCopyAccelerationStructureModeKHR = i32;
pub mod VkCopyAccelerationStructureModeKHRValue {
    use crate::VkCopyAccelerationStructureModeKHR;

    pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR : VkCopyAccelerationStructureModeKHR = 0;
    pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR : VkCopyAccelerationStructureModeKHR = 1;
}

pub type VkBuildAccelerationStructureModeKHR = i32;
pub mod VkBuildAccelerationStructureModeKHRValue {
    use crate::VkBuildAccelerationStructureModeKHR;

    pub const VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR : VkBuildAccelerationStructureModeKHR = 0;
    pub const VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR : VkBuildAccelerationStructureModeKHR = 1;
}

pub type VkAccelerationStructureTypeKHR = i32;
pub mod VkAccelerationStructureTypeKHRValue {
    use crate::VkAccelerationStructureTypeKHR;

    pub const VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR : VkAccelerationStructureTypeKHR = 0;
    pub const VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR : VkAccelerationStructureTypeKHR = 1;
    pub const VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR : VkAccelerationStructureTypeKHR = 2;
}

pub type VkGeometryTypeKHR = i32;
pub mod VkGeometryTypeKHRValue {
    use crate::VkGeometryTypeKHR;

    pub const VK_GEOMETRY_TYPE_TRIANGLES_KHR : VkGeometryTypeKHR = 0;
    pub const VK_GEOMETRY_TYPE_AABBS_KHR : VkGeometryTypeKHR = 1;
    pub const VK_GEOMETRY_TYPE_INSTANCES_KHR : VkGeometryTypeKHR = 2;
}

pub type VkAccelerationStructureMemoryRequirementsTypeNV = i32;
pub mod VkAccelerationStructureMemoryRequirementsTypeNVValue {
    use crate::VkAccelerationStructureMemoryRequirementsTypeNV;

    pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV : VkAccelerationStructureMemoryRequirementsTypeNV = 0;
    pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV : VkAccelerationStructureMemoryRequirementsTypeNV = 1;
    pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV : VkAccelerationStructureMemoryRequirementsTypeNV = 2;
}

pub type VkAccelerationStructureBuildTypeKHR = i32;
pub mod VkAccelerationStructureBuildTypeKHRValue {
    use crate::VkAccelerationStructureBuildTypeKHR;

    pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR : VkAccelerationStructureBuildTypeKHR = 0;
    pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR : VkAccelerationStructureBuildTypeKHR = 1;
    pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR : VkAccelerationStructureBuildTypeKHR = 2;
}

pub type VkRayTracingShaderGroupTypeKHR = i32;
pub mod VkRayTracingShaderGroupTypeKHRValue {
    use crate::VkRayTracingShaderGroupTypeKHR;

    pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR : VkRayTracingShaderGroupTypeKHR = 0;
    pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR : VkRayTracingShaderGroupTypeKHR = 1;
    pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR : VkRayTracingShaderGroupTypeKHR = 2;
}

pub type VkAccelerationStructureCompatibilityKHR = i32;
pub mod VkAccelerationStructureCompatibilityKHRValue {
    use crate::VkAccelerationStructureCompatibilityKHR;

    pub const VK_ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR : VkAccelerationStructureCompatibilityKHR = 0;
    pub const VK_ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR : VkAccelerationStructureCompatibilityKHR = 1;
}

pub type VkShaderGroupShaderKHR = i32;
pub mod VkShaderGroupShaderKHRValue {
    use crate::VkShaderGroupShaderKHR;

    pub const VK_SHADER_GROUP_SHADER_GENERAL_KHR : VkShaderGroupShaderKHR = 0;
    pub const VK_SHADER_GROUP_SHADER_CLOSEST_HIT_KHR : VkShaderGroupShaderKHR = 1;
    pub const VK_SHADER_GROUP_SHADER_ANY_HIT_KHR : VkShaderGroupShaderKHR = 2;
    pub const VK_SHADER_GROUP_SHADER_INTERSECTION_KHR : VkShaderGroupShaderKHR = 3;
}

pub type VkMemoryOverallocationBehaviorAMD = i32;
pub mod VkMemoryOverallocationBehaviorAMDValue {
    use crate::VkMemoryOverallocationBehaviorAMD;

    pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD : VkMemoryOverallocationBehaviorAMD = 0;
    pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD : VkMemoryOverallocationBehaviorAMD = 1;
    pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD : VkMemoryOverallocationBehaviorAMD = 2;
}

pub type VkFramebufferCreateFlagBits = u32;
pub mod VkFramebufferCreateFlagBitsValue {
    use crate::VkFramebufferCreateFlagBits;

}

pub type VkQueryPoolCreateFlagBits = u32;
pub mod VkQueryPoolCreateFlagBitsValue {
    use crate::VkQueryPoolCreateFlagBits;

}

pub type VkDeviceDiagnosticsConfigFlagBitsNV = u32;
pub mod VkDeviceDiagnosticsConfigFlagBitsNVValue {
    use crate::VkDeviceDiagnosticsConfigFlagBitsNV;

    pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV : VkDeviceDiagnosticsConfigFlagBitsNV = 1;
    pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV : VkDeviceDiagnosticsConfigFlagBitsNV = 2;
    pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV : VkDeviceDiagnosticsConfigFlagBitsNV = 4;
    pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_ERROR_REPORTING_BIT_NV : VkDeviceDiagnosticsConfigFlagBitsNV = 8;
}

pub type VkPipelineCreationFeedbackFlagBits = u32;
pub mod VkPipelineCreationFeedbackFlagBitsValue {
    use crate::VkPipelineCreationFeedbackFlagBits;

    pub const VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT : VkPipelineCreationFeedbackFlagBits = 1;
    pub const VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT : VkPipelineCreationFeedbackFlagBits = 2;
    pub const VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT : VkPipelineCreationFeedbackFlagBits = 4;
}

pub type VkFullScreenExclusiveEXT = i32;
pub mod VkFullScreenExclusiveEXTValue {
    use crate::VkFullScreenExclusiveEXT;

    pub const VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT : VkFullScreenExclusiveEXT = 0;
    pub const VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT : VkFullScreenExclusiveEXT = 1;
    pub const VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT : VkFullScreenExclusiveEXT = 2;
    pub const VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT : VkFullScreenExclusiveEXT = 3;
}

pub type VkPerformanceCounterScopeKHR = i32;
pub mod VkPerformanceCounterScopeKHRValue {
    use crate::VkPerformanceCounterScopeKHR;

    pub const VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR : VkPerformanceCounterScopeKHR = 0;
    pub const VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR : VkPerformanceCounterScopeKHR = 1;
    pub const VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR : VkPerformanceCounterScopeKHR = 2;
}

pub type VkMemoryDecompressionMethodFlagBitsEXT = u64;
pub mod VkMemoryDecompressionMethodFlagBitsEXTValue {
    use crate::VkMemoryDecompressionMethodFlagBitsEXT;

}

pub type VkPerformanceCounterUnitKHR = i32;
pub mod VkPerformanceCounterUnitKHRValue {
    use crate::VkPerformanceCounterUnitKHR;

    pub const VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR : VkPerformanceCounterUnitKHR = 0;
    pub const VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR : VkPerformanceCounterUnitKHR = 1;
    pub const VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR : VkPerformanceCounterUnitKHR = 2;
    pub const VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR : VkPerformanceCounterUnitKHR = 3;
    pub const VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR : VkPerformanceCounterUnitKHR = 4;
    pub const VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR : VkPerformanceCounterUnitKHR = 5;
    pub const VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR : VkPerformanceCounterUnitKHR = 6;
    pub const VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR : VkPerformanceCounterUnitKHR = 7;
    pub const VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR : VkPerformanceCounterUnitKHR = 8;
    pub const VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR : VkPerformanceCounterUnitKHR = 9;
    pub const VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR : VkPerformanceCounterUnitKHR = 10;
}

pub type VkPerformanceCounterStorageKHR = i32;
pub mod VkPerformanceCounterStorageKHRValue {
    use crate::VkPerformanceCounterStorageKHR;

    pub const VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR : VkPerformanceCounterStorageKHR = 0;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR : VkPerformanceCounterStorageKHR = 1;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR : VkPerformanceCounterStorageKHR = 2;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR : VkPerformanceCounterStorageKHR = 3;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR : VkPerformanceCounterStorageKHR = 4;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR : VkPerformanceCounterStorageKHR = 5;
}

pub type VkPerformanceCounterDescriptionFlagBitsKHR = u32;
pub mod VkPerformanceCounterDescriptionFlagBitsKHRValue {
    use crate::VkPerformanceCounterDescriptionFlagBitsKHR;

    pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR : VkPerformanceCounterDescriptionFlagBitsKHR = 1;
    pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR : VkPerformanceCounterDescriptionFlagBitsKHR = 2;
}

pub type VkAcquireProfilingLockFlagBitsKHR = u32;
pub mod VkAcquireProfilingLockFlagBitsKHRValue {
    use crate::VkAcquireProfilingLockFlagBitsKHR;

}

pub type VkShaderCorePropertiesFlagBitsAMD = u32;
pub mod VkShaderCorePropertiesFlagBitsAMDValue {
    use crate::VkShaderCorePropertiesFlagBitsAMD;

}

pub type VkRefreshObjectFlagBitsKHR = u32;
pub mod VkRefreshObjectFlagBitsKHRValue {
    use crate::VkRefreshObjectFlagBitsKHR;

}

pub type VkPerformanceConfigurationTypeINTEL = i32;
pub mod VkPerformanceConfigurationTypeINTELValue {
    use crate::VkPerformanceConfigurationTypeINTEL;

    pub const VK_PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL : VkPerformanceConfigurationTypeINTEL = 0;
}

pub type VkQueryPoolSamplingModeINTEL = i32;
pub mod VkQueryPoolSamplingModeINTELValue {
    use crate::VkQueryPoolSamplingModeINTEL;

    pub const VK_QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL : VkQueryPoolSamplingModeINTEL = 0;
}

pub type VkPerformanceOverrideTypeINTEL = i32;
pub mod VkPerformanceOverrideTypeINTELValue {
    use crate::VkPerformanceOverrideTypeINTEL;

    pub const VK_PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL : VkPerformanceOverrideTypeINTEL = 0;
    pub const VK_PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL : VkPerformanceOverrideTypeINTEL = 1;
}

pub type VkPerformanceParameterTypeINTEL = i32;
pub mod VkPerformanceParameterTypeINTELValue {
    use crate::VkPerformanceParameterTypeINTEL;

    pub const VK_PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL : VkPerformanceParameterTypeINTEL = 0;
    pub const VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL : VkPerformanceParameterTypeINTEL = 1;
}

pub type VkPerformanceValueTypeINTEL = i32;
pub mod VkPerformanceValueTypeINTELValue {
    use crate::VkPerformanceValueTypeINTEL;

    pub const VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL : VkPerformanceValueTypeINTEL = 0;
    pub const VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL : VkPerformanceValueTypeINTEL = 1;
    pub const VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL : VkPerformanceValueTypeINTEL = 2;
    pub const VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL : VkPerformanceValueTypeINTEL = 3;
    pub const VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL : VkPerformanceValueTypeINTEL = 4;
}

pub type VkShaderFloatControlsIndependence = i32;
pub mod VkShaderFloatControlsIndependenceValue {
    use crate::VkShaderFloatControlsIndependence;

    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY : VkShaderFloatControlsIndependence = 0;
    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL : VkShaderFloatControlsIndependence = 1;
    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE : VkShaderFloatControlsIndependence = 2;
}

pub type VkPipelineExecutableStatisticFormatKHR = i32;
pub mod VkPipelineExecutableStatisticFormatKHRValue {
    use crate::VkPipelineExecutableStatisticFormatKHR;

    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR : VkPipelineExecutableStatisticFormatKHR = 0;
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR : VkPipelineExecutableStatisticFormatKHR = 1;
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR : VkPipelineExecutableStatisticFormatKHR = 2;
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR : VkPipelineExecutableStatisticFormatKHR = 3;
}

pub type VkLineRasterizationMode = i32;
pub mod VkLineRasterizationModeValue {
    use crate::VkLineRasterizationMode;

    pub const VK_LINE_RASTERIZATION_MODE_DEFAULT : VkLineRasterizationMode = 0;
    pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR : VkLineRasterizationMode = 1;
    pub const VK_LINE_RASTERIZATION_MODE_BRESENHAM : VkLineRasterizationMode = 2;
    pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH : VkLineRasterizationMode = 3;
}

pub type VkShaderModuleCreateFlagBits = u32;
pub mod VkShaderModuleCreateFlagBitsValue {
    use crate::VkShaderModuleCreateFlagBits;

}

pub type VkPipelineCompilerControlFlagBitsAMD = u32;
pub mod VkPipelineCompilerControlFlagBitsAMDValue {
    use crate::VkPipelineCompilerControlFlagBitsAMD;

}

pub type VkFaultLevel = i32;
pub mod VkFaultLevelValue {
    use crate::VkFaultLevel;

    pub const VK_FAULT_LEVEL_UNASSIGNED : VkFaultLevel = 0;
    pub const VK_FAULT_LEVEL_CRITICAL : VkFaultLevel = 1;
    pub const VK_FAULT_LEVEL_RECOVERABLE : VkFaultLevel = 2;
    pub const VK_FAULT_LEVEL_WARNING : VkFaultLevel = 3;
}

pub type VkFaultType = i32;
pub mod VkFaultTypeValue {
    use crate::VkFaultType;

    pub const VK_FAULT_TYPE_INVALID : VkFaultType = 0;
    pub const VK_FAULT_TYPE_UNASSIGNED : VkFaultType = 1;
    pub const VK_FAULT_TYPE_IMPLEMENTATION : VkFaultType = 2;
    pub const VK_FAULT_TYPE_SYSTEM : VkFaultType = 3;
    pub const VK_FAULT_TYPE_PHYSICAL_DEVICE : VkFaultType = 4;
    pub const VK_FAULT_TYPE_COMMAND_BUFFER_FULL : VkFaultType = 5;
    pub const VK_FAULT_TYPE_INVALID_API_USAGE : VkFaultType = 6;
}

pub type VkFaultQueryBehavior = i32;
pub mod VkFaultQueryBehaviorValue {
    use crate::VkFaultQueryBehavior;

    pub const VK_FAULT_QUERY_BEHAVIOR_GET_AND_CLEAR_ALL_FAULTS : VkFaultQueryBehavior = 0;
}

pub type VkToolPurposeFlagBits = u32;
pub mod VkToolPurposeFlagBitsValue {
    use crate::VkToolPurposeFlagBits;

    pub const VK_TOOL_PURPOSE_VALIDATION_BIT : VkToolPurposeFlagBits = 1;
    pub const VK_TOOL_PURPOSE_PROFILING_BIT : VkToolPurposeFlagBits = 2;
    pub const VK_TOOL_PURPOSE_TRACING_BIT : VkToolPurposeFlagBits = 4;
    pub const VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT : VkToolPurposeFlagBits = 8;
    pub const VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT : VkToolPurposeFlagBits = 16;
}

pub type VkPipelineMatchControl = i32;
pub mod VkPipelineMatchControlValue {
    use crate::VkPipelineMatchControl;

    pub const VK_PIPELINE_MATCH_CONTROL_APPLICATION_UUID_EXACT_MATCH : VkPipelineMatchControl = 0;
}

pub type VkFragmentShadingRateCombinerOpKHR = i32;
pub mod VkFragmentShadingRateCombinerOpKHRValue {
    use crate::VkFragmentShadingRateCombinerOpKHR;

    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR : VkFragmentShadingRateCombinerOpKHR = 0;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR : VkFragmentShadingRateCombinerOpKHR = 1;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR : VkFragmentShadingRateCombinerOpKHR = 2;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR : VkFragmentShadingRateCombinerOpKHR = 3;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR : VkFragmentShadingRateCombinerOpKHR = 4;
}

pub type VkFragmentShadingRateNV = i32;
pub mod VkFragmentShadingRateNVValue {
    use crate::VkFragmentShadingRateNV;

    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV : VkFragmentShadingRateNV = 0;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV : VkFragmentShadingRateNV = 1;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV : VkFragmentShadingRateNV = 4;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV : VkFragmentShadingRateNV = 5;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV : VkFragmentShadingRateNV = 6;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV : VkFragmentShadingRateNV = 9;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV : VkFragmentShadingRateNV = 10;
    pub const VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV : VkFragmentShadingRateNV = 11;
    pub const VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV : VkFragmentShadingRateNV = 12;
    pub const VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV : VkFragmentShadingRateNV = 13;
    pub const VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV : VkFragmentShadingRateNV = 14;
    pub const VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV : VkFragmentShadingRateNV = 15;
}

pub type VkFragmentShadingRateTypeNV = i32;
pub mod VkFragmentShadingRateTypeNVValue {
    use crate::VkFragmentShadingRateTypeNV;

    pub const VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV : VkFragmentShadingRateTypeNV = 0;
    pub const VK_FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV : VkFragmentShadingRateTypeNV = 1;
}

pub type VkSubpassMergeStatusEXT = i32;
pub mod VkSubpassMergeStatusEXTValue {
    use crate::VkSubpassMergeStatusEXT;

    pub const VK_SUBPASS_MERGE_STATUS_MERGED_EXT : VkSubpassMergeStatusEXT = 0;
    pub const VK_SUBPASS_MERGE_STATUS_DISALLOWED_EXT : VkSubpassMergeStatusEXT = 1;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SIDE_EFFECTS_EXT : VkSubpassMergeStatusEXT = 2;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SAMPLES_MISMATCH_EXT : VkSubpassMergeStatusEXT = 3;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_VIEWS_MISMATCH_EXT : VkSubpassMergeStatusEXT = 4;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_ALIASING_EXT : VkSubpassMergeStatusEXT = 5;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_DEPENDENCIES_EXT : VkSubpassMergeStatusEXT = 6;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT : VkSubpassMergeStatusEXT = 7;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT : VkSubpassMergeStatusEXT = 8;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_INSUFFICIENT_STORAGE_EXT : VkSubpassMergeStatusEXT = 9;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_DEPTH_STENCIL_COUNT_EXT : VkSubpassMergeStatusEXT = 10;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT : VkSubpassMergeStatusEXT = 11;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SINGLE_SUBPASS_EXT : VkSubpassMergeStatusEXT = 12;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_UNSPECIFIED_EXT : VkSubpassMergeStatusEXT = 13;
}

pub type VkAccessFlagBits2 = u64;
pub mod VkAccessFlagBits2Value {
    use crate::VkAccessFlagBits2;

}

pub type VkPipelineStageFlagBits2 = u64;
pub mod VkPipelineStageFlagBits2Value {
    use crate::VkPipelineStageFlagBits2;

}

pub type VkSubmitFlagBits = u32;
pub mod VkSubmitFlagBitsValue {
    use crate::VkSubmitFlagBits;

    pub const VK_SUBMIT_PROTECTED_BIT : VkSubmitFlagBits = 1;
}

pub type VkEventCreateFlagBits = u32;
pub mod VkEventCreateFlagBitsValue {
    use crate::VkEventCreateFlagBits;

}

pub type VkPipelineLayoutCreateFlagBits = u32;
pub mod VkPipelineLayoutCreateFlagBitsValue {
    use crate::VkPipelineLayoutCreateFlagBits;

}

pub type VkSciSyncClientTypeNV = i32;
pub mod VkSciSyncClientTypeNVValue {
    use crate::VkSciSyncClientTypeNV;

    pub const VK_SCI_SYNC_CLIENT_TYPE_SIGNALER_NV : VkSciSyncClientTypeNV = 0;
    pub const VK_SCI_SYNC_CLIENT_TYPE_WAITER_NV : VkSciSyncClientTypeNV = 1;
    pub const VK_SCI_SYNC_CLIENT_TYPE_SIGNALER_WAITER_NV : VkSciSyncClientTypeNV = 2;
}

pub type VkSciSyncPrimitiveTypeNV = i32;
pub mod VkSciSyncPrimitiveTypeNVValue {
    use crate::VkSciSyncPrimitiveTypeNV;

    pub const VK_SCI_SYNC_PRIMITIVE_TYPE_FENCE_NV : VkSciSyncPrimitiveTypeNV = 0;
    pub const VK_SCI_SYNC_PRIMITIVE_TYPE_SEMAPHORE_NV : VkSciSyncPrimitiveTypeNV = 1;
}

pub type VkProvokingVertexModeEXT = i32;
pub mod VkProvokingVertexModeEXTValue {
    use crate::VkProvokingVertexModeEXT;

    pub const VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT : VkProvokingVertexModeEXT = 0;
    pub const VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT : VkProvokingVertexModeEXT = 1;
}

pub type VkPipelineCacheValidationVersion = i32;
pub mod VkPipelineCacheValidationVersionValue {
    use crate::VkPipelineCacheValidationVersion;

    pub const VK_PIPELINE_CACHE_VALIDATION_VERSION_SAFETY_CRITICAL_ONE : VkPipelineCacheValidationVersion = 1;
}

pub type VkAccelerationStructureMotionInstanceTypeNV = i32;
pub mod VkAccelerationStructureMotionInstanceTypeNVValue {
    use crate::VkAccelerationStructureMotionInstanceTypeNV;

    pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV : VkAccelerationStructureMotionInstanceTypeNV = 0;
    pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV : VkAccelerationStructureMotionInstanceTypeNV = 1;
    pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV : VkAccelerationStructureMotionInstanceTypeNV = 2;
}

pub type VkPipelineColorBlendStateCreateFlagBits = u32;
pub mod VkPipelineColorBlendStateCreateFlagBitsValue {
    use crate::VkPipelineColorBlendStateCreateFlagBits;

}

pub type VkPipelineDepthStencilStateCreateFlagBits = u32;
pub mod VkPipelineDepthStencilStateCreateFlagBitsValue {
    use crate::VkPipelineDepthStencilStateCreateFlagBits;

}

pub type VkGraphicsPipelineLibraryFlagBitsEXT = u32;
pub mod VkGraphicsPipelineLibraryFlagBitsEXTValue {
    use crate::VkGraphicsPipelineLibraryFlagBitsEXT;

    pub const VK_GRAPHICS_PIPELINE_LIBRARY_VERTEX_INPUT_INTERFACE_BIT_EXT : VkGraphicsPipelineLibraryFlagBitsEXT = 1;
    pub const VK_GRAPHICS_PIPELINE_LIBRARY_PRE_RASTERIZATION_SHADERS_BIT_EXT : VkGraphicsPipelineLibraryFlagBitsEXT = 2;
    pub const VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_SHADER_BIT_EXT : VkGraphicsPipelineLibraryFlagBitsEXT = 4;
    pub const VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_OUTPUT_INTERFACE_BIT_EXT : VkGraphicsPipelineLibraryFlagBitsEXT = 8;
}

pub type VkRenderingAttachmentFlagBitsKHR = u32;
pub mod VkRenderingAttachmentFlagBitsKHRValue {
    use crate::VkRenderingAttachmentFlagBitsKHR;

}

pub type VkResolveImageFlagBitsKHR = u32;
pub mod VkResolveImageFlagBitsKHRValue {
    use crate::VkResolveImageFlagBitsKHR;

}

pub type VkDeviceAddressBindingFlagBitsEXT = u32;
pub mod VkDeviceAddressBindingFlagBitsEXTValue {
    use crate::VkDeviceAddressBindingFlagBitsEXT;

    pub const VK_DEVICE_ADDRESS_BINDING_INTERNAL_OBJECT_BIT_EXT : VkDeviceAddressBindingFlagBitsEXT = 1;
}

pub type VkDeviceAddressBindingTypeEXT = i32;
pub mod VkDeviceAddressBindingTypeEXTValue {
    use crate::VkDeviceAddressBindingTypeEXT;

    pub const VK_DEVICE_ADDRESS_BINDING_TYPE_BIND_EXT : VkDeviceAddressBindingTypeEXT = 0;
    pub const VK_DEVICE_ADDRESS_BINDING_TYPE_UNBIND_EXT : VkDeviceAddressBindingTypeEXT = 1;
}

pub type VkFrameBoundaryFlagBitsEXT = u32;
pub mod VkFrameBoundaryFlagBitsEXTValue {
    use crate::VkFrameBoundaryFlagBitsEXT;

    pub const VK_FRAME_BOUNDARY_FRAME_END_BIT_EXT : VkFrameBoundaryFlagBitsEXT = 1;
}

pub type VkPresentScalingFlagBitsKHR = u32;
pub mod VkPresentScalingFlagBitsKHRValue {
    use crate::VkPresentScalingFlagBitsKHR;

    pub const VK_PRESENT_SCALING_ONE_TO_ONE_BIT_KHR : VkPresentScalingFlagBitsKHR = 1;
    pub const VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_KHR : VkPresentScalingFlagBitsKHR = 2;
    pub const VK_PRESENT_SCALING_STRETCH_BIT_KHR : VkPresentScalingFlagBitsKHR = 4;
}

pub type VkPresentGravityFlagBitsKHR = u32;
pub mod VkPresentGravityFlagBitsKHRValue {
    use crate::VkPresentGravityFlagBitsKHR;

    pub const VK_PRESENT_GRAVITY_MIN_BIT_KHR : VkPresentGravityFlagBitsKHR = 1;
    pub const VK_PRESENT_GRAVITY_MAX_BIT_KHR : VkPresentGravityFlagBitsKHR = 2;
    pub const VK_PRESENT_GRAVITY_CENTERED_BIT_KHR : VkPresentGravityFlagBitsKHR = 4;
}

pub type VkPhysicalDeviceSchedulingControlsFlagBitsARM = u64;
pub mod VkPhysicalDeviceSchedulingControlsFlagBitsARMValue {
    use crate::VkPhysicalDeviceSchedulingControlsFlagBitsARM;

}

pub type VkPresentStageFlagBitsEXT = u32;
pub mod VkPresentStageFlagBitsEXTValue {
    use crate::VkPresentStageFlagBitsEXT;

    pub const VK_PRESENT_STAGE_QUEUE_OPERATIONS_END_BIT_EXT : VkPresentStageFlagBitsEXT = 1;
    pub const VK_PRESENT_STAGE_REQUEST_DEQUEUED_BIT_EXT : VkPresentStageFlagBitsEXT = 2;
    pub const VK_PRESENT_STAGE_IMAGE_FIRST_PIXEL_OUT_BIT_EXT : VkPresentStageFlagBitsEXT = 4;
    pub const VK_PRESENT_STAGE_IMAGE_FIRST_PIXEL_VISIBLE_BIT_EXT : VkPresentStageFlagBitsEXT = 8;
}

pub type VkPastPresentationTimingFlagBitsEXT = u32;
pub mod VkPastPresentationTimingFlagBitsEXTValue {
    use crate::VkPastPresentationTimingFlagBitsEXT;

    pub const VK_PAST_PRESENTATION_TIMING_ALLOW_PARTIAL_RESULTS_BIT_EXT : VkPastPresentationTimingFlagBitsEXT = 1;
    pub const VK_PAST_PRESENTATION_TIMING_ALLOW_OUT_OF_ORDER_RESULTS_BIT_EXT : VkPastPresentationTimingFlagBitsEXT = 2;
}

pub type VkPresentTimingInfoFlagBitsEXT = u32;
pub mod VkPresentTimingInfoFlagBitsEXTValue {
    use crate::VkPresentTimingInfoFlagBitsEXT;

    pub const VK_PRESENT_TIMING_INFO_PRESENT_AT_RELATIVE_TIME_BIT_EXT : VkPresentTimingInfoFlagBitsEXT = 1;
    pub const VK_PRESENT_TIMING_INFO_PRESENT_AT_NEAREST_REFRESH_CYCLE_BIT_EXT : VkPresentTimingInfoFlagBitsEXT = 2;
}

pub type VkVideoCodecOperationFlagBitsKHR = u32;
pub mod VkVideoCodecOperationFlagBitsKHRValue {
    use crate::VkVideoCodecOperationFlagBitsKHR;

    pub const VK_VIDEO_CODEC_OPERATION_NONE_KHR : VkVideoCodecOperationFlagBitsKHR = 0;
}

pub type VkVideoChromaSubsamplingFlagBitsKHR = u32;
pub mod VkVideoChromaSubsamplingFlagBitsKHRValue {
    use crate::VkVideoChromaSubsamplingFlagBitsKHR;

    pub const VK_VIDEO_CHROMA_SUBSAMPLING_INVALID_KHR : VkVideoChromaSubsamplingFlagBitsKHR = 0;
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_MONOCHROME_BIT_KHR : VkVideoChromaSubsamplingFlagBitsKHR = 1;
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_420_BIT_KHR : VkVideoChromaSubsamplingFlagBitsKHR = 2;
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_422_BIT_KHR : VkVideoChromaSubsamplingFlagBitsKHR = 4;
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_444_BIT_KHR : VkVideoChromaSubsamplingFlagBitsKHR = 8;
}

pub type VkVideoComponentBitDepthFlagBitsKHR = u32;
pub mod VkVideoComponentBitDepthFlagBitsKHRValue {
    use crate::VkVideoComponentBitDepthFlagBitsKHR;

    pub const VK_VIDEO_COMPONENT_BIT_DEPTH_INVALID_KHR : VkVideoComponentBitDepthFlagBitsKHR = 0;
    pub const VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR : VkVideoComponentBitDepthFlagBitsKHR = 1;
    pub const VK_VIDEO_COMPONENT_BIT_DEPTH_10_BIT_KHR : VkVideoComponentBitDepthFlagBitsKHR = 4;
    pub const VK_VIDEO_COMPONENT_BIT_DEPTH_12_BIT_KHR : VkVideoComponentBitDepthFlagBitsKHR = 16;
}

pub type VkVideoCapabilityFlagBitsKHR = u32;
pub mod VkVideoCapabilityFlagBitsKHRValue {
    use crate::VkVideoCapabilityFlagBitsKHR;

    pub const VK_VIDEO_CAPABILITY_PROTECTED_CONTENT_BIT_KHR : VkVideoCapabilityFlagBitsKHR = 1;
    pub const VK_VIDEO_CAPABILITY_SEPARATE_REFERENCE_IMAGES_BIT_KHR : VkVideoCapabilityFlagBitsKHR = 2;
}

pub type VkVideoSessionCreateFlagBitsKHR = u32;
pub mod VkVideoSessionCreateFlagBitsKHRValue {
    use crate::VkVideoSessionCreateFlagBitsKHR;

    pub const VK_VIDEO_SESSION_CREATE_PROTECTED_CONTENT_BIT_KHR : VkVideoSessionCreateFlagBitsKHR = 1;
}

pub type VkVideoSessionParametersCreateFlagBitsKHR = u32;
pub mod VkVideoSessionParametersCreateFlagBitsKHRValue {
    use crate::VkVideoSessionParametersCreateFlagBitsKHR;

}

pub type VkVideoDecodeH264PictureLayoutFlagBitsKHR = u32;
pub mod VkVideoDecodeH264PictureLayoutFlagBitsKHRValue {
    use crate::VkVideoDecodeH264PictureLayoutFlagBitsKHR;

    pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_KHR : VkVideoDecodeH264PictureLayoutFlagBitsKHR = 0;
    pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_BIT_KHR : VkVideoDecodeH264PictureLayoutFlagBitsKHR = 1;
    pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_BIT_KHR : VkVideoDecodeH264PictureLayoutFlagBitsKHR = 2;
}

pub type VkVideoCodingControlFlagBitsKHR = u32;
pub mod VkVideoCodingControlFlagBitsKHRValue {
    use crate::VkVideoCodingControlFlagBitsKHR;

    pub const VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR : VkVideoCodingControlFlagBitsKHR = 1;
}

pub type VkQueryResultStatusKHR = i32;
pub mod VkQueryResultStatusKHRValue {
    use crate::VkQueryResultStatusKHR;

    pub const VK_QUERY_RESULT_STATUS_ERROR_KHR : VkQueryResultStatusKHR = -1;
    pub const VK_QUERY_RESULT_STATUS_NOT_READY_KHR : VkQueryResultStatusKHR = 0;
    pub const VK_QUERY_RESULT_STATUS_COMPLETE_KHR : VkQueryResultStatusKHR = 1;
}

pub type VkVideoDecodeUsageFlagBitsKHR = u32;
pub mod VkVideoDecodeUsageFlagBitsKHRValue {
    use crate::VkVideoDecodeUsageFlagBitsKHR;

    pub const VK_VIDEO_DECODE_USAGE_DEFAULT_KHR : VkVideoDecodeUsageFlagBitsKHR = 0;
    pub const VK_VIDEO_DECODE_USAGE_TRANSCODING_BIT_KHR : VkVideoDecodeUsageFlagBitsKHR = 1;
    pub const VK_VIDEO_DECODE_USAGE_OFFLINE_BIT_KHR : VkVideoDecodeUsageFlagBitsKHR = 2;
    pub const VK_VIDEO_DECODE_USAGE_STREAMING_BIT_KHR : VkVideoDecodeUsageFlagBitsKHR = 4;
}

pub type VkVideoDecodeCapabilityFlagBitsKHR = u32;
pub mod VkVideoDecodeCapabilityFlagBitsKHRValue {
    use crate::VkVideoDecodeCapabilityFlagBitsKHR;

    pub const VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR : VkVideoDecodeCapabilityFlagBitsKHR = 1;
    pub const VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR : VkVideoDecodeCapabilityFlagBitsKHR = 2;
}

pub type VkVideoEncodeFlagBitsKHR = u32;
pub mod VkVideoEncodeFlagBitsKHRValue {
    use crate::VkVideoEncodeFlagBitsKHR;

}

pub type VkVideoEncodeUsageFlagBitsKHR = u32;
pub mod VkVideoEncodeUsageFlagBitsKHRValue {
    use crate::VkVideoEncodeUsageFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_USAGE_DEFAULT_KHR : VkVideoEncodeUsageFlagBitsKHR = 0;
    pub const VK_VIDEO_ENCODE_USAGE_TRANSCODING_BIT_KHR : VkVideoEncodeUsageFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_USAGE_STREAMING_BIT_KHR : VkVideoEncodeUsageFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_USAGE_RECORDING_BIT_KHR : VkVideoEncodeUsageFlagBitsKHR = 4;
    pub const VK_VIDEO_ENCODE_USAGE_CONFERENCING_BIT_KHR : VkVideoEncodeUsageFlagBitsKHR = 8;
}

pub type VkVideoEncodeContentFlagBitsKHR = u32;
pub mod VkVideoEncodeContentFlagBitsKHRValue {
    use crate::VkVideoEncodeContentFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_CONTENT_DEFAULT_KHR : VkVideoEncodeContentFlagBitsKHR = 0;
    pub const VK_VIDEO_ENCODE_CONTENT_CAMERA_BIT_KHR : VkVideoEncodeContentFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_CONTENT_DESKTOP_BIT_KHR : VkVideoEncodeContentFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_CONTENT_RENDERED_BIT_KHR : VkVideoEncodeContentFlagBitsKHR = 4;
}

pub type VkVideoEncodeTuningModeKHR = i32;
pub mod VkVideoEncodeTuningModeKHRValue {
    use crate::VkVideoEncodeTuningModeKHR;

    pub const VK_VIDEO_ENCODE_TUNING_MODE_DEFAULT_KHR : VkVideoEncodeTuningModeKHR = 0;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_HIGH_QUALITY_KHR : VkVideoEncodeTuningModeKHR = 1;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_LOW_LATENCY_KHR : VkVideoEncodeTuningModeKHR = 2;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_ULTRA_LOW_LATENCY_KHR : VkVideoEncodeTuningModeKHR = 3;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_LOSSLESS_KHR : VkVideoEncodeTuningModeKHR = 4;
}

pub type VkVideoEncodeCapabilityFlagBitsKHR = u32;
pub mod VkVideoEncodeCapabilityFlagBitsKHRValue {
    use crate::VkVideoEncodeCapabilityFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_CAPABILITY_PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR : VkVideoEncodeCapabilityFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_CAPABILITY_INSUFFICIENT_BITSTREAM_BUFFER_RANGE_DETECTION_BIT_KHR : VkVideoEncodeCapabilityFlagBitsKHR = 2;
}

pub type VkVideoEncodeFeedbackFlagBitsKHR = u32;
pub mod VkVideoEncodeFeedbackFlagBitsKHRValue {
    use crate::VkVideoEncodeFeedbackFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_BUFFER_OFFSET_BIT_KHR : VkVideoEncodeFeedbackFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_BYTES_WRITTEN_BIT_KHR : VkVideoEncodeFeedbackFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_HAS_OVERRIDES_BIT_KHR : VkVideoEncodeFeedbackFlagBitsKHR = 4;
}

pub type VkVideoEncodePerPartitionFeedbackFlagBitsKHR = u32;
pub mod VkVideoEncodePerPartitionFeedbackFlagBitsKHRValue {
    use crate::VkVideoEncodePerPartitionFeedbackFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_STATUS_BIT_KHR : VkVideoEncodePerPartitionFeedbackFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_BITSTREAM_BUFFER_OFFSET_BIT_KHR : VkVideoEncodePerPartitionFeedbackFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_BITSTREAM_BYTES_WRITTEN_BIT_KHR : VkVideoEncodePerPartitionFeedbackFlagBitsKHR = 4;
}

pub type VkVideoEncodeRateControlModeFlagBitsKHR = u32;
pub mod VkVideoEncodeRateControlModeFlagBitsKHRValue {
    use crate::VkVideoEncodeRateControlModeFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_DEFAULT_KHR : VkVideoEncodeRateControlModeFlagBitsKHR = 0;
    pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_DISABLED_BIT_KHR : VkVideoEncodeRateControlModeFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_CBR_BIT_KHR : VkVideoEncodeRateControlModeFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR : VkVideoEncodeRateControlModeFlagBitsKHR = 4;
}

pub type VkVideoEncodeIntraRefreshModeFlagBitsKHR = u32;
pub mod VkVideoEncodeIntraRefreshModeFlagBitsKHRValue {
    use crate::VkVideoEncodeIntraRefreshModeFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_NONE_KHR : VkVideoEncodeIntraRefreshModeFlagBitsKHR = 0;
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_PER_PICTURE_PARTITION_BIT_KHR : VkVideoEncodeIntraRefreshModeFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_BASED_BIT_KHR : VkVideoEncodeIntraRefreshModeFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_ROW_BASED_BIT_KHR : VkVideoEncodeIntraRefreshModeFlagBitsKHR = 4;
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_COLUMN_BASED_BIT_KHR : VkVideoEncodeIntraRefreshModeFlagBitsKHR = 8;
}

pub type VkVideoEncodeH264CapabilityFlagBitsKHR = u32;
pub mod VkVideoEncodeH264CapabilityFlagBitsKHRValue {
    use crate::VkVideoEncodeH264CapabilityFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_HRD_COMPLIANCE_BIT_KHR : VkVideoEncodeH264CapabilityFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR : VkVideoEncodeH264CapabilityFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_ROW_UNALIGNED_SLICE_BIT_KHR : VkVideoEncodeH264CapabilityFlagBitsKHR = 4;
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_KHR : VkVideoEncodeH264CapabilityFlagBitsKHR = 8;
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR : VkVideoEncodeH264CapabilityFlagBitsKHR = 16;
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR : VkVideoEncodeH264CapabilityFlagBitsKHR = 32;
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR : VkVideoEncodeH264CapabilityFlagBitsKHR = 64;
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_PER_SLICE_CONSTANT_QP_BIT_KHR : VkVideoEncodeH264CapabilityFlagBitsKHR = 128;
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_GENERATE_PREFIX_NALU_BIT_KHR : VkVideoEncodeH264CapabilityFlagBitsKHR = 256;
}

pub type VkVideoEncodeH264StdFlagBitsKHR = u32;
pub mod VkVideoEncodeH264StdFlagBitsKHRValue {
    use crate::VkVideoEncodeH264StdFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_H264_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_H264_STD_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_H264_STD_SCALING_MATRIX_PRESENT_FLAG_SET_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 4;
    pub const VK_VIDEO_ENCODE_H264_STD_CHROMA_QP_INDEX_OFFSET_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 8;
    pub const VK_VIDEO_ENCODE_H264_STD_SECOND_CHROMA_QP_INDEX_OFFSET_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 16;
    pub const VK_VIDEO_ENCODE_H264_STD_PIC_INIT_QP_MINUS26_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 32;
    pub const VK_VIDEO_ENCODE_H264_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 64;
    pub const VK_VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_EXPLICIT_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 128;
    pub const VK_VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_IMPLICIT_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 256;
    pub const VK_VIDEO_ENCODE_H264_STD_TRANSFORM_8X8_MODE_FLAG_SET_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 512;
    pub const VK_VIDEO_ENCODE_H264_STD_DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 1024;
    pub const VK_VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_UNSET_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 2048;
    pub const VK_VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_SET_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 4096;
    pub const VK_VIDEO_ENCODE_H264_STD_DIRECT_8X8_INFERENCE_FLAG_UNSET_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 8192;
    pub const VK_VIDEO_ENCODE_H264_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 16384;
    pub const VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_DISABLED_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 32768;
    pub const VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_ENABLED_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 65536;
    pub const VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_PARTIAL_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 131072;
    pub const VK_VIDEO_ENCODE_H264_STD_SLICE_QP_DELTA_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 524288;
    pub const VK_VIDEO_ENCODE_H264_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR : VkVideoEncodeH264StdFlagBitsKHR = 1048576;
}

pub type VkVideoEncodeH264RateControlFlagBitsKHR = u32;
pub mod VkVideoEncodeH264RateControlFlagBitsKHRValue {
    use crate::VkVideoEncodeH264RateControlFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR : VkVideoEncodeH264RateControlFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_REGULAR_GOP_BIT_KHR : VkVideoEncodeH264RateControlFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR : VkVideoEncodeH264RateControlFlagBitsKHR = 4;
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR : VkVideoEncodeH264RateControlFlagBitsKHR = 8;
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR : VkVideoEncodeH264RateControlFlagBitsKHR = 16;
}

pub type VkHostImageCopyFlagBits = u32;
pub mod VkHostImageCopyFlagBitsValue {
    use crate::VkHostImageCopyFlagBits;

    pub const VK_HOST_IMAGE_COPY_MEMCPY_BIT : VkHostImageCopyFlagBits = 1;
}

pub type VkPartitionedAccelerationStructureOpTypeNV = i32;
pub mod VkPartitionedAccelerationStructureOpTypeNVValue {
    use crate::VkPartitionedAccelerationStructureOpTypeNV;

    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_WRITE_INSTANCE_NV : VkPartitionedAccelerationStructureOpTypeNV = 0;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_UPDATE_INSTANCE_NV : VkPartitionedAccelerationStructureOpTypeNV = 1;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_WRITE_PARTITION_TRANSLATION_NV : VkPartitionedAccelerationStructureOpTypeNV = 2;
}

pub type VkPartitionedAccelerationStructureInstanceFlagBitsNV = u32;
pub mod VkPartitionedAccelerationStructureInstanceFlagBitsNVValue {
    use crate::VkPartitionedAccelerationStructureInstanceFlagBitsNV;

    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FACING_CULL_DISABLE_BIT_NV : VkPartitionedAccelerationStructureInstanceFlagBitsNV = 1;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FLIP_FACING_BIT_NV : VkPartitionedAccelerationStructureInstanceFlagBitsNV = 2;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_OPAQUE_BIT_NV : VkPartitionedAccelerationStructureInstanceFlagBitsNV = 4;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_NO_OPAQUE_BIT_NV : VkPartitionedAccelerationStructureInstanceFlagBitsNV = 8;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV : VkPartitionedAccelerationStructureInstanceFlagBitsNV = 16;
}

pub type VkImageFormatConstraintsFlagBitsFUCHSIA = u32;
pub mod VkImageFormatConstraintsFlagBitsFUCHSIAValue {
    use crate::VkImageFormatConstraintsFlagBitsFUCHSIA;

}

pub type VkImageConstraintsInfoFlagBitsFUCHSIA = u32;
pub mod VkImageConstraintsInfoFlagBitsFUCHSIAValue {
    use crate::VkImageConstraintsInfoFlagBitsFUCHSIA;

    pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA : VkImageConstraintsInfoFlagBitsFUCHSIA = 1;
    pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA : VkImageConstraintsInfoFlagBitsFUCHSIA = 2;
    pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA : VkImageConstraintsInfoFlagBitsFUCHSIA = 4;
    pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA : VkImageConstraintsInfoFlagBitsFUCHSIA = 8;
    pub const VK_IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA : VkImageConstraintsInfoFlagBitsFUCHSIA = 16;
}

pub type VkFormatFeatureFlagBits2 = u64;
pub mod VkFormatFeatureFlagBits2Value {
    use crate::VkFormatFeatureFlagBits2;

}

pub type VkFormatFeatureFlagBits4KHR = u64;
pub mod VkFormatFeatureFlagBits4KHRValue {
    use crate::VkFormatFeatureFlagBits4KHR;

}

pub type VkRenderingFlagBits = u32;
pub mod VkRenderingFlagBitsValue {
    use crate::VkRenderingFlagBits;

    pub const VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT : VkRenderingFlagBits = 1;
    pub const VK_RENDERING_SUSPENDING_BIT : VkRenderingFlagBits = 2;
    pub const VK_RENDERING_RESUMING_BIT : VkRenderingFlagBits = 4;
}

pub type VkVideoEncodeH265CapabilityFlagBitsKHR = u32;
pub mod VkVideoEncodeH265CapabilityFlagBitsKHRValue {
    use crate::VkVideoEncodeH265CapabilityFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_HRD_COMPLIANCE_BIT_KHR : VkVideoEncodeH265CapabilityFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR : VkVideoEncodeH265CapabilityFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_ROW_UNALIGNED_SLICE_SEGMENT_BIT_KHR : VkVideoEncodeH265CapabilityFlagBitsKHR = 4;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_SLICE_SEGMENT_TYPE_BIT_KHR : VkVideoEncodeH265CapabilityFlagBitsKHR = 8;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR : VkVideoEncodeH265CapabilityFlagBitsKHR = 16;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR : VkVideoEncodeH265CapabilityFlagBitsKHR = 32;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR : VkVideoEncodeH265CapabilityFlagBitsKHR = 64;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_PER_SLICE_SEGMENT_CONSTANT_QP_BIT_KHR : VkVideoEncodeH265CapabilityFlagBitsKHR = 128;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILES_PER_SLICE_SEGMENT_BIT_KHR : VkVideoEncodeH265CapabilityFlagBitsKHR = 256;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_SLICE_SEGMENTS_PER_TILE_BIT_KHR : VkVideoEncodeH265CapabilityFlagBitsKHR = 512;
}

pub type VkVideoEncodeH265StdFlagBitsKHR = u32;
pub mod VkVideoEncodeH265StdFlagBitsKHRValue {
    use crate::VkVideoEncodeH265StdFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_H265_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_H265_STD_SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_H265_STD_SCALING_LIST_DATA_PRESENT_FLAG_SET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 4;
    pub const VK_VIDEO_ENCODE_H265_STD_PCM_ENABLED_FLAG_SET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 8;
    pub const VK_VIDEO_ENCODE_H265_STD_SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 16;
    pub const VK_VIDEO_ENCODE_H265_STD_INIT_QP_MINUS26_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 32;
    pub const VK_VIDEO_ENCODE_H265_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 64;
    pub const VK_VIDEO_ENCODE_H265_STD_WEIGHTED_BIPRED_FLAG_SET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 128;
    pub const VK_VIDEO_ENCODE_H265_STD_LOG2_PARALLEL_MERGE_LEVEL_MINUS2_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 256;
    pub const VK_VIDEO_ENCODE_H265_STD_SIGN_DATA_HIDING_ENABLED_FLAG_SET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 512;
    pub const VK_VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_SET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 1024;
    pub const VK_VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_UNSET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 2048;
    pub const VK_VIDEO_ENCODE_H265_STD_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 4096;
    pub const VK_VIDEO_ENCODE_H265_STD_TRANSQUANT_BYPASS_ENABLED_FLAG_SET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 8192;
    pub const VK_VIDEO_ENCODE_H265_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 16384;
    pub const VK_VIDEO_ENCODE_H265_STD_ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 32768;
    pub const VK_VIDEO_ENCODE_H265_STD_DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 65536;
    pub const VK_VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 131072;
    pub const VK_VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENT_FLAG_SET_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 262144;
    pub const VK_VIDEO_ENCODE_H265_STD_SLICE_QP_DELTA_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 524288;
    pub const VK_VIDEO_ENCODE_H265_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR : VkVideoEncodeH265StdFlagBitsKHR = 1048576;
}

pub type VkVideoEncodeH265RateControlFlagBitsKHR = u32;
pub mod VkVideoEncodeH265RateControlFlagBitsKHRValue {
    use crate::VkVideoEncodeH265RateControlFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR : VkVideoEncodeH265RateControlFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_REGULAR_GOP_BIT_KHR : VkVideoEncodeH265RateControlFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR : VkVideoEncodeH265RateControlFlagBitsKHR = 4;
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR : VkVideoEncodeH265RateControlFlagBitsKHR = 8;
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_TEMPORAL_SUB_LAYER_PATTERN_DYADIC_BIT_KHR : VkVideoEncodeH265RateControlFlagBitsKHR = 16;
}

pub type VkVideoEncodeH265CtbSizeFlagBitsKHR = u32;
pub mod VkVideoEncodeH265CtbSizeFlagBitsKHRValue {
    use crate::VkVideoEncodeH265CtbSizeFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_16_BIT_KHR : VkVideoEncodeH265CtbSizeFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_32_BIT_KHR : VkVideoEncodeH265CtbSizeFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_64_BIT_KHR : VkVideoEncodeH265CtbSizeFlagBitsKHR = 4;
}

pub type VkVideoEncodeH265TransformBlockSizeFlagBitsKHR = u32;
pub mod VkVideoEncodeH265TransformBlockSizeFlagBitsKHRValue {
    use crate::VkVideoEncodeH265TransformBlockSizeFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_4_BIT_KHR : VkVideoEncodeH265TransformBlockSizeFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_8_BIT_KHR : VkVideoEncodeH265TransformBlockSizeFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_16_BIT_KHR : VkVideoEncodeH265TransformBlockSizeFlagBitsKHR = 4;
    pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_32_BIT_KHR : VkVideoEncodeH265TransformBlockSizeFlagBitsKHR = 8;
}

pub type VkVideoEncodeAV1CapabilityFlagBitsKHR = u32;
pub mod VkVideoEncodeAV1CapabilityFlagBitsKHRValue {
    use crate::VkVideoEncodeAV1CapabilityFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_BIT_KHR : VkVideoEncodeAV1CapabilityFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_GENERATE_OBU_EXTENSION_HEADER_BIT_KHR : VkVideoEncodeAV1CapabilityFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_PRIMARY_REFERENCE_CDF_ONLY_BIT_KHR : VkVideoEncodeAV1CapabilityFlagBitsKHR = 4;
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_FRAME_SIZE_OVERRIDE_BIT_KHR : VkVideoEncodeAV1CapabilityFlagBitsKHR = 8;
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_MOTION_VECTOR_SCALING_BIT_KHR : VkVideoEncodeAV1CapabilityFlagBitsKHR = 16;
}

pub type VkVideoEncodeAV1StdFlagBitsKHR = u32;
pub mod VkVideoEncodeAV1StdFlagBitsKHRValue {
    use crate::VkVideoEncodeAV1StdFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_AV1_STD_UNIFORM_TILE_SPACING_FLAG_SET_BIT_KHR : VkVideoEncodeAV1StdFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_AV1_STD_SKIP_MODE_PRESENT_UNSET_BIT_KHR : VkVideoEncodeAV1StdFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_AV1_STD_PRIMARY_REF_FRAME_BIT_KHR : VkVideoEncodeAV1StdFlagBitsKHR = 4;
    pub const VK_VIDEO_ENCODE_AV1_STD_DELTA_Q_BIT_KHR : VkVideoEncodeAV1StdFlagBitsKHR = 8;
}

pub type VkVideoEncodeAV1RateControlFlagBitsKHR = u32;
pub mod VkVideoEncodeAV1RateControlFlagBitsKHRValue {
    use crate::VkVideoEncodeAV1RateControlFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REGULAR_GOP_BIT_KHR : VkVideoEncodeAV1RateControlFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR : VkVideoEncodeAV1RateControlFlagBitsKHR = 2;
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR : VkVideoEncodeAV1RateControlFlagBitsKHR = 4;
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR : VkVideoEncodeAV1RateControlFlagBitsKHR = 8;
}

pub type VkVideoEncodeAV1SuperblockSizeFlagBitsKHR = u32;
pub mod VkVideoEncodeAV1SuperblockSizeFlagBitsKHRValue {
    use crate::VkVideoEncodeAV1SuperblockSizeFlagBitsKHR;

    pub const VK_VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_64_BIT_KHR : VkVideoEncodeAV1SuperblockSizeFlagBitsKHR = 1;
    pub const VK_VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_128_BIT_KHR : VkVideoEncodeAV1SuperblockSizeFlagBitsKHR = 2;
}

pub type VkVideoEncodeAV1PredictionModeKHR = i32;
pub mod VkVideoEncodeAV1PredictionModeKHRValue {
    use crate::VkVideoEncodeAV1PredictionModeKHR;

    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_INTRA_ONLY_KHR : VkVideoEncodeAV1PredictionModeKHR = 0;
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_SINGLE_REFERENCE_KHR : VkVideoEncodeAV1PredictionModeKHR = 1;
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_UNIDIRECTIONAL_COMPOUND_KHR : VkVideoEncodeAV1PredictionModeKHR = 2;
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_BIDIRECTIONAL_COMPOUND_KHR : VkVideoEncodeAV1PredictionModeKHR = 3;
}

pub type VkVideoEncodeAV1RateControlGroupKHR = i32;
pub mod VkVideoEncodeAV1RateControlGroupKHRValue {
    use crate::VkVideoEncodeAV1RateControlGroupKHR;

    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_INTRA_KHR : VkVideoEncodeAV1RateControlGroupKHR = 0;
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_PREDICTIVE_KHR : VkVideoEncodeAV1RateControlGroupKHR = 1;
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_BIPREDICTIVE_KHR : VkVideoEncodeAV1RateControlGroupKHR = 2;
}

pub type VkExportMetalObjectTypeFlagBitsEXT = u32;
pub mod VkExportMetalObjectTypeFlagBitsEXTValue {
    use crate::VkExportMetalObjectTypeFlagBitsEXT;

    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_DEVICE_BIT_EXT : VkExportMetalObjectTypeFlagBitsEXT = 1;
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_COMMAND_QUEUE_BIT_EXT : VkExportMetalObjectTypeFlagBitsEXT = 2;
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_BUFFER_BIT_EXT : VkExportMetalObjectTypeFlagBitsEXT = 4;
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_TEXTURE_BIT_EXT : VkExportMetalObjectTypeFlagBitsEXT = 8;
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_IOSURFACE_BIT_EXT : VkExportMetalObjectTypeFlagBitsEXT = 16;
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_SHARED_EVENT_BIT_EXT : VkExportMetalObjectTypeFlagBitsEXT = 32;
}

pub type VkInstanceCreateFlagBits = u32;
pub mod VkInstanceCreateFlagBitsValue {
    use crate::VkInstanceCreateFlagBits;

}

pub type VkImageCompressionFlagBitsEXT = u32;
pub mod VkImageCompressionFlagBitsEXTValue {
    use crate::VkImageCompressionFlagBitsEXT;

    pub const VK_IMAGE_COMPRESSION_DEFAULT_EXT : VkImageCompressionFlagBitsEXT = 0;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_DEFAULT_EXT : VkImageCompressionFlagBitsEXT = 1;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_EXPLICIT_EXT : VkImageCompressionFlagBitsEXT = 2;
    pub const VK_IMAGE_COMPRESSION_DISABLED_EXT : VkImageCompressionFlagBitsEXT = 4;
}

pub type VkImageCompressionFixedRateFlagBitsEXT = u32;
pub mod VkImageCompressionFixedRateFlagBitsEXTValue {
    use crate::VkImageCompressionFixedRateFlagBitsEXT;

    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_NONE_EXT : VkImageCompressionFixedRateFlagBitsEXT = 0;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_1BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 1;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_2BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 2;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_3BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 4;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_4BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 8;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_5BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 16;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_6BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 32;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_7BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 64;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_8BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 128;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_9BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 256;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_10BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 512;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_11BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 1024;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_12BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 2048;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_13BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 4096;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_14BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 8192;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_15BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 16384;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_16BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 32768;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_17BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 65536;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_18BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 131072;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_19BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 262144;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_20BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 524288;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_21BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 1048576;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_22BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 2097152;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_23BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 4194304;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_24BPC_BIT_EXT : VkImageCompressionFixedRateFlagBitsEXT = 8388608;
}

pub type VkPipelineRobustnessBufferBehavior = i32;
pub mod VkPipelineRobustnessBufferBehaviorValue {
    use crate::VkPipelineRobustnessBufferBehavior;

    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DEVICE_DEFAULT : VkPipelineRobustnessBufferBehavior = 0;
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DISABLED : VkPipelineRobustnessBufferBehavior = 1;
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS : VkPipelineRobustnessBufferBehavior = 2;
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_2 : VkPipelineRobustnessBufferBehavior = 3;
}

pub type VkPipelineRobustnessImageBehavior = i32;
pub mod VkPipelineRobustnessImageBehaviorValue {
    use crate::VkPipelineRobustnessImageBehavior;

    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DEVICE_DEFAULT : VkPipelineRobustnessImageBehavior = 0;
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DISABLED : VkPipelineRobustnessImageBehavior = 1;
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS : VkPipelineRobustnessImageBehavior = 2;
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_2 : VkPipelineRobustnessImageBehavior = 3;
}

pub type VkOpticalFlowGridSizeFlagBitsNV = u32;
pub mod VkOpticalFlowGridSizeFlagBitsNVValue {
    use crate::VkOpticalFlowGridSizeFlagBitsNV;

    pub const VK_OPTICAL_FLOW_GRID_SIZE_UNKNOWN_NV : VkOpticalFlowGridSizeFlagBitsNV = 0;
    pub const VK_OPTICAL_FLOW_GRID_SIZE_1X1_BIT_NV : VkOpticalFlowGridSizeFlagBitsNV = 1;
    pub const VK_OPTICAL_FLOW_GRID_SIZE_2X2_BIT_NV : VkOpticalFlowGridSizeFlagBitsNV = 2;
    pub const VK_OPTICAL_FLOW_GRID_SIZE_4X4_BIT_NV : VkOpticalFlowGridSizeFlagBitsNV = 4;
    pub const VK_OPTICAL_FLOW_GRID_SIZE_8X8_BIT_NV : VkOpticalFlowGridSizeFlagBitsNV = 8;
}

pub type VkOpticalFlowUsageFlagBitsNV = u32;
pub mod VkOpticalFlowUsageFlagBitsNVValue {
    use crate::VkOpticalFlowUsageFlagBitsNV;

    pub const VK_OPTICAL_FLOW_USAGE_UNKNOWN_NV : VkOpticalFlowUsageFlagBitsNV = 0;
    pub const VK_OPTICAL_FLOW_USAGE_INPUT_BIT_NV : VkOpticalFlowUsageFlagBitsNV = 1;
    pub const VK_OPTICAL_FLOW_USAGE_OUTPUT_BIT_NV : VkOpticalFlowUsageFlagBitsNV = 2;
    pub const VK_OPTICAL_FLOW_USAGE_HINT_BIT_NV : VkOpticalFlowUsageFlagBitsNV = 4;
    pub const VK_OPTICAL_FLOW_USAGE_COST_BIT_NV : VkOpticalFlowUsageFlagBitsNV = 8;
    pub const VK_OPTICAL_FLOW_USAGE_GLOBAL_FLOW_BIT_NV : VkOpticalFlowUsageFlagBitsNV = 16;
}

pub type VkOpticalFlowPerformanceLevelNV = i32;
pub mod VkOpticalFlowPerformanceLevelNVValue {
    use crate::VkOpticalFlowPerformanceLevelNV;

    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_UNKNOWN_NV : VkOpticalFlowPerformanceLevelNV = 0;
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_SLOW_NV : VkOpticalFlowPerformanceLevelNV = 1;
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_MEDIUM_NV : VkOpticalFlowPerformanceLevelNV = 2;
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_FAST_NV : VkOpticalFlowPerformanceLevelNV = 3;
}

pub type VkOpticalFlowSessionBindingPointNV = i32;
pub mod VkOpticalFlowSessionBindingPointNVValue {
    use crate::VkOpticalFlowSessionBindingPointNV;

    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_UNKNOWN_NV : VkOpticalFlowSessionBindingPointNV = 0;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_INPUT_NV : VkOpticalFlowSessionBindingPointNV = 1;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_REFERENCE_NV : VkOpticalFlowSessionBindingPointNV = 2;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_HINT_NV : VkOpticalFlowSessionBindingPointNV = 3;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_FLOW_VECTOR_NV : VkOpticalFlowSessionBindingPointNV = 4;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_FLOW_VECTOR_NV : VkOpticalFlowSessionBindingPointNV = 5;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_COST_NV : VkOpticalFlowSessionBindingPointNV = 6;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_COST_NV : VkOpticalFlowSessionBindingPointNV = 7;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_GLOBAL_FLOW_NV : VkOpticalFlowSessionBindingPointNV = 8;
}

pub type VkOpticalFlowSessionCreateFlagBitsNV = u32;
pub mod VkOpticalFlowSessionCreateFlagBitsNVValue {
    use crate::VkOpticalFlowSessionCreateFlagBitsNV;

    pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_HINT_BIT_NV : VkOpticalFlowSessionCreateFlagBitsNV = 1;
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_COST_BIT_NV : VkOpticalFlowSessionCreateFlagBitsNV = 2;
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_GLOBAL_FLOW_BIT_NV : VkOpticalFlowSessionCreateFlagBitsNV = 4;
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_ALLOW_REGIONS_BIT_NV : VkOpticalFlowSessionCreateFlagBitsNV = 8;
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_BOTH_DIRECTIONS_BIT_NV : VkOpticalFlowSessionCreateFlagBitsNV = 16;
}

pub type VkOpticalFlowExecuteFlagBitsNV = u32;
pub mod VkOpticalFlowExecuteFlagBitsNVValue {
    use crate::VkOpticalFlowExecuteFlagBitsNV;

    pub const VK_OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_NV : VkOpticalFlowExecuteFlagBitsNV = 1;
}

pub type VkMicromapTypeEXT = i32;
pub mod VkMicromapTypeEXTValue {
    use crate::VkMicromapTypeEXT;

    pub const VK_MICROMAP_TYPE_OPACITY_MICROMAP_EXT : VkMicromapTypeEXT = 0;
}

pub type VkBuildMicromapFlagBitsEXT = u32;
pub mod VkBuildMicromapFlagBitsEXTValue {
    use crate::VkBuildMicromapFlagBitsEXT;

    pub const VK_BUILD_MICROMAP_PREFER_FAST_TRACE_BIT_EXT : VkBuildMicromapFlagBitsEXT = 1;
    pub const VK_BUILD_MICROMAP_PREFER_FAST_BUILD_BIT_EXT : VkBuildMicromapFlagBitsEXT = 2;
    pub const VK_BUILD_MICROMAP_ALLOW_COMPACTION_BIT_EXT : VkBuildMicromapFlagBitsEXT = 4;
}

pub type VkMicromapCreateFlagBitsEXT = u32;
pub mod VkMicromapCreateFlagBitsEXTValue {
    use crate::VkMicromapCreateFlagBitsEXT;

    pub const VK_MICROMAP_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT : VkMicromapCreateFlagBitsEXT = 1;
}

pub type VkCopyMicromapModeEXT = i32;
pub mod VkCopyMicromapModeEXTValue {
    use crate::VkCopyMicromapModeEXT;

    pub const VK_COPY_MICROMAP_MODE_CLONE_EXT : VkCopyMicromapModeEXT = 0;
    pub const VK_COPY_MICROMAP_MODE_SERIALIZE_EXT : VkCopyMicromapModeEXT = 1;
    pub const VK_COPY_MICROMAP_MODE_DESERIALIZE_EXT : VkCopyMicromapModeEXT = 2;
    pub const VK_COPY_MICROMAP_MODE_COMPACT_EXT : VkCopyMicromapModeEXT = 3;
}

pub type VkBuildMicromapModeEXT = i32;
pub mod VkBuildMicromapModeEXTValue {
    use crate::VkBuildMicromapModeEXT;

    pub const VK_BUILD_MICROMAP_MODE_BUILD_EXT : VkBuildMicromapModeEXT = 0;
}

pub type VkOpacityMicromapFormatKHR = i32;
pub mod VkOpacityMicromapFormatKHRValue {
    use crate::VkOpacityMicromapFormatKHR;

    pub const VK_OPACITY_MICROMAP_FORMAT_2_STATE_KHR : VkOpacityMicromapFormatKHR = 1;
    pub const VK_OPACITY_MICROMAP_FORMAT_4_STATE_KHR : VkOpacityMicromapFormatKHR = 2;
}

pub type VkOpacityMicromapSpecialIndexKHR = i32;
pub mod VkOpacityMicromapSpecialIndexKHRValue {
    use crate::VkOpacityMicromapSpecialIndexKHR;

    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_TRANSPARENT_KHR : VkOpacityMicromapSpecialIndexKHR = -1;
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_OPAQUE_KHR : VkOpacityMicromapSpecialIndexKHR = -2;
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_TRANSPARENT_KHR : VkOpacityMicromapSpecialIndexKHR = -3;
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_OPAQUE_KHR : VkOpacityMicromapSpecialIndexKHR = -4;
}

pub type VkAccelerationStructureSerializedBlockTypeKHR = i32;
pub mod VkAccelerationStructureSerializedBlockTypeKHRValue {
    use crate::VkAccelerationStructureSerializedBlockTypeKHR;

    pub const VK_ACCELERATION_STRUCTURE_SERIALIZED_BLOCK_TYPE_OPACITY_MICROMAP_KHR : VkAccelerationStructureSerializedBlockTypeKHR = 0;
}

pub type VkDepthBiasRepresentationEXT = i32;
pub mod VkDepthBiasRepresentationEXTValue {
    use crate::VkDepthBiasRepresentationEXT;

    pub const VK_DEPTH_BIAS_REPRESENTATION_LEAST_REPRESENTABLE_VALUE_FORMAT_EXT : VkDepthBiasRepresentationEXT = 0;
    pub const VK_DEPTH_BIAS_REPRESENTATION_LEAST_REPRESENTABLE_VALUE_FORCE_UNORM_EXT : VkDepthBiasRepresentationEXT = 1;
    pub const VK_DEPTH_BIAS_REPRESENTATION_FLOAT_EXT : VkDepthBiasRepresentationEXT = 2;
}

pub type VkDeviceFaultFlagBitsKHR = u32;
pub mod VkDeviceFaultFlagBitsKHRValue {
    use crate::VkDeviceFaultFlagBitsKHR;

    pub const VK_DEVICE_FAULT_FLAG_DEVICE_LOST_KHR : VkDeviceFaultFlagBitsKHR = 1;
    pub const VK_DEVICE_FAULT_FLAG_MEMORY_ADDRESS_KHR : VkDeviceFaultFlagBitsKHR = 2;
    pub const VK_DEVICE_FAULT_FLAG_INSTRUCTION_ADDRESS_KHR : VkDeviceFaultFlagBitsKHR = 4;
    pub const VK_DEVICE_FAULT_FLAG_VENDOR_KHR : VkDeviceFaultFlagBitsKHR = 8;
    pub const VK_DEVICE_FAULT_FLAG_WATCHDOG_TIMEOUT_KHR : VkDeviceFaultFlagBitsKHR = 16;
    pub const VK_DEVICE_FAULT_FLAG_OVERFLOW_KHR : VkDeviceFaultFlagBitsKHR = 32;
}

pub type VkDeviceFaultAddressTypeKHR = i32;
pub mod VkDeviceFaultAddressTypeKHRValue {
    use crate::VkDeviceFaultAddressTypeKHR;

    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_NONE_KHR : VkDeviceFaultAddressTypeKHR = 0;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_READ_INVALID_KHR : VkDeviceFaultAddressTypeKHR = 1;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_WRITE_INVALID_KHR : VkDeviceFaultAddressTypeKHR = 2;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_EXECUTE_INVALID_KHR : VkDeviceFaultAddressTypeKHR = 3;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_UNKNOWN_KHR : VkDeviceFaultAddressTypeKHR = 4;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_INVALID_KHR : VkDeviceFaultAddressTypeKHR = 5;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_FAULT_KHR : VkDeviceFaultAddressTypeKHR = 6;
}

pub type VkDeviceFaultVendorBinaryHeaderVersionKHR = i32;
pub mod VkDeviceFaultVendorBinaryHeaderVersionKHRValue {
    use crate::VkDeviceFaultVendorBinaryHeaderVersionKHR;

    pub const VK_DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_ONE_KHR : VkDeviceFaultVendorBinaryHeaderVersionKHR = 1;
}

pub type VkIndirectCommandsLayoutUsageFlagBitsEXT = u32;
pub mod VkIndirectCommandsLayoutUsageFlagBitsEXTValue {
    use crate::VkIndirectCommandsLayoutUsageFlagBitsEXT;

    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_EXT : VkIndirectCommandsLayoutUsageFlagBitsEXT = 1;
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_EXT : VkIndirectCommandsLayoutUsageFlagBitsEXT = 2;
}

pub type VkIndirectExecutionSetInfoTypeEXT = i32;
pub mod VkIndirectExecutionSetInfoTypeEXTValue {
    use crate::VkIndirectExecutionSetInfoTypeEXT;

    pub const VK_INDIRECT_EXECUTION_SET_INFO_TYPE_PIPELINES_EXT : VkIndirectExecutionSetInfoTypeEXT = 0;
    pub const VK_INDIRECT_EXECUTION_SET_INFO_TYPE_SHADER_OBJECTS_EXT : VkIndirectExecutionSetInfoTypeEXT = 1;
}

pub type VkIndirectCommandsInputModeFlagBitsEXT = u32;
pub mod VkIndirectCommandsInputModeFlagBitsEXTValue {
    use crate::VkIndirectCommandsInputModeFlagBitsEXT;

    pub const VK_INDIRECT_COMMANDS_INPUT_MODE_VULKAN_INDEX_BUFFER_EXT : VkIndirectCommandsInputModeFlagBitsEXT = 1;
    pub const VK_INDIRECT_COMMANDS_INPUT_MODE_DXGI_INDEX_BUFFER_EXT : VkIndirectCommandsInputModeFlagBitsEXT = 2;
}

pub type VkIndirectCommandsTokenTypeEXT = i32;
pub mod VkIndirectCommandsTokenTypeEXTValue {
    use crate::VkIndirectCommandsTokenTypeEXT;

    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_EXECUTION_SET_EXT : VkIndirectCommandsTokenTypeEXT = 0;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_EXT : VkIndirectCommandsTokenTypeEXT = 1;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_SEQUENCE_INDEX_EXT : VkIndirectCommandsTokenTypeEXT = 2;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_EXT : VkIndirectCommandsTokenTypeEXT = 3;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_EXT : VkIndirectCommandsTokenTypeEXT = 4;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_EXT : VkIndirectCommandsTokenTypeEXT = 5;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_EXT : VkIndirectCommandsTokenTypeEXT = 6;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_COUNT_EXT : VkIndirectCommandsTokenTypeEXT = 7;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_COUNT_EXT : VkIndirectCommandsTokenTypeEXT = 8;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DISPATCH_EXT : VkIndirectCommandsTokenTypeEXT = 9;
}

pub type VkDisplacementMicromapFormatNV = i32;
pub mod VkDisplacementMicromapFormatNVValue {
    use crate::VkDisplacementMicromapFormatNV;

    pub const VK_DISPLACEMENT_MICROMAP_FORMAT_64_TRIANGLES_64_BYTES_NV : VkDisplacementMicromapFormatNV = 1;
    pub const VK_DISPLACEMENT_MICROMAP_FORMAT_256_TRIANGLES_128_BYTES_NV : VkDisplacementMicromapFormatNV = 2;
    pub const VK_DISPLACEMENT_MICROMAP_FORMAT_1024_TRIANGLES_128_BYTES_NV : VkDisplacementMicromapFormatNV = 3;
}

pub type VkShaderCreateFlagBitsEXT = u32;
pub mod VkShaderCreateFlagBitsEXTValue {
    use crate::VkShaderCreateFlagBitsEXT;

    pub const VK_SHADER_CREATE_LINK_STAGE_BIT_EXT : VkShaderCreateFlagBitsEXT = 1;
}

pub type VkShaderCodeTypeEXT = i32;
pub mod VkShaderCodeTypeEXTValue {
    use crate::VkShaderCodeTypeEXT;

    pub const VK_SHADER_CODE_TYPE_BINARY_EXT : VkShaderCodeTypeEXT = 0;
    pub const VK_SHADER_CODE_TYPE_SPIRV_EXT : VkShaderCodeTypeEXT = 1;
}

pub type VkScopeKHR = i32;
pub mod VkScopeKHRValue {
    use crate::VkScopeKHR;

    pub const VK_SCOPE_DEVICE_KHR : VkScopeKHR = 1;
    pub const VK_SCOPE_WORKGROUP_KHR : VkScopeKHR = 2;
    pub const VK_SCOPE_SUBGROUP_KHR : VkScopeKHR = 3;
    pub const VK_SCOPE_QUEUE_FAMILY_KHR : VkScopeKHR = 5;
}

pub type VkComponentTypeKHR = i32;
pub mod VkComponentTypeKHRValue {
    use crate::VkComponentTypeKHR;

    pub const VK_COMPONENT_TYPE_FLOAT16_KHR : VkComponentTypeKHR = 0;
    pub const VK_COMPONENT_TYPE_FLOAT32_KHR : VkComponentTypeKHR = 1;
    pub const VK_COMPONENT_TYPE_FLOAT64_KHR : VkComponentTypeKHR = 2;
    pub const VK_COMPONENT_TYPE_SINT8_KHR : VkComponentTypeKHR = 3;
    pub const VK_COMPONENT_TYPE_SINT16_KHR : VkComponentTypeKHR = 4;
    pub const VK_COMPONENT_TYPE_SINT32_KHR : VkComponentTypeKHR = 5;
    pub const VK_COMPONENT_TYPE_SINT64_KHR : VkComponentTypeKHR = 6;
    pub const VK_COMPONENT_TYPE_UINT8_KHR : VkComponentTypeKHR = 7;
    pub const VK_COMPONENT_TYPE_UINT16_KHR : VkComponentTypeKHR = 8;
    pub const VK_COMPONENT_TYPE_UINT32_KHR : VkComponentTypeKHR = 9;
    pub const VK_COMPONENT_TYPE_UINT64_KHR : VkComponentTypeKHR = 10;
}

pub type VkCubicFilterWeightsQCOM = i32;
pub mod VkCubicFilterWeightsQCOMValue {
    use crate::VkCubicFilterWeightsQCOM;

    pub const VK_CUBIC_FILTER_WEIGHTS_CATMULL_ROM_QCOM : VkCubicFilterWeightsQCOM = 0;
    pub const VK_CUBIC_FILTER_WEIGHTS_ZERO_TANGENT_CARDINAL_QCOM : VkCubicFilterWeightsQCOM = 1;
    pub const VK_CUBIC_FILTER_WEIGHTS_B_SPLINE_QCOM : VkCubicFilterWeightsQCOM = 2;
    pub const VK_CUBIC_FILTER_WEIGHTS_MITCHELL_NETRAVALI_QCOM : VkCubicFilterWeightsQCOM = 3;
}

pub type VkBlockMatchWindowCompareModeQCOM = i32;
pub mod VkBlockMatchWindowCompareModeQCOMValue {
    use crate::VkBlockMatchWindowCompareModeQCOM;

    pub const VK_BLOCK_MATCH_WINDOW_COMPARE_MODE_MIN_QCOM : VkBlockMatchWindowCompareModeQCOM = 0;
    pub const VK_BLOCK_MATCH_WINDOW_COMPARE_MODE_MAX_QCOM : VkBlockMatchWindowCompareModeQCOM = 1;
}

pub type VkPhysicalDeviceLayeredApiKHR = i32;
pub mod VkPhysicalDeviceLayeredApiKHRValue {
    use crate::VkPhysicalDeviceLayeredApiKHR;

    pub const VK_PHYSICAL_DEVICE_LAYERED_API_VULKAN_KHR : VkPhysicalDeviceLayeredApiKHR = 0;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_D3D12_KHR : VkPhysicalDeviceLayeredApiKHR = 1;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_METAL_KHR : VkPhysicalDeviceLayeredApiKHR = 2;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_OPENGL_KHR : VkPhysicalDeviceLayeredApiKHR = 3;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_OPENGLES_KHR : VkPhysicalDeviceLayeredApiKHR = 4;
}

pub type VkLayeredDriverUnderlyingApiMSFT = i32;
pub mod VkLayeredDriverUnderlyingApiMSFTValue {
    use crate::VkLayeredDriverUnderlyingApiMSFT;

    pub const VK_LAYERED_DRIVER_UNDERLYING_API_NONE_MSFT : VkLayeredDriverUnderlyingApiMSFT = 0;
    pub const VK_LAYERED_DRIVER_UNDERLYING_API_D3D12_MSFT : VkLayeredDriverUnderlyingApiMSFT = 1;
}

pub type VkLatencyMarkerNV = i32;
pub mod VkLatencyMarkerNVValue {
    use crate::VkLatencyMarkerNV;

    pub const VK_LATENCY_MARKER_SIMULATION_START_NV : VkLatencyMarkerNV = 0;
    pub const VK_LATENCY_MARKER_SIMULATION_END_NV : VkLatencyMarkerNV = 1;
    pub const VK_LATENCY_MARKER_RENDERSUBMIT_START_NV : VkLatencyMarkerNV = 2;
    pub const VK_LATENCY_MARKER_RENDERSUBMIT_END_NV : VkLatencyMarkerNV = 3;
    pub const VK_LATENCY_MARKER_PRESENT_START_NV : VkLatencyMarkerNV = 4;
    pub const VK_LATENCY_MARKER_PRESENT_END_NV : VkLatencyMarkerNV = 5;
    pub const VK_LATENCY_MARKER_INPUT_SAMPLE_NV : VkLatencyMarkerNV = 6;
    pub const VK_LATENCY_MARKER_TRIGGER_FLASH_NV : VkLatencyMarkerNV = 7;
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_RENDERSUBMIT_START_NV : VkLatencyMarkerNV = 8;
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_RENDERSUBMIT_END_NV : VkLatencyMarkerNV = 9;
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_PRESENT_START_NV : VkLatencyMarkerNV = 10;
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_PRESENT_END_NV : VkLatencyMarkerNV = 11;
}

pub type VkOutOfBandQueueTypeNV = i32;
pub mod VkOutOfBandQueueTypeNVValue {
    use crate::VkOutOfBandQueueTypeNV;

    pub const VK_OUT_OF_BAND_QUEUE_TYPE_RENDER_NV : VkOutOfBandQueueTypeNV = 0;
    pub const VK_OUT_OF_BAND_QUEUE_TYPE_PRESENT_NV : VkOutOfBandQueueTypeNV = 1;
}

pub type VkMemoryUnmapFlagBits = u32;
pub mod VkMemoryUnmapFlagBitsValue {
    use crate::VkMemoryUnmapFlagBits;

}

pub type VkCompressedTriangleFormatAMDX = i32;
pub mod VkCompressedTriangleFormatAMDXValue {
    use crate::VkCompressedTriangleFormatAMDX;

    pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_AMDX : VkCompressedTriangleFormatAMDX = 0;
}

pub type VkWaylandSurfaceCreateFlagBitsKHR = u32;
pub mod VkWaylandSurfaceCreateFlagBitsKHRValue {
    use crate::VkWaylandSurfaceCreateFlagBitsKHR;

}

pub type VkDepthClampModeEXT = i32;
pub mod VkDepthClampModeEXTValue {
    use crate::VkDepthClampModeEXT;

    pub const VK_DEPTH_CLAMP_MODE_VIEWPORT_RANGE_EXT : VkDepthClampModeEXT = 0;
    pub const VK_DEPTH_CLAMP_MODE_USER_DEFINED_RANGE_EXT : VkDepthClampModeEXT = 1;
}

pub type VkAccessFlagBits3KHR = u64;
pub mod VkAccessFlagBits3KHRValue {
    use crate::VkAccessFlagBits3KHR;

}

pub type VkTileShadingRenderPassFlagBitsQCOM = u32;
pub mod VkTileShadingRenderPassFlagBitsQCOMValue {
    use crate::VkTileShadingRenderPassFlagBitsQCOM;

    pub const VK_TILE_SHADING_RENDER_PASS_ENABLE_BIT_QCOM : VkTileShadingRenderPassFlagBitsQCOM = 1;
    pub const VK_TILE_SHADING_RENDER_PASS_PER_TILE_EXECUTION_BIT_QCOM : VkTileShadingRenderPassFlagBitsQCOM = 2;
}

pub type VkCooperativeVectorMatrixLayoutNV = i32;
pub mod VkCooperativeVectorMatrixLayoutNVValue {
    use crate::VkCooperativeVectorMatrixLayoutNV;

    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_ROW_MAJOR_NV : VkCooperativeVectorMatrixLayoutNV = 0;
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_COLUMN_MAJOR_NV : VkCooperativeVectorMatrixLayoutNV = 1;
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_INFERENCING_OPTIMAL_NV : VkCooperativeVectorMatrixLayoutNV = 2;
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_TRAINING_OPTIMAL_NV : VkCooperativeVectorMatrixLayoutNV = 3;
}

pub type VkAddressCopyFlagBitsKHR = u32;
pub mod VkAddressCopyFlagBitsKHRValue {
    use crate::VkAddressCopyFlagBitsKHR;

    pub const VK_ADDRESS_COPY_DEVICE_LOCAL_BIT_KHR : VkAddressCopyFlagBitsKHR = 1;
    pub const VK_ADDRESS_COPY_SPARSE_BIT_KHR : VkAddressCopyFlagBitsKHR = 2;
    pub const VK_ADDRESS_COPY_PROTECTED_BIT_KHR : VkAddressCopyFlagBitsKHR = 4;
}

pub type VkTensorCreateFlagBitsARM = u64;
pub mod VkTensorCreateFlagBitsARMValue {
    use crate::VkTensorCreateFlagBitsARM;

}

pub type VkTensorUsageFlagBitsARM = u64;
pub mod VkTensorUsageFlagBitsARMValue {
    use crate::VkTensorUsageFlagBitsARM;

}

pub type VkTensorTilingARM = i32;
pub mod VkTensorTilingARMValue {
    use crate::VkTensorTilingARM;

    pub const VK_TENSOR_TILING_OPTIMAL_ARM : VkTensorTilingARM = 0;
    pub const VK_TENSOR_TILING_LINEAR_ARM : VkTensorTilingARM = 1;
}

pub type VkTensorViewCreateFlagBitsARM = u64;
pub mod VkTensorViewCreateFlagBitsARMValue {
    use crate::VkTensorViewCreateFlagBitsARM;

}

pub type VkDefaultVertexAttributeValueKHR = i32;
pub mod VkDefaultVertexAttributeValueKHRValue {
    use crate::VkDefaultVertexAttributeValueKHR;

    pub const VK_DEFAULT_VERTEX_ATTRIBUTE_VALUE_ZERO_ZERO_ZERO_ZERO_KHR : VkDefaultVertexAttributeValueKHR = 0;
    pub const VK_DEFAULT_VERTEX_ATTRIBUTE_VALUE_ZERO_ZERO_ZERO_ONE_KHR : VkDefaultVertexAttributeValueKHR = 1;
}

pub type VkDataGraphPipelineSessionCreateFlagBitsARM = u64;
pub mod VkDataGraphPipelineSessionCreateFlagBitsARMValue {
    use crate::VkDataGraphPipelineSessionCreateFlagBitsARM;

}

pub type VkDataGraphPipelineSessionBindPointARM = i32;
pub mod VkDataGraphPipelineSessionBindPointARMValue {
    use crate::VkDataGraphPipelineSessionBindPointARM;

    pub const VK_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_TRANSIENT_ARM : VkDataGraphPipelineSessionBindPointARM = 0;
}

pub type VkDataGraphPipelineSessionBindPointTypeARM = i32;
pub mod VkDataGraphPipelineSessionBindPointTypeARMValue {
    use crate::VkDataGraphPipelineSessionBindPointTypeARM;

    pub const VK_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_TYPE_MEMORY_ARM : VkDataGraphPipelineSessionBindPointTypeARM = 0;
}

pub type VkDataGraphPipelinePropertyARM = i32;
pub mod VkDataGraphPipelinePropertyARMValue {
    use crate::VkDataGraphPipelinePropertyARM;

    pub const VK_DATA_GRAPH_PIPELINE_PROPERTY_CREATION_LOG_ARM : VkDataGraphPipelinePropertyARM = 0;
    pub const VK_DATA_GRAPH_PIPELINE_PROPERTY_IDENTIFIER_ARM : VkDataGraphPipelinePropertyARM = 1;
}

pub type VkDataGraphPipelineDispatchFlagBitsARM = u64;
pub mod VkDataGraphPipelineDispatchFlagBitsARMValue {
    use crate::VkDataGraphPipelineDispatchFlagBitsARM;

}

pub type VkPhysicalDeviceDataGraphProcessingEngineTypeARM = i32;
pub mod VkPhysicalDeviceDataGraphProcessingEngineTypeARMValue {
    use crate::VkPhysicalDeviceDataGraphProcessingEngineTypeARM;

    pub const VK_PHYSICAL_DEVICE_DATA_GRAPH_PROCESSING_ENGINE_TYPE_DEFAULT_ARM : VkPhysicalDeviceDataGraphProcessingEngineTypeARM = 0;
}

pub type VkPhysicalDeviceDataGraphOperationTypeARM = i32;
pub mod VkPhysicalDeviceDataGraphOperationTypeARMValue {
    use crate::VkPhysicalDeviceDataGraphOperationTypeARM;

    pub const VK_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_TYPE_SPIRV_EXTENDED_INSTRUCTION_SET_ARM : VkPhysicalDeviceDataGraphOperationTypeARM = 0;
}

pub type VkDataGraphModelCacheTypeQCOM = i32;
pub mod VkDataGraphModelCacheTypeQCOMValue {
    use crate::VkDataGraphModelCacheTypeQCOM;

    pub const VK_DATA_GRAPH_MODEL_CACHE_TYPE_GENERIC_BINARY_QCOM : VkDataGraphModelCacheTypeQCOM = 0;
}

pub type VkPerfHintTypeQCOM = i32;
pub mod VkPerfHintTypeQCOMValue {
    use crate::VkPerfHintTypeQCOM;

    pub const VK_PERF_HINT_TYPE_DEFAULT_QCOM : VkPerfHintTypeQCOM = 0;
    pub const VK_PERF_HINT_TYPE_FREQUENCY_MIN_QCOM : VkPerfHintTypeQCOM = 1;
    pub const VK_PERF_HINT_TYPE_FREQUENCY_MAX_QCOM : VkPerfHintTypeQCOM = 2;
    pub const VK_PERF_HINT_TYPE_FREQUENCY_SCALED_QCOM : VkPerfHintTypeQCOM = 3;
}

pub type VkThrottleHintTypeSEC = i32;
pub mod VkThrottleHintTypeSECValue {
    use crate::VkThrottleHintTypeSEC;

    pub const VK_THROTTLE_HINT_TYPE_DEFAULT_SEC : VkThrottleHintTypeSEC = 0;
    pub const VK_THROTTLE_HINT_TYPE_LOW_SEC : VkThrottleHintTypeSEC = 1;
    pub const VK_THROTTLE_HINT_TYPE_HIGH_SEC : VkThrottleHintTypeSEC = 2;
}

pub type VkVideoEncodeRgbModelConversionFlagBitsVALVE = u32;
pub mod VkVideoEncodeRgbModelConversionFlagBitsVALVEValue {
    use crate::VkVideoEncodeRgbModelConversionFlagBitsVALVE;

    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_RGB_IDENTITY_BIT_VALVE : VkVideoEncodeRgbModelConversionFlagBitsVALVE = 1;
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_IDENTITY_BIT_VALVE : VkVideoEncodeRgbModelConversionFlagBitsVALVE = 2;
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_709_BIT_VALVE : VkVideoEncodeRgbModelConversionFlagBitsVALVE = 4;
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_601_BIT_VALVE : VkVideoEncodeRgbModelConversionFlagBitsVALVE = 8;
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_2020_BIT_VALVE : VkVideoEncodeRgbModelConversionFlagBitsVALVE = 16;
}

pub type VkVideoEncodeRgbRangeCompressionFlagBitsVALVE = u32;
pub mod VkVideoEncodeRgbRangeCompressionFlagBitsVALVEValue {
    use crate::VkVideoEncodeRgbRangeCompressionFlagBitsVALVE;

    pub const VK_VIDEO_ENCODE_RGB_RANGE_COMPRESSION_FULL_RANGE_BIT_VALVE : VkVideoEncodeRgbRangeCompressionFlagBitsVALVE = 1;
    pub const VK_VIDEO_ENCODE_RGB_RANGE_COMPRESSION_NARROW_RANGE_BIT_VALVE : VkVideoEncodeRgbRangeCompressionFlagBitsVALVE = 2;
}

pub type VkVideoEncodeRgbChromaOffsetFlagBitsVALVE = u32;
pub mod VkVideoEncodeRgbChromaOffsetFlagBitsVALVEValue {
    use crate::VkVideoEncodeRgbChromaOffsetFlagBitsVALVE;

    pub const VK_VIDEO_ENCODE_RGB_CHROMA_OFFSET_COSITED_EVEN_BIT_VALVE : VkVideoEncodeRgbChromaOffsetFlagBitsVALVE = 1;
    pub const VK_VIDEO_ENCODE_RGB_CHROMA_OFFSET_MIDPOINT_BIT_VALVE : VkVideoEncodeRgbChromaOffsetFlagBitsVALVE = 2;
}

pub type VkSwapchainImageUsageFlagBitsOHOS = u32;
pub mod VkSwapchainImageUsageFlagBitsOHOSValue {
    use crate::VkSwapchainImageUsageFlagBitsOHOS;

    pub const VK_SWAPCHAIN_IMAGE_USAGE_SHARED_BIT_OHOS : VkSwapchainImageUsageFlagBitsOHOS = 1;
}

pub type VkDescriptorMappingSourceEXT = i32;
pub mod VkDescriptorMappingSourceEXTValue {
    use crate::VkDescriptorMappingSourceEXT;

    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_CONSTANT_OFFSET_EXT : VkDescriptorMappingSourceEXT = 0;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_PUSH_INDEX_EXT : VkDescriptorMappingSourceEXT = 1;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_INDIRECT_INDEX_EXT : VkDescriptorMappingSourceEXT = 2;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_INDIRECT_INDEX_ARRAY_EXT : VkDescriptorMappingSourceEXT = 3;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_RESOURCE_HEAP_DATA_EXT : VkDescriptorMappingSourceEXT = 4;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_PUSH_DATA_EXT : VkDescriptorMappingSourceEXT = 5;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_PUSH_ADDRESS_EXT : VkDescriptorMappingSourceEXT = 6;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_INDIRECT_ADDRESS_EXT : VkDescriptorMappingSourceEXT = 7;
}

pub type VkSpirvResourceTypeFlagBitsEXT = u32;
pub mod VkSpirvResourceTypeFlagBitsEXTValue {
    use crate::VkSpirvResourceTypeFlagBitsEXT;

    pub const VK_SPIRV_RESOURCE_TYPE_ALL_EXT : VkSpirvResourceTypeFlagBitsEXT = 0x7FFFFFFF;
    pub const VK_SPIRV_RESOURCE_TYPE_SAMPLER_BIT_EXT : VkSpirvResourceTypeFlagBitsEXT = 1;
    pub const VK_SPIRV_RESOURCE_TYPE_SAMPLED_IMAGE_BIT_EXT : VkSpirvResourceTypeFlagBitsEXT = 2;
    pub const VK_SPIRV_RESOURCE_TYPE_READ_ONLY_IMAGE_BIT_EXT : VkSpirvResourceTypeFlagBitsEXT = 4;
    pub const VK_SPIRV_RESOURCE_TYPE_READ_WRITE_IMAGE_BIT_EXT : VkSpirvResourceTypeFlagBitsEXT = 8;
    pub const VK_SPIRV_RESOURCE_TYPE_COMBINED_SAMPLED_IMAGE_BIT_EXT : VkSpirvResourceTypeFlagBitsEXT = 16;
    pub const VK_SPIRV_RESOURCE_TYPE_UNIFORM_BUFFER_BIT_EXT : VkSpirvResourceTypeFlagBitsEXT = 32;
    pub const VK_SPIRV_RESOURCE_TYPE_READ_ONLY_STORAGE_BUFFER_BIT_EXT : VkSpirvResourceTypeFlagBitsEXT = 64;
    pub const VK_SPIRV_RESOURCE_TYPE_READ_WRITE_STORAGE_BUFFER_BIT_EXT : VkSpirvResourceTypeFlagBitsEXT = 128;
}

pub type VkGpaSqShaderStageFlagBitsAMD = u32;
pub mod VkGpaSqShaderStageFlagBitsAMDValue {
    use crate::VkGpaSqShaderStageFlagBitsAMD;

    pub const VK_GPA_SQ_SHADER_STAGE_PS_BIT_AMD : VkGpaSqShaderStageFlagBitsAMD = 1;
    pub const VK_GPA_SQ_SHADER_STAGE_VS_BIT_AMD : VkGpaSqShaderStageFlagBitsAMD = 2;
    pub const VK_GPA_SQ_SHADER_STAGE_GS_BIT_AMD : VkGpaSqShaderStageFlagBitsAMD = 4;
    pub const VK_GPA_SQ_SHADER_STAGE_ES_BIT_AMD : VkGpaSqShaderStageFlagBitsAMD = 8;
    pub const VK_GPA_SQ_SHADER_STAGE_HS_BIT_AMD : VkGpaSqShaderStageFlagBitsAMD = 16;
    pub const VK_GPA_SQ_SHADER_STAGE_LS_BIT_AMD : VkGpaSqShaderStageFlagBitsAMD = 32;
    pub const VK_GPA_SQ_SHADER_STAGE_CS_BIT_AMD : VkGpaSqShaderStageFlagBitsAMD = 64;
}

pub type VkGpaPerfBlockAMD = i32;
pub mod VkGpaPerfBlockAMDValue {
    use crate::VkGpaPerfBlockAMD;

    pub const VK_GPA_PERF_BLOCK_CPF_AMD : VkGpaPerfBlockAMD = 0;
    pub const VK_GPA_PERF_BLOCK_IA_AMD : VkGpaPerfBlockAMD = 1;
    pub const VK_GPA_PERF_BLOCK_VGT_AMD : VkGpaPerfBlockAMD = 2;
    pub const VK_GPA_PERF_BLOCK_PA_AMD : VkGpaPerfBlockAMD = 3;
    pub const VK_GPA_PERF_BLOCK_SC_AMD : VkGpaPerfBlockAMD = 4;
    pub const VK_GPA_PERF_BLOCK_SPI_AMD : VkGpaPerfBlockAMD = 5;
    pub const VK_GPA_PERF_BLOCK_SQ_AMD : VkGpaPerfBlockAMD = 6;
    pub const VK_GPA_PERF_BLOCK_SX_AMD : VkGpaPerfBlockAMD = 7;
    pub const VK_GPA_PERF_BLOCK_TA_AMD : VkGpaPerfBlockAMD = 8;
    pub const VK_GPA_PERF_BLOCK_TD_AMD : VkGpaPerfBlockAMD = 9;
    pub const VK_GPA_PERF_BLOCK_TCP_AMD : VkGpaPerfBlockAMD = 10;
    pub const VK_GPA_PERF_BLOCK_TCC_AMD : VkGpaPerfBlockAMD = 11;
    pub const VK_GPA_PERF_BLOCK_TCA_AMD : VkGpaPerfBlockAMD = 12;
    pub const VK_GPA_PERF_BLOCK_DB_AMD : VkGpaPerfBlockAMD = 13;
    pub const VK_GPA_PERF_BLOCK_CB_AMD : VkGpaPerfBlockAMD = 14;
    pub const VK_GPA_PERF_BLOCK_GDS_AMD : VkGpaPerfBlockAMD = 15;
    pub const VK_GPA_PERF_BLOCK_SRBM_AMD : VkGpaPerfBlockAMD = 16;
    pub const VK_GPA_PERF_BLOCK_GRBM_AMD : VkGpaPerfBlockAMD = 17;
    pub const VK_GPA_PERF_BLOCK_GRBM_SE_AMD : VkGpaPerfBlockAMD = 18;
    pub const VK_GPA_PERF_BLOCK_RLC_AMD : VkGpaPerfBlockAMD = 19;
    pub const VK_GPA_PERF_BLOCK_DMA_AMD : VkGpaPerfBlockAMD = 20;
    pub const VK_GPA_PERF_BLOCK_MC_AMD : VkGpaPerfBlockAMD = 21;
    pub const VK_GPA_PERF_BLOCK_CPG_AMD : VkGpaPerfBlockAMD = 22;
    pub const VK_GPA_PERF_BLOCK_CPC_AMD : VkGpaPerfBlockAMD = 23;
    pub const VK_GPA_PERF_BLOCK_WD_AMD : VkGpaPerfBlockAMD = 24;
    pub const VK_GPA_PERF_BLOCK_TCS_AMD : VkGpaPerfBlockAMD = 25;
    pub const VK_GPA_PERF_BLOCK_ATC_AMD : VkGpaPerfBlockAMD = 26;
    pub const VK_GPA_PERF_BLOCK_ATC_L2_AMD : VkGpaPerfBlockAMD = 27;
    pub const VK_GPA_PERF_BLOCK_MC_VM_L2_AMD : VkGpaPerfBlockAMD = 28;
    pub const VK_GPA_PERF_BLOCK_EA_AMD : VkGpaPerfBlockAMD = 29;
    pub const VK_GPA_PERF_BLOCK_RPB_AMD : VkGpaPerfBlockAMD = 30;
    pub const VK_GPA_PERF_BLOCK_RMI_AMD : VkGpaPerfBlockAMD = 31;
    pub const VK_GPA_PERF_BLOCK_UMCCH_AMD : VkGpaPerfBlockAMD = 32;
    pub const VK_GPA_PERF_BLOCK_GE_AMD : VkGpaPerfBlockAMD = 33;
    pub const VK_GPA_PERF_BLOCK_GL1A_AMD : VkGpaPerfBlockAMD = 34;
    pub const VK_GPA_PERF_BLOCK_GL1C_AMD : VkGpaPerfBlockAMD = 35;
    pub const VK_GPA_PERF_BLOCK_GL1CG_AMD : VkGpaPerfBlockAMD = 36;
    pub const VK_GPA_PERF_BLOCK_GL2A_AMD : VkGpaPerfBlockAMD = 37;
    pub const VK_GPA_PERF_BLOCK_GL2C_AMD : VkGpaPerfBlockAMD = 38;
    pub const VK_GPA_PERF_BLOCK_CHA_AMD : VkGpaPerfBlockAMD = 39;
    pub const VK_GPA_PERF_BLOCK_CHC_AMD : VkGpaPerfBlockAMD = 40;
    pub const VK_GPA_PERF_BLOCK_CHCG_AMD : VkGpaPerfBlockAMD = 41;
    pub const VK_GPA_PERF_BLOCK_GUS_AMD : VkGpaPerfBlockAMD = 42;
    pub const VK_GPA_PERF_BLOCK_GCR_AMD : VkGpaPerfBlockAMD = 43;
    pub const VK_GPA_PERF_BLOCK_PH_AMD : VkGpaPerfBlockAMD = 44;
    pub const VK_GPA_PERF_BLOCK_UTCL1_AMD : VkGpaPerfBlockAMD = 45;
    pub const VK_GPA_PERF_BLOCK_GE_DIST_AMD : VkGpaPerfBlockAMD = 46;
    pub const VK_GPA_PERF_BLOCK_GE_SE_AMD : VkGpaPerfBlockAMD = 47;
    pub const VK_GPA_PERF_BLOCK_DF_MALL_AMD : VkGpaPerfBlockAMD = 48;
    pub const VK_GPA_PERF_BLOCK_SQ_WGP_AMD : VkGpaPerfBlockAMD = 49;
    pub const VK_GPA_PERF_BLOCK_PC_AMD : VkGpaPerfBlockAMD = 50;
    pub const VK_GPA_PERF_BLOCK_GL1XA_AMD : VkGpaPerfBlockAMD = 51;
    pub const VK_GPA_PERF_BLOCK_GL1XC_AMD : VkGpaPerfBlockAMD = 52;
    pub const VK_GPA_PERF_BLOCK_WGS_AMD : VkGpaPerfBlockAMD = 53;
    pub const VK_GPA_PERF_BLOCK_EACPWD_AMD : VkGpaPerfBlockAMD = 54;
    pub const VK_GPA_PERF_BLOCK_EASE_AMD : VkGpaPerfBlockAMD = 55;
    pub const VK_GPA_PERF_BLOCK_RLCUSER_AMD : VkGpaPerfBlockAMD = 56;
}

pub type VkGpaSampleTypeAMD = i32;
pub mod VkGpaSampleTypeAMDValue {
    use crate::VkGpaSampleTypeAMD;

    pub const VK_GPA_SAMPLE_TYPE_CUMULATIVE_AMD : VkGpaSampleTypeAMD = 0;
    pub const VK_GPA_SAMPLE_TYPE_TRACE_AMD : VkGpaSampleTypeAMD = 1;
    pub const VK_GPA_SAMPLE_TYPE_TIMING_AMD : VkGpaSampleTypeAMD = 2;
}

pub type VkGpaDeviceClockModeAMD = i32;
pub mod VkGpaDeviceClockModeAMDValue {
    use crate::VkGpaDeviceClockModeAMD;

    pub const VK_GPA_DEVICE_CLOCK_MODE_DEFAULT_AMD : VkGpaDeviceClockModeAMD = 0;
    pub const VK_GPA_DEVICE_CLOCK_MODE_QUERY_AMD : VkGpaDeviceClockModeAMD = 1;
    pub const VK_GPA_DEVICE_CLOCK_MODE_PROFILING_AMD : VkGpaDeviceClockModeAMD = 2;
    pub const VK_GPA_DEVICE_CLOCK_MODE_MIN_MEMORY_AMD : VkGpaDeviceClockModeAMD = 3;
    pub const VK_GPA_DEVICE_CLOCK_MODE_MIN_ENGINE_AMD : VkGpaDeviceClockModeAMD = 4;
    pub const VK_GPA_DEVICE_CLOCK_MODE_PEAK_AMD : VkGpaDeviceClockModeAMD = 5;
}

pub type VkAddressCommandFlagBitsKHR = u32;
pub mod VkAddressCommandFlagBitsKHRValue {
    use crate::VkAddressCommandFlagBitsKHR;

    pub const VK_ADDRESS_COMMAND_PROTECTED_BIT_KHR : VkAddressCommandFlagBitsKHR = 1;
    pub const VK_ADDRESS_COMMAND_FULLY_BOUND_BIT_KHR : VkAddressCommandFlagBitsKHR = 2;
    pub const VK_ADDRESS_COMMAND_STORAGE_BUFFER_USAGE_BIT_KHR : VkAddressCommandFlagBitsKHR = 4;
    pub const VK_ADDRESS_COMMAND_UNKNOWN_STORAGE_BUFFER_USAGE_BIT_KHR : VkAddressCommandFlagBitsKHR = 8;
}

pub type VkDataGraphTOSAQualityFlagBitsARM = u32;
pub mod VkDataGraphTOSAQualityFlagBitsARMValue {
    use crate::VkDataGraphTOSAQualityFlagBitsARM;

    pub const VK_DATA_GRAPH_TOSA_QUALITY_ACCELERATED_ARM : VkDataGraphTOSAQualityFlagBitsARM = 1;
    pub const VK_DATA_GRAPH_TOSA_QUALITY_CONFORMANT_ARM : VkDataGraphTOSAQualityFlagBitsARM = 2;
    pub const VK_DATA_GRAPH_TOSA_QUALITY_EXPERIMENTAL_ARM : VkDataGraphTOSAQualityFlagBitsARM = 4;
    pub const VK_DATA_GRAPH_TOSA_QUALITY_DEPRECATED_ARM : VkDataGraphTOSAQualityFlagBitsARM = 8;
}

pub type VkDataGraphTOSALevelARM = i32;
pub mod VkDataGraphTOSALevelARMValue {
    use crate::VkDataGraphTOSALevelARM;

    pub const VK_DATA_GRAPH_TOSA_LEVEL_NONE_ARM : VkDataGraphTOSALevelARM = 0;
    pub const VK_DATA_GRAPH_TOSA_LEVEL_8K_ARM : VkDataGraphTOSALevelARM = 1;
}

pub type VkDataGraphOpticalFlowGridSizeFlagBitsARM = u32;
pub mod VkDataGraphOpticalFlowGridSizeFlagBitsARMValue {
    use crate::VkDataGraphOpticalFlowGridSizeFlagBitsARM;

    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_UNKNOWN_ARM : VkDataGraphOpticalFlowGridSizeFlagBitsARM = 0;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_1X1_BIT_ARM : VkDataGraphOpticalFlowGridSizeFlagBitsARM = 1;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_2X2_BIT_ARM : VkDataGraphOpticalFlowGridSizeFlagBitsARM = 2;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_4X4_BIT_ARM : VkDataGraphOpticalFlowGridSizeFlagBitsARM = 4;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_8X8_BIT_ARM : VkDataGraphOpticalFlowGridSizeFlagBitsARM = 8;
}

pub type VkDataGraphOpticalFlowImageUsageFlagBitsARM = u32;
pub mod VkDataGraphOpticalFlowImageUsageFlagBitsARMValue {
    use crate::VkDataGraphOpticalFlowImageUsageFlagBitsARM;

    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_UNKNOWN_ARM : VkDataGraphOpticalFlowImageUsageFlagBitsARM = 0;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_INPUT_BIT_ARM : VkDataGraphOpticalFlowImageUsageFlagBitsARM = 1;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_OUTPUT_BIT_ARM : VkDataGraphOpticalFlowImageUsageFlagBitsARM = 2;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_HINT_BIT_ARM : VkDataGraphOpticalFlowImageUsageFlagBitsARM = 4;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_COST_BIT_ARM : VkDataGraphOpticalFlowImageUsageFlagBitsARM = 8;
}

pub type VkDataGraphOpticalFlowPerformanceLevelARM = i32;
pub mod VkDataGraphOpticalFlowPerformanceLevelARMValue {
    use crate::VkDataGraphOpticalFlowPerformanceLevelARM;

    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_UNKNOWN_ARM : VkDataGraphOpticalFlowPerformanceLevelARM = 0;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_SLOW_ARM : VkDataGraphOpticalFlowPerformanceLevelARM = 1;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_MEDIUM_ARM : VkDataGraphOpticalFlowPerformanceLevelARM = 2;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_FAST_ARM : VkDataGraphOpticalFlowPerformanceLevelARM = 3;
}

pub type VkDataGraphPipelineNodeConnectionTypeARM = i32;
pub mod VkDataGraphPipelineNodeConnectionTypeARMValue {
    use crate::VkDataGraphPipelineNodeConnectionTypeARM;

}

pub type VkDataGraphPipelineNodeTypeARM = i32;
pub mod VkDataGraphPipelineNodeTypeARMValue {
    use crate::VkDataGraphPipelineNodeTypeARM;

}

pub type VkDataGraphOpticalFlowCreateFlagBitsARM = u32;
pub mod VkDataGraphOpticalFlowCreateFlagBitsARMValue {
    use crate::VkDataGraphOpticalFlowCreateFlagBitsARM;

    pub const VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_ENABLE_HINT_BIT_ARM : VkDataGraphOpticalFlowCreateFlagBitsARM = 1;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_ENABLE_COST_BIT_ARM : VkDataGraphOpticalFlowCreateFlagBitsARM = 2;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_RESERVED_30_BIT_ARM : VkDataGraphOpticalFlowCreateFlagBitsARM = 1073741824;
}

pub type VkDataGraphOpticalFlowExecuteFlagBitsARM = u32;
pub mod VkDataGraphOpticalFlowExecuteFlagBitsARMValue {
    use crate::VkDataGraphOpticalFlowExecuteFlagBitsARM;

    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_ARM : VkDataGraphOpticalFlowExecuteFlagBitsARM = 1;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_INPUT_UNCHANGED_BIT_ARM : VkDataGraphOpticalFlowExecuteFlagBitsARM = 2;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_REFERENCE_UNCHANGED_BIT_ARM : VkDataGraphOpticalFlowExecuteFlagBitsARM = 4;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_INPUT_IS_PREVIOUS_REFERENCE_BIT_ARM : VkDataGraphOpticalFlowExecuteFlagBitsARM = 8;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_REFERENCE_IS_PREVIOUS_INPUT_BIT_ARM : VkDataGraphOpticalFlowExecuteFlagBitsARM = 16;
}

pub type VkNeuralAcceleratorStatisticsModeARM = i32;
pub mod VkNeuralAcceleratorStatisticsModeARMValue {
    use crate::VkNeuralAcceleratorStatisticsModeARM;

    pub const VK_NEURAL_ACCELERATOR_STATISTICS_MODE_DISABLED_ARM : VkNeuralAcceleratorStatisticsModeARM = 0;
    pub const VK_NEURAL_ACCELERATOR_STATISTICS_MODE_STATISTICS0_ARM : VkNeuralAcceleratorStatisticsModeARM = 1;
    pub const VK_NEURAL_ACCELERATOR_STATISTICS_MODE_STATISTICS1_ARM : VkNeuralAcceleratorStatisticsModeARM = 2;
}
