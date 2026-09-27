// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

mod wvk_library_dispatch_table;
pub(in crate::wvk_library) use wvk_library_dispatch_table::WvkLibraryDispatchTable;

mod wvk_library_dispatch_table_builder;
pub(in crate::wvk_library) use wvk_library_dispatch_table_builder::WvkLibraryDispatchTableBuilder;

mod wvk_library_dispatch_table_0_1_0_0;
mod wvk_library_dispatch_table_0_1_1_0;