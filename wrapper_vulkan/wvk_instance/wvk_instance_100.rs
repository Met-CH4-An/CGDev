// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use {
    std::marker::PhantomData,
    crate::wvk_instance::{
        WvkInstance,
        WvkInstanceBuilder
    },
    crate::wvk::WvkFeature_0_1_0_0,
    crate::wvk_error::{
        WvkError,
        WvkErrorType
    },
};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// публичные функции
/// public functions
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkVersion> WvkInstance<TWvkVersion>
where TWvkVersion : WvkFeature_0_1_0_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create(wvk_instance_builder : & WvkInstanceBuilder<TWvkVersion>) -> Result<Self, WvkError> {
        let vk_instance_ = Self::s_createVkInstance(&wvk_instance_builder)?;

        return Ok(
            Self {
                phantom_data : PhantomData,
                vk_instance : vk_instance_,
            }
        );
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// защищённые функции
/// protected functions
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkVersion> WvkInstance<TWvkVersion>
where TWvkVersion : WvkFeature_0_1_0_0 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// приватные функции
/// private functions
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkVersion> WvkInstance<TWvkVersion>
where TWvkVersion : WvkFeature_0_1_0_0 {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn s_createVkInstance(builder : &WvkInstanceBuilder<TWvkVersion>) -> Result<svk::VkInstance, WvkError> {
        let mut vk_instance_ : svk::VkInstance = std::ptr::null_mut();

        let application_name_cchar_ptr_ = builder.application_name__opt
            .as_ref()
            .map(|r| r
                .as_ref()
                .as_ptr())
            .unwrap_or(std::ptr::null());

        let engine_name_cchar_ptr_ = builder.engine_name__opt
            .as_ref()
            .map(|r| r
                .as_ref()
                .as_ptr())
            .unwrap_or(std::ptr::null());

        // в вулкане можно описать своё приложение через VkApplicationInfo
        // In Vulkan, you can describe your application using VkApplicationInfo
        let vk_application_info_ = svk::VkApplicationInfo {
            sType : svk::VkStructureTypeValue::VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext : std::ptr::null_mut(),
            pApplicationName : application_name_cchar_ptr_,
            applicationVersion : builder.application_version,
            pEngineName : engine_name_cchar_ptr_,
            engineVersion : builder.engine_version,
            apiVersion : crate::GET_VULKAN_VERSION(),
        };

        // для создания VkInstance описываем его через VkInstanceCreateInfo
        // to create a VkInstance, we describe it using VkInstanceCreateInfo
        let vk_create_info_ = svk::VkInstanceCreateInfo {
            sType : svk::VkStructureTypeValue::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext : std::ptr::null(),
            flags : 0,
            pApplicationInfo : &vk_application_info_,
            enabledLayerCount : 0,
            ppEnabledLayerNames : std::ptr::null(),
            enabledExtensionCount : 0,
            ppEnabledExtensionNames : std::ptr::null(),
        };

        vk_instance_ = builder.wvk_library.wvkCreateInstance(&vk_create_info_, None)
            .map_err(|wvk_error| wvk_error.addError(WvkErrorType::WVK_INSTANCE_CREATE_FAILED, "Не удалось выполнить wvkCreateInstance"))?;

        return Ok(vk_instance_);
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn s_createDebugUtilsMessengerCreateInfo() {
        //svk::VkDebugUtilsMessengerCreateInfoEXT {
        //    sType : svk::VkStructureTypeValue::Vk_structure_typeme
        //};
    }
}