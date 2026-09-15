// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use crate::wvk::{WvkBackend};
use crate::wvk_error::WvkError;
use crate::dispatch_table::{WvkDispatchTable, WvkDispatchTableBuilder, WVK_DISPATCH_TABLE_GLOBAL};
use crate::wvk_library::platform::WvkLibraryPlatform;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkLibrary<TWvkBackend>
where
TWvkBackend : WvkBackend {
    pub(in crate::wvk_library) _phantom : PhantomData<TWvkBackend>,
    /// Платформозависимая часть.
    /// Platform-specific section.
    pub(in crate::wvk_library) _wvk_library_platform: WvkLibraryPlatform,
    /// Таблица функций вулкана, которые создаются без инстанса и без логического устройства. Глобальные функции.
    /// Table of Vulcan functions that are created without an instance and without a logical device. Global functions.
    pub(in crate) wvk_dispatch_table : WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>,
}