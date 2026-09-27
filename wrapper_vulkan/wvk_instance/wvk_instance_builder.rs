// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::borrow::Cow;
use std::sync::Arc;
use crate::wvk_error::WvkError;
use crate::wvk_library::WvkLibrary;
use crate::wvk_instance::wvk_instance::WvkInstance;

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// 
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkInstanceBuilder {
    /// Ссылка на библиотеку врапера, с глобальными функциями.
    /// Link to the wrapper library with global functions.
    pub(in crate::wvk_instance) wvk_library : Arc<WvkLibrary>,
    /// Набор расширений.
    /// A set of extensions.
    pub(in crate::wvk_instance) extension_name_vec: Vec<Cow<'static, str>>,
    /// Опционально. Название приложения. Метаданные, которые используются только информативно.
    /// Optional. Application name. Metadata used for informational purposes only.
    pub(in crate::wvk_instance) application_name: Option<Cow<'static, str>>,
    /// Опционально. Версия приложения. Метаданные, которые используются только информативно.
    /// Optional. Application version. Metadata used for informational purposes only.
    pub(in crate::wvk_instance) application_version : Option<u32>,
    /// Опционально. Название движка. Метаданные, которые используются только информативно.
    /// Optional. Engine name. Metadata used for informational purposes only.
    pub(in crate::wvk_instance) engine_name: Option<Cow<'static, str>>,
    /// Опционально. Версия движка. Метаданные, которые используются только информативно.
    /// Optional. Engine version. Metadata used for informational purposes only.
    pub(in crate::wvk_instance) engine_version : Option<u32>,
}

impl WvkInstanceBuilder {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn create(wvk_library: Arc<WvkLibrary>) -> Self {
        Self {
            wvk_library: wvk_library,
            extension_name_vec: Vec::new(),
            application_name: None,
            application_version : None,
            engine_name: Some(Cow::Borrowed(crate::wvk::WRAPPER_VULKAN_NAME)),
            engine_version : None,
        }
    }
    
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn build(self) -> Result<Arc<WvkInstance>, WvkError> {
        WvkInstance::create(self)
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn addExtension(mut self, name_extension: Cow<'static, str>) -> Self {
        self.extension_name_vec.push(name_extension);
        self
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn applicationName(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.application_name = Some(name.into());
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
    pub fn engineName(mut self, name: impl Into<Cow<'static, str>>) -> Self{
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

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn isExtension(&self, name: &str) -> bool {
        self.extension_name_vec
            .iter()
            .any(|v|{
                v.as_ref() == name
            })
    }
}