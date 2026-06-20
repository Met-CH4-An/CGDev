// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// зависимости
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use {
    crate::dhi_error::DHIError,
    crate::dhi_error::DHIErrorType,
};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// выбор бэкенда
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[cfg(feature = "wvk")]
pub(crate) type DHIBackendContext = crate::backend::dhi_wvk::dhi_wvk_context::DhiWvkContext;
#[cfg(not(any(feature = "wvk")))]
compile_error!("Бэкенд для Device Hardware Interface не выбран.");

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct DHIContextCreateInfo {
    
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct DHIContext {
    /// бэкенд
    dhi_context_backend : DHIBackendContext,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl DHIContext {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn new() -> Result<Self, DHIError> {
        let dhi_context_backend_ = crate::backend::dhi_wvk::dhi_wvk_context::DhiWvkContext::new()
            .map_err(|v| v.addMessage("Не удалось создать DHIBackendContext."))?;

        return Ok(Self{
            dhi_context_backend : dhi_context_backend_,
        })
    }
}