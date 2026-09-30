// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::panic::panic_any;
use crate::wvk_instance::dispatch_table::WvkInstanceDispatchTable;

impl WvkInstanceDispatchTable {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(in crate::wvk_instance) fn vkDestroyInstance(
        &self,
        instance: svk::VkInstance,
        pAllocator: *const svk::VkAllocationCallbacks)
    {
        unsafe { (self.vk_destroy_instance)(instance, pAllocator) }
    }
    #[inline(always)]
    pub(in crate::wvk_instance::dispatch_table) unsafe extern "system" fn vkDestroyInstanceDummy(
        instance: svk::VkInstance,
        pAllocator: *const svk::VkAllocationCallbacks)
    {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(in crate::wvk_instance) fn vkEnumeratePhysicalDevices(
        &self,
        instance: svk::VkInstance,
        pPhysicalDeviceCount: *mut u32,
        pPhysicalDevices: *mut svk::VkPhysicalDevice)
        -> svk::VkResult
    {
        unsafe { (self.vk_enumerate_physical_devices)(instance, pPhysicalDeviceCount, pPhysicalDevices) }
    }
    #[inline(always)]
    pub(in crate::wvk_instance::dispatch_table) unsafe extern "system" fn vkEnumeratePhysicalDevicesDummy(
        instance: svk::VkInstance,
        pPhysicalDeviceCount: *mut u32,
        pPhysicalDevices: *mut svk::VkPhysicalDevice)
        -> svk::VkResult
    {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(in crate::wvk_instance) fn vkGetPhysicalDeviceProperties(
        &self,
        physicalDevice: svk::VkPhysicalDevice,
        pProperties: *mut svk::VkPhysicalDeviceProperties)
    {
        unsafe { (self.vk_get_physical_device_properties)(physicalDevice, pProperties) }
    }
    #[inline(always)]
    pub(in crate::wvk_instance::dispatch_table) unsafe extern "system" fn vkGetPhysicalDevicePropertiesDummy(
        physicalDevice: svk::VkPhysicalDevice,
        pProperties: *mut svk::VkPhysicalDeviceProperties)
    {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(in crate::wvk_instance) fn vkEnumerateDeviceExtensionProperties(
        &self,
        physicalDevice: svk::VkPhysicalDevice,
        pLayerName: *const std::ffi::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut svk::VkExtensionProperties)
        -> svk::VkResult
    {
        unsafe { (self.vk_enumerate_device_extension_properties)(physicalDevice, pLayerName, pPropertyCount, pProperties) }
    }
    #[inline(always)]
    pub(in crate::wvk_instance::dispatch_table) unsafe extern "system" fn vkEnumerateDeviceExtensionPropertiesDummy(
        physicalDevice: svk::VkPhysicalDevice,
        pLayerName: *const std::ffi::c_char,
        pPropertyCount: *mut u32,
        pProperties: *mut svk::VkExtensionProperties)
        -> svk::VkResult
    {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }
}
