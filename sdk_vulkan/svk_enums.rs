pub struct VkImageLayout(i32); //
impl VkImageLayout {
    pub const VK_IMAGE_LAYOUT_UNDEFINED = 0;
    pub const VK_IMAGE_LAYOUT_GENERAL = 1;
    pub const VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL = 2;
    pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL = 3;
    pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL = 4;
    pub const VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL = 5;
    pub const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL = 6;
    pub const VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL = 7;
    pub const VK_IMAGE_LAYOUT_PREINITIALIZED = 8;
}

pub struct VkAttachmentLoadOp(i32); //
impl VkAttachmentLoadOp {
    pub const VK_ATTACHMENT_LOAD_OP_LOAD = 0;
    pub const VK_ATTACHMENT_LOAD_OP_CLEAR = 1;
    pub const VK_ATTACHMENT_LOAD_OP_DONT_CARE = 2;
}

pub struct VkAttachmentStoreOp(i32); //
impl VkAttachmentStoreOp {
    pub const VK_ATTACHMENT_STORE_OP_STORE = 0;
    pub const VK_ATTACHMENT_STORE_OP_DONT_CARE = 1;
}

pub struct VkImageType(i32); //
impl VkImageType {
    pub const VK_IMAGE_TYPE_1D = 0;
    pub const VK_IMAGE_TYPE_2D = 1;
    pub const VK_IMAGE_TYPE_3D = 2;
}

pub struct VkImageTiling(i32); //
impl VkImageTiling {
    pub const VK_IMAGE_TILING_OPTIMAL = 0;
    pub const VK_IMAGE_TILING_LINEAR = 1;
}

pub struct VkImageViewType(i32); //
impl VkImageViewType {
    pub const VK_IMAGE_VIEW_TYPE_1D = 0;
    pub const VK_IMAGE_VIEW_TYPE_2D = 1;
    pub const VK_IMAGE_VIEW_TYPE_3D = 2;
    pub const VK_IMAGE_VIEW_TYPE_CUBE = 3;
    pub const VK_IMAGE_VIEW_TYPE_1D_ARRAY = 4;
    pub const VK_IMAGE_VIEW_TYPE_2D_ARRAY = 5;
    pub const VK_IMAGE_VIEW_TYPE_CUBE_ARRAY = 6;
}

pub struct VkCommandBufferLevel(i32); //
impl VkCommandBufferLevel {
    pub const VK_COMMAND_BUFFER_LEVEL_PRIMARY = 0;
    pub const VK_COMMAND_BUFFER_LEVEL_SECONDARY = 1;
}

pub struct VkComponentSwizzle(i32); //
impl VkComponentSwizzle {
    pub const VK_COMPONENT_SWIZZLE_IDENTITY = 0;
    pub const VK_COMPONENT_SWIZZLE_ZERO = 1;
    pub const VK_COMPONENT_SWIZZLE_ONE = 2;
    pub const VK_COMPONENT_SWIZZLE_R = 3;
    pub const VK_COMPONENT_SWIZZLE_G = 4;
    pub const VK_COMPONENT_SWIZZLE_B = 5;
    pub const VK_COMPONENT_SWIZZLE_A = 6;
}

pub struct VkDescriptorType(i32); //
impl VkDescriptorType {
    pub const VK_DESCRIPTOR_TYPE_SAMPLER = 0;
    pub const VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER = 1;
    pub const VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE = 2;
    pub const VK_DESCRIPTOR_TYPE_STORAGE_IMAGE = 3;
    pub const VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER = 4;
    pub const VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER = 5;
    pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER = 6;
    pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER = 7;
    pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC = 8;
    pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC = 9;
    pub const VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT = 10;
}

pub struct VkQueryType(i32); //
impl VkQueryType {
    pub const VK_QUERY_TYPE_OCCLUSION = 0;
    pub const VK_QUERY_TYPE_PIPELINE_STATISTICS = 1;
    pub const VK_QUERY_TYPE_TIMESTAMP = 2;
}

pub struct VkBorderColor(i32); //
impl VkBorderColor {
    pub const VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK = 0;
    pub const VK_BORDER_COLOR_INT_TRANSPARENT_BLACK = 1;
    pub const VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK = 2;
    pub const VK_BORDER_COLOR_INT_OPAQUE_BLACK = 3;
    pub const VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE = 4;
    pub const VK_BORDER_COLOR_INT_OPAQUE_WHITE = 5;
}

pub struct VkPipelineBindPoint(i32); //
impl VkPipelineBindPoint {
    pub const VK_PIPELINE_BIND_POINT_GRAPHICS = 0;
    pub const VK_PIPELINE_BIND_POINT_COMPUTE = 1;
}

pub struct VkPipelineCacheHeaderVersion(i32); //
impl VkPipelineCacheHeaderVersion {
    pub const VK_PIPELINE_CACHE_HEADER_VERSION_ONE = 1;
}

pub type VkPipelineCacheCreateFlagBits = ...; //
impl VkPipelineCacheCreateFlagBits {
}

pub struct VkPrimitiveTopology(i32); //
impl VkPrimitiveTopology {
    pub const VK_PRIMITIVE_TOPOLOGY_POINT_LIST = 0;
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST = 1;
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP = 2;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST = 3;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP = 4;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN = 5;
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY = 6;
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY = 7;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY = 8;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY = 9;
    pub const VK_PRIMITIVE_TOPOLOGY_PATCH_LIST = 10;
}

pub struct VkSharingMode(i32); //
impl VkSharingMode {
    pub const VK_SHARING_MODE_EXCLUSIVE = 0;
    pub const VK_SHARING_MODE_CONCURRENT = 1;
}

pub struct VkIndexType(i32); //
impl VkIndexType {
    pub const VK_INDEX_TYPE_UINT16 = 0;
    pub const VK_INDEX_TYPE_UINT32 = 1;
}

pub struct VkFilter(i32); //
impl VkFilter {
    pub const VK_FILTER_NEAREST = 0;
    pub const VK_FILTER_LINEAR = 1;
}

pub struct VkSamplerMipmapMode(i32); //
impl VkSamplerMipmapMode {
    pub const VK_SAMPLER_MIPMAP_MODE_NEAREST = 0;
    pub const VK_SAMPLER_MIPMAP_MODE_LINEAR = 1;
}

pub struct VkSamplerAddressMode(i32); //
impl VkSamplerAddressMode {
    pub const VK_SAMPLER_ADDRESS_MODE_REPEAT = 0;
    pub const VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT = 1;
    pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE = 2;
    pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER = 3;
}

pub struct VkCompareOp(i32); //
impl VkCompareOp {
    pub const VK_COMPARE_OP_NEVER = 0;
    pub const VK_COMPARE_OP_LESS = 1;
    pub const VK_COMPARE_OP_EQUAL = 2;
    pub const VK_COMPARE_OP_LESS_OR_EQUAL = 3;
    pub const VK_COMPARE_OP_GREATER = 4;
    pub const VK_COMPARE_OP_NOT_EQUAL = 5;
    pub const VK_COMPARE_OP_GREATER_OR_EQUAL = 6;
    pub const VK_COMPARE_OP_ALWAYS = 7;
}

pub struct VkPolygonMode(i32); //
impl VkPolygonMode {
    pub const VK_POLYGON_MODE_FILL = 0;
    pub const VK_POLYGON_MODE_LINE = 1;
    pub const VK_POLYGON_MODE_POINT = 2;
}

pub struct VkFrontFace(i32); //
impl VkFrontFace {
    pub const VK_FRONT_FACE_COUNTER_CLOCKWISE = 0;
    pub const VK_FRONT_FACE_CLOCKWISE = 1;
}

pub struct VkBlendFactor(i32); //
impl VkBlendFactor {
    pub const VK_BLEND_FACTOR_ZERO = 0;
    pub const VK_BLEND_FACTOR_ONE = 1;
    pub const VK_BLEND_FACTOR_SRC_COLOR = 2;
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR = 3;
    pub const VK_BLEND_FACTOR_DST_COLOR = 4;
    pub const VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR = 5;
    pub const VK_BLEND_FACTOR_SRC_ALPHA = 6;
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA = 7;
    pub const VK_BLEND_FACTOR_DST_ALPHA = 8;
    pub const VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA = 9;
    pub const VK_BLEND_FACTOR_CONSTANT_COLOR = 10;
    pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR = 11;
    pub const VK_BLEND_FACTOR_CONSTANT_ALPHA = 12;
    pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA = 13;
    pub const VK_BLEND_FACTOR_SRC_ALPHA_SATURATE = 14;
    pub const VK_BLEND_FACTOR_SRC1_COLOR = 15;
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR = 16;
    pub const VK_BLEND_FACTOR_SRC1_ALPHA = 17;
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA = 18;
}

pub struct VkBlendOp(i32); //
impl VkBlendOp {
    pub const VK_BLEND_OP_ADD = 0;
    pub const VK_BLEND_OP_SUBTRACT = 1;
    pub const VK_BLEND_OP_REVERSE_SUBTRACT = 2;
    pub const VK_BLEND_OP_MIN = 3;
    pub const VK_BLEND_OP_MAX = 4;
}

pub struct VkStencilOp(i32); //
impl VkStencilOp {
    pub const VK_STENCIL_OP_KEEP = 0;
    pub const VK_STENCIL_OP_ZERO = 1;
    pub const VK_STENCIL_OP_REPLACE = 2;
    pub const VK_STENCIL_OP_INCREMENT_AND_CLAMP = 3;
    pub const VK_STENCIL_OP_DECREMENT_AND_CLAMP = 4;
    pub const VK_STENCIL_OP_INVERT = 5;
    pub const VK_STENCIL_OP_INCREMENT_AND_WRAP = 6;
    pub const VK_STENCIL_OP_DECREMENT_AND_WRAP = 7;
}

pub struct VkLogicOp(i32); //
impl VkLogicOp {
    pub const VK_LOGIC_OP_CLEAR = 0;
    pub const VK_LOGIC_OP_AND = 1;
    pub const VK_LOGIC_OP_AND_REVERSE = 2;
    pub const VK_LOGIC_OP_COPY = 3;
    pub const VK_LOGIC_OP_AND_INVERTED = 4;
    pub const VK_LOGIC_OP_NO_OP = 5;
    pub const VK_LOGIC_OP_XOR = 6;
    pub const VK_LOGIC_OP_OR = 7;
    pub const VK_LOGIC_OP_NOR = 8;
    pub const VK_LOGIC_OP_EQUIVALENT = 9;
    pub const VK_LOGIC_OP_INVERT = 10;
    pub const VK_LOGIC_OP_OR_REVERSE = 11;
    pub const VK_LOGIC_OP_COPY_INVERTED = 12;
    pub const VK_LOGIC_OP_OR_INVERTED = 13;
    pub const VK_LOGIC_OP_NAND = 14;
    pub const VK_LOGIC_OP_SET = 15;
}

pub struct VkInternalAllocationType(i32); //
impl VkInternalAllocationType {
    pub const VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE = 0;
}

pub struct VkSystemAllocationScope(i32); //
impl VkSystemAllocationScope {
    pub const VK_SYSTEM_ALLOCATION_SCOPE_COMMAND = 0;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_OBJECT = 1;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_CACHE = 2;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_DEVICE = 3;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE = 4;
}

pub struct VkPhysicalDeviceType(i32); //
impl VkPhysicalDeviceType {
    pub const VK_PHYSICAL_DEVICE_TYPE_OTHER = 0;
    pub const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU = 1;
    pub const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU = 2;
    pub const VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU = 3;
    pub const VK_PHYSICAL_DEVICE_TYPE_CPU = 4;
}

pub struct VkVertexInputRate(i32); //
impl VkVertexInputRate {
    pub const VK_VERTEX_INPUT_RATE_VERTEX = 0;
    pub const VK_VERTEX_INPUT_RATE_INSTANCE = 1;
}

pub struct VkFormat(i32); //Vulkan format definitions
impl VkFormat {
    pub const VK_FORMAT_UNDEFINED = 0;
    pub const VK_FORMAT_R4G4_UNORM_PACK8 = 1;
    pub const VK_FORMAT_R4G4B4A4_UNORM_PACK16 = 2;
    pub const VK_FORMAT_B4G4R4A4_UNORM_PACK16 = 3;
    pub const VK_FORMAT_R5G6B5_UNORM_PACK16 = 4;
    pub const VK_FORMAT_B5G6R5_UNORM_PACK16 = 5;
    pub const VK_FORMAT_R5G5B5A1_UNORM_PACK16 = 6;
    pub const VK_FORMAT_B5G5R5A1_UNORM_PACK16 = 7;
    pub const VK_FORMAT_A1R5G5B5_UNORM_PACK16 = 8;
    pub const VK_FORMAT_R8_UNORM = 9;
    pub const VK_FORMAT_R8_SNORM = 10;
    pub const VK_FORMAT_R8_USCALED = 11;
    pub const VK_FORMAT_R8_SSCALED = 12;
    pub const VK_FORMAT_R8_UINT = 13;
    pub const VK_FORMAT_R8_SINT = 14;
    pub const VK_FORMAT_R8_SRGB = 15;
    pub const VK_FORMAT_R8G8_UNORM = 16;
    pub const VK_FORMAT_R8G8_SNORM = 17;
    pub const VK_FORMAT_R8G8_USCALED = 18;
    pub const VK_FORMAT_R8G8_SSCALED = 19;
    pub const VK_FORMAT_R8G8_UINT = 20;
    pub const VK_FORMAT_R8G8_SINT = 21;
    pub const VK_FORMAT_R8G8_SRGB = 22;
    pub const VK_FORMAT_R8G8B8_UNORM = 23;
    pub const VK_FORMAT_R8G8B8_SNORM = 24;
    pub const VK_FORMAT_R8G8B8_USCALED = 25;
    pub const VK_FORMAT_R8G8B8_SSCALED = 26;
    pub const VK_FORMAT_R8G8B8_UINT = 27;
    pub const VK_FORMAT_R8G8B8_SINT = 28;
    pub const VK_FORMAT_R8G8B8_SRGB = 29;
    pub const VK_FORMAT_B8G8R8_UNORM = 30;
    pub const VK_FORMAT_B8G8R8_SNORM = 31;
    pub const VK_FORMAT_B8G8R8_USCALED = 32;
    pub const VK_FORMAT_B8G8R8_SSCALED = 33;
    pub const VK_FORMAT_B8G8R8_UINT = 34;
    pub const VK_FORMAT_B8G8R8_SINT = 35;
    pub const VK_FORMAT_B8G8R8_SRGB = 36;
    pub const VK_FORMAT_R8G8B8A8_UNORM = 37;
    pub const VK_FORMAT_R8G8B8A8_SNORM = 38;
    pub const VK_FORMAT_R8G8B8A8_USCALED = 39;
    pub const VK_FORMAT_R8G8B8A8_SSCALED = 40;
    pub const VK_FORMAT_R8G8B8A8_UINT = 41;
    pub const VK_FORMAT_R8G8B8A8_SINT = 42;
    pub const VK_FORMAT_R8G8B8A8_SRGB = 43;
    pub const VK_FORMAT_B8G8R8A8_UNORM = 44;
    pub const VK_FORMAT_B8G8R8A8_SNORM = 45;
    pub const VK_FORMAT_B8G8R8A8_USCALED = 46;
    pub const VK_FORMAT_B8G8R8A8_SSCALED = 47;
    pub const VK_FORMAT_B8G8R8A8_UINT = 48;
    pub const VK_FORMAT_B8G8R8A8_SINT = 49;
    pub const VK_FORMAT_B8G8R8A8_SRGB = 50;
    pub const VK_FORMAT_A8B8G8R8_UNORM_PACK32 = 51;
    pub const VK_FORMAT_A8B8G8R8_SNORM_PACK32 = 52;
    pub const VK_FORMAT_A8B8G8R8_USCALED_PACK32 = 53;
    pub const VK_FORMAT_A8B8G8R8_SSCALED_PACK32 = 54;
    pub const VK_FORMAT_A8B8G8R8_UINT_PACK32 = 55;
    pub const VK_FORMAT_A8B8G8R8_SINT_PACK32 = 56;
    pub const VK_FORMAT_A8B8G8R8_SRGB_PACK32 = 57;
    pub const VK_FORMAT_A2R10G10B10_UNORM_PACK32 = 58;
    pub const VK_FORMAT_A2R10G10B10_SNORM_PACK32 = 59;
    pub const VK_FORMAT_A2R10G10B10_USCALED_PACK32 = 60;
    pub const VK_FORMAT_A2R10G10B10_SSCALED_PACK32 = 61;
    pub const VK_FORMAT_A2R10G10B10_UINT_PACK32 = 62;
    pub const VK_FORMAT_A2R10G10B10_SINT_PACK32 = 63;
    pub const VK_FORMAT_A2B10G10R10_UNORM_PACK32 = 64;
    pub const VK_FORMAT_A2B10G10R10_SNORM_PACK32 = 65;
    pub const VK_FORMAT_A2B10G10R10_USCALED_PACK32 = 66;
    pub const VK_FORMAT_A2B10G10R10_SSCALED_PACK32 = 67;
    pub const VK_FORMAT_A2B10G10R10_UINT_PACK32 = 68;
    pub const VK_FORMAT_A2B10G10R10_SINT_PACK32 = 69;
    pub const VK_FORMAT_R16_UNORM = 70;
    pub const VK_FORMAT_R16_SNORM = 71;
    pub const VK_FORMAT_R16_USCALED = 72;
    pub const VK_FORMAT_R16_SSCALED = 73;
    pub const VK_FORMAT_R16_UINT = 74;
    pub const VK_FORMAT_R16_SINT = 75;
    pub const VK_FORMAT_R16_SFLOAT = 76;
    pub const VK_FORMAT_R16G16_UNORM = 77;
    pub const VK_FORMAT_R16G16_SNORM = 78;
    pub const VK_FORMAT_R16G16_USCALED = 79;
    pub const VK_FORMAT_R16G16_SSCALED = 80;
    pub const VK_FORMAT_R16G16_UINT = 81;
    pub const VK_FORMAT_R16G16_SINT = 82;
    pub const VK_FORMAT_R16G16_SFLOAT = 83;
    pub const VK_FORMAT_R16G16B16_UNORM = 84;
    pub const VK_FORMAT_R16G16B16_SNORM = 85;
    pub const VK_FORMAT_R16G16B16_USCALED = 86;
    pub const VK_FORMAT_R16G16B16_SSCALED = 87;
    pub const VK_FORMAT_R16G16B16_UINT = 88;
    pub const VK_FORMAT_R16G16B16_SINT = 89;
    pub const VK_FORMAT_R16G16B16_SFLOAT = 90;
    pub const VK_FORMAT_R16G16B16A16_UNORM = 91;
    pub const VK_FORMAT_R16G16B16A16_SNORM = 92;
    pub const VK_FORMAT_R16G16B16A16_USCALED = 93;
    pub const VK_FORMAT_R16G16B16A16_SSCALED = 94;
    pub const VK_FORMAT_R16G16B16A16_UINT = 95;
    pub const VK_FORMAT_R16G16B16A16_SINT = 96;
    pub const VK_FORMAT_R16G16B16A16_SFLOAT = 97;
    pub const VK_FORMAT_R32_UINT = 98;
    pub const VK_FORMAT_R32_SINT = 99;
    pub const VK_FORMAT_R32_SFLOAT = 100;
    pub const VK_FORMAT_R32G32_UINT = 101;
    pub const VK_FORMAT_R32G32_SINT = 102;
    pub const VK_FORMAT_R32G32_SFLOAT = 103;
    pub const VK_FORMAT_R32G32B32_UINT = 104;
    pub const VK_FORMAT_R32G32B32_SINT = 105;
    pub const VK_FORMAT_R32G32B32_SFLOAT = 106;
    pub const VK_FORMAT_R32G32B32A32_UINT = 107;
    pub const VK_FORMAT_R32G32B32A32_SINT = 108;
    pub const VK_FORMAT_R32G32B32A32_SFLOAT = 109;
    pub const VK_FORMAT_R64_UINT = 110;
    pub const VK_FORMAT_R64_SINT = 111;
    pub const VK_FORMAT_R64_SFLOAT = 112;
    pub const VK_FORMAT_R64G64_UINT = 113;
    pub const VK_FORMAT_R64G64_SINT = 114;
    pub const VK_FORMAT_R64G64_SFLOAT = 115;
    pub const VK_FORMAT_R64G64B64_UINT = 116;
    pub const VK_FORMAT_R64G64B64_SINT = 117;
    pub const VK_FORMAT_R64G64B64_SFLOAT = 118;
    pub const VK_FORMAT_R64G64B64A64_UINT = 119;
    pub const VK_FORMAT_R64G64B64A64_SINT = 120;
    pub const VK_FORMAT_R64G64B64A64_SFLOAT = 121;
    pub const VK_FORMAT_B10G11R11_UFLOAT_PACK32 = 122;
    pub const VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 = 123;
    pub const VK_FORMAT_D16_UNORM = 124;
    pub const VK_FORMAT_X8_D24_UNORM_PACK32 = 125;
    pub const VK_FORMAT_D32_SFLOAT = 126;
    pub const VK_FORMAT_S8_UINT = 127;
    pub const VK_FORMAT_D16_UNORM_S8_UINT = 128;
    pub const VK_FORMAT_D24_UNORM_S8_UINT = 129;
    pub const VK_FORMAT_D32_SFLOAT_S8_UINT = 130;
    pub const VK_FORMAT_BC1_RGB_UNORM_BLOCK = 131;
    pub const VK_FORMAT_BC1_RGB_SRGB_BLOCK = 132;
    pub const VK_FORMAT_BC1_RGBA_UNORM_BLOCK = 133;
    pub const VK_FORMAT_BC1_RGBA_SRGB_BLOCK = 134;
    pub const VK_FORMAT_BC2_UNORM_BLOCK = 135;
    pub const VK_FORMAT_BC2_SRGB_BLOCK = 136;
    pub const VK_FORMAT_BC3_UNORM_BLOCK = 137;
    pub const VK_FORMAT_BC3_SRGB_BLOCK = 138;
    pub const VK_FORMAT_BC4_UNORM_BLOCK = 139;
    pub const VK_FORMAT_BC4_SNORM_BLOCK = 140;
    pub const VK_FORMAT_BC5_UNORM_BLOCK = 141;
    pub const VK_FORMAT_BC5_SNORM_BLOCK = 142;
    pub const VK_FORMAT_BC6H_UFLOAT_BLOCK = 143;
    pub const VK_FORMAT_BC6H_SFLOAT_BLOCK = 144;
    pub const VK_FORMAT_BC7_UNORM_BLOCK = 145;
    pub const VK_FORMAT_BC7_SRGB_BLOCK = 146;
    pub const VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK = 147;
    pub const VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK = 148;
    pub const VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK = 149;
    pub const VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK = 150;
    pub const VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK = 151;
    pub const VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK = 152;
    pub const VK_FORMAT_EAC_R11_UNORM_BLOCK = 153;
    pub const VK_FORMAT_EAC_R11_SNORM_BLOCK = 154;
    pub const VK_FORMAT_EAC_R11G11_UNORM_BLOCK = 155;
    pub const VK_FORMAT_EAC_R11G11_SNORM_BLOCK = 156;
    pub const VK_FORMAT_ASTC_4x4_UNORM_BLOCK = 157;
    pub const VK_FORMAT_ASTC_4x4_SRGB_BLOCK = 158;
    pub const VK_FORMAT_ASTC_5x4_UNORM_BLOCK = 159;
    pub const VK_FORMAT_ASTC_5x4_SRGB_BLOCK = 160;
    pub const VK_FORMAT_ASTC_5x5_UNORM_BLOCK = 161;
    pub const VK_FORMAT_ASTC_5x5_SRGB_BLOCK = 162;
    pub const VK_FORMAT_ASTC_6x5_UNORM_BLOCK = 163;
    pub const VK_FORMAT_ASTC_6x5_SRGB_BLOCK = 164;
    pub const VK_FORMAT_ASTC_6x6_UNORM_BLOCK = 165;
    pub const VK_FORMAT_ASTC_6x6_SRGB_BLOCK = 166;
    pub const VK_FORMAT_ASTC_8x5_UNORM_BLOCK = 167;
    pub const VK_FORMAT_ASTC_8x5_SRGB_BLOCK = 168;
    pub const VK_FORMAT_ASTC_8x6_UNORM_BLOCK = 169;
    pub const VK_FORMAT_ASTC_8x6_SRGB_BLOCK = 170;
    pub const VK_FORMAT_ASTC_8x8_UNORM_BLOCK = 171;
    pub const VK_FORMAT_ASTC_8x8_SRGB_BLOCK = 172;
    pub const VK_FORMAT_ASTC_10x5_UNORM_BLOCK = 173;
    pub const VK_FORMAT_ASTC_10x5_SRGB_BLOCK = 174;
    pub const VK_FORMAT_ASTC_10x6_UNORM_BLOCK = 175;
    pub const VK_FORMAT_ASTC_10x6_SRGB_BLOCK = 176;
    pub const VK_FORMAT_ASTC_10x8_UNORM_BLOCK = 177;
    pub const VK_FORMAT_ASTC_10x8_SRGB_BLOCK = 178;
    pub const VK_FORMAT_ASTC_10x10_UNORM_BLOCK = 179;
    pub const VK_FORMAT_ASTC_10x10_SRGB_BLOCK = 180;
    pub const VK_FORMAT_ASTC_12x10_UNORM_BLOCK = 181;
    pub const VK_FORMAT_ASTC_12x10_SRGB_BLOCK = 182;
    pub const VK_FORMAT_ASTC_12x12_UNORM_BLOCK = 183;
    pub const VK_FORMAT_ASTC_12x12_SRGB_BLOCK = 184;
}

