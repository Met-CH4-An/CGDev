// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::sync::Arc;
use crate::wvk_physical_device::wvk_physical_device::WvkPhysicalDevice;
use crate::wvk_physical_device::wvk_physical_device_x_properties::WvkPhysicalDeviceXProperties2;

impl WvkPhysicalDevice {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn wvkGetPhysicalDeviceProperties2(self: &Arc<Self>) -> WvkPhysicalDeviceXProperties2 {
        //self.wvk_instance.

        let mut wvk_physical_device_x_properties2_ = WvkPhysicalDeviceXProperties2::create();
        let p_pnext_ = wvk_physical_device_x_properties2_.buildPNext();

        //self.builder.wvk_instance.wvkGetPhysicalDeviceProperties2(self.vk_physical_device, p_pnext_);

        wvk_physical_device_x_properties2_
    }
}