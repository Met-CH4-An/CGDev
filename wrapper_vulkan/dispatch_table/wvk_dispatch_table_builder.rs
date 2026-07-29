// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;

use crate::wvk::{ WvkEnvironment, WvkEnvironment_0_1_0_0 };
use crate::wvk_error::WvkError;
use crate::dispatch_table::{ WVK_DISPATCH_TABLE_GLOBAL, WVK_DISPATCH_TABLE_INSTANCE };
use crate::dispatch_table::wvk_dispatch_table::WvkDispatchTable;

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkDispatchTableBuilder<TWvkBackend, TLevel> {
    phantom_data: PhantomData<(TWvkBackend, TLevel)>,
    /// Экземпляр вулкана.
    /// Vulkan instance.
    pub(in crate::dispatch_table) vk_instance__opt: Option<svk::svk_types::VkInstance>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные ассоциированные функции.
/// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend> WvkDispatchTableBuilder<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>
where
TWvkBackend : WvkEnvironment {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create() -> Self {
        Self {
            phantom_data: PhantomData,
            vk_instance__opt: None,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные ассоциированные функции.
/// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend> WvkDispatchTableBuilder<TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE>
where
TWvkBackend : WvkEnvironment  {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create(vk_instance: svk::svk_types::VkInstance) -> Self {
        Self {
            phantom_data: PhantomData,
            vk_instance__opt: Some(vk_instance),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные методы.
/// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend> WvkDispatchTableBuilder<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>
where
    TWvkBackend : WvkEnvironment_0_1_0_0 {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn build(self) -> Result<WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>, WvkError> {
        WvkDispatchTable::<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>::s_create(&self)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные методы.
/// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend> WvkDispatchTableBuilder<TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE>
where
    TWvkBackend : WvkEnvironment_0_1_0_0 {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn build(self) -> Result<WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE>, WvkError> {
        WvkDispatchTable::<TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE>::s_create(&self)
    }
}
