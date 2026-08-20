pub type VkRayTracingLssPrimitiveEndCapsModeNV = i32;
pub mod VkRayTracingLssPrimitiveEndCapsModeNVValue {
    use crate::VkRayTracingLssPrimitiveEndCapsModeNV;
    pub const VK_RAY_TRACING_LSS_PRIMITIVE_END_CAPS_MODE_NONE_NV: VkRayTracingLssPrimitiveEndCapsModeNV = 0;
    pub const VK_RAY_TRACING_LSS_PRIMITIVE_END_CAPS_MODE_CHAINED_NV: VkRayTracingLssPrimitiveEndCapsModeNV = 1;
}

pub type VkAccelerationStructureMotionInstanceTypeNV = i32;
pub mod VkAccelerationStructureMotionInstanceTypeNVValue {
    use crate::VkAccelerationStructureMotionInstanceTypeNV;
    pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_STATIC_NV: VkAccelerationStructureMotionInstanceTypeNV = 0;
    pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_MATRIX_MOTION_NV: VkAccelerationStructureMotionInstanceTypeNV = 1;
    pub const VK_ACCELERATION_STRUCTURE_MOTION_INSTANCE_TYPE_SRT_MOTION_NV: VkAccelerationStructureMotionInstanceTypeNV = 2;
}

pub type VkDebugReportObjectTypeEXT = i32;
pub mod VkDebugReportObjectTypeEXTValue {
    use crate::VkDebugReportObjectTypeEXT;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_UNKNOWN_EXT: VkDebugReportObjectTypeEXT = 0;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_INSTANCE_EXT: VkDebugReportObjectTypeEXT = 1;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PHYSICAL_DEVICE_EXT: VkDebugReportObjectTypeEXT = 2;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_EXT: VkDebugReportObjectTypeEXT = 3;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUEUE_EXT: VkDebugReportObjectTypeEXT = 4;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SEMAPHORE_EXT: VkDebugReportObjectTypeEXT = 5;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_BUFFER_EXT: VkDebugReportObjectTypeEXT = 6;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_FENCE_EXT: VkDebugReportObjectTypeEXT = 7;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEVICE_MEMORY_EXT: VkDebugReportObjectTypeEXT = 8;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_EXT: VkDebugReportObjectTypeEXT = 9;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_EXT: VkDebugReportObjectTypeEXT = 10;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_EVENT_EXT: VkDebugReportObjectTypeEXT = 11;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_QUERY_POOL_EXT: VkDebugReportObjectTypeEXT = 12;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_BUFFER_VIEW_EXT: VkDebugReportObjectTypeEXT = 13;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_IMAGE_VIEW_EXT: VkDebugReportObjectTypeEXT = 14;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SHADER_MODULE_EXT: VkDebugReportObjectTypeEXT = 15;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_CACHE_EXT: VkDebugReportObjectTypeEXT = 16;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_LAYOUT_EXT: VkDebugReportObjectTypeEXT = 17;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_RENDER_PASS_EXT: VkDebugReportObjectTypeEXT = 18;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_PIPELINE_EXT: VkDebugReportObjectTypeEXT = 19;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT_EXT: VkDebugReportObjectTypeEXT = 20;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_EXT: VkDebugReportObjectTypeEXT = 21;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_POOL_EXT: VkDebugReportObjectTypeEXT = 22;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_SET_EXT: VkDebugReportObjectTypeEXT = 23;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_FRAMEBUFFER_EXT: VkDebugReportObjectTypeEXT = 24;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_COMMAND_POOL_EXT: VkDebugReportObjectTypeEXT = 25;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SURFACE_KHR_EXT: VkDebugReportObjectTypeEXT = 26;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SWAPCHAIN_KHR_EXT: VkDebugReportObjectTypeEXT = 27;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT: VkDebugReportObjectTypeEXT = 28;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_EXT: VkDebugReportObjectTypeEXT = VK_DEBUG_REPORT_OBJECT_TYPE_DEBUG_REPORT_CALLBACK_EXT_EXT;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_KHR_EXT: VkDebugReportObjectTypeEXT = 29;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DISPLAY_MODE_KHR_EXT: VkDebugReportObjectTypeEXT = 30;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT: VkDebugReportObjectTypeEXT = 33;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT: VkDebugReportObjectTypeEXT = VK_DEBUG_REPORT_OBJECT_TYPE_VALIDATION_CACHE_EXT_EXT;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION_EXT: VkDebugReportObjectTypeEXT = 1000156000;
    pub const VK_DEBUG_REPORT_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_EXT: VkDebugReportObjectTypeEXT = 1000085000;
}

pub type VkDefaultVertexAttributeValueKHR = i32;
pub mod VkDefaultVertexAttributeValueKHRValue {
    use crate::VkDefaultVertexAttributeValueKHR;
    pub const VK_DEFAULT_VERTEX_ATTRIBUTE_VALUE_ZERO_ZERO_ZERO_ZERO_KHR: VkDefaultVertexAttributeValueKHR = 0;
    pub const VK_DEFAULT_VERTEX_ATTRIBUTE_VALUE_ZERO_ZERO_ZERO_ONE_KHR: VkDefaultVertexAttributeValueKHR = 1;
}

pub type VkTessellationDomainOrigin = i32;
pub mod VkTessellationDomainOriginValue {
    use crate::VkTessellationDomainOrigin;
    pub const VK_TESSELLATION_DOMAIN_ORIGIN_UPPER_LEFT: VkTessellationDomainOrigin = 0;
    pub const VK_TESSELLATION_DOMAIN_ORIGIN_LOWER_LEFT: VkTessellationDomainOrigin = 1;
}

pub type VkGeometryTypeKHR = i32;
pub mod VkGeometryTypeKHRValue {
    use crate::VkGeometryTypeKHR;
    pub const VK_GEOMETRY_TYPE_TRIANGLES_KHR: VkGeometryTypeKHR = 0;
    pub const VK_GEOMETRY_TYPE_AABBS_KHR: VkGeometryTypeKHR = 1;
    pub const VK_GEOMETRY_TYPE_INSTANCES_KHR: VkGeometryTypeKHR = 2;
}

pub type VkTimeDomainKHR = i32;
pub mod VkTimeDomainKHRValue {
    use crate::VkTimeDomainKHR;
    pub const VK_TIME_DOMAIN_DEVICE_KHR: VkTimeDomainKHR = 0;
    pub const VK_TIME_DOMAIN_CLOCK_MONOTONIC_KHR: VkTimeDomainKHR = 1;
    pub const VK_TIME_DOMAIN_CLOCK_MONOTONIC_RAW_KHR: VkTimeDomainKHR = 2;
    pub const VK_TIME_DOMAIN_QUERY_PERFORMANCE_COUNTER_KHR: VkTimeDomainKHR = 3;
}

pub type VkShaderInfoTypeAMD = i32;
pub mod VkShaderInfoTypeAMDValue {
    use crate::VkShaderInfoTypeAMD;
    pub const VK_SHADER_INFO_TYPE_STATISTICS_AMD: VkShaderInfoTypeAMD = 0;
    pub const VK_SHADER_INFO_TYPE_BINARY_AMD: VkShaderInfoTypeAMD = 1;
    pub const VK_SHADER_INFO_TYPE_DISASSEMBLY_AMD: VkShaderInfoTypeAMD = 2;
}

pub type VkDescriptorType = i32;
pub mod VkDescriptorTypeValue {
    use crate::VkDescriptorType;
    pub const VK_DESCRIPTOR_TYPE_SAMPLER: VkDescriptorType = 0;
    pub const VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER: VkDescriptorType = 1;
    pub const VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE: VkDescriptorType = 2;
    pub const VK_DESCRIPTOR_TYPE_STORAGE_IMAGE: VkDescriptorType = 3;
    pub const VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER: VkDescriptorType = 4;
    pub const VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER: VkDescriptorType = 5;
    pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER: VkDescriptorType = 6;
    pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER: VkDescriptorType = 7;
    pub const VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC: VkDescriptorType = 8;
    pub const VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC: VkDescriptorType = 9;
    pub const VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT: VkDescriptorType = 10;
    pub const VK_DESCRIPTOR_TYPE_INLINE_UNIFORM_BLOCK: VkDescriptorType = 1000138000;
    pub const VK_DESCRIPTOR_TYPE_MUTABLE_EXT: VkDescriptorType = 1000351000;
}

pub type VkFaultQueryBehavior = i32;
pub mod VkFaultQueryBehaviorValue {
    use crate::VkFaultQueryBehavior;
    pub const VK_FAULT_QUERY_BEHAVIOR_GET_AND_CLEAR_ALL_FAULTS: VkFaultQueryBehavior = 0;
}

pub type VkPipelineRobustnessImageBehavior = i32;
pub mod VkPipelineRobustnessImageBehaviorValue {
    use crate::VkPipelineRobustnessImageBehavior;
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DEVICE_DEFAULT: VkPipelineRobustnessImageBehavior = 0;
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_DISABLED: VkPipelineRobustnessImageBehavior = 1;
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS: VkPipelineRobustnessImageBehavior = 2;
    pub const VK_PIPELINE_ROBUSTNESS_IMAGE_BEHAVIOR_ROBUST_IMAGE_ACCESS_2: VkPipelineRobustnessImageBehavior = 3;
}

pub type VkSystemAllocationScope = i32;
pub mod VkSystemAllocationScopeValue {
    use crate::VkSystemAllocationScope;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_COMMAND: VkSystemAllocationScope = 0;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_OBJECT: VkSystemAllocationScope = 1;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_CACHE: VkSystemAllocationScope = 2;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_DEVICE: VkSystemAllocationScope = 3;
    pub const VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE: VkSystemAllocationScope = 4;
}

pub type VkSamplerYcbcrRange = i32;
pub mod VkSamplerYcbcrRangeValue {
    use crate::VkSamplerYcbcrRange;
    pub const VK_SAMPLER_YCBCR_RANGE_ITU_FULL: VkSamplerYcbcrRange = 0;
    pub const VK_SAMPLER_YCBCR_RANGE_ITU_NARROW: VkSamplerYcbcrRange = 1;
}

pub type VkOpacityMicromapSpecialIndexKHR = i32;
pub mod VkOpacityMicromapSpecialIndexKHRValue {
    use crate::VkOpacityMicromapSpecialIndexKHR;
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_TRANSPARENT_KHR: VkOpacityMicromapSpecialIndexKHR = -1;
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_OPAQUE_KHR: VkOpacityMicromapSpecialIndexKHR = -2;
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_TRANSPARENT_KHR: VkOpacityMicromapSpecialIndexKHR = -3;
    pub const VK_OPACITY_MICROMAP_SPECIAL_INDEX_FULLY_UNKNOWN_OPAQUE_KHR: VkOpacityMicromapSpecialIndexKHR = -4;
}

pub type VkImageViewType = i32;
pub mod VkImageViewTypeValue {
    use crate::VkImageViewType;
    pub const VK_IMAGE_VIEW_TYPE_1D: VkImageViewType = 0;
    pub const VK_IMAGE_VIEW_TYPE_2D: VkImageViewType = 1;
    pub const VK_IMAGE_VIEW_TYPE_3D: VkImageViewType = 2;
    pub const VK_IMAGE_VIEW_TYPE_CUBE: VkImageViewType = 3;
    pub const VK_IMAGE_VIEW_TYPE_1D_ARRAY: VkImageViewType = 4;
    pub const VK_IMAGE_VIEW_TYPE_2D_ARRAY: VkImageViewType = 5;
    pub const VK_IMAGE_VIEW_TYPE_CUBE_ARRAY: VkImageViewType = 6;
}

pub type VkDepthClampModeEXT = i32;
pub mod VkDepthClampModeEXTValue {
    use crate::VkDepthClampModeEXT;
    pub const VK_DEPTH_CLAMP_MODE_VIEWPORT_RANGE_EXT: VkDepthClampModeEXT = 0;
    pub const VK_DEPTH_CLAMP_MODE_USER_DEFINED_RANGE_EXT: VkDepthClampModeEXT = 1;
}

pub type VkInternalAllocationType = i32;
pub mod VkInternalAllocationTypeValue {
    use crate::VkInternalAllocationType;
    pub const VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE: VkInternalAllocationType = 0;
}

pub type VkDisplayEventTypeEXT = i32;
pub mod VkDisplayEventTypeEXTValue {
    use crate::VkDisplayEventTypeEXT;
    pub const VK_DISPLAY_EVENT_TYPE_FIRST_PIXEL_OUT_EXT: VkDisplayEventTypeEXT = 0;
}

pub type VkClusterAccelerationStructureOpModeNV = i32;
pub mod VkClusterAccelerationStructureOpModeNVValue {
    use crate::VkClusterAccelerationStructureOpModeNV;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_IMPLICIT_DESTINATIONS_NV: VkClusterAccelerationStructureOpModeNV = 0;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_EXPLICIT_DESTINATIONS_NV: VkClusterAccelerationStructureOpModeNV = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_COMPUTE_SIZES_NV: VkClusterAccelerationStructureOpModeNV = 2;
}

pub type VkPerformanceCounterStorageKHR = i32;
pub mod VkPerformanceCounterStorageKHRValue {
    use crate::VkPerformanceCounterStorageKHR;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_INT32_KHR: VkPerformanceCounterStorageKHR = 0;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_INT64_KHR: VkPerformanceCounterStorageKHR = 1;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_UINT32_KHR: VkPerformanceCounterStorageKHR = 2;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_UINT64_KHR: VkPerformanceCounterStorageKHR = 3;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_FLOAT32_KHR: VkPerformanceCounterStorageKHR = 4;
    pub const VK_PERFORMANCE_COUNTER_STORAGE_FLOAT64_KHR: VkPerformanceCounterStorageKHR = 5;
}

pub type VkSharingMode = i32;
pub mod VkSharingModeValue {
    use crate::VkSharingMode;
    pub const VK_SHARING_MODE_EXCLUSIVE: VkSharingMode = 0;
    pub const VK_SHARING_MODE_CONCURRENT: VkSharingMode = 1;
}

pub type VkScopeKHR = i32;
pub mod VkScopeKHRValue {
    use crate::VkScopeKHR;
    pub const VK_SCOPE_DEVICE_KHR: VkScopeKHR = 1;
    pub const VK_SCOPE_WORKGROUP_KHR: VkScopeKHR = 2;
    pub const VK_SCOPE_SUBGROUP_KHR: VkScopeKHR = 3;
    pub const VK_SCOPE_QUEUE_FAMILY_KHR: VkScopeKHR = 5;
}

pub type VkDataGraphPipelineSessionBindPointARM = i32;
pub mod VkDataGraphPipelineSessionBindPointARMValue {
    use crate::VkDataGraphPipelineSessionBindPointARM;
    pub const VK_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_TRANSIENT_ARM: VkDataGraphPipelineSessionBindPointARM = 0;
}

pub type VkOutOfBandQueueTypeNV = i32;
pub mod VkOutOfBandQueueTypeNVValue {
    use crate::VkOutOfBandQueueTypeNV;
    pub const VK_OUT_OF_BAND_QUEUE_TYPE_RENDER_NV: VkOutOfBandQueueTypeNV = 0;
    pub const VK_OUT_OF_BAND_QUEUE_TYPE_PRESENT_NV: VkOutOfBandQueueTypeNV = 1;
}

pub type VkQueueGlobalPriority = i32;
pub mod VkQueueGlobalPriorityValue {
    use crate::VkQueueGlobalPriority;
    pub const VK_QUEUE_GLOBAL_PRIORITY_LOW: VkQueueGlobalPriority = 128;
    pub const VK_QUEUE_GLOBAL_PRIORITY_MEDIUM: VkQueueGlobalPriority = 256;
    pub const VK_QUEUE_GLOBAL_PRIORITY_HIGH: VkQueueGlobalPriority = 512;
    pub const VK_QUEUE_GLOBAL_PRIORITY_REALTIME: VkQueueGlobalPriority = 1024;
}

pub type VkPerformanceCounterScopeKHR = i32;
pub mod VkPerformanceCounterScopeKHRValue {
    use crate::VkPerformanceCounterScopeKHR;
    pub const VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR: VkPerformanceCounterScopeKHR = 0;
    pub const VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR: VkPerformanceCounterScopeKHR = 1;
    pub const VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR: VkPerformanceCounterScopeKHR = 2;
    pub const VK_QUERY_SCOPE_COMMAND_BUFFER_KHR: VkPerformanceCounterScopeKHR = VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_BUFFER_KHR;
    pub const VK_QUERY_SCOPE_RENDER_PASS_KHR: VkPerformanceCounterScopeKHR = VK_PERFORMANCE_COUNTER_SCOPE_RENDER_PASS_KHR;
    pub const VK_QUERY_SCOPE_COMMAND_KHR: VkPerformanceCounterScopeKHR = VK_PERFORMANCE_COUNTER_SCOPE_COMMAND_KHR;
}

pub type VkPhysicalDeviceType = i32;
pub mod VkPhysicalDeviceTypeValue {
    use crate::VkPhysicalDeviceType;
    pub const VK_PHYSICAL_DEVICE_TYPE_OTHER: VkPhysicalDeviceType = 0;
    pub const VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU: VkPhysicalDeviceType = 1;
    pub const VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU: VkPhysicalDeviceType = 2;
    pub const VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU: VkPhysicalDeviceType = 3;
    pub const VK_PHYSICAL_DEVICE_TYPE_CPU: VkPhysicalDeviceType = 4;
}

pub type VkValidationCheckEXT = i32;
pub mod VkValidationCheckEXTValue {
    use crate::VkValidationCheckEXT;
    pub const VK_VALIDATION_CHECK_ALL_EXT: VkValidationCheckEXT = 0;
    pub const VK_VALIDATION_CHECK_SHADERS_EXT: VkValidationCheckEXT = 1;
}

pub type VkDataGraphOpticalFlowPerformanceLevelARM = i32;
pub mod VkDataGraphOpticalFlowPerformanceLevelARMValue {
    use crate::VkDataGraphOpticalFlowPerformanceLevelARM;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_UNKNOWN_ARM: VkDataGraphOpticalFlowPerformanceLevelARM = 0;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_SLOW_ARM: VkDataGraphOpticalFlowPerformanceLevelARM = 1;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_MEDIUM_ARM: VkDataGraphOpticalFlowPerformanceLevelARM = 2;
    pub const VK_DATA_GRAPH_OPTICAL_FLOW_PERFORMANCE_LEVEL_FAST_ARM: VkDataGraphOpticalFlowPerformanceLevelARM = 3;
}

pub type VkLatencyMarkerNV = i32;
pub mod VkLatencyMarkerNVValue {
    use crate::VkLatencyMarkerNV;
    pub const VK_LATENCY_MARKER_SIMULATION_START_NV: VkLatencyMarkerNV = 0;
    pub const VK_LATENCY_MARKER_SIMULATION_END_NV: VkLatencyMarkerNV = 1;
    pub const VK_LATENCY_MARKER_RENDERSUBMIT_START_NV: VkLatencyMarkerNV = 2;
    pub const VK_LATENCY_MARKER_RENDERSUBMIT_END_NV: VkLatencyMarkerNV = 3;
    pub const VK_LATENCY_MARKER_PRESENT_START_NV: VkLatencyMarkerNV = 4;
    pub const VK_LATENCY_MARKER_PRESENT_END_NV: VkLatencyMarkerNV = 5;
    pub const VK_LATENCY_MARKER_INPUT_SAMPLE_NV: VkLatencyMarkerNV = 6;
    pub const VK_LATENCY_MARKER_TRIGGER_FLASH_NV: VkLatencyMarkerNV = 7;
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_RENDERSUBMIT_START_NV: VkLatencyMarkerNV = 8;
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_RENDERSUBMIT_END_NV: VkLatencyMarkerNV = 9;
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_PRESENT_START_NV: VkLatencyMarkerNV = 10;
    pub const VK_LATENCY_MARKER_OUT_OF_BAND_PRESENT_END_NV: VkLatencyMarkerNV = 11;
}

pub type VkFragmentShadingRateTypeNV = i32;
pub mod VkFragmentShadingRateTypeNVValue {
    use crate::VkFragmentShadingRateTypeNV;
    pub const VK_FRAGMENT_SHADING_RATE_TYPE_FRAGMENT_SIZE_NV: VkFragmentShadingRateTypeNV = 0;
    pub const VK_FRAGMENT_SHADING_RATE_TYPE_ENUMS_NV: VkFragmentShadingRateTypeNV = 1;
}

pub type VkLayerSettingTypeEXT = i32;
pub mod VkLayerSettingTypeEXTValue {
    use crate::VkLayerSettingTypeEXT;
    pub const VK_LAYER_SETTING_TYPE_BOOL32_EXT: VkLayerSettingTypeEXT = 0;
    pub const VK_LAYER_SETTING_TYPE_INT32_EXT: VkLayerSettingTypeEXT = 1;
    pub const VK_LAYER_SETTING_TYPE_INT64_EXT: VkLayerSettingTypeEXT = 2;
    pub const VK_LAYER_SETTING_TYPE_UINT32_EXT: VkLayerSettingTypeEXT = 3;
    pub const VK_LAYER_SETTING_TYPE_UINT64_EXT: VkLayerSettingTypeEXT = 4;
    pub const VK_LAYER_SETTING_TYPE_FLOAT32_EXT: VkLayerSettingTypeEXT = 5;
    pub const VK_LAYER_SETTING_TYPE_FLOAT64_EXT: VkLayerSettingTypeEXT = 6;
    pub const VK_LAYER_SETTING_TYPE_STRING_EXT: VkLayerSettingTypeEXT = 7;
}

pub type VkIndirectCommandsTokenTypeEXT = i32;
pub mod VkIndirectCommandsTokenTypeEXTValue {
    use crate::VkIndirectCommandsTokenTypeEXT;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_EXECUTION_SET_EXT: VkIndirectCommandsTokenTypeEXT = 0;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_EXT: VkIndirectCommandsTokenTypeEXT = 1;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_SEQUENCE_INDEX_EXT: VkIndirectCommandsTokenTypeEXT = 2;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_EXT: VkIndirectCommandsTokenTypeEXT = 3;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_EXT: VkIndirectCommandsTokenTypeEXT = 4;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_EXT: VkIndirectCommandsTokenTypeEXT = 5;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_EXT: VkIndirectCommandsTokenTypeEXT = 6;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_COUNT_EXT: VkIndirectCommandsTokenTypeEXT = 7;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_COUNT_EXT: VkIndirectCommandsTokenTypeEXT = 8;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DISPATCH_EXT: VkIndirectCommandsTokenTypeEXT = 9;
}

