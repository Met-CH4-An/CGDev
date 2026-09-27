// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::panic::panic_any;
use crate::wvk_library::dispatch_table::WvkLibraryDispatchTable;

impl WvkLibraryDispatchTable {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(in crate::wvk_library) fn vkGetInstanceProcAddr(&self, instance: svk::VkInstance, pName: *const std::ffi::c_char) -> *mut std::ffi::c_void {
        unsafe { (self.vk_get_instance_proc_addr)(instance, pName) }
    }
    #[inline(always)]
    pub(in crate::wvk_library::dispatch_table) unsafe extern "system" fn vkGetInstanceProcAddrDummy(instance: svk::VkInstance, pName: *const std::ffi::c_char) -> *mut std::ffi::c_void{
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn vkEnumerateInstanceLayerProperties(&self, pPropertyCount: *mut u32, pProperties: *mut svk::VkLayerProperties) -> svk::VkResult {
        unsafe { (self.vk_enumerate_instance_layer_properties)(pPropertyCount, pProperties) }
    }
    #[inline(always)]
    pub(in crate::wvk_library::dispatch_table) unsafe extern "system" fn vkEnumerateInstanceLayerPropertiesDummy(pPropertyCount: *mut u32, pProperties: *mut svk::VkLayerProperties) -> svk::VkResult {
        //Err("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn vkEnumerateInstanceExtensionProperties(&self, pLayerName: *const std::ffi::c_char, pPropertyCount: *mut u32, pProperties: *mut svk::VkExtensionProperties) -> svk::VkResult {
        unsafe { (self.vk_enumerate_instance_extension_properties)(pLayerName, pPropertyCount, pProperties) }
    }
    #[inline(always)]
    pub(in crate::wvk_library::dispatch_table) unsafe extern "system" fn vkEnumerateInstanceExtensionPropertiesDummy(pLayerName: *const std::ffi::c_char, pPropertyCount: *mut u32, pProperties: *mut svk::VkExtensionProperties) -> svk::VkResult {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn vkCreateInstance(&self, pCreateInfo: *const svk::VkInstanceCreateInfo, pAllocator: *const svk::VkAllocationCallbacks, pInstance: *mut svk::VkInstance) -> svk::VkResult {
        unsafe { (self.vk_create_instance)(pCreateInfo, pAllocator, pInstance) }
    }
    #[inline(always)]
    pub(in crate::wvk_library::dispatch_table) unsafe extern "system" fn vkCreateInstanceDummy(pCreateInfo: *const svk::VkInstanceCreateInfo, pAllocator: *const svk::VkAllocationCallbacks, pInstance: *mut svk::VkInstance) -> crate::svk::VkResult {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }
}
