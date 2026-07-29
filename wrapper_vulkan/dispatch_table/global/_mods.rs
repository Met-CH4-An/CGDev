// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::dispatch_table::{sealed, WvkDispatchTableLevel, WvkDispatchTableGlobal, WVK_DISPATCH_TABLE_GLOBAL};

// файл wvk_dispatch_table_global.rs
// file wvk_dispatch_table_global.rs
pub(crate) mod wvk_dispatch_table_global;

// файл wvk_dispatch_table_global_0_1_0_0.rs
// file wvk_dispatch_table_global_0_1_0_0.rs
pub(crate) mod wvk_dispatch_table_global_0_1_0_0;

// файл wvk_dispatch_table_global_0_1_1_0.rs
// file wvk_dispatch_table_global_0_1_1_0.rs
pub(crate) mod wvk_dispatch_table_global_0_1_1_0;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// приватная область
// private area
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl sealed::Sealed for WVK_DISPATCH_TABLE_GLOBAL {}
impl WvkDispatchTableLevel for WVK_DISPATCH_TABLE_GLOBAL {}
impl WvkDispatchTableGlobal for WVK_DISPATCH_TABLE_GLOBAL {}