pub type VkStencilOp = i32;
pub mod VkStencilOpValue {
    use crate::VkStencilOp;
    pub const VK_STENCIL_OP_KEEP: VkStencilOp = 0;
    pub const VK_STENCIL_OP_ZERO: VkStencilOp = 1;
    pub const VK_STENCIL_OP_REPLACE: VkStencilOp = 2;
    pub const VK_STENCIL_OP_INCREMENT_AND_CLAMP: VkStencilOp = 3;
    pub const VK_STENCIL_OP_DECREMENT_AND_CLAMP: VkStencilOp = 4;
    pub const VK_STENCIL_OP_INVERT: VkStencilOp = 5;
    pub const VK_STENCIL_OP_INCREMENT_AND_WRAP: VkStencilOp = 6;
    pub const VK_STENCIL_OP_DECREMENT_AND_WRAP: VkStencilOp = 7;
}

pub type VkGpaSampleTypeAMD = i32;
pub mod VkGpaSampleTypeAMDValue {
    use crate::VkGpaSampleTypeAMD;
    pub const VK_GPA_SAMPLE_TYPE_CUMULATIVE_AMD: VkGpaSampleTypeAMD = 0;
    pub const VK_GPA_SAMPLE_TYPE_TRACE_AMD: VkGpaSampleTypeAMD = 1;
    pub const VK_GPA_SAMPLE_TYPE_TIMING_AMD: VkGpaSampleTypeAMD = 2;
}

pub type VkFaultType = i32;
pub mod VkFaultTypeValue {
    use crate::VkFaultType;
    pub const VK_FAULT_TYPE_INVALID: VkFaultType = 0;
    pub const VK_FAULT_TYPE_UNASSIGNED: VkFaultType = 1;
    pub const VK_FAULT_TYPE_IMPLEMENTATION: VkFaultType = 2;
    pub const VK_FAULT_TYPE_SYSTEM: VkFaultType = 3;
    pub const VK_FAULT_TYPE_PHYSICAL_DEVICE: VkFaultType = 4;
    pub const VK_FAULT_TYPE_COMMAND_BUFFER_FULL: VkFaultType = 5;
    pub const VK_FAULT_TYPE_INVALID_API_USAGE: VkFaultType = 6;
}

pub type VkPrimitiveTopology = i32;
pub mod VkPrimitiveTopologyValue {
    use crate::VkPrimitiveTopology;
    pub const VK_PRIMITIVE_TOPOLOGY_POINT_LIST: VkPrimitiveTopology = 0;
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST: VkPrimitiveTopology = 1;
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP: VkPrimitiveTopology = 2;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST: VkPrimitiveTopology = 3;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP: VkPrimitiveTopology = 4;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN: VkPrimitiveTopology = 5;
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY: VkPrimitiveTopology = 6;
    pub const VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology = 7;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY: VkPrimitiveTopology = 8;
    pub const VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY: VkPrimitiveTopology = 9;
    pub const VK_PRIMITIVE_TOPOLOGY_PATCH_LIST: VkPrimitiveTopology = 10;
}

pub type VkFilter = i32;
pub mod VkFilterValue {
    use crate::VkFilter;
    pub const VK_FILTER_NEAREST: VkFilter = 0;
    pub const VK_FILTER_LINEAR: VkFilter = 1;
    pub const VK_FILTER_CUBIC_EXT: VkFilter = 1000015000;
}

pub type VkValidationFeatureEnableEXT = i32;
pub mod VkValidationFeatureEnableEXTValue {
    use crate::VkValidationFeatureEnableEXT;
    pub const VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_EXT: VkValidationFeatureEnableEXT = 0;
    pub const VK_VALIDATION_FEATURE_ENABLE_GPU_ASSISTED_RESERVE_BINDING_SLOT_EXT: VkValidationFeatureEnableEXT = 1;
    pub const VK_VALIDATION_FEATURE_ENABLE_BEST_PRACTICES_EXT: VkValidationFeatureEnableEXT = 2;
    pub const VK_VALIDATION_FEATURE_ENABLE_DEBUG_PRINTF_EXT: VkValidationFeatureEnableEXT = 3;
    pub const VK_VALIDATION_FEATURE_ENABLE_SYNCHRONIZATION_VALIDATION_EXT: VkValidationFeatureEnableEXT = 4;
}

pub type VkVendorId = i32;
pub mod VkVendorIdValue {
    use crate::VkVendorId;
    pub const VK_VENDOR_ID_KHRONOS: VkVendorId = 0x10000;
    pub const VK_VENDOR_ID_VIV: VkVendorId = 0x10001;
    pub const VK_VENDOR_ID_VSI: VkVendorId = 0x10002;
    pub const VK_VENDOR_ID_KAZAN: VkVendorId = 0x10003;
    pub const VK_VENDOR_ID_CODEPLAY: VkVendorId = 0x10004;
    pub const VK_VENDOR_ID_MESA: VkVendorId = 0x10005;
    pub const VK_VENDOR_ID_POCL: VkVendorId = 0x10006;
    pub const VK_VENDOR_ID_MOBILEYE: VkVendorId = 0x10007;
    pub const VK_VENDOR_ID_APE: VkVendorId = 0x10008;
}

pub type VkCopyAccelerationStructureModeKHR = i32;
pub mod VkCopyAccelerationStructureModeKHRValue {
    use crate::VkCopyAccelerationStructureModeKHR;
    pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_CLONE_KHR: VkCopyAccelerationStructureModeKHR = 0;
    pub const VK_COPY_ACCELERATION_STRUCTURE_MODE_COMPACT_KHR: VkCopyAccelerationStructureModeKHR = 1;
}

pub type VkDeviceFaultAddressTypeKHR = i32;
pub mod VkDeviceFaultAddressTypeKHRValue {
    use crate::VkDeviceFaultAddressTypeKHR;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_NONE_KHR: VkDeviceFaultAddressTypeKHR = 0;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_READ_INVALID_KHR: VkDeviceFaultAddressTypeKHR = 1;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_WRITE_INVALID_KHR: VkDeviceFaultAddressTypeKHR = 2;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_EXECUTE_INVALID_KHR: VkDeviceFaultAddressTypeKHR = 3;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_UNKNOWN_KHR: VkDeviceFaultAddressTypeKHR = 4;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_INVALID_KHR: VkDeviceFaultAddressTypeKHR = 5;
    pub const VK_DEVICE_FAULT_ADDRESS_TYPE_INSTRUCTION_POINTER_FAULT_KHR: VkDeviceFaultAddressTypeKHR = 6;
}

pub type VkDescriptorMappingSourceEXT = i32;
pub mod VkDescriptorMappingSourceEXTValue {
    use crate::VkDescriptorMappingSourceEXT;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_CONSTANT_OFFSET_EXT: VkDescriptorMappingSourceEXT = 0;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_PUSH_INDEX_EXT: VkDescriptorMappingSourceEXT = 1;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_INDIRECT_INDEX_EXT: VkDescriptorMappingSourceEXT = 2;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_HEAP_WITH_INDIRECT_INDEX_ARRAY_EXT: VkDescriptorMappingSourceEXT = 3;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_RESOURCE_HEAP_DATA_EXT: VkDescriptorMappingSourceEXT = 4;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_PUSH_DATA_EXT: VkDescriptorMappingSourceEXT = 5;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_PUSH_ADDRESS_EXT: VkDescriptorMappingSourceEXT = 6;
    pub const VK_DESCRIPTOR_MAPPING_SOURCE_INDIRECT_ADDRESS_EXT: VkDescriptorMappingSourceEXT = 7;
}

pub type VkImageTilingControlEXT = i32;
pub mod VkImageTilingControlEXTValue {
    use crate::VkImageTilingControlEXT;
    pub const VK_IMAGE_TILING_CONTROL_DEFAULT_EXT: VkImageTilingControlEXT = 0;
    pub const VK_IMAGE_TILING_CONTROL_MIN_SIZE_EXT: VkImageTilingControlEXT = 1;
    pub const VK_IMAGE_TILING_CONTROL_MAX_PERFORMANCE_EXT: VkImageTilingControlEXT = 2;
}

pub type VkNeuralAcceleratorStatisticsModeARM = i32;
pub mod VkNeuralAcceleratorStatisticsModeARMValue {
    use crate::VkNeuralAcceleratorStatisticsModeARM;
    pub const VK_NEURAL_ACCELERATOR_STATISTICS_MODE_DISABLED_ARM: VkNeuralAcceleratorStatisticsModeARM = 0;
    pub const VK_NEURAL_ACCELERATOR_STATISTICS_MODE_STATISTICS0_ARM: VkNeuralAcceleratorStatisticsModeARM = 1;
    pub const VK_NEURAL_ACCELERATOR_STATISTICS_MODE_STATISTICS1_ARM: VkNeuralAcceleratorStatisticsModeARM = 2;
}

pub type VkCoverageReductionModeNV = i32;
pub mod VkCoverageReductionModeNVValue {
    use crate::VkCoverageReductionModeNV;
    pub const VK_COVERAGE_REDUCTION_MODE_MERGE_NV: VkCoverageReductionModeNV = 0;
    pub const VK_COVERAGE_REDUCTION_MODE_TRUNCATE_NV: VkCoverageReductionModeNV = 1;
}

pub type VkShaderCodeTypeEXT = i32;
pub mod VkShaderCodeTypeEXTValue {
    use crate::VkShaderCodeTypeEXT;
    pub const VK_SHADER_CODE_TYPE_BINARY_EXT: VkShaderCodeTypeEXT = 0;
    pub const VK_SHADER_CODE_TYPE_SPIRV_EXT: VkShaderCodeTypeEXT = 1;
}

pub type VkConservativeRasterizationModeEXT = i32;
pub mod VkConservativeRasterizationModeEXTValue {
    use crate::VkConservativeRasterizationModeEXT;
    pub const VK_CONSERVATIVE_RASTERIZATION_MODE_DISABLED_EXT: VkConservativeRasterizationModeEXT = 0;
    pub const VK_CONSERVATIVE_RASTERIZATION_MODE_OVERESTIMATE_EXT: VkConservativeRasterizationModeEXT = 1;
    pub const VK_CONSERVATIVE_RASTERIZATION_MODE_UNDERESTIMATE_EXT: VkConservativeRasterizationModeEXT = 2;
}

pub type VkIndexType = i32;
pub mod VkIndexTypeValue {
    use crate::VkIndexType;
    pub const VK_INDEX_TYPE_UINT16: VkIndexType = 0;
    pub const VK_INDEX_TYPE_UINT32: VkIndexType = 1;
    pub const VK_INDEX_TYPE_UINT8: VkIndexType = 1000265000;
    pub const VK_INDEX_TYPE_NONE_KHR: VkIndexType = 1000165000;
}

pub type VkDiscardRectangleModeEXT = i32;
pub mod VkDiscardRectangleModeEXTValue {
    use crate::VkDiscardRectangleModeEXT;
    pub const VK_DISCARD_RECTANGLE_MODE_INCLUSIVE_EXT: VkDiscardRectangleModeEXT = 0;
    pub const VK_DISCARD_RECTANGLE_MODE_EXCLUSIVE_EXT: VkDiscardRectangleModeEXT = 1;
}

pub type VkLogicOp = i32;
pub mod VkLogicOpValue {
    use crate::VkLogicOp;
    pub const VK_LOGIC_OP_CLEAR: VkLogicOp = 0;
    pub const VK_LOGIC_OP_AND: VkLogicOp = 1;
    pub const VK_LOGIC_OP_AND_REVERSE: VkLogicOp = 2;
    pub const VK_LOGIC_OP_COPY: VkLogicOp = 3;
    pub const VK_LOGIC_OP_AND_INVERTED: VkLogicOp = 4;
    pub const VK_LOGIC_OP_NO_OP: VkLogicOp = 5;
    pub const VK_LOGIC_OP_XOR: VkLogicOp = 6;
    pub const VK_LOGIC_OP_OR: VkLogicOp = 7;
    pub const VK_LOGIC_OP_NOR: VkLogicOp = 8;
    pub const VK_LOGIC_OP_EQUIVALENT: VkLogicOp = 9;
    pub const VK_LOGIC_OP_INVERT: VkLogicOp = 10;
    pub const VK_LOGIC_OP_OR_REVERSE: VkLogicOp = 11;
    pub const VK_LOGIC_OP_COPY_INVERTED: VkLogicOp = 12;
    pub const VK_LOGIC_OP_OR_INVERTED: VkLogicOp = 13;
    pub const VK_LOGIC_OP_NAND: VkLogicOp = 14;
    pub const VK_LOGIC_OP_SET: VkLogicOp = 15;
}

pub type VkPipelineBindPoint = i32;
pub mod VkPipelineBindPointValue {
    use crate::VkPipelineBindPoint;
    pub const VK_PIPELINE_BIND_POINT_GRAPHICS: VkPipelineBindPoint = 0;
    pub const VK_PIPELINE_BIND_POINT_COMPUTE: VkPipelineBindPoint = 1;
    pub const VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR: VkPipelineBindPoint = 1000165000;
    pub const VK_PIPELINE_BIND_POINT_SUBPASS_SHADING_HUAWEI: VkPipelineBindPoint = 1000369003;
}

pub type VkMemoryOverallocationBehaviorAMD = i32;
pub mod VkMemoryOverallocationBehaviorAMDValue {
    use crate::VkMemoryOverallocationBehaviorAMD;
    pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_DEFAULT_AMD: VkMemoryOverallocationBehaviorAMD = 0;
    pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_ALLOWED_AMD: VkMemoryOverallocationBehaviorAMD = 1;
    pub const VK_MEMORY_OVERALLOCATION_BEHAVIOR_DISALLOWED_AMD: VkMemoryOverallocationBehaviorAMD = 2;
}

