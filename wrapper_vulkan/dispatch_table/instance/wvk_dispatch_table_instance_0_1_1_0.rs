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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные ассоциированные функции.
/// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные методы.
/// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// приватная область
// private area
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Приватные ассоциированные функции.
/// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend, TLevel> WvkDispatchTable<TWvkBackend, TLevel>
where
TWvkBackend: WvkBackend_0_1_1_0,
TLevel: WvkDispatchTableInstance, {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Получение адресов команд вулкана, которые можно получить с помощью экземпляра.
    /// Getting the addresses of the volcano commands that can be obtained using the instance.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::dispatch_table) fn loadCommandWithInstance_0_1_1_0(&mut self, wvk_dispatch_table_global: &WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>, vk_instance: svk::VkInstance) -> Result<(), WvkError> {
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Vulkan commands: VkPhysicalDevice
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        self.vk_get_physical_device_properties_2.write(self.loadCommandAddress::<svk::PFN_vkGetPhysicalDeviceProperties2>(vk_instance, c"vkGetPhysicalDeviceProperties2")? );

        Ok(())
    }
}