// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::mem::MaybeUninit;
use std::sync::Arc;
use crate::wvk_call_with_check;
use crate::wvk_error::{ WvkError, WvkErrorType };
use crate::wvk_instance::wvk_instance::WvkInstance;
use crate::wvk_physical_device::{WvkPhysicalDeviceBuilder, WvkPhysicalDevice};

impl WvkInstance {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn wvkDestroyInstance(&self) {
        self.wvk_dispatch_table.vkDestroyInstance(self.vk_instance, std::ptr::null_mut());
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn wvkEnumeratePhysicalDevices(self: &Arc<Self>) -> Result<Vec<Arc<WvkPhysicalDevice>>, WvkError> {
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Получаем количество физических устройств VkPhysicalDevice.
        // Get the number of physical devices VkPhysicalDevice.
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let mut count_ : u32 = 0;
        wvk_call_with_check!(
            self.wvk_dispatch_table.vkEnumeratePhysicalDevices(self.vk_instance, &mut count_, std::ptr::null_mut())
        );

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Выделить место для данных, исходя из полученного количества.
        // Allocate space for data based on the received quantity.
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let mut vk_physical_devices_ = Vec::<svk::VkPhysicalDevice>::with_capacity(count_ as usize);
        unsafe { vk_physical_devices_.set_len(count_ as usize) }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Получаем список физических устройств VkPhysicalDevice.
        // We get a list of physical devices VkPhysicalDevice.
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        wvk_call_with_check!(
            self.wvk_dispatch_table.vkEnumeratePhysicalDevices(self.vk_instance, &mut count_, vk_physical_devices_.as_mut_ptr())
        );

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Перебираем полученный список физических устройств VkPhysicalDevice и формируем обертки WvkPhysicalDevice.
        // We iterate over the received list of physical devices VkPhysicalDevice and form WvkPhysicalDevice wrappers.
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let wvk_physical_devices_ = vk_physical_devices_
            .iter()
            .map(|v| {
                WvkPhysicalDeviceBuilder::create(*v, &self).build()
            })
            .collect::<Result<Vec<Arc<WvkPhysicalDevice>>, WvkError>>()?;

        Ok(wvk_physical_devices_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn vkGetPhysicalDeviceProperties(self: &Arc<Self>, vk_physical_device: svk::VkPhysicalDevice) -> svk::VkPhysicalDeviceProperties {
        let mut output_ = MaybeUninit::<svk::VkPhysicalDeviceProperties>::uninit();

        self.wvk_dispatch_table.vkGetPhysicalDeviceProperties(vk_physical_device, output_.as_mut_ptr());

        unsafe {output_.assume_init()}
    }
}

