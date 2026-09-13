/*
// SPDX-License-Identifier: None
// Copyright (c) 2026 None

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
 */

/*
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
 */

// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// Provided by VK_VERSION_1_0
pub type PFN_vkAllocationFunction = unsafe extern "system" fn (
    pUserData : *mut std::ffi::c_void,
    size : usize,
    alignment : usize,
    allocationScope : crate::svk::VkSystemAllocationScope)
    -> *mut std::ffi::c_void;

// Provided by VK_VERSION_1_0
pub type PFN_vkReallocationFunction = unsafe extern "system" fn (
    pUserData : *mut std::ffi::c_void,
    pOriginal : *mut std::ffi::c_void,
    size : usize,
    alignment : usize,
    allocationScope : crate::svk::VkSystemAllocationScope)
    -> *mut std::ffi::c_void;

// Provided by VK_VERSION_1_0
pub type PFN_vkFreeFunction = unsafe extern "system" fn (
    pUserData : *mut std::ffi::c_void,
    pMemory : *mut std::ffi::c_void)
    -> std::ffi::c_void;

// Provided by VK_VERSION_1_0
pub type PFN_vkInternalAllocationNotification = unsafe extern "system" fn (
    pUserData : *mut std::ffi::c_void,
    size : usize,
    allocationType : crate::svk::VkInternalAllocationType,
    allocationScope : crate::svk::VkSystemAllocationScope)
    -> std::ffi::c_void;

// Provided by VK_VERSION_1_0
pub type PFN_vkInternalFreeNotification = unsafe extern "system" fn (
    pUserData : *mut std::ffi::c_void,
    size : usize,
    allocationType : crate::svk::VkInternalAllocationType,
    allocationScope : crate::svk::VkSystemAllocationScope)
    -> std::ffi::c_void;

// Provided by VK_VERSION_1_0
pub type PFN_vkGetInstanceProcAddr = unsafe extern "system" fn (
    instance : crate::svk::VkInstance,
    pName : *const std::ffi::c_char,)
    -> *mut std::ffi::c_void;


// Provided by VK_VERSION_1_0
pub type PFN_vkEnumerateInstanceExtensionProperties = unsafe extern "system" fn (
    pLayerName : *const std::ffi::c_char,
    pPropertyCount : *mut u32,
    pProperties : *mut crate::svk::VkExtensionProperties)
    -> crate::svk::VkResult;

// Provided by VK_VERSION_1_0
pub type PFN_vkEnumerateInstanceLayerProperties = unsafe extern "system" fn (
    pPropertyCount : *mut u32,
    pProperties : *mut crate::svk::VkLayerProperties)
    -> crate::svk::VkResult;

// Provided by VK_VERSION_1_0
pub type PFN_vkCreateInstance = unsafe extern "system" fn (
    pCreateInfo : *const crate::svk::VkInstanceCreateInfo,
    pAllocator : *const crate::svk::VkAllocationCallbacks,
    pInstance : *mut crate::svk::VkInstance)
    -> crate::svk::VkResult;

// Provided by VK_VERSION_1_1
pub type PFN_vkEnumerateInstanceVersion = unsafe extern "system" fn (
    pApiVersion : *mut u32) 
    -> crate::svk::VkResult;

// Provided by VK_EXT_debug_report
pub type PFN_vkDebugReportCallbackEXT = unsafe extern "system" fn (
    flags : crate::svk::VkDebugReportFlagsEXT,
    objectType : crate::svk::VkDebugReportObjectTypeEXT,
    object : u64,
    location : usize,
    messageCode : i32,
    pLayerPrefix : *const std::ffi::c_char,
    pMessage : *const std::ffi::c_char,
    pUserData : *mut std::ffi::c_void) -> bool;

// Provided by VK_EXT_debug_utils
pub type PFN_vkDebugUtilsMessengerCallbackEXT = unsafe extern "system" fn (
    messageSeverity : crate::svk::VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes : crate::svk::VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData : *const crate::svk::VkDebugUtilsMessengerCallbackDataEXT,
    pUserData : *mut std::ffi::c_void) -> bool;

// Provided by VK_VERSION_1_0
pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn (
    instance : crate::svk::VkInstance,
    pPhysicalDeviceCount : *mut u32,
    pPhysicalDevices : *mut crate::svk::VkPhysicalDevice)
    -> crate::svk::VkResult;

// Provided by VK_VERSION_1_0
pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn (
    physicalDevice: crate::svk::VkPhysicalDevice,
    pProperties: *mut crate::svk::VkPhysicalDeviceProperties);

// Provided by VK_VERSION_1_1
pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn (
    physicalDevice: crate::svk::VkPhysicalDevice,
    pProperties: *mut crate::svk::VkPhysicalDeviceProperties2);

// Provided by VK_VERSION_1_0
pub type PFN_vkDestroyInstance = unsafe extern "system" fn (
    instance: crate::svk::VkInstance,
    pAllocator: *const crate::svk::VkAllocationCallbacks);