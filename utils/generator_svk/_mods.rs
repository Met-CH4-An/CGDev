// SPDX-License-Identifier: None
// Copyright (c) 2026 None

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::hash::{Hash, Hasher};
use utils__tokenizer_xml;

// файл generator.rs
// file generator.rs
mod generator;
pub use generator::Generator;

// файл registry.rs
// file registry.rs
mod registry;

// файл registry_types.rs
// file registry_types.rs
mod registry_types;

// файл registry_type.rs
// file registry_type.rs
mod registry_type;

// файл registry_common_type_attributes.rs
// file registry_common_type_attributes.rs
mod registry_common_type_attributes;

// файл registry_type_body.rs
// file registry_type_body.rs
mod registry_type_body;

// файл registry_type_base_type.rs
// file registry_type_base_type.rs
mod registry_type_base_type;

// файл registry_type_bitmask.rs
// file registry_type_bitmask.rs
mod registry_type_bitmask;

// файл registry_type_define.rs
// file registry_type_define.rs
mod registry_type_define;

// файл registry_type_enum.rs
// file registry_type_enum.rs
mod registry_type_enum;

// файл registry_type_funcpointer.rs
// file registry_type_funcpointer.rs
mod registry_type_funcpointer;

// файл registry_type_handle.rs
// file registry_type_handle.rs
mod registry_type_handle;

// файл registry_type_include.rs
// file registry_type_include.rs
mod registry_type_include;

// файл registry_type_requires.rs
// file registry_type_requires.rs
mod registry_type_requires;

// файл registry_type_struct.rs
// file registry_type_struct.rs
mod registry_type_struct;

// файл registry_enums
// file registry_enums
mod registry_enums;

// файл registry_enum.rs
// file registry_enum.rs
mod registry_enum;

pub(crate) fn makeHash(data: &str) -> u64 {
    let mut hasher_ = std::hash::DefaultHasher::new();

    data.hash(&mut hasher_);

    let hash_ = hasher_.finish();

    hash_
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные ассоциированные функции.
// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные методы.
// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные ассоциированные функции.
// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные методы.
// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~