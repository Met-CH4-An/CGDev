// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use std::ffi::c_void;
use {
    crate::wvk_library::WvkLibrary,
    crate::extensions::VK_EXT_debug_utils::VK_EXT_debug_utils,
};

pub(crate) trait WvkDebug : 'static {
    fn buildWvkInstanceDebugCreateInfo(wvk_library : &WvkLibrary) -> *const std::ffi::c_void;
}

impl WvkDebug for VK_EXT_debug_utils {
    fn buildWvkInstanceDebugCreateInfo(wvk_library: &WvkLibrary) -> *const c_void {
        todo!()
    }
}
