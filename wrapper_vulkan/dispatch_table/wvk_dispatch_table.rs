// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use std::mem::MaybeUninit;

use crate::wvk::WvkEnvironment;
use crate::wvk_error::{ WvkError, WvkErrorType };

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkDispatchTable<TWvkBackend, TLevel> {
    pub(in crate::dispatch_table) phantom_data: PhantomData<(TWvkBackend, TLevel)>,

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Vulkan commands: Global
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // Vulkan 1.0

    pub(in crate::dispatch_table) vk_get_instance_proc_addr : MaybeUninit<svk::PFN_vkGetInstanceProcAddr>,
    pub(in crate::dispatch_table) vk_enumerate_instance_layer_properties : MaybeUninit<svk::PFN_vkEnumerateInstanceLayerProperties>,
    pub(in crate::dispatch_table) vk_enumerate_instance_extension_properties : MaybeUninit<svk::PFN_vkEnumerateInstanceExtensionProperties>,
    pub(in crate::dispatch_table) vk_create_instance : MaybeUninit<svk::PFN_vkCreateInstance>,

    // Vulkan 1.1
    pub(in crate::dispatch_table) vk_enumerate_instance_version : MaybeUninit<svk::PFN_vkEnumerateInstanceVersion>,

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Vulkan commands: VkPhysicalDevice
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // Vulkan 1.0

    pub(in crate::dispatch_table) vk_enumerate_physical_devices : MaybeUninit<svk::svk_commands::PFN_vkEnumeratePhysicalDevices>,
    pub(in crate::dispatch_table) vk_get_physical_device_properties : MaybeUninit<svk::svk_commands::PFN_vkGetPhysicalDeviceProperties>,

    // Vulkan 1.1

    pub(in crate::dispatch_table) vk_get_physical_device_properties_2 : MaybeUninit<svk::svk_commands::PFN_vkGetPhysicalDeviceProperties2>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные ассоциированные функции.
/// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend, TLevel> WvkDispatchTable<TWvkBackend, TLevel> {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// приватная область
// private area
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Приватные ассоциированные функции.
/// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend, TLevel> WvkDispatchTable<TWvkBackend, TLevel>
where
TWvkBackend : WvkEnvironment {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::dispatch_table) fn s_initialize() -> Self {
        Self {
            phantom_data: PhantomData,

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Vulkan commands: Global
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            // Vulkan 1.0

            vk_get_instance_proc_addr: MaybeUninit::uninit(),
            vk_enumerate_instance_layer_properties: MaybeUninit::uninit(),
            vk_enumerate_instance_extension_properties: MaybeUninit::uninit(),
            vk_create_instance: MaybeUninit::uninit(),

            // Vulkan 1.1
            vk_enumerate_instance_version: MaybeUninit::uninit(),

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Vulkan commands: VkPhysicalDevice
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            // Vulkan 1.0

            vk_enumerate_physical_devices: MaybeUninit::uninit(),
            vk_get_physical_device_properties: MaybeUninit::uninit(),

            // Vulkan 1.1

            vk_get_physical_device_properties_2: MaybeUninit::uninit(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::dispatch_table) fn s_loadCommand(mut dispatch_table: Self) -> Result<Self, WvkError> {
        // Получаем адрес vkGetInstanceProcAddr.
        // Get the address vkGetInstanceProcAddr.
        let vkGetInstanceProcAddr_ = unsafe { Self::s_loadVkGetInstanceProcAddr()? };

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Vulkan commands: Global
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        // Vulkan 1.0

        dispatch_table.vk_get_instance_proc_addr.write(vkGetInstanceProcAddr_);
        dispatch_table.vk_enumerate_instance_layer_properties.write(
            unsafe { Self::s_loadCommandAddress::<svk::svk_commands::PFN_vkEnumerateInstanceLayerProperties>(vkGetInstanceProcAddr_, std::ptr::null_mut(), c"vkEnumerateInstanceLayerProperties")? });
        dispatch_table.vk_enumerate_instance_extension_properties.write(
            unsafe { Self::s_loadCommandAddress::<svk::svk_commands::PFN_vkEnumerateInstanceExtensionProperties>(vkGetInstanceProcAddr_, std::ptr::null_mut(), c"vkEnumerateInstanceExtensionProperties")? });
        dispatch_table.vk_create_instance.write(
            unsafe { Self::s_loadCommandAddress::<svk::svk_commands::PFN_vkCreateInstance>(vkGetInstanceProcAddr_, std::ptr::null_mut(), c"vkCreateInstance")? });

        if TWvkBackend::WVK_ENCODED_VULKAN_VERSION < svk::svk_macros::VK_MAKE_API_VERSION(0, 1, 1,0) {
            return Ok(dispatch_table);
        }

        dispatch_table.vk_enumerate_instance_version.write(
            unsafe { Self::s_loadCommandAddress::<svk::svk_commands::PFN_vkEnumerateInstanceVersion>(*dispatch_table.vk_get_instance_proc_addr.as_ptr(), std::ptr::null_mut(), c"vkEnumerateInstanceVersion")? });

        Ok(dispatch_table)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Получение адресов команд вулкана, которые можно получить с помощью экземпляра.
    /// Getting the addresses of the volcano commands that can be obtained using the instance.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::dispatch_table) fn s_loadCommandWithInstance(mut dispatch_table: Self, vk_instance: svk::svk_types::VkInstance) -> Result<Self, WvkError> {
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Vulkan commands: VkPhysicalDevice
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        // Vulkan 1.0

        dispatch_table.vk_enumerate_physical_devices.write(
            unsafe { Self::s_loadCommandAddress::<svk::svk_commands::PFN_vkEnumeratePhysicalDevices>(*dispatch_table.vk_get_instance_proc_addr.as_ptr(), vk_instance, c"vkEnumeratePhysicalDevices")? });
        dispatch_table.vk_get_physical_device_properties.write(
            unsafe { Self::s_loadCommandAddress::<svk::svk_commands::PFN_vkGetPhysicalDeviceProperties>(*dispatch_table.vk_get_instance_proc_addr.as_ptr(), vk_instance, c"vkGetPhysicalDeviceProperties")? });

        // Vulkan 1.1

        dispatch_table.vk_get_physical_device_properties_2.write(
            unsafe { Self::s_loadCommandAddress::<svk::svk_commands::PFN_vkGetPhysicalDeviceProperties2>(*dispatch_table.vk_get_instance_proc_addr.as_ptr(), vk_instance, c"vkGetPhysicalDeviceProperties2")? });

        Ok(dispatch_table)
    }
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Функция загружает адреса команд вулкана, через первичную главную функцию PFN_vkGetInstanceProcAddr.
    /// The function loads the addresses of the volcano commands through the primary main function PFN vkGetInstanceProcAddr.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::dispatch_table) unsafe fn s_loadCommandAddress<TCommand>(vkGetInstanceProcAddr : svk::PFN_vkGetInstanceProcAddr, vk_instance_ptr : svk::svk_types::VkInstance, name_cstr : &std::ffi::CStr) -> Result<TCommand, WvkError> {
        // Загружаем команду через vkGetInstanceProcAddr.
        // Load the command via vkGetInstanceProcAddr.
        let command_cvoid_ = vkGetInstanceProcAddr(vk_instance_ptr, name_cstr.as_ptr() as *const i8);

        // если не удалось
        if command_cvoid_.is_null() {
            return Err(WvkError::createWithDescription(
                WvkErrorType::WVK_LIBRARY_VULKAN_COMMAND_LOAD_FAILED,
                &format!("Не удалось загрузить команду вулкана. Failed to load volcano command: {}", name_cstr.to_string_lossy())
            ));
        };

        // Превращаем в конкретный тип команды.
        // Convert to a specific command type.
        let command_ = std::mem::transmute_copy::<*mut std::ffi::c_void, TCommand>(&command_cvoid_);

        Ok(command_)
    }
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Функция загружает dll вулкана 'vulkan-1.dll' и затем получает из загруженной
    /// dll адрес функции 'vkGetInstanceProcAddr'.
    /// Для этого используется официальный крейт 'windows' от MSWindows и их официальный WinAPI.
    ///
    /// The function loads the Vulkan DLL 'vulkan-1.dll' and then obtains the address of the 'vkGetInstanceProcAddr' function from the loaded
    /// DLL.
    /// This uses the official 'windows' crate from MSWindows and their official WinAPI.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[cfg(target_os = "windows")]
    unsafe fn s_loadVkGetInstanceProcAddr() -> Result<svk::PFN_vkGetInstanceProcAddr, WvkError> {
        // Загружаем vulkan-1.dll.
        // Loading vulkan-1.dll.
        let _hmodule = windows::Win32::System::LibraryLoader::LoadLibraryA(windows::core::PCSTR(c"vulkan-1.dll".as_ptr() as *const u8))
            .map_err(|windows_core_error| {
                WvkError::createWithDescription(
                    WvkErrorType::WVK_LIBRARY_VULKAN_LIBRARY_LOAD_FAILED,
                    &format!("Не удалось загрузить vulkan-1.dll. LoadLibraryA вернула. Failed to load vulkan-1.dll. LoadLibraryA returned {}.", &windows_core_error.message())
                )
            })
        ?;

        // Получаем адрес vkGetInstanceProcAddr.
        // Get the address vkGetInstanceProcAddr.
        let _proc = windows::Win32::System::LibraryLoader::GetProcAddress(_hmodule, windows::core::PCSTR(c"vkGetInstanceProcAddr".as_ptr() as *const u8))
            .ok_or_else(|| {
                WvkError::createWithDescription(
                    WvkErrorType::WVK_LIBRARY_VULKAN_LIBRARY_LOAD_FAILED,
                    "Не удалось получить адрес vkGetInstanceProcAddr. Функция не найдена в vulkan-1.dll. Failed to get vkGetInstanceProcAddr address. Function not found in vulkan-1.dll."
                )
            })
        ?;

        // Преобразовываем в памяти в нужный тип.
        // Convert in memory to the required type.
        let vkGetInstanceProcAddr = std::mem::transmute::<_,svk::PFN_vkGetInstanceProcAddr>(_proc);

        Ok(vkGetInstanceProcAddr)
    }
}


