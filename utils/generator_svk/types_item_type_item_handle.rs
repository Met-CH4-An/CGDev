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
    pub(crate) common_type_attributes: CommonTypeAttributes,
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
    pub(crate) type_body: TypeBody,
}

impl TypesItemTypeItemHandle {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            common_type_attributes: CommonTypeAttributes::create(),
            category: 1 ..= 0,
            name: 1 ..= 0,
            alias: 1 ..= 0,
            parent: 1 ..= 0,
            obj_type_enum: 1 ..= 0,
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

        Ok(())
    }
}