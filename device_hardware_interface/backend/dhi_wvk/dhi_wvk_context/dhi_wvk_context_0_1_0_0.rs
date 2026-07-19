// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use std::marker::PhantomData;
use wvk::wvk_library::WvkLibraryBuilder;
use wvk::wvk_instance::WvkInstanceBuilder;
use crate::backend::dhi_wvk::dhi_wvk::DhiWvkBackend_0_1_0_0;
use crate::backend::dhi_wvk::dhi_wvk_context::dhi_wvk_context::DhiWvkContext;
use crate::dhi_error::{DHIError, DHIErrorType};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TDhiWvkBackend> DhiWvkContext<TDhiWvkBackend>
where
    TDhiWvkBackend : DhiWvkBackend_0_1_0_0,
    TDhiWvkBackend::WvkVersion : wvk::wvk::WvkEnvironment_0_1_0_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// создание объекта
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create() -> Result<Self, DHIError> {
        // Создаём главную библиотеку врапера WVK.
        // Create the main WVK wrapper library.
        let wvk_library_ = WvkLibraryBuilder::<TDhiWvkBackend::WvkVersion>::s_create()
            .build()
            .map_err(|wvk_error| {
                DHIError::createWithDescription(DHIErrorType::DHI_CONTEXT_BACKEND_CREATE_FAILED, &format!("Failed to create WvkLibrary: {}", wvk_error.getMessage()))
            }
        )?;

        // Создаём WvkInstance.
        // Create WvkInstance.
        let wvk_instance_ = WvkInstanceBuilder::<TDhiWvkBackend::WvkVersion>::s_create(&wvk_library_)
            .applicationNameFromCStr(crate::dhi::DHI_NAME_CSTR)
            .engineNameFromCStr(crate::dhi::DHI_NAME_CSTR)
            .engineVersion(crate::dhi::DHI_VERSION_ENCODED)
            .build()
            .map_err(|wvk_error|
                DHIError::createWithDescription(DHIErrorType::DHI_CONTEXT_BACKEND_CREATE_FAILED, &format!("Failed to create WvkLibrary: {}", wvk_error.getMessage()))
        )?;

        Ok(Self{
            phantom_data : PhantomData,
            wvk_library : wvk_library_,
            wvk_instance : wvk_instance_,
        })
    }
}