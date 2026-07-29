// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use std::mem::MaybeUninit;
use crate::wvk::{ WvkEnvironment_0_1_0_0 };
use crate::wvk_error::WvkError;
use crate::wvk_physical_device::wvk_physical_device::WvkPhysicalDevice;
use crate::wvk_physical_device::wvk_physical_device_builder::WvkPhysicalDeviceBuilder;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные ассоциированные функции.
/// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a, TWvkBackend> WvkPhysicalDevice<TWvkBackend>
where
TWvkBackend : WvkEnvironment_0_1_0_0 {
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные методы.
/// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a, TWvkBackend> WvkPhysicalDevice<TWvkBackend>
where
TWvkBackend : WvkEnvironment_0_1_0_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn wvkGetPhysicalDeviceProperties(&self) -> svk::svk_structures::VkPhysicalDeviceProperties {
        let mut vk_props_ = MaybeUninit::<svk::svk_structures::VkPhysicalDeviceProperties>::uninit();

        self.wvk_instance_arc.getDispatchTable().vkGetPhysicalDeviceProperties(self.vk_physical_device, vk_props_.as_mut_ptr());

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
TWvkBackend : WvkEnvironment_0_1_0_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_physical_device) fn s_create(wvk_physical_device_builder__ref: & WvkPhysicalDeviceBuilder<TWvkBackend>) -> Result<Self, WvkError> {
        Ok(
            Self {
                phantom_data: PhantomData,
                wvk_instance_arc: wvk_physical_device_builder__ref.wvk_instance_arc.clone(),
                vk_physical_device: wvk_physical_device_builder__ref.vk_physical_device,
            }
        )
    }
}