// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use std::convert::Into;
use std::ffi::CStr;
use std::borrow::Cow;

pub(crate) const WRAPPER_VULKAN_NAME : &'static CStr = c"Wrapper Vulkan: WVK";
pub(crate) const WRAPPER_VULKAN_NAME_COW : Cow<'static, CStr> = Cow::Borrowed(c"Wrapper Vulkan: WVK");

pub(crate) const fn GET_VULKAN_VERSION()  -> u32 {
    #[cfg(all(feature = "vulkan_1_0", not(feature = "vulkan_1_1")))]
    return crate::svk::VK_MAKE_API_VERSION(0, 1, 0, 0);

    #[cfg(all(feature = "vulkan_1_1", not(feature = "vulkan_1_2")))]
    return crate::svk::VK_MAKE_API_VERSION(0, 1, 1, 0);

    #[cfg(all(feature = "vulkan_1_2", not(feature = "vulkan_1_3")))]
    return crate::svk::VK_MAKE_API_VERSION(0, 1, 2, 0);

    #[cfg(all(feature = "vulkan_1_3", not(feature = "vulkan_1_4")))]
    return crate::svk::VK_MAKE_API_VERSION(0, 1, 3, 0);

    #[cfg(all(feature = "vulkan_1_4"))]
    return crate::svk::VK_MAKE_API_VERSION(0, 1, 4, 0);
}