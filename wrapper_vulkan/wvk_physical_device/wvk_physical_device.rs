// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use std::sync::Arc;
use crate::wvk::WvkBackend;
use crate::wvk_instance::WvkInstance;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkPhysicalDevice {
    /// Обёртка моего враппера WvkInstance для управления VkInstance. Нужна, т.к. хранит таблицу диспетчеризации инстанс команд вулкана.
    /// A wrapper for my WvkInstance wrapper for managing VkInstance. It's needed because it stores the Vulkan instance command dispatch table.
    pub(in crate::wvk_physical_device) wvk_instance: Arc<WvkInstance>,
    /// Полученный через vkEnumeratePhysicalDevices тип физического устройства. VkPhysicalDevice
    /// Physical device type obtained via vkEnumeratePhysicalDevices. VkPhysicalDevice
    pub(in crate::wvk_physical_device) vk_physical_device : svk::VkPhysicalDevice,
}