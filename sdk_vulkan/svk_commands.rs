// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// Provided by VK_VERSION_1_0
pub type PFN_vkAllocationFunction = unsafe extern "system" fn (
    pUserData : *mut std::ffi::c_void,
    size : usize,
    alignment : usize,
    allocationScope : crate::svk_enums::VkSystemAllocationScope)
    -> *mut std::ffi::c_void;

// Provided by VK_VERSION_1_0
pub type PFN_vkReallocationFunction = unsafe extern "system" fn (
    pUserData : *mut std::ffi::c_void,
    pOriginal : *mut std::ffi::c_void,
    size : usize,
    alignment : usize,
    allocationScope : crate::svk_enums::VkSystemAllocationScope)
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
    allocationType : crate::svk_enums::VkInternalAllocationType,
    allocationScope : crate::svk_enums::VkSystemAllocationScope)
    -> std::ffi::c_void;

// Provided by VK_VERSION_1_0
pub type PFN_vkInternalFreeNotification = unsafe extern "system" fn (
    pUserData : *mut std::ffi::c_void,
    size : usize,
    allocationType : crate::svk_enums::VkInternalAllocationType,
    allocationScope : crate::svk_enums::VkSystemAllocationScope)
    -> std::ffi::c_void;

// Provided by VK_VERSION_1_0
pub type PFN_vkGetInstanceProcAddr = unsafe extern "system" fn (
    instance : crate::svk_types::VkInstance,
    pName : *const std::ffi::c_char,)
    -> *mut std::ffi::c_void;


// Provided by VK_VERSION_1_0
pub type PFN_vkEnumerateInstanceExtensionProperties = unsafe extern "system" fn (
    pLayerName : *const std::ffi::c_char,
    pPropertyCount : *mut u32,
    pProperties : *mut crate::svk_structures::VkExtensionProperties)
    -> crate::svk_enums::VkResult;

// Provided by VK_VERSION_1_0
pub type PFN_vkEnumerateInstanceLayerProperties = unsafe extern "system" fn (
    pPropertyCount : *mut u32,
    pProperties : *mut crate::svk_structures::VkLayerProperties)
    -> crate::svk_enums::VkResult;

// Provided by VK_VERSION_1_0
pub type PFN_vkCreateInstance = unsafe extern "system" fn (
    pCreateInfo : *const crate::svk_structures::VkInstanceCreateInfo,
    pAllocator : *const crate::svk_structures::VkAllocationCallbacks,
    pInstance : *mut crate::svk_types::VkInstance)
    -> crate::svk_enums::VkResult;

// Provided by VK_VERSION_1_1
pub type PFN_vkEnumerateInstanceVersion = unsafe extern "system" fn (
    pApiVersion : *mut u32) 
    -> crate::svk_enums::VkResult;

// Provided by VK_EXT_debug_report
pub type PFN_vkDebugReportCallbackEXT = unsafe extern "system" fn (
    flags : crate::svk_types::VkDebugReportFlagsEXT,
    objectType : crate::svk_enums::VkDebugReportObjectTypeEXT,
    object : u64,
    location : usize,
    messageCode : i32,
    pLayerPrefix : *const std::ffi::c_char,
    pMessage : *const std::ffi::c_char,
    pUserData : *mut std::ffi::c_void) -> bool;

// Provided by VK_EXT_debug_utils
pub type PFN_vkDebugUtilsMessengerCallbackEXT = unsafe extern "system" fn (
    messageSeverity : crate::svk_enums::VkDebugUtilsMessageSeverityFlagBitsEXT,
    messageTypes : crate::svk_types::VkDebugUtilsMessageTypeFlagsEXT,
    pCallbackData : *const crate::svk_structures::VkDebugUtilsMessengerCallbackDataEXT,
    pUserData : *mut std::ffi::c_void) -> bool;

// Provided by VK_VERSION_1_0
pub type PFN_vkEnumeratePhysicalDevices = unsafe extern "system" fn (
    instance : crate::svk_types::VkInstance,
    pPhysicalDeviceCount : *mut u32,
    pPhysicalDevices : *mut crate::svk_types::VkPhysicalDevice)
    -> crate::svk_enums::VkResult;

// Provided by VK_VERSION_1_0
pub type PFN_vkGetPhysicalDeviceProperties = unsafe extern "system" fn (
    physicalDevice: crate::svk_types::VkPhysicalDevice,
    pProperties: *mut crate::svk_structures::VkPhysicalDeviceProperties);

// Provided by VK_VERSION_1_1
pub type PFN_vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn (
    physicalDevice: crate::svk_types::VkPhysicalDevice,
    pProperties: *mut crate::svk_structures::VkPhysicalDeviceProperties2);