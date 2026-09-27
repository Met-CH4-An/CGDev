// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::sync::Arc;
use crate::wvk::WvkVersion;
use crate::wvk_error::WvkError;
use crate::wvk_library::WvkLibraryBuilder;
use crate::wvk_library::platform::WvkLibraryPlatform;
use crate::wvk_library::dispatch_table::{WvkLibraryDispatchTable, WvkLibraryDispatchTableBuilder};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkLibrary {
    ///
    /// 
    pub(in crate::wvk_library) builder: WvkLibraryBuilder,
    /// Платформозависимая часть.
    /// Platform-specific section.
    pub(in crate::wvk_library) _wvk_library_platform: WvkLibraryPlatform,
    /// Таблица функций вулкана, которые создаются без инстанса и без логического устройства. Глобальные функции.
    /// Table of Vulcan functions that are created without an instance and without a logical device. Global functions.
    pub(in crate::wvk_library) wvk_dispatch_table : WvkLibraryDispatchTable,
}

impl WvkLibrary {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn version(&self) -> WvkVersion {
        self.builder.wvk_version.clone()
    }
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_library) fn create(builder: WvkLibraryBuilder) -> Result<Arc<Self>, WvkError> {
        let wvk_library_platform_ = WvkLibraryPlatform::create()?;
        let vk_get_instance_proc_addr_ = wvk_library_platform_.loadVkGetInstanceProcAddr()?;
        let wvk_dispatch_table = WvkLibraryDispatchTableBuilder::create(builder.wvk_version.clone(), vk_get_instance_proc_addr_).build()?;

        let self_ = Self {
            builder: builder,
            _wvk_library_platform: wvk_library_platform_,
            wvk_dispatch_table: wvk_dispatch_table,
        };

        Ok(Arc::new(self_))
    }
}