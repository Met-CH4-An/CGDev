// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;

use crate::wvk::WvkEnvironment;
use crate::wvk_error::{ WvkError, WvkErrorType };
use crate::dispatch_table::{ WvkDispatchTable, WVK_DISPATCH_TABLE_INSTANCE };
use crate::wvk_instance::wvk_instance_builder::WvkInstanceBuilder;


//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkInstance<TWvkBackend>
where
TWvkBackend : WvkEnvironment {
    pub(crate) phantom_data: PhantomData<TWvkBackend>,

    /// Таблица функций вулкана, которые создаются с помощью инстанса.
    /// Table of volcano functions that are created using an instance.
    pub(in crate::wvk_instance) wvk_dispatch_table_instance : WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE>,
    /// Созданный VkInstance.
    /// Created by VkInstance.
    pub(in crate::wvk_instance) vk_instance : svk::VkInstance,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// приватная область
// private area
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Приватные ассоциированные функции.
/// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend> WvkInstance<TWvkBackend>
where
TWvkBackend : WvkEnvironment {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Приватные методы.
/// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend> WvkInstance<TWvkBackend>
where
TWvkBackend : WvkEnvironment {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Приватные методы.
    /// Private methods.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn getDispatchTable(&self) -> &WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE> {
        &self.wvk_dispatch_table_instance
    }
}

