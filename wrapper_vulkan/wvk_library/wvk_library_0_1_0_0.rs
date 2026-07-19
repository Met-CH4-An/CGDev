// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use crate::wvk_call_with_check;
use crate::wvk::{ WvkEnvironment_0_1_0_0 };
use crate::wvk_error::{ WvkError, WvkErrorType };
use crate::wvk_library::wvk_library::WvkLibrary;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkEnvironment> WvkLibrary<TWvkEnvironment>
where TWvkEnvironment : WvkEnvironment_0_1_0_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn wvkEnumerateInstanceLayerProperties(&self) -> Result<Vec<svk::VkLayerProperties>, WvkError> {
        let mut properties_ : Vec<svk::VkLayerProperties> = Vec::new();

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // получаем количество
        // get the quantity
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let mut count_ : u32 = 0;
        wvk_call_with_check!(
            (self.vkEnumerateInstanceLayerProperties)(&mut count_, std::ptr::null_mut())
        );

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // выделяем память под свойства и запрашиваем свойства через vkEnumerateInstanceLayerProperties
        // allocate memory for properties and request properties via vkEnumerateInstanceLayerProperties
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        //properties.resize_with(count_ as usize, || unsafe { std::mem::zeroed()});
        properties_.reserve(count_ as usize);
        unsafe {
            properties_.set_len(count_ as usize);
        };

        wvk_call_with_check!(
            (self.vkEnumerateInstanceLayerProperties)(&mut count_, properties_.as_mut_ptr())
        );

        Ok(properties_)
    }



    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn wvkEnumerateInstanceExtensionProperties(&self, layer_name : Option<&str>) -> Result<Vec<svk::VkExtensionProperties>, WvkError> {
        let mut properties_ = Vec::<svk::VkExtensionProperties>::new();

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // вулкан апи использует си строки. конвертируем str в CString
        // The Vulcan API uses C strings. Converting str to CString
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let layer_name_cstring_ = layer_name.map(|layer_name_str| {
            std::ffi::CString::new(layer_name_str).map_err(|std_ffi_nul_error| {
                WvkError::createWithDescription(WvkErrorType::WVK_INPUT_PARAMETER_INVALID, &format!("Не удалось получить CString: в &str обнаружен нулевой байт {}", std_ffi_nul_error.nul_position()))
            })
        }).transpose()?;

        let layer_name_ptr_ = layer_name_cstring_
            .as_ref()
            .map_or_else(
                std::ptr::null,
                |v| {
                    v.as_ptr()
                });

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // получаем количество
        // get the quantity
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let mut count_ : u32 = 0;

        wvk_call_with_check!(
            (self.vkEnumerateInstanceExtensionProperties)(layer_name_ptr_, &mut count_, std::ptr::null_mut())
        );

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // выделяем память под свойства и запрашиваем свойства через vkEnumerateInstanceLayerProperties
        // allocate memory for properties and request properties via vkEnumerateInstanceLayerProperties
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        properties_.reserve(count_ as usize);
        unsafe {
            properties_.set_len(count_ as usize);
        }

        wvk_call_with_check!(
            (self.vkEnumerateInstanceExtensionProperties)(layer_name_ptr_, &mut count_, properties_.as_mut_ptr())
        );

        Ok(properties_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn wvkCreateInstance(&self, create_info : &svk::VkInstanceCreateInfo, allocator : Option<&svk::VkAllocationCallbacks>) -> Result<svk::VkInstance, WvkError> {
        /*let allocator_ptr_ = match allocator {
            Some(value) => {
                value as *const svk::VkAllocationCallbacks
            }
            None => std::ptr::null()
        };
        let allocator_ptr_ = allocator.map(|value| {value as *const svk::VkAllocationCallbacks}).unwrap_or(std::ptr::null());
        let allocator_ptr_ = allocator.map_or(std::ptr::null(), |value| {value as *const svk::VkAllocationCallbacks});
        let allocator_ptr_ = allocator.map_or_else(std::ptr::null, |value| {value as *const svk::VkAllocationCallbacks});*/

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // если аалокатор задан - превращает в *const svk::VkAllocationCallbacks
        // если не задан - std::ptr::null
        // If the allocation callback is specified, it converts to *const svk::VkAllocationCallbacks
        // If not specified, it converts to std::ptr::null
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let allocator_ptr_ = allocator.map_or_else(std::ptr::null, |value| {value as *const _});

        let mut vk_instance_ : svk::VkInstance = std::ptr::null_mut();

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // создаем svk::VkInstance
        // create svk::VkInstance
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        wvk_call_with_check!(
            (self.vkCreateInstance)(create_info, allocator_ptr_, &mut vk_instance_)
        );

        Ok(vk_instance_)
    }
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkEnvironment> WvkLibrary<TWvkEnvironment>
where TWvkEnvironment : WvkEnvironment_0_1_0_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_library) fn s_create() -> Result<Self, WvkError> {
        // получаем первичную функцию PFN_vkGetInstanceProcAddr
        let _vkGetInstanceProcAddr = Self::s_loadVkGetInstanceProcAddr().map_err(|wvk_error| {
            wvk_error.addError(WvkErrorType::WVK_LIBRARY_CREATE_FAILED, "Не удалось выполнить loadVkGetInstanceProcAddr. Failed to execute loadVkGetInstanceProcAddr.")
        })?;

        Ok(Self {
            phantom: PhantomData,

            // vulkan 1.0
            vkGetInstanceProcAddr: _vkGetInstanceProcAddr,
            vkEnumerateInstanceLayerProperties: Self::s_loadCommand::<svk::PFN_vkEnumerateInstanceLayerProperties>(_vkGetInstanceProcAddr, c"vkEnumerateInstanceLayerProperties")?,
            vkEnumerateInstanceExtensionProperties: Self::s_loadCommand::<svk::PFN_vkEnumerateInstanceExtensionProperties>(_vkGetInstanceProcAddr, c"vkEnumerateInstanceExtensionProperties")?,
            vkCreateInstance: Self::s_loadCommand::<svk::PFN_vkCreateInstance>(_vkGetInstanceProcAddr, c"vkCreateInstance")?,

            // vulkan 1.1
            vkEnumerateInstanceVersion: Self::s_loadCommand::<svk::PFN_vkEnumerateInstanceVersion>(_vkGetInstanceProcAddr, c"vkEnumerateInstanceVersion")?,
        })
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn s_loadVkGetInstanceProcAddr() -> Result<svk::PFN_vkGetInstanceProcAddr, WvkError> {
        // поддержка платформы MSWindows
        #[cfg(target_os = "windows")]
        {
            // пробуем загрузить vulkan-1.dll
            let _hmodule = unsafe {
                windows::Win32::System::LibraryLoader::LoadLibraryA(windows::core::PCSTR(b"vulkan-1.dll\0".as_ptr()))
                    .map_err(|windows_core_error| {
                        WvkError::createWithDescription(
                            WvkErrorType::WVK_LIBRARY_VULKAN_LIBRARY_LOAD_FAILED,
                            &format!("Не удалось загрузить vulkan-1.dll: LoadLibraryA вернула. Failed to load vulkan-1.dll: LoadLibraryA returned {}.", &windows_core_error.message())
                        )
                    }
                    )?
            };

            // пробуем получить vkGetInstanceProcAddr
            let _proc = unsafe {
                windows::Win32::System::LibraryLoader::GetProcAddress(_hmodule, windows::core::PCSTR(b"vkGetInstanceProcAddr\0".as_ptr()))
                    .ok_or_else(|| {
                        WvkError::createWithDescription(
                            WvkErrorType::WVK_LIBRARY_VULKAN_LIBRARY_LOAD_FAILED,
                            "Не удалось получить адрес vkGetInstanceProcAddr: не найдена в vulkan-1.dll. Failed to get vkGetInstanceProcAddr address: not found in vulkan-1.dll."
                        )
                    }
                    )?
            };

            // преобразовываем в памяти в нужный тип
            let vkGetInstanceProcAddr = unsafe {
                std::mem::transmute::<
                    _,
                    svk::PFN_vkGetInstanceProcAddr
                >(_proc)
            };

            return Ok(vkGetInstanceProcAddr);
        }

        // никакая платформа не поддерживается
        #[cfg(not(any(target_os = "windows")))]
        compile_error!("Платформа не поддерживается. The platform is not supported.");
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn s_loadCommand<TCommand>(vkGetInstanceProcAddr : svk::PFN_vkGetInstanceProcAddr, name : &std::ffi::CStr) -> Result<TCommand, WvkError> {
        // загружаем команду через vkGetInstanceProcAddr
        let _command_raw = unsafe {
            vkGetInstanceProcAddr(std::ptr::null_mut(), name.as_ptr() as *const i8)
        };

        // если не удалось
        if _command_raw.is_null() {
            return Err(WvkError::createWithDescription(
                WvkErrorType::WVK_LIBRARY_VULKAN_COMMAND_LOAD_FAILED,
                &format!("Не удалось загрузить команду вулкана: {}", name.to_string_lossy()))
            );
        };

        // превращаем в конкретный тип команды
        let _command = unsafe {
            std::mem::transmute_copy::<*mut std::ffi::c_void, TCommand>(
                &_command_raw
            )
        };

        Ok(_command)
    }
}
