// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use utils__tokenizer_xml::{Tokenizer, AVX2};
use crate::common_type_attributes::CommonTypeAttributes;
use crate::type_body::TypeBody;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// TypeDefine =
///     CommonTypeAttributes,
///     attribute category { "define" },
///     NameAttr?,
///     TypeBody
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct TypesItemTypeItemDefine {
    /// CommonTypeAttributes,
    pub(crate) common_type_attributes: CommonTypeAttributes,
    /// attribute category { "define" },
    pub(crate) category: RangeInclusive<usize>,
    /// NameAttr,
    pub(crate) name: RangeInclusive<usize>,
    /// TypeBody
    pub(crate) type_body: TypeBody,
}

impl TypesItemTypeItemDefine {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            common_type_attributes: CommonTypeAttributes::create(),
            category: 1 ..= 0,
            name: 1 ..= 0,
            type_body: TypeBody::create(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn parse(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8], is_body: bool) -> Result<(), String> {
        if is_body {
            self.type_body.parse(tokenizer, data)?;
        } // if is_body_ {

        else {
            return Err(String::from("Не валидный формат vk.xml. Invalid vk.xml format."));
        }

        Ok(())
    }
}