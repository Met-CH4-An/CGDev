// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

mod wvk_physical_device;
pub use wvk_physical_device::WvkPhysicalDevice;

mod wvk_physical_device_builder;
pub use wvk_physical_device_builder::WvkPhysicalDeviceBuilder;

mod wvk_physical_device_0_1_0_0;
mod wvk_physical_device_0_1_1_0;
mod wvk_physical_device_0_1_2_0;
mod wvk_physical_device_0_1_3_0;
mod wvk_physical_device_0_1_4_0;

mod wvk_physical_device_x_properties;