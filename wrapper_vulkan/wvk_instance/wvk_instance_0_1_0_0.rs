// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use std::sync::Arc;
use crate::wvk_call_with_check;
use crate::wvk:: { WvkEnvironment_0_1_0_0 };
use crate::wvk_error::{ WvkError, WvkErrorType };
use crate::dispatch_table::{ WvkDispatchTableBuilder, WVK_DISPATCH_TABLE_INSTANCE };
use crate::wvk_instance::wvk_instance_builder::WvkInstanceBuilder;
use crate::wvk_instance::wvk_instance::WvkInstance;
use crate::wvk_physical_device::wvk_physical_device_builder::WvkPhysicalDeviceBuilder;
use crate::wvk_physical_device::wvk_physical_device::WvkPhysicalDevice;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные ассоциированные функции.
/// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend> WvkInstance<TWvkBackend>
where TWvkBackend : WvkEnvironment_0_1_0_0 {
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные методы.
/// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend> WvkInstance<TWvkBackend>
where TWvkBackend : WvkEnvironment_0_1_0_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn wvkEnumeratePhysicalDevices(self: &Arc<Self>) -> Result<Vec<WvkPhysicalDevice<TWvkBackend>>, WvkError> {
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Получаем количество физических устройств VkPhysicalDevice.
        // Get the number of physical devices VkPhysicalDevice.
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let mut count_ : u32 = 0;
        wvk_call_with_check!(
            self.wvk_dispatch_table_instance.vkEnumeratePhysicalDevices(self.vk_instance, &mut count_, std::ptr::null_mut())
        );

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Выделить место для данных, исходя из полученного количества.
        // Allocate space for data based on the received quantity.
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let mut vk_physical_devices_ = Vec::<svk::svk_types::VkPhysicalDevice>::with_capacity(count_ as usize);
        unsafe { vk_physical_devices_.set_len(count_ as usize) }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Получаем список физических устройств VkPhysicalDevice.
        // We get a list of physical devices VkPhysicalDevice.
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        wvk_call_with_check!(
            self.wvk_dispatch_table_instance.vkEnumeratePhysicalDevices(self.vk_instance, &mut count_, vk_physical_devices_.as_mut_ptr())
        );

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Перебираем полученный список физических устройств VkPhysicalDevice и формируем обертки WvkPhysicalDevice.
        // We iterate over the received list of physical devices VkPhysicalDevice and form WvkPhysicalDevice wrappers.
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let mut wvk_physical_devices_ = Vec::<WvkPhysicalDevice<TWvkBackend>>::with_capacity(count_ as usize);
        for vk_physical_device_ in &vk_physical_devices_ {
            let wvk_physical_device_ = WvkPhysicalDeviceBuilder::<TWvkBackend>::s_create(*vk_physical_device_, self.clone()).build()?;

            wvk_physical_devices_.push(wvk_physical_device_);
        }

        Ok(wvk_physical_devices_)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// приватная область
// private area
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Приватные ассоциированные функции.
/// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend> WvkInstance<TWvkBackend>
where TWvkBackend : WvkEnvironment_0_1_0_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_instance) fn s_create(wvk_instance_builder : & WvkInstanceBuilder<TWvkBackend>) -> Result<Arc<WvkInstance<TWvkBackend>>, WvkError> {
        // Создаем непосредственно VkInstance.
        // Create VkInstance directly.
        let vk_instance_ = Self::s_createVkInstance(&wvk_instance_builder)?;

        let wvk_dispatch_table_instance_ = WvkDispatchTableBuilder::<TWvkBackend, WVK_DISPATCH_TABLE_INSTANCE>::s_create(vk_instance_).build()?;

        let self_ = Self {
            phantom_data : PhantomData,
            wvk_dispatch_table_instance : wvk_dispatch_table_instance_,
            vk_instance : vk_instance_,
        };

        let self_arc = Arc::new(self_);

        Ok(self_arc)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn s_createVkInstance(builder : &WvkInstanceBuilder<TWvkBackend>) -> Result<svk::VkInstance, WvkError> {
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

        // получаем pNext с загруженным VkDebugUtilsMessengerCreateInfoEXT
        // get pNext with loaded VkDebugUtilsMessengerCreateInfoEXT
        let vk_debug_utils_create_info_opt_ = Self::s_createDebugUtilsMessengerCreateInfoForVkInstance(&builder)?;

        let p_next_ = vk_debug_utils_create_info_opt_
            .as_ref()
            .map(|vk_debug_utils_create_info_ref| vk_debug_utils_create_info_ref as *const _ as *const _)
            .unwrap_or(std::ptr::null());

        /*let p_next_ = Self::s_createDebugUtilsMessengerCreateInfoForVkInstance(&builder)
            .map(|vk_debug_utils_create_info_opt_| vk_debug_utils_create_info_opt_
                .as_ref()
                .map(|debug_utils_create_info_| debug_utils_create_info_ as *const svk::VkDebugUtilsMessengerCreateInfoEXT as *const std::ffi::c_void)
                .unwrap_or(std::ptr::null())
            )?;*/

        // в вулкане можно описать своё приложение через VkApplicationInfo
        // In Vulkan, you can describe your application using VkApplicationInfo
        let vk_application_info_ = svk::VkApplicationInfo {
            sType : svk::VkStructureTypeValue::VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext : std::ptr::null(),
            pApplicationName : application_name_cchar_ptr_,
            applicationVersion : builder.application_version,
            pEngineName : engine_name_cchar_ptr_,
            engineVersion : builder.engine_version,
            apiVersion : TWvkBackend::WVK_ENCODED_VULKAN_VERSION,
        };

        // для создания VkInstance описываем его через VkInstanceCreateInfo
        // to create a VkInstance, we describe it using VkInstanceCreateInfo
        let vk_create_info_ = svk::VkInstanceCreateInfo {
            sType : svk::VkStructureTypeValue::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            //sType : svk::VkStructureTypeValue::VK_STRUCTURE_TYPE_APPLICATION_INFO,
            pNext : p_next_,
            flags : 0,
            pApplicationInfo : &vk_application_info_,
            enabledLayerCount : 0,
            ppEnabledLayerNames : std::ptr::null(),
            enabledExtensionCount : 0,
            ppEnabledExtensionNames : std::ptr::null(),
        };

        let vk_instance_ = builder.wvk_library.wvkCreateInstance(&vk_create_info_, None)
            .map_err(|wvk_error| wvk_error.addError(WvkErrorType::WVK_INSTANCE_CREATE_FAILED, "Не удалось выполнить wvkCreateInstance"))?;

        Ok(vk_instance_)
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// ТОЛЬКО ДЛЯ ОТЛАДКИ. Для релиза другая функция.
    /// Функция для получения заполненной структуры VkDebugUtilsMessengerCreateInfoEXT.
    /// Функция проверяет наличие расширения VK_EXT_debug_utils.
    /// В случае ошибки, возвращается WvkError с описанием причины.
    /// В случаен успеха возвращается структура VkDebugUtilsMessengerCreateInfoEXT.
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[cfg(debug_assertions)]
    fn s_createDebugUtilsMessengerCreateInfoForVkInstance(builder : &WvkInstanceBuilder<TWvkBackend>) -> Result<Option<svk::VkDebugUtilsMessengerCreateInfoEXT>, WvkError> {
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // ищем расширение VK_EXT_debug_utils
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        let extensions_ = builder.wvk_library.wvkEnumerateInstanceExtensionProperties(None)?;

        for extension in extensions_ {
            let extension_name_cstr = unsafe { std::ffi::CStr::from_ptr(extension.extensionName.as_ptr()) };

            // расширение найдено
            // extension found
            if extension_name_cstr == crate::extensions::VK_EXT_debug_utils::NAME_cstr {
                // описываем структуру VkDebugUtilsMessengerCreateInfoEXT
                // describe the VkDebugUtilsMessengerCreateInfoEXT structure
                let vk_create_info_ = svk::VkDebugUtilsMessengerCreateInfoEXT {
                    sType: svk::VkStructureTypeValue::VK_STRUCTURE_TYPE_DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
                    pNext: std::ptr::null(),
                    flags: 0,
                    messageSeverity: svk::svk_enums::VkDebugUtilsMessageSeverityFlagBitsEXTValue::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT,
                    messageType: svk::svk_enums::VkDebugUtilsMessageTypeFlagBitsEXTValue::VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT,
                    pfnUserCallback: Self::s_wvkDebugUtilsMessengerCallbackEXT,
                    pUserData: std::ptr::null_mut(),
                };

                return Ok(Some(vk_create_info_));
            }
        }

        Err(WvkError::createWithDescription(WvkErrorType::WVK_INSTANCE_EXTENSION_NOT_FOUND, "Extension not found: VK_EXT_debug_utils."))
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// ТОЛЬКО ДЛЯ РЕЛИЗА. Для дебага другая функция.
    /// Функция для получения заполненной структуры VkDebugUtilsMessengerCreateInfoEXT.
    /// Так как в релизе не используется расширение VK_EXT_debug_utils,
    /// то функция просто возвращает Ok(None)
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[cfg(not(debug_assertions))]
    fn s_createDebugUtilsMessengerCreateInfoForVkInstance(builder : &WvkInstanceBuilder<TWvkVersion>) -> Result<Option<svk::VkDebugUtilsMessengerCreateInfoEXT>, WvkError> {
        Ok(None)
    }

    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    unsafe extern "system" fn s_wvkDebugUtilsMessengerCallbackEXT(
        messageSeverity : svk::svk_types::VkDebugUtilsMessageSeverityFlagsEXT,
        messageTypes : svk::svk_types::VkDebugUtilsMessageTypeFlagsEXT,
        pCallbackData : *const svk::svk_structures::VkDebugUtilsMessengerCallbackDataEXT,
        _pUserData : *mut std::ffi::c_void)
        -> bool {

        let mut message_print_ = String::new();

        if (messageSeverity & svk::svk_enums::VkDebugUtilsMessageSeverityFlagBitsEXTValue::VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT) == svk::svk_enums::VkDebugUtilsMessageSeverityFlagBitsEXTValue::VK_DEBUG_UTILS_MESSAGE_SEVERITY_INFO_BIT_EXT {
            message_print_.push_str("[INFO] ");
        }
        if (messageSeverity & svk::svk_enums::VkDebugUtilsMessageSeverityFlagBitsEXTValue::VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT) == svk::svk_enums::VkDebugUtilsMessageSeverityFlagBitsEXTValue::VK_DEBUG_UTILS_MESSAGE_SEVERITY_VERBOSE_BIT_EXT {
            message_print_.push_str("[VERBOSE] ");
        }
        if (messageSeverity & svk::svk_enums::VkDebugUtilsMessageSeverityFlagBitsEXTValue::VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT) == svk::svk_enums::VkDebugUtilsMessageSeverityFlagBitsEXTValue::VK_DEBUG_UTILS_MESSAGE_SEVERITY_WARNING_BIT_EXT {
            message_print_.push_str("[WARNING] ");
        }
        if (messageSeverity & svk::svk_enums::VkDebugUtilsMessageSeverityFlagBitsEXTValue::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT) == svk::svk_enums::VkDebugUtilsMessageSeverityFlagBitsEXTValue::VK_DEBUG_UTILS_MESSAGE_SEVERITY_ERROR_BIT_EXT {
            message_print_.push_str("[ERROR] ");
        }

        if (messageTypes & svk::svk_enums::VkDebugUtilsMessageTypeFlagBitsEXTValue::VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT) == svk::svk_enums::VkDebugUtilsMessageTypeFlagBitsEXTValue::VK_DEBUG_UTILS_MESSAGE_TYPE_GENERAL_BIT_EXT {
            message_print_.push_str("[GENERAL] ");
        }
        if (messageTypes & svk::svk_enums::VkDebugUtilsMessageTypeFlagBitsEXTValue::VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT) == svk::svk_enums::VkDebugUtilsMessageTypeFlagBitsEXTValue::VK_DEBUG_UTILS_MESSAGE_TYPE_PERFORMANCE_BIT_EXT {
            message_print_.push_str("[PERFORMANCE] ");
        }
        if (messageTypes & svk::svk_enums::VkDebugUtilsMessageTypeFlagBitsEXTValue::VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT) == svk::svk_enums::VkDebugUtilsMessageTypeFlagBitsEXTValue::VK_DEBUG_UTILS_MESSAGE_TYPE_VALIDATION_BIT_EXT {
            message_print_.push_str("[VALIDATION] ");
        }

        let a= std::ffi::CStr::from_ptr((*pCallbackData).pMessage).to_str().unwrap();

        message_print_.push_str(a);

        println!("{}", message_print_);

        false
    }
}

