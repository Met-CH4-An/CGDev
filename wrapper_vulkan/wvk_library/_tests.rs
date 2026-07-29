// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use crate::wvk::WVK_0_1_4_0;
use crate::wvk_error::WvkErrorType;
use crate::wvk_library::wvk_library_builder::WvkLibraryBuilder;

#[test]
fn wvk_library__create() {
    let result_ = WvkLibraryBuilder::<WVK_0_1_4_0>::s_create().build();

    if let Err(error_) = result_ {
        panic!("{}", error_.getMessage());
    }
}

#[test]
fn wvk_library__invalid_layer_name() {
    let wvk_library_ = WvkLibraryBuilder::<WVK_0_1_4_0>::s_create().build().ok().unwrap();

    let wvk_error_ = wvk_library_.wvkEnumerateInstanceExtensionProperties(Some("Invalid layer name")).unwrap_err();

    assert_eq!(wvk_error_.getCode(), WvkErrorType::WVK_VK_RESULT(svk::svk_enums::VkResultValue::VK_ERROR_LAYER_NOT_PRESENT));
}