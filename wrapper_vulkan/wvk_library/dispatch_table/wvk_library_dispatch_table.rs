// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::wvk::WvkVersion::WVK_0_1_1_0;
use crate::wvk_error::{WvkError, WvkErrorType};
use crate::wvk_library::dispatch_table::{WvkLibraryDispatchTableBuilder};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(in crate::wvk_library) struct WvkLibraryDispatchTable {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Vulkan commands: Global
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // Vulkan 1.0

    pub(in crate::wvk_library::dispatch_table) vk_get_instance_proc_addr : svk::PFN_vkGetInstanceProcAddr,
    pub(in crate::wvk_library::dispatch_table) vk_enumerate_instance_layer_properties : svk::PFN_vkEnumerateInstanceLayerProperties,
    pub(in crate::wvk_library::dispatch_table) vk_enumerate_instance_extension_properties : svk::PFN_vkEnumerateInstanceExtensionProperties,
    pub(in crate::wvk_library::dispatch_table) vk_create_instance : svk::PFN_vkCreateInstance,

    // Vulkan 1.1
    pub(in crate::wvk_library::dispatch_table) vk_enumerate_instance_version : svk::PFN_vkEnumerateInstanceVersion,
}

impl WvkLibraryDispatchTable {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_library::dispatch_table) fn create(builder: WvkLibraryDispatchTableBuilder) -> Result<Self, WvkError> {
        let mut self_ = Self {
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Vulkan commands: Global
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            // Vulkan 1.0

            vk_get_instance_proc_addr: builder.vk_get_instance_proc_addr,
            vk_enumerate_instance_layer_properties: Self::loadCommandAddress::<svk::PFN_vkEnumerateInstanceLayerProperties>(builder.vk_get_instance_proc_addr, std::ptr::null_mut(), c"vkEnumerateInstanceLayerProperties")?,
            vk_enumerate_instance_extension_properties: Self::loadCommandAddress::<svk::PFN_vkEnumerateInstanceExtensionProperties>(builder.vk_get_instance_proc_addr, std::ptr::null_mut(), c"vkEnumerateInstanceExtensionProperties")?,
            vk_create_instance: Self::loadCommandAddress::<svk::PFN_vkCreateInstance>(builder.vk_get_instance_proc_addr, std::ptr::null_mut(), c"vkCreateInstance")?,

            // Vulkan 1.1
            vk_enumerate_instance_version: WvkLibraryDispatchTable::vkEnumerateInstanceVersionDummy,
        };

        if builder.wvk_version >= WVK_0_1_1_0 {
            self_.vk_enumerate_instance_version = Self::loadCommandAddress::<svk::PFN_vkEnumerateInstanceVersion>(builder.vk_get_instance_proc_addr, std::ptr::null_mut(), c"vkEnumerateInstanceVersion")?;
        }

        Ok(self_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Функция загружает адреса команд вулкана, через первичную главную функцию PFN_vkGetInstanceProcAddr.
    /// The function loads the addresses of the volcano commands through the primary main function PFN vkGetInstanceProcAddr.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn loadCommandAddress<TCommand>(vk_get_instance_proc_addr: svk::PFN_vkGetInstanceProcAddr, vk_instance_ptr: svk::VkInstance, name_cstr: &std::ffi::CStr) -> Result<TCommand, WvkError> {
        // Загружаем команду через vkGetInstanceProcAddr.
        // Load the command via vkGetInstanceProcAddr.
        let command_cvoid_ = unsafe {vk_get_instance_proc_addr(vk_instance_ptr, name_cstr.as_ptr() as *const i8)};

        // если не удалось
        if command_cvoid_.is_null() {
            return Err(WvkError::createWithDescription(
                WvkErrorType::WVK_LIBRARY_VULKAN_COMMAND_LOAD_FAILED,
                &format!("Не удалось загрузить команду вулкана. Failed to load volcano command: {}", name_cstr.to_string_lossy())
            ));
        };

        // Превращаем в конкретный тип команды.
        // Convert to a specific command type.
        let command_ = unsafe {std::mem::transmute_copy::<*mut std::ffi::c_void, TCommand>(&command_cvoid_)};

        Ok(command_)
    }
}

