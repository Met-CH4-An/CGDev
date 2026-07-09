// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use std::borrow::Cow;
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use {
    crate::WvkError,
    crate::WvkErrorType,
    crate::WvkLibrary,
    crate::WvkInstance,
};

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// 
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkInstanceBuilder<'a> {
    pub(crate) wvk_library : &'a WvkLibrary,
    pub(crate) application_name__opt: Option<Cow<'a, std::ffi::CStr>>,
    pub(crate) application_version : u32,
    pub(crate) engine_name__opt: Option<Cow<'a, std::ffi::CStr>>,
    pub(crate) engine_version : u32,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// публичные функции
/// public functions
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a> WvkInstanceBuilder<'a> {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create(wvk_library : &'a WvkLibrary) -> Self {
        return Self {
            wvk_library: wvk_library,
            application_name__opt: None,
            application_version : 0,
            engine_name__opt: Some(crate::WRAPPER_VULKAN_NAME_COW),
            engine_version : 0,
        };
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// защищённые функции
/// protected functions
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a> WvkInstanceBuilder<'a> {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// приватные функции
/// private functions
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a> WvkInstanceBuilder<'a> {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// публичные методы
/// public methods
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<'a> WvkInstanceBuilder<'a> {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn build(self) -> Result<WvkInstance, WvkError> {
        let wvk_instance_ = WvkInstance::s_create(&self)?;

        return Ok(wvk_instance_);
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn applicationNameFromCStr<T>(mut self, name : T) -> Self
    where T : Into<Cow<'a, std::ffi::CStr>> {
        let name_cow_cstr_ = name.into();
        self.application_name__opt = Some(name_cow_cstr_);
        return self;
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
        return self;
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn applicationVersion(mut self, version : u32) -> Self {
        self.application_version = version;
        return self;
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn engineNameFromCStr<T>(mut self, name : T) -> Self
    where T : Into<Cow<'a, std::ffi::CStr>> {
        let name_cow_cstr_ = name.into();
        self.engine_name__opt = Some(name_cow_cstr_);
        return self;
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
        return self;
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn engineVersion(mut self, version : u32) -> Self {
        self.engine_version = version;
        return self;
    }
}