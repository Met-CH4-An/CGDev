// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use utils__tokenizer_xml::{Tokenizer, AVX2};
use crate::common_type_attributes::CommonTypeAttributes;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// TypeEnum =
///     CommonTypeAttributes,
///     attribute category { "enum" },
///     NameAttr,
///     attribute alias { text }?
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct TypesItemTypeItemEnum {
    /// CommonTypeAttributes,
    pub(crate) common_type_attributes: CommonTypeAttributes,
    /// attribute category { "define" },
    pub(crate) category: RangeInclusive<usize>,
    /// NameAttr,
    pub(crate) name: RangeInclusive<usize>,
    /// attribute alias { text }?
    pub(crate) alias: RangeInclusive<usize>,
}

impl TypesItemTypeItemEnum {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            common_type_attributes: CommonTypeAttributes::create(),
            category: 1 ..= 0,
            name: 1 ..= 0,
            alias: 1 ..= 0,
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn parse(&mut self, _tokenizer: &mut Tokenizer<AVX2>, _data: &[u8], is_body: bool) -> Result<(), String> {
        if is_body {
            return Err(String::from("Не валидный формат vk.xml. Invalid vk.xml format."));
        } // if is_body_ {

        Ok(())
    }
}