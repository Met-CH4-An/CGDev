// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// example:
/// pub type VkNameEnums = VkFlag;
/// pub mod VkNameEnumsValue {
///     pub const NAME: VkNameEnums = bitpos; // comment
/// }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct BindingBitmask {
    /// bitpos
    pub(in crate::parser) bitpos_rng: RangeInclusive<usize>,
    /// NAME
    pub(in crate::parser) name_rng: RangeInclusive<usize>,
    /// comment
    pub(in crate::parser) comment_rng: RangeInclusive<usize>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные ассоциированные функции.
// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl BindingBitmask {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор пустого объекта.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::parser) fn s_createEmpty() -> Self {
        Self {
            bitpos_rng: 1..=0,
            name_rng: 1..=0,
            comment_rng: 1..=0,
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::parser) fn s_create(bitpos_rng: RangeInclusive<usize>, name_rng: RangeInclusive<usize>, comment_rng: RangeInclusive<usize>) -> Self {
        Self {
            bitpos_rng: bitpos_rng,
            name_rng: name_rng,
            comment_rng: comment_rng,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные методы.
// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl BindingBitmask {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Печатает в выходную строку.
    // Prints to the output string.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub unsafe fn print(&self, type_rng: RangeInclusive<usize>, data: &[u8], output: &mut String) {
        let type_str_ = std::str::from_utf8_unchecked(&data[*type_rng.start() ..= *type_rng.end()]);
        let bitpos_str_ = std::str::from_utf8_unchecked(&data[*self.bitpos_rng.start() ..= *self.bitpos_rng.end()]);
        let name_str_ = std::str::from_utf8_unchecked(&data[*self.name_rng.start() ..= *self.name_rng.end()]);
        let comment_str_ = std::str::from_utf8_unchecked(&data[*self.comment_rng.start() ..= *self.comment_rng.end()]);

        let output_str_ = &format!("pub const {}: {} = {}; // {}\n", name_str_, type_str_, bitpos_str_, comment_str_);

        output.push_str(output_str_);
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