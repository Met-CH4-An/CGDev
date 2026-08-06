// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// Provided by VK_VERSION_1_0
#[repr(C)]
#[derive(Debug)]
pub struct VkAllocationCallbacks {
    pub pUserDatavoid : *mut std::ffi::c_void,
    pub pfnAllocation : crate::svk_commands::PFN_vkAllocationFunction,
    pub pfnReallocation : crate::svk_commands::PFN_vkReallocationFunction,
    pub pfnFree : crate::svk_commands::PFN_vkFreeFunction,
    pub pfnInternalAllocation : crate::svk_commands::PFN_vkInternalAllocationNotification,
    pub pfnInternalFree : crate::svk_commands::PFN_vkInternalFreeNotification,
}

// Provided by VK_VERSION_1_0
#[repr(C)]
#[derive(Debug)]
pub struct VkExtensionProperties {
    pub extensionName : [std::ffi::c_char; crate::svk_constants::VK_MAX_EXTENSION_NAME_SIZE as usize],
    pub specVersion : u32,
}

// Provided by VK_VERSION_1_0
#[repr(C)]
#[derive(Debug)]
pub struct VkLayerProperties {
    pub layerName : [std::ffi::c_char; crate::svk_constants::VK_MAX_EXTENSION_NAME_SIZE as usize],
    pub specVersion : u32,
    pub implementationVersion : u32,
    pub description : [std::ffi::c_char; crate::svk_constants::VK_MAX_DESCRIPTION_SIZE as usize],
}

// Provided by VK_VERSION_1_0
#[repr(C)]
#[derive(Debug)]
pub struct VkInstanceCreateInfo {
    pub sType : crate::svk_enums::VkStructureType,
    pub pNext : *const std::ffi::c_void,
    pub flags : crate::svk_types::VkInstanceCreateFlags,
    pub pApplicationInfo : *const VkApplicationInfo,
    pub enabledLayerCount : u32,
    pub ppEnabledLayerNames : *const *const std::ffi::c_char,
    pub enabledExtensionCount : u32,
    pub ppEnabledExtensionNames : *const *const std::ffi::c_char,
}

// Provided by VK_VERSION_1_0
#[repr(C)]
#[derive(Debug)]
pub struct VkApplicationInfo {
    pub sType : crate::svk_enums::VkStructureType,
    pub pNext : *const std::ffi::c_void,
    pub pApplicationName : *const std::ffi::c_char,
    pub applicationVersion : u32,
    pub pEngineName : *const std::ffi::c_char,
    pub engineVersion : u32,
    pub apiVersion : u32,
}

// Provided by VK_EXT_debug_report
#[repr(C)]
#[derive(Debug)]
pub struct VkDebugReportCallbackCreateInfoEXT {
    pub sType : crate::svk_enums::VkStructureType,
    pub pNext : *const std::ffi::c_void,
    pub flags : crate::svk_types::VkDebugReportFlagsEXT,
    pub pfnCallback : crate::svk_commands::PFN_vkDebugReportCallbackEXT,
    pub pUserData : *mut std::ffi::c_void,
}

// Provided by VK_EXT_debug_utils
#[repr(C)]
#[derive(Debug)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
    pub sType : crate::svk_enums::VkStructureType,
    pub pNext : *const std::ffi::c_void,
    pub flags : crate::svk_types::VkDebugUtilsMessengerCreateFlagsEXT,
    pub messageSeverity : crate::svk_types::VkDebugUtilsMessageSeverityFlagsEXT,
    pub messageType : crate::svk_types::VkDebugUtilsMessageTypeFlagsEXT,
    pub pfnUserCallback : crate::svk_commands::PFN_vkDebugUtilsMessengerCallbackEXT,
    pub pUserData : *mut std::ffi::c_void,
}

// Provided by VK_EXT_debug_utils
#[repr(C)]
#[derive(Debug)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
    pub sType : crate::svk_enums::VkStructureType,
    pub pNext : *const std::ffi::c_void,
    pub flags : crate::svk_types::VkDebugUtilsMessengerCallbackDataFlagsEXT,
    pub pMessageIdName : *const std::ffi::c_char,
    pub messageIdNumber : u32,
    pub pMessage : *const std::ffi::c_char,
    pub queueLabelCount : u32,
    pub pQueueLabels : *const VkDebugUtilsLabelEXT,
    pub cmdBufLabelCount : u32,
    pub pCmdBufLabels : *const VkDebugUtilsLabelEXT,
    pub objectCount : u32,
    pub pObjects : *const VkDebugUtilsObjectNameInfoEXT,
}

