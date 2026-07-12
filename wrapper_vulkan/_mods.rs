// SPDX-License-Identifier: None
// Copyright (c) 2026 None

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use common;
pub use svk;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// модули
// modules
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
mod wvk;
pub use wvk::*;

mod wvk_error;
pub use wvk_error::*;

mod wvk_library;
pub use wvk_library::*;

pub mod wvk_instance {
    mod wvk_instance;
    pub use wvk_instance::*;

    mod wvk_instance_100;
    pub use wvk_instance_100::*;

    mod wvk_instance_builder;
    pub use wvk_instance_builder::*;
}

pub(crate) mod extensions {
    pub(crate) mod VK_EXT_debug_utils;
    pub use VK_EXT_debug_utils::*;
}

pub(crate) mod traits {
    pub(crate) mod wvk_debug;
    pub use wvk_debug::*;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[macro_export]
macro_rules! wvk_call_with_check {
    ($vk_command : expr) => {{
        let vk_result_ = unsafe {
            $vk_command
        };

        if vk_result_ != svk::VkResultValue::VK_SUCCESS {
            return Err(WvkError::createWithDescription(WvkErrorType::WVK_VK_RESULT(vk_result_), &format!("Не удалось выполнить {}: {}.", stringify!($vk_command).replace("\n", " "), vk_result_)))
        }
    }};
}

