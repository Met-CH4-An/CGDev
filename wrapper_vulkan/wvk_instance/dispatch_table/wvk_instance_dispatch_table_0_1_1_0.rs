// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::panic::panic_any;
use crate::wvk_instance::dispatch_table::WvkInstanceDispatchTable;

impl WvkInstanceDispatchTable {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(in crate::wvk_instance) fn vkGetPhysicalDeviceProperties2(&self, physicalDevice: svk::VkPhysicalDevice, pProperties: *mut svk::VkPhysicalDeviceProperties2) {
        unsafe { (self.vk_get_physical_device_properties_2)(physicalDevice, pProperties) }
    }
    #[inline(always)]
    pub(in crate::wvk_instance::dispatch_table) unsafe extern "system" fn vkGetPhysicalDeviceProperties2Dummy(physicalDevice: svk::VkPhysicalDevice, pProperties: *mut svk::VkPhysicalDeviceProperties2) {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }
}
