// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::borrow::Cow;
use crate::wvk::{WvkBackend, WvkBackend_0_1_0_0};
use crate::wvk_error::WvkError;
use crate::wvk_library::WvkLibrary;
use crate::wvk_instance::wvk_instance::WvkInstance;

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// 
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkInstanceBuilder<'a, TWvkBackend>
where TWvkBackend : WvkBackend {
    /// Ссылка на библиотеку врапера, с глобальными функциями.
    /// Link to the wrapper library with global functions.
    pub(in crate::wvk_instance) wvk_library : &'a WvkLibrary<TWvkBackend>,
    /// Опционально. Название приложения. Метаданные, которые используются только информативно.
    /// Optional. Application name. Metadata used for informational purposes only.
    pub(in crate::wvk_instance) application_name: Option<Cow<'a, str>>,
    /// Опционально. Версия приложения. Метаданные, которые используются только информативно.
    /// Optional. Application version. Metadata used for informational purposes only.
    pub(in crate::wvk_instance) application_version : Option<u32>,
    /// Опционально. Название движка. Метаданные, которые используются только информативно.
    /// Optional. Engine name. Metadata used for informational purposes only.
    pub(in crate::wvk_instance) engine_name: Option<Cow<'a, str>>,
    /// Опционально. Версия движка. Метаданные, которые используются только информативно.
    /// Optional. Engine version. Metadata used for informational purposes only.
    pub(in crate::wvk_instance) engine_version : Option<u32>,
}

impl<'a, TWvkBackend> WvkInstanceBuilder<'a, TWvkBackend>
where TWvkBackend : WvkBackend {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn create(wvk_library: &'a WvkLibrary<TWvkBackend>) -> Self {
        Self {
            wvk_library: wvk_library,
            application_name: None,
            application_version : None,
            engine_name: Some(Cow::Borrowed(crate::wvk::WRAPPER_VULKAN_NAME)),
            engine_version : None,
        }
    }
    
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn build(self) -> Result<WvkInstance<TWvkBackend>, WvkError>
    where 
    TWvkBackend : WvkBackend_0_1_0_0 {
        WvkInstance::create(&self)
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn applicationName(mut self, name: &'a str) -> Self {
        self.application_name = Some(Cow::from(name));
        self
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn applicationVersion(mut self, version: u32) -> Self {
        self.application_version = Some(version);
        self
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn engineName<T>(mut self, name: impl Into<Cow<'a, str>>) -> Self{
        self.engine_name = Some(name.into());
        self
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn engineVersion(mut self, version: u32) -> Self {
        self.engine_version = Some(version);
        self
    }
}