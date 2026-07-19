// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use std::borrow::Cow;
use std::ffi::CStr;

use crate::dhi_error::DHIError;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// маркеры версий
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub struct DHI_WVK_0_1_0_0;
pub struct DHI_WVK_0_1_1_0;
pub struct DHI_WVK_0_1_2_0;
pub struct DHI_WVK_0_1_3_0;
pub struct DHI_WVK_0_1_4_0;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub trait DhiBackend : sealed::Sealed {
    type Backend;

    fn s_create() -> Result<Self::Backend, DHIError>;
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// приватная область
// private area
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
//
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub(crate) mod sealed {
    pub trait Sealed {}
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Константы.
// Constants.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

pub(crate) const DHI_NAME_STR : &str = "Device Hardware Interface";
pub(crate) const DHI_NAME_CSTR : &CStr = c"Device Hardware Interface";
pub(crate) const DHI_VERSION_ENCODED : u32 = {
    let major_ = parse_u32(env!("CARGO_PKG_VERSION_MAJOR"));
    let minor_ = parse_u32(env!("CARGO_PKG_VERSION_MINOR"));
    let patch_ = parse_u32(env!("CARGO_PKG_VERSION_PATCH"));
    (major_ << 22) | (minor_ << 12) | (patch_)
};

const fn parse_u32(s: &str) -> u32 {
    let bytes = s.as_bytes();
    let mut i = 0;
    let mut value = 0;

    while i < bytes.len() {
        value = value * 10 + (bytes[i] - b'0') as u32;
        i += 1;
    }

    value
}
