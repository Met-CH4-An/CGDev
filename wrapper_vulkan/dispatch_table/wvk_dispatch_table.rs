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

impl<TWvkBackend, TLevel> WvkDispatchTable<TWvkBackend, TLevel>
where
TWvkBackend: WvkBackend {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::dispatch_table) fn createAsGlobal(wvk_dispatch_table_builder: WvkDispatchTableBuilder<TWvkBackend, TLevel>) -> Result<Self, WvkError> {
        let mut self_ = Self::create();

        self_.vk_get_instance_proc_addr.write(wvk_dispatch_table_builder.vk_get_instance_proc_addr.unwrap());

        self_.loadCommandAsGlobal()?;

        Ok(self_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::dispatch_table) fn createAsInstance(wvk_dispatch_table_builder: WvkDispatchTableBuilder<TWvkBackend, TLevel>) -> Result<Self, WvkError> {
        let mut self_ = Self::create();

        // Глобальные команды просто копируются из таблицы глобальных команд.
        // Global commands are simply copied from the global commands table.

        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
        // Vulkan commands: Global
        // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

        // Vulkan 1.0

        self_.vk_get_instance_proc_addr = wvk_dispatch_table_builder.wvk_dispatch_table_global.unwrap().vk_get_instance_proc_addr;
        self_.vk_enumerate_instance_layer_properties = wvk_dispatch_table_builder.wvk_dispatch_table_global.unwrap().vk_enumerate_instance_layer_properties;
        self_.vk_enumerate_instance_extension_properties = wvk_dispatch_table_builder.wvk_dispatch_table_global.unwrap().vk_enumerate_instance_extension_properties;
        self_.vk_create_instance = wvk_dispatch_table_builder.wvk_dispatch_table_global.unwrap().vk_create_instance;

        // Vulkan 1.1

        if TWvkBackend::WVK_ENCODED_VULKAN_VERSION >= svk::VK_MAKE_API_VERSION(0, 1, 1,0) {
            self_.vk_enumerate_instance_version = wvk_dispatch_table_builder.wvk_dispatch_table_global.unwrap().vk_enumerate_instance_version;
        }

        self_.loadCommandAsInstance(wvk_dispatch_table_builder.vk_instance.unwrap())?;

        Ok(self_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn create() -> Self {
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

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Получение адресов команд вулкана, которые можно получить без помощи экземпляра. Так называемые глобальные команды.
    /// Retrieving the addresses of Vulcan commands that can be obtained without using an instance. These are known as global commands.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn loadCommandAsGlobal(&mut self) -> Result<(), WvkError> {
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
    fn loadCommandAsInstance(&mut self, vk_instance: svk::VkInstance) -> Result<(), WvkError> {
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