pub struct VkStructureType(i32); //Structure type enumerant
impl VkStructureType {
    pub const VK_STRUCTURE_TYPE_APPLICATION_INFO = 0;
    pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO = 1;
    pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO = 2;
    pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO = 3;
    pub const VK_STRUCTURE_TYPE_SUBMIT_INFO = 4;
    pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO = 5;
    pub const VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE = 6;
    pub const VK_STRUCTURE_TYPE_BIND_SPARSE_INFO = 7;
    pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO = 8;
    pub const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO = 9;
    pub const VK_STRUCTURE_TYPE_EVENT_CREATE_INFO = 10;
    pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO = 11;
    pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO = 12;
    pub const VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO = 13;
    pub const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO = 14;
    pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO = 15;
    pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO = 16;
    pub const VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO = 17;
    pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO = 18;
    pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO = 19;
    pub const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO = 20;
    pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO = 21;
    pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO = 22;
    pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO = 23;
    pub const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO = 24;
    pub const VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO = 25;
    pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO = 26;
    pub const VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO = 27;
    pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO = 28;
    pub const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO = 29;
    pub const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO = 30;
    pub const VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO = 31;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO = 32;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO = 33;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO = 34;
    pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET = 35;
    pub const VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET = 36;
    pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO = 37;
    pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO = 38;
    pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO = 39;
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO = 40;
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO = 41;
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO = 42;
    pub const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO = 43;
    pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER = 44;
    pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER = 45;
    pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER = 46;
    pub const VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO = 47;
    pub const VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO = 48;
}

pub struct VkSubpassContents(i32); //
impl VkSubpassContents {
    pub const VK_SUBPASS_CONTENTS_INLINE = 0;
    pub const VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS = 1;
}

pub struct VkResult(i32); //API result codes
impl VkResult {
    pub const VK_SUCCESS = 0;
    pub const VK_NOT_READY = 1;
    pub const VK_TIMEOUT = 2;
    pub const VK_EVENT_SET = 3;
    pub const VK_EVENT_RESET = 4;
    pub const VK_INCOMPLETE = 5;
    pub const VK_ERROR_OUT_OF_HOST_MEMORY = -1;
    pub const VK_ERROR_OUT_OF_DEVICE_MEMORY = -2;
    pub const VK_ERROR_INITIALIZATION_FAILED = -3;
    pub const VK_ERROR_DEVICE_LOST = -4;
    pub const VK_ERROR_MEMORY_MAP_FAILED = -5;
    pub const VK_ERROR_LAYER_NOT_PRESENT = -6;
    pub const VK_ERROR_EXTENSION_NOT_PRESENT = -7;
    pub const VK_ERROR_FEATURE_NOT_PRESENT = -8;
    pub const VK_ERROR_INCOMPATIBLE_DRIVER = -9;
    pub const VK_ERROR_TOO_MANY_OBJECTS = -10;
    pub const VK_ERROR_FORMAT_NOT_SUPPORTED = -11;
    pub const VK_ERROR_FRAGMENTED_POOL = -12;
    pub const VK_ERROR_UNKNOWN = -13;
}

pub struct VkDynamicState(i32); //
impl VkDynamicState {
    pub const VK_DYNAMIC_STATE_VIEWPORT = 0;
    pub const VK_DYNAMIC_STATE_SCISSOR = 1;
    pub const VK_DYNAMIC_STATE_LINE_WIDTH = 2;
    pub const VK_DYNAMIC_STATE_DEPTH_BIAS = 3;
    pub const VK_DYNAMIC_STATE_BLEND_CONSTANTS = 4;
    pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS = 5;
    pub const VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK = 6;
    pub const VK_DYNAMIC_STATE_STENCIL_WRITE_MASK = 7;
    pub const VK_DYNAMIC_STATE_STENCIL_REFERENCE = 8;
}

pub struct VkDescriptorUpdateTemplateType(i32); //
impl VkDescriptorUpdateTemplateType {
    pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET = 0;
}

pub struct VkObjectType(i32); //Enums to track objects of various types - also see objtypeenum attributes on type tags
impl VkObjectType {
    pub const VK_OBJECT_TYPE_UNKNOWN = 0;
    pub const VK_OBJECT_TYPE_INSTANCE = 1;
    pub const VK_OBJECT_TYPE_PHYSICAL_DEVICE = 2;
    pub const VK_OBJECT_TYPE_DEVICE = 3;
    pub const VK_OBJECT_TYPE_QUEUE = 4;
    pub const VK_OBJECT_TYPE_SEMAPHORE = 5;
    pub const VK_OBJECT_TYPE_COMMAND_BUFFER = 6;
    pub const VK_OBJECT_TYPE_FENCE = 7;
    pub const VK_OBJECT_TYPE_DEVICE_MEMORY = 8;
    pub const VK_OBJECT_TYPE_BUFFER = 9;
    pub const VK_OBJECT_TYPE_IMAGE = 10;
    pub const VK_OBJECT_TYPE_EVENT = 11;
    pub const VK_OBJECT_TYPE_QUERY_POOL = 12;
    pub const VK_OBJECT_TYPE_BUFFER_VIEW = 13;
    pub const VK_OBJECT_TYPE_IMAGE_VIEW = 14;
    pub const VK_OBJECT_TYPE_SHADER_MODULE = 15;
    pub const VK_OBJECT_TYPE_PIPELINE_CACHE = 16;
    pub const VK_OBJECT_TYPE_PIPELINE_LAYOUT = 17;
    pub const VK_OBJECT_TYPE_RENDER_PASS = 18;
    pub const VK_OBJECT_TYPE_PIPELINE = 19;
    pub const VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT = 20;
    pub const VK_OBJECT_TYPE_SAMPLER = 21;
    pub const VK_OBJECT_TYPE_DESCRIPTOR_POOL = 22;
    pub const VK_OBJECT_TYPE_DESCRIPTOR_SET = 23;
    pub const VK_OBJECT_TYPE_FRAMEBUFFER = 24;
    pub const VK_OBJECT_TYPE_COMMAND_POOL = 25;
}

pub struct VkRayTracingInvocationReorderModeEXT(i32); //
impl VkRayTracingInvocationReorderModeEXT {
    pub const VK_RAY_TRACING_INVOCATION_REORDER_MODE_NONE_EXT = 0;
    pub const VK_RAY_TRACING_INVOCATION_REORDER_MODE_REORDER_EXT = 1;
}

pub struct VkRayTracingLssIndexingModeNV(i32); //
impl VkRayTracingLssIndexingModeNV {
    pub const VK_RAY_TRACING_LSS_INDEXING_MODE_LIST_NV = 0;
    pub const VK_RAY_TRACING_LSS_INDEXING_MODE_SUCCESSIVE_NV = 1;
}

pub struct VkRayTracingLssPrimitiveEndCapsModeNV(i32); //
impl VkRayTracingLssPrimitiveEndCapsModeNV {
    pub const VK_RAY_TRACING_LSS_PRIMITIVE_END_CAPS_MODE_NONE_NV = 0;
    pub const VK_RAY_TRACING_LSS_PRIMITIVE_END_CAPS_MODE_CHAINED_NV = 1;
}

pub struct VkDirectDriverLoadingModeLUNARG(i32); //
impl VkDirectDriverLoadingModeLUNARG {
    pub const VK_DIRECT_DRIVER_LOADING_MODE_EXCLUSIVE_LUNARG = 0;
    pub const VK_DIRECT_DRIVER_LOADING_MODE_INCLUSIVE_LUNARG = 1;
}

pub struct VkAntiLagModeAMD(i32); //
impl VkAntiLagModeAMD {
    pub const VK_ANTI_LAG_MODE_DRIVER_CONTROL_AMD = 0;
    pub const VK_ANTI_LAG_MODE_ON_AMD = 1;
    pub const VK_ANTI_LAG_MODE_OFF_AMD = 2;
}

pub struct VkAntiLagStageAMD(i32); //
impl VkAntiLagStageAMD {
    pub const VK_ANTI_LAG_STAGE_INPUT_AMD = 0;
    pub const VK_ANTI_LAG_STAGE_PRESENT_AMD = 1;
}

pub type VkQueueFlagBits = ...; //
impl VkQueueFlagBits {
    pub const VK_QUEUE_GRAPHICS_BIT = 1;
    pub const VK_QUEUE_COMPUTE_BIT = 2;
    pub const VK_QUEUE_TRANSFER_BIT = 4;
    pub const VK_QUEUE_SPARSE_BINDING_BIT = 8;
}

pub type VkCullModeFlagBits = ...; //
impl VkCullModeFlagBits {
    pub const VK_CULL_MODE_NONE = 0;
    pub const VK_CULL_MODE_FRONT_BIT = 1;
    pub const VK_CULL_MODE_BACK_BIT = 2;
    pub const VK_CULL_MODE_FRONT_AND_BACK = 0;
}

pub type VkRenderPassCreateFlagBits = ...; //
impl VkRenderPassCreateFlagBits {
}

pub type VkDeviceQueueCreateFlagBits = ...; //
impl VkDeviceQueueCreateFlagBits {
}

pub type VkMemoryPropertyFlagBits = ...; //
impl VkMemoryPropertyFlagBits {
    pub const VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT = 1;
    pub const VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT = 2;
    pub const VK_MEMORY_PROPERTY_HOST_COHERENT_BIT = 4;
    pub const VK_MEMORY_PROPERTY_HOST_CACHED_BIT = 8;
    pub const VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = 16;
}

pub type VkMemoryHeapFlagBits = ...; //
impl VkMemoryHeapFlagBits {
    pub const VK_MEMORY_HEAP_DEVICE_LOCAL_BIT = 1;
}

pub type VkAccessFlagBits = ...; //
impl VkAccessFlagBits {
    pub const VK_ACCESS_INDIRECT_COMMAND_READ_BIT = 1;
    pub const VK_ACCESS_INDEX_READ_BIT = 2;
    pub const VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT = 4;
    pub const VK_ACCESS_UNIFORM_READ_BIT = 8;
    pub const VK_ACCESS_INPUT_ATTACHMENT_READ_BIT = 16;
    pub const VK_ACCESS_SHADER_READ_BIT = 32;
    pub const VK_ACCESS_SHADER_WRITE_BIT = 64;
    pub const VK_ACCESS_COLOR_ATTACHMENT_READ_BIT = 128;
    pub const VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT = 256;
    pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT = 512;
    pub const VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 1024;
    pub const VK_ACCESS_TRANSFER_READ_BIT = 2048;
    pub const VK_ACCESS_TRANSFER_WRITE_BIT = 4096;
    pub const VK_ACCESS_HOST_READ_BIT = 8192;
    pub const VK_ACCESS_HOST_WRITE_BIT = 16384;
    pub const VK_ACCESS_MEMORY_READ_BIT = 32768;
    pub const VK_ACCESS_MEMORY_WRITE_BIT = 65536;
}

pub type VkBufferUsageFlagBits = ...; //
impl VkBufferUsageFlagBits {
    pub const VK_BUFFER_USAGE_TRANSFER_SRC_BIT = 1;
    pub const VK_BUFFER_USAGE_TRANSFER_DST_BIT = 2;
    pub const VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT = 4;
    pub const VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT = 8;
    pub const VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT = 16;
    pub const VK_BUFFER_USAGE_STORAGE_BUFFER_BIT = 32;
    pub const VK_BUFFER_USAGE_INDEX_BUFFER_BIT = 64;
    pub const VK_BUFFER_USAGE_VERTEX_BUFFER_BIT = 128;
    pub const VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT = 256;
}

pub type VkBufferUsageFlagBits2 = ...; //
impl VkBufferUsageFlagBits2 {
    pub const VK_BUFFER_USAGE_2_TRANSFER_SRC_BIT = 1;
    pub const VK_BUFFER_USAGE_2_TRANSFER_DST_BIT = 2;
    pub const VK_BUFFER_USAGE_2_UNIFORM_TEXEL_BUFFER_BIT = 4;
    pub const VK_BUFFER_USAGE_2_STORAGE_TEXEL_BUFFER_BIT = 8;
    pub const VK_BUFFER_USAGE_2_UNIFORM_BUFFER_BIT = 16;
    pub const VK_BUFFER_USAGE_2_STORAGE_BUFFER_BIT = 32;
    pub const VK_BUFFER_USAGE_2_INDEX_BUFFER_BIT = 64;
    pub const VK_BUFFER_USAGE_2_VERTEX_BUFFER_BIT = 128;
    pub const VK_BUFFER_USAGE_2_INDIRECT_BUFFER_BIT = 256;
}

pub type VkBufferCreateFlagBits = ...; //
impl VkBufferCreateFlagBits {
    pub const VK_BUFFER_CREATE_SPARSE_BINDING_BIT = 1;
    pub const VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT = 2;
    pub const VK_BUFFER_CREATE_SPARSE_ALIASED_BIT = 4;
}

pub type VkShaderStageFlagBits = ...; //
impl VkShaderStageFlagBits {
    pub const VK_SHADER_STAGE_VERTEX_BIT = 1;
    pub const VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT = 2;
    pub const VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT = 4;
    pub const VK_SHADER_STAGE_GEOMETRY_BIT = 8;
    pub const VK_SHADER_STAGE_FRAGMENT_BIT = 16;
    pub const VK_SHADER_STAGE_COMPUTE_BIT = 32;
    pub const VK_SHADER_STAGE_ALL_GRAPHICS = 0;
    pub const VK_SHADER_STAGE_ALL = 0;
}

pub type VkImageUsageFlagBits = ...; //
impl VkImageUsageFlagBits {
    pub const VK_IMAGE_USAGE_TRANSFER_SRC_BIT = 1;
    pub const VK_IMAGE_USAGE_TRANSFER_DST_BIT = 2;
    pub const VK_IMAGE_USAGE_SAMPLED_BIT = 4;
    pub const VK_IMAGE_USAGE_STORAGE_BIT = 8;
    pub const VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT = 16;
    pub const VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT = 32;
    pub const VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT = 64;
    pub const VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT = 128;
}

pub type VkImageUsageFlagBits2KHR = ...; //
impl VkImageUsageFlagBits2KHR {
    pub const VK_IMAGE_USAGE_2_TRANSFER_SRC_BIT_KHR = 1;
    pub const VK_IMAGE_USAGE_2_TRANSFER_DST_BIT_KHR = 2;
    pub const VK_IMAGE_USAGE_2_SAMPLED_BIT_KHR = 4;
    pub const VK_IMAGE_USAGE_2_STORAGE_BIT_KHR = 8;
    pub const VK_IMAGE_USAGE_2_COLOR_ATTACHMENT_BIT_KHR = 16;
    pub const VK_IMAGE_USAGE_2_DEPTH_STENCIL_ATTACHMENT_BIT_KHR = 32;
    pub const VK_IMAGE_USAGE_2_TRANSIENT_ATTACHMENT_BIT_KHR = 64;
    pub const VK_IMAGE_USAGE_2_INPUT_ATTACHMENT_BIT_KHR = 128;
}

