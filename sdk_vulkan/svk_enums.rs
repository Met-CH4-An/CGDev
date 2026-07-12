// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// Provided by VK_VERSION_1_0
#[repr(C)]
pub enum VkSystemAllocationScope {
    VK_SYSTEM_ALLOCATION_SCOPE_COMMAND = 0,
    VK_SYSTEM_ALLOCATION_SCOPE_OBJECT = 1,
    VK_SYSTEM_ALLOCATION_SCOPE_CACHE = 2,
    VK_SYSTEM_ALLOCATION_SCOPE_DEVICE = 3,
    VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE = 4,
}

// Provided by VK_VERSION_1_0
#[repr(C)]
pub enum VkInternalAllocationType {
    VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE = 0,
}

// Provided by VK_VERSION_1_0
pub type VkStructureType = crate::svk_types::r#enum;
pub mod VkStructureTypeValue {
    use crate::VkStructureType;

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
    pub const VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO : VkStructureType = 47;      // comment="Reserved for internal use by the loader;  layers;  and ICDs
    pub const VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO : VkStructureType = 48;        // comment="Reserved for internal use by the loader;  layers;  and ICDs
}

// Provided by VK_VERSION_1_0
pub type VkResult = i32;

// Provided by VK_VERSION_1_0
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
    // Provided by VK_VERSION_1_0
    pub const VK_ERROR_VALIDATION_FAILED : VkResult = -1000011001;
    // Provided by VK_VERSION_1_1
    pub const VK_ERROR_OUT_OF_POOL_MEMORY : VkResult = -1000069000;
    // Provided by VK_VERSION_1_1
    pub const VK_ERROR_INVALID_EXTERNAL_HANDLE : VkResult = -1000072003;
    // Provided by VK_VERSION_1_2
    pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS : VkResult = -1000257000;
    // Provided by VK_VERSION_1_2
    pub const VK_ERROR_FRAGMENTATION : VkResult = -1000161000;
    // Provided by VK_VERSION_1_3
    pub const VK_PIPELINE_COMPILE_REQUIRED : VkResult = 1000297000;
    // Provided by VK_VERSION_1_4
    pub const VK_ERROR_NOT_PERMITTED : VkResult = -1000174001;
    // Provided by VK_KHR_surface
    pub const VK_ERROR_SURFACE_LOST_KHR : VkResult = -1000000000;
    // Provided by VK_KHR_surface
    pub const VK_ERROR_NATIVE_WINDOW_IN_USE_KHR : VkResult = -1000000001;
    // Provided by VK_KHR_swapchain
    pub const VK_SUBOPTIMAL_KHR : VkResult = 1000001003;
    // Provided by VK_KHR_swapchain
    pub const VK_ERROR_OUT_OF_DATE_KHR : VkResult = -1000001004;
    // Provided by VK_KHR_display_swapchain
    pub const VK_ERROR_INCOMPATIBLE_DISPLAY_KHR : VkResult = -1000003001;
    // Provided by VK_NV_glsl_shader
    pub const VK_ERROR_INVALID_SHADER_NV : VkResult = -1000012000;
    // Provided by VK_KHR_video_queue
    pub const VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR : VkResult = -1000023000;
    // Provided by VK_KHR_video_queue
    pub const VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR : VkResult = -1000023001;
    // Provided by VK_KHR_video_queue
    pub const VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR : VkResult = -1000023002;
    // Provided by VK_KHR_video_queue
    pub const VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR : VkResult = -1000023003;
    // Provided by VK_KHR_video_queue
    pub const VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR : VkResult = -1000023004;
    // Provided by VK_KHR_video_queue
    pub const VK_ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR : VkResult = -1000023005;
    // Provided by VK_EXT_image_drm_format_modifier
    pub const VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT : VkResult = -1000158000;
    // Provided by VK_EXT_present_timing
    pub const VK_ERROR_PRESENT_TIMING_QUEUE_FULL_EXT : VkResult = -1000208000;
    // Provided by VK_EXT_full_screen_exclusive
    pub const VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT : VkResult = -1000255000;
    // Provided by VK_KHR_deferred_host_operations
    pub const VK_THREAD_IDLE_KHR : VkResult = 1000268000;
    // Provided by VK_KHR_deferred_host_operations
    pub const VK_THREAD_DONE_KHR : VkResult = 1000268001;
    // Provided by VK_KHR_deferred_host_operations
    pub const VK_OPERATION_DEFERRED_KHR : VkResult = 1000268002;
    // Provided by VK_KHR_deferred_host_operations
    pub const VK_OPERATION_NOT_DEFERRED_KHR : VkResult = 1000268003;
    // Provided by VK_KHR_video_encode_queue
    pub const VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR : VkResult = -1000299000;
    // Provided by VK_EXT_image_compression_control
    pub const VK_ERROR_COMPRESSION_EXHAUSTED_EXT : VkResult = -1000338000;
    // Provided by VK_EXT_shader_object
    pub const VK_INCOMPATIBLE_SHADER_BINARY_EXT : VkResult = 1000482000;
    // Provided by VK_KHR_pipeline_binary
    pub const VK_PIPELINE_BINARY_MISSING_KHR : VkResult = 1000483000;
    // Provided by VK_KHR_pipeline_binary
    pub const VK_ERROR_NOT_ENOUGH_SPACE_KHR : VkResult = -1000483000;
    // Provided by VK_EXT_debug_report
    pub const VK_ERROR_VALIDATION_FAILED_EXT : VkResult = VK_ERROR_VALIDATION_FAILED;
    // Provided by VK_KHR_maintenance1
    pub const VK_ERROR_OUT_OF_POOL_MEMORY_KHR : VkResult = VK_ERROR_OUT_OF_POOL_MEMORY;
    // Provided by VK_KHR_external_memory
    pub const VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR : VkResult = VK_ERROR_INVALID_EXTERNAL_HANDLE;
    // Provided by VK_EXT_descriptor_indexing
    pub const VK_ERROR_FRAGMENTATION_EXT : VkResult = VK_ERROR_FRAGMENTATION;
    // Provided by VK_EXT_global_priority
    pub const VK_ERROR_NOT_PERMITTED_EXT : VkResult = VK_ERROR_NOT_PERMITTED;
    // Provided by VK_KHR_global_priority
    pub const VK_ERROR_NOT_PERMITTED_KHR : VkResult = VK_ERROR_NOT_PERMITTED;
    // Provided by VK_EXT_buffer_device_address
    pub const VK_ERROR_INVALID_DEVICE_ADDRESS_EXT : VkResult = VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
    // Provided by VK_KHR_buffer_device_address
    pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR : VkResult = VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS;
    // Provided by VK_EXT_pipeline_creation_cache_control
    pub const VK_PIPELINE_COMPILE_REQUIRED_EXT : VkResult = VK_PIPELINE_COMPILE_REQUIRED;
    // Provided by VK_EXT_pipeline_creation_cache_control
    pub const VK_ERROR_PIPELINE_COMPILE_REQUIRED_EXT : VkResult = VK_PIPELINE_COMPILE_REQUIRED;
    // Provided by VK_EXT_shader_object
    // VK_ERROR_INCOMPATIBLE_SHADER_BINARY_EXT is a legacy alias
    pub const VK_ERROR_INCOMPATIBLE_SHADER_BINARY_EXT : VkResult = VK_INCOMPATIBLE_SHADER_BINARY_EXT;
}

