// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// example:
/// pub const NAME: type = value;
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct BindingEnum {
    /// value
    pub(in crate::parser) value_rng: RangeInclusive<usize>,
    /// NAME
    pub(in crate::parser) name_rng: RangeInclusive<usize>,
    /// comment
    pub(in crate::parser) comment_rng: RangeInclusive<usize>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl BindingEnum {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Публичные ассоциированные функции.
    // Public associated functions.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::parser) fn s_createEmpty() -> Self {
        Self {
            value_rng: 1 ..= 0,
            name_rng: 1 ..= 0,
            comment_rng: 1 ..= 0,
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(in crate::parser) fn s_create(value_rng: RangeInclusive<usize>, name_rng: RangeInclusive<usize>, comment_rng: RangeInclusive<usize>) -> Self {
        Self {
            value_rng: value_rng,
            name_rng: name_rng,
            comment_rng: comment_rng,
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Публичные методы.
    // Public methods.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub unsafe fn appendsString(&self, data: &[u8], output: &mut String) {
        let value_str_ = std::str::from_utf8_unchecked(&data[*self.value_rng.start() ..= *self.value_rng.end()]);
        let name_str_ = std::str::from_utf8_unchecked(&data[*self.name_rng.start() ..= *self.name_rng.end()]);
        let comment_str_ = std::str::from_utf8_unchecked(&data[*self.comment_rng.start() ..= *self.comment_rng.end()]);

        let output_str_ = &format!("pub const {}: = {}; // {}\n", name_str_, value_str_, comment_str_);

        output.push_str(output_str_);
    }
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Приватные ассоциированные функции.
    // Private associated functions.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Приватные методы.
    // Private methods.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
}