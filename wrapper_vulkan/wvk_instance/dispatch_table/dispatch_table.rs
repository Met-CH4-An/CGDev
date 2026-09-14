// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use std::mem::MaybeUninit;
use crate::wvk::{WvkBackend, WvkBackend_0_1_0_0};
use crate::wvk_error::{WvkError};
use crate::wvk_instance::WvkInstanceBuilder;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkDispatchTable<TWvkBackend> {
    _phantom_data: PhantomData<TWvkBackend>,

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Vulkan commands: Instance
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    pub(in crate::wvk_instance) vk_destroy_instance : MaybeUninit<svk::PFN_vkDestroyInstance>,

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Vulkan commands: VkPhysicalDevice
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // Vulkan 1.0

    pub(in crate::wvk_instance) vk_enumerate_physical_devices : MaybeUninit<svk::PFN_vkEnumeratePhysicalDevices>,
    pub(in crate::wvk_instance) vk_get_physical_device_properties : MaybeUninit<svk::PFN_vkGetPhysicalDeviceProperties>,

    // Vulkan 1.1

    pub(in crate::wvk_instance) vk_get_physical_device_properties_2 : MaybeUninit<svk::PFN_vkGetPhysicalDeviceProperties2>,
}

impl<TWvkBackend> WvkDispatchTable<TWvkBackend>
where
TWvkBackend: WvkBackend_0_1_0_0
{
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_instance) fn create(wvk_instance_builder: &WvkInstanceBuilder<TWvkBackend>, vk_instance: svk::VkInstance) -> Result<Self, WvkError> {
        let mut self_ = Self {
            _phantom_data: PhantomData,

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Vulkan commands: Instance
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            vk_destroy_instance: MaybeUninit::uninit(),

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Vulkan commands: VkPhysicalDevice
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            // Vulkan 1.0

            vk_enumerate_physical_devices: MaybeUninit::uninit(),
            vk_get_physical_device_properties: MaybeUninit::uninit(),

            // Vulkan 1.1

            vk_get_physical_device_properties_2: MaybeUninit::uninit(),
        };

        self_ = Self::loadCommand(self_, wvk_instance_builder, vk_instance)?;

        Ok(self_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Получение адресов команд вулкана, которые можно получить с помощью экземпляра.
    /// Getting the addresses of the volcano commands that can be obtained using the instance.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn loadCommand(mut dispatch_table: Self, wvk_instance_builder: &WvkInstanceBuilder<TWvkBackend>, vk_instance: svk::VkInstance) -> Result<Self, WvkError> {
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Vulkan commands: Instance
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        // Vulkan 1.0

        dispatch_table.vk_destroy_instance.write(
            wvk_instance_builder.wvk_library.wvkGetInstanceProcAddr::<svk::PFN_vkDestroyInstance>(vk_instance, c"vkDestroyInstance")?);

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Vulkan commands: VkPhysicalDevice
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        // Vulkan 1.0

        dispatch_table.vk_enumerate_physical_devices.write(wvk_instance_builder.wvk_library.wvkGetInstanceProcAddr::<svk::PFN_vkEnumeratePhysicalDevices>(vk_instance, c"vkEnumeratePhysicalDevices")?);
        dispatch_table.vk_get_physical_device_properties.write(wvk_instance_builder.wvk_library.wvkGetInstanceProcAddr::<svk::PFN_vkGetPhysicalDeviceProperties>(vk_instance, c"vkGetPhysicalDeviceProperties")?);

        // Vulkan 1.1

        if TWvkBackend::WVK_ENCODED_VULKAN_VERSION >= svk::VK_MAKE_API_VERSION(0, 1, 1, 0) {
            dispatch_table.vk_get_physical_device_properties_2.write(wvk_instance_builder.wvk_library.wvkGetInstanceProcAddr::<svk::PFN_vkGetPhysicalDeviceProperties2>(vk_instance, c"vkGetPhysicalDeviceProperties2")?);
        }

        Ok(dispatch_table)
    }
}