pub type VkStructureType = i32;
pub mod VkStructureTypeValue {
    use crate::VkStructureType;
    pub const VK_STRUCTURE_TYPE_APPLICATION_INFO: VkStructureType = 0;
    pub const VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO: VkStructureType = 1;
    pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO: VkStructureType = 2;
    pub const VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO: VkStructureType = 3;
    pub const VK_STRUCTURE_TYPE_SUBMIT_INFO: VkStructureType = 4;
    pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO: VkStructureType = 5;
    pub const VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE: VkStructureType = 6;
    pub const VK_STRUCTURE_TYPE_BIND_SPARSE_INFO: VkStructureType = 7;
    pub const VK_STRUCTURE_TYPE_FENCE_CREATE_INFO: VkStructureType = 8;
    pub const VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO: VkStructureType = 9;
    pub const VK_STRUCTURE_TYPE_EVENT_CREATE_INFO: VkStructureType = 10;
    pub const VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO: VkStructureType = 11;
    pub const VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO: VkStructureType = 12;
    pub const VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO: VkStructureType = 13;
    pub const VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO: VkStructureType = 14;
    pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO: VkStructureType = 15;
    pub const VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO: VkStructureType = 16;
    pub const VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO: VkStructureType = 17;
    pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO: VkStructureType = 18;
    pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO: VkStructureType = 19;
    pub const VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO: VkStructureType = 20;
    pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO: VkStructureType = 21;
    pub const VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO: VkStructureType = 22;
    pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO: VkStructureType = 23;
    pub const VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO: VkStructureType = 24;
    pub const VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO: VkStructureType = 25;
    pub const VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO: VkStructureType = 26;
    pub const VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO: VkStructureType = 27;
    pub const VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO: VkStructureType = 28;
    pub const VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO: VkStructureType = 29;
    pub const VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO: VkStructureType = 30;
    pub const VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO: VkStructureType = 31;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO: VkStructureType = 32;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO: VkStructureType = 33;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO: VkStructureType = 34;
    pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET: VkStructureType = 35;
    pub const VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET: VkStructureType = 36;
    pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO: VkStructureType = 37;
    pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO: VkStructureType = 38;
    pub const VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO: VkStructureType = 39;
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO: VkStructureType = 40;
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO: VkStructureType = 41;
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO: VkStructureType = 42;
    pub const VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO: VkStructureType = 43;
    pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER: VkStructureType = 44;
    pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER: VkStructureType = 45;
    pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER: VkStructureType = 46;
    pub const VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO: VkStructureType = 47;
    pub const VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO: VkStructureType = 48;
    pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO: VkStructureType = 1000157000;
    pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_INFO: VkStructureType = 1000157001;
    pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_REQUIREMENTS: VkStructureType = 1000127000;
    pub const VK_STRUCTURE_TYPE_MEMORY_DEDICATED_ALLOCATE_INFO: VkStructureType = 1000127001;
    pub const VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_FLAGS_INFO: VkStructureType = 1000060000;
    pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_COMMAND_BUFFER_BEGIN_INFO: VkStructureType = 1000060004;
    pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SUBMIT_INFO: VkStructureType = 1000060005;
    pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_BIND_SPARSE_INFO: VkStructureType = 1000060006;
    pub const VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_DEVICE_GROUP_INFO: VkStructureType = 1000060013;
    pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_DEVICE_GROUP_INFO: VkStructureType = 1000060014;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GROUP_PROPERTIES: VkStructureType = 1000070000;
    pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_DEVICE_CREATE_INFO: VkStructureType = 1000070001;
    pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = 1000146000;
    pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = 1000146001;
    pub const VK_STRUCTURE_TYPE_IMAGE_SPARSE_MEMORY_REQUIREMENTS_INFO_2: VkStructureType = 1000146002;
    pub const VK_STRUCTURE_TYPE_MEMORY_REQUIREMENTS_2: VkStructureType = 1000146003;
    pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_MEMORY_REQUIREMENTS_2: VkStructureType = 1000146004;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FEATURES_2: VkStructureType = 1000059000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2: VkStructureType = 1000059001;
    pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_2: VkStructureType = 1000059002;
    pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_PROPERTIES_2: VkStructureType = 1000059003;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_FORMAT_INFO_2: VkStructureType = 1000059004;
    pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_PROPERTIES_2: VkStructureType = 1000059005;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_PROPERTIES_2: VkStructureType = 1000059006;
    pub const VK_STRUCTURE_TYPE_SPARSE_IMAGE_FORMAT_PROPERTIES_2: VkStructureType = 1000059007;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SPARSE_IMAGE_FORMAT_INFO_2: VkStructureType = 1000059008;
    pub const VK_STRUCTURE_TYPE_IMAGE_VIEW_USAGE_CREATE_INFO: VkStructureType = 1000117002;
    pub const VK_STRUCTURE_TYPE_PROTECTED_SUBMIT_INFO: VkStructureType = 1000145000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_FEATURES: VkStructureType = 1000145001;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROTECTED_MEMORY_PROPERTIES: VkStructureType = 1000145002;
    pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_INFO_2: VkStructureType = 1000145003;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_IMAGE_FORMAT_INFO: VkStructureType = 1000071000;
    pub const VK_STRUCTURE_TYPE_EXTERNAL_IMAGE_FORMAT_PROPERTIES: VkStructureType = 1000071001;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_BUFFER_INFO: VkStructureType = 1000071002;
    pub const VK_STRUCTURE_TYPE_EXTERNAL_BUFFER_PROPERTIES: VkStructureType = 1000071003;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ID_PROPERTIES: VkStructureType = 1000071004;
    pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_BUFFER_CREATE_INFO: VkStructureType = 1000072000;
    pub const VK_STRUCTURE_TYPE_EXTERNAL_MEMORY_IMAGE_CREATE_INFO: VkStructureType = 1000072001;
    pub const VK_STRUCTURE_TYPE_EXPORT_MEMORY_ALLOCATE_INFO: VkStructureType = 1000072002;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_FENCE_INFO: VkStructureType = 1000112000;
    pub const VK_STRUCTURE_TYPE_EXTERNAL_FENCE_PROPERTIES: VkStructureType = 1000112001;
    pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_CREATE_INFO: VkStructureType = 1000113000;
    pub const VK_STRUCTURE_TYPE_EXPORT_SEMAPHORE_CREATE_INFO: VkStructureType = 1000077000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_EXTERNAL_SEMAPHORE_INFO: VkStructureType = 1000076000;
    pub const VK_STRUCTURE_TYPE_EXTERNAL_SEMAPHORE_PROPERTIES: VkStructureType = 1000076001;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_PROPERTIES: VkStructureType = 1000094000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_16BIT_STORAGE_FEATURES: VkStructureType = 1000083000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VARIABLE_POINTERS_FEATURES: VkStructureType = 1000120000;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_UPDATE_TEMPLATE_CREATE_INFO: VkStructureType = 1000085000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_3_PROPERTIES: VkStructureType = 1000168000;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_SUPPORT: VkStructureType = 1000168001;
    pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_CREATE_INFO: VkStructureType = 1000156000;
    pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_INFO: VkStructureType = 1000156001;
    pub const VK_STRUCTURE_TYPE_BIND_IMAGE_PLANE_MEMORY_INFO: VkStructureType = 1000156002;
    pub const VK_STRUCTURE_TYPE_IMAGE_PLANE_MEMORY_REQUIREMENTS_INFO: VkStructureType = 1000156003;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_YCBCR_CONVERSION_FEATURES: VkStructureType = 1000156004;
    pub const VK_STRUCTURE_TYPE_SAMPLER_YCBCR_CONVERSION_IMAGE_FORMAT_PROPERTIES: VkStructureType = 1000156005;
    pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_RENDER_PASS_BEGIN_INFO: VkStructureType = 1000060003;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_POINT_CLIPPING_PROPERTIES: VkStructureType = 1000117000;
    pub const VK_STRUCTURE_TYPE_RENDER_PASS_INPUT_ATTACHMENT_ASPECT_CREATE_INFO: VkStructureType = 1000117001;
    pub const VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_DOMAIN_ORIGIN_STATE_CREATE_INFO: VkStructureType = 1000117003;
    pub const VK_STRUCTURE_TYPE_RENDER_PASS_MULTIVIEW_CREATE_INFO: VkStructureType = 1000053000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_FEATURES: VkStructureType = 1000053001;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MULTIVIEW_PROPERTIES: VkStructureType = 1000053002;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DRAW_PARAMETERS_FEATURES: VkStructureType = 1000063000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DRIVER_PROPERTIES: VkStructureType = 1000196000;
    pub const VK_STRUCTURE_TYPE_IMAGE_FORMAT_LIST_CREATE_INFO: VkStructureType = 1000147000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_MEMORY_MODEL_FEATURES: VkStructureType = 1000211000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_QUERY_RESET_FEATURES: VkStructureType = 1000261000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_FEATURES: VkStructureType = 1000207000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TIMELINE_SEMAPHORE_PROPERTIES: VkStructureType = 1000207001;
    pub const VK_STRUCTURE_TYPE_SEMAPHORE_TYPE_CREATE_INFO: VkStructureType = 1000207002;
    pub const VK_STRUCTURE_TYPE_TIMELINE_SEMAPHORE_SUBMIT_INFO: VkStructureType = 1000207003;
    pub const VK_STRUCTURE_TYPE_SEMAPHORE_WAIT_INFO: VkStructureType = 1000207004;
    pub const VK_STRUCTURE_TYPE_SEMAPHORE_SIGNAL_INFO: VkStructureType = 1000207005;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_BUFFER_DEVICE_ADDRESS_FEATURES: VkStructureType = 1000257000;
    pub const VK_STRUCTURE_TYPE_BUFFER_DEVICE_ADDRESS_INFO: VkStructureType = 1000244001;
    pub const VK_STRUCTURE_TYPE_BUFFER_OPAQUE_CAPTURE_ADDRESS_CREATE_INFO: VkStructureType = 1000257002;
    pub const VK_STRUCTURE_TYPE_MEMORY_OPAQUE_CAPTURE_ADDRESS_ALLOCATE_INFO: VkStructureType = 1000257003;
    pub const VK_STRUCTURE_TYPE_DEVICE_MEMORY_OPAQUE_CAPTURE_ADDRESS_INFO: VkStructureType = 1000257004;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_8BIT_STORAGE_FEATURES: VkStructureType = 1000177000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_ATOMIC_INT64_FEATURES: VkStructureType = 1000180000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT16_INT8_FEATURES: VkStructureType = 1000082000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FLOAT_CONTROLS_PROPERTIES: VkStructureType = 1000197000;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO: VkStructureType = 1000161000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_FEATURES: VkStructureType = 1000161001;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DESCRIPTOR_INDEXING_PROPERTIES: VkStructureType = 1000161002;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO: VkStructureType = 1000161003;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_LAYOUT_SUPPORT: VkStructureType = 1000161004;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SCALAR_BLOCK_LAYOUT_FEATURES: VkStructureType = 1000221000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SAMPLER_FILTER_MINMAX_PROPERTIES: VkStructureType = 1000130000;
    pub const VK_STRUCTURE_TYPE_SAMPLER_REDUCTION_MODE_CREATE_INFO: VkStructureType = 1000130001;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_UNIFORM_BUFFER_STANDARD_LAYOUT_FEATURES: VkStructureType = 1000253000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_EXTENDED_TYPES_FEATURES: VkStructureType = 1000175000;
    pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_2: VkStructureType = 1000109000;
    pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_2: VkStructureType = 1000109001;
    pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_2: VkStructureType = 1000109002;
    pub const VK_STRUCTURE_TYPE_SUBPASS_DEPENDENCY_2: VkStructureType = 1000109003;
    pub const VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO_2: VkStructureType = 1000109004;
    pub const VK_STRUCTURE_TYPE_SUBPASS_BEGIN_INFO: VkStructureType = 1000109005;
    pub const VK_STRUCTURE_TYPE_SUBPASS_END_INFO: VkStructureType = 1000109006;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_STENCIL_RESOLVE_PROPERTIES: VkStructureType = 1000199000;
    pub const VK_STRUCTURE_TYPE_SUBPASS_DESCRIPTION_DEPTH_STENCIL_RESOLVE: VkStructureType = 1000199001;
    pub const VK_STRUCTURE_TYPE_IMAGE_STENCIL_USAGE_CREATE_INFO: VkStructureType = 1000246000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGELESS_FRAMEBUFFER_FEATURES: VkStructureType = 1000108000;
    pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENTS_CREATE_INFO: VkStructureType = 1000108001;
    pub const VK_STRUCTURE_TYPE_FRAMEBUFFER_ATTACHMENT_IMAGE_INFO: VkStructureType = 1000108002;
    pub const VK_STRUCTURE_TYPE_RENDER_PASS_ATTACHMENT_BEGIN_INFO: VkStructureType = 1000108003;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SEPARATE_DEPTH_STENCIL_LAYOUTS_FEATURES: VkStructureType = 1000241000;
    pub const VK_STRUCTURE_TYPE_ATTACHMENT_REFERENCE_STENCIL_LAYOUT: VkStructureType = 1000241001;
    pub const VK_STRUCTURE_TYPE_ATTACHMENT_DESCRIPTION_STENCIL_LAYOUT: VkStructureType = 1000241002;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TOOL_PROPERTIES: VkStructureType = 1000245000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRIVATE_DATA_FEATURES: VkStructureType = 1000295000;
    pub const VK_STRUCTURE_TYPE_DEVICE_PRIVATE_DATA_CREATE_INFO: VkStructureType = 1000295001;
    pub const VK_STRUCTURE_TYPE_PRIVATE_DATA_SLOT_CREATE_INFO: VkStructureType = 1000295002;
    pub const VK_STRUCTURE_TYPE_MEMORY_BARRIER_2: VkStructureType = 1000314000;
    pub const VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER_2: VkStructureType = 1000314001;
    pub const VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER_2: VkStructureType = 1000314002;
    pub const VK_STRUCTURE_TYPE_DEPENDENCY_INFO: VkStructureType = 1000314003;
    pub const VK_STRUCTURE_TYPE_SUBMIT_INFO_2: VkStructureType = 1000314004;
    pub const VK_STRUCTURE_TYPE_SEMAPHORE_SUBMIT_INFO: VkStructureType = 1000314005;
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_SUBMIT_INFO: VkStructureType = 1000314006;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SYNCHRONIZATION_2_FEATURES: VkStructureType = 1000314007;
    pub const VK_STRUCTURE_TYPE_COPY_BUFFER_INFO_2: VkStructureType = 1000337000;
    pub const VK_STRUCTURE_TYPE_COPY_IMAGE_INFO_2: VkStructureType = 1000337001;
    pub const VK_STRUCTURE_TYPE_COPY_BUFFER_TO_IMAGE_INFO_2: VkStructureType = 1000337002;
    pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_BUFFER_INFO_2: VkStructureType = 1000337003;
    pub const VK_STRUCTURE_TYPE_BUFFER_COPY_2: VkStructureType = 1000337006;
    pub const VK_STRUCTURE_TYPE_IMAGE_COPY_2: VkStructureType = 1000337007;
    pub const VK_STRUCTURE_TYPE_BUFFER_IMAGE_COPY_2: VkStructureType = 1000337009;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXTURE_COMPRESSION_ASTC_HDR_FEATURES: VkStructureType = 1000066000;
    pub const VK_STRUCTURE_TYPE_FORMAT_PROPERTIES_3: VkStructureType = 1000360000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_FEATURES: VkStructureType = 1000413000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_4_PROPERTIES: VkStructureType = 1000413001;
    pub const VK_STRUCTURE_TYPE_DEVICE_BUFFER_MEMORY_REQUIREMENTS: VkStructureType = 1000413002;
    pub const VK_STRUCTURE_TYPE_DEVICE_IMAGE_MEMORY_REQUIREMENTS: VkStructureType = 1000413003;
    pub const VK_STRUCTURE_TYPE_PIPELINE_CREATION_FEEDBACK_CREATE_INFO: VkStructureType = 1000192000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_TERMINATE_INVOCATION_FEATURES: VkStructureType = 1000215000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_DEMOTE_TO_HELPER_INVOCATION_FEATURES: VkStructureType = 1000276000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_CREATION_CACHE_CONTROL_FEATURES: VkStructureType = 1000297000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ZERO_INITIALIZE_WORKGROUP_MEMORY_FEATURES: VkStructureType = 1000325000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_IMAGE_ROBUSTNESS_FEATURES: VkStructureType = 1000335000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_PROPERTIES: VkStructureType = 1000225000;
    pub const VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_REQUIRED_SUBGROUP_SIZE_CREATE_INFO: VkStructureType = 1000225001;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SUBGROUP_SIZE_CONTROL_FEATURES: VkStructureType = 1000225002;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_FEATURES: VkStructureType = 1000138000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INLINE_UNIFORM_BLOCK_PROPERTIES: VkStructureType = 1000138001;
    pub const VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET_INLINE_UNIFORM_BLOCK: VkStructureType = 1000138002;
    pub const VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_INLINE_UNIFORM_BLOCK_CREATE_INFO: VkStructureType = 1000138003;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_FEATURES: VkStructureType = 1000280000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_INTEGER_DOT_PRODUCT_PROPERTIES: VkStructureType = 1000280001;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_TEXEL_BUFFER_ALIGNMENT_PROPERTIES: VkStructureType = 1000281001;
    pub const VK_STRUCTURE_TYPE_BLIT_IMAGE_INFO_2: VkStructureType = 1000337004;
    pub const VK_STRUCTURE_TYPE_RESOLVE_IMAGE_INFO_2: VkStructureType = 1000337005;
    pub const VK_STRUCTURE_TYPE_IMAGE_BLIT_2: VkStructureType = 1000337008;
    pub const VK_STRUCTURE_TYPE_IMAGE_RESOLVE_2: VkStructureType = 1000337010;
    pub const VK_STRUCTURE_TYPE_RENDERING_INFO: VkStructureType = 1000044000;
    pub const VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_INFO: VkStructureType = 1000044001;
    pub const VK_STRUCTURE_TYPE_PIPELINE_RENDERING_CREATE_INFO: VkStructureType = 1000044002;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_FEATURES: VkStructureType = 1000044003;
    pub const VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_RENDERING_INFO: VkStructureType = 1000044004;
    pub const VK_STRUCTURE_TYPE_DEVICE_QUEUE_GLOBAL_PRIORITY_CREATE_INFO: VkStructureType = 1000174000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_GLOBAL_PRIORITY_QUERY_FEATURES: VkStructureType = 1000388000;
    pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_GLOBAL_PRIORITY_PROPERTIES: VkStructureType = 1000388001;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_INDEX_TYPE_UINT8_FEATURES: VkStructureType = 1000265000;
    pub const VK_STRUCTURE_TYPE_MEMORY_MAP_INFO: VkStructureType = 1000271000;
    pub const VK_STRUCTURE_TYPE_MEMORY_UNMAP_INFO: VkStructureType = 1000271001;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_FEATURES: VkStructureType = 1000470000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_5_PROPERTIES: VkStructureType = 1000470001;
    pub const VK_STRUCTURE_TYPE_DEVICE_IMAGE_SUBRESOURCE_INFO: VkStructureType = 1000470004;
    pub const VK_STRUCTURE_TYPE_SUBRESOURCE_LAYOUT_2: VkStructureType = 1000338002;
    pub const VK_STRUCTURE_TYPE_IMAGE_SUBRESOURCE_2: VkStructureType = 1000338003;
    pub const VK_STRUCTURE_TYPE_BUFFER_USAGE_FLAGS_2_CREATE_INFO: VkStructureType = 1000470006;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_FEATURES: VkStructureType = 1000545000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MAINTENANCE_6_PROPERTIES: VkStructureType = 1000545001;
    pub const VK_STRUCTURE_TYPE_BIND_MEMORY_STATUS: VkStructureType = 1000545002;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_FEATURES: VkStructureType = 1000270000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_HOST_IMAGE_COPY_PROPERTIES: VkStructureType = 1000270001;
    pub const VK_STRUCTURE_TYPE_MEMORY_TO_IMAGE_COPY: VkStructureType = 1000270002;
    pub const VK_STRUCTURE_TYPE_IMAGE_TO_MEMORY_COPY: VkStructureType = 1000270003;
    pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_MEMORY_INFO: VkStructureType = 1000270004;
    pub const VK_STRUCTURE_TYPE_COPY_MEMORY_TO_IMAGE_INFO: VkStructureType = 1000270005;
    pub const VK_STRUCTURE_TYPE_HOST_IMAGE_LAYOUT_TRANSITION_INFO: VkStructureType = 1000270006;
    pub const VK_STRUCTURE_TYPE_COPY_IMAGE_TO_IMAGE_INFO: VkStructureType = 1000270007;
    pub const VK_STRUCTURE_TYPE_SUBRESOURCE_HOST_MEMCPY_SIZE: VkStructureType = 1000270008;
    pub const VK_STRUCTURE_TYPE_HOST_IMAGE_COPY_DEVICE_PERFORMANCE_QUERY: VkStructureType = 1000270009;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_SUBGROUP_ROTATE_FEATURES: VkStructureType = 1000416000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_FLOAT_CONTROLS_2_FEATURES: VkStructureType = 1000528000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SHADER_EXPECT_ASSUME_FEATURES: VkStructureType = 1000544000;
    pub const VK_STRUCTURE_TYPE_PIPELINE_CREATE_FLAGS_2_CREATE_INFO: VkStructureType = 1000470005;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PUSH_DESCRIPTOR_PROPERTIES: VkStructureType = 1000080000;
    pub const VK_STRUCTURE_TYPE_BIND_DESCRIPTOR_SETS_INFO: VkStructureType = 1000545003;
    pub const VK_STRUCTURE_TYPE_PUSH_CONSTANTS_INFO: VkStructureType = 1000545004;
    pub const VK_STRUCTURE_TYPE_PUSH_DESCRIPTOR_SET_INFO: VkStructureType = 1000545005;
    pub const VK_STRUCTURE_TYPE_PUSH_DESCRIPTOR_SET_WITH_TEMPLATE_INFO: VkStructureType = 1000545006;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_PROTECTED_ACCESS_FEATURES: VkStructureType = 1000466000;
    pub const VK_STRUCTURE_TYPE_PIPELINE_ROBUSTNESS_CREATE_INFO: VkStructureType = 1000068000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_FEATURES: VkStructureType = 1000068001;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PIPELINE_ROBUSTNESS_PROPERTIES: VkStructureType = 1000068002;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_FEATURES: VkStructureType = 1000259000;
    pub const VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_LINE_STATE_CREATE_INFO: VkStructureType = 1000259001;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_LINE_RASTERIZATION_PROPERTIES: VkStructureType = 1000259002;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_PROPERTIES: VkStructureType = 1000525000;
    pub const VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_DIVISOR_STATE_CREATE_INFO: VkStructureType = 1000190001;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VERTEX_ATTRIBUTE_DIVISOR_FEATURES: VkStructureType = 1000190002;
    pub const VK_STRUCTURE_TYPE_RENDERING_AREA_INFO: VkStructureType = 1000470003;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DYNAMIC_RENDERING_LOCAL_READ_FEATURES: VkStructureType = 1000232000;
    pub const VK_STRUCTURE_TYPE_RENDERING_ATTACHMENT_LOCATION_INFO: VkStructureType = 1000232001;
    pub const VK_STRUCTURE_TYPE_RENDERING_INPUT_ATTACHMENT_INDEX_INFO: VkStructureType = 1000232002;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_SC_1_0_FEATURES: VkStructureType = 1000298000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_SC_1_0_PROPERTIES: VkStructureType = 1000298001;
    pub const VK_STRUCTURE_TYPE_DEVICE_OBJECT_RESERVATION_CREATE_INFO: VkStructureType = 1000298002;
    pub const VK_STRUCTURE_TYPE_COMMAND_POOL_MEMORY_RESERVATION_CREATE_INFO: VkStructureType = 1000298003;
    pub const VK_STRUCTURE_TYPE_COMMAND_POOL_MEMORY_CONSUMPTION: VkStructureType = 1000298004;
    pub const VK_STRUCTURE_TYPE_PIPELINE_POOL_SIZE: VkStructureType = 1000298005;
    pub const VK_STRUCTURE_TYPE_FAULT_DATA: VkStructureType = 1000298007;
    pub const VK_STRUCTURE_TYPE_FAULT_CALLBACK_INFO: VkStructureType = 1000298008;
    pub const VK_STRUCTURE_TYPE_PIPELINE_OFFLINE_CREATE_INFO: VkStructureType = 1000298010;
    pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_CAPABILITIES_KHR: VkStructureType = 1000060007;
    pub const VK_STRUCTURE_TYPE_IMAGE_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = 1000060008;
    pub const VK_STRUCTURE_TYPE_BIND_IMAGE_MEMORY_SWAPCHAIN_INFO_KHR: VkStructureType = 1000060009;
    pub const VK_STRUCTURE_TYPE_ACQUIRE_NEXT_IMAGE_INFO_KHR: VkStructureType = 1000060010;
    pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_PRESENT_INFO_KHR: VkStructureType = 1000060011;
    pub const VK_STRUCTURE_TYPE_DEVICE_GROUP_SWAPCHAIN_CREATE_INFO_KHR: VkStructureType = 1000060012;
    pub const VK_STRUCTURE_TYPE_MULTIVIEW_PER_VIEW_ATTRIBUTES_INFO_NVX: VkStructureType = 1000044009;
    pub const VK_STRUCTURE_TYPE_ATTACHMENT_SAMPLE_COUNT_INFO_AMD: VkStructureType = 1000044008;
    pub const VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_CREATE_INFO_KHR: VkStructureType = 1000150015;
    pub const VK_STRUCTURE_TYPE_RAY_TRACING_SHADER_GROUP_CREATE_INFO_KHR: VkStructureType = 1000150016;
    pub const VK_STRUCTURE_TYPE_RAY_TRACING_PIPELINE_INTERFACE_CREATE_INFO_KHR: VkStructureType = 1000150018;
    pub const VK_STRUCTURE_TYPE_QUEUE_FAMILY_CHECKPOINT_PROPERTIES_2_NV: VkStructureType = 1000314008;
    pub const VK_STRUCTURE_TYPE_CHECKPOINT_DATA_2_NV: VkStructureType = 1000314009;
    pub const VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_DENSITY_MAP_ATTACHMENT_INFO_EXT: VkStructureType = 1000044007;
    pub const VK_STRUCTURE_TYPE_RENDERING_FRAGMENT_SHADING_RATE_ATTACHMENT_INFO_KHR: VkStructureType = 1000044006;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_SHADER_BARYCENTRIC_FEATURES_KHR: VkStructureType = 1000203000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_RASTERIZATION_ORDER_ATTACHMENT_ACCESS_FEATURES_EXT: VkStructureType = 1000342000;
    pub const VK_STRUCTURE_TYPE_VERTEX_INPUT_BINDING_DESCRIPTION_2_EXT: VkStructureType = 1000352001;
    pub const VK_STRUCTURE_TYPE_VERTEX_INPUT_ATTRIBUTE_DESCRIPTION_2_EXT: VkStructureType = 1000352002;
    pub const VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_KHR: VkStructureType = 1000274000;
    pub const VK_STRUCTURE_TYPE_SURFACE_PRESENT_SCALING_CAPABILITIES_KHR: VkStructureType = 1000274001;
    pub const VK_STRUCTURE_TYPE_SURFACE_PRESENT_MODE_COMPATIBILITY_KHR: VkStructureType = 1000274002;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_SWAPCHAIN_MAINTENANCE_1_FEATURES_KHR: VkStructureType = 1000275000;
    pub const VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_FENCE_INFO_KHR: VkStructureType = 1000275001;
    pub const VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_MODES_CREATE_INFO_KHR: VkStructureType = 1000275002;
    pub const VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_MODE_INFO_KHR: VkStructureType = 1000275003;
    pub const VK_STRUCTURE_TYPE_SWAPCHAIN_PRESENT_SCALING_CREATE_INFO_KHR: VkStructureType = 1000275004;
    pub const VK_STRUCTURE_TYPE_RELEASE_SWAPCHAIN_IMAGES_INFO_KHR: VkStructureType = 1000275005;
    pub const VK_STRUCTURE_TYPE_IMPORT_FENCE_SCI_SYNC_INFO_NV: VkStructureType = 1000373000;
    pub const VK_STRUCTURE_TYPE_EXPORT_FENCE_SCI_SYNC_INFO_NV: VkStructureType = 1000373001;
    pub const VK_STRUCTURE_TYPE_FENCE_GET_SCI_SYNC_INFO_NV: VkStructureType = 1000373002;
    pub const VK_STRUCTURE_TYPE_SCI_SYNC_ATTRIBUTES_INFO_NV: VkStructureType = 1000373003;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MUTABLE_DESCRIPTOR_TYPE_FEATURES_EXT: VkStructureType = 1000351000;
    pub const VK_STRUCTURE_TYPE_MUTABLE_DESCRIPTOR_TYPE_CREATE_INFO_EXT: VkStructureType = 1000351002;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COMPUTE_SHADER_DERIVATIVES_FEATURES_KHR: VkStructureType = 1000201000;
    pub const VK_STRUCTURE_TYPE_CALIBRATED_TIMESTAMP_INFO_KHR: VkStructureType = 1000184000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_COPY_MEMORY_INDIRECT_PROPERTIES_KHR: VkStructureType = 1000426001;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_FEATURES_EXT: VkStructureType = 1000427000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_MEMORY_DECOMPRESSION_PROPERTIES_EXT: VkStructureType = 1000427001;
    pub const VK_STRUCTURE_TYPE_NATIVE_BUFFER_OHOS: VkStructureType = 1000453001;
    pub const VK_STRUCTURE_TYPE_SWAPCHAIN_IMAGE_CREATE_INFO_OHOS: VkStructureType = 1000453002;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENTATION_PROPERTIES_OHOS: VkStructureType = 1000453003;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_DEPTH_CLAMP_ZERO_ONE_FEATURES_KHR: VkStructureType = 1000421000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_FEATURES_KHR: VkStructureType = 1000286000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_ROBUSTNESS_2_PROPERTIES_KHR: VkStructureType = 1000286001;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_FEATURES_EXT: VkStructureType = 1000425000;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_FRAGMENT_DENSITY_MAP_OFFSET_PROPERTIES_EXT: VkStructureType = 1000425001;
    pub const VK_STRUCTURE_TYPE_RENDER_PASS_FRAGMENT_DENSITY_MAP_OFFSET_END_INFO_EXT: VkStructureType = 1000425002;
    pub const VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PRESENT_MODE_FIFO_LATEST_READY_FEATURES_KHR: VkStructureType = 1000361000;
    pub const VK_STRUCTURE_TYPE_RENDERING_END_INFO_KHR: VkStructureType = 1000619003;
}

pub type VkRasterizationOrderAMD = i32;
pub mod VkRasterizationOrderAMDValue {
    use crate::VkRasterizationOrderAMD;
    pub const VK_RASTERIZATION_ORDER_STRICT_AMD: VkRasterizationOrderAMD = 0;
    pub const VK_RASTERIZATION_ORDER_RELAXED_AMD: VkRasterizationOrderAMD = 1;
}

pub type VkComponentSwizzle = i32;
pub mod VkComponentSwizzleValue {
    use crate::VkComponentSwizzle;
    pub const VK_COMPONENT_SWIZZLE_IDENTITY: VkComponentSwizzle = 0;
    pub const VK_COMPONENT_SWIZZLE_ZERO: VkComponentSwizzle = 1;
    pub const VK_COMPONENT_SWIZZLE_ONE: VkComponentSwizzle = 2;
    pub const VK_COMPONENT_SWIZZLE_R: VkComponentSwizzle = 3;
    pub const VK_COMPONENT_SWIZZLE_G: VkComponentSwizzle = 4;
    pub const VK_COMPONENT_SWIZZLE_B: VkComponentSwizzle = 5;
    pub const VK_COMPONENT_SWIZZLE_A: VkComponentSwizzle = 6;
}