pub type VkImageCreateFlagBits = ...; //
impl VkImageCreateFlagBits {
    pub const VK_IMAGE_CREATE_SPARSE_BINDING_BIT = 1;
    pub const VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT = 2;
    pub const VK_IMAGE_CREATE_SPARSE_ALIASED_BIT = 4;
    pub const VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT = 8;
    pub const VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT = 16;
}

pub type VkImageCreateFlagBits2KHR = ...; //
impl VkImageCreateFlagBits2KHR {
    pub const VK_IMAGE_CREATE_2_SPARSE_BINDING_BIT_KHR = 1;
    pub const VK_IMAGE_CREATE_2_SPARSE_RESIDENCY_BIT_KHR = 2;
    pub const VK_IMAGE_CREATE_2_SPARSE_ALIASED_BIT_KHR = 4;
    pub const VK_IMAGE_CREATE_2_MUTABLE_FORMAT_BIT_KHR = 8;
    pub const VK_IMAGE_CREATE_2_CUBE_COMPATIBLE_BIT_KHR = 16;
}

pub type VkImageViewCreateFlagBits = ...; //
impl VkImageViewCreateFlagBits {
}

pub type VkSamplerCreateFlagBits = ...; //
impl VkSamplerCreateFlagBits {
}

pub type VkPipelineCreateFlagBits = ...; //
impl VkPipelineCreateFlagBits {
    pub const VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT = 1;
    pub const VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT = 2;
    pub const VK_PIPELINE_CREATE_DERIVATIVE_BIT = 4;
}

pub type VkPipelineCreateFlagBits2 = ...; //
impl VkPipelineCreateFlagBits2 {
    pub const VK_PIPELINE_CREATE_2_DISABLE_OPTIMIZATION_BIT = 1;
    pub const VK_PIPELINE_CREATE_2_ALLOW_DERIVATIVES_BIT = 2;
    pub const VK_PIPELINE_CREATE_2_DERIVATIVE_BIT = 4;
    pub const VK_PIPELINE_CREATE_2_VIEW_INDEX_FROM_DEVICE_INDEX_BIT = 8;
    pub const VK_PIPELINE_CREATE_2_DISPATCH_BASE_BIT = 16;
    pub const VK_PIPELINE_CREATE_2_FAIL_ON_PIPELINE_COMPILE_REQUIRED_BIT = 256;
    pub const VK_PIPELINE_CREATE_2_EARLY_RETURN_ON_FAILURE_BIT = 512;
    pub const VK_PIPELINE_CREATE_2_NO_PROTECTED_ACCESS_BIT = 134217728;
    pub const VK_PIPELINE_CREATE_2_PROTECTED_ACCESS_ONLY_BIT = 1073741824;
}

pub type VkPipelineShaderStageCreateFlagBits = ...; //
impl VkPipelineShaderStageCreateFlagBits {
}

pub type VkColorComponentFlagBits = ...; //
impl VkColorComponentFlagBits {
    pub const VK_COLOR_COMPONENT_R_BIT = 1;
    pub const VK_COLOR_COMPONENT_G_BIT = 2;
    pub const VK_COLOR_COMPONENT_B_BIT = 4;
    pub const VK_COLOR_COMPONENT_A_BIT = 8;
}

pub type VkFenceCreateFlagBits = ...; //
impl VkFenceCreateFlagBits {
    pub const VK_FENCE_CREATE_SIGNALED_BIT = 1;
}

pub type VkSemaphoreCreateFlagBits = ...; //
impl VkSemaphoreCreateFlagBits {
}

pub type VkFormatFeatureFlagBits = ...; //
impl VkFormatFeatureFlagBits {
    pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT = 1;
    pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT = 2;
    pub const VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT = 4;
    pub const VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT = 8;
    pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT = 16;
    pub const VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 32;
    pub const VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT = 64;
    pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT = 128;
    pub const VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT = 256;
    pub const VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT = 512;
    pub const VK_FORMAT_FEATURE_BLIT_SRC_BIT = 1024;
    pub const VK_FORMAT_FEATURE_BLIT_DST_BIT = 2048;
    pub const VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT = 4096;
}

pub type VkQueryControlFlagBits = ...; //
impl VkQueryControlFlagBits {
    pub const VK_QUERY_CONTROL_PRECISE_BIT = 1;
}

pub type VkQueryResultFlagBits = ...; //
impl VkQueryResultFlagBits {
    pub const VK_QUERY_RESULT_64_BIT = 1;
    pub const VK_QUERY_RESULT_WAIT_BIT = 2;
    pub const VK_QUERY_RESULT_WITH_AVAILABILITY_BIT = 4;
    pub const VK_QUERY_RESULT_PARTIAL_BIT = 8;
}

pub type VkCommandBufferUsageFlagBits = ...; //
impl VkCommandBufferUsageFlagBits {
    pub const VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT = 1;
    pub const VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT = 2;
    pub const VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT = 4;
}

pub type VkQueryPipelineStatisticFlagBits = ...; //
impl VkQueryPipelineStatisticFlagBits {
    pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT = 1;
    pub const VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT = 2;
    pub const VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT = 4;
    pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT = 8;
    pub const VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT = 16;
    pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT = 32;
    pub const VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT = 64;
    pub const VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT = 128;
    pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT = 256;
    pub const VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = 512;
    pub const VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT = 1024;
}

pub type VkMemoryMapFlagBits = ...; //
impl VkMemoryMapFlagBits {
}

pub type VkImageAspectFlagBits = ...; //
impl VkImageAspectFlagBits {
    pub const VK_IMAGE_ASPECT_COLOR_BIT = 1;
    pub const VK_IMAGE_ASPECT_DEPTH_BIT = 2;
    pub const VK_IMAGE_ASPECT_STENCIL_BIT = 4;
    pub const VK_IMAGE_ASPECT_METADATA_BIT = 8;
}

pub type VkSparseImageFormatFlagBits = ...; //
impl VkSparseImageFormatFlagBits {
    pub const VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT = 1;
    pub const VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT = 2;
    pub const VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT = 4;
}

pub type VkSparseMemoryBindFlagBits = ...; //
impl VkSparseMemoryBindFlagBits {
    pub const VK_SPARSE_MEMORY_BIND_METADATA_BIT = 1;
}

pub type VkPipelineStageFlagBits = ...; //
impl VkPipelineStageFlagBits {
    pub const VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT = 1;
    pub const VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT = 2;
    pub const VK_PIPELINE_STAGE_VERTEX_INPUT_BIT = 4;
    pub const VK_PIPELINE_STAGE_VERTEX_SHADER_BIT = 8;
    pub const VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT = 16;
    pub const VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT = 32;
    pub const VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT = 64;
    pub const VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT = 128;
    pub const VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT = 256;
    pub const VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT = 512;
    pub const VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT = 1024;
    pub const VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT = 2048;
    pub const VK_PIPELINE_STAGE_TRANSFER_BIT = 4096;
    pub const VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT = 8192;
    pub const VK_PIPELINE_STAGE_HOST_BIT = 16384;
    pub const VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT = 32768;
    pub const VK_PIPELINE_STAGE_ALL_COMMANDS_BIT = 65536;
}

pub type VkCommandPoolCreateFlagBits = ...; //
impl VkCommandPoolCreateFlagBits {
    pub const VK_COMMAND_POOL_CREATE_TRANSIENT_BIT = 1;
    pub const VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = 2;
}

pub type VkCommandPoolResetFlagBits = ...; //
impl VkCommandPoolResetFlagBits {
    pub const VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT = 1;
}

pub type VkCommandBufferResetFlagBits = ...; //
impl VkCommandBufferResetFlagBits {
    pub const VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT = 1;
}

pub type VkSampleCountFlagBits = ...; //
impl VkSampleCountFlagBits {
    pub const VK_SAMPLE_COUNT_1_BIT = 1;
    pub const VK_SAMPLE_COUNT_2_BIT = 2;
    pub const VK_SAMPLE_COUNT_4_BIT = 4;
    pub const VK_SAMPLE_COUNT_8_BIT = 8;
    pub const VK_SAMPLE_COUNT_16_BIT = 16;
    pub const VK_SAMPLE_COUNT_32_BIT = 32;
    pub const VK_SAMPLE_COUNT_64_BIT = 64;
}

pub type VkAttachmentDescriptionFlagBits = ...; //
impl VkAttachmentDescriptionFlagBits {
    pub const VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT = 1;
}

pub type VkStencilFaceFlagBits = ...; //
impl VkStencilFaceFlagBits {
    pub const VK_STENCIL_FACE_FRONT_BIT = 1;
    pub const VK_STENCIL_FACE_BACK_BIT = 2;
    pub const VK_STENCIL_FACE_FRONT_AND_BACK = 0;
    pub const VK_STENCIL_FRONT_AND_BACK = 0;
}

pub type VkDescriptorPoolCreateFlagBits = ...; //
impl VkDescriptorPoolCreateFlagBits {
    pub const VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT = 1;
}

pub type VkDependencyFlagBits = ...; //
impl VkDependencyFlagBits {
    pub const VK_DEPENDENCY_BY_REGION_BIT = 1;
}

pub struct VkSemaphoreType(i32); //
impl VkSemaphoreType {
    pub const VK_SEMAPHORE_TYPE_BINARY = 0;
    pub const VK_SEMAPHORE_TYPE_TIMELINE = 1;
}

pub type VkSemaphoreWaitFlagBits = ...; //
impl VkSemaphoreWaitFlagBits {
    pub const VK_SEMAPHORE_WAIT_ANY_BIT = 1;
}

pub struct VkPresentModeKHR(i32); //
impl VkPresentModeKHR {
    pub const VK_PRESENT_MODE_IMMEDIATE_KHR = 0;
    pub const VK_PRESENT_MODE_MAILBOX_KHR = 1;
    pub const VK_PRESENT_MODE_FIFO_KHR = 2;
    pub const VK_PRESENT_MODE_FIFO_RELAXED_KHR = 3;
}

pub struct VkColorSpaceKHR(i32); //
impl VkColorSpaceKHR {
    pub const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR = 0;
    pub const VK_COLORSPACE_SRGB_NONLINEAR_KHR = ;
}

pub type VkDisplayPlaneAlphaFlagBitsKHR = ...; //
impl VkDisplayPlaneAlphaFlagBitsKHR {
    pub const VK_DISPLAY_PLANE_ALPHA_OPAQUE_BIT_KHR = 1;
    pub const VK_DISPLAY_PLANE_ALPHA_GLOBAL_BIT_KHR = 2;
    pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_BIT_KHR = 4;
    pub const VK_DISPLAY_PLANE_ALPHA_PER_PIXEL_PREMULTIPLIED_BIT_KHR = 8;
}

pub type VkCompositeAlphaFlagBitsKHR = ...; //
impl VkCompositeAlphaFlagBitsKHR {
    pub const VK_COMPOSITE_ALPHA_OPAQUE_BIT_KHR = 1;
    pub const VK_COMPOSITE_ALPHA_PRE_MULTIPLIED_BIT_KHR = 2;
    pub const VK_COMPOSITE_ALPHA_POST_MULTIPLIED_BIT_KHR = 4;
    pub const VK_COMPOSITE_ALPHA_INHERIT_BIT_KHR = 8;
}

pub type VkSurfaceTransformFlagBitsKHR = ...; //
impl VkSurfaceTransformFlagBitsKHR {
    pub const VK_SURFACE_TRANSFORM_IDENTITY_BIT_KHR = 1;
    pub const VK_SURFACE_TRANSFORM_ROTATE_90_BIT_KHR = 2;
    pub const VK_SURFACE_TRANSFORM_ROTATE_180_BIT_KHR = 4;
    pub const VK_SURFACE_TRANSFORM_ROTATE_270_BIT_KHR = 8;
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_BIT_KHR = 16;
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_90_BIT_KHR = 32;
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_180_BIT_KHR = 64;
    pub const VK_SURFACE_TRANSFORM_HORIZONTAL_MIRROR_ROTATE_270_BIT_KHR = 128;
    pub const VK_SURFACE_TRANSFORM_INHERIT_BIT_KHR = 256;
}

pub struct VkDisplaySurfaceStereoTypeNV(i32); //
impl VkDisplaySurfaceStereoTypeNV {
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_NONE_NV = 0;
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_ONBOARD_DIN_NV = 1;
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_HDMI_3D_NV = 2;
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_INBAND_DISPLAYPORT_NV = 3;
}

pub type VkSwapchainImageUsageFlagBitsANDROID = ...; //
impl VkSwapchainImageUsageFlagBitsANDROID {
    pub const VK_SWAPCHAIN_IMAGE_USAGE_SHARED_BIT_ANDROID = 1;
}

pub struct VkTimeDomainKHR(i32); //
impl VkTimeDomainKHR {
    pub const VK_TIME_DOMAIN_DEVICE_KHR = 0;
    pub const VK_TIME_DOMAIN_CLOCK_MONOTONIC_KHR = 1;
    pub const VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_KHR = 2;
    pub const VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_KHR = 3;
}

pub type VkDebugReportFlagBitsEXT = ...; //
impl VkDebugReportFlagBitsEXT {
    pub const VK_DEBUG_REPORT_INFORMATION_BIT_EXT = 1;
    pub const VK_DEBUG_REPORT_WARNING_BIT_EXT = 2;
    pub const VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT = 4;
    pub const VK_DEBUG_REPORT_ERROR_BIT_EXT = 8;
    pub const VK_DEBUG_REPORT_DEBUG_BIT_EXT = 16;
}

pub struct VkDebugReportObjectTypeEXT(i32); //
impl VkDebugReportObjectTypeEXT {
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT = 0;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT = 1;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT = 2;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT = 3;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT = 4;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT = 5;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT = 6;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT = 7;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT = 8;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT = 9;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT = 10;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT = 11;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT = 12;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT = 13;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT = 14;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT = 15;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT = 16;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT = 17;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT = 18;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT = 19;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT = 20;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT = 21;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT = 22;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT = 23;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT = 24;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT = 25;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT = 26;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT = 27;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT = 28;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT = ;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT = 29;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT = 30;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT = 33;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT = ;
}

pub struct VkDeviceMemoryReportEventTypeEXT(i32); //
impl VkDeviceMemoryReportEventTypeEXT {
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT = 0;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT = 1;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT = 2;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT = 3;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT = 4;
}

pub struct VkRasterizationOrderAMD(i32); //
impl VkRasterizationOrderAMD {
    pub const VK_RASTERIZATION_ORDER_STRICT_AMD = 0;
    pub const VK_RASTERIZATION_ORDER_RELAXED_AMD = 1;
}

pub type VkExternalMemoryHandleTypeFlagBitsNV = ...; //
impl VkExternalMemoryHandleTypeFlagBitsNV {
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT_NV = 1;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT_NV = 2;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_BIT_NV = 4;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_IMAGE_KMT_BIT_NV = 8;
}

pub type VkExternalMemoryFeatureFlagBitsNV = ...; //
impl VkExternalMemoryFeatureFlagBitsNV {
    pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT_NV = 1;
    pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT_NV = 2;
    pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT_NV = 4;
}

pub type VkClusterAccelerationStructureIndexFormatFlagBitsNV = ...; //
impl VkClusterAccelerationStructureIndexFormatFlagBitsNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_8BIT_NV = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_16BIT_NV = 2;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_INDEX_FORMAT_32BIT_NV = 4;
}

pub struct VkClusterAccelerationStructureTypeNV(i32); //
impl VkClusterAccelerationStructureTypeNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_TYPE_CLUSTERS_BOTTOM_LEVEL_NV = 0;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_TYPE_TRIANGLE_CLUSTER_NV = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_TYPE_TRIANGLE_CLUSTER_TEMPLATE_NV = 2;
}

pub struct VkClusterAccelerationStructureOpTypeNV(i32); //
impl VkClusterAccelerationStructureOpTypeNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_MOVE_OBJECTS_NV = 0;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_CLUSTERS_BOTTOM_LEVEL_NV = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_TRIANGLE_CLUSTER_NV = 2;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_TRIANGLE_CLUSTER_TEMPLATE_NV = 3;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_INSTANTIATE_TRIANGLE_CLUSTER_NV = 4;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_GET_CLUSTER_TEMPLATE_INDICES_NV = 5;
}

pub struct VkClusterAccelerationStructureOpModeNV(i32); //
impl VkClusterAccelerationStructureOpModeNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_IMPLICIT_DESTINATIONS_NV = 0;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_EXPLICIT_DESTINATIONS_NV = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_COMPUTE_SIZES_NV = 2;
}

pub type VkClusterAccelerationStructureClusterFlagBitsNV = ...; //
impl VkClusterAccelerationStructureClusterFlagBitsNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_CLUSTER_ALLOW_DISABLE_OPACITY_MICROMAPS_NV = 1;
}

pub type VkClusterAccelerationStructureGeometryFlagBitsNV = ...; //
impl VkClusterAccelerationStructureGeometryFlagBitsNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_CULL_DISABLE_BIT_NV = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_NO_DUPLICATE_ANYHIT_INVOCATION_BIT_NV = 2;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_GEOMETRY_OPAQUE_BIT_NV = 4;
}

pub type VkClusterAccelerationStructureAddressResolutionFlagBitsNV = ...; //
impl VkClusterAccelerationStructureAddressResolutionFlagBitsNV {
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_NONE_NV = 0;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_IMPLICIT_DATA_BIT_NV = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SCRATCH_DATA_BIT_NV = 2;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_ADDRESS_ARRAY_BIT_NV = 4;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_DST_SIZES_ARRAY_BIT_NV = 8;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_ARRAY_BIT_NV = 16;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_ADDRESS_RESOLUTION_INDIRECTED_SRC_INFOS_COUNT_BIT_NV = 32;
}

pub struct VkValidationCheckEXT(i32); //
impl VkValidationCheckEXT {
    pub const VK_VALIDATION_CHECK_ALL_EXT = 0;
    pub const VK_VALIDATION_CHECK_SHADERS_EXT = 1;
}

pub struct VkValidationFeatureEnableEXT(i32); //
impl VkValidationFeatureEnableEXT {
    pub const VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT = 0;
    pub const VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT = 1;
    pub const VK_VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT = 2;
    pub const VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT = 3;
    pub const VK_VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT = 4;
}

pub struct VkValidationFeatureDisableEXT(i32); //
impl VkValidationFeatureDisableEXT {
    pub const VK_VALIDATION_FEATURE_DISABLE_ALL_EXT = 0;
    pub const VK_VALIDATION_FEATURE_DISABLE_SHADERS_EXT = 1;
    pub const VK_VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT = 2;
    pub const VK_VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT = 3;
    pub const VK_VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT = 4;
    pub const VK_VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT = 5;
    pub const VK_VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT = 6;
    pub const VK_VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE_EXT = 7;
}

pub struct VkLayerSettingTypeEXT(i32); //
impl VkLayerSettingTypeEXT {
    pub const VK_LAYER_SETTING_TYPE_BOOL32_EXT = 0;
    pub const VK_LAYER_SETTING_TYPE_INT32_EXT = 1;
    pub const VK_LAYER_SETTING_TYPE_INT64_EXT = 2;
    pub const VK_LAYER_SETTING_TYPE_UINT32_EXT = 3;
    pub const VK_LAYER_SETTING_TYPE_UINT64_EXT = 4;
    pub const VK_LAYER_SETTING_TYPE_FLOAT32_EXT = 5;
    pub const VK_LAYER_SETTING_TYPE_FLOAT64_EXT = 6;
    pub const VK_LAYER_SETTING_TYPE_STRING_EXT = 7;
}

pub type VkSubgroupFeatureFlagBits = ...; //
impl VkSubgroupFeatureFlagBits {
    pub const VK_SUBGROUP_FEATURE_BASIC_BIT = 1;
    pub const VK_SUBGROUP_FEATURE_VOTE_BIT = 2;
    pub const VK_SUBGROUP_FEATURE_ARITHMETIC_BIT = 4;
    pub const VK_SUBGROUP_FEATURE_BALLOT_BIT = 8;
    pub const VK_SUBGROUP_FEATURE_SHUFFLE_BIT = 16;
    pub const VK_SUBGROUP_FEATURE_SHUFFLE_RELATIVE_BIT = 32;
    pub const VK_SUBGROUP_FEATURE_CLUSTERED_BIT = 64;
    pub const VK_SUBGROUP_FEATURE_QUAD_BIT = 128;
}

