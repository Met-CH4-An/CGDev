// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~


use std::marker::PhantomData;
use crate::wvk::{WvkBackend};
use crate::wvk_instance::dispatch_table::WvkDispatchTable;

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkInstance<TWvkBackend>
where
TWvkBackend : WvkBackend {
    pub(in crate::wvk_instance) _phantom_data: PhantomData<TWvkBackend>,

    /// Таблица функций вулкана, которые создаются с помощью инстанса.
    /// Table of volcano functions that are created using an instance.
    pub(in crate::wvk_instance) wvk_dispatch_table: WvkDispatchTable<TWvkBackend>,
    /// Созданный VkInstance.
    /// Created by VkInstance.
    pub(in crate::wvk_instance) vk_instance: svk::VkInstance,
}
