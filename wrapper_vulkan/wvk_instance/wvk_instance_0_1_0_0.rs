// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ffi::CString;
use std::marker::PhantomData;
use std::sync::Arc;
use crate::wvk_call_with_check;
use crate::wvk::{ WvkBackend_0_1_0_0 };
use crate::wvk_error::{ WvkError, WvkErrorType };
use crate::wvk_instance::dispatch_table::WvkDispatchTable;
use crate::wvk_instance::wvk_instance_builder::WvkInstanceBuilder;
use crate::wvk_instance::wvk_instance::WvkInstance;
use crate::wvk_physical_device::wvk_physical_device_builder::WvkPhysicalDeviceBuilder;
use crate::wvk_physical_device::wvk_physical_device::WvkPhysicalDevice;

impl<TWvkBackend> WvkInstance<TWvkBackend>
where TWvkBackend : WvkBackend_0_1_0_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn wvkDestroyInstance(&self) {
        unsafe {
            self.wvk_dispatch_table.vk_destroy_instance.assume_init()(self.vk_instance, std::ptr::null_mut())
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //pub fn wvkEnumeratePhysicalDevices(self: &Arc<Self>) -> Result<Vec<WvkPhysicalDevice<TWvkBackend>>, WvkError> {
    pub fn wvkEnumeratePhysicalDevices(self: &Arc<Self>) -> Result<Vec<WvkPhysicalDevice<TWvkBackend>>, WvkError> {
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Получаем количество физических устройств VkPhysicalDevice.
        // Get the number of physical devices VkPhysicalDevice.
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let mut count_ : u32 = 0;
        wvk_call_with_check!(
            unsafe {
                self.wvk_dispatch_table.vk_enumerate_physical_devices.assume_init()(self.vk_instance, &mut count_, std::ptr::null_mut())
            }
        );

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Выделить место для данных, исходя из полученного количества.
        // Allocate space for data based on the received quantity.
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let mut vk_physical_devices_ = Vec::<svk::VkPhysicalDevice>::with_capacity(count_ as usize);
        unsafe { vk_physical_devices_.set_len(count_ as usize) }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Получаем список физических устройств VkPhysicalDevice.
        // We get a list of physical devices VkPhysicalDevice.
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        wvk_call_with_check!(
            unsafe {
                self.wvk_dispatch_table.vk_enumerate_physical_devices.assume_init()(self.vk_instance, &mut count_, vk_physical_devices_.as_mut_ptr())
            }
        );

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Перебираем полученный список физических устройств VkPhysicalDevice и формируем обертки WvkPhysicalDevice.
        // We iterate over the received list of physical devices VkPhysicalDevice and form WvkPhysicalDevice wrappers.
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        vk_physical_devices_
            .iter()
            .map(|v| {
                WvkPhysicalDeviceBuilder::<TWvkBackend>::create();
            });
        let mut wvk_physical_devices_ = Vec::<WvkPhysicalDevice<TWvkBackend>>::with_capacity(count_ as usize);
        wvk_physical_devices_
            .iter()
            .for_each(|v| {

            });

        for vk_physical_device_ in &vk_physical_devices_ {
            let wvk_physical_device_ = WvkPhysicalDeviceBuilder::<TWvkBackend>::s_create(*vk_physical_device_, self.clone()).build()?;

            wvk_physical_devices_.push(wvk_physical_device_);
        }

        Ok(wvk_physical_devices_)
    }
}

impl<TWvkBackend> WvkInstance<TWvkBackend>
where TWvkBackend : WvkBackend_0_1_0_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_instance) fn create(wvk_instance_builder: & WvkInstanceBuilder<TWvkBackend>) -> Result<WvkInstance<TWvkBackend>, WvkError> {
        // Создаем непосредственно VkInstance.
        // Create VkInstance directly.
        let vk_instance_ = Self::createVkInstance(&wvk_instance_builder)?;

        // Далее используя VkInstance можно создать таблицу функций.
        // Next, using VkInstance, you can create a table of functions.
        let wvk_dispatch_table_ = WvkDispatchTable::<TWvkBackend>::create(&wvk_instance_builder, vk_instance_)?;

        let self_ = Self {
            _phantom_data : PhantomData,
            wvk_dispatch_table: wvk_dispatch_table_,
            vk_instance : vk_instance_,
        };

        Ok(self_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Создает экземпляр вулкана.
    /// Creates a volcano instance.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn createVkInstance(builder: &WvkInstanceBuilder<TWvkBackend>) -> Result<svk::VkInstance, WvkError> {
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
            apiVersion : TWvkBackend::WVK_ENCODED_VULKAN_VERSION,
        };

        // Для создания VkInstance описываем его через VkInstanceCreateInfo
        // To create a VkInstance, we describe it using VkInstanceCreateInfo
        let vk_create_info_ = svk::VkInstanceCreateInfo {
            sType : svk::VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
            pNext : p_next_,
            flags : svk::VkInstanceCreateFlags(0),
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
    fn createDebugUtilsMessengerCreateInfoForVkInstance(builder: &WvkInstanceBuilder<TWvkBackend>) -> Result<Option<svk::VkDebugUtilsMessengerCreateInfoEXT>, WvkError> {
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
}

