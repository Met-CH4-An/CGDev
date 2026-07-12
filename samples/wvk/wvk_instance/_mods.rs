// SPDX-License-Identifier: None
// Copyright (c) 2026 None

fn main() {
    let wvk_library_ = wvk::WvkLibrary::create().ok().unwrap();

    let name_cstring_ = std::ffi::CString::from(c"Application name");
    let name_cstr_ = c"Application name";
    let name_string_ = String::from("Application name");

    // Метаданные, которые вулканом не используются. Но могут храниться
    // Metadata that is not used by the volcano. But can be stored
    let wvk_instance= wvk::wvk_instance::WvkInstanceBuilder::<wvk::Wvk_0_1_4_0>::s_create(&wvk_library_)
        .applicationNameFromCStr(name_cstring_)
        .applicationNameFromCStr(name_cstr_)
        .applicationName(name_string_)
        .applicationVersion(1)
        .engineNameFromCStr(c"Engine name")
        .engineVersion(1)
        .build();
}