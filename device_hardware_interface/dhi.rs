// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// маркеры версий
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub struct DHI_WVK_0_1_0_0;
pub struct DHI_WVK_0_1_1_0;
pub struct DHI_WVK_0_1_2_0;
pub struct DHI_WVK_0_1_3_0;
pub struct DHI_WVK_0_1_4_0;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Wvk1.0
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub trait DhiWvkVersion_0_1_0_0 {
    type WvkVersion;
}
pub trait DhiWvkVersion_0_1_1_0 : DhiWvkVersion_0_1_0_0 {}
pub trait DhiWvkVersion_0_1_2_0 : DhiWvkVersion_0_1_1_0 {}
pub trait DhiWvkVersion_0_1_3_0 : DhiWvkVersion_0_1_2_0 {}
pub trait DhiWvkVersion_0_1_4_0 : DhiWvkVersion_0_1_3_0 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Wvk 1.0
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl DhiWvkVersion_0_1_0_0 for DHI_WVK_0_1_0_0 {
    type WvkVersion = wvk::WVK_0_1_0_0;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Wvk 1.1
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl DhiWvkVersion_0_1_0_0 for DHI_WVK_0_1_1_0 {
    type WvkVersion = wvk::WVK_0_1_1_0;
}
impl DhiWvkVersion_0_1_1_0 for DHI_WVK_0_1_1_0 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Wvk 1.2
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl DhiWvkVersion_0_1_0_0 for DHI_WVK_0_1_2_0 {
    type WvkVersion = wvk::WVK_0_1_2_0;
}
impl DhiWvkVersion_0_1_1_0 for DHI_WVK_0_1_2_0 {}
impl DhiWvkVersion_0_1_2_0 for DHI_WVK_0_1_2_0 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Wvk 1.3
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl DhiWvkVersion_0_1_0_0 for DHI_WVK_0_1_3_0 {
    type WvkVersion = wvk::WVK_0_1_3_0;
}
impl DhiWvkVersion_0_1_1_0 for DHI_WVK_0_1_3_0 {}
impl DhiWvkVersion_0_1_2_0 for DHI_WVK_0_1_3_0 {}
impl DhiWvkVersion_0_1_3_0 for DHI_WVK_0_1_3_0 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Wvk 1.4
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl DhiWvkVersion_0_1_0_0 for DHI_WVK_0_1_4_0 {
    type WvkVersion = wvk::WVK_0_1_4_0;
}
impl DhiWvkVersion_0_1_1_0 for DHI_WVK_0_1_4_0 {}
impl DhiWvkVersion_0_1_2_0 for DHI_WVK_0_1_4_0 {}
impl DhiWvkVersion_0_1_3_0 for DHI_WVK_0_1_4_0 {}
impl DhiWvkVersion_0_1_4_0 for DHI_WVK_0_1_4_0 {}