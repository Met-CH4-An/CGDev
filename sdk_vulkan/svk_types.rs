// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// Provided by VK_VERSION_1_0
pub type r#enum = i32;

// Provided by VK_VERSION_1_0
pub type bitmask = u32;

// Provided by VK_VERSION_1_0
pub type VkFlags = u32;

// Provided by VK_VERSION_1_0
pub type VkFlags64 = u64;

// Provided by VK_VERSION_1_0
pub type VkInstanceCreateFlags = VkFlags;

// Provided by VK_VERSION_1_0
#[repr(C)]
pub struct VkInstance_T {
    _private: [u8; 0],
}
pub type VkInstance = *mut VkInstance_T;

// Provided by VK_EXT_debug_report
pub type VkDebugReportFlagsEXT = VkFlags;