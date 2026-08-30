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

// файл registry_type_section.rs
// file registry_type_section.rs
mod registry_type_section;

// файл registry_type_subsection.rs
// file registry_type_subsection.rs
mod registry_type_subsection;

// файл registry_type.rs
// file registry_type.rs
mod registry_type;

// файл registry_enum_section.rs
// file registry_enum_section.rs
mod registry_enum_section;

// файл registry_enum.rs
// file registry_enum.rs
mod registry_enum;

// файл registry_enum_enumerator.rs
// file registry_enum_enumerator.rs
mod registry_enum_enumerator;

// файл registry_enum_enumerator_extended.rs
// file registry_enum_enumerator_extended.rs
mod registry_enum_enumerator_extended;

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