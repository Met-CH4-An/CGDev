// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


use std::marker::PhantomData;
use std::sync::Arc;
use crate::wvk::{WvkBackend};
use crate::dispatch_table::{WvkDispatchTable, WVK_DISPATCH_TABLE_INSTANCE};
use crate::wvk_library::WvkLibrary;

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkInstance<TWvkBackend>
where
TWvkBackend : WvkBackend {
    pub(in crate::wvk_instance) _phantom_data: PhantomData<TWvkBackend>,

    /// Родительский WvkLibrary. 
    /// Parent class for WvkLibrary. 
    pub(in crate::wvk_instance) wvk_library: Arc<WvkLibrary<TWvkBackend>>,
    /// Таблица функций вулкана, которые создаются с помощью инстанса.
    /// Table of volcano functions that are created using an instance.
    pub(in crate::wvk_instance) wvk_dispatch_table: WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE>,
    /// Созданный VkInstance.
    /// Created by VkInstance.
    pub(in crate::wvk_instance) vk_instance: svk::VkInstance,
}