// Provided by VK_VERSION_1_0
pub type VkInstanceCreateFlagBits = crate::svk_types::bitmask;
pub mod VkInstanceCreateFlagBitsValue {
    use crate::VkInstanceCreateFlagBits;

    // Provided by VK_KHR_portability_enumeration
    pub const VK_INSTANCE_CREATE_ENUMERATE_PORTABILITY_BIT_KHR : VkInstanceCreateFlagBits = 0x00000001;
}

// Provided by VK_EXT_debug_report
pub type VkDebugReportFlagBitsEXT = crate::svk_types::bitmask;
pub mod VkDebugReportFlagBitsEXTValue {
    use crate::VkDebugReportFlagBitsEXT;

    pub const VK_DEBUG_REPORT_INFORMATION_BIT_EXT : VkDebugReportFlagBitsEXT = 0x00000001;
    pub const VK_DEBUG_REPORT_WARNING_BIT_EXT : VkDebugReportFlagBitsEXT = 0x00000002;
    pub const VK_DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT : VkDebugReportFlagBitsEXT = 0x00000004;
    pub const VK_DEBUG_REPORT_ERROR_BIT_EXT : VkDebugReportFlagBitsEXT = 0x00000008;
    pub const VK_DEBUG_REPORT_DEBUG_BIT_EXT : VkDebugReportFlagBitsEXT = 0x00000010;
}

