// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::panic::panic_any;
use crate::wvk_library::dispatch_table::WvkLibraryDispatchTable;

impl WvkLibraryDispatchTable {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(in crate::wvk_library) fn vkEnumerateInstanceVersion(&self, pApiVersion: *mut u32) -> svk::VkResult {
        unsafe { (self.vk_enumerate_instance_version)(pApiVersion) }
    }
    #[inline(always)]
    pub(in crate::wvk_library::dispatch_table) unsafe extern "system" fn vkEnumerateInstanceVersionDummy(pApiVersion : *mut u32) -> crate::svk::VkResult {
        panic_any("Вызов фиктивной команды вулкана. Invoking a fictitious volcano command.")
    }
}
