// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::mem::MaybeUninit;
use crate::wvk::{ WvkEnvironment_0_1_1_0 };
use crate::wvk_physical_device::wvk_physical_device::WvkPhysicalDevice;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные ассоциированные функции.
/// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a, TWvkBackend> WvkPhysicalDevice<TWvkBackend>
where
    TWvkBackend : WvkEnvironment_0_1_1_0 {
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные методы.
/// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a, TWvkBackend> WvkPhysicalDevice<TWvkBackend>
where
    TWvkBackend : WvkEnvironment_0_1_1_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn wvkGetPhysicalDeviceProperties2(&self) -> svk::svk_structures::VkPhysicalDeviceProperties2 {
        let mut vk_props_ = MaybeUninit::<svk::svk_structures::VkPhysicalDeviceProperties2>::uninit();

        self.wvk_instance_arc.getDispatchTable().vkGetPhysicalDeviceProperties2(self.vk_physical_device, vk_props_.as_mut_ptr());

        unsafe { vk_props_.assume_init() }
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
impl<TWvkBackend> WvkPhysicalDevice<TWvkBackend>
where
TWvkBackend : WvkEnvironment_0_1_1_0 {
    
}