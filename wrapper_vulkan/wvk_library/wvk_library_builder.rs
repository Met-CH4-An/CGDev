// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use std::sync::Arc;
use crate::wvk::{WvkBackend, WvkBackend_0_1_0_0};
use crate::wvk_error::WvkError;
use crate::wvk_library::wvk_library::WvkLibrary;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkLibraryBuilder<TWvkBackend>
where TWvkBackend : WvkBackend {
    phantom_data: PhantomData<TWvkBackend>,
}

impl<TWvkBackend> WvkLibraryBuilder<TWvkBackend>
where
TWvkBackend : WvkBackend {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create() -> Self {
        Self{
            phantom_data: PhantomData,
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn build(self) -> Result<Arc<WvkLibrary<TWvkBackend>>, WvkError>
    where
    TWvkBackend: WvkBackend_0_1_0_0 {
        WvkLibrary::<TWvkBackend>::create(self)
    }
}