pub type VkRayTracingLssIndexingModeNV = i32;
pub mod VkRayTracingLssIndexingModeNVValue {
    use crate::VkRayTracingLssIndexingModeNV;
    pub const VK_RAY_TRACING_LSS_INDEXING_MODE_LIST_NV: VkRayTracingLssIndexingModeNV = 0;
    pub const VK_RAY_TRACING_LSS_INDEXING_MODE_SUCCESSIVE_NV: VkRayTracingLssIndexingModeNV = 1;
}

pub type VkSamplerYcbcrModelConversion = i32;
pub mod VkSamplerYcbcrModelConversionValue {
    use crate::VkSamplerYcbcrModelConversion;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_RGB_IDENTITY: VkSamplerYcbcrModelConversion = 0;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_IDENTITY: VkSamplerYcbcrModelConversion = 1;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_709: VkSamplerYcbcrModelConversion = 2;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_601: VkSamplerYcbcrModelConversion = 3;
    pub const VK_SAMPLER_YCBCR_MODEL_CONVERSION_YCBCR_2020: VkSamplerYcbcrModelConversion = 4;
}

pub type VkClusterAccelerationStructureTypeNV = i32;
pub mod VkClusterAccelerationStructureTypeNVValue {
    use crate::VkClusterAccelerationStructureTypeNV;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_TYPE_CLUSTERS_BOTTOM_LEVEL_NV: VkClusterAccelerationStructureTypeNV = 0;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_TYPE_TRIANGLE_CLUSTER_NV: VkClusterAccelerationStructureTypeNV = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_TYPE_TRIANGLE_CLUSTER_TEMPLATE_NV: VkClusterAccelerationStructureTypeNV = 2;
}

pub type VkCoarseSampleOrderTypeNV = i32;
pub mod VkCoarseSampleOrderTypeNVValue {
    use crate::VkCoarseSampleOrderTypeNV;
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_DEFAULT_NV: VkCoarseSampleOrderTypeNV = 0;
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_CUSTOM_NV: VkCoarseSampleOrderTypeNV = 1;
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_PIXEL_MAJOR_NV: VkCoarseSampleOrderTypeNV = 2;
    pub const VK_COARSE_SAMPLE_ORDER_TYPE_SAMPLE_MAJOR_NV: VkCoarseSampleOrderTypeNV = 3;
}

pub type VkFaultLevel = i32;
pub mod VkFaultLevelValue {
    use crate::VkFaultLevel;
    pub const VK_FAULT_LEVEL_UNASSIGNED: VkFaultLevel = 0;
    pub const VK_FAULT_LEVEL_CRITICAL: VkFaultLevel = 1;
    pub const VK_FAULT_LEVEL_RECOVERABLE: VkFaultLevel = 2;
    pub const VK_FAULT_LEVEL_WARNING: VkFaultLevel = 3;
}

pub type VkFragmentShadingRateCombinerOpKHR = i32;
pub mod VkFragmentShadingRateCombinerOpKHRValue {
    use crate::VkFragmentShadingRateCombinerOpKHR;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_KEEP_KHR: VkFragmentShadingRateCombinerOpKHR = 0;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_REPLACE_KHR: VkFragmentShadingRateCombinerOpKHR = 1;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MIN_KHR: VkFragmentShadingRateCombinerOpKHR = 2;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MAX_KHR: VkFragmentShadingRateCombinerOpKHR = 3;
    pub const VK_FRAGMENT_SHADING_RATE_COMBINER_OP_MUL_KHR: VkFragmentShadingRateCombinerOpKHR = 4;
}

pub type VkPartitionedAccelerationStructureOpTypeNV = i32;
pub mod VkPartitionedAccelerationStructureOpTypeNVValue {
    use crate::VkPartitionedAccelerationStructureOpTypeNV;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_WRITE_INSTANCE_NV: VkPartitionedAccelerationStructureOpTypeNV = 0;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_UPDATE_INSTANCE_NV: VkPartitionedAccelerationStructureOpTypeNV = 1;
    pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_OP_TYPE_WRITE_PARTITION_TRANSLATION_NV: VkPartitionedAccelerationStructureOpTypeNV = 2;
}

pub type VkSamplerReductionMode = i32;
pub mod VkSamplerReductionModeValue {
    use crate::VkSamplerReductionMode;
    pub const VK_SAMPLER_REDUCTION_MODE_WEIGHTED_AVERAGE: VkSamplerReductionMode = 0;
    pub const VK_SAMPLER_REDUCTION_MODE_MIN: VkSamplerReductionMode = 1;
    pub const VK_SAMPLER_REDUCTION_MODE_MAX: VkSamplerReductionMode = 2;
}

pub type VkPipelineRobustnessBufferBehavior = i32;
pub mod VkPipelineRobustnessBufferBehaviorValue {
    use crate::VkPipelineRobustnessBufferBehavior;
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DEVICE_DEFAULT: VkPipelineRobustnessBufferBehavior = 0;
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_DISABLED: VkPipelineRobustnessBufferBehavior = 1;
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS: VkPipelineRobustnessBufferBehavior = 2;
    pub const VK_PIPELINE_ROBUSTNESS_BUFFER_BEHAVIOR_ROBUST_BUFFER_ACCESS_2: VkPipelineRobustnessBufferBehavior = 3;
}

pub type VkComponentTypeKHR = i32;
pub mod VkComponentTypeKHRValue {
    use crate::VkComponentTypeKHR;
    pub const VK_COMPONENT_TYPE_FLOAT16_KHR: VkComponentTypeKHR = 0;
    pub const VK_COMPONENT_TYPE_FLOAT32_KHR: VkComponentTypeKHR = 1;
    pub const VK_COMPONENT_TYPE_FLOAT64_KHR: VkComponentTypeKHR = 2;
    pub const VK_COMPONENT_TYPE_SINT8_KHR: VkComponentTypeKHR = 3;
    pub const VK_COMPONENT_TYPE_SINT16_KHR: VkComponentTypeKHR = 4;
    pub const VK_COMPONENT_TYPE_SINT32_KHR: VkComponentTypeKHR = 5;
    pub const VK_COMPONENT_TYPE_SINT64_KHR: VkComponentTypeKHR = 6;
    pub const VK_COMPONENT_TYPE_UINT8_KHR: VkComponentTypeKHR = 7;
    pub const VK_COMPONENT_TYPE_UINT16_KHR: VkComponentTypeKHR = 8;
    pub const VK_COMPONENT_TYPE_UINT32_KHR: VkComponentTypeKHR = 9;
    pub const VK_COMPONENT_TYPE_UINT64_KHR: VkComponentTypeKHR = 10;
    pub const VK_COMPONENT_TYPE_FLOAT8_E4M3_EXT: VkComponentTypeKHR = 1000491002;
    pub const VK_COMPONENT_TYPE_FLOAT8_E5M2_EXT: VkComponentTypeKHR = 1000491003;
}

pub type VkAccelerationStructureTypeKHR = i32;
pub mod VkAccelerationStructureTypeKHRValue {
    use crate::VkAccelerationStructureTypeKHR;
    pub const VK_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL_KHR: VkAccelerationStructureTypeKHR = 0;
    pub const VK_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL_KHR: VkAccelerationStructureTypeKHR = 1;
    pub const VK_ACCELERATION_STRUCTURE_TYPE_GENERIC_KHR: VkAccelerationStructureTypeKHR = 2;
}

pub type VkDisplaySurfaceStereoTypeNV = i32;
pub mod VkDisplaySurfaceStereoTypeNVValue {
    use crate::VkDisplaySurfaceStereoTypeNV;
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_NONE_NV: VkDisplaySurfaceStereoTypeNV = 0;
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_ONBOARD_DIN_NV: VkDisplaySurfaceStereoTypeNV = 1;
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_HDMI_3D_NV: VkDisplaySurfaceStereoTypeNV = 2;
    pub const VK_DISPLAY_SURFACE_STEREO_TYPE_INBAND_DISPLAYPORT_NV: VkDisplaySurfaceStereoTypeNV = 3;
}

pub type VkBuildAccelerationStructureModeKHR = i32;
pub mod VkBuildAccelerationStructureModeKHRValue {
    use crate::VkBuildAccelerationStructureModeKHR;
    pub const VK_BUILD_ACCELERATION_STRUCTURE_MODE_BUILD_KHR: VkBuildAccelerationStructureModeKHR = 0;
    pub const VK_BUILD_ACCELERATION_STRUCTURE_MODE_UPDATE_KHR: VkBuildAccelerationStructureModeKHR = 1;
}

pub type VkAccelerationStructureSerializedBlockTypeKHR = i32;
pub mod VkAccelerationStructureSerializedBlockTypeKHRValue {
    use crate::VkAccelerationStructureSerializedBlockTypeKHR;
    pub const VK_ACCELERATION_STRUCTURE_SERIALIZED_BLOCK_TYPE_OPACITY_MICROMAP_KHR: VkAccelerationStructureSerializedBlockTypeKHR = 0;
}

pub type VkDataGraphPipelineSessionBindPointTypeARM = i32;
pub mod VkDataGraphPipelineSessionBindPointTypeARMValue {
    use crate::VkDataGraphPipelineSessionBindPointTypeARM;
    pub const VK_DATA_GRAPH_PIPELINE_SESSION_BIND_POINT_TYPE_MEMORY_ARM: VkDataGraphPipelineSessionBindPointTypeARM = 0;
}

pub type VkDataGraphPipelinePropertyARM = i32;
pub mod VkDataGraphPipelinePropertyARMValue {
    use crate::VkDataGraphPipelinePropertyARM;
    pub const VK_DATA_GRAPH_PIPELINE_PROPERTY_CREATION_LOG_ARM: VkDataGraphPipelinePropertyARM = 0;
    pub const VK_DATA_GRAPH_PIPELINE_PROPERTY_IDENTIFIER_ARM: VkDataGraphPipelinePropertyARM = 1;
}

pub type VkDataGraphTOSALevelARM = i32;
pub mod VkDataGraphTOSALevelARMValue {
    use crate::VkDataGraphTOSALevelARM;
    pub const VK_DATA_GRAPH_TOSA_LEVEL_NONE_ARM: VkDataGraphTOSALevelARM = 0;
    pub const VK_DATA_GRAPH_TOSA_LEVEL_8K_ARM: VkDataGraphTOSALevelARM = 1;
}

pub type VkSamplerAddressMode = i32;
pub mod VkSamplerAddressModeValue {
    use crate::VkSamplerAddressMode;
    pub const VK_SAMPLER_ADDRESS_MODE_REPEAT: VkSamplerAddressMode = 0;
    pub const VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT: VkSamplerAddressMode = 1;
    pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE: VkSamplerAddressMode = 2;
    pub const VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER: VkSamplerAddressMode = 3;
}

pub type VkValidationFeatureDisableEXT = i32;
pub mod VkValidationFeatureDisableEXTValue {
    use crate::VkValidationFeatureDisableEXT;
    pub const VK_VALIDATION_FEATURE_DISABLE_ALL_EXT: VkValidationFeatureDisableEXT = 0;
    pub const VK_VALIDATION_FEATURE_DISABLE_SHADERS_EXT: VkValidationFeatureDisableEXT = 1;
    pub const VK_VALIDATION_FEATURE_DISABLE_THREAD_SAFETY_EXT: VkValidationFeatureDisableEXT = 2;
    pub const VK_VALIDATION_FEATURE_DISABLE_API_PARAMETERS_EXT: VkValidationFeatureDisableEXT = 3;
    pub const VK_VALIDATION_FEATURE_DISABLE_OBJECT_LIFETIMES_EXT: VkValidationFeatureDisableEXT = 4;
    pub const VK_VALIDATION_FEATURE_DISABLE_CORE_CHECKS_EXT: VkValidationFeatureDisableEXT = 5;
    pub const VK_VALIDATION_FEATURE_DISABLE_UNIQUE_HANDLES_EXT: VkValidationFeatureDisableEXT = 6;
    pub const VK_VALIDATION_FEATURE_DISABLE_SHADER_VALIDATION_CACHE_EXT: VkValidationFeatureDisableEXT = 7;
}

pub type VkGpaPerfBlockAMD = i32;
pub mod VkGpaPerfBlockAMDValue {
    use crate::VkGpaPerfBlockAMD;
    pub const VK_GPA_PERF_BLOCK_CPF_AMD: VkGpaPerfBlockAMD = 0;
    pub const VK_GPA_PERF_BLOCK_IA_AMD: VkGpaPerfBlockAMD = 1;
    pub const VK_GPA_PERF_BLOCK_VGT_AMD: VkGpaPerfBlockAMD = 2;
    pub const VK_GPA_PERF_BLOCK_PA_AMD: VkGpaPerfBlockAMD = 3;
    pub const VK_GPA_PERF_BLOCK_SC_AMD: VkGpaPerfBlockAMD = 4;
    pub const VK_GPA_PERF_BLOCK_SPI_AMD: VkGpaPerfBlockAMD = 5;
    pub const VK_GPA_PERF_BLOCK_SQ_AMD: VkGpaPerfBlockAMD = 6;
    pub const VK_GPA_PERF_BLOCK_SX_AMD: VkGpaPerfBlockAMD = 7;
    pub const VK_GPA_PERF_BLOCK_TA_AMD: VkGpaPerfBlockAMD = 8;
    pub const VK_GPA_PERF_BLOCK_TD_AMD: VkGpaPerfBlockAMD = 9;
    pub const VK_GPA_PERF_BLOCK_TCP_AMD: VkGpaPerfBlockAMD = 10;
    pub const VK_GPA_PERF_BLOCK_TCC_AMD: VkGpaPerfBlockAMD = 11;
    pub const VK_GPA_PERF_BLOCK_TCA_AMD: VkGpaPerfBlockAMD = 12;
    pub const VK_GPA_PERF_BLOCK_DB_AMD: VkGpaPerfBlockAMD = 13;
    pub const VK_GPA_PERF_BLOCK_CB_AMD: VkGpaPerfBlockAMD = 14;
    pub const VK_GPA_PERF_BLOCK_GDS_AMD: VkGpaPerfBlockAMD = 15;
    pub const VK_GPA_PERF_BLOCK_SRBM_AMD: VkGpaPerfBlockAMD = 16;
    pub const VK_GPA_PERF_BLOCK_GRBM_AMD: VkGpaPerfBlockAMD = 17;
    pub const VK_GPA_PERF_BLOCK_GRBM_SE_AMD: VkGpaPerfBlockAMD = 18;
    pub const VK_GPA_PERF_BLOCK_RLC_AMD: VkGpaPerfBlockAMD = 19;
    pub const VK_GPA_PERF_BLOCK_DMA_AMD: VkGpaPerfBlockAMD = 20;
    pub const VK_GPA_PERF_BLOCK_MC_AMD: VkGpaPerfBlockAMD = 21;
    pub const VK_GPA_PERF_BLOCK_CPG_AMD: VkGpaPerfBlockAMD = 22;
    pub const VK_GPA_PERF_BLOCK_CPC_AMD: VkGpaPerfBlockAMD = 23;
    pub const VK_GPA_PERF_BLOCK_WD_AMD: VkGpaPerfBlockAMD = 24;
    pub const VK_GPA_PERF_BLOCK_TCS_AMD: VkGpaPerfBlockAMD = 25;
    pub const VK_GPA_PERF_BLOCK_ATC_AMD: VkGpaPerfBlockAMD = 26;
    pub const VK_GPA_PERF_BLOCK_ATC_L2_AMD: VkGpaPerfBlockAMD = 27;
    pub const VK_GPA_PERF_BLOCK_MC_VM_L2_AMD: VkGpaPerfBlockAMD = 28;
    pub const VK_GPA_PERF_BLOCK_EA_AMD: VkGpaPerfBlockAMD = 29;
    pub const VK_GPA_PERF_BLOCK_RPB_AMD: VkGpaPerfBlockAMD = 30;
    pub const VK_GPA_PERF_BLOCK_RMI_AMD: VkGpaPerfBlockAMD = 31;
    pub const VK_GPA_PERF_BLOCK_UMCCH_AMD: VkGpaPerfBlockAMD = 32;
    pub const VK_GPA_PERF_BLOCK_GE_AMD: VkGpaPerfBlockAMD = 33;
    pub const VK_GPA_PERF_BLOCK_GL1A_AMD: VkGpaPerfBlockAMD = 34;
    pub const VK_GPA_PERF_BLOCK_GL1C_AMD: VkGpaPerfBlockAMD = 35;
    pub const VK_GPA_PERF_BLOCK_GL1CG_AMD: VkGpaPerfBlockAMD = 36;
    pub const VK_GPA_PERF_BLOCK_GL2A_AMD: VkGpaPerfBlockAMD = 37;
    pub const VK_GPA_PERF_BLOCK_GL2C_AMD: VkGpaPerfBlockAMD = 38;
    pub const VK_GPA_PERF_BLOCK_CHA_AMD: VkGpaPerfBlockAMD = 39;
    pub const VK_GPA_PERF_BLOCK_CHC_AMD: VkGpaPerfBlockAMD = 40;
    pub const VK_GPA_PERF_BLOCK_CHCG_AMD: VkGpaPerfBlockAMD = 41;
    pub const VK_GPA_PERF_BLOCK_GUS_AMD: VkGpaPerfBlockAMD = 42;
    pub const VK_GPA_PERF_BLOCK_GCR_AMD: VkGpaPerfBlockAMD = 43;
    pub const VK_GPA_PERF_BLOCK_PH_AMD: VkGpaPerfBlockAMD = 44;
    pub const VK_GPA_PERF_BLOCK_UTCL1_AMD: VkGpaPerfBlockAMD = 45;
    pub const VK_GPA_PERF_BLOCK_GE1_AMD: VkGpaPerfBlockAMD = VK_GPA_PERF_BLOCK_GE_AMD;
    pub const VK_GPA_PERF_BLOCK_GE_DIST_AMD: VkGpaPerfBlockAMD = 46;
    pub const VK_GPA_PERF_BLOCK_GE_SE_AMD: VkGpaPerfBlockAMD = 47;
    pub const VK_GPA_PERF_BLOCK_DF_MALL_AMD: VkGpaPerfBlockAMD = 48;
    pub const VK_GPA_PERF_BLOCK_SQ_WGP_AMD: VkGpaPerfBlockAMD = 49;
    pub const VK_GPA_PERF_BLOCK_PC_AMD: VkGpaPerfBlockAMD = 50;
    pub const VK_GPA_PERF_BLOCK_GL1XA_AMD: VkGpaPerfBlockAMD = 51;
    pub const VK_GPA_PERF_BLOCK_GL1XC_AMD: VkGpaPerfBlockAMD = 52;
    pub const VK_GPA_PERF_BLOCK_WGS_AMD: VkGpaPerfBlockAMD = 53;
    pub const VK_GPA_PERF_BLOCK_EACPWD_AMD: VkGpaPerfBlockAMD = 54;
    pub const VK_GPA_PERF_BLOCK_EASE_AMD: VkGpaPerfBlockAMD = 55;
    pub const VK_GPA_PERF_BLOCK_RLCUSER_AMD: VkGpaPerfBlockAMD = 56;
    pub const VK_GPA_PERF_BLOCK_RLCLOCAL_AMD: VkGpaPerfBlockAMD = VK_GPA_PERF_BLOCK_RLCUSER_AMD;
}

pub type VkIndirectCommandsTokenTypeNV = i32;
pub mod VkIndirectCommandsTokenTypeNVValue {
    use crate::VkIndirectCommandsTokenTypeNV;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_SHADER_GROUP_NV: VkIndirectCommandsTokenTypeNV = 0;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_STATE_FLAGS_NV: VkIndirectCommandsTokenTypeNV = 1;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_INDEX_BUFFER_NV: VkIndirectCommandsTokenTypeNV = 2;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_VERTEX_BUFFER_NV: VkIndirectCommandsTokenTypeNV = 3;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_PUSH_CONSTANT_NV: VkIndirectCommandsTokenTypeNV = 4;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_INDEXED_NV: VkIndirectCommandsTokenTypeNV = 5;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_NV: VkIndirectCommandsTokenTypeNV = 6;
    pub const VK_INDIRECT_COMMANDS_TOKEN_TYPE_DRAW_TASKS_NV: VkIndirectCommandsTokenTypeNV = 7;
}

pub type VkPipelineExecutableStatisticFormatKHR = i32;
pub mod VkPipelineExecutableStatisticFormatKHRValue {
    use crate::VkPipelineExecutableStatisticFormatKHR;
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_BOOL32_KHR: VkPipelineExecutableStatisticFormatKHR = 0;
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_INT64_KHR: VkPipelineExecutableStatisticFormatKHR = 1;
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_UINT64_KHR: VkPipelineExecutableStatisticFormatKHR = 2;
    pub const VK_PIPELINE_EXECUTABLE_STATISTIC_FORMAT_FLOAT64_KHR: VkPipelineExecutableStatisticFormatKHR = 3;
}

pub type VkDataGraphPipelineNodeTypeARM = i32;
pub mod VkDataGraphPipelineNodeTypeARMValue {
    use crate::VkDataGraphPipelineNodeTypeARM;
}

pub type VkClusterAccelerationStructureOpTypeNV = i32;
pub mod VkClusterAccelerationStructureOpTypeNVValue {
    use crate::VkClusterAccelerationStructureOpTypeNV;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_MOVE_OBJECTS_NV: VkClusterAccelerationStructureOpTypeNV = 0;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_CLUSTERS_BOTTOM_LEVEL_NV: VkClusterAccelerationStructureOpTypeNV = 1;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_TRIANGLE_CLUSTER_NV: VkClusterAccelerationStructureOpTypeNV = 2;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_BUILD_TRIANGLE_CLUSTER_TEMPLATE_NV: VkClusterAccelerationStructureOpTypeNV = 3;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_INSTANTIATE_TRIANGLE_CLUSTER_NV: VkClusterAccelerationStructureOpTypeNV = 4;
    pub const VK_CLUSTER_ACCELERATION_STRUCTURE_OP_TYPE_GET_CLUSTER_TEMPLATE_INDICES_NV: VkClusterAccelerationStructureOpTypeNV = 5;
}

