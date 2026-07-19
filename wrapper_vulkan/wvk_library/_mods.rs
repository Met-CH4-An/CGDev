// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub(crate) mod wvk_library;
pub use wvk_library::WvkLibrary;

pub(crate) mod wvk_library_builder;
pub use wvk_library_builder::WvkLibraryBuilder;

pub(in crate::wvk_library) mod wvk_library_0_1_0_0;
pub(in crate::wvk_library) mod wvk_library_0_1_1_0;