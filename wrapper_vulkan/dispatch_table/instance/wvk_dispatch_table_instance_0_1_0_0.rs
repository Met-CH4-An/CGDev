// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::wvk::{ WvkBackend_0_1_0_0 };
use crate::dispatch_table::{WvkDispatchTableInstance, WVK_DISPATCH_TABLE_GLOBAL};
use crate::dispatch_table::wvk_dispatch_table::WvkDispatchTable;
use crate::wvk_error::WvkError;

impl<TWvkBackend, TLevel> WvkDispatchTable<TWvkBackend, TLevel>
where
TWvkBackend: WvkBackend_0_1_0_0,
TLevel: WvkDispatchTableInstance, {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn vkDestroyInstance(&self, instance: svk::VkInstance, pAllocator: *const svk::VkAllocationCallbacks, ) {
        unsafe { self.vk_destroy_instance.assume_init()(instance, pAllocator) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn vkEnumeratePhysicalDevices(&self, instance: svk::VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut svk::VkPhysicalDevice) -> svk::VkResult {
        unsafe { self.vk_enumerate_physical_devices.assume_init()(instance, pPhysicalDeviceCount, pPhysicalDevices) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn vkGetPhysicalDeviceProperties(&self, physicalDevice: svk::VkPhysicalDevice, pProperties: *mut svk::VkPhysicalDeviceProperties) {
        unsafe { self.vk_get_physical_device_properties.assume_init()(physicalDevice, pProperties) }
    }
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные методы.
// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend, TLevel> WvkDispatchTable<TWvkBackend, TLevel>
where
TWvkBackend: WvkBackend_0_1_0_0,
TLevel: WvkDispatchTableInstance, {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Получение адресов команд вулкана, которые можно получить с помощью экземпляра.
    /// Getting the addresses of the volcano commands that can be obtained using the instance.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::dispatch_table) fn loadCommandWithInstance_0_1_0_0(&mut self, wvk_dispatch_table_global: &WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>, vk_instance: svk::VkInstance) -> Result<(), WvkError> {
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Vulkan commands: Instance
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        self.vk_destroy_instance.write(
            self.loadCommandAddress::<svk::PFN_vkDestroyInstance>(vk_instance, c"vkDestroyInstance")? );

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Vulkan commands: VkPhysicalDevice
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        self.vk_enumerate_physical_devices.write(self.loadCommandAddress::<svk::PFN_vkEnumeratePhysicalDevices>(vk_instance, c"vkEnumeratePhysicalDevices")? );
        self.vk_get_physical_device_properties.write(self.loadCommandAddress::<svk::PFN_vkGetPhysicalDeviceProperties>(vk_instance, c"vkGetPhysicalDeviceProperties")? );

        Ok(())
    }
}