// Provided by VK_EXT_debug_utils
#[repr(C)]
#[derive(Debug)]
pub struct VkDebugUtilsLabelEXT {
    pub sType : crate::svk_enums::VkStructureType,
    pub pNext : *const std::ffi::c_void,
    pub pLabelName  : *const std::ffi::c_char,
    pub color : [f32; 4],
}

// Provided by VK_EXT_debug_utils
#[repr(C)]
#[derive(Debug)]
pub struct VkDebugUtilsObjectNameInfoEXT {
    pub sType : crate::svk_enums::VkStructureType,
    pub pNext : *const std::ffi::c_void,
    pub objectType : crate::svk_enums::VkObjectType,
    pub objectHandle : u64,
    pub pObjectName : *const std::ffi::c_char,
}

// Provided by VK_VERSION_1_0
#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceProperties {
    pub apiVersion: u32,
    pub driverVersion: u32,
    pub vendorID: u32,
    pub deviceID: u32,
    pub deviceType: crate::svk_enums::VkPhysicalDeviceType,
    pub deviceName: [std::ffi::c_char; crate::svk_constants::VK_MAX_PHYSICAL_DEVICE_NAME_SIZE as usize],
    pub pipelineCacheUUID: [u8; crate::svk_constants::VK_UUID_SIZE as usize],
    pub limits: VkPhysicalDeviceLimits,
    pub sparseProperties: VkPhysicalDeviceSparseProperties,
}
impl VkPhysicalDeviceProperties {
    pub fn s_createEmpty() -> Self {
        Self {}
    }
}