pub type VkDriverId = i32;
pub mod VkDriverIdValue {
    use crate::VkDriverId;
    pub const VK_DRIVER_ID_AMD_PROPRIETARY: VkDriverId = 1;
    pub const VK_DRIVER_ID_AMD_OPEN_SOURCE: VkDriverId = 2;
    pub const VK_DRIVER_ID_MESA_RADV: VkDriverId = 3;
    pub const VK_DRIVER_ID_NVIDIA_PROPRIETARY: VkDriverId = 4;
    pub const VK_DRIVER_ID_INTEL_PROPRIETARY_WINDOWS: VkDriverId = 5;
    pub const VK_DRIVER_ID_INTEL_OPEN_SOURCE_MESA: VkDriverId = 6;
    pub const VK_DRIVER_ID_IMAGINATION_PROPRIETARY: VkDriverId = 7;
    pub const VK_DRIVER_ID_QUALCOMM_PROPRIETARY: VkDriverId = 8;
    pub const VK_DRIVER_ID_ARM_PROPRIETARY: VkDriverId = 9;
    pub const VK_DRIVER_ID_GOOGLE_SWIFTSHADER: VkDriverId = 10;
    pub const VK_DRIVER_ID_GGP_PROPRIETARY: VkDriverId = 11;
    pub const VK_DRIVER_ID_BROADCOM_PROPRIETARY: VkDriverId = 12;
    pub const VK_DRIVER_ID_MESA_LLVMPIPE: VkDriverId = 13;
    pub const VK_DRIVER_ID_MOLTENVK: VkDriverId = 14;
    pub const VK_DRIVER_ID_COREAVI_PROPRIETARY: VkDriverId = 15;
    pub const VK_DRIVER_ID_JUICE_PROPRIETARY: VkDriverId = 16;
    pub const VK_DRIVER_ID_VERISILICON_PROPRIETARY: VkDriverId = 17;
    pub const VK_DRIVER_ID_MESA_TURNIP: VkDriverId = 18;
    pub const VK_DRIVER_ID_MESA_V3DV: VkDriverId = 19;
    pub const VK_DRIVER_ID_MESA_PANVK: VkDriverId = 20;
    pub const VK_DRIVER_ID_SAMSUNG_PROPRIETARY: VkDriverId = 21;
    pub const VK_DRIVER_ID_MESA_VENUS: VkDriverId = 22;
    pub const VK_DRIVER_ID_MESA_DOZEN: VkDriverId = 23;
    pub const VK_DRIVER_ID_MESA_NVK: VkDriverId = 24;
    pub const VK_DRIVER_ID_IMAGINATION_OPEN_SOURCE_MESA: VkDriverId = 25;
    pub const VK_DRIVER_ID_MESA_HONEYKRISP: VkDriverId = 26;
    pub const VK_DRIVER_ID_VULKAN_SC_EMULATION_ON_VULKAN: VkDriverId = 27;
    pub const VK_DRIVER_ID_MESA_KOSMICKRISP: VkDriverId = 28;
    pub const VK_DRIVER_ID_MESA_GFXSTREAM: VkDriverId = 29;
    pub const VK_DRIVER_ID_APE_SOFT: VkDriverId = 30;
    pub const VK_DRIVER_ID_RESERVED_31: VkDriverId = 31;
}

pub type VkObjectType = i32;
pub mod VkObjectTypeValue {
    use crate::VkObjectType;
    pub const VK_OBJECT_TYPE_UNKNOWN: VkObjectType = 0;
    pub const VK_OBJECT_TYPE_INSTANCE: VkObjectType = 1;
    pub const VK_OBJECT_TYPE_PHYSICAL_DEVICE: VkObjectType = 2;
    pub const VK_OBJECT_TYPE_DEVICE: VkObjectType = 3;
    pub const VK_OBJECT_TYPE_QUEUE: VkObjectType = 4;
    pub const VK_OBJECT_TYPE_SEMAPHORE: VkObjectType = 5;
    pub const VK_OBJECT_TYPE_COMMAND_BUFFER: VkObjectType = 6;
    pub const VK_OBJECT_TYPE_FENCE: VkObjectType = 7;
    pub const VK_OBJECT_TYPE_DEVICE_MEMORY: VkObjectType = 8;
    pub const VK_OBJECT_TYPE_BUFFER: VkObjectType = 9;
    pub const VK_OBJECT_TYPE_IMAGE: VkObjectType = 10;
    pub const VK_OBJECT_TYPE_EVENT: VkObjectType = 11;
    pub const VK_OBJECT_TYPE_QUERY_POOL: VkObjectType = 12;
    pub const VK_OBJECT_TYPE_BUFFER_VIEW: VkObjectType = 13;
    pub const VK_OBJECT_TYPE_IMAGE_VIEW: VkObjectType = 14;
    pub const VK_OBJECT_TYPE_SHADER_MODULE: VkObjectType = 15;
    pub const VK_OBJECT_TYPE_PIPELINE_CACHE: VkObjectType = 16;
    pub const VK_OBJECT_TYPE_PIPELINE_LAYOUT: VkObjectType = 17;
    pub const VK_OBJECT_TYPE_RENDER_PASS: VkObjectType = 18;
    pub const VK_OBJECT_TYPE_PIPELINE: VkObjectType = 19;
    pub const VK_OBJECT_TYPE_DESCRIPTOR_SET_LAYOUT: VkObjectType = 20;
    pub const VK_OBJECT_TYPE_SAMPLER: VkObjectType = 21;
    pub const VK_OBJECT_TYPE_DESCRIPTOR_POOL: VkObjectType = 22;
    pub const VK_OBJECT_TYPE_DESCRIPTOR_SET: VkObjectType = 23;
    pub const VK_OBJECT_TYPE_FRAMEBUFFER: VkObjectType = 24;
    pub const VK_OBJECT_TYPE_COMMAND_POOL: VkObjectType = 25;
    pub const VK_OBJECT_TYPE_DESCRIPTOR_UPDATE_TEMPLATE: VkObjectType = 1000085000;
    pub const VK_OBJECT_TYPE_SAMPLER_YCBCR_CONVERSION: VkObjectType = 1000156000;
    pub const VK_OBJECT_TYPE_PRIVATE_DATA_SLOT: VkObjectType = 1000295000;
}

pub type VkDisplayPowerStateEXT = i32;
pub mod VkDisplayPowerStateEXTValue {
    use crate::VkDisplayPowerStateEXT;
    pub const VK_DISPLAY_POWER_STATE_OFF_EXT: VkDisplayPowerStateEXT = 0;
    pub const VK_DISPLAY_POWER_STATE_SUSPEND_EXT: VkDisplayPowerStateEXT = 1;
    pub const VK_DISPLAY_POWER_STATE_ON_EXT: VkDisplayPowerStateEXT = 2;
}

pub type VkPhysicalDeviceDataGraphOperationTypeARM = i32;
pub mod VkPhysicalDeviceDataGraphOperationTypeARMValue {
    use crate::VkPhysicalDeviceDataGraphOperationTypeARM;
    pub const VK_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_TYPE_SPIRV_EXTENDED_INSTRUCTION_SET_ARM: VkPhysicalDeviceDataGraphOperationTypeARM = 0;
}

pub type VkDataGraphModelCacheTypeQCOM = i32;
pub mod VkDataGraphModelCacheTypeQCOMValue {
    use crate::VkDataGraphModelCacheTypeQCOM;
    pub const VK_DATA_GRAPH_MODEL_CACHE_TYPE_GENERIC_BINARY_QCOM: VkDataGraphModelCacheTypeQCOM = 0;
}

pub type VkPipelineMatchControl = i32;
pub mod VkPipelineMatchControlValue {
    use crate::VkPipelineMatchControl;
    pub const VK_PIPELINE_MATCH_CONTROL_APPLICATION_UUID_EXACT_MATCH: VkPipelineMatchControl = 0;
}

pub type VkResult = i32;
pub mod VkResultValue {
    use crate::VkResult;
    pub const VK_SUCCESS: VkResult = 0;
    pub const VK_NOT_READY: VkResult = 1;
    pub const VK_TIMEOUT: VkResult = 2;
    pub const VK_EVENT_SET: VkResult = 3;
    pub const VK_EVENT_RESET: VkResult = 4;
    pub const VK_INCOMPLETE: VkResult = 5;
    pub const VK_ERROR_OUT_OF_HOST_MEMORY: VkResult = -1;
    pub const VK_ERROR_OUT_OF_DEVICE_MEMORY: VkResult = -2;
    pub const VK_ERROR_INITIALIZATION_FAILED: VkResult = -3;
    pub const VK_ERROR_DEVICE_LOST: VkResult = -4;
    pub const VK_ERROR_MEMORY_MAP_FAILED: VkResult = -5;
    pub const VK_ERROR_LAYER_NOT_PRESENT: VkResult = -6;
    pub const VK_ERROR_EXTENSION_NOT_PRESENT: VkResult = -7;
    pub const VK_ERROR_FEATURE_NOT_PRESENT: VkResult = -8;
    pub const VK_ERROR_INCOMPATIBLE_DRIVER: VkResult = -9;
    pub const VK_ERROR_TOO_MANY_OBJECTS: VkResult = -10;
    pub const VK_ERROR_FORMAT_NOT_SUPPORTED: VkResult = -11;
    pub const VK_ERROR_FRAGMENTED_POOL: VkResult = -12;
    pub const VK_ERROR_UNKNOWN: VkResult = -13;
    pub const VK_ERROR_VALIDATION_FAILED: VkResult = 1000010999;
    pub const VK_ERROR_OUT_OF_POOL_MEMORY: VkResult = 1000069000;
    pub const VK_ERROR_INVALID_EXTERNAL_HANDLE: VkResult = 1000071997;
    pub const VK_ERROR_INVALID_OPAQUE_CAPTURE_ADDRESS: VkResult = 1000257000;
    pub const VK_ERROR_FRAGMENTATION: VkResult = 1000161000;
    pub const VK_PIPELINE_COMPILE_REQUIRED: VkResult = 1000297000;
    pub const VK_ERROR_NOT_PERMITTED: VkResult = 1000173999;
    pub const VK_ERROR_INVALID_PIPELINE_CACHE_DATA: VkResult = 1000298000;
    pub const VK_ERROR_NO_PIPELINE_MATCH: VkResult = 1000297999;
}

pub type VkCommandBufferLevel = i32;
pub mod VkCommandBufferLevelValue {
    use crate::VkCommandBufferLevel;
    pub const VK_COMMAND_BUFFER_LEVEL_PRIMARY: VkCommandBufferLevel = 0;
    pub const VK_COMMAND_BUFFER_LEVEL_SECONDARY: VkCommandBufferLevel = 1;
}

pub type VkValidationCacheHeaderVersionEXT = i32;
pub mod VkValidationCacheHeaderVersionEXTValue {
    use crate::VkValidationCacheHeaderVersionEXT;
    pub const VK_VALIDATION_CACHE_HEADER_VERSION_ONE_EXT: VkValidationCacheHeaderVersionEXT = 1;
}

pub type VkChromaLocation = i32;
pub mod VkChromaLocationValue {
    use crate::VkChromaLocation;
    pub const VK_CHROMA_LOCATION_COSITED_EVEN: VkChromaLocation = 0;
    pub const VK_CHROMA_LOCATION_MIDPOINT: VkChromaLocation = 1;
}

pub type VkQueryResultStatusKHR = i32;
pub mod VkQueryResultStatusKHRValue {
    use crate::VkQueryResultStatusKHR;
    pub const VK_QUERY_RESULT_STATUS_ERROR_KHR: VkQueryResultStatusKHR = -1;
    pub const VK_QUERY_RESULT_STATUS_NOT_READY_KHR: VkQueryResultStatusKHR = 0;
    pub const VK_QUERY_RESULT_STATUS_COMPLETE_KHR: VkQueryResultStatusKHR = 1;
}

pub type VkShaderGroupShaderKHR = i32;
pub mod VkShaderGroupShaderKHRValue {
    use crate::VkShaderGroupShaderKHR;
    pub const VK_SHADER_GROUP_SHADER_GENERAL_KHR: VkShaderGroupShaderKHR = 0;
    pub const VK_SHADER_GROUP_SHADER_CLOSEST_HIT_KHR: VkShaderGroupShaderKHR = 1;
    pub const VK_SHADER_GROUP_SHADER_ANY_HIT_KHR: VkShaderGroupShaderKHR = 2;
    pub const VK_SHADER_GROUP_SHADER_INTERSECTION_KHR: VkShaderGroupShaderKHR = 3;
}

pub type VkRayTracingInvocationReorderModeEXT = i32;
pub mod VkRayTracingInvocationReorderModeEXTValue {
    use crate::VkRayTracingInvocationReorderModeEXT;
    pub const VK_RAY_TRACING_INVOCATION_REORDER_MODE_NONE_EXT: VkRayTracingInvocationReorderModeEXT = 0;
    pub const VK_RAY_TRACING_INVOCATION_REORDER_MODE_REORDER_EXT: VkRayTracingInvocationReorderModeEXT = 1;
}

pub type VkDeviceEventTypeEXT = i32;
pub mod VkDeviceEventTypeEXTValue {
    use crate::VkDeviceEventTypeEXT;
    pub const VK_DEVICE_EVENT_TYPE_DISPLAY_HOTPLUG_EXT: VkDeviceEventTypeEXT = 0;
}

pub type VkPerformanceConfigurationTypeINTEL = i32;
pub mod VkPerformanceConfigurationTypeINTELValue {
    use crate::VkPerformanceConfigurationTypeINTEL;
    pub const VK_PERFORMANCE_CONFIGURATION_TYPE_COMMAND_QUEUE_METRICS_DISCOVERY_ACTIVATED_INTEL: VkPerformanceConfigurationTypeINTEL = 0;
}

pub type VkCooperativeVectorMatrixLayoutNV = i32;
pub mod VkCooperativeVectorMatrixLayoutNVValue {
    use crate::VkCooperativeVectorMatrixLayoutNV;
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_ROW_MAJOR_NV: VkCooperativeVectorMatrixLayoutNV = 0;
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_COLUMN_MAJOR_NV: VkCooperativeVectorMatrixLayoutNV = 1;
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_INFERENCING_OPTIMAL_NV: VkCooperativeVectorMatrixLayoutNV = 2;
    pub const VK_COOPERATIVE_VECTOR_MATRIX_LAYOUT_TRAINING_OPTIMAL_NV: VkCooperativeVectorMatrixLayoutNV = 3;
}

pub type VkPhysicalDeviceLayeredApiKHR = i32;
pub mod VkPhysicalDeviceLayeredApiKHRValue {
    use crate::VkPhysicalDeviceLayeredApiKHR;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_VULKAN_KHR: VkPhysicalDeviceLayeredApiKHR = 0;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_D3D12_KHR: VkPhysicalDeviceLayeredApiKHR = 1;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_METAL_KHR: VkPhysicalDeviceLayeredApiKHR = 2;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_OPENGL_KHR: VkPhysicalDeviceLayeredApiKHR = 3;
    pub const VK_PHYSICAL_DEVICE_LAYERED_API_OPENGLES_KHR: VkPhysicalDeviceLayeredApiKHR = 4;
}

pub type VkPipelineCacheHeaderVersion = i32;
pub mod VkPipelineCacheHeaderVersionValue {
    use crate::VkPipelineCacheHeaderVersion;
    pub const VK_PIPELINE_CACHE_HEADER_VERSION_ONE: VkPipelineCacheHeaderVersion = 1;
    pub const VK_PIPELINE_CACHE_HEADER_VERSION_SAFETY_CRITICAL_ONE: VkPipelineCacheHeaderVersion = 1000298001;
}

pub type VkSemaphoreType = i32;
pub mod VkSemaphoreTypeValue {
    use crate::VkSemaphoreType;
    pub const VK_SEMAPHORE_TYPE_BINARY: VkSemaphoreType = 0;
    pub const VK_SEMAPHORE_TYPE_TIMELINE: VkSemaphoreType = 1;
}

pub type VkIndirectExecutionSetInfoTypeEXT = i32;
pub mod VkIndirectExecutionSetInfoTypeEXTValue {
    use crate::VkIndirectExecutionSetInfoTypeEXT;
    pub const VK_INDIRECT_EXECUTION_SET_INFO_TYPE_PIPELINES_EXT: VkIndirectExecutionSetInfoTypeEXT = 0;
    pub const VK_INDIRECT_EXECUTION_SET_INFO_TYPE_SHADER_OBJECTS_EXT: VkIndirectExecutionSetInfoTypeEXT = 1;
}

pub type VkLayeredDriverUnderlyingApiMSFT = i32;
pub mod VkLayeredDriverUnderlyingApiMSFTValue {
    use crate::VkLayeredDriverUnderlyingApiMSFT;
    pub const VK_LAYERED_DRIVER_UNDERLYING_API_NONE_MSFT: VkLayeredDriverUnderlyingApiMSFT = 0;
    pub const VK_LAYERED_DRIVER_UNDERLYING_API_D3D12_MSFT: VkLayeredDriverUnderlyingApiMSFT = 1;
}

pub type VkSubpassMergeStatusEXT = i32;
pub mod VkSubpassMergeStatusEXTValue {
    use crate::VkSubpassMergeStatusEXT;
    pub const VK_SUBPASS_MERGE_STATUS_MERGED_EXT: VkSubpassMergeStatusEXT = 0;
    pub const VK_SUBPASS_MERGE_STATUS_DISALLOWED_EXT: VkSubpassMergeStatusEXT = 1;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SIDE_EFFECTS_EXT: VkSubpassMergeStatusEXT = 2;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SAMPLES_MISMATCH_EXT: VkSubpassMergeStatusEXT = 3;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_VIEWS_MISMATCH_EXT: VkSubpassMergeStatusEXT = 4;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_ALIASING_EXT: VkSubpassMergeStatusEXT = 5;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_DEPENDENCIES_EXT: VkSubpassMergeStatusEXT = 6;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_INCOMPATIBLE_INPUT_ATTACHMENT_EXT: VkSubpassMergeStatusEXT = 7;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_TOO_MANY_ATTACHMENTS_EXT: VkSubpassMergeStatusEXT = 8;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_INSUFFICIENT_STORAGE_EXT: VkSubpassMergeStatusEXT = 9;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_DEPTH_STENCIL_COUNT_EXT: VkSubpassMergeStatusEXT = 10;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_RESOLVE_ATTACHMENT_REUSE_EXT: VkSubpassMergeStatusEXT = 11;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_SINGLE_SUBPASS_EXT: VkSubpassMergeStatusEXT = 12;
    pub const VK_SUBPASS_MERGE_STATUS_NOT_MERGED_UNSPECIFIED_EXT: VkSubpassMergeStatusEXT = 13;
}

pub type VkOpticalFlowPerformanceLevelNV = i32;
pub mod VkOpticalFlowPerformanceLevelNVValue {
    use crate::VkOpticalFlowPerformanceLevelNV;
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_UNKNOWN_NV: VkOpticalFlowPerformanceLevelNV = 0;
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_SLOW_NV: VkOpticalFlowPerformanceLevelNV = 1;
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_MEDIUM_NV: VkOpticalFlowPerformanceLevelNV = 2;
    pub const VK_OPTICAL_FLOW_PERFORMANCE_LEVEL_FAST_NV: VkOpticalFlowPerformanceLevelNV = 3;
}

pub type VkVideoEncodeTuningModeKHR = i32;
pub mod VkVideoEncodeTuningModeKHRValue {
    use crate::VkVideoEncodeTuningModeKHR;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_DEFAULT_KHR: VkVideoEncodeTuningModeKHR = 0;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_HIGH_QUALITY_KHR: VkVideoEncodeTuningModeKHR = 1;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_LOW_LATENCY_KHR: VkVideoEncodeTuningModeKHR = 2;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_ULTRA_LOW_LATENCY_KHR: VkVideoEncodeTuningModeKHR = 3;
    pub const VK_VIDEO_ENCODE_TUNING_MODE_LOSSLESS_KHR: VkVideoEncodeTuningModeKHR = 4;
}

pub type VkDepthBiasRepresentationEXT = i32;
pub mod VkDepthBiasRepresentationEXTValue {
    use crate::VkDepthBiasRepresentationEXT;
    pub const VK_DEPTH_BIAS_REPRESENTATION_LEAST_REPRESENTABLE_VALUE_FORMAT_EXT: VkDepthBiasRepresentationEXT = 0;
    pub const VK_DEPTH_BIAS_REPRESENTATION_LEAST_REPRESENTABLE_VALUE_FORCE_UNORM_EXT: VkDepthBiasRepresentationEXT = 1;
    pub const VK_DEPTH_BIAS_REPRESENTATION_FLOAT_EXT: VkDepthBiasRepresentationEXT = 2;
}

pub type VkSciSyncPrimitiveTypeNV = i32;
pub mod VkSciSyncPrimitiveTypeNVValue {
    use crate::VkSciSyncPrimitiveTypeNV;
    pub const VK_SCI_SYNC_PRIMITIVE_TYPE_FENCE_NV: VkSciSyncPrimitiveTypeNV = 0;
    pub const VK_SCI_SYNC_PRIMITIVE_TYPE_SEMAPHORE_NV: VkSciSyncPrimitiveTypeNV = 1;
}

pub type VkPointClippingBehavior = i32;
pub mod VkPointClippingBehaviorValue {
    use crate::VkPointClippingBehavior;
    pub const VK_POINT_CLIPPING_BEHAVIOR_ALL_CLIP_PLANES: VkPointClippingBehavior = 0;
    pub const VK_POINT_CLIPPING_BEHAVIOR_USER_CLIP_PLANES_ONLY: VkPointClippingBehavior = 1;
}

pub type VkBuildMicromapModeEXT = i32;
pub mod VkBuildMicromapModeEXTValue {
    use crate::VkBuildMicromapModeEXT;
    pub const VK_BUILD_MICROMAP_MODE_BUILD_EXT: VkBuildMicromapModeEXT = 0;
}

pub type VkAccelerationStructureCompatibilityKHR = i32;
pub mod VkAccelerationStructureCompatibilityKHRValue {
    use crate::VkAccelerationStructureCompatibilityKHR;
    pub const VK_ACCELERATION_STRUCTURE_COMPATIBILITY_COMPATIBLE_KHR: VkAccelerationStructureCompatibilityKHR = 0;
    pub const VK_ACCELERATION_STRUCTURE_COMPATIBILITY_INCOMPATIBLE_KHR: VkAccelerationStructureCompatibilityKHR = 1;
}

pub type VkBlendOp = i32;
pub mod VkBlendOpValue {
    use crate::VkBlendOp;
    pub const VK_BLEND_OP_ADD: VkBlendOp = 0;
    pub const VK_BLEND_OP_SUBTRACT: VkBlendOp = 1;
    pub const VK_BLEND_OP_REVERSE_SUBTRACT: VkBlendOp = 2;
    pub const VK_BLEND_OP_MIN: VkBlendOp = 3;
    pub const VK_BLEND_OP_MAX: VkBlendOp = 4;
}

pub type VkAccelerationStructureBuildTypeKHR = i32;
pub mod VkAccelerationStructureBuildTypeKHRValue {
    use crate::VkAccelerationStructureBuildTypeKHR;
    pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_KHR: VkAccelerationStructureBuildTypeKHR = 0;
    pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_DEVICE_KHR: VkAccelerationStructureBuildTypeKHR = 1;
    pub const VK_ACCELERATION_STRUCTURE_BUILD_TYPE_HOST_OR_DEVICE_KHR: VkAccelerationStructureBuildTypeKHR = 2;
}

