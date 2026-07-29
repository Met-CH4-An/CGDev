// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// файл wvk_instance.rs
// file wvk_instance.rs
pub(crate) mod wvk_instance;
pub use wvk_instance::WvkInstance;

// файл wvk_instance_builder.rs
// file wvk_instance_builder.rs
pub(crate) mod wvk_instance_builder;
pub use wvk_instance_builder::WvkInstanceBuilder;

// файл wvk_instance_0_1_0_0.rs
// file wvk_instance_0_1_0_0.rs
pub(crate) mod wvk_instance_0_1_0_0;

#[cfg(test)]
mod _tests;
