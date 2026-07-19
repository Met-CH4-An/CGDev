// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub type DHIError = crate::common::Error<DHIErrorType>;

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[derive(Debug, Clone)]
pub enum DHIErrorType {
    /// ошибка создания контектста
    DHI_CONTEXT_CREATE_FAILED,
    /// ошибка создания бэкенд контекста
    DHI_CONTEXT_BACKEND_CREATE_FAILED,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl common::ErrorTypeToStr for DHIErrorType {
    fn toStr(&self) -> &str {
        return match self {
            DHIErrorType::DHI_CONTEXT_CREATE_FAILED => "",
            DHIErrorType::DHI_CONTEXT_BACKEND_CREATE_FAILED => "",

            (_) => {""}
        };
    }
}
