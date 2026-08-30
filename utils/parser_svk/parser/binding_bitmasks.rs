// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use crate::parser::binding::Binding;
use crate::parser::binding_bitmask::BindingBitmask;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// example:
/// pub type VkNameEnums = VkFlag; // comment
/// pub mod VkNameEnumsValue {
///     use crate::VkNameEnums;
///     pub const NAME: VkNameEnums = bitpos; // comment
/// }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct BindingBitmasks {
    /// VkNameEnums
    pub(in crate::parser) name_rng: RangeInclusive<usize>,
    /// comment
    pub(in crate::parser) comment_rng: RangeInclusive<usize>,
    ///
    pub(in crate::parser) values: Vec<BindingBitmask>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные ассоциированные функции.
// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl BindingBitmasks {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::parser) fn s_createEmpty() -> Self {
        Self {
            name_rng: 1 ..= 0,
            comment_rng: 1 ..= 0,
            values: Vec::<BindingBitmask>::new(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::parser) fn s_create(name_rng: RangeInclusive<usize>, comment_rng: RangeInclusive<usize>,values: Vec<BindingBitmask>) -> Self {
        Self {
            name_rng: name_rng,
            comment_rng: comment_rng,
            values: values,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные методы.
// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl BindingBitmasks {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub unsafe fn print(&self, binding: &Binding, data: &[u8], output: &mut String) {
        //let type_str_ = std::str::from_utf8_unchecked(&data[*self.bitpos_rng.start() ..= *self.bitpos_rng.end()]);
        let name_str_ = std::str::from_utf8_unchecked(&data[*self.name_rng.start() ..= *self.name_rng.end()]);
        let comment_str_ = std::str::from_utf8_unchecked(&data[*self.comment_rng.start() ..= *self.comment_rng.end()]);

        //let  hash_ = crate::parser::makeHash(data, self.name_rng);
        //binding.types_hmap.get(hash_);

        output.push_str(&format!("pub type {} = VkFlag; // {}\n", name_str_, comment_str_));
        output.push_str(&format!("pub mode {}Value {{\n", name_str_));
        output.push_str(&format!("\t use crate::{};\n", name_str_));

        for binding_bitmask_ in &self.values {
        //    binding_bitmask_.print(data, output);
        }

        output.push_str(&format!("}}"));
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные ассоциированные функции.
// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl BindingBitmask {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные методы.
// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl BindingBitmask {}