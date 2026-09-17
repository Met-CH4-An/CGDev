// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use wvk::wvk::WVK_0_1_4_0;
use wvk::wvk_library::{ WvkLibraryBuilder };
use wvk::wvk_instance::{ WvkInstanceBuilder };


fn main() {
    let wvk_library_ = WvkLibraryBuilder::<WVK_0_1_4_0>::create().build().ok().unwrap();

    let application_name_string_ = String::from("name");
    let application_name_str_ = "name";
    
    let engine_name_string_ = String::from("name");
    let engine_name_str_ = "name";

    // Метаданные, которые вулканом не используются. Но могут храниться
    // Metadata that is not used by the volcano. But can be stored
    let _wvk_instance_= WvkInstanceBuilder::<WVK_0_1_4_0>::create(&wvk_library_)
        .applicationName(application_name_string_)
        .applicationName(application_name_str_)
        .applicationName("name")
        .applicationVersion(1)
        .engineName(engine_name_string_)
        .engineName(engine_name_str_)
        .engineName("name")
        .engineVersion(1)
        .build();
}