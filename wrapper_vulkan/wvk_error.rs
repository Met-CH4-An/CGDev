// SPDX-License-Identifier: None
// Copyright (c) 2026 None

pub type WvkError = common::Error::<WvkErrorType>;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[derive(Debug, Clone)]
pub enum WvkErrorType {
    /// ф ыв
    WVK_VK_RESULT(svk::VkResult),
    /// Неправильные входные параметры. Incorrect input parameters.
    WVK_INPUT_PARAMETER_INVALID,
    /// Не удалось создать WvkLibrary. Failed to create WvkLibrary.
    WVK_LIBRARY_CREATE_FAILED,
    /// Не удалось загрузить библиотеку вулкана. Failed to load volcano library.
    WVK_LIBRARY_VULKAN_LIBRARY_LOAD_FAILED,
    /// Не удалось загрузить команду вулкана. Failed to load volcano command.
    WVK_LIBRARY_VULKAN_COMMAND_LOAD_FAILED,
    /// Не удалось создать WvkInstance. Failed to create WvkInstance.
    WVK_INSTANCE_CREATE_FAILED,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl common::ErrorTypeToStr for WvkErrorType {
    fn toStr(&self) -> &str {
        return match self {
            WvkErrorType::WVK_VK_RESULT(_) => "",
            WvkErrorType::WVK_INPUT_PARAMETER_INVALID => "Неправильные входные параметры. Incorrect input parameters.",
            WvkErrorType::WVK_LIBRARY_CREATE_FAILED => "Не удалось создать WvkLibrary. Failed to create WvkLibrary.",
            WvkErrorType::WVK_LIBRARY_VULKAN_LIBRARY_LOAD_FAILED => "Не удалось загрузить библиотеку вулкана. Failed to load volcano library.",
            WvkErrorType::WVK_LIBRARY_VULKAN_COMMAND_LOAD_FAILED => "Не удалось загрузить команду вулкана. Failed to load volcano command.",
            WvkErrorType::WVK_INSTANCE_CREATE_FAILED => "Не удалось создать WvkInstance. Failed to create WvkInstance.",
        };
    }
}