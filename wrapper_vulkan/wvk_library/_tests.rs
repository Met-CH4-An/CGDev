// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use crate::wvk::WvkVersion;
use crate::wvk_library::wvk_library_builder::WvkLibraryBuilder;

#[test]
fn wvk_library__create() {
    let wvk_library_ = WvkLibraryBuilder::create(WvkVersion::WVK_0_1_0_0).build();

    if let Err(error_) = wvk_library_ {
        panic!("{}", error_.getMessage());
    }
}

//#[test]
//fn wvk_library__enumerate_instance_version__panics_when_unavailable() {
//    let wvk_library_ = WvkLibraryBuilder::create(WvkVersion::WVK_0_1_0_0).build().unwrap();

//    wvk_library_.wvkEnumerateInstanceVersion();
//}