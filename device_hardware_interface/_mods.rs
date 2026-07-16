// SPDX-License-Identifier: None
// Copyright (c) 2026 None

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости крейта
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub use common;
pub use wvk;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub mod dhi;

pub(crate) mod dhi_error;
pub use dhi_error::*;

pub(crate) mod dhi_context;
pub use dhi_context::*;

#[path ="backend/_mods.rs"]
pub(crate) mod backend;