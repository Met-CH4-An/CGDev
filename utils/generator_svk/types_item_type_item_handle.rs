// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use utils__tokenizer_xml::{Tokenizer, AVX2};
use crate::types_item_type_item_common_type_attributes::{TypesItemTypeItemCommonTypeAttributes, TypesItemCommonTypeAttributesView};
use crate::types_item_type_item_type_body::{TypesItemTypeItemTypeBody, TypesItemTypeItemTypeBodyView};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct TypesItemTypeItemHandleView<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) content: &'a TypesItemTypeItemHandle,
}

impl<'a> TypesItemTypeItemHandleView<'a> {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn commonTypeAttributes(&self) -> TypesItemCommonTypeAttributesView {
        TypesItemCommonTypeAttributesView {
            data: self.data,
            content: &self.content.common_type_attributes,
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn category(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.category.start() ..= *self.content.category.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn name(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.name.start() ..= *self.content.name.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn alias(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.alias.start() ..= *self.content.alias.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn parent(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.parent.start() ..= *self.content.parent.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn objTypeEnum(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.obj_type_enum.start() ..= *self.content.obj_type_enum.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn typeBody(&self) -> TypesItemTypeItemTypeBodyView {
        TypesItemTypeItemTypeBodyView {
            data: self.data,
            content: &self.content.type_body,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// TypeHandle =
///     CommonTypeAttributes,
///     attribute category { "handle" },
///     (
///        ( NameAttr,
///          attribute alias { text }
///        )
///      | ( attribute parent { TypeName_t }?,
///          attribute objtypeenum { text },
///          TypeBody
///        )
///     )
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct TypesItemTypeItemHandle {
    /// CommonTypeAttributes,
    pub(crate) common_type_attributes: TypesItemTypeItemCommonTypeAttributes,
    /// attribute category { "define" },
    pub(crate) category: RangeInclusive<usize>,
    /// NameAttr,
    pub(crate) name: RangeInclusive<usize>,
    /// attribute alias { text }?
    pub(crate) alias: RangeInclusive<usize>,
    /// attribute parent { TypeName_t }?
    pub(crate) parent: RangeInclusive<usize>,
    /// attribute objtypeenum { text }
    pub(crate) obj_type_enum: RangeInclusive<usize>,
    /// TypeBody
    pub(crate) type_body: TypesItemTypeItemTypeBody,
}

impl TypesItemTypeItemHandle {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            common_type_attributes: TypesItemTypeItemCommonTypeAttributes::create(),
            category: 1 ..= 0,
            name: 1 ..= 0,
            alias: 1 ..= 0,
            parent: 1 ..= 0,
            obj_type_enum: 1 ..= 0,
            type_body: TypesItemTypeItemTypeBody::create(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn parse(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8], is_body: bool) -> Result<(), String> {
        if is_body {
            self.type_body.parse(tokenizer, data)?;
        } // if is_body_ {

        Ok(())
    }
}