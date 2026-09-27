// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::wvk::WvkVersion;
use crate::wvk_error::WvkError;
use crate::wvk_library::dispatch_table::WvkLibraryDispatchTable;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(in crate::wvk_library) struct WvkLibraryDispatchTableBuilder {
    pub(in crate::wvk_library::dispatch_table) wvk_version: WvkVersion,
    pub(in crate::wvk_library::dispatch_table) vk_get_instance_proc_addr: svk::PFN_vkGetInstanceProcAddr,
}

impl WvkLibraryDispatchTableBuilder {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_library) fn create(wvk_version: WvkVersion, vk_get_instance_proc_addr: svk::PFN_vkGetInstanceProcAddr) -> Self {
        Self{
            wvk_version: wvk_version,
            vk_get_instance_proc_addr: vk_get_instance_proc_addr,
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_library) fn build(self) -> Result<WvkLibraryDispatchTable, WvkError> {
        WvkLibraryDispatchTable::create(self)
    }
}