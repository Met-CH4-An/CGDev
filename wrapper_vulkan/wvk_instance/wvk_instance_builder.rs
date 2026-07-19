// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::borrow::Cow;
use std::marker::PhantomData;

use crate::wvk_error::{ WvkError } ;
use crate::wvk_library::wvk_library::WvkLibrary;
use crate::wvk::{WvkEnvironment, WvkEnvironment_0_1_0_0};
use crate::wvk_instance::wvk_instance::WvkInstance;

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// 
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkInstanceBuilder<'a, TWvkEnvironment>
where TWvkEnvironment : WvkEnvironment {
    phantom_data: PhantomData<TWvkEnvironment>,
    /// Ссылка на библиотеку врапера, с глобальными функциями.
    /// Link to the wrapper library with global functions.
    pub(in crate::wvk_instance) wvk_library : &'a WvkLibrary<TWvkEnvironment>,
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
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a, TWvkEnvironment> WvkInstanceBuilder<'a, TWvkEnvironment>
where TWvkEnvironment : WvkEnvironment {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create(wvk_library : &'a WvkLibrary<TWvkEnvironment>) -> Self {
        Self {
            phantom_data : PhantomData,
            wvk_library: wvk_library,
            application_name__opt: None,
            application_version : 0,
            engine_name__opt: Some(crate::wvk::WRAPPER_VULKAN_NAME_COW),
            engine_version : 0,
        }
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn build(self) -> Result<WvkInstance<TWvkEnvironment>, WvkError>
    where TWvkEnvironment : WvkEnvironment_0_1_0_0 {
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