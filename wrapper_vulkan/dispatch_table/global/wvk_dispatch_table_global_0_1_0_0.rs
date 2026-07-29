// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::wvk::{WvkEnvironment_0_1_0_0};
use crate::dispatch_table::{WvkDispatchTableGlobal};
use crate::dispatch_table::wvk_dispatch_table::WvkDispatchTable;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные ассоциированные функции.
/// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные методы.
/// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend, TLevel> WvkDispatchTable<TWvkBackend, TLevel>
where
    TWvkBackend : WvkEnvironment_0_1_0_0,
    TLevel : WvkDispatchTableGlobal, {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn vkGetInstanceProcAddr(&self, instance: svk::svk_types::VkInstance, pName: *const std::ffi::c_char) -> *mut std::ffi::c_void {
        unsafe { self.vk_get_instance_proc_addr.assume_init()(instance, pName) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn vkEnumerateInstanceLayerProperties(&self, pPropertyCount: *mut u32, pProperties: *mut svk::svk_structures::VkLayerProperties) -> svk::svk_enums::VkResult {
        unsafe { self.vk_enumerate_instance_layer_properties.assume_init()(pPropertyCount, pProperties) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn vkEnumerateInstanceExtensionProperties(&self, pLayerName: *const std::ffi::c_char, pPropertyCount: *mut u32, pProperties: *mut svk::svk_structures::VkExtensionProperties) -> svk::svk_enums::VkResult {
        unsafe { self.vk_enumerate_instance_extension_properties.assume_init()(pLayerName, pPropertyCount, pProperties) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn vkCreateInstance(&self, pCreateInfo: *const svk::svk_structures::VkInstanceCreateInfo, pAllocator: *const svk::svk_structures::VkAllocationCallbacks, pInstance: *mut svk::svk_types::VkInstance) -> svk::svk_enums::VkResult {
        unsafe { self.vk_create_instance.assume_init()(pCreateInfo, pAllocator, pInstance) }
    }

}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// приватная область
// private area
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Приватные ассоциированные функции.
/// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend, TLevel> WvkDispatchTable<TWvkBackend, TLevel>
where
TWvkBackend : WvkEnvironment_0_1_0_0,
TLevel : WvkDispatchTableGlobal, {}
