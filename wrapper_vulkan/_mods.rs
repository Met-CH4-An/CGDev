// SPDX-License-Identifier: None
// Copyright (c) 2026 None

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use common;
pub use svk;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// модули
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
mod wvk_error;
pub use wvk_error::*;

mod wvk_library;
pub use wvk_library::*;

mod wvk_instance;
pub use wvk_instance::*;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[macro_export]
macro_rules! wvk_call_with_check {
    ($vk_command : expr) => {{
        let vk_result_ = unsafe {
            $vk_command
        };

        if vk_result_ != svk::VkResultCode::VK_SUCCESS {
            return Err(WvkError::newWithMessage(WvkErrorType::WVK_VK_RESULT(vk_result_), &format!("Не удалось выполнить {}: {}.", stringify!($vk_command).replace("\n", " "), vk_result_)))
        }
    }};
}

