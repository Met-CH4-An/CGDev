// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[path = "dispatch_table/_mods.rs"]
mod dispatch_table;

mod wvk_instance;
pub use wvk_instance::WvkInstance;

mod wvk_instance_builder;
pub use wvk_instance_builder::WvkInstanceBuilder;

mod wvk_instance_0_1_0_0;

#[cfg(test)]
mod _tests;