pub type VkDisplacementMicromapFormatNV = i32;
pub mod VkDisplacementMicromapFormatNVValue {
    use crate::VkDisplacementMicromapFormatNV;
    pub const VK_DISPLACEMENT_MICROMAP_FORMAT_64_TRIANGLES_64_BYTES_NV: VkDisplacementMicromapFormatNV = 1;
    pub const VK_DISPLACEMENT_MICROMAP_FORMAT_256_TRIANGLES_128_BYTES_NV: VkDisplacementMicromapFormatNV = 2;
    pub const VK_DISPLACEMENT_MICROMAP_FORMAT_1024_TRIANGLES_128_BYTES_NV: VkDisplacementMicromapFormatNV = 3;
}

pub type VkFragmentShadingRateNV = i32;
pub mod VkFragmentShadingRateNVValue {
    use crate::VkFragmentShadingRateNV;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_PIXEL_NV: VkFragmentShadingRateNV = 0;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_1X2_PIXELS_NV: VkFragmentShadingRateNV = 1;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X1_PIXELS_NV: VkFragmentShadingRateNV = 4;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X2_PIXELS_NV: VkFragmentShadingRateNV = 5;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_2X4_PIXELS_NV: VkFragmentShadingRateNV = 6;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X2_PIXELS_NV: VkFragmentShadingRateNV = 9;
    pub const VK_FRAGMENT_SHADING_RATE_1_INVOCATION_PER_4X4_PIXELS_NV: VkFragmentShadingRateNV = 10;
    pub const VK_FRAGMENT_SHADING_RATE_2_INVOCATIONS_PER_PIXEL_NV: VkFragmentShadingRateNV = 11;
    pub const VK_FRAGMENT_SHADING_RATE_4_INVOCATIONS_PER_PIXEL_NV: VkFragmentShadingRateNV = 12;
    pub const VK_FRAGMENT_SHADING_RATE_8_INVOCATIONS_PER_PIXEL_NV: VkFragmentShadingRateNV = 13;
    pub const VK_FRAGMENT_SHADING_RATE_16_INVOCATIONS_PER_PIXEL_NV: VkFragmentShadingRateNV = 14;
    pub const VK_FRAGMENT_SHADING_RATE_NO_INVOCATIONS_NV: VkFragmentShadingRateNV = 15;
}

pub type VkDeviceMemoryReportEventTypeEXT = i32;
pub mod VkDeviceMemoryReportEventTypeEXTValue {
    use crate::VkDeviceMemoryReportEventTypeEXT;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATE_EXT: VkDeviceMemoryReportEventTypeEXT = 0;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_FREE_EXT: VkDeviceMemoryReportEventTypeEXT = 1;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_IMPORT_EXT: VkDeviceMemoryReportEventTypeEXT = 2;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_UNIMPORT_EXT: VkDeviceMemoryReportEventTypeEXT = 3;
    pub const VK_DEVICE_MEMORY_REPORT_EVENT_TYPE_ALLOCATION_FAILED_EXT: VkDeviceMemoryReportEventTypeEXT = 4;
}

pub type VkSciSyncClientTypeNV = i32;
pub mod VkSciSyncClientTypeNVValue {
    use crate::VkSciSyncClientTypeNV;
    pub const VK_SCI_SYNC_CLIENT_TYPE_SIGNALER_NV: VkSciSyncClientTypeNV = 0;
    pub const VK_SCI_SYNC_CLIENT_TYPE_WAITER_NV: VkSciSyncClientTypeNV = 1;
    pub const VK_SCI_SYNC_CLIENT_TYPE_SIGNALER_WAITER_NV: VkSciSyncClientTypeNV = 2;
}

pub type VkMicromapTypeEXT = i32;
pub mod VkMicromapTypeEXTValue {
    use crate::VkMicromapTypeEXT;
    pub const VK_MICROMAP_TYPE_OPACITY_MICROMAP_EXT: VkMicromapTypeEXT = 0;
}

pub type VkFullScreenExclusiveEXT = i32;
pub mod VkFullScreenExclusiveEXTValue {
    use crate::VkFullScreenExclusiveEXT;
    pub const VK_FULL_SCREEN_EXCLUSIVE_DEFAULT_EXT: VkFullScreenExclusiveEXT = 0;
    pub const VK_FULL_SCREEN_EXCLUSIVE_ALLOWED_EXT: VkFullScreenExclusiveEXT = 1;
    pub const VK_FULL_SCREEN_EXCLUSIVE_DISALLOWED_EXT: VkFullScreenExclusiveEXT = 2;
    pub const VK_FULL_SCREEN_EXCLUSIVE_APPLICATION_CONTROLLED_EXT: VkFullScreenExclusiveEXT = 3;
}

pub type VkThrottleHintTypeSEC = i32;
pub mod VkThrottleHintTypeSECValue {
    use crate::VkThrottleHintTypeSEC;
    pub const VK_THROTTLE_HINT_TYPE_DEFAULT_SEC: VkThrottleHintTypeSEC = 0;
    pub const VK_THROTTLE_HINT_TYPE_LOW_SEC: VkThrottleHintTypeSEC = 1;
    pub const VK_THROTTLE_HINT_TYPE_HIGH_SEC: VkThrottleHintTypeSEC = 2;
}

pub type VkDescriptorUpdateTemplateType = i32;
pub mod VkDescriptorUpdateTemplateTypeValue {
    use crate::VkDescriptorUpdateTemplateType;
    pub const VK_DESCRIPTOR_UPDATE_TEMPLATE_TYPE_DESCRIPTOR_SET: VkDescriptorUpdateTemplateType = 0;
}

pub type VkImageTiling = i32;
pub mod VkImageTilingValue {
    use crate::VkImageTiling;
    pub const VK_IMAGE_TILING_OPTIMAL: VkImageTiling = 0;
    pub const VK_IMAGE_TILING_LINEAR: VkImageTiling = 1;
}

pub type VkPipelineCacheValidationVersion = i32;
pub mod VkPipelineCacheValidationVersionValue {
    use crate::VkPipelineCacheValidationVersion;
    pub const VK_PIPELINE_CACHE_VALIDATION_VERSION_SAFETY_CRITICAL_ONE: VkPipelineCacheValidationVersion = 1;
}

pub type VkCopyMicromapModeEXT = i32;
pub mod VkCopyMicromapModeEXTValue {
    use crate::VkCopyMicromapModeEXT;
    pub const VK_COPY_MICROMAP_MODE_CLONE_EXT: VkCopyMicromapModeEXT = 0;
    pub const VK_COPY_MICROMAP_MODE_SERIALIZE_EXT: VkCopyMicromapModeEXT = 1;
    pub const VK_COPY_MICROMAP_MODE_DESERIALIZE_EXT: VkCopyMicromapModeEXT = 2;
    pub const VK_COPY_MICROMAP_MODE_COMPACT_EXT: VkCopyMicromapModeEXT = 3;
}

pub type VkOpacityMicromapFormatKHR = i32;
pub mod VkOpacityMicromapFormatKHRValue {
    use crate::VkOpacityMicromapFormatKHR;
    pub const VK_OPACITY_MICROMAP_FORMAT_2_STATE_KHR: VkOpacityMicromapFormatKHR = 1;
    pub const VK_OPACITY_MICROMAP_FORMAT_4_STATE_KHR: VkOpacityMicromapFormatKHR = 2;
}

pub type VkGpaDeviceClockModeAMD = i32;
pub mod VkGpaDeviceClockModeAMDValue {
    use crate::VkGpaDeviceClockModeAMD;
    pub const VK_GPA_DEVICE_CLOCK_MODE_DEFAULT_AMD: VkGpaDeviceClockModeAMD = 0;
    pub const VK_GPA_DEVICE_CLOCK_MODE_QUERY_AMD: VkGpaDeviceClockModeAMD = 1;
    pub const VK_GPA_DEVICE_CLOCK_MODE_PROFILING_AMD: VkGpaDeviceClockModeAMD = 2;
    pub const VK_GPA_DEVICE_CLOCK_MODE_MIN_MEMORY_AMD: VkGpaDeviceClockModeAMD = 3;
    pub const VK_GPA_DEVICE_CLOCK_MODE_MIN_ENGINE_AMD: VkGpaDeviceClockModeAMD = 4;
    pub const VK_GPA_DEVICE_CLOCK_MODE_PEAK_AMD: VkGpaDeviceClockModeAMD = 5;
}

pub type VkQueryPoolSamplingModeINTEL = i32;
pub mod VkQueryPoolSamplingModeINTELValue {
    use crate::VkQueryPoolSamplingModeINTEL;
    pub const VK_QUERY_POOL_SAMPLING_MODE_MANUAL_INTEL: VkQueryPoolSamplingModeINTEL = 0;
}

pub type VkBorderColor = i32;
pub mod VkBorderColorValue {
    use crate::VkBorderColor;
    pub const VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK: VkBorderColor = 0;
    pub const VK_BORDER_COLOR_INT_TRANSPARENT_BLACK: VkBorderColor = 1;
    pub const VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK: VkBorderColor = 2;
    pub const VK_BORDER_COLOR_INT_OPAQUE_BLACK: VkBorderColor = 3;
    pub const VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE: VkBorderColor = 4;
    pub const VK_BORDER_COLOR_INT_OPAQUE_WHITE: VkBorderColor = 5;
}

pub type VkBlendOverlapEXT = i32;
pub mod VkBlendOverlapEXTValue {
    use crate::VkBlendOverlapEXT;
    pub const VK_BLEND_OVERLAP_UNCORRELATED_EXT: VkBlendOverlapEXT = 0;
    pub const VK_BLEND_OVERLAP_DISJOINT_EXT: VkBlendOverlapEXT = 1;
    pub const VK_BLEND_OVERLAP_CONJOINT_EXT: VkBlendOverlapEXT = 2;
}

pub type VkTensorTilingARM = i32;
pub mod VkTensorTilingARMValue {
    use crate::VkTensorTilingARM;
    pub const VK_TENSOR_TILING_OPTIMAL_ARM: VkTensorTilingARM = 0;
    pub const VK_TENSOR_TILING_LINEAR_ARM: VkTensorTilingARM = 1;
}

pub type VkImageLayout = i32;
pub mod VkImageLayoutValue {
    use crate::VkImageLayout;
    pub const VK_IMAGE_LAYOUT_UNDEFINED: VkImageLayout = 0;
    pub const VK_IMAGE_LAYOUT_GENERAL: VkImageLayout = 1;
    pub const VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL: VkImageLayout = 2;
    pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout = 3;
    pub const VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout = 4;
    pub const VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL: VkImageLayout = 5;
    pub const VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL: VkImageLayout = 6;
    pub const VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL: VkImageLayout = 7;
    pub const VK_IMAGE_LAYOUT_PREINITIALIZED: VkImageLayout = 8;
    pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout = 1000117000;
    pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout = 1000117001;
    pub const VK_IMAGE_LAYOUT_DEPTH_ATTACHMENT_OPTIMAL: VkImageLayout = 1000241000;
    pub const VK_IMAGE_LAYOUT_DEPTH_READ_ONLY_OPTIMAL: VkImageLayout = 1000241001;
    pub const VK_IMAGE_LAYOUT_STENCIL_ATTACHMENT_OPTIMAL: VkImageLayout = 1000241002;
    pub const VK_IMAGE_LAYOUT_STENCIL_READ_ONLY_OPTIMAL: VkImageLayout = 1000241003;
    pub const VK_IMAGE_LAYOUT_READ_ONLY_OPTIMAL: VkImageLayout = 1000314000;
    pub const VK_IMAGE_LAYOUT_ATTACHMENT_OPTIMAL: VkImageLayout = 1000314001;
    pub const VK_IMAGE_LAYOUT_RENDERING_LOCAL_READ: VkImageLayout = 1000232000;
    pub const VK_IMAGE_LAYOUT_FRAGMENT_SHADING_RATE_ATTACHMENT_OPTIMAL_KHR: VkImageLayout = 1000164003;
}

pub type VkAccelerationStructureMemoryRequirementsTypeNV = i32;
pub mod VkAccelerationStructureMemoryRequirementsTypeNVValue {
    use crate::VkAccelerationStructureMemoryRequirementsTypeNV;
    pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_OBJECT_NV: VkAccelerationStructureMemoryRequirementsTypeNV = 0;
    pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_BUILD_SCRATCH_NV: VkAccelerationStructureMemoryRequirementsTypeNV = 1;
    pub const VK_ACCELERATION_STRUCTURE_MEMORY_REQUIREMENTS_TYPE_UPDATE_SCRATCH_NV: VkAccelerationStructureMemoryRequirementsTypeNV = 2;
}

pub type VkLineRasterizationMode = i32;
pub mod VkLineRasterizationModeValue {
    use crate::VkLineRasterizationMode;
    pub const VK_LINE_RASTERIZATION_MODE_DEFAULT: VkLineRasterizationMode = 0;
    pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR: VkLineRasterizationMode = 1;
    pub const VK_LINE_RASTERIZATION_MODE_BRESENHAM: VkLineRasterizationMode = 2;
    pub const VK_LINE_RASTERIZATION_MODE_RECTANGULAR_SMOOTH: VkLineRasterizationMode = 3;
}

pub type VkBlendFactor = i32;
pub mod VkBlendFactorValue {
    use crate::VkBlendFactor;
    pub const VK_BLEND_FACTOR_ZERO: VkBlendFactor = 0;
    pub const VK_BLEND_FACTOR_ONE: VkBlendFactor = 1;
    pub const VK_BLEND_FACTOR_SRC_COLOR: VkBlendFactor = 2;
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR: VkBlendFactor = 3;
    pub const VK_BLEND_FACTOR_DST_COLOR: VkBlendFactor = 4;
    pub const VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR: VkBlendFactor = 5;
    pub const VK_BLEND_FACTOR_SRC_ALPHA: VkBlendFactor = 6;
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA: VkBlendFactor = 7;
    pub const VK_BLEND_FACTOR_DST_ALPHA: VkBlendFactor = 8;
    pub const VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA: VkBlendFactor = 9;
    pub const VK_BLEND_FACTOR_CONSTANT_COLOR: VkBlendFactor = 10;
    pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR: VkBlendFactor = 11;
    pub const VK_BLEND_FACTOR_CONSTANT_ALPHA: VkBlendFactor = 12;
    pub const VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA: VkBlendFactor = 13;
    pub const VK_BLEND_FACTOR_SRC_ALPHA_SATURATE: VkBlendFactor = 14;
    pub const VK_BLEND_FACTOR_SRC1_COLOR: VkBlendFactor = 15;
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR: VkBlendFactor = 16;
    pub const VK_BLEND_FACTOR_SRC1_ALPHA: VkBlendFactor = 17;
    pub const VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA: VkBlendFactor = 18;
}

pub type VkPerformanceCounterUnitKHR = i32;
pub mod VkPerformanceCounterUnitKHRValue {
    use crate::VkPerformanceCounterUnitKHR;
    pub const VK_PERFORMANCE_COUNTER_UNIT_GENERIC_KHR: VkPerformanceCounterUnitKHR = 0;
    pub const VK_PERFORMANCE_COUNTER_UNIT_PERCENTAGE_KHR: VkPerformanceCounterUnitKHR = 1;
    pub const VK_PERFORMANCE_COUNTER_UNIT_NANOSECONDS_KHR: VkPerformanceCounterUnitKHR = 2;
    pub const VK_PERFORMANCE_COUNTER_UNIT_BYTES_KHR: VkPerformanceCounterUnitKHR = 3;
    pub const VK_PERFORMANCE_COUNTER_UNIT_BYTES_PER_SECOND_KHR: VkPerformanceCounterUnitKHR = 4;
    pub const VK_PERFORMANCE_COUNTER_UNIT_KELVIN_KHR: VkPerformanceCounterUnitKHR = 5;
    pub const VK_PERFORMANCE_COUNTER_UNIT_WATTS_KHR: VkPerformanceCounterUnitKHR = 6;
    pub const VK_PERFORMANCE_COUNTER_UNIT_VOLTS_KHR: VkPerformanceCounterUnitKHR = 7;
    pub const VK_PERFORMANCE_COUNTER_UNIT_AMPS_KHR: VkPerformanceCounterUnitKHR = 8;
    pub const VK_PERFORMANCE_COUNTER_UNIT_HERTZ_KHR: VkPerformanceCounterUnitKHR = 9;
    pub const VK_PERFORMANCE_COUNTER_UNIT_CYCLES_KHR: VkPerformanceCounterUnitKHR = 10;
}

pub type VkPresentModeKHR = i32;
pub mod VkPresentModeKHRValue {
    use crate::VkPresentModeKHR;
    pub const VK_PRESENT_MODE_IMMEDIATE_KHR: VkPresentModeKHR = 0;
    pub const VK_PRESENT_MODE_MAILBOX_KHR: VkPresentModeKHR = 1;
    pub const VK_PRESENT_MODE_FIFO_KHR: VkPresentModeKHR = 2;
    pub const VK_PRESENT_MODE_FIFO_RELAXED_KHR: VkPresentModeKHR = 3;
    pub const VK_PRESENT_MODE_FIFO_LATEST_READY_KHR: VkPresentModeKHR = 1000361000;
}

pub type VkCubicFilterWeightsQCOM = i32;
pub mod VkCubicFilterWeightsQCOMValue {
    use crate::VkCubicFilterWeightsQCOM;
    pub const VK_CUBIC_FILTER_WEIGHTS_CATMULL_ROM_QCOM: VkCubicFilterWeightsQCOM = 0;
    pub const VK_CUBIC_FILTER_WEIGHTS_ZERO_TANGENT_CARDINAL_QCOM: VkCubicFilterWeightsQCOM = 1;
    pub const VK_CUBIC_FILTER_WEIGHTS_B_SPLINE_QCOM: VkCubicFilterWeightsQCOM = 2;
    pub const VK_CUBIC_FILTER_WEIGHTS_MITCHELL_NETRAVALI_QCOM: VkCubicFilterWeightsQCOM = 3;
}

pub type VkPerfHintTypeQCOM = i32;
pub mod VkPerfHintTypeQCOMValue {
    use crate::VkPerfHintTypeQCOM;
    pub const VK_PERF_HINT_TYPE_DEFAULT_QCOM: VkPerfHintTypeQCOM = 0;
    pub const VK_PERF_HINT_TYPE_FREQUENCY_MIN_QCOM: VkPerfHintTypeQCOM = 1;
    pub const VK_PERF_HINT_TYPE_FREQUENCY_MAX_QCOM: VkPerfHintTypeQCOM = 2;
    pub const VK_PERF_HINT_TYPE_FREQUENCY_SCALED_QCOM: VkPerfHintTypeQCOM = 3;
}

pub type VkOpticalFlowSessionBindingPointNV = i32;
pub mod VkOpticalFlowSessionBindingPointNVValue {
    use crate::VkOpticalFlowSessionBindingPointNV;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_UNKNOWN_NV: VkOpticalFlowSessionBindingPointNV = 0;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_INPUT_NV: VkOpticalFlowSessionBindingPointNV = 1;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_REFERENCE_NV: VkOpticalFlowSessionBindingPointNV = 2;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_HINT_NV: VkOpticalFlowSessionBindingPointNV = 3;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_FLOW_VECTOR_NV: VkOpticalFlowSessionBindingPointNV = 4;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_FLOW_VECTOR_NV: VkOpticalFlowSessionBindingPointNV = 5;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_COST_NV: VkOpticalFlowSessionBindingPointNV = 6;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_BACKWARD_COST_NV: VkOpticalFlowSessionBindingPointNV = 7;
    pub const VK_OPTICAL_FLOW_SESSION_BINDING_POINT_GLOBAL_FLOW_NV: VkOpticalFlowSessionBindingPointNV = 8;
}

pub type VkDirectDriverLoadingModeLUNARG = i32;
pub mod VkDirectDriverLoadingModeLUNARGValue {
    use crate::VkDirectDriverLoadingModeLUNARG;
    pub const VK_DIRECT_DRIVER_LOADING_MODE_EXCLUSIVE_LUNARG: VkDirectDriverLoadingModeLUNARG = 0;
    pub const VK_DIRECT_DRIVER_LOADING_MODE_INCLUSIVE_LUNARG: VkDirectDriverLoadingModeLUNARG = 1;
}

pub type VkDeviceAddressBindingTypeEXT = i32;
pub mod VkDeviceAddressBindingTypeEXTValue {
    use crate::VkDeviceAddressBindingTypeEXT;
    pub const VK_DEVICE_ADDRESS_BINDING_TYPE_BIND_EXT: VkDeviceAddressBindingTypeEXT = 0;
    pub const VK_DEVICE_ADDRESS_BINDING_TYPE_UNBIND_EXT: VkDeviceAddressBindingTypeEXT = 1;
}

pub type VkBlockMatchWindowCompareModeQCOM = i32;
pub mod VkBlockMatchWindowCompareModeQCOMValue {
    use crate::VkBlockMatchWindowCompareModeQCOM;
    pub const VK_BLOCK_MATCH_WINDOW_COMPARE_MODE_MIN_QCOM: VkBlockMatchWindowCompareModeQCOM = 0;
    pub const VK_BLOCK_MATCH_WINDOW_COMPARE_MODE_MAX_QCOM: VkBlockMatchWindowCompareModeQCOM = 1;
}

pub type VkSamplerMipmapMode = i32;
pub mod VkSamplerMipmapModeValue {
    use crate::VkSamplerMipmapMode;
    pub const VK_SAMPLER_MIPMAP_MODE_NEAREST: VkSamplerMipmapMode = 0;
    pub const VK_SAMPLER_MIPMAP_MODE_LINEAR: VkSamplerMipmapMode = 1;
}

pub type VkPhysicalDeviceDataGraphProcessingEngineTypeARM = i32;
pub mod VkPhysicalDeviceDataGraphProcessingEngineTypeARMValue {
    use crate::VkPhysicalDeviceDataGraphProcessingEngineTypeARM;
    pub const VK_PHYSICAL_DEVICE_DATA_GRAPH_PROCESSING_ENGINE_TYPE_DEFAULT_ARM: VkPhysicalDeviceDataGraphProcessingEngineTypeARM = 0;
}

pub type VkAntiLagModeAMD = i32;
pub mod VkAntiLagModeAMDValue {
    use crate::VkAntiLagModeAMD;
    pub const VK_ANTI_LAG_MODE_DRIVER_CONTROL_AMD: VkAntiLagModeAMD = 0;
    pub const VK_ANTI_LAG_MODE_ON_AMD: VkAntiLagModeAMD = 1;
    pub const VK_ANTI_LAG_MODE_OFF_AMD: VkAntiLagModeAMD = 2;
}

pub type VkProvokingVertexModeEXT = i32;
pub mod VkProvokingVertexModeEXTValue {
    use crate::VkProvokingVertexModeEXT;
    pub const VK_PROVOKING_VERTEX_MODE_FIRST_VERTEX_EXT: VkProvokingVertexModeEXT = 0;
    pub const VK_PROVOKING_VERTEX_MODE_LAST_VERTEX_EXT: VkProvokingVertexModeEXT = 1;
}

