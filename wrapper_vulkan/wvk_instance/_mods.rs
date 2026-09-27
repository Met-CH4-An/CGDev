// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

mod wvk_instance;
pub use wvk_instance::WvkInstance;

mod wvk_instance_builder;
pub use wvk_instance_builder::WvkInstanceBuilder;

#[path = "dispatch_table/_mods.rs"]
pub(in crate::wvk_instance) mod dispatch_table;

mod wvk_instance_0_1_0_0;
mod wvk_instance_0_1_1_0;
mod wvk_instance_0_1_2_0;
mod wvk_instance_0_1_3_0;
mod wvk_instance_0_1_4_0;

#[cfg(test)]
mod _tests;