// Provided by VK_EXT_debug_marker, VK_EXT_debug_report
pub type VkDebugReportObjectTypeEXT = crate::svk_types::r#enum;
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
    // Provided by VK_VERSION_1_1 with VK_EXT_debug_report, VK_KHR_sampler_ycbcr_conversion with VK_EXT_debug_report
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT : VkDebugReportObjectTypeEXT = 1000156000;
    // Provided by VK_VERSION_1_1 with VK_EXT_debug_report
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT : VkDebugReportObjectTypeEXT = 1000085000;
    // Provided by VK_EXT_debug_report with VK_NVX_binary_import
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_CU_MODULE_NVX_EXT : VkDebugReportObjectTypeEXT = 1000029000;
    // Provided by VK_EXT_debug_report with VK_NVX_binary_import
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_CU_FUNCTION_NVX_EXT : VkDebugReportObjectTypeEXT = 1000029001;
    // Provided by VK_KHR_acceleration_structure with VK_EXT_debug_report
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR_EXT : VkDebugReportObjectTypeEXT = 1000150000;
    // Provided by VK_EXT_debug_report with VK_NV_ray_tracing
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV_EXT : VkDebugReportObjectTypeEXT = 1000165000;
    // Provided by VK_EXT_debug_report with VK_NV_cuda_kernel_launch
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_CUDA_MODULE_NV_EXT : VkDebugReportObjectTypeEXT = 1000307000;
    // Provided by VK_EXT_debug_report with VK_NV_cuda_kernel_launch
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_CUDA_FUNCTION_NV_EXT : VkDebugReportObjectTypeEXT = 1000307001;
    // Provided by VK_EXT_debug_report with VK_FUCHSIA_buffer_collection
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA_EXT : VkDebugReportObjectTypeEXT = 1000366000;
    // VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT is a legacy alias
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT : VkDebugReportObjectTypeEXT = VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT;
    // VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT is a legacy alias
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT : VkDebugReportObjectTypeEXT = VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT;
    // Provided by VK_KHR_descriptor_update_template with VK_EXT_debug_report
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR_EXT : VkDebugReportObjectTypeEXT = VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT;
    // Provided by VK_KHR_sampler_ycbcr_conversion with VK_EXT_debug_report
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR_EXT : VkDebugReportObjectTypeEXT = VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT;
}

// Provided by VK_EXT_debug_utils
pub type VkDebugUtilsMessageSeverityFlagBitsEXT = crate::svk_types::bitmask;
pub mod VkDebugUtilsMessageSeverityFlagBitsEXTValue {
    use crate::VkDebugUtilsMessageSeverityFlagBitsEXT;

    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT : VkDebugUtilsMessageSeverityFlagBitsEXT = 0x00000001;
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT : VkDebugUtilsMessageSeverityFlagBitsEXT = 0x00000010;
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT : VkDebugUtilsMessageSeverityFlagBitsEXT = 0x00000100;
    pub const VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT : VkDebugUtilsMessageSeverityFlagBitsEXT = 0x00001000;
}

// Provided by VK_EXT_debug_utils
pub type VkDebugUtilsMessageTypeFlagBitsEXT = crate::svk_types::bitmask;
pub mod VkDebugUtilsMessageTypeFlagBitsEXTValue {
    use crate::VkDebugUtilsMessageTypeFlagBitsEXT;
    
