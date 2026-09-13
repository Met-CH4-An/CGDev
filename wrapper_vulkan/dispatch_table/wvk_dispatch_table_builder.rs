// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;

use crate::wvk::{WvkBackend, WvkBackend_0_1_0_0, WvkBackend_0_1_1_0, WvkBackend_0_1_2_0, WvkBackend_0_1_3_0, WvkBackend_Max};
use crate::wvk_error::WvkError;
use crate::dispatch_table::{ WVK_DISPATCH_TABLE_GLOBAL, WVK_DISPATCH_TABLE_INSTANCE };
use crate::dispatch_table::wvk_dispatch_table::WvkDispatchTable;

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkDispatchTableBuilder<'a, TWvkBackend, TLevel> {
    phantom_data: PhantomData<(TWvkBackend, TLevel)>,
    /// Экземпляр вулкана.
    /// Vulkan instance.
    pub(in crate::dispatch_table) vk_instance__opt: Option<svk::VkInstance>,
    ///
    ///
    pub(in crate::dispatch_table) wvk_dispatch_table_global__opt: Option<&'a WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные ассоциированные функции.
/// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a, TWvkBackend> WvkDispatchTableBuilder<'a, TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>
where
TWvkBackend: WvkBackend {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create() -> Self {
        Self {
            phantom_data: PhantomData,
            vk_instance__opt: None,
            wvk_dispatch_table_global__opt: None,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные ассоциированные функции.
/// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a, TWvkBackend> WvkDispatchTableBuilder<'a, TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE>
where
TWvkBackend: WvkBackend {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create(vk_instance: svk::VkInstance, wvk_dispatch_table_global: &'a WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>) -> Self {
        Self {
            phantom_data: PhantomData,
            vk_instance__opt: Some(vk_instance),
            wvk_dispatch_table_global__opt: Some(wvk_dispatch_table_global),

        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные методы.
/// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a, TWvkBackend> WvkDispatchTableBuilder<'a, TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>
where
TWvkBackend: WvkBackend {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn build(self) -> Result<WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>, WvkError> {
        WvkDispatchTable::<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>::s_createWithGlobal(self)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные методы.
/// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a, TWvkBackend> WvkDispatchTableBuilder<'a, TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE>
where
TWvkBackend: WvkBackend {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn build(self) -> Result<WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE>, WvkError> {
        WvkDispatchTable::<TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE>::s_createWithInstance(self)
    }
}
