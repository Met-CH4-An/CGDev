// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

mod dispatch_table_mswindows;
pub(in crate::wvk_library::dispatch_table) use dispatch_table_mswindows::WvkDispatchTableMSWindows;
