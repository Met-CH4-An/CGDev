// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use wvk::wvk::WVK_0_1_4_0;
use wvk::wvk_library::{ WvkLibraryBuilder };
use wvk::wvk_instance::{ WvkInstanceBuilder };


fn main() {
    let wvk_library_ = WvkLibraryBuilder::<WVK_0_1_4_0>::s_create().build().ok().unwrap();

    let name_cstring_ = std::ffi::CString::from(c"Application name");
    let name_cstr_ = c"Application name";
    let name_string_ = String::from("Application name");

    // Метаданные, которые вулканом не используются. Но могут храниться
    // Metadata that is not used by the volcano. But can be stored
    let _wvk_instance_= WvkInstanceBuilder::<WVK_0_1_4_0>::s_create(&wvk_library_)
        .applicationNameFromCStr(name_cstring_)
        .applicationNameFromCStr(name_cstr_)
        .applicationName(name_string_)
        .applicationVersion(1)
        .engineNameFromCStr(c"Engine name")
        .engineVersion(1)
        .build();
}