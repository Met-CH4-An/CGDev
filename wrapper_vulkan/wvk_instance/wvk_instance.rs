// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ffi::CString;
use std::sync::Arc;
use crate::wvk_error::{WvkError, WvkErrorType};
use crate::wvk_instance::WvkInstanceBuilder;
use crate::wvk_instance::dispatch_table::{WvkInstanceDispatchTable};

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkInstance {
    ///
    ///
    builder: WvkInstanceBuilder,
    /// Таблица функций вулкана, которые создаются с помощью инстанса.
    /// Table of volcano functions that are created using an instance.
    pub(in crate::wvk_instance) wvk_dispatch_table: WvkInstanceDispatchTable,
    /// Созданный VkInstance.
    /// Created by VkInstance.
    pub(in crate::wvk_instance) vk_instance: svk::VkInstance,
}

impl WvkInstance {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn isExtension(&self, name: &str) -> bool {
        self.builder.extension_name_vec
            .iter()
            .any(|v|{
                v.as_ref() == name
            })
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_instance) fn create(builder: WvkInstanceBuilder) -> Result<Arc<Self>, WvkError> {
        // Создаем непосредственно VkInstance.
        // Create VkInstance directly.
        let vk_instance_ = Self::createVkInstance(&builder)?;

        // Далее используя VkInstance можно создать таблицу функций.
        // Next, using VkInstance, you can create a table of functions.
        let wvk_dispatch_table_ = WvkInstanceDispatchTable::create(&builder, vk_instance_)?;

        let self_ = Self {
            builder: builder,
            wvk_dispatch_table: wvk_dispatch_table_,
            vk_instance : vk_instance_,
        };

        Ok(Arc::new(self_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Создает экземпляр вулкана.
    /// Creates a volcano instance.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn createVkInstance(builder: &WvkInstanceBuilder) -> Result<svk::VkInstance, WvkError> {
        // CString
        let application_name_ = builder.application_name
            .as_ref()
            .map(|v| {
                CString::new(v.as_ref())
            })
            .transpose()
            .map_err(|a| {
                WvkError::createWithDescription(WvkErrorType::WVK_INSTANCE_CREATE_FAILED, "WvkInstanceBuilder::application_name:\nНе удалось конвертировать в си строку.\nFailed to convert to C string.")
            })?;

        // *const c_char
        let application_name_ = application_name_
            .as_ref()
            .map_or_else(std::ptr::null,
                         |value| {
                             value.as_ptr()
                         }
            );

        // CString
        let engine_name_ = builder.engine_name
            .as_ref()
            .map(|v|
                CString::new(v.as_ref())
            )
            .transpose()
            .map_err(|v| {
                WvkError::createWithDescription(WvkErrorType::WVK_INSTANCE_CREATE_FAILED, "WvkInstanceBuilder::engine_name:\nНе удалось конвертировать в си строку.\nFailed to convert to C string.")
            })?;

        // *const c_char
        let engine_name_ = engine_name_
            .as_ref()
            .map_or_else(std::ptr::null,
                         |value| {
                             value.as_ptr()
                         }
            );

        let engine_version_ = builder.engine_version.unwrap_or(0);
        let application_version_ = builder.application_version.unwrap_or(0);

        // Список имен расширений.
        // List of extension names.
        let extension_name_vec_ = Self::makeExtensionNames(&builder);

        // Получаем pNext с загруженным VkDebugUtilsMessengerCreateInfoEXT
        // Get pNext with loaded VkDebugUtilsMessengerCreateInfoEXT
        let vk_debug_utils_create_info_ = Self::createDebugUtilsMessengerCreateInfoForVkInstance(&builder)?;

        let p_next_ = vk_debug_utils_create_info_
            .as_ref()
            .map(|vk_debug_utils_create_info_ref| vk_debug_utils_create_info_ref as *const _ as *const _)
            .unwrap_or(std::ptr::null());

        // В вулкане можно описать своё приложение через VkApplicationInfo
        // In Vulkan, you can describe your application using VkApplicationInfo
        let vk_application_info_ = svk::VkApplicationInfo {
            sType : svk::VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext : std::ptr::null_mut(),
            pApplicationName : application_name_,
            applicationVersion : application_version_,
            pEngineName : engine_name_,
            engineVersion : engine_version_,
            apiVersion : crate::wvk::WVK_ENCODED_VULKAN_VERSION,
        };

        // Для создания VkInstance описываем его через VkInstanceCreateInfo
        // To create a VkInstance, we describe it using VkInstanceCreateInfo
        let vk_create_info_ = svk::VkInstanceCreateInfo {
            sType: svk::VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext: p_next_,
            flags: svk::VkInstanceCreateFlags(0),
            pApplicationInfo: &vk_application_info_,
            enabledLayerCount: 0,
            ppEnabledLayerNames: std::ptr::null(),
            enabledExtensionCount: extension_name_vec_.len() as u32,
            ppEnabledExtensionNames: extension_name_vec_.as_ptr()
        };

        let vk_instance_ = builder.wvk_library.wvkCreateInstance(&vk_create_info_, None)
            .map_err(|wvk_error| wvk_error.addError(WvkErrorType::WVK_INSTANCE_CREATE_FAILED, "Не удалось выполнить wvkCreateInstance"))?;

        Ok(vk_instance_)
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Функция для получения заполненной структуры VkDebugUtilsMessengerCreateInfoEXT.
    /// В ОТЛАДКЕ функция ищет расширение VK_EXT_debug_utils.
    /// Если расширение не найдено, кидает Err. Для отладки расширение должно быть!!!
    /// В РЕЛИЗЕ функция просто возвращает None. Для релиза расширение не обязательно.
    ///
    /// Function for getting the filled VkDebugUtilsMessengerCreateInfoEXT structure.
    /// IN DEBUG, the function looks for the VK_EXT_debug_utils extension.
    /// If the extension is not found, throws Err. For debugging, the extension must be!!!
    /// IN RELEASE the function simply returns None. The extension is not required for release.
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn createDebugUtilsMessengerCreateInfoForVkInstance(builder: &WvkInstanceBuilder) -> Result<Option<svk::VkDebugUtilsMessengerCreateInfoEXT>, WvkError> {
        #[cfg(debug_assertions)]
        {
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Ищем расширение VK_EXT_debug_utils.
            // Looking for the VK_EXT_debug_utils extension.
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            let extensions_ = builder.wvk_library.wvkEnumerateInstanceExtensionProperties(None)?;

            let vk_debug_utils_messenger_create_info_ = extensions_
                .iter()
                .find_map(|v| {
                    let extension_name_ = unsafe {
                        std::ffi::CStr::from_ptr(v.extensionName.as_ptr())
                    };

                    if extension_name_ == crate::extensions::VkExtDebugUtils::NAME_C {
                        // описываем структуру VkDebugUtilsMessengerCreateInfoEXT
                        // describe the VkDebugUtilsMessengerCreateInfoEXT structure
                        let vk_debug_utils_messenger_create_info_ = svk::VkDebugUtilsMessengerCreateInfoEXT {
                            sType: svk::VkStructureType::VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
                            pNext: std::ptr::null_mut(),
                            flags: svk::VkDebugUtilsMessengerCreateFlagsEXT(0),
                            messageSeverity: svk::VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT,
                            messageType: svk::VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT,
                            pfnUserCallback: Self::wvkDebugUtilsMessengerCallbackEXT,
                            pUserData: std::ptr::null_mut(),
                        };

                        Some(vk_debug_utils_messenger_create_info_)
                    }

                    else {
                        None
                    }
                });


            vk_debug_utils_messenger_create_info_
                .map(|v| Some(v))
                .ok_or_else(|| {
                    WvkError::createWithDescription(WvkErrorType::WVK_INSTANCE_EXTENSION_NOT_FOUND, "Extension not found: VK_EXT_debug_utils.")
                })
        }

        #[cfg(not(debug_assertions))]
        {
            Ok(None)
        }
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    unsafe extern "system" fn wvkDebugUtilsMessengerCallbackEXT(
        messageSeverity : svk::VkDebugUtilsMessageSeverityFlagsEXT,
        messageTypes : svk::VkDebugUtilsMessageTypeFlagsEXT,
        pCallbackData : *const svk::VkDebugUtilsMessengerCallbackDataEXT,
        _pUserData : *mut std::ffi::c_void)
        -> bool {

        let mut message_print_ = String::new();

        if (messageSeverity & svk::VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT) == svk::VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT {
            message_print_.push_str("[INFO] ");
        }
        else if (messageSeverity & svk::VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT) == svk::VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT {
            message_print_.push_str("[VERBOSE] ");
        }
        else if (messageSeverity & svk::VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT) == svk::VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT {
            message_print_.push_str("[WARNING] ");
        }
        else if (messageSeverity & svk::VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT) == svk::VkDebugUtilsMessageSeverityFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT {
            message_print_.push_str("[ERROR] ");
        }

        if (messageTypes & svk::VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT) == svk::VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT {
            message_print_.push_str("[GENERAL] ");
        }
        if (messageTypes & svk::VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT) == svk::VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT {
            message_print_.push_str("[PERFORMANCE] ");
        }
        if (messageTypes & svk::VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT) == svk::VkDebugUtilsMessageTypeFlagBitsEXT::VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT {
            message_print_.push_str("[VALIDATION] ");
        }

        let a= std::ffi::CStr::from_ptr((*pCallbackData).pMessage).to_str().unwrap();

        message_print_.push_str(a);

        println!("{}", message_print_);

        false
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Получаем список имен расширений, которые нужно создать в экземпляре.
    /// Get a list of extension names to create in the instance.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn makeExtensionNames(builder: &WvkInstanceBuilder) -> Vec<*const i8> {
        let name_vec_ = builder.extension_name_vec
            .iter()
            .map(|v|{
                v.as_ref().as_ptr() as *const i8
            })
            .collect::<Vec::<*const i8>>();

        name_vec_
    }
}
