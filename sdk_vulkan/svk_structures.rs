// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// Provided by VK_VERSION_1_0
#[repr(C)]
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
pub struct VkExtensionProperties {
    pub extensionName : [std::ffi::c_char; crate::svk_constants::VK_MAX_EXTENSION_NAME_SIZE as usize],
    pub specVersion : u32,
}

// Provided by VK_VERSION_1_0
#[repr(C)]
pub struct VkLayerProperties {
    pub layerName : [std::ffi::c_char; crate::svk_constants::VK_MAX_EXTENSION_NAME_SIZE as usize],
    pub specVersion : u32,
    pub implementationVersion : u32,
    pub description : [std::ffi::c_char; crate::svk_constants::VK_MAX_DESCRIPTION_SIZE as usize],
}

// Provided by VK_VERSION_1_0
#[repr(C)]
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
pub struct VkDebugReportCallbackCreateInfoEXT {
    sType : crate::svk_enums::VkStructureType,
    pNext : *const std::ffi::c_void,
    flags : crate::svk_types::VkDebugReportFlagsEXT,
    pfnCallback : crate::svk_commands::PFN_vkDebugReportCallbackEXT,
    pUserData : *mut std::ffi::c_void,
}

// Provided by VK_EXT_debug_utils
#[repr(C)]
pub struct VkDebugUtilsMessengerCreateInfoEXT {
    sType : crate::svk_enums::VkStructureType,
    pNext : *const std::ffi::c_void,
    flags : crate::svk_types::VkDebugUtilsMessengerCreateFlagsEXT,
    messageSeverity : crate::svk_types::VkDebugUtilsMessageSeverityFlagsEXT,
    messageType : crate::svk_types::VkDebugUtilsMessageTypeFlagsEXT,
    pfnUserCallback : crate::svk_commands::PFN_vkDebugUtilsMessengerCallbackEXT,
    pUserData : *mut std::ffi::c_void,
}

// Provided by VK_EXT_debug_utils
#[repr(C)]
pub struct VkDebugUtilsMessengerCallbackDataEXT {
    sType : crate::svk_enums::VkStructureType,
    pNext : *const std::ffi::c_void,
    flags : crate::svk_types::VkDebugUtilsMessengerCallbackDataFlagsEXT,
    pMessageIdName : *const std::ffi::c_char,
    messageIdNumber : u32,
    pMessage : *const std::ffi::c_char,
    queueLabelCount : u32,
    pQueueLabels : *const VkDebugUtilsLabelEXT,
    cmdBufLabelCount : u32,
    pCmdBufLabels : *const VkDebugUtilsLabelEXT,
    objectCount : u32,
    pObjects : *const VkDebugUtilsObjectNameInfoEXT,
}

// Provided by VK_EXT_debug_utils
#[repr(C)]
pub struct VkDebugUtilsLabelEXT {
    sType : crate::svk_enums::VkStructureType,
    pNext : *const std::ffi::c_void,
    pLabelName  : *const std::ffi::c_char,
    color : [f32; 4],
}

// Provided by VK_EXT_debug_utils
#[repr(C)]
pub struct VkDebugUtilsObjectNameInfoEXT {
    sType : crate::svk_enums::VkStructureType,
    pNext : *const std::ffi::c_void,
    objectType : crate::svk_enums::VkObjectType,
    objectHandle : u64,
    pObjectName : *const std::ffi::c_char,
}