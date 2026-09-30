// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::sync::Arc;
use crate::wvk_physical_device::wvk_physical_device::WvkPhysicalDevice;

impl WvkPhysicalDevice {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn wvkGetPhysicalDeviceProperties(self: &Arc<Self>) -> svk::VkPhysicalDeviceProperties {
        self.builder.wvk_instance.wvkGetPhysicalDeviceProperties(self.builder.vk_physical_device)
    }
}