pub type VkIndirectCommandsLayoutUsageFlagBitsNV = ...; //
impl VkIndirectCommandsLayoutUsageFlagBitsNV {
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_NV = 1;
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_INDEXED_SEQUENCES_BIT_NV = 2;
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_NV = 4;
}

pub type VkIndirectStateFlagBitsNV = ...; //
impl VkIndirectStateFlagBitsNV {
    pub const VK_INDIRECT_STATE_FLAG_FRONTFACE_BIT_NV = 1;
}

pub struct VkIndirectCommandsTokenTypeNV(i32); //
impl VkIndirectCommandsTokenTypeNV {
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV = 0;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV = 1;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV = 2;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV = 3;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV = 4;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV = 5;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV = 6;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV = 7;
}

pub type VkPrivateDataSlotCreateFlagBits = ...; //
impl VkPrivateDataSlotCreateFlagBits {
}

pub type VkDescriptorSetLayoutCreateFlagBits = ...; //
impl VkDescriptorSetLayoutCreateFlagBits {
}

pub type VkExternalMemoryHandleTypeFlagBits = ...; //
impl VkExternalMemoryHandleTypeFlagBits {
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_FD_BIT = 1;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_BIT = 2;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = 4;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_BIT = 8;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D11_TEXTURE_KMT_BIT = 16;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_HEAP_BIT = 32;
    pub const VK_EXTERNAL_MEMORY_HANDLE_TYPE_D3D12_RESOURCE_BIT = 64;
}

pub type VkExternalMemoryFeatureFlagBits = ...; //
impl VkExternalMemoryFeatureFlagBits {
    pub const VK_EXTERNAL_MEMORY_FEATURE_DEDICATED_ONLY_BIT = 1;
    pub const VK_EXTERNAL_MEMORY_FEATURE_EXPORTABLE_BIT = 2;
    pub const VK_EXTERNAL_MEMORY_FEATURE_IMPORTABLE_BIT = 4;
}

pub type VkExternalSemaphoreHandleTypeFlagBits = ...; //
impl VkExternalSemaphoreHandleTypeFlagBits {
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_FD_BIT = 1;
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_BIT = 2;
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = 4;
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D12_FENCE_BIT = 8;
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_D3D11_FENCE_BIT = 0;
    pub const VK_EXTERNAL_SEMAPHORE_HANDLE_TYPE_SYNC_FD_BIT = 16;
}

pub type VkExternalSemaphoreFeatureFlagBits = ...; //
impl VkExternalSemaphoreFeatureFlagBits {
    pub const VK_EXTERNAL_SEMAPHORE_FEATURE_EXPORTABLE_BIT = 1;
    pub const VK_EXTERNAL_SEMAPHORE_FEATURE_IMPORTABLE_BIT = 2;
}

pub type VkSemaphoreImportFlagBits = ...; //
impl VkSemaphoreImportFlagBits {
    pub const VK_SEMAPHORE_IMPORT_TEMPORARY_BIT = 1;
}

pub type VkExternalFenceHandleTypeFlagBits = ...; //
impl VkExternalFenceHandleTypeFlagBits {
    pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_FD_BIT = 1;
    pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_BIT = 2;
    pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_OPAQUE_WIN32_KMT_BIT = 4;
    pub const VK_EXTERNAL_FENCE_HANDLE_TYPE_SYNC_FD_BIT = 8;
}

pub type VkExternalFenceFeatureFlagBits = ...; //
impl VkExternalFenceFeatureFlagBits {
    pub const VK_EXTERNAL_FENCE_FEATURE_EXPORTABLE_BIT = 1;
    pub const VK_EXTERNAL_FENCE_FEATURE_IMPORTABLE_BIT = 2;
}

pub type VkFenceImportFlagBits = ...; //
impl VkFenceImportFlagBits {
    pub const VK_FENCE_IMPORT_TEMPORARY_BIT = 1;
}

pub type VkSurfaceCounterFlagBitsEXT = ...; //
impl VkSurfaceCounterFlagBitsEXT {
    pub const VK_SURFACE_COUNTER_VBLANK_BIT_EXT = 1;
    pub const VK_SURFACE_COUNTER_VBLANK_EXT = 0;
}

pub struct VkDisplayPowerStateEXT(i32); //
impl VkDisplayPowerStateEXT {
    pub const VK_DISPLAY_POWER_STATE_OFF_EXT = 0;
    pub const VK_DISPLAY_POWER_STATE_SUSPEND_EXT = 1;
    pub const VK_DISPLAY_POWER_STATE_ON_EXT = 2;
}

pub struct VkDeviceEventTypeEXT(i32); //
impl VkDeviceEventTypeEXT {
    pub const VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT = 0;
}

pub struct VkDisplayEventTypeEXT(i32); //
impl VkDisplayEventTypeEXT {
    pub const VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT = 0;
}

pub type VkPeerMemoryFeatureFlagBits = ...; //
impl VkPeerMemoryFeatureFlagBits {
    pub const VK_PEER_MEMORY_FEATURE_COPY_SRC_BIT = 1;
    pub const VK_PEER_MEMORY_FEATURE_COPY_DST_BIT = 2;
    pub const VK_PEER_MEMORY_FEATURE_GENERIC_SRC_BIT = 4;
    pub const VK_PEER_MEMORY_FEATURE_GENERIC_DST_BIT = 8;
}

pub type VkMemoryAllocateFlagBits = ...; //
impl VkMemoryAllocateFlagBits {
    pub const VK_MEMORY_ALLOCATE_DEVICE_MASK_BIT = 1;
}

pub type VkDeviceGroupPresentModeFlagBitsKHR = ...; //
impl VkDeviceGroupPresentModeFlagBitsKHR {
    pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_BIT_KHR = 1;
    pub const VK_DEVICE_GROUP_PRESENT_MODE_REMOTE_BIT_KHR = 2;
    pub const VK_DEVICE_GROUP_PRESENT_MODE_SUM_BIT_KHR = 4;
    pub const VK_DEVICE_GROUP_PRESENT_MODE_LOCAL_MULTI_DEVICE_BIT_KHR = 8;
}

pub type VkSwapchainCreateFlagBitsKHR = ...; //
impl VkSwapchainCreateFlagBitsKHR {
}

pub struct VkViewportCoordinateSwizzleNV(i32); //
impl VkViewportCoordinateSwizzleNV {
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV = 0;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV = 1;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV = 2;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV = 3;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV = 4;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV = 5;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV = 6;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV = 7;
}

pub struct VkDiscardRectangleModeEXT(i32); //
impl VkDiscardRectangleModeEXT {
    pub const VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT = 0;
    pub const VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT = 1;
}

pub type VkSubpassDescriptionFlagBits = ...; //
impl VkSubpassDescriptionFlagBits {
}

pub struct VkPointClippingBehavior(i32); //
impl VkPointClippingBehavior {
    pub const VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES = 0;
    pub const VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY = 1;
}

pub struct VkSamplerReductionMode(i32); //
impl VkSamplerReductionMode {
    pub const VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE = 0;
    pub const VK_SAMPLER_REDUCTION_MODE_MIN = 1;
    pub const VK_SAMPLER_REDUCTION_MODE_MAX = 2;
}

pub struct VkTessellationDomainOrigin(i32); //
impl VkTessellationDomainOrigin {
    pub const VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT = 0;
    pub const VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT = 1;
}

pub struct VkSamplerYcbcrModelConversion(i32); //
impl VkSamplerYcbcrModelConversion {
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY = 0;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY = 1;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709 = 2;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601 = 3;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020 = 4;
}

pub struct VkSamplerYcbcrRange(i32); //
impl VkSamplerYcbcrRange {
    pub const VK_SAMPLER_YCBCR_RANGE_ITU_FULL = 0;
    pub const VK_SAMPLER_YCBCR_RANGE_ITU_NARROW = 1;
}

pub struct VkChromaLocation(i32); //
impl VkChromaLocation {
    pub const VK_CHROMA_LOCATION_COSITED_EVEN = 0;
    pub const VK_CHROMA_LOCATION_MIDPOINT = 1;
}

pub struct VkBlendOverlapEXT(i32); //
impl VkBlendOverlapEXT {
    pub const VK_BLEND_OVERLAP_UNCORRELATED_EXT = 0;
    pub const VK_BLEND_OVERLAP_DISJOINT_EXT = 1;
    pub const VK_BLEND_OVERLAP_CONJOINT_EXT = 2;
}

pub struct VkCoverageModulationModeNV(i32); //
impl VkCoverageModulationModeNV {
    pub const VK_COVERAGE_MODULATION_MODE_NONE_NV = 0;
    pub const VK_COVERAGE_MODULATION_MODE_RGB_NV = 1;
    pub const VK_COVERAGE_MODULATION_MODE_ALPHA_NV = 2;
    pub const VK_COVERAGE_MODULATION_MODE_RGBA_NV = 3;
}

pub struct VkCoverageReductionModeNV(i32); //
impl VkCoverageReductionModeNV {
    pub const VK_COVERAGE_REDUCTION_MODE_MERGE_NV = 0;
    pub const VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV = 1;
}

pub struct VkValidationCacheHeaderVersionEXT(i32); //
impl VkValidationCacheHeaderVersionEXT {
    pub const VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT = 1;
}

pub struct VkShaderInfoTypeAMD(i32); //
impl VkShaderInfoTypeAMD {
    pub const VK_SHADER_INFO_TYPE_STATISTICS_AMD = 0;
    pub const VK_SHADER_INFO_TYPE_BINARY_AMD = 1;
    pub const VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD = 2;
}

pub struct VkQueueGlobalPriority(i32); //
impl VkQueueGlobalPriority {
    pub const VK_QUEUE_GLOBAL_PRIORITY_LOW = 128;
    pub const VK_QUEUE_GLOBAL_PRIORITY_MEDIUM = 256;
    pub const VK_QUEUE_GLOBAL_PRIORITY_HIGH = 512;
    pub const VK_QUEUE_GLOBAL_PRIORITY_REALTIME = 1024;
}

pub type VkDebugUtilsMessageSeverityFlagBitsEXT = ...; //
impl VkDebugUtilsMessageSeverityFlagBitsEXT {
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT = 1;
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT = 16;
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT = 256;
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT = 4096;
}

pub type VkDebugUtilsMessageTypeFlagBitsEXT = ...; //
impl VkDebugUtilsMessageTypeFlagBitsEXT {
    pub const VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT = 1;
    pub const VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT = 2;
    pub const VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT = 4;
}

pub struct VkConservativeRasterizationModeEXT(i32); //
impl VkConservativeRasterizationModeEXT {
    pub const VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT = 0;
    pub const VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT = 1;
    pub const VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT = 2;
}

pub type VkDescriptorBindingFlagBits = ...; //
impl VkDescriptorBindingFlagBits {
    pub const VK_DESCRIPTOR_BINDING_UPDATE_AFTER_BIND_BIT = 1;
    pub const VK_DESCRIPTOR_BINDING_UPDATE_UNUSED_WHILE_PENDING_BIT = 2;
    pub const VK_DESCRIPTOR_BINDING_PARTIALLY_BOUND_BIT = 4;
    pub const VK_DESCRIPTOR_BINDING_VARIABLE_DESCRIPTOR_COUNT_BIT = 8;
}

pub struct VkVendorId(i32); //
impl VkVendorId {
    pub const VK_VENDOR_ID_KHRONOS = 0x10000;
    pub const VK_VENDOR_ID_VIV = 0x10001;
    pub const VK_VENDOR_ID_VSI = 0x10002;
    pub const VK_VENDOR_ID_KAZAN = 0x10003;
    pub const VK_VENDOR_ID_CODEPLAY = 0x10004;
    pub const VK_VENDOR_ID_MESA = 0x10005;
    pub const VK_VENDOR_ID_POCL = 0x10006;
    pub const VK_VENDOR_ID_MOBILEYE = 0x10007;
    pub const VK_VENDOR_ID_APE = 0x10008;
}

pub struct VkDriverId(i32); //
impl VkDriverId {
    pub const VK_DRIVER_ID_AMD_PROPRIETARY = 1;
    pub const VK_DRIVER_ID_AMD_OPEN_SOURCE = 2;
    pub const VK_DRIVER_ID_MESA_RADV = 3;
    pub const VK_DRIVER_ID_NVIDIA_PROPRIETARY = 4;
    pub const VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS = 5;
    pub const VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA = 6;
    pub const VK_DRIVER_ID_IMAGINATION_PROPRIETARY = 7;
    pub const VK_DRIVER_ID_QUALCOMM_PROPRIETARY = 8;
    pub const VK_DRIVER_ID_ARM_PROPRIETARY = 9;
    pub const VK_DRIVER_ID_GOOGLE_SWIFTSHADER = 10;
    pub const VK_DRIVER_ID_GGP_PROPRIETARY = 11;
    pub const VK_DRIVER_ID_BROADCOM_PROPRIETARY = 12;
    pub const VK_DRIVER_ID_MESA_LLVMPIPE = 13;
    pub const VK_DRIVER_ID_MOLTENVK = 14;
    pub const VK_DRIVER_ID_COREAVI_PROPRIETARY = 15;
    pub const VK_DRIVER_ID_JUICE_PROPRIETARY = 16;
    pub const VK_DRIVER_ID_VERISILICON_PROPRIETARY = 17;
    pub const VK_DRIVER_ID_MESA_TURNIP = 18;
    pub const VK_DRIVER_ID_MESA_V3DV = 19;
    pub const VK_DRIVER_ID_MESA_PANVK = 20;
    pub const VK_DRIVER_ID_SAMSUNG_PROPRIETARY = 21;
    pub const VK_DRIVER_ID_MESA_VENUS = 22;
    pub const VK_DRIVER_ID_MESA_DOZEN = 23;
    pub const VK_DRIVER_ID_MESA_NVK = 24;
    pub const VK_DRIVER_ID_IMAGINATION_OPEN_SOURCE_MESA = 25;
    pub const VK_DRIVER_ID_MESA_HONEYKRISP = 26;
    pub const VK_DRIVER_ID_VULKAN_SC_EMULATION_ON_VULKAN = 27;
    pub const VK_DRIVER_ID_MESA_KOSMICKRISP = 28;
    pub const VK_DRIVER_ID_MESA_GFXSTREAM = 29;
    pub const VK_DRIVER_ID_APE_SOFT = 30;
    pub const VK_DRIVER_ID_RESERVED_31 = 31;
}

pub type VkConditionalRenderingFlagBitsEXT = ...; //
impl VkConditionalRenderingFlagBitsEXT {
    pub const VK_CONDITIONAL_RENDERING_INVERTED_BIT_EXT = 1;
}

pub type VkResolveModeFlagBits = ...; //
impl VkResolveModeFlagBits {
    pub const VK_RESOLVE_MODE_NONE = 0;
    pub const VK_RESOLVE_MODE_SAMPLE_ZERO_BIT = 1;
    pub const VK_RESOLVE_MODE_AVERAGE_BIT = 2;
    pub const VK_RESOLVE_MODE_MIN_BIT = 4;
    pub const VK_RESOLVE_MODE_MAX_BIT = 8;
}

pub struct VkShadingRatePaletteEntryNV(i32); //
impl VkShadingRatePaletteEntryNV {
    pub const VK_SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS_NV = 0;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV = 1;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL_NV = 2;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL_NV = 3;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL_NV = 4;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL_NV = 5;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS_NV = 6;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS_NV = 7;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS_NV = 8;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS_NV = 9;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS_NV = 10;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS_NV = 11;
}

pub struct VkCoarseSampleOrderTypeNV(i32); //
impl VkCoarseSampleOrderTypeNV {
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV = 0;
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV = 1;
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV = 2;
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV = 3;
}

pub type VkGeometryInstanceFlagBitsKHR = ...; //
impl VkGeometryInstanceFlagBitsKHR {
    pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FACING_CULL_DISABLE_BIT_KHR = 1;
    pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FLIP_FACING_BIT_KHR = 2;
    pub const VK_GEOMETRY_INSTANCE_FORCE_OPAQUE_BIT_KHR = 4;
    pub const VK_GEOMETRY_INSTANCE_FORCE_NO_OPAQUE_BIT_KHR = 8;
    pub const VK_GEOMETRY_INSTANCE_TRIANGLE_FRONT_COUNTERCLOCKWISE_BIT_KHR = 0;
}

pub type VkGeometryFlagBitsKHR = ...; //
impl VkGeometryFlagBitsKHR {
    pub const VK_GEOMETRY_OPAQUE_BIT_KHR = 1;
    pub const VK_GEOMETRY_NO_DUPLICATE_ANY_HIT_INVOCATION_BIT_KHR = 2;
}

pub type VkBuildAccelerationStructureFlagBitsKHR = ...; //
impl VkBuildAccelerationStructureFlagBitsKHR {
    pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_UPDATE_BIT_KHR = 1;
    pub const VK_BUILD_ACCELERATION_STRUCTURE_ALLOW_COMPACTION_BIT_KHR = 2;
    pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_TRACE_BIT_KHR = 4;
    pub const VK_BUILD_ACCELERATION_STRUCTURE_PREFER_FAST_BUILD_BIT_KHR = 8;
    pub const VK_BUILD_ACCELERATION_STRUCTURE_LOW_MEMORY_BIT_KHR = 16;
}

pub type VkAccelerationStructureCreateFlagBitsKHR = ...; //
impl VkAccelerationStructureCreateFlagBitsKHR {
    pub const VK_ACCELERATION_STRUCTURE_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_KHR = 1;
}

pub struct VkCopyAccelerationStructureModeKHR(i32); //
impl VkCopyAccelerationStructureModeKHR {
    pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR = 0;
    pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR = 1;
}

pub struct VkBuildAccelerationStructureModeKHR(i32); //
impl VkBuildAccelerationStructureModeKHR {
    pub const VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR = 0;
    pub const VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR = 1;
}

pub struct VkAccelerationStructureTypeKHR(i32); //
impl VkAccelerationStructureTypeKHR {
    pub const VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR = 0;
    pub const VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR = 1;
    pub const VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR = 2;
}

pub struct VkGeometryTypeKHR(i32); //
impl VkGeometryTypeKHR {
    pub const VK_GEOMETRY_TYPE_TRIANGLES_KHR = 0;
    pub const VK_GEOMETRY_TYPE_AABBS_KHR = 1;
    pub const VK_GEOMETRY_TYPE_INSTANCES_KHR = 2;
}

pub struct VkAccelerationStructureMemoryRequirementsTypeNV(i32); //
impl VkAccelerationStructureMemoryRequirementsTypeNV {
    pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV = 0;
    pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV = 1;
    pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV = 2;
}

pub struct VkAccelerationStructureBuildTypeKHR(i32); //
impl VkAccelerationStructureBuildTypeKHR {
    pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR = 0;
    pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR = 1;
    pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR = 2;
}

pub struct VkRayTracingShaderGroupTypeKHR(i32); //
impl VkRayTracingShaderGroupTypeKHR {
    pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR = 0;
    pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR = 1;
    pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR = 2;
}

pub struct VkAccelerationStructureCompatibilityKHR(i32); //
impl VkAccelerationStructureCompatibilityKHR {
    pub const VK_ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR = 0;
    pub const VK_ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR = 1;
}

pub struct VkShaderGroupShaderKHR(i32); //
impl VkShaderGroupShaderKHR {
    pub const VK_SHADER_GROUP_SHADER_GENERAL_KHR = 0;
    pub const VK_SHADER_GROUP_SHADER_CLOSEST_HIT_KHR = 1;
    pub const VK_SHADER_GROUP_SHADER_ANY_HIT_KHR = 2;
    pub const VK_SHADER_GROUP_SHADER_INTERSECTION_KHR = 3;
}

pub struct VkMemoryOverallocationBehaviorAMD(i32); //
impl VkMemoryOverallocationBehaviorAMD {
    pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD = 0;
    pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD = 1;
    pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD = 2;
}

pub type VkFramebufferCreateFlagBits = ...; //
impl VkFramebufferCreateFlagBits {
}

pub type VkQueryPoolCreateFlagBits = ...; //
impl VkQueryPoolCreateFlagBits {
}

pub type VkDeviceDiagnosticsConfigFlagBitsNV = ...; //
impl VkDeviceDiagnosticsConfigFlagBitsNV {
    pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_DEBUG_INFO_BIT_NV = 1;
    pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_RESOURCE_TRACKING_BIT_NV = 2;
    pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_AUTOMATIC_CHECKPOINTS_BIT_NV = 4;
    pub const VK_DEVICE_DIAGNOSTICS_CONFIG_ENABLE_SHADER_ERROR_REPORTING_BIT_NV = 8;
}

