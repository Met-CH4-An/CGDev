// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::borrow::Cow;
use std::sync::Arc;
use crate::wvk::{WvkEnvironment, WvkEnvironment_0_1_0_0 };
use crate::wvk_error::WvkError;
use crate::wvk_library::wvk_library::WvkLibrary;
use crate::wvk_instance::wvk_instance::WvkInstance;

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// 
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkInstanceBuilder<'a, TWvkBackend>
where TWvkBackend : WvkEnvironment {
    /// Ссылка на библиотеку врапера, с глобальными функциями.
    /// Link to the wrapper library with global functions.
    pub(in crate::wvk_instance) wvk_library : &'a WvkLibrary<TWvkBackend>,
    /// Опционально. Название приложения. Метаданные, которые используются только информативно.
    /// Optional. Application name. Metadata used for informational purposes only.
    pub(in crate::wvk_instance) application_name__opt: Option<Cow<'a, std::ffi::CStr>>,
    /// Опционально. Версия приложения. Метаданные, которые используются только информативно.
    /// Optional. Application version. Metadata used for informational purposes only.
    pub(in crate::wvk_instance) application_version : u32,
    /// Опционально. Название движка. Метаданные, которые используются только информативно.
    /// Optional. Engine name. Metadata used for informational purposes only.
    pub(in crate::wvk_instance) engine_name__opt: Option<Cow<'a, std::ffi::CStr>>,
    /// Опционально. Версия движка. Метаданные, которые используются только информативно.
    /// Optional. Engine version. Metadata used for informational purposes only.
    pub(in crate::wvk_instance) engine_version : u32,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные ассоциированные функции.
/// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a, TWvkBackend> WvkInstanceBuilder<'a, TWvkBackend>
where TWvkBackend : WvkEnvironment {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create(wvk_library : &'a WvkLibrary<TWvkBackend>) -> Self {
        Self {
            wvk_library: wvk_library,
            application_name__opt: None,
            application_version : 0,
            engine_name__opt: Some(crate::wvk::WRAPPER_VULKAN_NAME_COW),
            engine_version : 0,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные методы.
/// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a, TWvkBackend> WvkInstanceBuilder<'a, TWvkBackend>
where TWvkBackend : WvkEnvironment {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn build(self) -> Result<Arc<WvkInstance<TWvkBackend>>, WvkError>
    where 
    TWvkBackend : WvkEnvironment_0_1_0_0 {
        WvkInstance::s_create(&self)
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn applicationNameFromCStr<T>(mut self, name : T) -> Self
    where T : Into<Cow<'a, std::ffi::CStr>> {
        let name_cow_cstr_ = name.into();
        self.application_name__opt = Some(name_cow_cstr_);
        self
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn applicationName<T>(mut self, name : T) -> Self
    where T : Into<Cow<'a, str>> {
        let name_cow_str_ = name.into();
        let name_cstring_ = std::ffi::CString::new(name_cow_str_.as_ref()).unwrap();
        let name_cow_cstr_ = name_cstring_.into();
        self.application_name__opt = Some(name_cow_cstr_);
        self
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn applicationVersion(mut self, version : u32) -> Self {
        self.application_version = version;
        self
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn engineNameFromCStr<T>(mut self, name : T) -> Self
    where T : Into<Cow<'a, std::ffi::CStr>> {
        let name_cow_cstr_ = name.into();
        self.engine_name__opt = Some(name_cow_cstr_);
        self
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn engineName<T>(mut self, name : T) -> Self
    where T : Into<Cow<'a, str>> {
        let name_cow_str_ = name.into();
        let name_cstring_ = std::ffi::CString::new(name_cow_str_.as_ref()).unwrap();
        let name_cow_cstr_ = name_cstring_.into();
        self.engine_name__opt = Some(name_cow_cstr_);
        self
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn engineVersion(mut self, version : u32) -> Self {
        self.engine_version = version;
        self
    }
}