// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::wvk::WvkVersion::{WVK_0_1_0_0, WVK_0_1_1_0, WVK_0_1_2_0, WVK_0_1_3_0,WVK_0_1_4_0};
use crate::wvk_error::{WvkError};
use crate::extensions;
use crate::wvk_instance::{WvkInstanceBuilder};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(in crate::wvk_instance) struct WvkInstanceDispatchTable {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Vulkan commands: Instance
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    pub(in crate::wvk_instance::dispatch_table) vk_destroy_instance: svk::PFN_vkDestroyInstance,

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Vulkan commands: VkPhysicalDevice
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // Vulkan 1.0

    pub(in crate::wvk_instance::dispatch_table) vk_enumerate_physical_devices: svk::PFN_vkEnumeratePhysicalDevices,
    pub(in crate::wvk_instance::dispatch_table) vk_get_physical_device_properties: svk::PFN_vkGetPhysicalDeviceProperties,
    pub(in crate::wvk_instance::dispatch_table) vk_enumerate_device_extension_properties: svk::PFN_vkEnumerateDeviceExtensionProperties,

    // Vulkan 1.1

    pub(in crate::wvk_instance::dispatch_table) vk_get_physical_device_properties_2: svk::PFN_vkGetPhysicalDeviceProperties2,

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // extensions
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    
    pub(in crate::wvk_instance::dispatch_table) vk_ext_debug_utils: extensions::VkExtDebugUtils,
}

macro_rules! init_command {
    ($builder: ident, $version: ident, $PFN: path, $vk_instance: ident, $name: expr, $dummy: path) => {{
        let output_ = if (*$builder).wvk_library.version() >= $version {
            (*$builder).wvk_library.wvkGetInstanceProcAddr::<$PFN>($vk_instance, $name)?
        }
        else {
            $dummy
        };

        output_
    }};
}

macro_rules! init_extension {
    ($builder: ident, $extension: path, $vk_instance: ident) => {{
        let output_ = if $builder.extension_name_vec
            .iter()
            .any(|v| {
                v.as_ref() == <$extension>::NAME
            })
        {
            <$extension>::createWithInstance(&(*$builder).wvk_library, $vk_instance)?
        }
        else {
            <$extension>::create()?
        };

        output_
    }};
}

impl WvkInstanceDispatchTable {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_instance) fn create(builder: &WvkInstanceBuilder, vk_instance: svk::VkInstance) -> Result<Self, WvkError> {
        Ok(Self {
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Vulkan commands: Instance
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            vk_destroy_instance: init_command!(builder, WVK_0_1_0_0, svk::PFN_vkDestroyInstance, vk_instance, c"vkDestroyInstance", Self::vkDestroyInstanceDummy),

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Vulkan commands: VkPhysicalDevice
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            // Vulkan 1.0

            vk_enumerate_physical_devices: init_command!(builder, WVK_0_1_0_0, svk::PFN_vkEnumeratePhysicalDevices, vk_instance, c"vkEnumeratePhysicalDevices", Self::vkEnumeratePhysicalDevicesDummy),
            vk_get_physical_device_properties: init_command!(builder, WVK_0_1_0_0, svk::PFN_vkGetPhysicalDeviceProperties, vk_instance, c"vkGetPhysicalDeviceProperties", Self::vkGetPhysicalDevicePropertiesDummy),
            vk_enumerate_device_extension_properties: init_command!(builder, WVK_0_1_0_0, svk::PFN_vkEnumerateDeviceExtensionProperties, vk_instance, c"vkEnumerateDeviceExtensionProperties", Self::vkEnumerateDeviceExtensionPropertiesDummy),
            
            // Vulkan 1.1

            vk_get_physical_device_properties_2: init_command!(builder, WVK_0_1_1_0, svk::PFN_vkGetPhysicalDeviceProperties2, vk_instance, c"vkGetPhysicalDeviceProperties2", Self::vkGetPhysicalDeviceProperties2Dummy),

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // extensions
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            vk_ext_debug_utils: init_extension!(builder, extensions::VkExtDebugUtils, vk_instance),
            
        })
    }
}

