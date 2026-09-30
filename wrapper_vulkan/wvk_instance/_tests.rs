// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use crate::wvk::{WvkVersion};
use crate::wvk_library::WvkLibraryBuilder;
use crate::wvk_instance::wvk_instance_builder::WvkInstanceBuilder;

#[test]
fn wvk_instance__create() {
    let wvk_library_ = WvkLibraryBuilder::create(WvkVersion::WVK_0_1_0_0).build().ok().unwrap();
    
    let wvk_instance_ = WvkInstanceBuilder::create(wvk_library_)
        .addExtension(crate::extensions::VkExtDebugUtils::NAME.into())
        .build();
    
    if let Err(error_) = wvk_instance_ {
       panic!("{}", error_.getMessage());
    }
}