// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use crate::wvk::{WvkVersion};
use crate::wvk_library::WvkLibraryBuilder;
use crate::wvk_instance::wvk_instance_builder::WvkInstanceBuilder;

/*pub struct MyInstanceExtensions {
    pub debug_utils: crate::extensions::VkExtDebugUtils,
}
impl crate::wvk::WvkInstanceExtensions for MyInstanceExtensions {
    fn create() -> Self {
        Self{
            debug_utils: crate::extensions::VkExtDebugUtils::create()
        }

    }
}

impl crate::wvk::WvkInstanceExtensionGet<crate::extensions::VkExtDebugUtils> for MyInstanceExtensions {
    fn get(&self) -> &crate::extensions::VkExtDebugUtils {
        todo!()
    }
}*/

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