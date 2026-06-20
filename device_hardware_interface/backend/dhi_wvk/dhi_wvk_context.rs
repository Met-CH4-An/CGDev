// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// зависимости
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use {
    crate::dhi_error::DHIError,
    crate::dhi_error::DHIErrorType,
};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// бэкенд для Device Hardware Interface на основе Wrapper Vulkan (WVK)
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct DhiWvkContext {
    wvk_library : wvk::WvkLibrary,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// бэкенд для Device Hardware Interface на основе Wrapper Vulkan (WVK)
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl DhiWvkContext {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// создание объекта
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn new() -> Result<Self, DHIError> {
        let wvk_library_ = crate::wvk::WvkLibrary::new().map_err(|wvk_error| {
            DHIError::newWithMessage(DHIErrorType::DHI_BACKEND_CONTEXT_CREATE_FAILED, &format!("Не удалось создать WvkLibrary: {}", wvk_error.getMessage()))
        })?;

        return Ok(Self {
            wvk_library : wvk_library_,
        });
    }
}