pub type VkPipelineCreationFeedbackFlagBits = ...; //
impl VkPipelineCreationFeedbackFlagBits {
    pub const VK_PIPELINE_CREATION_FEEDBACK_VALID_BIT = 1;
    pub const VK_PIPELINE_CREATION_FEEDBACK_APPLICATION_PIPELINE_CACHE_HIT_BIT = 2;
    pub const VK_PIPELINE_CREATION_FEEDBACK_BASE_PIPELINE_ACCELERATION_BIT = 4;
}

pub struct VkFullScreenExclusiveEXT(i32); //
impl VkFullScreenExclusiveEXT {
    pub const VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT = 0;
    pub const VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT = 1;
    pub const VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT = 2;
    pub const VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT = 3;
}

pub struct VkPerformanceCounterScopeKHR(i32); //
impl VkPerformanceCounterScopeKHR {
    pub const VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR = 0;
    pub const VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR = 1;
    pub const VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR = 2;
    pub const VK_QUERY_SCOPE_COMMAND_BUFFER_KHR = ;
    pub const VK_QUERY_SCOPE_RENDER_PASS_KHR = ;
    pub const VK_QUERY_SCOPE_COMMAND_KHR = ;
}

pub type VkMemoryDecompressionMethodFlagBitsEXT = ...; //
impl VkMemoryDecompressionMethodFlagBitsEXT {
    pub const VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_EXT = 1;
    pub const VK_MEMORY_DECOMPRESSION_METHOD_GDEFLATE_1_0_BIT_NV = 0;
}

pub struct VkPerformanceCounterUnitKHR(i32); //
impl VkPerformanceCounterUnitKHR {
    pub const VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR = 0;
    pub const VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR = 1;
    pub const VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR = 2;
    pub const VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR = 3;
    pub const VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR = 4;
    pub const VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR = 5;
    pub const VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR = 6;
    pub const VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR = 7;
    pub const VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR = 8;
    pub const VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR = 9;
    pub const VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR = 10;
}

pub struct VkPerformanceCounterStorageKHR(i32); //
impl VkPerformanceCounterStorageKHR {
    pub const VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR = 0;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR = 1;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR = 2;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR = 3;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR = 4;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR = 5;
}

pub type VkPerformanceCounterDescriptionFlagBitsKHR = ...; //
impl VkPerformanceCounterDescriptionFlagBitsKHR {
    pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_BIT_KHR = 1;
    pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_PERFORMANCE_IMPACTING_KHR = 0;
    pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_BIT_KHR = 2;
    pub const VK_PERFORMANCE_COUNTER_DESCRIPTION_CONCURRENTLY_IMPACTED_KHR = 0;
}

pub type VkAcquireProfilingLockFlagBitsKHR = ...; //
impl VkAcquireProfilingLockFlagBitsKHR {
}

pub type VkShaderCorePropertiesFlagBitsAMD = ...; //
impl VkShaderCorePropertiesFlagBitsAMD {
}

pub type VkRefreshObjectFlagBitsKHR = ...; //
impl VkRefreshObjectFlagBitsKHR {
}

pub struct VkPerformanceConfigurationTypeINTEL(i32); //
impl VkPerformanceConfigurationTypeINTEL {
    pub const VK_PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL = 0;
}

pub struct VkQueryPoolSamplingModeINTEL(i32); //
impl VkQueryPoolSamplingModeINTEL {
    pub const VK_QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL = 0;
}

pub struct VkPerformanceOverrideTypeINTEL(i32); //
impl VkPerformanceOverrideTypeINTEL {
    pub const VK_PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL = 0;
    pub const VK_PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL = 1;
}

pub struct VkPerformanceParameterTypeINTEL(i32); //
impl VkPerformanceParameterTypeINTEL {
    pub const VK_PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL = 0;
    pub const VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL = 1;
}

pub struct VkPerformanceValueTypeINTEL(i32); //
impl VkPerformanceValueTypeINTEL {
    pub const VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL = 0;
    pub const VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL = 1;
    pub const VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL = 2;
    pub const VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL = 3;
    pub const VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL = 4;
}

pub struct VkShaderFloatControlsIndependence(i32); //
impl VkShaderFloatControlsIndependence {
    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY = 0;
    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL = 1;
    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE = 2;
}

pub struct VkPipelineExecutableStatisticFormatKHR(i32); //
impl VkPipelineExecutableStatisticFormatKHR {
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR = 0;
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR = 1;
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR = 2;
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR = 3;
}

pub struct VkLineRasterizationMode(i32); //
impl VkLineRasterizationMode {
    pub const VK_LINE_RASTERIZATION_MODE_DEFAULT = 0;
    pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR = 1;
    pub const VK_LINE_RASTERIZATION_MODE_BRESENHAM = 2;
    pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH = 3;
}

pub type VkShaderModuleCreateFlagBits = ...; //
impl VkShaderModuleCreateFlagBits {
}

pub type VkPipelineCompilerControlFlagBitsAMD = ...; //
impl VkPipelineCompilerControlFlagBitsAMD {
}

pub struct VkFaultLevel(i32); //
impl VkFaultLevel {
    pub const VK_FAULT_LEVEL_UNASSIGNED = 0;
    pub const VK_FAULT_LEVEL_CRITICAL = 1;
    pub const VK_FAULT_LEVEL_RECOVERABLE = 2;
    pub const VK_FAULT_LEVEL_WARNING = 3;
}

pub struct VkFaultType(i32); //
impl VkFaultType {
    pub const VK_FAULT_TYPE_INVALID = 0;
    pub const VK_FAULT_TYPE_UNASSIGNED = 1;
    pub const VK_FAULT_TYPE_IMPLEMENTATION = 2;
    pub const VK_FAULT_TYPE_SYSTEM = 3;
    pub const VK_FAULT_TYPE_PHYSICAL_DEVICE = 4;
    pub const VK_FAULT_TYPE_COMMAND_BUFFER_FULL = 5;
    pub const VK_FAULT_TYPE_INVALID_API_USAGE = 6;
}

pub struct VkFaultQueryBehavior(i32); //
impl VkFaultQueryBehavior {
    pub const VK_FAULT_QUERY_BEHAVIOR_GET_AND_CLEAR_ALL_FAULTS = 0;
}

pub type VkToolPurposeFlagBits = ...; //
impl VkToolPurposeFlagBits {
    pub const VK_TOOL_PURPOSE_VALIDATION_BIT = 1;
    pub const VK_TOOL_PURPOSE_PROFILING_BIT = 2;
    pub const VK_TOOL_PURPOSE_TRACING_BIT = 4;
    pub const VK_TOOL_PURPOSE_ADDITIONAL_FEATURES_BIT = 8;
    pub const VK_TOOL_PURPOSE_MODIFYING_FEATURES_BIT = 16;
}

pub struct VkPipelineMatchControl(i32); //
impl VkPipelineMatchControl {
    pub const VK_PIPELINE_MATCH_CONTROL_APPLICATION_UUID_EXACT_MATCH = 0;
}

pub struct VkFragmentShadingRateCombinerOpKHR(i32); //
impl VkFragmentShadingRateCombinerOpKHR {
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR = 0;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR = 1;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR = 2;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR = 3;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR = 4;
}

pub struct VkFragmentShadingRateNV(i32); //
impl VkFragmentShadingRateNV {
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV = 0;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV = 1;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV = 4;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV = 5;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV = 6;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV = 9;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV = 10;
    pub const VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV = 11;
    pub const VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV = 12;
    pub const VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV = 13;
    pub const VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV = 14;
    pub const VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV = 15;
}

pub struct VkFragmentShadingRateTypeNV(i32); //
impl VkFragmentShadingRateTypeNV {
    pub const VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV = 0;
    pub const VK_FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV = 1;
}

pub struct VkSubpassMergeStatusEXT(i32); //
impl VkSubpassMergeStatusEXT {
    pub const VK_SUBPASS_MERGE_STATUS_MERGED_EXT = 0;
    pub const VK_SUBPASS_MERGE_STATUS_DISALLOWED_EXT = 1;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SIDE_EFFECTS_EXT = 2;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SAMPLES_MISMATCH_EXT = 3;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_VIEWS_MISMATCH_EXT = 4;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_ALIASING_EXT = 5;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_DEPENDENCIES_EXT = 6;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT = 7;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT = 8;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_INSUFFICIENT_STORAGE_EXT = 9;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_DEPTH_STENCIL_COUNT_EXT = 10;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT = 11;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SINGLE_SUBPASS_EXT = 12;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_UNSPECIFIED_EXT = 13;
}

pub type VkAccessFlagBits2 = ...; //
impl VkAccessFlagBits2 {
    pub const VK_ACCESS_2_NONE = 0;
    pub const VK_ACCESS_2_INDIRECT_COMMAND_READ_BIT = 1;
    pub const VK_ACCESS_2_INDEX_READ_BIT = 2;
    pub const VK_ACCESS_2_VERTEX_ATTRIBUTE_READ_BIT = 4;
    pub const VK_ACCESS_2_UNIFORM_READ_BIT = 8;
    pub const VK_ACCESS_2_INPUT_ATTACHMENT_READ_BIT = 16;
    pub const VK_ACCESS_2_SHADER_READ_BIT = 32;
    pub const VK_ACCESS_2_SHADER_WRITE_BIT = 64;
    pub const VK_ACCESS_2_COLOR_ATTACHMENT_READ_BIT = 128;
    pub const VK_ACCESS_2_COLOR_ATTACHMENT_WRITE_BIT = 256;
    pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_READ_BIT = 512;
    pub const VK_ACCESS_2_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 1024;
    pub const VK_ACCESS_2_TRANSFER_READ_BIT = 2048;
    pub const VK_ACCESS_2_TRANSFER_WRITE_BIT = 4096;
    pub const VK_ACCESS_2_HOST_READ_BIT = 8192;
    pub const VK_ACCESS_2_HOST_WRITE_BIT = 16384;
    pub const VK_ACCESS_2_MEMORY_READ_BIT = 32768;
    pub const VK_ACCESS_2_MEMORY_WRITE_BIT = 65536;
    pub const VK_ACCESS_2_SHADER_SAMPLED_READ_BIT = 4294967296;
    pub const VK_ACCESS_2_SHADER_STORAGE_READ_BIT = 8589934592;
    pub const VK_ACCESS_2_SHADER_STORAGE_WRITE_BIT = 17179869184;
}

pub type VkPipelineStageFlagBits2 = ...; //
impl VkPipelineStageFlagBits2 {
    pub const VK_PIPELINE_STAGE_2_NONE = 0;
    pub const VK_PIPELINE_STAGE_2_TOP_OF_PIPE_BIT = 1;
    pub const VK_PIPELINE_STAGE_2_DRAW_INDIRECT_BIT = 2;
    pub const VK_PIPELINE_STAGE_2_VERTEX_INPUT_BIT = 4;
    pub const VK_PIPELINE_STAGE_2_VERTEX_SHADER_BIT = 8;
    pub const VK_PIPELINE_STAGE_2_TESSELLATION_CONTROL_SHADER_BIT = 16;
    pub const VK_PIPELINE_STAGE_2_TESSELLATION_EVALUATION_SHADER_BIT = 32;
    pub const VK_PIPELINE_STAGE_2_GEOMETRY_SHADER_BIT = 64;
    pub const VK_PIPELINE_STAGE_2_FRAGMENT_SHADER_BIT = 128;
    pub const VK_PIPELINE_STAGE_2_EARLY_FRAGMENT_TESTS_BIT = 256;
    pub const VK_PIPELINE_STAGE_2_LATE_FRAGMENT_TESTS_BIT = 512;
    pub const VK_PIPELINE_STAGE_2_COLOR_ATTACHMENT_OUTPUT_BIT = 1024;
    pub const VK_PIPELINE_STAGE_2_COMPUTE_SHADER_BIT = 2048;
    pub const VK_PIPELINE_STAGE_2_ALL_TRANSFER_BIT = 4096;
    pub const VK_PIPELINE_STAGE_2_TRANSFER_BIT = 0;
    pub const VK_PIPELINE_STAGE_2_BOTTOM_OF_PIPE_BIT = 8192;
    pub const VK_PIPELINE_STAGE_2_HOST_BIT = 16384;
    pub const VK_PIPELINE_STAGE_2_ALL_GRAPHICS_BIT = 32768;
    pub const VK_PIPELINE_STAGE_2_ALL_COMMANDS_BIT = 65536;
    pub const VK_PIPELINE_STAGE_2_COPY_BIT = 4294967296;
    pub const VK_PIPELINE_STAGE_2_RESOLVE_BIT = 8589934592;
    pub const VK_PIPELINE_STAGE_2_BLIT_BIT = 17179869184;
    pub const VK_PIPELINE_STAGE_2_CLEAR_BIT = 34359738368;
    pub const VK_PIPELINE_STAGE_2_INDEX_INPUT_BIT = 68719476736;
    pub const VK_PIPELINE_STAGE_2_VERTEX_ATTRIBUTE_INPUT_BIT = 137438953472;
    pub const VK_PIPELINE_STAGE_2_PRE_RASTERIZATION_SHADERS_BIT = 274877906944;
}

pub type VkSubmitFlagBits = ...; //
impl VkSubmitFlagBits {
    pub const VK_SUBMIT_PROTECTED_BIT = 1;
}

pub type VkEventCreateFlagBits = ...; //
impl VkEventCreateFlagBits {
}

pub type VkPipelineLayoutCreateFlagBits = ...; //
impl VkPipelineLayoutCreateFlagBits {
}

pub struct VkSciSyncClientTypeNV(i32); //
impl VkSciSyncClientTypeNV {
    pub const VK_SCI_SYNC_CLIENT_TYPE_SIGNALER_NV = 0;
    pub const VK_SCI_SYNC_CLIENT_TYPE_WAITER_NV = 1;
    pub const VK_SCI_SYNC_CLIENT_TYPE_SIGNALER_WAITER_NV = 2;
}

pub struct VkSciSyncPrimitiveTypeNV(i32); //
impl VkSciSyncPrimitiveTypeNV {
    pub const VK_SCI_SYNC_PRIMITIVE_TYPE_FENCE_NV = 0;
    pub const VK_SCI_SYNC_PRIMITIVE_TYPE_SEMAPHORE_NV = 1;
}

pub struct VkProvokingVertexModeEXT(i32); //
impl VkProvokingVertexModeEXT {
    pub const VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT = 0;
    pub const VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT = 1;
}

pub struct VkPipelineCacheValidationVersion(i32); //
impl VkPipelineCacheValidationVersion {
    pub const VK_PIPELINE_CACHE_VALIDATION_VERSION_SAFETY_CRITICAL_ONE = 1;
}

pub struct VkAccelerationStructureMotionInstanceTypeNV(i32); //
impl VkAccelerationStructureMotionInstanceTypeNV {
    pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV = 0;
    pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV = 1;
    pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV = 2;
}

pub type VkPipelineColorBlendStateCreateFlagBits = ...; //
impl VkPipelineColorBlendStateCreateFlagBits {
}

pub type VkPipelineDepthStencilStateCreateFlagBits = ...; //
impl VkPipelineDepthStencilStateCreateFlagBits {
}

pub type VkGraphicsPipelineLibraryFlagBitsEXT = ...; //
impl VkGraphicsPipelineLibraryFlagBitsEXT {
    pub const VK_GRAPHICS_PIPELINE_LIBRARY_VERTEX_INPUT_INTERFACE_BIT_EXT = 1;
    pub const VK_GRAPHICS_PIPELINE_LIBRARY_PRE_RASTERIZATION_SHADERS_BIT_EXT = 2;
    pub const VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_SHADER_BIT_EXT = 4;
    pub const VK_GRAPHICS_PIPELINE_LIBRARY_FRAGMENT_OUTPUT_INTERFACE_BIT_EXT = 8;
}

pub type VkRenderingAttachmentFlagBitsKHR = ...; //
impl VkRenderingAttachmentFlagBitsKHR {
}

pub type VkResolveImageFlagBitsKHR = ...; //
impl VkResolveImageFlagBitsKHR {
}

pub type VkDeviceAddressBindingFlagBitsEXT = ...; //
impl VkDeviceAddressBindingFlagBitsEXT {
    pub const VK_DEVICE_ADDRESS_BINDING_INTERNAL_OBJECT_BIT_EXT = 1;
}

pub struct VkDeviceAddressBindingTypeEXT(i32); //
impl VkDeviceAddressBindingTypeEXT {
    pub const VK_DEVICE_ADDRESS_BINDING_TYPE_BIND_EXT = 0;
    pub const VK_DEVICE_ADDRESS_BINDING_TYPE_UNBIND_EXT = 1;
}

pub type VkFrameBoundaryFlagBitsEXT = ...; //
impl VkFrameBoundaryFlagBitsEXT {
    pub const VK_FRAME_BOUNDARY_FRAME_END_BIT_EXT = 1;
}

pub type VkPresentScalingFlagBitsKHR = ...; //
impl VkPresentScalingFlagBitsKHR {
    pub const VK_PRESENT_SCALING_ONE_TO_ONE_BIT_KHR = 1;
    pub const VK_PRESENT_SCALING_ONE_TO_ONE_BIT_EXT = 0;
    pub const VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_KHR = 2;
    pub const VK_PRESENT_SCALING_ASPECT_RATIO_STRETCH_BIT_EXT = 0;
    pub const VK_PRESENT_SCALING_STRETCH_BIT_KHR = 4;
    pub const VK_PRESENT_SCALING_STRETCH_BIT_EXT = 0;
}

pub type VkPresentGravityFlagBitsKHR = ...; //
impl VkPresentGravityFlagBitsKHR {
    pub const VK_PRESENT_GRAVITY_MIN_BIT_KHR = 1;
    pub const VK_PRESENT_GRAVITY_MIN_BIT_EXT = 0;
    pub const VK_PRESENT_GRAVITY_MAX_BIT_KHR = 2;
    pub const VK_PRESENT_GRAVITY_MAX_BIT_EXT = 0;
    pub const VK_PRESENT_GRAVITY_CENTERED_BIT_KHR = 4;
    pub const VK_PRESENT_GRAVITY_CENTERED_BIT_EXT = 0;
}

pub type VkPhysicalDeviceSchedulingControlsFlagBitsARM = ...; //
impl VkPhysicalDeviceSchedulingControlsFlagBitsARM {
    pub const VK_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_SHADER_CORE_COUNT_ARM = 1;
    pub const VK_PHYSICAL_DEVICE_SCHEDULING_CONTROLS_DISPATCH_PARAMETERS_ARM = 2;
}

pub type VkPresentStageFlagBitsEXT = ...; //
impl VkPresentStageFlagBitsEXT {
    pub const VK_PRESENT_STAGE_QUEUE_OPERATIONS_END_BIT_EXT = 1;
    pub const VK_PRESENT_STAGE_REQUEST_DEQUEUED_BIT_EXT = 2;
    pub const VK_PRESENT_STAGE_IMAGE_FIRST_PIXEL_OUT_BIT_EXT = 4;
    pub const VK_PRESENT_STAGE_IMAGE_FIRST_PIXEL_VISIBLE_BIT_EXT = 8;
}

pub type VkPastPresentationTimingFlagBitsEXT = ...; //
impl VkPastPresentationTimingFlagBitsEXT {
    pub const VK_PAST_PRESENTATION_TIMING_ALLOW_PARTIAL_RESULTS_BIT_EXT = 1;
    pub const VK_PAST_PRESENTATION_TIMING_ALLOW_OUT_OF_ORDER_RESULTS_BIT_EXT = 2;
}

pub type VkPresentTimingInfoFlagBitsEXT = ...; //
impl VkPresentTimingInfoFlagBitsEXT {
    pub const VK_PRESENT_TIMING_INFO_PRESENT_AT_RELATIVE_TIME_BIT_EXT = 1;
    pub const VK_PRESENT_TIMING_INFO_PRESENT_AT_NEAREST_REFRESH_CYCLE_BIT_EXT = 2;
}

pub type VkVideoCodecOperationFlagBitsKHR = ...; //
impl VkVideoCodecOperationFlagBitsKHR {
    pub const VK_VIDEO_CODEC_OPERATION_NONE_KHR = 0;
}

