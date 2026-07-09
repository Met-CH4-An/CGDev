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
    pfnCallback : PFN_vkDebugReportCallbackEXT,
    pUserData : *const std::ffi::c_void,
}