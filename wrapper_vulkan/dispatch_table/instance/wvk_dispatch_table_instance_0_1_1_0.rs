// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::wvk::{ WvkBackend_0_1_1_0 };
use crate::dispatch_table::{WvkDispatchTableInstance, WVK_DISPATCH_TABLE_GLOBAL};
use crate::dispatch_table::wvk_dispatch_table::WvkDispatchTable;
use crate::wvk_error::WvkError;

impl<TWvkBackend, TLevel> WvkDispatchTable<TWvkBackend, TLevel>
where
TWvkBackend: WvkBackend_0_1_1_0,
TLevel: WvkDispatchTableInstance, {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn vkGetPhysicalDeviceProperties2(&self, physicalDevice: svk::VkPhysicalDevice, pProperties: *mut svk::VkPhysicalDeviceProperties2) {
        unsafe { self.vk_get_physical_device_properties_2.assume_init()(physicalDevice, pProperties) }
    }
}