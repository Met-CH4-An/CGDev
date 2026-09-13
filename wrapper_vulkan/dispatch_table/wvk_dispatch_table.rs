// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::marker::PhantomData;
use std::mem::MaybeUninit;
use crate::dispatch_table::{WvkDispatchTableBuilder, WVK_DISPATCH_TABLE_GLOBAL};
use crate::wvk::{WvkBackend};
use crate::wvk_error::{ WvkError, WvkErrorType };

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkDispatchTable<TWvkBackend, TLevel> {
    _phantom_data: PhantomData<(TWvkBackend, TLevel)>,

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
    // Vulkan commands: Instance
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    pub(in crate::dispatch_table) vk_destroy_instance : MaybeUninit<svk::PFN_vkDestroyInstance>,

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Vulkan commands: VkPhysicalDevice
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // Vulkan 1.0

    pub(in crate::dispatch_table) vk_enumerate_physical_devices : MaybeUninit<svk::PFN_vkEnumeratePhysicalDevices>,
    pub(in crate::dispatch_table) vk_get_physical_device_properties : MaybeUninit<svk::PFN_vkGetPhysicalDeviceProperties>,

    // Vulkan 1.1

    pub(in crate::dispatch_table) vk_get_physical_device_properties_2 : MaybeUninit<svk::PFN_vkGetPhysicalDeviceProperties2>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные ассоциированные функции.
// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend, TLevel> WvkDispatchTable<TWvkBackend, TLevel> {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные ассоциированные функции.
// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend, TLevel> WvkDispatchTable<TWvkBackend, TLevel>
where
TWvkBackend: WvkBackend {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::dispatch_table) fn s_createWithGlobal(_wvk_dispatch_table_builder: WvkDispatchTableBuilder<TWvkBackend, TLevel>) -> Result<Self, WvkError> {
        let mut self_ = Self::s_create();

        self_.loadCommand()?;

        Ok(self_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::dispatch_table) fn s_createWithInstance(wvk_dispatch_table_builder: WvkDispatchTableBuilder<TWvkBackend, TLevel>) -> Result<Self, WvkError> {
        let mut self_ = Self::s_create();

        self_.loadCommandWithInstance(&wvk_dispatch_table_builder.wvk_dispatch_table_global__opt.unwrap(), wvk_dispatch_table_builder.vk_instance__opt.unwrap())?;

        Ok(self_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn s_create() -> Self {
        Self {
            _phantom_data: PhantomData,

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
            // Vulkan commands: Instance
            // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

            vk_destroy_instance:  MaybeUninit::uninit(),

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
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные методы.
// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl<TWvkBackend, TLevel> WvkDispatchTable<TWvkBackend, TLevel>
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

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Получение адресов команд вулкана, которые можно получить с без помощи экземпляра.
    /// Obtaining addresses of volcano commands that can be obtained from without the help of an instance.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn loadCommand(&mut self) -> Result<(), WvkError> {
        // Получаем адрес vkGetInstanceProcAddr.
        // Get the address vkGetInstanceProcAddr.
        self.loadVkGetInstanceProcAddr()?;

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Vulkan commands: Global
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        // Vulkan 1.0

        self.vk_enumerate_instance_layer_properties.write(self.loadCommandAddress::<svk::PFN_vkEnumerateInstanceLayerProperties>(std::ptr::null_mut(), c"vkEnumerateInstanceLayerProperties")? );
        self.vk_enumerate_instance_extension_properties.write(self.loadCommandAddress::<svk::PFN_vkEnumerateInstanceExtensionProperties>(std::ptr::null_mut(), c"vkEnumerateInstanceExtensionProperties")? );
        self.vk_create_instance.write(self.loadCommandAddress::<svk::PFN_vkCreateInstance>(std::ptr::null_mut(), c"vkCreateInstance")? );

        if TWvkBackend::WVK_ENCODED_VULKAN_VERSION >= svk::VK_MAKE_API_VERSION(0, 1, 1,0) {
            self.vk_enumerate_instance_version.write(self.loadCommandAddress::<svk::PFN_vkEnumerateInstanceVersion>(std::ptr::null_mut(), c"vkEnumerateInstanceVersion")? );
        }

        Ok(())
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Получение адресов команд вулкана, которые можно получить с помощью экземпляра.
    /// Getting the addresses of the volcano commands that can be obtained using the instance.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn loadCommandWithInstance(&mut self, wvk_dispatch_table_global: &WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL>, vk_instance: svk::VkInstance) -> Result<(), WvkError> {
        // Глобальные команды просто копируются из WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL.
        // Global commands are simply copied from WvkDispatchTable<TWvkBackend, WVK_DISPATCH_TABLE_GLOBAL.

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Vulkan commands: Global
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        // Vulkan 1.0

        self.vk_get_instance_proc_addr = wvk_dispatch_table_global.vk_get_instance_proc_addr;
        self.vk_enumerate_instance_layer_properties = wvk_dispatch_table_global.vk_enumerate_instance_layer_properties;
        self.vk_enumerate_instance_extension_properties = wvk_dispatch_table_global.vk_enumerate_instance_extension_properties;
        self.vk_create_instance = wvk_dispatch_table_global.vk_create_instance;

        // Vulkan 1.1

        if TWvkBackend::WVK_ENCODED_VULKAN_VERSION >= svk::VK_MAKE_API_VERSION(0, 1, 1,0) {
            self.vk_enumerate_instance_version = wvk_dispatch_table_global.vk_enumerate_instance_version;
        }

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Vulkan commands: Instance
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        // Vulkan 1.0

        self.vk_destroy_instance.write(
            self.loadCommandAddress::<svk::PFN_vkDestroyInstance>(vk_instance, c"vkDestroyInstance")? );

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Vulkan commands: VkPhysicalDevice
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        // Vulkan 1.0

        self.vk_enumerate_physical_devices.write(self.loadCommandAddress::<svk::PFN_vkEnumeratePhysicalDevices>(vk_instance, c"vkEnumeratePhysicalDevices")? );
        self.vk_get_physical_device_properties.write(self.loadCommandAddress::<svk::PFN_vkGetPhysicalDeviceProperties>(vk_instance, c"vkGetPhysicalDeviceProperties")? );

        // Vulkan 1.1

        if TWvkBackend::WVK_ENCODED_VULKAN_VERSION >= svk::VK_MAKE_API_VERSION(0, 1, 1,0) {
            self.vk_get_physical_device_properties_2.write(self.loadCommandAddress::<svk::PFN_vkGetPhysicalDeviceProperties2>(vk_instance, c"vkGetPhysicalDeviceProperties2")? );
        }

        Ok(())
    }



    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Функция загружает адреса команд вулкана, через первичную главную функцию PFN_vkGetInstanceProcAddr.
    /// The function loads the addresses of the volcano commands through the primary main function PFN vkGetInstanceProcAddr.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::dispatch_table) fn loadCommandAddress<TCommand>(&self, vk_instance_ptr: svk::VkInstance, name_cstr: &std::ffi::CStr) -> Result<TCommand, WvkError> {
        // Загружаем команду через vkGetInstanceProcAddr.
        // Load the command via vkGetInstanceProcAddr.
        let command_cvoid_ = unsafe {self.vk_get_instance_proc_addr.assume_init()(vk_instance_ptr, name_cstr.as_ptr() as *const i8)};

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

impl<TWvkBackend, TLevel> Drop for WvkDispatchTable<TWvkBackend, TLevel> {
    fn drop(&mut self) {
        //self.
        todo!()
    }
}


