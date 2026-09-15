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
    _phantom_data: PhantomData<(TWvkBackend, TLevel)>,
    /// Команда вулкана, с помощью которой можно получить адреса для всех остальных команд.
    /// A Vulkan command that can be used to retrieve the addresses for all other commands.
    pub(in crate::dispatch_table) vk_get_instance_proc_addr: Option<svk::PFN_vkGetInstanceProcAddr>,
    /// Экземпляр вулкана.
    /// Vulkan instance.
    pub(in crate::dispatch_table) vk_instance: Option<svk::VkInstance>,
    /// Таблица команд, которые были получены бех экземпляра вулкана. Глобальные команды.
    /// Table of commands received by the Volcano instance. Global commands.
    pub(in crate::dispatch_table) wvk_dispatch_table_global: Option<&'a WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>>,
}

impl<'a, TWvkBackend> WvkDispatchTableBuilder<'a, TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>
where
TWvkBackend: WvkBackend {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn create(vk_get_instance_proc_addr: svk::PFN_vkGetInstanceProcAddr) -> Self {
        Self {
            _phantom_data: PhantomData,
            vk_get_instance_proc_addr: Some(vk_get_instance_proc_addr),
            vk_instance: None,
            wvk_dispatch_table_global: None,
        }
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn build(self) -> Result<WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>, WvkError> {
        WvkDispatchTable::<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>::createAsGlobal(self)
    }
}

impl<'a, TWvkBackend> WvkDispatchTableBuilder<'a, TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE>
where
TWvkBackend: WvkBackend {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn create(vk_instance: svk::VkInstance, wvk_dispatch_table_global: &'a WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>) -> Self {
        Self {
            _phantom_data: PhantomData,
            vk_get_instance_proc_addr: None,
            vk_instance: Some(vk_instance),
            wvk_dispatch_table_global: Some(wvk_dispatch_table_global),

        }
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn build(self) -> Result<WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE>, WvkError> {
        WvkDispatchTable::<TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE>::createAsInstance(self)
    }
}