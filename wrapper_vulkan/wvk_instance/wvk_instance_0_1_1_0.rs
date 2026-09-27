// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::mem::MaybeUninit;
use crate::wvk_instance::wvk_instance::WvkInstance;

#[cfg(feature = "vulkan_1_1")]
impl WvkInstance {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn vkGetPhysicalDeviceProperties2(&self, physicalDevice: svk::VkPhysicalDevice, p_next: *mut std::ffi::c_void) -> svk::VkPhysicalDeviceProperties2 {
        let mut pProperties_ = MaybeUninit::<svk::VkPhysicalDeviceProperties2>::uninit();

        unsafe {
            (*pProperties_.as_mut_ptr()).sType = svk::VkStructureType::VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_PROPERTIES_2;
            (*pProperties_.as_mut_ptr()).pNext = p_next;
        };

        self.wvk_dispatch_table.vkGetPhysicalDeviceProperties2(physicalDevice, pProperties_.as_mut_ptr());

        unsafe {pProperties_.assume_init()}
    }
}