pub type VkShadingRatePaletteEntryNV = i32;
pub mod VkShadingRatePaletteEntryNVValue {
    use crate::VkShadingRatePaletteEntryNV;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_NO_INVOCATIONS_NV: VkShadingRatePaletteEntryNV = 0;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV: VkShadingRatePaletteEntryNV = 1;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_8_INVOCATIONS_PER_PIXEL_NV: VkShadingRatePaletteEntryNV = 2;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_4_INVOCATIONS_PER_PIXEL_NV: VkShadingRatePaletteEntryNV = 3;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_2_INVOCATIONS_PER_PIXEL_NV: VkShadingRatePaletteEntryNV = 4;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_PIXEL_NV: VkShadingRatePaletteEntryNV = 5;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X1_PIXELS_NV: VkShadingRatePaletteEntryNV = 6;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_1X2_PIXELS_NV: VkShadingRatePaletteEntryNV = 7;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X2_PIXELS_NV: VkShadingRatePaletteEntryNV = 8;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X2_PIXELS_NV: VkShadingRatePaletteEntryNV = 9;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_2X4_PIXELS_NV: VkShadingRatePaletteEntryNV = 10;
    pub const VK_SHADING_RATE_PALETTE_ENTRY_1_INVOCATION_PER_4X4_PIXELS_NV: VkShadingRatePaletteEntryNV = 11;
}

pub type VkAttachmentLoadOp = i32;
pub mod VkAttachmentLoadOpValue {
    use crate::VkAttachmentLoadOp;
    pub const VK_ATTACHMENT_LOAD_OP_LOAD: VkAttachmentLoadOp = 0;
    pub const VK_ATTACHMENT_LOAD_OP_CLEAR: VkAttachmentLoadOp = 1;
    pub const VK_ATTACHMENT_LOAD_OP_DONT_CARE: VkAttachmentLoadOp = 2;
    pub const VK_ATTACHMENT_LOAD_OP_NONE: VkAttachmentLoadOp = 1000400000;
}

pub type VkVideoEncodeAV1RateControlGroupKHR = i32;
pub mod VkVideoEncodeAV1RateControlGroupKHRValue {
    use crate::VkVideoEncodeAV1RateControlGroupKHR;
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_INTRA_KHR: VkVideoEncodeAV1RateControlGroupKHR = 0;
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_PREDICTIVE_KHR: VkVideoEncodeAV1RateControlGroupKHR = 1;
    pub const VK_VIDEO_ENCODE_AV1_RATE_CONTROL_GROUP_BIPREDICTIVE_KHR: VkVideoEncodeAV1RateControlGroupKHR = 2;
}

pub type VkQueryType = i32;
pub mod VkQueryTypeValue {
    use crate::VkQueryType;
    pub const VK_QUERY_TYPE_OCCLUSION: VkQueryType = 0;
    pub const VK_QUERY_TYPE_PIPELINE_STATISTICS: VkQueryType = 1;
    pub const VK_QUERY_TYPE_TIMESTAMP: VkQueryType = 2;
}

pub type VkColorSpaceKHR = i32;
pub mod VkColorSpaceKHRValue {
    use crate::VkColorSpaceKHR;
    pub const VK_COLOR_SPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = 0;
    pub const VK_COLORSPACE_SRGB_NONLINEAR_KHR: VkColorSpaceKHR = VK_COLOR_SPACE_SRGB_NONLINEAR_KHR;
}

pub type VkDataGraphPipelineNodeConnectionTypeARM = i32;
pub mod VkDataGraphPipelineNodeConnectionTypeARMValue {
    use crate::VkDataGraphPipelineNodeConnectionTypeARM;
}

pub type VkImageType = i32;
pub mod VkImageTypeValue {
    use crate::VkImageType;
    pub const VK_IMAGE_TYPE_1D: VkImageType = 0;
    pub const VK_IMAGE_TYPE_2D: VkImageType = 1;
    pub const VK_IMAGE_TYPE_3D: VkImageType = 2;
}

pub type VkDynamicState = i32;
pub mod VkDynamicStateValue {
    use crate::VkDynamicState;
    pub const VK_DYNAMIC_STATE_VIEWPORT: VkDynamicState = 0;
    pub const VK_DYNAMIC_STATE_SCISSOR: VkDynamicState = 1;
    pub const VK_DYNAMIC_STATE_LINE_WIDTH: VkDynamicState = 2;
    pub const VK_DYNAMIC_STATE_DEPTH_BIAS: VkDynamicState = 3;
    pub const VK_DYNAMIC_STATE_BLEND_CONSTANTS: VkDynamicState = 4;
    pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS: VkDynamicState = 5;
    pub const VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK: VkDynamicState = 6;
    pub const VK_DYNAMIC_STATE_STENCIL_WRITE_MASK: VkDynamicState = 7;
    pub const VK_DYNAMIC_STATE_STENCIL_REFERENCE: VkDynamicState = 8;
    pub const VK_DYNAMIC_STATE_CULL_MODE: VkDynamicState = 1000267000;
    pub const VK_DYNAMIC_STATE_FRONT_FACE: VkDynamicState = 1000267001;
    pub const VK_DYNAMIC_STATE_PRIMITIVE_TOPOLOGY: VkDynamicState = 1000267002;
    pub const VK_DYNAMIC_STATE_VIEWPORT_WITH_COUNT: VkDynamicState = 1000267003;
    pub const VK_DYNAMIC_STATE_SCISSOR_WITH_COUNT: VkDynamicState = 1000267004;
    pub const VK_DYNAMIC_STATE_VERTEX_INPUT_BINDING_STRIDE: VkDynamicState = 1000267005;
    pub const VK_DYNAMIC_STATE_DEPTH_TEST_ENABLE: VkDynamicState = 1000267006;
    pub const VK_DYNAMIC_STATE_DEPTH_WRITE_ENABLE: VkDynamicState = 1000267007;
    pub const VK_DYNAMIC_STATE_DEPTH_COMPARE_OP: VkDynamicState = 1000267008;
    pub const VK_DYNAMIC_STATE_DEPTH_BOUNDS_TEST_ENABLE: VkDynamicState = 1000267009;
    pub const VK_DYNAMIC_STATE_STENCIL_TEST_ENABLE: VkDynamicState = 1000267010;
    pub const VK_DYNAMIC_STATE_STENCIL_OP: VkDynamicState = 1000267011;
    pub const VK_DYNAMIC_STATE_RASTERIZER_DISCARD_ENABLE: VkDynamicState = 1000377001;
    pub const VK_DYNAMIC_STATE_DEPTH_BIAS_ENABLE: VkDynamicState = 1000377002;
    pub const VK_DYNAMIC_STATE_PRIMITIVE_RESTART_ENABLE: VkDynamicState = 1000377004;
    pub const VK_DYNAMIC_STATE_LINE_STIPPLE: VkDynamicState = 1000259000;
}

pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
pub const VK_UUID_SIZE: u32 = 16;
pub const VK_LUID_SIZE: u32 = 8;
pub const VK_MAX_EXTENSION_NAME_SIZE: u32 = 256;
pub const VK_MAX_DESCRIPTION_SIZE: u32 = 256;
pub const VK_MAX_MEMORY_TYPES: u32 = 32;
pub const VK_MAX_MEMORY_HEAPS: u32 = 16;
pub const VK_LOD_CLAMP_NONE: f32 = 1000.0F;
pub const VK_REMAINING_MIP_LEVELS: u32 = u32::MAX;
pub const VK_REMAINING_ARRAY_LAYERS: u32 = u32::MAX;
pub const VK_REMAINING_3D_SLICES_EXT: u32 = u32::MAX;
pub const VK_WHOLE_SIZE: u64 = u64::MAX;
pub const VK_ATTACHMENT_UNUSED: u32 = u32::MAX;
pub const VK_TRUE: u32 = 1;
pub const VK_FALSE: u32 = 0;
pub const VK_QUEUE_FAMILY_IGNORED: u32 = u32::MAX;
pub const VK_QUEUE_FAMILY_EXTERNAL: u32 = !1u32;
pub const VK_QUEUE_FAMILY_FOREIGN_EXT: u32 = !2u32;
pub const VK_SUBPASS_EXTERNAL: u32 = u32::MAX;
pub const VK_MAX_DEVICE_GROUP_SIZE: u32 = 32;
pub const VK_MAX_DRIVER_NAME_SIZE: u32 = 256;
pub const VK_MAX_DRIVER_INFO_SIZE: u32 = 256;
pub const VK_SHADER_UNUSED_KHR: u32 = u32::MAX;
pub const VK_MAX_GLOBAL_PRIORITY_SIZE: u32 = 16;
pub const VK_MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: u32 = 32;
pub const VK_MAX_PIPELINE_BINARY_KEY_SIZE_KHR: u32 = 32;
pub const VK_MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR: u32 = 7;
pub const VK_MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR: u32 = 3;
pub const VK_SHADER_INDEX_UNUSED_AMDX: u32 = u32::MAX;
pub const VK_PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV: u32 = u32::MAX;
pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX: u32 = 128;
pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX: u32 = 128;
pub const VK_MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM: u32 = 128;
pub const VK_DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM: u32 = 3;
pub const VK_COMPUTE_OCCUPANCY_PRIORITY_LOW_NV: f32 = 0.25f;
pub const VK_COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV: f32 = 0.50f;
pub const VK_COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV: f32 = 0.75f;
pub const VK_MAX_DATA_GRAPH_TOSA_NAME_SIZE_ARM: u32 = 128;
pub const VK_MAX_TENSOR_CREATE_INFO_ROLLING_BACKING_WRAP_COUNT_ARM: u32 = 4;


pub type VkCompareOp = i32;
pub mod VkCompareOpValue {
    use crate::VkCompareOp;
    pub const VK_COMPARE_OP_NEVER: VkCompareOp = 0;
    pub const VK_COMPARE_OP_LESS: VkCompareOp = 1;
    pub const VK_COMPARE_OP_EQUAL: VkCompareOp = 2;
    pub const VK_COMPARE_OP_LESS_OR_EQUAL: VkCompareOp = 3;
    pub const VK_COMPARE_OP_GREATER: VkCompareOp = 4;
    pub const VK_COMPARE_OP_NOT_EQUAL: VkCompareOp = 5;
    pub const VK_COMPARE_OP_GREATER_OR_EQUAL: VkCompareOp = 6;
    pub const VK_COMPARE_OP_ALWAYS: VkCompareOp = 7;
}

pub type VkViewportCoordinateSwizzleNV = i32;
pub mod VkViewportCoordinateSwizzleNVValue {
    use crate::VkViewportCoordinateSwizzleNV;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_X_NV: VkViewportCoordinateSwizzleNV = 0;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_X_NV: VkViewportCoordinateSwizzleNV = 1;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Y_NV: VkViewportCoordinateSwizzleNV = 2;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Y_NV: VkViewportCoordinateSwizzleNV = 3;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_Z_NV: VkViewportCoordinateSwizzleNV = 4;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_Z_NV: VkViewportCoordinateSwizzleNV = 5;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_POSITIVE_W_NV: VkViewportCoordinateSwizzleNV = 6;
    pub const VK_VIEWPORT_COORDINATE_SWIZZLE_NEGATIVE_W_NV: VkViewportCoordinateSwizzleNV = 7;
}

pub type VkAntiLagStageAMD = i32;
pub mod VkAntiLagStageAMDValue {
    use crate::VkAntiLagStageAMD;
    pub const VK_ANTI_LAG_STAGE_INPUT_AMD: VkAntiLagStageAMD = 0;
    pub const VK_ANTI_LAG_STAGE_PRESENT_AMD: VkAntiLagStageAMD = 1;
}

pub type VkDeviceFaultVendorBinaryHeaderVersionKHR = i32;
pub mod VkDeviceFaultVendorBinaryHeaderVersionKHRValue {
    use crate::VkDeviceFaultVendorBinaryHeaderVersionKHR;
    pub const VK_DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_ONE_KHR: VkDeviceFaultVendorBinaryHeaderVersionKHR = 1;
    pub const VK_DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_ONE_EXT: VkDeviceFaultVendorBinaryHeaderVersionKHR = VK_DEVICE_FAULT_VENDOR_BINARY_HEADER_VERSION_ONE_KHR;
}

pub type VkPolygonMode = i32;
pub mod VkPolygonModeValue {
    use crate::VkPolygonMode;
    pub const VK_POLYGON_MODE_FILL: VkPolygonMode = 0;
    pub const VK_POLYGON_MODE_LINE: VkPolygonMode = 1;
    pub const VK_POLYGON_MODE_POINT: VkPolygonMode = 2;
}

pub type VkAttachmentStoreOp = i32;
pub mod VkAttachmentStoreOpValue {
    use crate::VkAttachmentStoreOp;
    pub const VK_ATTACHMENT_STORE_OP_STORE: VkAttachmentStoreOp = 0;
    pub const VK_ATTACHMENT_STORE_OP_DONT_CARE: VkAttachmentStoreOp = 1;
    pub const VK_ATTACHMENT_STORE_OP_NONE: VkAttachmentStoreOp = 1000301000;
}

pub type VkPerformanceValueTypeINTEL = i32;
pub mod VkPerformanceValueTypeINTELValue {
    use crate::VkPerformanceValueTypeINTEL;
    pub const VK_PERFORMANCE_VALUE_TYPE_UINT32_INTEL: VkPerformanceValueTypeINTEL = 0;
    pub const VK_PERFORMANCE_VALUE_TYPE_UINT64_INTEL: VkPerformanceValueTypeINTEL = 1;
    pub const VK_PERFORMANCE_VALUE_TYPE_FLOAT_INTEL: VkPerformanceValueTypeINTEL = 2;
    pub const VK_PERFORMANCE_VALUE_TYPE_BOOL_INTEL: VkPerformanceValueTypeINTEL = 3;
    pub const VK_PERFORMANCE_VALUE_TYPE_STRING_INTEL: VkPerformanceValueTypeINTEL = 4;
}

pub type VkVideoEncodeAV1PredictionModeKHR = i32;
pub mod VkVideoEncodeAV1PredictionModeKHRValue {
    use crate::VkVideoEncodeAV1PredictionModeKHR;
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_INTRA_ONLY_KHR: VkVideoEncodeAV1PredictionModeKHR = 0;
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_SINGLE_REFERENCE_KHR: VkVideoEncodeAV1PredictionModeKHR = 1;
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_UNIDIRECTIONAL_COMPOUND_KHR: VkVideoEncodeAV1PredictionModeKHR = 2;
    pub const VK_VIDEO_ENCODE_AV1_PREDICTION_MODE_BIDIRECTIONAL_COMPOUND_KHR: VkVideoEncodeAV1PredictionModeKHR = 3;
}

pub type VkCoverageModulationModeNV = i32;
pub mod VkCoverageModulationModeNVValue {
    use crate::VkCoverageModulationModeNV;
    pub const VK_COVERAGE_MODULATION_MODE_NONE_NV: VkCoverageModulationModeNV = 0;
    pub const VK_COVERAGE_MODULATION_MODE_RGB_NV: VkCoverageModulationModeNV = 1;
    pub const VK_COVERAGE_MODULATION_MODE_ALPHA_NV: VkCoverageModulationModeNV = 2;
    pub const VK_COVERAGE_MODULATION_MODE_RGBA_NV: VkCoverageModulationModeNV = 3;
}

pub type VkFrontFace = i32;
pub mod VkFrontFaceValue {
    use crate::VkFrontFace;
    pub const VK_FRONT_FACE_COUNTER_CLOCKWISE: VkFrontFace = 0;
    pub const VK_FRONT_FACE_CLOCKWISE: VkFrontFace = 1;
}

pub type VkCompressedTriangleFormatAMDX = i32;
pub mod VkCompressedTriangleFormatAMDXValue {
    use crate::VkCompressedTriangleFormatAMDX;
    pub const VK_COMPRESSED_TRIANGLE_FORMAT_DGF1_AMDX: VkCompressedTriangleFormatAMDX = 0;
}

pub type VkRayTracingShaderGroupTypeKHR = i32;
pub mod VkRayTracingShaderGroupTypeKHRValue {
    use crate::VkRayTracingShaderGroupTypeKHR;
    pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_GENERAL_KHR: VkRayTracingShaderGroupTypeKHR = 0;
    pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_TRIANGLES_HIT_GROUP_KHR: VkRayTracingShaderGroupTypeKHR = 1;
    pub const VK_RAY_TRACING_SHADER_GROUP_TYPE_PROCEDURAL_HIT_GROUP_KHR: VkRayTracingShaderGroupTypeKHR = 2;
}

pub type VkPerformanceOverrideTypeINTEL = i32;
pub mod VkPerformanceOverrideTypeINTELValue {
    use crate::VkPerformanceOverrideTypeINTEL;
    pub const VK_PERFORMANCE_OVERRIDE_TYPE_NULL_HARDWARE_INTEL: VkPerformanceOverrideTypeINTEL = 0;
    pub const VK_PERFORMANCE_OVERRIDE_TYPE_FLUSH_GPU_CACHES_INTEL: VkPerformanceOverrideTypeINTEL = 1;
}