    pub const VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT : VkDebugUtilsMessageTypeFlagBitsEXT = 0x00000001;
    pub const VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT : VkDebugUtilsMessageTypeFlagBitsEXT = 0x00000002;
    pub const VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT : VkDebugUtilsMessageTypeFlagBitsEXT = 0x00000004;
    // Provided by VK_EXT_device_address_binding_report
    pub const VK_DEBUG_UTILS_MESSAGE_TYPE_DEVICE_ADDRESS_BINDING_BIT_EXT : VkDebugUtilsMessageTypeFlagBitsEXT = 0x00000008;
}

// Provided by VK_VERSION_1_0
pub type VkObjectType = crate::svk_types::r#enum;
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
    // Provided by VK_VERSION_1_1
    pub const VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE : VkObjectType = 1000085000;
    // Provided by VK_VERSION_1_1
    pub const VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION : VkObjectType = 1000156000;
    // Provided by VK_VERSION_1_3
    pub const VK_OBJECT_TYPE_PRIVATE_DATA_SLOT : VkObjectType = 1000295000;
    // Provided by VK_KHR_surface
    pub const VK_OBJECT_TYPE_SURFACE_KHR : VkObjectType = 1000000000;
    // Provided by VK_KHR_swapchain
    pub const VK_OBJECT_TYPE_SWAPCHAIN_KHR : VkObjectType = 1000001000;
    // Provided by VK_KHR_display
    pub const VK_OBJECT_TYPE_DISPLAY_KHR : VkObjectType = 1000002000;
    // Provided by VK_KHR_display
    pub const VK_OBJECT_TYPE_DISPLAY_MODE_KHR : VkObjectType = 1000002001;
    // Provided by VK_EXT_debug_report
    pub const VK_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT : VkObjectType = 1000011000;
    // Provided by VK_KHR_video_queue
    pub const VK_OBJECT_TYPE_VIDEO_SESSION_KHR : VkObjectType = 1000023000;
    // Provided by VK_KHR_video_queue
    pub const VK_OBJECT_TYPE_VIDEO_SESSION_PARAMETERS_KHR : VkObjectType = 1000023001;
    // Provided by VK_NVX_binary_import
    pub const VK_OBJECT_TYPE_CU_MODULE_NVX : VkObjectType = 1000029000;
    // Provided by VK_NVX_binary_import
    pub const VK_OBJECT_TYPE_CU_FUNCTION_NVX : VkObjectType = 1000029001;
    // Provided by VK_EXT_debug_utils
    pub const VK_OBJECT_TYPE_DEBUG_UTILS_MESSENGER_EXT : VkObjectType = 1000128000;
    // Provided by VK_AMD_gpa_interface
    pub const VK_OBJECT_TYPE_GPA_SESSION_AMD : VkObjectType = 1000133000;
    // Provided by VK_KHR_acceleration_structure
    pub const VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_KHR : VkObjectType = 1000150000;
    // Provided by VK_EXT_validation_cache
    pub const VK_OBJECT_TYPE_VALIDATION_CACHE_EXT : VkObjectType = 1000160000;
    // Provided by VK_NV_ray_tracing
    pub const VK_OBJECT_TYPE_ACCELERATION_STRUCTURE_NV : VkObjectType = 1000165000;
    // Provided by VK_INTEL_performance_query
    pub const VK_OBJECT_TYPE_PERFORMANCE_CONFIGURATION_INTEL : VkObjectType = 1000210000;
    // Provided by VK_KHR_deferred_host_operations
    pub const VK_OBJECT_TYPE_DEFERRED_OPERATION_KHR : VkObjectType = 1000268000;
    // Provided by VK_NV_device_generated_commands
    pub const VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_NV : VkObjectType = 1000277000;