pub type VkVideoChromaSubsamplingFlagBitsKHR = ...; //Vulkan video chroma subsampling definitions
impl VkVideoChromaSubsamplingFlagBitsKHR {
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_INVALID_KHR = 0;
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_MONOCHROME_BIT_KHR = 1;
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_420_BIT_KHR = 2;
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_422_BIT_KHR = 4;
    pub const VK_VIDEO_CHROMA_SUBSAMPLING_444_BIT_KHR = 8;
}

pub type VkVideoComponentBitDepthFlagBitsKHR = ...; //Vulkan video component bit depth definitions
impl VkVideoComponentBitDepthFlagBitsKHR {
    pub const VK_VIDEO_COMPONENT_BIT_DEPTH_INVALID_KHR = 0;
    pub const VK_VIDEO_COMPONENT_BIT_DEPTH_8_BIT_KHR = 1;
    pub const VK_VIDEO_COMPONENT_BIT_DEPTH_10_BIT_KHR = 4;
    pub const VK_VIDEO_COMPONENT_BIT_DEPTH_12_BIT_KHR = 16;
}

pub type VkVideoCapabilityFlagBitsKHR = ...; //
impl VkVideoCapabilityFlagBitsKHR {
    pub const VK_VIDEO_CAPABILITY_PROTECTED_CONTENT_BIT_KHR = 1;
    pub const VK_VIDEO_CAPABILITY_SEPARATE_REFERENCE_IMAGES_BIT_KHR = 2;
}

pub type VkVideoSessionCreateFlagBitsKHR = ...; //
impl VkVideoSessionCreateFlagBitsKHR {
    pub const VK_VIDEO_SESSION_CREATE_PROTECTED_CONTENT_BIT_KHR = 1;
}

pub type VkVideoSessionParametersCreateFlagBitsKHR = ...; //
impl VkVideoSessionParametersCreateFlagBitsKHR {
}

pub type VkVideoDecodeH264PictureLayoutFlagBitsKHR = ...; //
impl VkVideoDecodeH264PictureLayoutFlagBitsKHR {
    pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_PROGRESSIVE_KHR = 0;
    pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_INTERLEAVED_LINES_BIT_KHR = 1;
    pub const VK_VIDEO_DECODE_H264_PICTURE_LAYOUT_INTERLACED_SEPARATE_PLANES_BIT_KHR = 2;
}

pub type VkVideoCodingControlFlagBitsKHR = ...; //
impl VkVideoCodingControlFlagBitsKHR {
    pub const VK_VIDEO_CODING_CONTROL_RESET_BIT_KHR = 1;
}

pub struct VkQueryResultStatusKHR(i32); //
impl VkQueryResultStatusKHR {
    pub const VK_QUERY_RESULT_STATUS_ERROR_KHR = -1;
    pub const VK_QUERY_RESULT_STATUS_NOT_READY_KHR = 0;
    pub const VK_QUERY_RESULT_STATUS_COMPLETE_KHR = 1;
}

pub type VkVideoDecodeUsageFlagBitsKHR = ...; //
impl VkVideoDecodeUsageFlagBitsKHR {
    pub const VK_VIDEO_DECODE_USAGE_DEFAULT_KHR = 0;
    pub const VK_VIDEO_DECODE_USAGE_TRANSCODING_BIT_KHR = 1;
    pub const VK_VIDEO_DECODE_USAGE_OFFLINE_BIT_KHR = 2;
    pub const VK_VIDEO_DECODE_USAGE_STREAMING_BIT_KHR = 4;
}

pub type VkVideoDecodeCapabilityFlagBitsKHR = ...; //
impl VkVideoDecodeCapabilityFlagBitsKHR {
    pub const VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_COINCIDE_BIT_KHR = 1;
    pub const VK_VIDEO_DECODE_CAPABILITY_DPB_AND_OUTPUT_DISTINCT_BIT_KHR = 2;
}

pub type VkVideoEncodeFlagBitsKHR = ...; //
impl VkVideoEncodeFlagBitsKHR {
}

pub type VkCooperativeMatrixFlagBitsEXT = ...; //
impl VkCooperativeMatrixFlagBitsEXT {
}

pub type VkVideoEncodeUsageFlagBitsKHR = ...; //
impl VkVideoEncodeUsageFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_USAGE_DEFAULT_KHR = 0;
    pub const VK_VIDEO_ENCODE_USAGE_TRANSCODING_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_USAGE_STREAMING_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_USAGE_RECORDING_BIT_KHR = 4;
    pub const VK_VIDEO_ENCODE_USAGE_CONFERENCING_BIT_KHR = 8;
}

pub type VkVideoEncodeContentFlagBitsKHR = ...; //
impl VkVideoEncodeContentFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_CONTENT_DEFAULT_KHR = 0;
    pub const VK_VIDEO_ENCODE_CONTENT_CAMERA_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_CONTENT_DESKTOP_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_CONTENT_RENDERED_BIT_KHR = 4;
}

pub struct VkVideoEncodeTuningModeKHR(i32); //
impl VkVideoEncodeTuningModeKHR {
    pub const VK_VIDEO_ENCODE_TUNING_MODE_DEFAULT_KHR = 0;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_HIGH_QUALITY_KHR = 1;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_LOW_LATENCY_KHR = 2;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_ULTRA_LOW_LATENCY_KHR = 3;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_LOSSLESS_KHR = 4;
}

pub type VkVideoEncodeCapabilityFlagBitsKHR = ...; //
impl VkVideoEncodeCapabilityFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_CAPABILITY_PRECEDING_EXTERNALLY_ENCODED_BYTES_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_CAPABILITY_INSUFFICIENT_BITSTREAM_BUFFER_RANGE_DETECTION_BIT_KHR = 2;
}

pub type VkVideoEncodeFeedbackFlagBitsKHR = ...; //
impl VkVideoEncodeFeedbackFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_BUFFER_OFFSET_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_BYTES_WRITTEN_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_FEEDBACK_BITSTREAM_HAS_OVERRIDES_BIT_KHR = 4;
}

pub type VkVideoEncodePerPartitionFeedbackFlagBitsKHR = ...; //
impl VkVideoEncodePerPartitionFeedbackFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_STATUS_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_BITSTREAM_BUFFER_OFFSET_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_PER_PARTITION_FEEDBACK_BITSTREAM_BYTES_WRITTEN_BIT_KHR = 4;
}

pub type VkVideoEncodeRateControlModeFlagBitsKHR = ...; //
impl VkVideoEncodeRateControlModeFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_DEFAULT_KHR = 0;
    pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_DISABLED_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_CBR_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_RATE_CONTROL_MODE_VBR_BIT_KHR = 4;
}

pub type VkVideoEncodeIntraRefreshModeFlagBitsKHR = ...; //
impl VkVideoEncodeIntraRefreshModeFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_NONE_KHR = 0;
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_PER_PICTURE_PARTITION_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_BASED_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_ROW_BASED_BIT_KHR = 4;
    pub const VK_VIDEO_ENCODE_INTRA_REFRESH_MODE_BLOCK_COLUMN_BASED_BIT_KHR = 8;
}

pub type VkVideoEncodeH264CapabilityFlagBitsKHR = ...; //
impl VkVideoEncodeH264CapabilityFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_HRD_COMPLIANCE_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_ROW_UNALIGNED_SLICE_BIT_KHR = 4;
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_DIFFERENT_SLICE_TYPE_BIT_KHR = 8;
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR = 16;
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR = 32;
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR = 64;
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_PER_SLICE_CONSTANT_QP_BIT_KHR = 128;
    pub const VK_VIDEO_ENCODE_H264_CAPABILITY_GENERATE_PREFIX_NALU_BIT_KHR = 256;
}

pub type VkVideoEncodeH264StdFlagBitsKHR = ...; //
impl VkVideoEncodeH264StdFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H264_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_H264_STD_QPPRIME_Y_ZERO_TRANSFORM_BYPASS_FLAG_SET_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_H264_STD_SCALING_MATRIX_PRESENT_FLAG_SET_BIT_KHR = 4;
    pub const VK_VIDEO_ENCODE_H264_STD_CHROMA_QP_INDEX_OFFSET_BIT_KHR = 8;
    pub const VK_VIDEO_ENCODE_H264_STD_SECOND_CHROMA_QP_INDEX_OFFSET_BIT_KHR = 16;
    pub const VK_VIDEO_ENCODE_H264_STD_PIC_INIT_QP_MINUS26_BIT_KHR = 32;
    pub const VK_VIDEO_ENCODE_H264_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR = 64;
    pub const VK_VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_EXPLICIT_BIT_KHR = 128;
    pub const VK_VIDEO_ENCODE_H264_STD_WEIGHTED_BIPRED_IDC_IMPLICIT_BIT_KHR = 256;
    pub const VK_VIDEO_ENCODE_H264_STD_TRANSFORM_8X8_MODE_FLAG_SET_BIT_KHR = 512;
    pub const VK_VIDEO_ENCODE_H264_STD_DIRECT_SPATIAL_MV_PRED_FLAG_UNSET_BIT_KHR = 1024;
    pub const VK_VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_UNSET_BIT_KHR = 2048;
    pub const VK_VIDEO_ENCODE_H264_STD_ENTROPY_CODING_MODE_FLAG_SET_BIT_KHR = 4096;
    pub const VK_VIDEO_ENCODE_H264_STD_DIRECT_8X8_INFERENCE_FLAG_UNSET_BIT_KHR = 8192;
    pub const VK_VIDEO_ENCODE_H264_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR = 16384;
    pub const VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_DISABLED_BIT_KHR = 32768;
    pub const VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_ENABLED_BIT_KHR = 65536;
    pub const VK_VIDEO_ENCODE_H264_STD_DEBLOCKING_FILTER_PARTIAL_BIT_KHR = 131072;
    pub const VK_VIDEO_ENCODE_H264_STD_SLICE_QP_DELTA_BIT_KHR = 524288;
    pub const VK_VIDEO_ENCODE_H264_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR = 1048576;
}

pub type VkVideoEncodeH264RateControlFlagBitsKHR = ...; //
impl VkVideoEncodeH264RateControlFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_REGULAR_GOP_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR = 4;
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR = 8;
    pub const VK_VIDEO_ENCODE_H264_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR = 16;
}

pub type VkHostImageCopyFlagBits = ...; //
impl VkHostImageCopyFlagBits {
    pub const VK_HOST_IMAGE_COPY_MEMCPY_BIT = 1;
    pub const VK_HOST_IMAGE_COPY_MEMCPY = 0;
}

pub struct VkPartitionedAccelerationStructureOpTypeNV(i32); //
impl VkPartitionedAccelerationStructureOpTypeNV {
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_WRITE_INSTANCE_NV = 0;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_UPDATE_INSTANCE_NV = 1;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_WRITE_PARTITION_TRANSLATION_NV = 2;
}

pub type VkPartitionedAccelerationStructureInstanceFlagBitsNV = ...; //
impl VkPartitionedAccelerationStructureInstanceFlagBitsNV {
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FACING_CULL_DISABLE_BIT_NV = 1;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_TRIANGLE_FLIP_FACING_BIT_NV = 2;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_OPAQUE_BIT_NV = 4;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_FORCE_NO_OPAQUE_BIT_NV = 8;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_INSTANCE_FLAG_ENABLE_EXPLICIT_BOUNDING_BOX_NV = 16;
}

pub type VkImageFormatConstraintsFlagBitsFUCHSIA = ...; //
impl VkImageFormatConstraintsFlagBitsFUCHSIA {
}

pub type VkImageConstraintsInfoFlagBitsFUCHSIA = ...; //
impl VkImageConstraintsInfoFlagBitsFUCHSIA {
    pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_RARELY_FUCHSIA = 1;
    pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_READ_OFTEN_FUCHSIA = 2;
    pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_RARELY_FUCHSIA = 4;
    pub const VK_IMAGE_CONSTRAINTS_INFO_CPU_WRITE_OFTEN_FUCHSIA = 8;
    pub const VK_IMAGE_CONSTRAINTS_INFO_PROTECTED_OPTIONAL_FUCHSIA = 16;
}

pub type VkFormatFeatureFlagBits2 = ...; //
impl VkFormatFeatureFlagBits2 {
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_BIT = 1;
    pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_BIT = 2;
    pub const VK_FORMAT_FEATURE_2_STORAGE_IMAGE_ATOMIC_BIT = 4;
    pub const VK_FORMAT_FEATURE_2_UNIFORM_TEXEL_BUFFER_BIT = 8;
    pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_BIT = 16;
    pub const VK_FORMAT_FEATURE_2_STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 32;
    pub const VK_FORMAT_FEATURE_2_VERTEX_BUFFER_BIT = 64;
    pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BIT = 128;
    pub const VK_FORMAT_FEATURE_2_COLOR_ATTACHMENT_BLEND_BIT = 256;
    pub const VK_FORMAT_FEATURE_2_DEPTH_STENCIL_ATTACHMENT_BIT = 512;
    pub const VK_FORMAT_FEATURE_2_BLIT_SRC_BIT = 1024;
    pub const VK_FORMAT_FEATURE_2_BLIT_DST_BIT = 2048;
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_LINEAR_BIT = 4096;
    pub const VK_FORMAT_FEATURE_2_TRANSFER_SRC_BIT = 16384;
    pub const VK_FORMAT_FEATURE_2_TRANSFER_DST_BIT = 32768;
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_FILTER_MINMAX_BIT = 65536;
    pub const VK_FORMAT_FEATURE_2_MIDPOINT_CHROMA_SAMPLES_BIT = 131072;
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_LINEAR_FILTER_BIT = 262144;
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_SEPARATE_RECONSTRUCTION_FILTER_BIT = 524288;
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_BIT = 1048576;
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_YCBCR_CONVERSION_CHROMA_RECONSTRUCTION_EXPLICIT_FORCEABLE_BIT = 2097152;
    pub const VK_FORMAT_FEATURE_2_DISJOINT_BIT = 4194304;
    pub const VK_FORMAT_FEATURE_2_COSITED_CHROMA_SAMPLES_BIT = 8388608;
    pub const VK_FORMAT_FEATURE_2_STORAGE_READ_WITHOUT_FORMAT_BIT = 2147483648;
    pub const VK_FORMAT_FEATURE_2_STORAGE_WRITE_WITHOUT_FORMAT_BIT = 4294967296;
    pub const VK_FORMAT_FEATURE_2_SAMPLED_IMAGE_DEPTH_COMPARISON_BIT = 8589934592;
}

pub type VkFormatFeatureFlagBits4KHR = ...; //
impl VkFormatFeatureFlagBits4KHR {
}

pub type VkRenderingFlagBits = ...; //
impl VkRenderingFlagBits {
    pub const VK_RENDERING_CONTENTS_SECONDARY_COMMAND_BUFFERS_BIT = 1;
    pub const VK_RENDERING_SUSPENDING_BIT = 2;
    pub const VK_RENDERING_RESUMING_BIT = 4;
}

pub type VkVideoEncodeH265CapabilityFlagBitsKHR = ...; //
impl VkVideoEncodeH265CapabilityFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_HRD_COMPLIANCE_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_PREDICTION_WEIGHT_TABLE_GENERATED_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_ROW_UNALIGNED_SLICE_SEGMENT_BIT_KHR = 4;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_DIFFERENT_SLICE_SEGMENT_TYPE_BIT_KHR = 8;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L0_LIST_BIT_KHR = 16;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_B_FRAME_IN_L1_LIST_BIT_KHR = 32;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_PER_PICTURE_TYPE_MIN_MAX_QP_BIT_KHR = 64;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_PER_SLICE_SEGMENT_CONSTANT_QP_BIT_KHR = 128;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_TILES_PER_SLICE_SEGMENT_BIT_KHR = 256;
    pub const VK_VIDEO_ENCODE_H265_CAPABILITY_MULTIPLE_SLICE_SEGMENTS_PER_TILE_BIT_KHR = 512;
}

pub type VkVideoEncodeH265StdFlagBitsKHR = ...; //
impl VkVideoEncodeH265StdFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H265_STD_SEPARATE_COLOR_PLANE_FLAG_SET_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_H265_STD_SAMPLE_ADAPTIVE_OFFSET_ENABLED_FLAG_SET_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_H265_STD_SCALING_LIST_DATA_PRESENT_FLAG_SET_BIT_KHR = 4;
    pub const VK_VIDEO_ENCODE_H265_STD_PCM_ENABLED_FLAG_SET_BIT_KHR = 8;
    pub const VK_VIDEO_ENCODE_H265_STD_SPS_TEMPORAL_MVP_ENABLED_FLAG_SET_BIT_KHR = 16;
    pub const VK_VIDEO_ENCODE_H265_STD_INIT_QP_MINUS26_BIT_KHR = 32;
    pub const VK_VIDEO_ENCODE_H265_STD_WEIGHTED_PRED_FLAG_SET_BIT_KHR = 64;
    pub const VK_VIDEO_ENCODE_H265_STD_WEIGHTED_BIPRED_FLAG_SET_BIT_KHR = 128;
    pub const VK_VIDEO_ENCODE_H265_STD_LOG2_PARALLEL_MERGE_LEVEL_MINUS2_BIT_KHR = 256;
    pub const VK_VIDEO_ENCODE_H265_STD_SIGN_DATA_HIDING_ENABLED_FLAG_SET_BIT_KHR = 512;
    pub const VK_VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_SET_BIT_KHR = 1024;
    pub const VK_VIDEO_ENCODE_H265_STD_TRANSFORM_SKIP_ENABLED_FLAG_UNSET_BIT_KHR = 2048;
    pub const VK_VIDEO_ENCODE_H265_STD_PPS_SLICE_CHROMA_QP_OFFSETS_PRESENT_FLAG_SET_BIT_KHR = 4096;
    pub const VK_VIDEO_ENCODE_H265_STD_TRANSQUANT_BYPASS_ENABLED_FLAG_SET_BIT_KHR = 8192;
    pub const VK_VIDEO_ENCODE_H265_STD_CONSTRAINED_INTRA_PRED_FLAG_SET_BIT_KHR = 16384;
    pub const VK_VIDEO_ENCODE_H265_STD_ENTROPY_CODING_SYNC_ENABLED_FLAG_SET_BIT_KHR = 32768;
    pub const VK_VIDEO_ENCODE_H265_STD_DEBLOCKING_FILTER_OVERRIDE_ENABLED_FLAG_SET_BIT_KHR = 65536;
    pub const VK_VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENTS_ENABLED_FLAG_SET_BIT_KHR = 131072;
    pub const VK_VIDEO_ENCODE_H265_STD_DEPENDENT_SLICE_SEGMENT_FLAG_SET_BIT_KHR = 262144;
    pub const VK_VIDEO_ENCODE_H265_STD_SLICE_QP_DELTA_BIT_KHR = 524288;
    pub const VK_VIDEO_ENCODE_H265_STD_DIFFERENT_SLICE_QP_DELTA_BIT_KHR = 1048576;
}

pub type VkVideoEncodeH265RateControlFlagBitsKHR = ...; //
impl VkVideoEncodeH265RateControlFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_ATTEMPT_HRD_COMPLIANCE_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_REGULAR_GOP_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR = 4;
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR = 8;
    pub const VK_VIDEO_ENCODE_H265_RATE_CONTROL_TEMPORAL_SUB_LAYER_PATTERN_DYADIC_BIT_KHR = 16;
}

pub type VkVideoEncodeH265CtbSizeFlagBitsKHR = ...; //
impl VkVideoEncodeH265CtbSizeFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_16_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_32_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_H265_CTB_SIZE_64_BIT_KHR = 4;
}

pub type VkVideoEncodeH265TransformBlockSizeFlagBitsKHR = ...; //
impl VkVideoEncodeH265TransformBlockSizeFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_4_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_8_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_16_BIT_KHR = 4;
    pub const VK_VIDEO_ENCODE_H265_TRANSFORM_BLOCK_SIZE_32_BIT_KHR = 8;
}

pub type VkVideoEncodeAV1CapabilityFlagBitsKHR = ...; //
impl VkVideoEncodeAV1CapabilityFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_PER_RATE_CONTROL_GROUP_MIN_MAX_Q_INDEX_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_GENERATE_OBU_EXTENSION_HEADER_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_PRIMARY_REFERENCE_CDF_ONLY_BIT_KHR = 4;
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_FRAME_SIZE_OVERRIDE_BIT_KHR = 8;
    pub const VK_VIDEO_ENCODE_AV1_CAPABILITY_MOTION_VECTOR_SCALING_BIT_KHR = 16;
}