pub type VkFormat = i32;
pub mod VkFormatValue {
    use crate::VkFormat;
    pub const VK_FORMAT_UNDEFINED: VkFormat = 0;
    pub const VK_FORMAT_R4G4_UNORM_PACK8: VkFormat = 1;
    pub const VK_FORMAT_R4G4B4A4_UNORM_PACK16: VkFormat = 2;
    pub const VK_FORMAT_B4G4R4A4_UNORM_PACK16: VkFormat = 3;
    pub const VK_FORMAT_R5G6B5_UNORM_PACK16: VkFormat = 4;
    pub const VK_FORMAT_B5G6R5_UNORM_PACK16: VkFormat = 5;
    pub const VK_FORMAT_R5G5B5A1_UNORM_PACK16: VkFormat = 6;
    pub const VK_FORMAT_B5G5R5A1_UNORM_PACK16: VkFormat = 7;
    pub const VK_FORMAT_A1R5G5B5_UNORM_PACK16: VkFormat = 8;
    pub const VK_FORMAT_R8_UNORM: VkFormat = 9;
    pub const VK_FORMAT_R8_SNORM: VkFormat = 10;
    pub const VK_FORMAT_R8_USCALED: VkFormat = 11;
    pub const VK_FORMAT_R8_SSCALED: VkFormat = 12;
    pub const VK_FORMAT_R8_UINT: VkFormat = 13;
    pub const VK_FORMAT_R8_SINT: VkFormat = 14;
    pub const VK_FORMAT_R8_SRGB: VkFormat = 15;
    pub const VK_FORMAT_R8G8_UNORM: VkFormat = 16;
    pub const VK_FORMAT_R8G8_SNORM: VkFormat = 17;
    pub const VK_FORMAT_R8G8_USCALED: VkFormat = 18;
    pub const VK_FORMAT_R8G8_SSCALED: VkFormat = 19;
    pub const VK_FORMAT_R8G8_UINT: VkFormat = 20;
    pub const VK_FORMAT_R8G8_SINT: VkFormat = 21;
    pub const VK_FORMAT_R8G8_SRGB: VkFormat = 22;
    pub const VK_FORMAT_R8G8B8_UNORM: VkFormat = 23;
    pub const VK_FORMAT_R8G8B8_SNORM: VkFormat = 24;
    pub const VK_FORMAT_R8G8B8_USCALED: VkFormat = 25;
    pub const VK_FORMAT_R8G8B8_SSCALED: VkFormat = 26;
    pub const VK_FORMAT_R8G8B8_UINT: VkFormat = 27;
    pub const VK_FORMAT_R8G8B8_SINT: VkFormat = 28;
    pub const VK_FORMAT_R8G8B8_SRGB: VkFormat = 29;
    pub const VK_FORMAT_B8G8R8_UNORM: VkFormat = 30;
    pub const VK_FORMAT_B8G8R8_SNORM: VkFormat = 31;
    pub const VK_FORMAT_B8G8R8_USCALED: VkFormat = 32;
    pub const VK_FORMAT_B8G8R8_SSCALED: VkFormat = 33;
    pub const VK_FORMAT_B8G8R8_UINT: VkFormat = 34;
    pub const VK_FORMAT_B8G8R8_SINT: VkFormat = 35;
    pub const VK_FORMAT_B8G8R8_SRGB: VkFormat = 36;
    pub const VK_FORMAT_R8G8B8A8_UNORM: VkFormat = 37;
    pub const VK_FORMAT_R8G8B8A8_SNORM: VkFormat = 38;
    pub const VK_FORMAT_R8G8B8A8_USCALED: VkFormat = 39;
    pub const VK_FORMAT_R8G8B8A8_SSCALED: VkFormat = 40;
    pub const VK_FORMAT_R8G8B8A8_UINT: VkFormat = 41;
    pub const VK_FORMAT_R8G8B8A8_SINT: VkFormat = 42;
    pub const VK_FORMAT_R8G8B8A8_SRGB: VkFormat = 43;
    pub const VK_FORMAT_B8G8R8A8_UNORM: VkFormat = 44;
    pub const VK_FORMAT_B8G8R8A8_SNORM: VkFormat = 45;
    pub const VK_FORMAT_B8G8R8A8_USCALED: VkFormat = 46;
    pub const VK_FORMAT_B8G8R8A8_SSCALED: VkFormat = 47;
    pub const VK_FORMAT_B8G8R8A8_UINT: VkFormat = 48;
    pub const VK_FORMAT_B8G8R8A8_SINT: VkFormat = 49;
    pub const VK_FORMAT_B8G8R8A8_SRGB: VkFormat = 50;
    pub const VK_FORMAT_A8B8G8R8_UNORM_PACK32: VkFormat = 51;
    pub const VK_FORMAT_A8B8G8R8_SNORM_PACK32: VkFormat = 52;
    pub const VK_FORMAT_A8B8G8R8_USCALED_PACK32: VkFormat = 53;
    pub const VK_FORMAT_A8B8G8R8_SSCALED_PACK32: VkFormat = 54;
    pub const VK_FORMAT_A8B8G8R8_UINT_PACK32: VkFormat = 55;
    pub const VK_FORMAT_A8B8G8R8_SINT_PACK32: VkFormat = 56;
    pub const VK_FORMAT_A8B8G8R8_SRGB_PACK32: VkFormat = 57;
    pub const VK_FORMAT_A2R10G10B10_UNORM_PACK32: VkFormat = 58;
    pub const VK_FORMAT_A2R10G10B10_SNORM_PACK32: VkFormat = 59;
    pub const VK_FORMAT_A2R10G10B10_USCALED_PACK32: VkFormat = 60;
    pub const VK_FORMAT_A2R10G10B10_SSCALED_PACK32: VkFormat = 61;
    pub const VK_FORMAT_A2R10G10B10_UINT_PACK32: VkFormat = 62;
    pub const VK_FORMAT_A2R10G10B10_SINT_PACK32: VkFormat = 63;
    pub const VK_FORMAT_A2B10G10R10_UNORM_PACK32: VkFormat = 64;
    pub const VK_FORMAT_A2B10G10R10_SNORM_PACK32: VkFormat = 65;
    pub const VK_FORMAT_A2B10G10R10_USCALED_PACK32: VkFormat = 66;
    pub const VK_FORMAT_A2B10G10R10_SSCALED_PACK32: VkFormat = 67;
    pub const VK_FORMAT_A2B10G10R10_UINT_PACK32: VkFormat = 68;
    pub const VK_FORMAT_A2B10G10R10_SINT_PACK32: VkFormat = 69;
    pub const VK_FORMAT_R16_UNORM: VkFormat = 70;
    pub const VK_FORMAT_R16_SNORM: VkFormat = 71;
    pub const VK_FORMAT_R16_USCALED: VkFormat = 72;
    pub const VK_FORMAT_R16_SSCALED: VkFormat = 73;
    pub const VK_FORMAT_R16_UINT: VkFormat = 74;
    pub const VK_FORMAT_R16_SINT: VkFormat = 75;
    pub const VK_FORMAT_R16_SFLOAT: VkFormat = 76;
    pub const VK_FORMAT_R16G16_UNORM: VkFormat = 77;
    pub const VK_FORMAT_R16G16_SNORM: VkFormat = 78;
    pub const VK_FORMAT_R16G16_USCALED: VkFormat = 79;
    pub const VK_FORMAT_R16G16_SSCALED: VkFormat = 80;
    pub const VK_FORMAT_R16G16_UINT: VkFormat = 81;
    pub const VK_FORMAT_R16G16_SINT: VkFormat = 82;
    pub const VK_FORMAT_R16G16_SFLOAT: VkFormat = 83;
    pub const VK_FORMAT_R16G16B16_UNORM: VkFormat = 84;
    pub const VK_FORMAT_R16G16B16_SNORM: VkFormat = 85;
    pub const VK_FORMAT_R16G16B16_USCALED: VkFormat = 86;
    pub const VK_FORMAT_R16G16B16_SSCALED: VkFormat = 87;
    pub const VK_FORMAT_R16G16B16_UINT: VkFormat = 88;
    pub const VK_FORMAT_R16G16B16_SINT: VkFormat = 89;
    pub const VK_FORMAT_R16G16B16_SFLOAT: VkFormat = 90;
    pub const VK_FORMAT_R16G16B16A16_UNORM: VkFormat = 91;
    pub const VK_FORMAT_R16G16B16A16_SNORM: VkFormat = 92;
    pub const VK_FORMAT_R16G16B16A16_USCALED: VkFormat = 93;
    pub const VK_FORMAT_R16G16B16A16_SSCALED: VkFormat = 94;
    pub const VK_FORMAT_R16G16B16A16_UINT: VkFormat = 95;
    pub const VK_FORMAT_R16G16B16A16_SINT: VkFormat = 96;
    pub const VK_FORMAT_R16G16B16A16_SFLOAT: VkFormat = 97;
    pub const VK_FORMAT_R32_UINT: VkFormat = 98;
    pub const VK_FORMAT_R32_SINT: VkFormat = 99;
    pub const VK_FORMAT_R32_SFLOAT: VkFormat = 100;
    pub const VK_FORMAT_R32G32_UINT: VkFormat = 101;
    pub const VK_FORMAT_R32G32_SINT: VkFormat = 102;
    pub const VK_FORMAT_R32G32_SFLOAT: VkFormat = 103;
    pub const VK_FORMAT_R32G32B32_UINT: VkFormat = 104;
    pub const VK_FORMAT_R32G32B32_SINT: VkFormat = 105;
    pub const VK_FORMAT_R32G32B32_SFLOAT: VkFormat = 106;
    pub const VK_FORMAT_R32G32B32A32_UINT: VkFormat = 107;
    pub const VK_FORMAT_R32G32B32A32_SINT: VkFormat = 108;
    pub const VK_FORMAT_R32G32B32A32_SFLOAT: VkFormat = 109;
    pub const VK_FORMAT_R64_UINT: VkFormat = 110;
    pub const VK_FORMAT_R64_SINT: VkFormat = 111;
    pub const VK_FORMAT_R64_SFLOAT: VkFormat = 112;
    pub const VK_FORMAT_R64G64_UINT: VkFormat = 113;
    pub const VK_FORMAT_R64G64_SINT: VkFormat = 114;
    pub const VK_FORMAT_R64G64_SFLOAT: VkFormat = 115;
    pub const VK_FORMAT_R64G64B64_UINT: VkFormat = 116;
    pub const VK_FORMAT_R64G64B64_SINT: VkFormat = 117;
    pub const VK_FORMAT_R64G64B64_SFLOAT: VkFormat = 118;
    pub const VK_FORMAT_R64G64B64A64_UINT: VkFormat = 119;
    pub const VK_FORMAT_R64G64B64A64_SINT: VkFormat = 120;
    pub const VK_FORMAT_R64G64B64A64_SFLOAT: VkFormat = 121;
    pub const VK_FORMAT_B10G11R11_UFLOAT_PACK32: VkFormat = 122;
    pub const VK_FORMAT_E5B9G9R9_UFLOAT_PACK32: VkFormat = 123;
    pub const VK_FORMAT_D16_UNORM: VkFormat = 124;
    pub const VK_FORMAT_X8_D24_UNORM_PACK32: VkFormat = 125;
    pub const VK_FORMAT_D32_SFLOAT: VkFormat = 126;
    pub const VK_FORMAT_S8_UINT: VkFormat = 127;
    pub const VK_FORMAT_D16_UNORM_S8_UINT: VkFormat = 128;
    pub const VK_FORMAT_D24_UNORM_S8_UINT: VkFormat = 129;
    pub const VK_FORMAT_D32_SFLOAT_S8_UINT: VkFormat = 130;
    pub const VK_FORMAT_BC1_RGB_UNORM_BLOCK: VkFormat = 131;
    pub const VK_FORMAT_BC1_RGB_SRGB_BLOCK: VkFormat = 132;
    pub const VK_FORMAT_BC1_RGBA_UNORM_BLOCK: VkFormat = 133;
    pub const VK_FORMAT_BC1_RGBA_SRGB_BLOCK: VkFormat = 134;
    pub const VK_FORMAT_BC2_UNORM_BLOCK: VkFormat = 135;
    pub const VK_FORMAT_BC2_SRGB_BLOCK: VkFormat = 136;
    pub const VK_FORMAT_BC3_UNORM_BLOCK: VkFormat = 137;
    pub const VK_FORMAT_BC3_SRGB_BLOCK: VkFormat = 138;
    pub const VK_FORMAT_BC4_UNORM_BLOCK: VkFormat = 139;
    pub const VK_FORMAT_BC4_SNORM_BLOCK: VkFormat = 140;
    pub const VK_FORMAT_BC5_UNORM_BLOCK: VkFormat = 141;
    pub const VK_FORMAT_BC5_SNORM_BLOCK: VkFormat = 142;
    pub const VK_FORMAT_BC6H_UFLOAT_BLOCK: VkFormat = 143;
    pub const VK_FORMAT_BC6H_SFLOAT_BLOCK: VkFormat = 144;
    pub const VK_FORMAT_BC7_UNORM_BLOCK: VkFormat = 145;
    pub const VK_FORMAT_BC7_SRGB_BLOCK: VkFormat = 146;
    pub const VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK: VkFormat = 147;
    pub const VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK: VkFormat = 148;
    pub const VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK: VkFormat = 149;
    pub const VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK: VkFormat = 150;
    pub const VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK: VkFormat = 151;
    pub const VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK: VkFormat = 152;
    pub const VK_FORMAT_EAC_R11_UNORM_BLOCK: VkFormat = 153;
    pub const VK_FORMAT_EAC_R11_SNORM_BLOCK: VkFormat = 154;
    pub const VK_FORMAT_EAC_R11G11_UNORM_BLOCK: VkFormat = 155;
    pub const VK_FORMAT_EAC_R11G11_SNORM_BLOCK: VkFormat = 156;
    pub const VK_FORMAT_ASTC_4x4_UNORM_BLOCK: VkFormat = 157;
    pub const VK_FORMAT_ASTC_4x4_SRGB_BLOCK: VkFormat = 158;
    pub const VK_FORMAT_ASTC_5x4_UNORM_BLOCK: VkFormat = 159;
    pub const VK_FORMAT_ASTC_5x4_SRGB_BLOCK: VkFormat = 160;
    pub const VK_FORMAT_ASTC_5x5_UNORM_BLOCK: VkFormat = 161;
    pub const VK_FORMAT_ASTC_5x5_SRGB_BLOCK: VkFormat = 162;
    pub const VK_FORMAT_ASTC_6x5_UNORM_BLOCK: VkFormat = 163;
    pub const VK_FORMAT_ASTC_6x5_SRGB_BLOCK: VkFormat = 164;
    pub const VK_FORMAT_ASTC_6x6_UNORM_BLOCK: VkFormat = 165;
    pub const VK_FORMAT_ASTC_6x6_SRGB_BLOCK: VkFormat = 166;
    pub const VK_FORMAT_ASTC_8x5_UNORM_BLOCK: VkFormat = 167;
    pub const VK_FORMAT_ASTC_8x5_SRGB_BLOCK: VkFormat = 168;
    pub const VK_FORMAT_ASTC_8x6_UNORM_BLOCK: VkFormat = 169;
    pub const VK_FORMAT_ASTC_8x6_SRGB_BLOCK: VkFormat = 170;
    pub const VK_FORMAT_ASTC_8x8_UNORM_BLOCK: VkFormat = 171;
    pub const VK_FORMAT_ASTC_8x8_SRGB_BLOCK: VkFormat = 172;
    pub const VK_FORMAT_ASTC_10x5_UNORM_BLOCK: VkFormat = 173;
    pub const VK_FORMAT_ASTC_10x5_SRGB_BLOCK: VkFormat = 174;
    pub const VK_FORMAT_ASTC_10x6_UNORM_BLOCK: VkFormat = 175;
    pub const VK_FORMAT_ASTC_10x6_SRGB_BLOCK: VkFormat = 176;
    pub const VK_FORMAT_ASTC_10x8_UNORM_BLOCK: VkFormat = 177;
    pub const VK_FORMAT_ASTC_10x8_SRGB_BLOCK: VkFormat = 178;
    pub const VK_FORMAT_ASTC_10x10_UNORM_BLOCK: VkFormat = 179;
    pub const VK_FORMAT_ASTC_10x10_SRGB_BLOCK: VkFormat = 180;
    pub const VK_FORMAT_ASTC_12x10_UNORM_BLOCK: VkFormat = 181;
    pub const VK_FORMAT_ASTC_12x10_SRGB_BLOCK: VkFormat = 182;
    pub const VK_FORMAT_ASTC_12x12_UNORM_BLOCK: VkFormat = 183;
    pub const VK_FORMAT_ASTC_12x12_SRGB_BLOCK: VkFormat = 184;
    pub const VK_FORMAT_G8B8G8R8_422_UNORM: VkFormat = 1000156000;
    pub const VK_FORMAT_B8G8R8G8_422_UNORM: VkFormat = 1000156001;
    pub const VK_FORMAT_G8_B8_R8_3PLANE_420_UNORM: VkFormat = 1000156002;
    pub const VK_FORMAT_G8_B8R8_2PLANE_420_UNORM: VkFormat = 1000156003;
    pub const VK_FORMAT_G8_B8_R8_3PLANE_422_UNORM: VkFormat = 1000156004;
    pub const VK_FORMAT_G8_B8R8_2PLANE_422_UNORM: VkFormat = 1000156005;
    pub const VK_FORMAT_G8_B8_R8_3PLANE_444_UNORM: VkFormat = 1000156006;
    pub const VK_FORMAT_R10X6_UNORM_PACK16: VkFormat = 1000156007;
    pub const VK_FORMAT_R10X6G10X6_UNORM_2PACK16: VkFormat = 1000156008;
    pub const VK_FORMAT_R10X6G10X6B10X6A10X6_UNORM_4PACK16: VkFormat = 1000156009;
    pub const VK_FORMAT_G10X6B10X6G10X6R10X6_422_UNORM_4PACK16: VkFormat = 1000156010;
    pub const VK_FORMAT_B10X6G10X6R10X6G10X6_422_UNORM_4PACK16: VkFormat = 1000156011;
    pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_420_UNORM_3PACK16: VkFormat = 1000156012;
    pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_420_UNORM_3PACK16: VkFormat = 1000156013;
    pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_422_UNORM_3PACK16: VkFormat = 1000156014;
    pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_422_UNORM_3PACK16: VkFormat = 1000156015;
    pub const VK_FORMAT_G10X6_B10X6_R10X6_3PLANE_444_UNORM_3PACK16: VkFormat = 1000156016;
    pub const VK_FORMAT_R12X4_UNORM_PACK16: VkFormat = 1000156017;
    pub const VK_FORMAT_R12X4G12X4_UNORM_2PACK16: VkFormat = 1000156018;
    pub const VK_FORMAT_R12X4G12X4B12X4A12X4_UNORM_4PACK16: VkFormat = 1000156019;
    pub const VK_FORMAT_G12X4B12X4G12X4R12X4_422_UNORM_4PACK16: VkFormat = 1000156020;
    pub const VK_FORMAT_B12X4G12X4R12X4G12X4_422_UNORM_4PACK16: VkFormat = 1000156021;
    pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_420_UNORM_3PACK16: VkFormat = 1000156022;
    pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_420_UNORM_3PACK16: VkFormat = 1000156023;
    pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_422_UNORM_3PACK16: VkFormat = 1000156024;
    pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_422_UNORM_3PACK16: VkFormat = 1000156025;
    pub const VK_FORMAT_G12X4_B12X4_R12X4_3PLANE_444_UNORM_3PACK16: VkFormat = 1000156026;
    pub const VK_FORMAT_G16B16G16R16_422_UNORM: VkFormat = 1000156027;
    pub const VK_FORMAT_B16G16R16G16_422_UNORM: VkFormat = 1000156028;
    pub const VK_FORMAT_G16_B16_R16_3PLANE_420_UNORM: VkFormat = 1000156029;
    pub const VK_FORMAT_G16_B16R16_2PLANE_420_UNORM: VkFormat = 1000156030;
    pub const VK_FORMAT_G16_B16_R16_3PLANE_422_UNORM: VkFormat = 1000156031;
    pub const VK_FORMAT_G16_B16R16_2PLANE_422_UNORM: VkFormat = 1000156032;
    pub const VK_FORMAT_G16_B16_R16_3PLANE_444_UNORM: VkFormat = 1000156033;
    pub const VK_FORMAT_G8_B8R8_2PLANE_444_UNORM: VkFormat = 1000330000;
    pub const VK_FORMAT_G10X6_B10X6R10X6_2PLANE_444_UNORM_3PACK16: VkFormat = 1000330001;
    pub const VK_FORMAT_G12X4_B12X4R12X4_2PLANE_444_UNORM_3PACK16: VkFormat = 1000330002;
    pub const VK_FORMAT_G16_B16R16_2PLANE_444_UNORM: VkFormat = 1000330003;
    pub const VK_FORMAT_A4R4G4B4_UNORM_PACK16: VkFormat = 1000340000;
    pub const VK_FORMAT_A4B4G4R4_UNORM_PACK16: VkFormat = 1000340001;
    pub const VK_FORMAT_ASTC_4x4_SFLOAT_BLOCK: VkFormat = 1000066000;
    pub const VK_FORMAT_ASTC_5x4_SFLOAT_BLOCK: VkFormat = 1000066001;
    pub const VK_FORMAT_ASTC_5x5_SFLOAT_BLOCK: VkFormat = 1000066002;
    pub const VK_FORMAT_ASTC_6x5_SFLOAT_BLOCK: VkFormat = 1000066003;
    pub const VK_FORMAT_ASTC_6x6_SFLOAT_BLOCK: VkFormat = 1000066004;
    pub const VK_FORMAT_ASTC_8x5_SFLOAT_BLOCK: VkFormat = 1000066005;
    pub const VK_FORMAT_ASTC_8x6_SFLOAT_BLOCK: VkFormat = 1000066006;
    pub const VK_FORMAT_ASTC_8x8_SFLOAT_BLOCK: VkFormat = 1000066007;
    pub const VK_FORMAT_ASTC_10x5_SFLOAT_BLOCK: VkFormat = 1000066008;
    pub const VK_FORMAT_ASTC_10x6_SFLOAT_BLOCK: VkFormat = 1000066009;
    pub const VK_FORMAT_ASTC_10x8_SFLOAT_BLOCK: VkFormat = 1000066010;
    pub const VK_FORMAT_ASTC_10x10_SFLOAT_BLOCK: VkFormat = 1000066011;
    pub const VK_FORMAT_ASTC_12x10_SFLOAT_BLOCK: VkFormat = 1000066012;
    pub const VK_FORMAT_ASTC_12x12_SFLOAT_BLOCK: VkFormat = 1000066013;
    pub const VK_FORMAT_A1B5G5R5_UNORM_PACK16: VkFormat = 1000470000;
    pub const VK_FORMAT_A8_UNORM: VkFormat = 1000470001;
    pub const VK_FORMAT_ASTC_3x3x3_UNORM_BLOCK_EXT: VkFormat = 1000288000;
    pub const VK_FORMAT_ASTC_3x3x3_SRGB_BLOCK_EXT: VkFormat = 1000288001;
    pub const VK_FORMAT_ASTC_3x3x3_SFLOAT_BLOCK_EXT: VkFormat = 1000288002;
    pub const VK_FORMAT_ASTC_4x3x3_UNORM_BLOCK_EXT: VkFormat = 1000288003;
    pub const VK_FORMAT_ASTC_4x3x3_SRGB_BLOCK_EXT: VkFormat = 1000288004;
    pub const VK_FORMAT_ASTC_4x3x3_SFLOAT_BLOCK_EXT: VkFormat = 1000288005;
    pub const VK_FORMAT_ASTC_4x4x3_UNORM_BLOCK_EXT: VkFormat = 1000288006;
    pub const VK_FORMAT_ASTC_4x4x3_SRGB_BLOCK_EXT: VkFormat = 1000288007;
    pub const VK_FORMAT_ASTC_4x4x3_SFLOAT_BLOCK_EXT: VkFormat = 1000288008;
    pub const VK_FORMAT_ASTC_4x4x4_UNORM_BLOCK_EXT: VkFormat = 1000288009;
    pub const VK_FORMAT_ASTC_4x4x4_SRGB_BLOCK_EXT: VkFormat = 1000288010;
    pub const VK_FORMAT_ASTC_4x4x4_SFLOAT_BLOCK_EXT: VkFormat = 1000288011;
    pub const VK_FORMAT_ASTC_5x4x4_UNORM_BLOCK_EXT: VkFormat = 1000288012;
    pub const VK_FORMAT_ASTC_5x4x4_SRGB_BLOCK_EXT: VkFormat = 1000288013;
    pub const VK_FORMAT_ASTC_5x4x4_SFLOAT_BLOCK_EXT: VkFormat = 1000288014;
    pub const VK_FORMAT_ASTC_5x5x4_UNORM_BLOCK_EXT: VkFormat = 1000288015;
    pub const VK_FORMAT_ASTC_5x5x4_SRGB_BLOCK_EXT: VkFormat = 1000288016;
    pub const VK_FORMAT_ASTC_5x5x4_SFLOAT_BLOCK_EXT: VkFormat = 1000288017;
    pub const VK_FORMAT_ASTC_5x5x5_UNORM_BLOCK_EXT: VkFormat = 1000288018;
    pub const VK_FORMAT_ASTC_5x5x5_SRGB_BLOCK_EXT: VkFormat = 1000288019;
    pub const VK_FORMAT_ASTC_5x5x5_SFLOAT_BLOCK_EXT: VkFormat = 1000288020;
    pub const VK_FORMAT_ASTC_6x5x5_UNORM_BLOCK_EXT: VkFormat = 1000288021;
    pub const VK_FORMAT_ASTC_6x5x5_SRGB_BLOCK_EXT: VkFormat = 1000288022;
    pub const VK_FORMAT_ASTC_6x5x5_SFLOAT_BLOCK_EXT: VkFormat = 1000288023;
    pub const VK_FORMAT_ASTC_6x6x5_UNORM_BLOCK_EXT: VkFormat = 1000288024;
    pub const VK_FORMAT_ASTC_6x6x5_SRGB_BLOCK_EXT: VkFormat = 1000288025;
    pub const VK_FORMAT_ASTC_6x6x5_SFLOAT_BLOCK_EXT: VkFormat = 1000288026;
    pub const VK_FORMAT_ASTC_6x6x6_UNORM_BLOCK_EXT: VkFormat = 1000288027;
    pub const VK_FORMAT_ASTC_6x6x6_SRGB_BLOCK_EXT: VkFormat = 1000288028;
    pub const VK_FORMAT_ASTC_6x6x6_SFLOAT_BLOCK_EXT: VkFormat = 1000288029;
}

pub type VkPerformanceParameterTypeINTEL = i32;
pub mod VkPerformanceParameterTypeINTELValue {
    use crate::VkPerformanceParameterTypeINTEL;
    pub const VK_PERFORMANCE_PARAMETER_TYPE_HW_COUNTERS_SUPPORTED_INTEL: VkPerformanceParameterTypeINTEL = 0;
    pub const VK_PERFORMANCE_PARAMETER_TYPE_STREAM_MARKER_VALID_BITS_INTEL: VkPerformanceParameterTypeINTEL = 1;
}

pub type VkShaderFloatControlsIndependence = i32;
pub mod VkShaderFloatControlsIndependenceValue {
    use crate::VkShaderFloatControlsIndependence;
    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_32_BIT_ONLY: VkShaderFloatControlsIndependence = 0;
    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_ALL: VkShaderFloatControlsIndependence = 1;
    pub const VK_SHADER_FLOAT_CONTROLS_INDEPENDENCE_NONE: VkShaderFloatControlsIndependence = 2;
}

pub type VkSubpassContents = i32;
pub mod VkSubpassContentsValue {
    use crate::VkSubpassContents;
    pub const VK_SUBPASS_CONTENTS_INLINE: VkSubpassContents = 0;
    pub const VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS: VkSubpassContents = 1;
    pub const VK_SUBPASS_CONTENTS_INLINE_AND_SECONDARY_COMMAND_BUFFERS_KHR: VkSubpassContents = 1000451000;
}

pub type VkVertexInputRate = i32;
pub mod VkVertexInputRateValue {
    use crate::VkVertexInputRate;
    pub const VK_VERTEX_INPUT_RATE_VERTEX: VkVertexInputRate = 0;
    pub const VK_VERTEX_INPUT_RATE_INSTANCE: VkVertexInputRate = 1;
}