/*#ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_NV_cuda_kernel_launch
VK_OBJECT_TYPE_CUDA_MODULE_NV = 1000307000;
#endif
#ifdef VK_ENABLE_BETA_EXTENSIONS
// Provided by VK_NV_cuda_kernel_launch
VK_OBJECT_TYPE_CUDA_FUNCTION_NV = 1000307001;
#endif*/
    // Provided by VK_FUCHSIA_buffer_collection
    pub const VK_OBJECT_TYPE_BUFFER_COLLECTION_FUCHSIA : VkObjectType = 1000366000;
    // Provided by VK_EXT_opacity_micromap
    pub const VK_OBJECT_TYPE_MICROMAP_EXT : VkObjectType = 1000396000;
    // Provided by VK_ARM_tensors
    pub const VK_OBJECT_TYPE_TENSOR_ARM : VkObjectType = 1000460000;
    // Provided by VK_ARM_tensors
    pub const VK_OBJECT_TYPE_TENSOR_VIEW_ARM : VkObjectType = 1000460001;
    // Provided by VK_NV_optical_flow
    pub const VK_OBJECT_TYPE_OPTICAL_FLOW_SESSION_NV : VkObjectType = 1000464000;
    // Provided by VK_EXT_shader_object
    pub const VK_OBJECT_TYPE_SHADER_EXT : VkObjectType = 1000482000;
    // Provided by VK_KHR_pipeline_binary
    pub const VK_OBJECT_TYPE_PIPELINE_BINARY_KHR : VkObjectType = 1000483000;
    // Provided by VK_ARM_data_graph
    pub const VK_OBJECT_TYPE_DATA_GRAPH_PIPELINE_SESSION_ARM : VkObjectType = 1000507000;
    // Provided by VK_NV_external_compute_queue
    pub const VK_OBJECT_TYPE_EXTERNAL_COMPUTE_QUEUE_NV : VkObjectType = 1000556000;
    // Provided by VK_EXT_device_generated_commands
    pub const VK_OBJECT_TYPE_INDIRECT_COMMANDS_LAYOUT_EXT : VkObjectType = 1000572000;
    // Provided by VK_EXT_device_generated_commands
    pub const VK_OBJECT_TYPE_INDIRECT_EXECUTION_SET_EXT : VkObjectType = 1000572001;
    // Provided by VK_ARM_shader_instrumentation
    pub const VK_OBJECT_TYPE_SHADER_INSTRUMENTATION_ARM : VkObjectType = 1000607000;
    // Provided by VK_KHR_descriptor_update_template
    pub const VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_KHR : VkObjectType = VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE;
    // Provided by VK_KHR_sampler_ycbcr_conversion
    pub const VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_KHR : VkObjectType = VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION;
    // Provided by VK_EXT_private_data
    pub const VK_OBJECT_TYPE_PRIVATE_DATA_SLOT_EXT : VkObjectType = VK_OBJECT_TYPE_PRIVATE_DATA_SLOT;
}







