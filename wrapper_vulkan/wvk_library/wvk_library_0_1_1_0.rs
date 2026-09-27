// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::wvk_call_with_check;
use crate::wvk_error::{WvkError, WvkErrorType};
use crate::wvk_library::WvkLibrary;

impl WvkLibrary {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn wvkEnumerateInstanceVersion(&self) -> Result<u32, WvkError> {
        let mut version_: u32 = 0;

        wvk_call_with_check!(
            self.wvk_dispatch_table.vkEnumerateInstanceVersion(&mut version_)
        );

        Ok(version_)
    }
}