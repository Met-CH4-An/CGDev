// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use std::sync::Arc;
use crate::wvk::{WvkBackend, WvkBackend_0_1_0_0};
use crate::wvk_error::WvkError;
use crate::wvk_instance::WvkInstance;
use crate::wvk_physical_device::wvk_physical_device::WvkPhysicalDevice;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkPhysicalDeviceBuilder<TWvkBackend>
where
TWvkBackend : WvkBackend {
    phantom_data: PhantomData<TWvkBackend>,

    /// Обёртка моего враппера WvkInstance для управления VkInstance. Нужна, т.к. хранит таблицу диспетчеризации инстанс команд вулкана.
    /// A wrapper for my WvkInstance wrapper for managing VkInstance. It's needed because it stores the Vulkan instance command dispatch table.
    pub(in crate::wvk_physical_device) wvk_instance_arc: Arc<WvkInstance<TWvkBackend>>,
    /// Полученный через vkEnumeratePhysicalDevices тип физического устройства. VkPhysicalDevice
    /// Physical device type obtained via vkEnumeratePhysicalDevices. VkPhysicalDevice
    pub(in crate::wvk_physical_device) vk_physical_device : svk::VkPhysicalDevice,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные ассоциированные функции.
/// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a, TWvkBackend> WvkPhysicalDeviceBuilder<TWvkBackend>
where
TWvkBackend : WvkBackend_0_1_0_0 {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn create(vk_physical_device : svk::VkPhysicalDevice, wvk_instance_arc: Arc<WvkInstance<TWvkBackend>>) -> Self {
        Self {
            phantom_data: PhantomData,
            vk_physical_device: vk_physical_device,
            wvk_instance_arc: wvk_instance_arc,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные методы.
/// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a, TWvkBackend> WvkPhysicalDeviceBuilder<TWvkBackend>
where
TWvkBackend : WvkBackend {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn build(self) -> Result<WvkPhysicalDevice<TWvkBackend>, WvkError>
    where TWvkBackend: WvkBackend_0_1_0_0 {
        WvkPhysicalDevice::create(&self)
    }
}