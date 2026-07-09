// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use {
    crate::wvk_call_with_check,
    crate::WvkError,
    crate::WvkErrorType,
    crate::WvkLibrary,
    crate::WvkInstanceBuilder,
    std::rc::Rc,
};
use crate::wvk::GET_VULKAN_VERSION;

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkInstance {
    // инстанс вулкана
    // volcano instance
    vk_instance : svk::VkInstance,

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // команды вулкана
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // vulkan 1.0
    
    // vulkan 1.1
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// публичные функции
/// public functions
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl WvkInstance {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create(wvk_instance_builder : & WvkInstanceBuilder) -> Result<Self, WvkError> {
        let vk_instance_ = Self::s_createVkInstance(&wvk_instance_builder)?;

        return Ok(
            Self {
                vk_instance : vk_instance_,
            }
        );
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// защищённые функции
/// protected functions
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl WvkInstance {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// приватные функции
/// private functions
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl WvkInstance {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn s_createVkInstance(builder : &WvkInstanceBuilder) -> Result<svk::VkInstance, WvkError> {
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
}