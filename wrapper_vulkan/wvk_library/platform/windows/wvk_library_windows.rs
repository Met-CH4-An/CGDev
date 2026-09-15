// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use windows::core::Free;
use windows::Win32::Foundation::HMODULE;
use crate::wvk_error::{WvkError, WvkErrorType};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WvkLibraryWindows {
    h_module: HMODULE,
}

impl WvkLibraryWindows {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_library) fn create() -> Result<Self, WvkError> {
        // Загружаем vulkan-1.dll.
        // Loading vulkan-1.dll.
        let h_module_ = unsafe {
            windows::Win32::System::LibraryLoader::LoadLibraryA(windows::core::PCSTR(c"vulkan-1.dll".as_ptr() as *const u8))
                .map_err(|windows_core_error| {
                    WvkError::createWithDescription(
                        WvkErrorType::WVK_LIBRARY_VULKAN_LIBRARY_LOAD_FAILED,
                        &format!("Не удалось загрузить vulkan-1.dll. LoadLibraryA вернула. Failed to load vulkan-1.dll. LoadLibraryA returned {}.", &windows_core_error.message())
                    )
                })
        }?;

        Ok(Self{
            h_module: h_module_
        })
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::wvk_library) fn loadVkGetInstanceProcAddr(&self) -> Result<svk::PFN_vkGetInstanceProcAddr, WvkError> {
        // Получаем адрес vkGetInstanceProcAddr.
        // Get the address vkGetInstanceProcAddr.
        let address_raw_ = unsafe {
            windows::Win32::System::LibraryLoader::GetProcAddress(self.h_module, windows::core::PCSTR(c"vkGetInstanceProcAddr".as_ptr() as *const u8))
                .ok_or_else(|| {
                    WvkError::createWithDescription(
                        WvkErrorType::WVK_LIBRARY_VULKAN_LIBRARY_LOAD_FAILED,
                        "Не удалось получить адрес vkGetInstanceProcAddr. Функция не найдена в vulkan-1.dll. Failed to get vkGetInstanceProcAddr address. Function not found in vulkan-1.dll."
                    )
                })
        }?;

        // Преобразовываем в памяти в нужный тип.
        // Convert in memory to the required type.
        let vk_get_instance_proc_addr_ = unsafe {std::mem::transmute::<_,svk::PFN_vkGetInstanceProcAddr>(address_raw_)};

        Ok(vk_get_instance_proc_addr_)
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Деструктор.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Drop for WvkLibraryWindows {
    fn drop(&mut self) {
        unsafe {self.h_module.free()};
    }
}


