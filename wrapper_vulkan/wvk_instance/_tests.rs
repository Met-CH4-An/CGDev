// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use crate::wvk::WVK_0_1_0_0;
use crate::wvk_library::WvkLibraryBuilder;
use crate::wvk_instance::wvk_instance_builder::WvkInstanceBuilder;

#[test]
fn wvk_instance__create() {
    let wvk_library_ = WvkLibraryBuilder::<WVK_0_1_0_0>::s_create().build().ok().unwrap();

    let wvk_instance_ = WvkInstanceBuilder::<WVK_0_1_0_0>::create(&wvk_library_).build();

    if let Err(error_) = wvk_instance_ {
       panic!("{}", error_.getMessage());
    }
}