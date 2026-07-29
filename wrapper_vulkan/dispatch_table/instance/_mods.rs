// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// connecting modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::dispatch_table::{sealed, WvkDispatchTableLevel, WvkDispatchTableGlobal, WvkDispatchTableInstance, WVK_DISPATCH_TABLE_INSTANCE};

// файл wvk_dispatch_table_instance.rs
// file wvk_dispatch_table_instance.rs
pub(crate) mod wvk_dispatch_table_instance;

// файл wvk_dispatch_table_instance_0_1_0_0.rs
// file wvk_dispatch_table_instance_0_1_0_0.rs
pub(crate) mod wvk_dispatch_table_instance_0_1_0_0;

// файл wvk_dispatch_table_instance_0_1_1_0.rs
// file wvk_dispatch_table_instance_0_1_1_0.rs
pub(crate) mod wvk_dispatch_table_instance_0_1_1_0;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// приватная область
// private area
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl sealed::Sealed for WVK_DISPATCH_TABLE_INSTANCE {}
impl WvkDispatchTableLevel for WVK_DISPATCH_TABLE_INSTANCE {}
impl WvkDispatchTableGlobal for WVK_DISPATCH_TABLE_INSTANCE {}
impl WvkDispatchTableInstance for WVK_DISPATCH_TABLE_INSTANCE {}