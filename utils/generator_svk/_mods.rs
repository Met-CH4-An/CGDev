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

mod registry;
pub use registry::Registry;


mod registry_primitive_element_type;
mod registry_primitive_element_name;
mod registry_primitive_element_enum;

mod types;

mod common_type_attributes;

mod type_body;
mod type_body_type;
mod type_body_name;

mod type_body_with_enum;
mod type_body_with_enum_type;
mod type_body_with_enum_name;
mod type_body_with_enum_enum;

mod r#type;
mod type_base_type;
mod type_bitmask;
mod type_define;
mod type_enum;
mod type_func_pointer;
mod type_handle;
mod type_include;
mod type_requires;
mod type_struct;
mod registry_type_struct_member;

mod registry_enums;
mod registry_enum;

mod registry_extensions;
mod registry_extension;

mod comment_elt;
mod unused;

pub(crate) fn makeHash(data: &str) -> u64 {
    let mut hasher_ = std::hash::DefaultHasher::new();

    data.hash(&mut hasher_);

    let hash_ = hasher_.finish();

    hash_
}