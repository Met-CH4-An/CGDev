// SPDX-License-Identifier: None
// Copyright (c) 2026 None

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости крейта
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
use common;
pub use svk;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// подключение модулей
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[path = "wvk_error.rs"]
mod wvk_error;
pub use wvk_error::*;

mod wvk_library;
pub use wvk_library::*;

