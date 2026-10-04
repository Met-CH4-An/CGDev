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

mod types_item_type_item_common_type_attributes;
mod types_item_type_item_type_body;
mod types_item_type_item_type_body_item_type;
mod types_item_type_item_type_body_item_name;

mod types;
mod types_item_type;

mod types_item_type_item_type_body_with_enum;
mod types_item_type_item_type_body_with_enum_item_type;
mod types_item_type_item_type_body_with_enum_item_name;
mod types_item_type_item_type_body_with_enum_item_enum;


mod types_item_type_item_base_type;
mod types_item_type_item_bitmask;
mod types_item_type_item_define;
mod types_item_type_item_enum;
mod types_item_type_item_func_pointer;
mod types_item_type_item_func_pointer_item_param;
mod types_item_type_item_func_pointer_item_param_item_type;
mod types_item_type_item_func_pointer_item_param_item_name;
mod types_item_type_item_func_pointer_item_proto;
mod types_item_type_item_func_pointer_item_proto_item_type;
mod types_item_type_item_func_pointer_item_proto_item_name;
mod types_item_type_item_handle;
mod types_item_type_item_include;
mod types_item_type_item_requires;
mod types_item_type_item_struct;
mod types_item_type_item_struct_item_member;

mod enums;
mod enums_item_enum;

mod commands;
mod commands_item_command;
mod commands_item_command_item_alias;
mod commands_item_command_item_param;
mod commands_item_command_item_param_item_type;
mod commands_item_command_item_param_item_name;
mod commands_item_command_item_proto;
mod commands_item_command_item_proto_item_name;
mod commands_item_command_item_proto_item_type;
mod commands_item_command_item_description;
mod commands_item_command_item_implicit_extern_sync_params;
mod commands_item_command_item_implicit_extern_sync_params_item_param;

mod extensions;
mod extensions_item_extension;
mod extensions_item_extension_item_require;
mod extensions_item_extension_item_deprecate;
mod extensions_item_extension_item_remove;

mod comment_elt;
mod unused;

mod interface_element;
mod interface_element_item_type;
mod interface_element_item_command;
mod interface_element_item_feature;

mod deprecate_element;
mod deprecate_element_item_feature;
mod deprecate_element_item_command;
mod deprecate_element_item_enum;
mod deprecate_element_item_type;

pub use types::TypesViewVariant;
pub use types_item_type::{TypesItemTypeView, TypesItemTypeViewVariant};
pub use types_item_type_item_struct::{TypesItemTypeItemStructView, TypesItemTypeItemStructViewVariant};

pub(crate) fn makeHash(data: &str) -> u64 {
    let mut hasher_ = std::hash::DefaultHasher::new();

    data.hash(&mut hasher_);

    let hash_ = hasher_.finish();

    hash_
}