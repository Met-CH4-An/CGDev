// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use crate::wvk::{WvkBackend};
use crate::wvk_library::dispatch_table::{WvkDispatchTable};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkLibrary<TWvkBackend>
where
TWvkBackend : WvkBackend {
    pub(in crate::wvk_library) phantom : PhantomData<TWvkBackend>,
    /// Таблица функций вулкана, которые создаются без инстанса и без логического устройства. Глобальные функции.
    /// Table of Vulcan functions that are created without an instance and without a logical device. Global functions.
    pub(in crate::wvk_library) wvk_dispatch_table : WvkDispatchTable<TWvkBackend>,
}