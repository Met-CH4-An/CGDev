// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::wvk::{WvkBackend_0_1_1_0};
use crate::dispatch_table::{WvkDispatchTableGlobal};
use crate::dispatch_table::wvk_dispatch_table::WvkDispatchTable;

#[cfg(feature = "vulkan_1_1")]
impl<TLevel> WvkDispatchTable<TLevel>
where
TLevel: WvkDispatchTableGlobal, {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn vkEnumerateInstanceVersion(&self, pApiVersion: *mut u32) -> svk::VkResult {
        unsafe { self.vk_enumerate_instance_version.assume_init()(pApiVersion) }
    }
}

