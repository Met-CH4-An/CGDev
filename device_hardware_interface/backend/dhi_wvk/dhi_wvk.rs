// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// маркеры версий
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Wvk1.0
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::dhi::{DhiBackend, DHI_WVK_0_1_0_0, DHI_WVK_0_1_1_0, DHI_WVK_0_1_2_0, DHI_WVK_0_1_3_0, DHI_WVK_0_1_4_0};
use crate::backend::dhi_wvk::dhi_wvk_context::dhi_wvk_context::DhiWvkContext;
use crate::dhi_error::DHIError;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub trait DhiWvkBackend : DhiBackend {
    type WvkVersion: wvk::wvk::WvkEnvironment;
}

pub trait DhiWvkBackend_0_1_0_0 : DhiWvkBackend
where Self::WvkVersion : wvk::wvk::WvkEnvironment_0_1_0_0 {}

pub trait DhiWvkBackend_0_1_1_0 : DhiWvkBackend_0_1_0_0
where Self::WvkVersion : wvk::wvk::WvkEnvironment_0_1_1_0 {}

pub trait DhiWvkBackend_0_1_2_0 : DhiWvkBackend_0_1_1_0
where Self::WvkVersion : wvk::wvk::WvkEnvironment_0_1_2_0 {}

pub trait DhiWvkBackend_0_1_3_0 : DhiWvkBackend_0_1_2_0
where Self::WvkVersion : wvk::wvk::WvkEnvironment_0_1_3_0 {}

pub trait DhiWvkBackend_0_1_4_0 : DhiWvkBackend_0_1_3_0
where Self::WvkVersion : wvk::wvk::WvkEnvironment_0_1_4_0 {}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl crate::dhi::sealed::Sealed for DHI_WVK_0_1_0_0 {}
impl DhiBackend for DHI_WVK_0_1_0_0 {
    type Backend = DhiWvkContext<Self>;

    fn s_create() -> Result<Self::Backend, DHIError> {
        DhiWvkContext::s_create()
    }
}
impl DhiWvkBackend for DHI_WVK_0_1_0_0 {
    type WvkVersion = wvk::wvk::WVK_0_1_0_0;
}
impl DhiWvkBackend_0_1_0_0 for DHI_WVK_0_1_0_0 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl crate::dhi::sealed::Sealed for DHI_WVK_0_1_1_0 {}
impl DhiBackend for DHI_WVK_0_1_1_0 {
    type Backend = DhiWvkContext<Self>;

    fn s_create() -> Result<Self::Backend, DHIError> {
        DhiWvkContext::s_create()
    }
}
impl DhiWvkBackend for DHI_WVK_0_1_1_0 {
    type WvkVersion = wvk::wvk::WVK_0_1_1_0;
}
impl DhiWvkBackend_0_1_0_0 for DHI_WVK_0_1_1_0 {}
impl DhiWvkBackend_0_1_1_0 for DHI_WVK_0_1_1_0 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl crate::dhi::sealed::Sealed for DHI_WVK_0_1_2_0 {}
impl DhiBackend for DHI_WVK_0_1_2_0 {
    type Backend = DhiWvkContext<Self>;

    fn s_create() -> Result<Self::Backend, DHIError> {
        DhiWvkContext::s_create()
    }
}
impl DhiWvkBackend for DHI_WVK_0_1_2_0 {
    type WvkVersion = wvk::wvk::WVK_0_1_2_0;
}
impl DhiWvkBackend_0_1_0_0 for DHI_WVK_0_1_2_0 {}
impl DhiWvkBackend_0_1_1_0 for DHI_WVK_0_1_2_0 {}
impl DhiWvkBackend_0_1_2_0 for DHI_WVK_0_1_2_0 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl crate::dhi::sealed::Sealed for DHI_WVK_0_1_3_0 {}
impl DhiBackend for DHI_WVK_0_1_3_0 {
    type Backend = DhiWvkContext<Self>;

    fn s_create() -> Result<Self::Backend, DHIError> {
        DhiWvkContext::s_create()
    }
}
impl DhiWvkBackend for DHI_WVK_0_1_3_0 {
    type WvkVersion = wvk::wvk::WVK_0_1_3_0;
}
impl DhiWvkBackend_0_1_0_0 for DHI_WVK_0_1_3_0 {}
impl DhiWvkBackend_0_1_1_0 for DHI_WVK_0_1_3_0 {}
impl DhiWvkBackend_0_1_2_0 for DHI_WVK_0_1_3_0 {}
impl DhiWvkBackend_0_1_3_0 for DHI_WVK_0_1_3_0 {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

impl crate::dhi::sealed::Sealed for DHI_WVK_0_1_4_0 {}
impl DhiBackend for DHI_WVK_0_1_4_0 {
    type Backend = DhiWvkContext<Self>;

    fn s_create() -> Result<Self::Backend, DHIError> {
        DhiWvkContext::s_create()
    }
}
impl DhiWvkBackend for DHI_WVK_0_1_4_0 {
    type WvkVersion = wvk::wvk::WVK_0_1_4_0;
}
impl DhiWvkBackend_0_1_0_0 for DHI_WVK_0_1_4_0 {}
impl DhiWvkBackend_0_1_1_0 for DHI_WVK_0_1_4_0 {}
impl DhiWvkBackend_0_1_2_0 for DHI_WVK_0_1_4_0 {}
impl DhiWvkBackend_0_1_3_0 for DHI_WVK_0_1_4_0 {}
impl DhiWvkBackend_0_1_4_0 for DHI_WVK_0_1_4_0 {}