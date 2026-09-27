// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use std::sync::Arc;
use crate::wvk::{ WvkBackend_0_1_0_0 };
use crate::wvk_error::WvkError;
use crate::wvk_physical_device::wvk_physical_device::WvkPhysicalDevice;
use crate::wvk_physical_device::wvk_physical_device_builder::WvkPhysicalDeviceBuilder;

impl<'a> WvkPhysicalDevice {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //pub fn wvkGetPhysicalDeviceProperties(self: & Arc<Self>) -> svk::VkPhysicalDeviceProperties {
    pub fn wvkGetPhysicalDeviceProperties(&self) -> svk::VkPhysicalDeviceProperties {
        self.wvk_instance.vkGetPhysicalDeviceProperties(self.vk_physical_device)
    }
}

impl WvkPhysicalDevice {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_physical_device) fn create(builder: WvkPhysicalDeviceBuilder) -> Result<Arc<Self>, WvkError> {
        let self_ = Self {
            wvk_instance: builder.wvk_instance.clone(),
            vk_physical_device: builder.vk_physical_device,
        };

        Ok(Arc::new(self_))
    }
}