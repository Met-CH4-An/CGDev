// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::wvk_call_with_check;
use crate::wvk::WvkBackend_0_1_1_0;
use crate::wvk_error::{WvkError, WvkErrorType};
use crate::wvk_library::WvkLibrary;

impl<TWvkBackend> WvkLibrary<TWvkBackend>
where
TWvkBackend : WvkBackend_0_1_1_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn wvkEnumerateInstanceVersion(&self) -> Result<u32, WvkError> {
        let mut version_: u32 = 0;
        
        wvk_call_with_check!(
            unsafe {
                self.wvk_dispatch_table.vk_enumerate_instance_version.assume_init()(&mut version_)
            }
        );

        Ok(version_)
    }
}