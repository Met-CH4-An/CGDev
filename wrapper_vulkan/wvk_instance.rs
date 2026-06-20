// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// зависимости
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use {
    crate::wvk_call_with_check,
    crate::WvkError,
    crate::WvkErrorType,
};

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkInstanceCreateInfo {

}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkInstance {
    // vulkan 1.0
    vkGetInstanceProcAddr : crate::svk::PFN_vkGetInstanceProcAddr,
    vkEnumerateInstanceLayerProperties : crate::svk::PFN_vkEnumerateInstanceLayerProperties,
    vkEnumerateInstanceExtensionProperties : crate::svk::PFN_vkEnumerateInstanceExtensionProperties,
    vkCreateInstance : crate::svk::PFN_vkCreateInstance,    
    
    // vulkan 1.1
    vkEnumerateInstanceVersion : crate::svk::PFN_vkEnumerateInstanceVersion,
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl WvkInstance {
}