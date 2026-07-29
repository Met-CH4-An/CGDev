// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use crate::dispatch_table::{WvkDispatchTableBuilder, WVK_DISPATCH_TABLE_GLOBAL};
use crate::wvk_call_with_check;
use crate::wvk::{WvkEnvironment, WvkEnvironment_0_1_0_0};
use crate::wvk_error::{WvkError, WvkErrorType};
use crate::wvk_library::WvkLibrary;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные методы.
/// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend> WvkLibrary<TWvkBackend>
where
TWvkBackend : WvkEnvironment_0_1_0_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn wvkGetInstanceProcAddr<TCommand>(&self, vk_instance_ptr : svk::svk_types::VkInstance, name_cstr : &std::ffi::CStr) -> Result<TCommand, WvkError> {
        // Загружаем команду через vkGetInstanceProcAddr.
        // Load the command via vkGetInstanceProcAddr.
        let command_cvoid_ = self.wvk_dispatch_table_global.vkGetInstanceProcAddr(vk_instance_ptr, name_cstr.as_ptr());

        // если не удалось
        if command_cvoid_.is_null() {
            return Err(WvkError::createWithDescription(
                WvkErrorType::WVK_LIBRARY_VULKAN_COMMAND_LOAD_FAILED,
                &format!("Не удалось загрузить команду вулкана. Failed to load volcano command: {}", name_cstr.to_string_lossy())
            ));
        };

        // Превращаем в конкретный тип команды.
        // Convert to a specific command type.
        let command_ = unsafe { std::mem::transmute_copy::<*mut std::ffi::c_void, TCommand>(&command_cvoid_) };

        Ok(command_)
    }

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
            self.wvk_dispatch_table_global.vkEnumerateInstanceLayerProperties(&mut count_, std::ptr::null_mut())
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
            self.wvk_dispatch_table_global.vkEnumerateInstanceLayerProperties(&mut count_, properties_.as_mut_ptr())
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
            self.wvk_dispatch_table_global.vkEnumerateInstanceExtensionProperties(layer_name_ptr_, &mut count_, std::ptr::null_mut())
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
            self.wvk_dispatch_table_global.vkEnumerateInstanceExtensionProperties(layer_name_ptr_, &mut count_, properties_.as_mut_ptr())
        );

        Ok(properties_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn wvkCreateInstance(&self, create_info : &svk::VkInstanceCreateInfo, allocator_opt : Option<&svk::VkAllocationCallbacks>) -> Result<svk::VkInstance, WvkError> {
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
        // если алокатор задан - превращаем в *const svk::VkAllocationCallbacks
        // если не задан - std::ptr::null
        // If the allocation callback is specified, it converts to *const svk::VkAllocationCallbacks
        // If not specified, it converts to std::ptr::null
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        let allocator_ptr_ = allocator_opt.map_or_else(std::ptr::null, |value| {value as *const _});

        let mut vk_instance_ : svk::VkInstance = std::ptr::null_mut();

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // создаем svk::VkInstance
        // create svk::VkInstance
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        wvk_call_with_check!(
            self.wvk_dispatch_table_global.vkCreateInstance(create_info, allocator_ptr_, &mut vk_instance_)
        );

        Ok(vk_instance_)
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
impl<TWvkBackend> WvkLibrary<TWvkBackend>
where
    TWvkBackend : WvkEnvironment_0_1_0_0 {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_library) fn s_create() -> Result<Self, WvkError> {
        let wvk_dispatch_table_global_ = WvkDispatchTableBuilder::<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>::s_create().build()?;

        Ok(Self{
            phantom : PhantomData,
            wvk_dispatch_table_global : wvk_dispatch_table_global_,
        })
    }
}