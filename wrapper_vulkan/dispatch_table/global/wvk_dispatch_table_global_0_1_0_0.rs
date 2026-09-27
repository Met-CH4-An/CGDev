// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::dispatch_table::{WvkDispatchTableGlobal};
use crate::dispatch_table::wvk_dispatch_table::WvkDispatchTable;

#[cfg(feature = "vulkan_1_0")]
impl<TLevel> WvkDispatchTable<TLevel>
where
TLevel: WvkDispatchTableGlobal, {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn vkGetInstanceProcAddr(&self, instance: svk::VkInstance, pName: *const std::ffi::c_char) -> *mut std::ffi::c_void {
        unsafe { self.vk_get_instance_proc_addr.assume_init()(instance, pName) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn vkEnumerateInstanceLayerProperties(&self, pPropertyCount: *mut u32, pProperties: *mut svk::VkLayerProperties) -> svk::VkResult {
        unsafe { self.vk_enumerate_instance_layer_properties.assume_init()(pPropertyCount, pProperties) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn vkEnumerateInstanceExtensionProperties(&self, pLayerName: *const std::ffi::c_char, pPropertyCount: *mut u32, pProperties: *mut svk::VkExtensionProperties) -> svk::VkResult {
        unsafe { self.vk_enumerate_instance_extension_properties.assume_init()(pLayerName, pPropertyCount, pProperties) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn vkCreateInstance(&self, pCreateInfo: *const svk::VkInstanceCreateInfo, pAllocator: *const svk::VkAllocationCallbacks, pInstance: *mut svk::VkInstance) -> svk::VkResult {
        unsafe { self.vk_create_instance.assume_init()(pCreateInfo, pAllocator, pInstance) }
    }

}
