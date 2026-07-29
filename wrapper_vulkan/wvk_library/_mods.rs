// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// файл wvk_library.rs
// file wvk_library.rs
pub(crate) mod wvk_library;
pub use wvk_library::WvkLibrary;

// файл wvk_library_builder.rs
// file wvk_library_builder.rs
pub(crate) mod wvk_library_builder;
pub use wvk_library_builder::WvkLibraryBuilder;

// файл wvk_library_0_1_0_0.rs
// file wvk_library_0_1_0_0.rs
pub(crate) mod wvk_library_0_1_0_0;

// файл wvk_library_0_1_1_0.rs
// file wvk_library_0_1_1_0.rs
pub(crate) mod wvk_library_0_1_1_0;

#[cfg(test)]
mod _tests;