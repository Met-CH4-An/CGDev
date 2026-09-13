// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use crate::wvk::WVK_0_1_0_0;
use crate::wvk_error::WvkErrorType;
use crate::wvk_library::wvk_library_builder::WvkLibraryBuilder;

#[test]
fn wvk_library__create() {
    let wvk_library_ = WvkLibraryBuilder::<WVK_0_1_0_0>::s_create().build();

    if let Err(error_) = wvk_library_ {
        panic!("{}", error_.getMessage());
    }
}