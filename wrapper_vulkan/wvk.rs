// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use std::ffi::CStr;
use std::borrow::Cow;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// маркеры версий
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct WVK_0_1_0_0;
pub struct WVK_0_1_1_0;
pub struct WVK_0_1_2_0;
pub struct WVK_0_1_3_0;
pub struct WVK_0_1_4_0;

mod sealed {
    pub trait Sealed {}
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// приватная область
// private area
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

mod private {
    pub trait WvkBackendPrivate {}
}

pub trait WvkBackend : private::WvkBackendPrivate {
    const WVK_ENCODED_VULKAN_VERSION : u32;
}
pub trait WvkBackend_0_1_0_0 : WvkBackend {}
pub trait WvkBackend_0_1_1_0 : WvkBackend_0_1_0_0 {}
pub trait WvkBackend_0_1_2_0 : WvkBackend_0_1_1_0 {}
pub trait WvkBackend_0_1_3_0 : WvkBackend_0_1_2_0 {}
pub trait WvkBackend_0_1_4_0 : WvkBackend_0_1_3_0 {}
pub trait WvkBackend_Max : WvkBackend_0_1_4_0 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Вулкан 1.0
// Vulkan 1.0
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl private::WvkBackendPrivate for WVK_0_1_0_0 {}
impl WvkBackend for WVK_0_1_0_0 {
    const WVK_ENCODED_VULKAN_VERSION : u32 = crate::svk::VK_MAKE_API_VERSION(0, 1, 0, 0);
}
impl WvkBackend_0_1_0_0 for WVK_0_1_0_0 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Вулкан 1.1
// Vulkan 1.1
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl private::WvkBackendPrivate for WVK_0_1_1_0 {}
impl WvkBackend for WVK_0_1_1_0 {
    const WVK_ENCODED_VULKAN_VERSION : u32 = crate::svk::VK_MAKE_API_VERSION(0, 1, 1, 0);
}
impl WvkBackend_0_1_0_0 for WVK_0_1_1_0 {}
impl WvkBackend_0_1_1_0 for WVK_0_1_1_0 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Вулкан 1.2
// Vulkan 1.2
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl private::WvkBackendPrivate for WVK_0_1_2_0 {}
impl WvkBackend for WVK_0_1_2_0 {
    const WVK_ENCODED_VULKAN_VERSION : u32 = crate::svk::VK_MAKE_API_VERSION(0, 1, 2, 0);
}
impl WvkBackend_0_1_0_0 for WVK_0_1_2_0 {}
impl WvkBackend_0_1_1_0 for WVK_0_1_2_0 {}
impl WvkBackend_0_1_2_0 for WVK_0_1_2_0 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Вулкан 1.3
// Vulkan 1.3
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl private::WvkBackendPrivate for WVK_0_1_3_0 {}
impl WvkBackend for WVK_0_1_3_0 {
    const WVK_ENCODED_VULKAN_VERSION : u32 = crate::svk::VK_MAKE_API_VERSION(0, 1, 3, 0);
}
impl WvkBackend_0_1_0_0 for WVK_0_1_3_0 {}
impl WvkBackend_0_1_1_0 for WVK_0_1_3_0 {}
impl WvkBackend_0_1_2_0 for WVK_0_1_3_0 {}
impl WvkBackend_0_1_3_0 for WVK_0_1_3_0 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Вулкан 1.4
// Vulkan 1.4
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl private::WvkBackendPrivate for WVK_0_1_4_0 {}
impl WvkBackend for WVK_0_1_4_0 {
    const WVK_ENCODED_VULKAN_VERSION: u32 = crate::svk::VK_MAKE_API_VERSION(0, 1, 4, 0);
}
impl WvkBackend_0_1_0_0 for WVK_0_1_4_0 {}
impl WvkBackend_0_1_1_0 for WVK_0_1_4_0 {}
impl WvkBackend_0_1_2_0 for WVK_0_1_4_0 {}
impl WvkBackend_0_1_3_0 for WVK_0_1_4_0 {}
impl WvkBackend_0_1_4_0 for WVK_0_1_4_0 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl WvkBackend_Max for WVK_0_1_4_0 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub(crate) const WRAPPER_VULKAN_NAME : &'static str = "Wrapper Vulkan: WVK";
pub(crate) const WRAPPER_VULKAN_NAME_COW : Cow<'static, CStr> = Cow::Borrowed(c"Wrapper Vulkan: WVK");

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[macro_export]
macro_rules! wvk_call {
    ($vk_command : expr) => {{
        $vk_command;
    }};
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[macro_export]
macro_rules! wvk_call_with_check {
    ($vk_command : expr) => {{
        let vk_result_ = $vk_command;

        if vk_result_ != svk::VkResult::VK_SUCCESS {
            return Err(WvkError::createWithDescription(WvkErrorType::WVK_VK_RESULT(vk_result_), &format!("Не удалось выполнить {}: {:?}.", stringify!($vk_command).replace("\n", " "), vk_result_)))
        }
    }};
}