// Provided by VK_VERSION_1_0
#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceLimits {
    pub maxImageDimension1D: u32,
    pub maxImageDimension2D: u32,
    pub maxImageDimension3D: u32,
    pub maxImageDimensionCube: u32,
    pub maxImageArrayLayers: u32,
    pub maxTexelBufferElements: u32,
    pub maxUniformBufferRange: u32,
    pub maxStorageBufferRange: u32,
    pub maxPushConstantsSize: u32,
    pub maxMemoryAllocationCount: u32,
    pub maxSamplerAllocationCount: u32,
    pub bufferImageGranularity: crate::svk_types::VkDeviceSize,
    pub sparseAddressSpaceSize: crate::svk_types::VkDeviceSize,
    pub maxBoundDescriptorSets: u32,
    pub maxPerStageDescriptorSamplers: u32,
    pub maxPerStageDescriptorUniformBuffers: u32,
    pub maxPerStageDescriptorStorageBuffers: u32,
    pub maxPerStageDescriptorSampledImages: u32,
    pub maxPerStageDescriptorStorageImages: u32,
    pub maxPerStageDescriptorInputAttachments: u32,
    pub maxPerStageResources: u32,
    pub maxDescriptorSetSamplers: u32,
    pub maxDescriptorSetUniformBuffers: u32,
    pub maxDescriptorSetUniformBuffersDynamic: u32,
    pub maxDescriptorSetStorageBuffers: u32,
    pub maxDescriptorSetStorageBuffersDynamic: u32,
    pub maxDescriptorSetSampledImages: u32,
    pub maxDescriptorSetStorageImages: u32,
    pub maxDescriptorSetInputAttachments: u32,
    pub maxVertexInputAttributes: u32,
    pub maxVertexInputBindings: u32,
    pub maxVertexInputAttributeOffset: u32,
    pub maxVertexInputBindingStride: u32,
    pub maxVertexOutputComponents: u32,
    pub maxTessellationGenerationLevel: u32,
    pub maxTessellationPatchSize: u32,
    pub maxTessellationControlPerVertexInputComponents: u32,
    pub maxTessellationControlPerVertexOutputComponents: u32,
    pub maxTessellationControlPerPatchOutputComponents: u32,
    pub maxTessellationControlTotalOutputComponents: u32,
    pub maxTessellationEvaluationInputComponents: u32,
    pub maxTessellationEvaluationOutputComponents: u32,
    pub maxGeometryShaderInvocations: u32,
    pub maxGeometryInputComponents: u32,
    pub maxGeometryOutputComponents: u32,
    pub maxGeometryOutputVertices: u32,
    pub maxGeometryTotalOutputComponents: u32,
    pub maxFragmentInputComponents: u32,
    pub maxFragmentOutputAttachments: u32,
    pub maxFragmentDualSrcAttachments: u32,
    pub maxFragmentCombinedOutputResources: u32,
    pub maxComputeSharedMemorySize: u32,
    pub maxComputeWorkGroupCount: [u32; 3],
    pub maxComputeWorkGroupInvocations: u32,
    pub maxComputeWorkGroupSize: [u32; 3],
    pub subPixelPrecisionBits: u32,
    pub subTexelPrecisionBits: u32,
    pub mipmapPrecisionBits: u32,
    pub maxDrawIndexedIndexValue: u32,
    pub maxDrawIndirectCount: u32,
    pub maxSamplerLodBias: f32,
    pub maxSamplerAnisotropy: f32,
    pub maxViewports: u32,
    pub maxViewportDimensions: [u32; 2],
    pub viewportBoundsRange: [f32; 2],
    pub viewportSubPixelBits: u32,
    pub minMemoryMapAlignment : usize,
    pub minTexelBufferOffsetAlignment: crate::svk_types::VkDeviceSize,
    pub minUniformBufferOffsetAlignment: crate::svk_types::VkDeviceSize,
    pub minStorageBufferOffsetAlignment: crate::svk_types::VkDeviceSize,
    pub minTexelOffset: i32,
    pub maxTexelOffset: u32,
    pub minTexelGatherOffset: i32,
    pub maxTexelGatherOffset: u32,
    pub minInterpolationOffset: f32,
    pub maxInterpolationOffset: f32,
    pub subPixelInterpolationOffsetBits: u32,
    pub maxFramebufferWidth: u32,
    pub maxFramebufferHeight: u32,
    pub maxFramebufferLayers: u32,
    pub framebufferColorSampleCounts: crate::svk_types::VkSampleCountFlags,
    pub framebufferDepthSampleCounts: crate::svk_types::VkSampleCountFlags,
    pub framebufferStencilSampleCounts: crate::svk_types::VkSampleCountFlags,
    pub framebufferNoAttachmentsSampleCounts: crate::svk_types::VkSampleCountFlags,
    pub maxColorAttachments: u32,
    pub sampledImageColorSampleCounts: crate::svk_types::VkSampleCountFlags,
    pub sampledImageIntegerSampleCounts: crate::svk_types::VkSampleCountFlags,
    pub sampledImageDepthSampleCounts: crate::svk_types::VkSampleCountFlags,
    pub sampledImageStencilSampleCounts: crate::svk_types::VkSampleCountFlags,
    pub storageImageSampleCounts: crate::svk_types::VkSampleCountFlags,
    pub maxSampleMaskWords: u32,
    pub timestampComputeAndGraphics: bool,
    pub timestampPeriod: f32,
    pub maxClipDistances: u32,
    pub maxCullDistances: u32,
    pub maxCombinedClipAndCullDistances: u32,
    pub discreteQueuePriorities: u32,
    pub pointSizeRange: [f32; 2],
    pub lineWidthRange: [f32; 2],
    pub pointSizeGranularity: f32,
    pub lineWidthGranularity: f32,
    pub strictLines: bool,
    pub standardSampleLocations: bool,
    pub optimalBufferCopyOffsetAlignment: crate::svk_types::VkDeviceSize,
    pub optimalBufferCopyRowPitchAlignment: crate::svk_types::VkDeviceSize,
    pub nonCoherentAtomSize: crate::svk_types::VkDeviceSize,
}
impl VkPhysicalDeviceLimits {
    pub fn s_createEmpty() -> Self {
        Self{
            maxImageDimension1D: 0,
            maxImageDimension2D: 0,
            maxImageDimension3D: 0,
            maxImageDimensionCube: 0,
            maxImageArrayLayers: 0,
            maxTexelBufferElements: 0,
            maxUniformBufferRange: 0,
            maxStorageBufferRange: 0,
            maxPushConstantsSize: 0,
            maxMemoryAllocationCount: 0,
            maxSamplerAllocationCount: 0,
            bufferImageGranularity: 0,
            sparseAddressSpaceSize: 0,
            maxBoundDescriptorSets: 0,
            maxPerStageDescriptorSamplers: 0,
            maxPerStageDescriptorUniformBuffers: 0,
            maxPerStageDescriptorStorageBuffers: 0,
            maxPerStageDescriptorSampledImages: 0,
            maxPerStageDescriptorStorageImages: 0,
            maxPerStageDescriptorInputAttachments: 0,
            maxPerStageResources: 0,
            maxDescriptorSetSamplers: 0,
            maxDescriptorSetUniformBuffers: 0,
            maxDescriptorSetUniformBuffersDynamic: 0,
            maxDescriptorSetStorageBuffers: 0,
            maxDescriptorSetStorageBuffersDynamic: 0,
            maxDescriptorSetSampledImages: 0,
            maxDescriptorSetStorageImages: 0,
            maxDescriptorSetInputAttachments: 0,
            maxVertexInputAttributes: 0,
            maxVertexInputBindings: 0,
            maxVertexInputAttributeOffset: 0,
            maxVertexInputBindingStride: 0,
            maxVertexOutputComponents: 0,
            maxTessellationGenerationLevel: 0,
            maxTessellationPatchSize: 0,
            maxTessellationControlPerVertexInputComponents: 0,
            maxTessellationControlPerVertexOutputComponents: 0,
            maxTessellationControlPerPatchOutputComponents: 0,
            maxTessellationControlTotalOutputComponents: 0,
            maxTessellationEvaluationInputComponents: 0,
            maxTessellationEvaluationOutputComponents: 0,
            maxGeometryShaderInvocations: 0,
            maxGeometryInputComponents: 0,
            maxGeometryOutputComponents: 0,
            maxGeometryOutputVertices: 0,
            maxGeometryTotalOutputComponents: 0,
            maxFragmentInputComponents: 0,
            maxFragmentOutputAttachments: 0,
            maxFragmentDualSrcAttachments: 0,
            maxFragmentCombinedOutputResources: 0,
            maxComputeSharedMemorySize: 0,
            maxComputeWorkGroupCount: [0; 3],
            maxComputeWorkGroupInvocations: 0,
            maxComputeWorkGroupSize: [0; 3],
            subPixelPrecisionBits: 0,
            subTexelPrecisionBits: 0,
            mipmapPrecisionBits: 0,
            maxDrawIndexedIndexValue: 0,
            maxDrawIndirectCount: 0,
            maxSamplerLodBias: 0.0,
            maxSamplerAnisotropy: 0.0,
            maxViewports: 0,
            maxViewportDimensions: [0; 2],
            viewportBoundsRange: [0.0; 2],
            viewportSubPixelBits: 0,
            minMemoryMapAlignment : 0,
            minTexelBufferOffsetAlignment: 0,
            minUniformBufferOffsetAlignment: 0,
            minStorageBufferOffsetAlignment: 0,
            minTexelOffset: 0,
            maxTexelOffset: 0,
            minTexelGatherOffset: 0,
            maxTexelGatherOffset: 0,
            minInterpolationOffset: 0.0,
            maxInterpolationOffset: 0.0,
            subPixelInterpolationOffsetBits: 0,
            maxFramebufferWidth: 0,
            maxFramebufferHeight: 0,
            maxFramebufferLayers: 0,
            framebufferColorSampleCounts: 0,
            framebufferDepthSampleCounts: 0,
            framebufferStencilSampleCounts: 0,
            framebufferNoAttachmentsSampleCounts: 0,
            maxColorAttachments: 0,
            sampledImageColorSampleCounts: 0,
            sampledImageIntegerSampleCounts: 0,
            sampledImageDepthSampleCounts: 0,
            sampledImageStencilSampleCounts: 0,
            storageImageSampleCounts: 0,
            maxSampleMaskWords: 0,
            timestampComputeAndGraphics: false,
            timestampPeriod: 0.0,
            maxClipDistances: 0,
            maxCullDistances: 0,
            maxCombinedClipAndCullDistances: 0,
            discreteQueuePriorities: 0,
            pointSizeRange: [0.0; 2],
            lineWidthRange: [0.0; 2],
            pointSizeGranularity: 0.0,
            lineWidthGranularity: 0.0,
            strictLines: false,
            standardSampleLocations: false,
            optimalBufferCopyOffsetAlignment: 0,
            optimalBufferCopyRowPitchAlignment: 0,
            nonCoherentAtomSize: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceSparseProperties {
    pub residencyStandard2DBlockShape: bool,
    pub residencyStandard2DMultisampleBlockShape: bool,
    pub residencyStandard3DBlockShape: bool,
    pub residencyAlignedMipSize: bool,
    pub residencyNonResidentStrict: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct VkPhysicalDeviceProperties2 {
    pub sType: crate::svk_enums::VkStructureType,
    pub pNext: *const std::ffi::c_void,
    pub properties: VkPhysicalDeviceProperties,
}
impl VkPhysicalDeviceProperties2 {
    pub fn s_create() -> Self {
        Self{
            sType: crate::svk_enums::VkStructureTypeValue::
        }
    }
}