pub type VkVideoEncodeAV1StdFlagBitsKHR = ...; //
impl VkVideoEncodeAV1StdFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_AV1_STD_UNIFORM_TILE_SPACING_FLAG_SET_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_AV1_STD_SKIP_MODE_PRESENT_UNSET_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_AV1_STD_PRIMARY_REF_FRAME_BIT_KHR = 4;
    pub const VK_VIDEO_ENCODE_AV1_STD_DELTA_Q_BIT_KHR = 8;
}

pub type VkVideoEncodeAV1RateControlFlagBitsKHR = ...; //
impl VkVideoEncodeAV1RateControlFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REGULAR_GOP_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_TEMPORAL_LAYER_PATTERN_DYADIC_BIT_KHR = 2;
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_FLAT_BIT_KHR = 4;
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_REFERENCE_PATTERN_DYADIC_BIT_KHR = 8;
}

pub type VkVideoEncodeAV1SuperblockSizeFlagBitsKHR = ...; //
impl VkVideoEncodeAV1SuperblockSizeFlagBitsKHR {
    pub const VK_VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_64_BIT_KHR = 1;
    pub const VK_VIDEO_ENCODE_AV1_SUPERBLOCK_SIZE_128_BIT_KHR = 2;
}

pub struct VkVideoEncodeAV1PredictionModeKHR(i32); //
impl VkVideoEncodeAV1PredictionModeKHR {
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_INTRA_ONLY_KHR = 0;
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_SINGLE_REFERENCE_KHR = 1;
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_UNIDIRECTIONAL_COMPOUND_KHR = 2;
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_BIDIRECTIONAL_COMPOUND_KHR = 3;
}

pub struct VkVideoEncodeAV1RateControlGroupKHR(i32); //
impl VkVideoEncodeAV1RateControlGroupKHR {
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_INTRA_KHR = 0;
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_PREDICTIVE_KHR = 1;
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_BIPREDICTIVE_KHR = 2;
}

pub type VkExportMetalObjectTypeFlagBitsEXT = ...; //
impl VkExportMetalObjectTypeFlagBitsEXT {
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_DEVICE_BIT_EXT = 1;
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_COMMAND_QUEUE_BIT_EXT = 2;
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_BUFFER_BIT_EXT = 4;
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_TEXTURE_BIT_EXT = 8;
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_IOSURFACE_BIT_EXT = 16;
    pub const VK_EXPORT_METAL_OBJECT_TYPE_METAL_SHARED_EVENT_BIT_EXT = 32;
}

pub type VkInstanceCreateFlagBits = ...; //
impl VkInstanceCreateFlagBits {
}

pub type VkImageCompressionFlagBitsEXT = ...; //
impl VkImageCompressionFlagBitsEXT {
    pub const VK_IMAGE_COMPRESSION_DEFAULT_EXT = 0;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_DEFAULT_EXT = 1;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_EXPLICIT_EXT = 2;
    pub const VK_IMAGE_COMPRESSION_DISABLED_EXT = 4;
}

pub type VkImageCompressionFixedRateFlagBitsEXT = ...; //
impl VkImageCompressionFixedRateFlagBitsEXT {
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_NONE_EXT = 0;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_1BPC_BIT_EXT = 1;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_2BPC_BIT_EXT = 2;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_3BPC_BIT_EXT = 4;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_4BPC_BIT_EXT = 8;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_5BPC_BIT_EXT = 16;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_6BPC_BIT_EXT = 32;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_7BPC_BIT_EXT = 64;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_8BPC_BIT_EXT = 128;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_9BPC_BIT_EXT = 256;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_10BPC_BIT_EXT = 512;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_11BPC_BIT_EXT = 1024;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_12BPC_BIT_EXT = 2048;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_13BPC_BIT_EXT = 4096;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_14BPC_BIT_EXT = 8192;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_15BPC_BIT_EXT = 16384;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_16BPC_BIT_EXT = 32768;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_17BPC_BIT_EXT = 65536;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_18BPC_BIT_EXT = 131072;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_19BPC_BIT_EXT = 262144;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_20BPC_BIT_EXT = 524288;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_21BPC_BIT_EXT = 1048576;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_22BPC_BIT_EXT = 2097152;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_23BPC_BIT_EXT = 4194304;
    pub const VK_IMAGE_COMPRESSION_FIXED_RATE_24BPC_BIT_EXT = 8388608;
}

pub struct VkPipelineRobustnessBufferBehavior(i32); //
impl VkPipelineRobustnessBufferBehavior {
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DEVICE_DEFAULT = 0;
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DISABLED = 1;
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS = 2;
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_2 = 3;
}

pub struct VkPipelineRobustnessImageBehavior(i32); //
impl VkPipelineRobustnessImageBehavior {
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DEVICE_DEFAULT = 0;
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DISABLED = 1;
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS = 2;
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_2 = 3;
}

pub type VkOpticalFlowGridSizeFlagBitsNV = ...; //
impl VkOpticalFlowGridSizeFlagBitsNV {
    pub const VK_OPTICAL_FLOW_GRID_SIZE_UNKNOWN_NV = 0;
    pub const VK_OPTICAL_FLOW_GRID_SIZE_1X1_BIT_NV = 1;
    pub const VK_OPTICAL_FLOW_GRID_SIZE_2X2_BIT_NV = 2;
    pub const VK_OPTICAL_FLOW_GRID_SIZE_4X4_BIT_NV = 4;
    pub const VK_OPTICAL_FLOW_GRID_SIZE_8X8_BIT_NV = 8;
}

pub type VkOpticalFlowUsageFlagBitsNV = ...; //
impl VkOpticalFlowUsageFlagBitsNV {
    pub const VK_OPTICAL_FLOW_USAGE_UNKNOWN_NV = 0;
    pub const VK_OPTICAL_FLOW_USAGE_INPUT_BIT_NV = 1;
    pub const VK_OPTICAL_FLOW_USAGE_OUTPUT_BIT_NV = 2;
    pub const VK_OPTICAL_FLOW_USAGE_HINT_BIT_NV = 4;
    pub const VK_OPTICAL_FLOW_USAGE_COST_BIT_NV = 8;
    pub const VK_OPTICAL_FLOW_USAGE_GLOBAL_FLOW_BIT_NV = 16;
}

pub struct VkOpticalFlowPerformanceLevelNV(i32); //
impl VkOpticalFlowPerformanceLevelNV {
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_UNKNOWN_NV = 0;
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_SLOW_NV = 1;
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_MEDIUM_NV = 2;
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_FAST_NV = 3;
}

pub struct VkOpticalFlowSessionBindingPointNV(i32); //
impl VkOpticalFlowSessionBindingPointNV {
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_UNKNOWN_NV = 0;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_INPUT_NV = 1;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_REFERENCE_NV = 2;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_HINT_NV = 3;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_FLOW_VECTOR_NV = 4;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_FLOW_VECTOR_NV = 5;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_COST_NV = 6;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_COST_NV = 7;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_GLOBAL_FLOW_NV = 8;
}

pub type VkOpticalFlowSessionCreateFlagBitsNV = ...; //
impl VkOpticalFlowSessionCreateFlagBitsNV {
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_HINT_BIT_NV = 1;
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_COST_BIT_NV = 2;
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_ENABLE_GLOBAL_FLOW_BIT_NV = 4;
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_ALLOW_REGIONS_BIT_NV = 8;
    pub const VK_OPTICAL_FLOW_SESSION_CREATE_BOTH_DIRECTIONS_BIT_NV = 16;
}

pub type VkOpticalFlowExecuteFlagBitsNV = ...; //
impl VkOpticalFlowExecuteFlagBitsNV {
    pub const VK_OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_NV = 1;
}

pub struct VkMicromapTypeEXT(i32); //
impl VkMicromapTypeEXT {
    pub const VK_MICROMAP_TYPE_OPACITY_MICROMAP_EXT = 0;
}

pub type VkBuildMicromapFlagBitsEXT = ...; //
impl VkBuildMicromapFlagBitsEXT {
    pub const VK_BUILD_MICROMAP_PREFER_FAST_TRACE_BIT_EXT = 1;
    pub const VK_BUILD_MICROMAP_PREFER_FAST_BUILD_BIT_EXT = 2;
    pub const VK_BUILD_MICROMAP_ALLOW_COMPACTION_BIT_EXT = 4;
}

pub type VkMicromapCreateFlagBitsEXT = ...; //
impl VkMicromapCreateFlagBitsEXT {
    pub const VK_MICROMAP_CREATE_DEVICE_ADDRESS_CAPTURE_REPLAY_BIT_EXT = 1;
}

pub struct VkCopyMicromapModeEXT(i32); //
impl VkCopyMicromapModeEXT {
    pub const VK_COPY_MICROMAP_MODE_CLONE_EXT = 0;
    pub const VK_COPY_MICROMAP_MODE_SERIALIZE_EXT = 1;
    pub const VK_COPY_MICROMAP_MODE_DESERIALIZE_EXT = 2;
    pub const VK_COPY_MICROMAP_MODE_COMPACT_EXT = 3;
}

pub struct VkBuildMicromapModeEXT(i32); //
impl VkBuildMicromapModeEXT {
    pub const VK_BUILD_MICROMAP_MODE_BUILD_EXT = 0;
}

pub struct VkOpacityMicromapFormatKHR(i32); //
impl VkOpacityMicromapFormatKHR {
    pub const VK_OPACITY_MICROMAP_FORMAT_2_STATE_KHR = 1;
    pub const VK_OPACITY_MICROMAP_FORMAT_4_STATE_KHR = 2;
}

pub struct VkOpacityMicromapSpecialIndexKHR(i32); //
impl VkOpacityMicromapSpecialIndexKHR {
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_TRANSPARENT_KHR = -1;
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_OPAQUE_KHR = -2;
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_TRANSPARENT_KHR = -3;
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_OPAQUE_KHR = -4;
}

pub struct VkAccelerationStructureSerializedBlockTypeKHR(i32); //
impl VkAccelerationStructureSerializedBlockTypeKHR {
    pub const VK_ACCELERATION_STRUCTURE_SERIALIZED_BLOCK_TYPE_OPACITY_MICROMAP_KHR = 0;
}

pub struct VkDepthBiasRepresentationEXT(i32); //
impl VkDepthBiasRepresentationEXT {
    pub const VK_DEPTH_BIAS_REPRESENTATION_LEAST_REPRESENTABLE_VALUE_FORMAT_EXT = 0;
    pub const VK_DEPTH_BIAS_REPRESENTATION_LEAST_REPRESENTABLE_VALUE_FORCE_UNORM_EXT = 1;
    pub const VK_DEPTH_BIAS_REPRESENTATION_FLOAT_EXT = 2;
}

pub type VkDeviceFaultFlagBitsKHR = ...; //
impl VkDeviceFaultFlagBitsKHR {
    pub const VK_DEVICE_FAULT_FLAG_DEVICE_LOST_KHR = 1;
    pub const VK_DEVICE_FAULT_FLAG_MEMORY_ADDRESS_KHR = 2;
    pub const VK_DEVICE_FAULT_FLAG_INSTRUCTION_ADDRESS_KHR = 4;
    pub const VK_DEVICE_FAULT_FLAG_VENDOR_KHR = 8;
    pub const VK_DEVICE_FAULT_FLAG_WATCHDOG_TIMEOUT_KHR = 16;
    pub const VK_DEVICE_FAULT_FLAG_OVERFLOW_KHR = 32;
}

pub struct VkDeviceFaultAddressTypeKHR(i32); //
impl VkDeviceFaultAddressTypeKHR {
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_NONE_KHR = 0;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_READ_INVALID_KHR = 1;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_WRITE_INVALID_KHR = 2;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_EXECUTE_INVALID_KHR = 3;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_UNKNOWN_KHR = 4;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_INVALID_KHR = 5;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_FAULT_KHR = 6;
}

pub struct VkDeviceFaultVendorBinaryHeaderVersionKHR(i32); //
impl VkDeviceFaultVendorBinaryHeaderVersionKHR {
    pub const VK_DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_ONE_KHR = 1;
    pub const VK_DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_ONE_EXT = ;
}

pub type VkIndirectCommandsLayoutUsageFlagBitsEXT = ...; //
impl VkIndirectCommandsLayoutUsageFlagBitsEXT {
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_EXPLICIT_PREPROCESS_BIT_EXT = 1;
    pub const VK_INDIRECT_COMMANDS_LAYOUT_USAGE_UNORDERED_SEQUENCES_BIT_EXT = 2;
}

pub struct VkIndirectExecutionSetInfoTypeEXT(i32); //
impl VkIndirectExecutionSetInfoTypeEXT {
    pub const VK_INDIRECT_EXECUTION_SET_INFO_TYPE_PIPELINES_EXT = 0;
    pub const VK_INDIRECT_EXECUTION_SET_INFO_TYPE_SHADER_OBJECTS_EXT = 1;
}

pub type VkIndirectCommandsInputModeFlagBitsEXT = ...; //
impl VkIndirectCommandsInputModeFlagBitsEXT {
    pub const VK_INDIRECT_COMMANDS_INPUT_MODE_VULKAN_INDEX_BUFFER_EXT = 1;
    pub const VK_INDIRECT_COMMANDS_INPUT_MODE_DXGI_INDEX_BUFFER_EXT = 2;
}

pub struct VkIndirectCommandsTokenTypeEXT(i32); //
impl VkIndirectCommandsTokenTypeEXT {
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_EXECUTION_SET_EXT = 0;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_EXT = 1;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_SEQUENCE_INDEX_EXT = 2;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_EXT = 3;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_EXT = 4;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_EXT = 5;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_EXT = 6;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_COUNT_EXT = 7;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_COUNT_EXT = 8;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DISPATCH_EXT = 9;
}

pub struct VkDisplacementMicromapFormatNV(i32); //
impl VkDisplacementMicromapFormatNV {
    pub const VK_DISPLACEMENT_MICROMAP_FORMAT_64_TRIANGLES_64_BYTES_NV = 1;
    pub const VK_DISPLACEMENT_MICROMAP_FORMAT_256_TRIANGLES_128_BYTES_NV = 2;
    pub const VK_DISPLACEMENT_MICROMAP_FORMAT_1024_TRIANGLES_128_BYTES_NV = 3;
}

pub type VkShaderCreateFlagBitsEXT = ...; //
impl VkShaderCreateFlagBitsEXT {
    pub const VK_SHADER_CREATE_LINK_STAGE_BIT_EXT = 1;
}

pub struct VkShaderCodeTypeEXT(i32); //
impl VkShaderCodeTypeEXT {
    pub const VK_SHADER_CODE_TYPE_BINARY_EXT = 0;
    pub const VK_SHADER_CODE_TYPE_SPIRV_EXT = 1;
}

pub struct VkScopeKHR(i32); //
impl VkScopeKHR {
    pub const VK_SCOPE_DEVICE_KHR = 1;
    pub const VK_SCOPE_WORKGROUP_KHR = 2;
    pub const VK_SCOPE_SUBGROUP_KHR = 3;
    pub const VK_SCOPE_QUEUE_FAMILY_KHR = 5;
}

pub struct VkComponentTypeKHR(i32); //
impl VkComponentTypeKHR {
    pub const VK_COMPONENT_TYPE_FLOAT16_KHR = 0;
    pub const VK_COMPONENT_TYPE_FLOAT32_KHR = 1;
    pub const VK_COMPONENT_TYPE_FLOAT64_KHR = 2;
    pub const VK_COMPONENT_TYPE_SINT8_KHR = 3;
    pub const VK_COMPONENT_TYPE_SINT16_KHR = 4;
    pub const VK_COMPONENT_TYPE_SINT32_KHR = 5;
    pub const VK_COMPONENT_TYPE_SINT64_KHR = 6;
    pub const VK_COMPONENT_TYPE_UINT8_KHR = 7;
    pub const VK_COMPONENT_TYPE_UINT16_KHR = 8;
    pub const VK_COMPONENT_TYPE_UINT32_KHR = 9;
    pub const VK_COMPONENT_TYPE_UINT64_KHR = 10;
}

pub struct VkCubicFilterWeightsQCOM(i32); //
impl VkCubicFilterWeightsQCOM {
    pub const VK_CUBIC_FILTER_WEIGHTS_CATMULL_ROM_QCOM = 0;
    pub const VK_CUBIC_FILTER_WEIGHTS_ZERO_TANGENT_CARDINAL_QCOM = 1;
    pub const VK_CUBIC_FILTER_WEIGHTS_B_SPLINE_QCOM = 2;
    pub const VK_CUBIC_FILTER_WEIGHTS_MITCHELL_NETRAVALI_QCOM = 3;
}

pub struct VkBlockMatchWindowCompareModeQCOM(i32); //
impl VkBlockMatchWindowCompareModeQCOM {
    pub const VK_BLOCK_MATCH_WINDOW_COMPARE_MODE_MIN_QCOM = 0;
    pub const VK_BLOCK_MATCH_WINDOW_COMPARE_MODE_MAX_QCOM = 1;
}

pub struct VkPhysicalDeviceLayeredApiKHR(i32); //
impl VkPhysicalDeviceLayeredApiKHR {
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_VULKAN_KHR = 0;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_D3D12_KHR = 1;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_METAL_KHR = 2;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_OPENGL_KHR = 3;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_OPENGLES_KHR = 4;
}

pub struct VkLayeredDriverUnderlyingApiMSFT(i32); //
impl VkLayeredDriverUnderlyingApiMSFT {
    pub const VK_LAYERED_DRIVER_UNDERLYING_API_NONE_MSFT = 0;
    pub const VK_LAYERED_DRIVER_UNDERLYING_API_D3D12_MSFT = 1;
}

pub struct VkLatencyMarkerNV(i32); //
impl VkLatencyMarkerNV {
    pub const VK_LATENCY_MARKER_SIMULATION_START_NV = 0;
    pub const VK_LATENCY_MARKER_SIMULATION_END_NV = 1;
    pub const VK_LATENCY_MARKER_RENDERSUBMIT_START_NV = 2;
    pub const VK_LATENCY_MARKER_RENDERSUBMIT_END_NV = 3;
    pub const VK_LATENCY_MARKER_PRESENT_START_NV = 4;
    pub const VK_LATENCY_MARKER_PRESENT_END_NV = 5;
    pub const VK_LATENCY_MARKER_INPUT_SAMPLE_NV = 6;
    pub const VK_LATENCY_MARKER_TRIGGER_FLASH_NV = 7;
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_RENDERSUBMIT_START_NV = 8;
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_RENDERSUBMIT_END_NV = 9;
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_PRESENT_START_NV = 10;
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_PRESENT_END_NV = 11;
}

pub struct VkOutOfBandQueueTypeNV(i32); //
impl VkOutOfBandQueueTypeNV {
    pub const VK_OUT_OF_BAND_QUEUE_TYPE_RENDER_NV = 0;
    pub const VK_OUT_OF_BAND_QUEUE_TYPE_PRESENT_NV = 1;
}

pub type VkMemoryUnmapFlagBits = ...; //
impl VkMemoryUnmapFlagBits {
}

pub struct VkCompressedTriangleFormatAMDX(i32); //
impl VkCompressedTriangleFormatAMDX {
    pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_AMDX = 0;
}

pub type VkWaylandSurfaceCreateFlagBitsKHR = ...; //
impl VkWaylandSurfaceCreateFlagBitsKHR {
}

pub struct VkDepthClampModeEXT(i32); //
impl VkDepthClampModeEXT {
    pub const VK_DEPTH_CLAMP_MODE_VIEWPORT_RANGE_EXT = 0;
    pub const VK_DEPTH_CLAMP_MODE_USER_DEFINED_RANGE_EXT = 1;
}

pub type VkAccessFlagBits3KHR = ...; //
impl VkAccessFlagBits3KHR {
    pub const VK_ACCESS_3_NONE_KHR = 0;
}

pub type VkTileShadingRenderPassFlagBitsQCOM = ...; //
impl VkTileShadingRenderPassFlagBitsQCOM {
    pub const VK_TILE_SHADING_RENDER_PASS_ENABLE_BIT_QCOM = 1;
    pub const VK_TILE_SHADING_RENDER_PASS_PER_TILE_EXECUTION_BIT_QCOM = 2;
}

