// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::wvk::{ WvkBackend_0_1_1_0 };
use crate::wvk_physical_device::wvk_physical_device::WvkPhysicalDevice;
use crate::wvk_physical_device::wvk_physical_device_x_properties::WvkPhysicalDeviceXProperties2;

impl<'a, TWvkBackend> WvkPhysicalDevice<TWvkBackend>
where
    TWvkBackend : WvkBackend_0_1_1_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn wvkGetPhysicalDeviceProperties2(&self) -> WvkPhysicalDeviceXProperties2 {
        let mut wvk_physical_device_x_properties2_ = WvkPhysicalDeviceXProperties2::create();
        let p_pnext_ = wvk_physical_device_x_properties2_.buildPNext();

        self.wvk_instance.vkGetPhysicalDeviceProperties2(self.vk_physical_device, p_pnext_);

        wvk_physical_device_x_properties2_
    }
}