// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use std::mem::MaybeUninit;
use crate::wvk::{WvkEnvironment_0_1_0_0};
use crate::dispatch_table::{WvkDispatchTableBuilder, WVK_DISPATCH_TABLE_GLOBAL, WVK_DISPATCH_TABLE_INSTANCE};
use crate::dispatch_table::wvk_dispatch_table::WvkDispatchTable;
use crate::wvk_error::WvkError;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные ассоциированные функции.
/// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные методы.
/// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend> WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE>
where
    TWvkBackend : WvkEnvironment_0_1_0_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::dispatch_table) fn s_create(wvk_dispatch_table_builder: &WvkDispatchTableBuilder<TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE>) -> Result<Self, WvkError> {
        let mut self_ = Self::s_initialize();

        self_ = Self::s_loadCommand(self_)?;
        self_ = Self::s_loadCommandWithInstance(self_, wvk_dispatch_table_builder.vk_instance__opt.unwrap())?;
        
        Ok(self_)
    }
}