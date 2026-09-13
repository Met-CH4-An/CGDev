// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use std::mem::MaybeUninit;
use crate::wvk::{WvkBackend};
use crate::wvk_error::{ WvkError, WvkErrorType };
use crate::wvk_library::dispatch_table::WvkDispatchTablePlatform;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkDispatchTable<TWvkBackend> {
    _phantom_data: PhantomData<TWvkBackend>,
    /// Платформозависимая часть таблицы.
    /// Platform-dependent part of the table.
    dispatch_table_platform: WvkDispatchTablePlatform,

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Vulkan commands: Global
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // Vulkan 1.0

    pub(in crate::wvk_library) vk_get_instance_proc_addr : MaybeUninit<svk::PFN_vkGetInstanceProcAddr>,
    pub(in crate::wvk_library) vk_enumerate_instance_layer_properties : MaybeUninit<svk::PFN_vkEnumerateInstanceLayerProperties>,
    pub(in crate::wvk_library) vk_enumerate_instance_extension_properties : MaybeUninit<svk::PFN_vkEnumerateInstanceExtensionProperties>,
    pub(in crate::wvk_library) vk_create_instance : MaybeUninit<svk::PFN_vkCreateInstance>,

    // Vulkan 1.1
    pub(in crate::wvk_library) vk_enumerate_instance_version : MaybeUninit<svk::PFN_vkEnumerateInstanceVersion>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные ассоциированные функции.
// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend> WvkDispatchTable<TWvkBackend> {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные ассоциированные функции.
// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend> WvkDispatchTable<TWvkBackend>
where
TWvkBackend: WvkBackend {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_library) fn create() -> Result<Self, WvkError> {
        let dispatch_table_platform_ = WvkDispatchTablePlatform::create()?;
        let vk_get_instance_proc_addr_ = dispatch_table_platform_.loadVkGetInstanceProcAddr()?;

        let mut self_ = Self {
            _phantom_data: PhantomData,
            dispatch_table_platform: dispatch_table_platform_,

            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
            // Vulkan commands: Global
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            // Vulkan 1.0

            vk_get_instance_proc_addr: MaybeUninit::new(vk_get_instance_proc_addr_),
            vk_enumerate_instance_layer_properties: MaybeUninit::uninit(),
            vk_enumerate_instance_extension_properties: MaybeUninit::uninit(),
            vk_create_instance: MaybeUninit::uninit(),

            // Vulkan 1.1
            vk_enumerate_instance_version: MaybeUninit::uninit(),
        };

        self_ = Self::loadCommand(self_)?;

        Ok(self_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Получение адресов команд вулкана, которые можно получить с без помощи экземпляра.
    /// Obtaining addresses of volcano commands that can be obtained from without the help of an instance.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn loadCommand(mut dispatch_table: Self) -> Result<Self, WvkError> {
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Vulkan commands: Global
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        // Vulkan 1.0

        dispatch_table.vk_enumerate_instance_layer_properties.write(Self::loadCommandAddress::<svk::PFN_vkEnumerateInstanceLayerProperties>(dispatch_table.vk_get_instance_proc_addr, c"vkEnumerateInstanceLayerProperties")?);
        dispatch_table.vk_enumerate_instance_extension_properties.write(Self::loadCommandAddress::<svk::PFN_vkEnumerateInstanceExtensionProperties>(dispatch_table.vk_get_instance_proc_addr, c"vkEnumerateInstanceExtensionProperties")?);
        dispatch_table.vk_create_instance.write(Self::loadCommandAddress::<svk::PFN_vkCreateInstance>(dispatch_table.vk_get_instance_proc_addr, c"vkCreateInstance")?);

        if TWvkBackend::WVK_ENCODED_VULKAN_VERSION >= svk::VK_MAKE_API_VERSION(0, 1, 1,0) {
            dispatch_table.vk_enumerate_instance_version.write(Self::loadCommandAddress::<svk::PFN_vkEnumerateInstanceVersion>(dispatch_table.vk_get_instance_proc_addr, c"vkEnumerateInstanceVersion")?);
        }

        Ok(dispatch_table)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Функция загружает адреса команд вулкана, через первичную главную функцию PFN_vkGetInstanceProcAddr.
    /// The function loads the addresses of the volcano commands through the primary main function PFN vkGetInstanceProcAddr.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_library) fn loadCommandAddress<TCommand>(vk_get_instance_proc_addr: MaybeUninit<svk::PFN_vkGetInstanceProcAddr>, name_cstr: &std::ffi::CStr) -> Result<TCommand, WvkError> {
        // Загружаем команду через vkGetInstanceProcAddr.
        // Load the command via vkGetInstanceProcAddr.
        let command_cvoid_ = unsafe {vk_get_instance_proc_addr.assume_init()(std::ptr::null_mut(), name_cstr.as_ptr() as *const i8)};

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

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные методы.
// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend> WvkDispatchTable<TWvkBackend>
where
TWvkBackend: WvkBackend {
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
    fn loadVkGetInstanceProcAddr(&mut self) -> Result<(), WvkError> {
        // Загружаем vulkan-1.dll.
        // Loading vulkan-1.dll.
        let _hmodule = unsafe {
            windows::Win32::System::LibraryLoader::LoadLibraryA(windows::core::PCSTR(c"vulkan-1.dll".as_ptr() as *const u8))
                .map_err(|windows_core_error| {
                    WvkError::createWithDescription(
                        WvkErrorType::WVK_LIBRARY_VULKAN_LIBRARY_LOAD_FAILED,
                        &format!("Не удалось загрузить vulkan-1.dll. LoadLibraryA вернула. Failed to load vulkan-1.dll. LoadLibraryA returned {}.", &windows_core_error.message())
                    )
                })
        }?;


        // Получаем адрес vkGetInstanceProcAddr.
        // Get the address vkGetInstanceProcAddr.
        let _proc = unsafe {
            windows::Win32::System::LibraryLoader::GetProcAddress(_hmodule, windows::core::PCSTR(c"vkGetInstanceProcAddr".as_ptr() as *const u8))
                .ok_or_else(|| {
                    WvkError::createWithDescription(
                        WvkErrorType::WVK_LIBRARY_VULKAN_LIBRARY_LOAD_FAILED,
                        "Не удалось получить адрес vkGetInstanceProcAddr. Функция не найдена в vulkan-1.dll. Failed to get vkGetInstanceProcAddr address. Function not found in vulkan-1.dll."
                    )
                })
        }?;

        // Преобразовываем в памяти в нужный тип.
        // Convert in memory to the required type.
        unsafe {self.vk_get_instance_proc_addr.write(std::mem::transmute::<_,svk::PFN_vkGetInstanceProcAddr>(_proc))};

        Ok(())
    }
}


