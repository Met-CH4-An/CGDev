// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

mod wvk_library;
pub use wvk_library::WvkLibrary;

mod wvk_library_builder;
pub use wvk_library_builder::WvkLibraryBuilder;

#[path = "platform/_mods.rs"]
pub(in crate::wvk_library) mod platform;

#[path = "dispatch_table/_mods.rs"]
pub(in crate::wvk_library) mod dispatch_table;

mod wvk_library_0_1_0_0;
mod wvk_library_0_1_1_0;
mod wvk_library_0_1_2_0;
mod wvk_library_0_1_3_0;
mod wvk_library_0_1_4_0;

#[cfg(test)]
mod _tests;