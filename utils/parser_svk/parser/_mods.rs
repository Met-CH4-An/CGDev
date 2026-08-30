// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// файл parser.rs
// file parser.rs
mod parser;

use std::hash::{Hash, Hasher};
use std::ops::RangeInclusive;
pub use parser::Parser;

// файл binding
// file binding
mod binding;

// файл binding_type
// file binding_type
mod binding_type;

// файл binding_types
// file binding_types
mod binding_types;

// файл binding_bitmask
// file binding_bitmask
mod binding_bitmask;

// файл binding_bitmasks
// file binding_bitmasks
mod binding_bitmasks;

// файл binding_constant
// file binding_constant
mod binding_constant;

// файл binding_constants
// file binding_constants
mod binding_constants;

// файл binding_enum
// file binding_enum
mod binding_enum;

// файл binding_enum_extends
// file binding_enum_extends
mod binding_enum_extends;

// файл binding_enums
// file binding_enums
mod binding_enums;





// файл svk_enums
// file svk_enums
mod vulkan_registry_enums;

// файл vulkan_registry_enums_enum
// file vulkan_registry_enums_enum
mod vulkan_registry_enums_enum;

// файл vulkan_registry_enums_enum_ex
// file vulkan_registry_enums_enum_ex
mod vulkan_registry_enums_enum_ex;

// файл vulkan_feature_item.rs
// file vulkan_feature_item.rs
mod vulkan_feature_item;

// файл vulkan_registry.rs
// file vulkan_registry.rs
mod vulkan_registry;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub fn makeHash(data: &[u8], data_rng: RangeInclusive<usize>) -> u64 {
    let mut hasher_ = std::hash::DefaultHasher::new();

    // Делаем хеш имени.
    // Make a hash of the name.
    let value_str_ = unsafe { std::str::from_utf8_unchecked(&data[*data_rng.start() ..= *data_rng.end()]) };

    value_str_.hash(&mut hasher_);

    hasher_.finish()
}

