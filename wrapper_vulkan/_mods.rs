// SPDX-License-Identifier: None
// Copyright (c) 2026 None

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

#[cfg(not(any(target_os = "windows")))]
compile_error!("Платформа не поддерживается. The platform is not supported.");

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub use svk;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// файл wvk.rs
// file wvk.rs
pub mod wvk;

// файл wvk_error.rs
// file wvk_error.rs
pub mod wvk_error;

// папка dispatch_table
// folder dispatch_table
#[path = "dispatch_table/_mods.rs"]
pub mod dispatch_table;

// папка с WvkLibrary
// folder with WvkLibrary
#[path = "wvk_library/_mods.rs"]
pub mod wvk_library;

// папка с WvkInstance
// folder with WvkInstance
#[path = "wvk_instance/_mods.rs"]
pub mod wvk_instance;

// папка с WvkPhysicalDevice
// folder with WvkPhysicalDevice
#[path = "wvk_physical_device/_mods.rs"]
pub mod wvk_physical_device;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// приватная область
// private area
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// папка с расширениями
// folder with extensions
#[path = "extensions/_mods.rs"]
pub(crate) mod extensions;

