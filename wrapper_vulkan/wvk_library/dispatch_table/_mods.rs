// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[cfg(target_os = "windows")]
#[path = "mswindows/_mods.rs"]
mod mswindows;
#[cfg(target_os = "windows")]
pub(in crate::wvk_library::dispatch_table) type WvkDispatchTablePlatform = mswindows::WvkDispatchTableMSWindows;

mod dispatch_table;
pub(in crate::wvk_library) use dispatch_table::WvkDispatchTable;
