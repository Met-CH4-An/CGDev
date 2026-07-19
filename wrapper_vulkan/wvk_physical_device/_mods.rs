// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// файл wvk_physical_device.rs
// file wvk_physical_device.rs
pub(crate) mod wvk_physical_device;
pub use wvk_physical_device::WvkPhysicalDevice;

// файл wvk_physical_device_builder.rs
// file wvk_physical_device_builder.rs
pub(crate) mod wvk_physical_device_builder;
pub use wvk_physical_device_builder::WvkPhysicalDeviceBuilder;