// Provided by VK_VERSION_1_0
/*#[repr(i32)]
pub enum VkResult {
    VK_SUCCESS = VK_SUCCESS,
    VK_NOT_READY = VK_NOT_READY,
    VK_TIMEOUT = VK_TIMEOUT,
    VK_EVENT_SET = VK_EVENT_SET,
    VK_EVENT_RESET = VK_EVENT_RESET,
    VK_INCOMPLETE = VK_INCOMPLETE,
    VK_ERROR_OUT_OF_HOST_MEMORY = VK_ERROR_OUT_OF_HOST_MEMORY,
    VK_ERROR_OUT_OF_DEVICE_MEMORY = VK_ERROR_OUT_OF_DEVICE_MEMORY,
    VK_ERROR_INITIALIZATION_FAILED = VK_ERROR_INITIALIZATION_FAILED,
    VK_ERROR_DEVICE_LOST = VK_ERROR_DEVICE_LOST,
    VK_ERROR_MEMORY_MAP_FAILED = VK_ERROR_MEMORY_MAP_FAILED,
    VK_ERROR_LAYER_NOT_PRESENT = VK_ERROR_LAYER_NOT_PRESENT,
    VK_ERROR_EXTENSION_NOT_PRESENT = VK_ERROR_EXTENSION_NOT_PRESENT,
    VK_ERROR_FEATURE_NOT_PRESENT = VK_ERROR_FEATURE_NOT_PRESENT,
    VK_ERROR_INCOMPATIBLE_DRIVER = VK_ERROR_INCOMPATIBLE_DRIVER,
    VK_ERROR_TOO_MANY_OBJECTS = VK_ERROR_TOO_MANY_OBJECTS,
    VK_ERROR_FORMAT_NOT_SUPPORTED = VK_ERROR_FORMAT_NOT_SUPPORTED,
    VK_ERROR_FRAGMENTED_POOL = VK_ERROR_FRAGMENTED_POOL,
    VK_ERROR_UNKNOWN = VK_ERROR_UNKNOWN,
    // Provided by VK_VERSION_1_0
    VK_ERROR_VALIDATION_FAILED = VK_ERROR_VALIDATION_FAILED,
    // Provided by VK_VERSION_1_1
    VK_ERROR_OUT_OF_POOL_MEMORY = VK_ERROR_OUT_OF_POOL_MEMORY,
    // Provided by VK_VERSION_1_1
    VK_ERROR_INVALID_EXTERNAL_HANDLE = VK_ERROR_INVALID_EXTERNAL_HANDLE,
    // Provided by VK_VERSION_1_2
    VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS = VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
    // Provided by VK_VERSION_1_2
    VK_ERROR_FRAGMENTATION = VK_ERROR_FRAGMENTATION,
    // Provided by VK_VERSION_1_3
    VK_PIPELINE_COMPILE_REQUIRED = VK_PIPELINE_COMPILE_REQUIRED,
    // Provided by VK_VERSION_1_4
    VK_ERROR_NOT_PERMITTED = VK_ERROR_NOT_PERMITTED,
    // Provided by VK_KHR_surface
    VK_ERROR_SURFACE_LOST_KHR = VK_ERROR_SURFACE_LOST_KHR,
    // Provided by VK_KHR_surface
    VK_ERROR_NATIVE_WINDOW_IN_USE_KHR = VK_ERROR_NATIVE_WINDOW_IN_USE_KHR,
    // Provided by VK_KHR_swapchain
    VK_SUBOPTIMAL_KHR = VK_SUBOPTIMAL_KHR,
    // Provided by VK_KHR_swapchain
    VK_ERROR_OUT_OF_DATE_KHR = VK_ERROR_OUT_OF_DATE_KHR,
    // Provided by VK_KHR_display_swapchain
    VK_ERROR_INCOMPATIBLE_DISPLAY_KHR = VK_ERROR_INCOMPATIBLE_DISPLAY_KHR,
    // Provided by VK_NV_glsl_shader
    VK_ERROR_INVALID_SHADER_NV = VK_ERROR_INVALID_SHADER_NV,
    // Provided by VK_KHR_video_queue
    VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR = VK_ERROR_IMAGE_USAGE_NOT_SUPPORTED_KHR,
    // Provided by VK_KHR_video_queue
    VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR = VK_ERROR_VIDEO_PICTURE_LAYOUT_NOT_SUPPORTED_KHR,
    // Provided by VK_KHR_video_queue
    VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR = VK_ERROR_VIDEO_PROFILE_OPERATION_NOT_SUPPORTED_KHR,
    // Provided by VK_KHR_video_queue
    VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR = VK_ERROR_VIDEO_PROFILE_FORMAT_NOT_SUPPORTED_KHR,
    // Provided by VK_KHR_video_queue
    VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR = VK_ERROR_VIDEO_PROFILE_CODEC_NOT_SUPPORTED_KHR,
    // Provided by VK_KHR_video_queue
    VK_ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR = VK_ERROR_VIDEO_STD_VERSION_NOT_SUPPORTED_KHR,
    // Provided by VK_EXT_image_drm_format_modifier
    VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT = VK_ERROR_INVALID_DRM_FORMAT_MODIFIER_PLANE_LAYOUT_EXT,
    // Provided by VK_EXT_present_timing
    VK_ERROR_PRESENT_TIMING_QUEUE_FULL_EXT = VK_ERROR_PRESENT_TIMING_QUEUE_FULL_EXT,
    // Provided by VK_EXT_full_screen_exclusive
    VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT = VK_ERROR_FULL_SCREEN_EXCLUSIVE_MODE_LOST_EXT,
    // Provided by VK_KHR_deferred_host_operations
    VK_THREAD_IDLE_KHR = VK_THREAD_IDLE_KHR,
    // Provided by VK_KHR_deferred_host_operations
    VK_THREAD_DONE_KHR = VK_THREAD_DONE_KHR,
    // Provided by VK_KHR_deferred_host_operations
    VK_OPERATION_DEFERRED_KHR = VK_OPERATION_DEFERRED_KHR,
    // Provided by VK_KHR_deferred_host_operations
    VK_OPERATION_NOT_DEFERRED_KHR = VK_OPERATION_NOT_DEFERRED_KHR,
    // Provided by VK_KHR_video_encode_queue
    VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR = VK_ERROR_INVALID_VIDEO_STD_PARAMETERS_KHR,
    // Provided by VK_EXT_image_compression_control
    VK_ERROR_COMPRESSION_EXHAUSTED_EXT = VK_ERROR_COMPRESSION_EXHAUSTED_EXT,
    // Provided by VK_EXT_shader_object
    VK_INCOMPATIBLE_SHADER_BINARY_EXT = VK_INCOMPATIBLE_SHADER_BINARY_EXT,
    // Provided by VK_KHR_pipeline_binary
    VK_PIPELINE_BINARY_MISSING_KHR = VK_PIPELINE_BINARY_MISSING_KHR,
    // Provided by VK_KHR_pipeline_binary
    VK_ERROR_NOT_ENOUGH_SPACE_KHR = VK_ERROR_NOT_ENOUGH_SPACE_KHR,
    // Provided by VK_EXT_debug_report
    //VK_ERROR_VALIDATION_FAILED_EXT = VK_ERROR_VALIDATION_FAILED,
    // Provided by VK_KHR_maintenance1
    //VK_ERROR_OUT_OF_POOL_MEMORY_KHR = VK_ERROR_OUT_OF_POOL_MEMORY,
    // Provided by VK_KHR_external_memory
    //VK_ERROR_INVALID_EXTERNAL_HANDLE_KHR = VK_ERROR_INVALID_EXTERNAL_HANDLE,
    // Provided by VK_EXT_descriptor_indexing
    //VK_ERROR_FRAGMENTATION_EXT = VK_ERROR_FRAGMENTATION,
    // Provided by VK_EXT_global_priority
    //VK_ERROR_NOT_PERMITTED_EXT = VK_ERROR_NOT_PERMITTED,
    // Provided by VK_KHR_global_priority
    //VK_ERROR_NOT_PERMITTED_KHR = VK_ERROR_NOT_PERMITTED,
    // Provided by VK_EXT_buffer_device_address
    //VK_ERROR_INVALID_DEVICE_ADDRESS_EXT = VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
    // Provided by VK_KHR_buffer_device_address
    //VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS_KHR = VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS,
    // Provided by VK_EXT_pipeline_creation_cache_control
    //VK_PIPELINE_COMPILE_REQUIRED_EXT = VK_PIPELINE_COMPILE_REQUIRED,
    // Provided by VK_EXT_pipeline_creation_cache_control
    //VK_ERROR_PIPELINE_COMPILE_REQUIRED_EXT = VK_PIPELINE_COMPILE_REQUIRED,
    // Provided by VK_EXT_shader_object
    // VK_ERROR_INCOMPATIBLE_SHADER_BINARY_EXT is a legacy alias
    //VK_ERROR_INCOMPATIBLE_SHADER_BINARY_EXT = VK_INCOMPATIBLE_SHADER_BINARY_EXT,
}*/
