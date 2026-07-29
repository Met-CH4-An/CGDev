// SPDX-License-Identifier: None
// Copyright (c) 2026 None

use crate::wvk::WVK_0_1_4_0;
use crate::dispatch_table::WVK_DISPATCH_TABLE_GLOBAL;
use crate::dispatch_table::wvk_dispatch_table_builder::WvkDispatchTableBuilder;

#[test]
fn dispatch_table_global__create() {
    let result_ = WvkDispatchTableBuilder::<WVK_0_1_4_0, WVK_DISPATCH_TABLE_GLOBAL>::s_create().build();
    
    if let Err(error_) = result_ {
        panic!("{}", error_.getMessage());
    }
    
}