pub struct VkCooperativeVectorMatrixLayoutNV(i32); //
impl VkCooperativeVectorMatrixLayoutNV {
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_ROW_MAJOR_NV = 0;
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_COLUMN_MAJOR_NV = 1;
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_INFERENCING_OPTIMAL_NV = 2;
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_TRAINING_OPTIMAL_NV = 3;
}

pub type VkAddressCopyFlagBitsKHR = ...; //
impl VkAddressCopyFlagBitsKHR {
    pub const VK_ADDRESS_COPY_DEVICE_LOCAL_BIT_KHR = 1;
    pub const VK_ADDRESS_COPY_SPARSE_BIT_KHR = 2;
    pub const VK_ADDRESS_COPY_PROTECTED_BIT_KHR = 4;
}

pub type VkTensorCreateFlagBitsARM = ...; //
impl VkTensorCreateFlagBitsARM {
    pub const VK_TENSOR_CREATE_MUTABLE_FORMAT_BIT_ARM = 1;
    pub const VK_TENSOR_CREATE_PROTECTED_BIT_ARM = 2;
}

pub type VkTensorUsageFlagBitsARM = ...; //
impl VkTensorUsageFlagBitsARM {
    pub const VK_TENSOR_USAGE_SHADER_BIT_ARM = 2;
    pub const VK_TENSOR_USAGE_TRANSFER_SRC_BIT_ARM = 4;
    pub const VK_TENSOR_USAGE_TRANSFER_DST_BIT_ARM = 8;
    pub const VK_TENSOR_USAGE_IMAGE_ALIASING_BIT_ARM = 16;
}

pub struct VkTensorTilingARM(i32); //
impl VkTensorTilingARM {
    pub const VK_TENSOR_TILING_OPTIMAL_ARM = 0;
    pub const VK_TENSOR_TILING_LINEAR_ARM = 1;
}

pub type VkTensorViewCreateFlagBitsARM = ...; //
impl VkTensorViewCreateFlagBitsARM {
}

pub struct VkDefaultVertexAttributeValueKHR(i32); //
impl VkDefaultVertexAttributeValueKHR {
    pub const VK_DEFAULT_VERTEX_ATTRIBUTE_VALUE_ZERO_ZERO_ZERO_ZERO_KHR = 0;
    pub const VK_DEFAULT_VERTEX_ATTRIBUTE_VALUE_ZERO_ZERO_ZERO_ONE_KHR = 1;
}

pub type VkDataGraphPipelineSessionCreateFlagBitsARM = ...; //
impl VkDataGraphPipelineSessionCreateFlagBitsARM {
    pub const VK_DATA_GRAPH_PIPELINE_SESSION_CREATE_PROTECTED_BIT_ARM = 1;
}

pub struct VkDataGraphPipelineSessionBindPointARM(i32); //
impl VkDataGraphPipelineSessionBindPointARM {
    pub const VK_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_TRANSIENT_ARM = 0;
}

pub struct VkDataGraphPipelineSessionBindPointTypeARM(i32); //
impl VkDataGraphPipelineSessionBindPointTypeARM {
    pub const VK_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_TYPE_MEMORY_ARM = 0;
}

pub struct VkDataGraphPipelinePropertyARM(i32); //
impl VkDataGraphPipelinePropertyARM {
    pub const VK_DATA_GRAPH_PIPELINE_PROPERTY_CREATION_LOG_ARM = 0;
    pub const VK_DATA_GRAPH_PIPELINE_PROPERTY_IDENTIFIER_ARM = 1;
}

pub type VkDataGraphPipelineDispatchFlagBitsARM = ...; //
impl VkDataGraphPipelineDispatchFlagBitsARM {
}

pub struct VkPhysicalDeviceDataGraphProcessingEngineTypeARM(i32); //
impl VkPhysicalDeviceDataGraphProcessingEngineTypeARM {
    pub const VK_PHYSICAL_DEVICE_DATA_GRAPH_PROCESSING_ENGINE_TYPE_DEFAULT_ARM = 0;
}

pub struct VkPhysicalDeviceDataGraphOperationTypeARM(i32); //
impl VkPhysicalDeviceDataGraphOperationTypeARM {
    pub const VK_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_TYPE_SPIRV_EXTENDED_INSTRUCTION_SET_ARM = 0;
}

pub struct VkDataGraphModelCacheTypeQCOM(i32); //
impl VkDataGraphModelCacheTypeQCOM {
    pub const VK_DATA_GRAPH_MODEL_CACHE_TYPE_GENERIC_BINARY_QCOM = 0;
}

pub struct VkPerfHintTypeQCOM(i32); //
impl VkPerfHintTypeQCOM {
    pub const VK_PERF_HINT_TYPE_DEFAULT_QCOM = 0;
    pub const VK_PERF_HINT_TYPE_FREQUENCY_MIN_QCOM = 1;
    pub const VK_PERF_HINT_TYPE_FREQUENCY_MAX_QCOM = 2;
    pub const VK_PERF_HINT_TYPE_FREQUENCY_SCALED_QCOM = 3;
}

pub struct VkThrottleHintTypeSEC(i32); //
impl VkThrottleHintTypeSEC {
    pub const VK_THROTTLE_HINT_TYPE_DEFAULT_SEC = 0;
    pub const VK_THROTTLE_HINT_TYPE_LOW_SEC = 1;
    pub const VK_THROTTLE_HINT_TYPE_HIGH_SEC = 2;
}

pub type VkVideoEncodeRgbModelConversionFlagBitsVALVE = ...; //
impl VkVideoEncodeRgbModelConversionFlagBitsVALVE {
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_RGB_IDENTITY_BIT_VALVE = 1;
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_IDENTITY_BIT_VALVE = 2;
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_709_BIT_VALVE = 4;
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_601_BIT_VALVE = 8;
    pub const VK_VIDEO_ENCODE_RGB_MODEL_CONVERSION_YCBCR_2020_BIT_VALVE = 16;
}

pub type VkVideoEncodeRgbRangeCompressionFlagBitsVALVE = ...; //
impl VkVideoEncodeRgbRangeCompressionFlagBitsVALVE {
    pub const VK_VIDEO_ENCODE_RGB_RANGE_COMPRESSION_FULL_RANGE_BIT_VALVE = 1;
    pub const VK_VIDEO_ENCODE_RGB_RANGE_COMPRESSION_NARROW_RANGE_BIT_VALVE = 2;
}

pub type VkVideoEncodeRgbChromaOffsetFlagBitsVALVE = ...; //
impl VkVideoEncodeRgbChromaOffsetFlagBitsVALVE {
    pub const VK_VIDEO_ENCODE_RGB_CHROMA_OFFSET_COSITED_EVEN_BIT_VALVE = 1;
    pub const VK_VIDEO_ENCODE_RGB_CHROMA_OFFSET_MIDPOINT_BIT_VALVE = 2;
}

pub type VkSwapchainImageUsageFlagBitsOHOS = ...; //
impl VkSwapchainImageUsageFlagBitsOHOS {
    pub const VK_SWAPCHAIN_IMAGE_USAGE_SHARED_BIT_OHOS = 1;
}

pub struct VkDescriptorMappingSourceEXT(i32); //
impl VkDescriptorMappingSourceEXT {
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_CONSTANT_OFFSET_EXT = 0;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_PUSH_INDEX_EXT = 1;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_INDIRECT_INDEX_EXT = 2;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_INDIRECT_INDEX_ARRAY_EXT = 3;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_RESOURCE_HEAP_DATA_EXT = 4;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_PUSH_DATA_EXT = 5;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_PUSH_ADDRESS_EXT = 6;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_INDIRECT_ADDRESS_EXT = 7;
}

pub type VkSpirvResourceTypeFlagBitsEXT = ...; //
impl VkSpirvResourceTypeFlagBitsEXT {
    pub const VK_SPIRV_RESOURCE_TYPE_ALL_EXT = 0;
    pub const VK_SPIRV_RESOURCE_TYPE_SAMPLER_BIT_EXT = 1;
    pub const VK_SPIRV_RESOURCE_TYPE_SAMPLED_IMAGE_BIT_EXT = 2;
    pub const VK_SPIRV_RESOURCE_TYPE_READ_ONLY_IMAGE_BIT_EXT = 4;
    pub const VK_SPIRV_RESOURCE_TYPE_READ_WRITE_IMAGE_BIT_EXT = 8;
    pub const VK_SPIRV_RESOURCE_TYPE_COMBINED_SAMPLED_IMAGE_BIT_EXT = 16;
    pub const VK_SPIRV_RESOURCE_TYPE_UNIFORM_BUFFER_BIT_EXT = 32;
    pub const VK_SPIRV_RESOURCE_TYPE_READ_ONLY_STORAGE_BUFFER_BIT_EXT = 64;
    pub const VK_SPIRV_RESOURCE_TYPE_READ_WRITE_STORAGE_BUFFER_BIT_EXT = 128;
}

pub type VkGpaSqShaderStageFlagBitsAMD = ...; //
impl VkGpaSqShaderStageFlagBitsAMD {
    pub const VK_GPA_SQ_SHADER_STAGE_PS_BIT_AMD = 1;
    pub const VK_GPA_SQ_SHADER_STAGE_VS_BIT_AMD = 2;
    pub const VK_GPA_SQ_SHADER_STAGE_GS_BIT_AMD = 4;
    pub const VK_GPA_SQ_SHADER_STAGE_ES_BIT_AMD = 8;
    pub const VK_GPA_SQ_SHADER_STAGE_HS_BIT_AMD = 16;
    pub const VK_GPA_SQ_SHADER_STAGE_LS_BIT_AMD = 32;
    pub const VK_GPA_SQ_SHADER_STAGE_CS_BIT_AMD = 64;
}

pub struct VkGpaPerfBlockAMD(i32); //
impl VkGpaPerfBlockAMD {
    pub const VK_GPA_PERF_BLOCK_CPF_AMD = 0;
    pub const VK_GPA_PERF_BLOCK_IA_AMD = 1;
    pub const VK_GPA_PERF_BLOCK_VGT_AMD = 2;
    pub const VK_GPA_PERF_BLOCK_PA_AMD = 3;
    pub const VK_GPA_PERF_BLOCK_SC_AMD = 4;
    pub const VK_GPA_PERF_BLOCK_SPI_AMD = 5;
    pub const VK_GPA_PERF_BLOCK_SQ_AMD = 6;
    pub const VK_GPA_PERF_BLOCK_SX_AMD = 7;
    pub const VK_GPA_PERF_BLOCK_TA_AMD = 8;
    pub const VK_GPA_PERF_BLOCK_TD_AMD = 9;
    pub const VK_GPA_PERF_BLOCK_TCP_AMD = 10;
    pub const VK_GPA_PERF_BLOCK_TCC_AMD = 11;
    pub const VK_GPA_PERF_BLOCK_TCA_AMD = 12;
    pub const VK_GPA_PERF_BLOCK_DB_AMD = 13;
    pub const VK_GPA_PERF_BLOCK_CB_AMD = 14;
    pub const VK_GPA_PERF_BLOCK_GDS_AMD = 15;
    pub const VK_GPA_PERF_BLOCK_SRBM_AMD = 16;
    pub const VK_GPA_PERF_BLOCK_GRBM_AMD = 17;
    pub const VK_GPA_PERF_BLOCK_GRBM_SE_AMD = 18;
    pub const VK_GPA_PERF_BLOCK_RLC_AMD = 19;
    pub const VK_GPA_PERF_BLOCK_DMA_AMD = 20;
    pub const VK_GPA_PERF_BLOCK_MC_AMD = 21;
    pub const VK_GPA_PERF_BLOCK_CPG_AMD = 22;
    pub const VK_GPA_PERF_BLOCK_CPC_AMD = 23;
    pub const VK_GPA_PERF_BLOCK_WD_AMD = 24;
    pub const VK_GPA_PERF_BLOCK_TCS_AMD = 25;
    pub const VK_GPA_PERF_BLOCK_ATC_AMD = 26;
    pub const VK_GPA_PERF_BLOCK_ATC_L2_AMD = 27;
    pub const VK_GPA_PERF_BLOCK_MC_VM_L2_AMD = 28;
    pub const VK_GPA_PERF_BLOCK_EA_AMD = 29;
    pub const VK_GPA_PERF_BLOCK_RPB_AMD = 30;
    pub const VK_GPA_PERF_BLOCK_RMI_AMD = 31;
    pub const VK_GPA_PERF_BLOCK_UMCCH_AMD = 32;
    pub const VK_GPA_PERF_BLOCK_GE_AMD = 33;
    pub const VK_GPA_PERF_BLOCK_GL1A_AMD = 34;
    pub const VK_GPA_PERF_BLOCK_GL1C_AMD = 35;
    pub const VK_GPA_PERF_BLOCK_GL1CG_AMD = 36;
    pub const VK_GPA_PERF_BLOCK_GL2A_AMD = 37;
    pub const VK_GPA_PERF_BLOCK_GL2C_AMD = 38;
    pub const VK_GPA_PERF_BLOCK_CHA_AMD = 39;
    pub const VK_GPA_PERF_BLOCK_CHC_AMD = 40;
    pub const VK_GPA_PERF_BLOCK_CHCG_AMD = 41;
    pub const VK_GPA_PERF_BLOCK_GUS_AMD = 42;
    pub const VK_GPA_PERF_BLOCK_GCR_AMD = 43;
    pub const VK_GPA_PERF_BLOCK_PH_AMD = 44;
    pub const VK_GPA_PERF_BLOCK_UTCL1_AMD = 45;
    pub const VK_GPA_PERF_BLOCK_GE1_AMD = ;
    pub const VK_GPA_PERF_BLOCK_GE_DIST_AMD = 46;
    pub const VK_GPA_PERF_BLOCK_GE_SE_AMD = 47;
    pub const VK_GPA_PERF_BLOCK_DF_MALL_AMD = 48;
    pub const VK_GPA_PERF_BLOCK_SQ_WGP_AMD = 49;
    pub const VK_GPA_PERF_BLOCK_PC_AMD = 50;
    pub const VK_GPA_PERF_BLOCK_GL1XA_AMD = 51;
    pub const VK_GPA_PERF_BLOCK_GL1XC_AMD = 52;
    pub const VK_GPA_PERF_BLOCK_WGS_AMD = 53;
    pub const VK_GPA_PERF_BLOCK_EACPWD_AMD = 54;
    pub const VK_GPA_PERF_BLOCK_EASE_AMD = 55;
    pub const VK_GPA_PERF_BLOCK_RLCUSER_AMD = 56;
    pub const VK_GPA_PERF_BLOCK_RLCLOCAL_AMD = ;
}

pub struct VkGpaSampleTypeAMD(i32); //
impl VkGpaSampleTypeAMD {
    pub const VK_GPA_SAMPLE_TYPE_CUMULATIVE_AMD = 0;
    pub const VK_GPA_SAMPLE_TYPE_TRACE_AMD = 1;
    pub const VK_GPA_SAMPLE_TYPE_TIMING_AMD = 2;
}

pub struct VkGpaDeviceClockModeAMD(i32); //
impl VkGpaDeviceClockModeAMD {
    pub const VK_GPA_DEVICE_CLOCK_MODE_DEFAULT_AMD = 0;
    pub const VK_GPA_DEVICE_CLOCK_MODE_QUERY_AMD = 1;
    pub const VK_GPA_DEVICE_CLOCK_MODE_PROFILING_AMD = 2;
    pub const VK_GPA_DEVICE_CLOCK_MODE_MIN_MEMORY_AMD = 3;
    pub const VK_GPA_DEVICE_CLOCK_MODE_MIN_ENGINE_AMD = 4;
    pub const VK_GPA_DEVICE_CLOCK_MODE_PEAK_AMD = 5;
}

pub type VkAddressCommandFlagBitsKHR = ...; //
impl VkAddressCommandFlagBitsKHR {
    pub const VK_ADDRESS_COMMAND_PROTECTED_BIT_KHR = 1;
    pub const VK_ADDRESS_COMMAND_FULLY_BOUND_BIT_KHR = 2;
    pub const VK_ADDRESS_COMMAND_STORAGE_BUFFER_USAGE_BIT_KHR = 4;
    pub const VK_ADDRESS_COMMAND_UNKNOWN_STORAGE_BUFFER_USAGE_BIT_KHR = 8;
}

pub type VkDataGraphTOSAQualityFlagBitsARM = ...; //
impl VkDataGraphTOSAQualityFlagBitsARM {
    pub const VK_DATA_GRAPH_TOSA_QUALITY_ACCELERATED_ARM = 1;
    pub const VK_DATA_GRAPH_TOSA_QUALITY_CONFORMANT_ARM = 2;
    pub const VK_DATA_GRAPH_TOSA_QUALITY_EXPERIMENTAL_ARM = 4;
    pub const VK_DATA_GRAPH_TOSA_QUALITY_DEPRECATED_ARM = 8;
}

pub struct VkDataGraphTOSALevelARM(i32); //
impl VkDataGraphTOSALevelARM {
    pub const VK_DATA_GRAPH_TOSA_LEVEL_NONE_ARM = 0;
    pub const VK_DATA_GRAPH_TOSA_LEVEL_8K_ARM = 1;
}

pub type VkDataGraphOpticalFlowGridSizeFlagBitsARM = ...; //
impl VkDataGraphOpticalFlowGridSizeFlagBitsARM {
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_UNKNOWN_ARM = 0;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_1X1_BIT_ARM = 1;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_2X2_BIT_ARM = 2;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_4X4_BIT_ARM = 4;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_GRID_SIZE_8X8_BIT_ARM = 8;
}

pub type VkDataGraphOpticalFlowImageUsageFlagBitsARM = ...; //
impl VkDataGraphOpticalFlowImageUsageFlagBitsARM {
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_UNKNOWN_ARM = 0;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_INPUT_BIT_ARM = 1;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_OUTPUT_BIT_ARM = 2;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_HINT_BIT_ARM = 4;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_IMAGE_USAGE_COST_BIT_ARM = 8;
}

pub struct VkDataGraphOpticalFlowPerformanceLevelARM(i32); //
impl VkDataGraphOpticalFlowPerformanceLevelARM {
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_UNKNOWN_ARM = 0;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_SLOW_ARM = 1;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_MEDIUM_ARM = 2;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_FAST_ARM = 3;
}

pub struct VkDataGraphPipelineNodeConnectionTypeARM(i32); //
impl VkDataGraphPipelineNodeConnectionTypeARM {
}

pub struct VkDataGraphPipelineNodeTypeARM(i32); //
impl VkDataGraphPipelineNodeTypeARM {
}

pub type VkDataGraphOpticalFlowCreateFlagBitsARM = ...; //
impl VkDataGraphOpticalFlowCreateFlagBitsARM {
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_ENABLE_HINT_BIT_ARM = 1;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_ENABLE_COST_BIT_ARM = 2;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_CREATE_RESERVED_30_BIT_ARM = 1073741824;
}

pub type VkDataGraphOpticalFlowExecuteFlagBitsARM = ...; //
impl VkDataGraphOpticalFlowExecuteFlagBitsARM {
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_DISABLE_TEMPORAL_HINTS_BIT_ARM = 1;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_INPUT_UNCHANGED_BIT_ARM = 2;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_REFERENCE_UNCHANGED_BIT_ARM = 4;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_INPUT_IS_PREVIOUS_REFERENCE_BIT_ARM = 8;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_EXECUTE_REFERENCE_IS_PREVIOUS_INPUT_BIT_ARM = 16;
}

pub struct VkNeuralAcceleratorStatisticsModeARM(i32); //
impl VkNeuralAcceleratorStatisticsModeARM {
    pub const VK_NEURAL_ACCELERATOR_STATISTICS_MODE_DISABLED_ARM = 0;
    pub const VK_NEURAL_ACCELERATOR_STATISTICS_MODE_STATISTICS0_ARM = 1;
    pub const VK_NEURAL_ACCELERATOR_STATISTICS_MODE_STATISTICS1_ARM = 2;
}

pub struct VkImageTilingControlEXT(i32); //
impl VkImageTilingControlEXT {
    pub const VK_IMAGE_TILING_CONTROL_DEFAULT_EXT = 0;
    pub const VK_IMAGE_TILING_CONTROL_MIN_SIZE_EXT = 1;
    pub const VK_IMAGE_TILING_CONTROL_MAX_PERFORMANCE_EXT = 2;
}

