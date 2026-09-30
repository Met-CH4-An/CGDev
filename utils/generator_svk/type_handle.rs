// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
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
pub(crate) struct TypeHandle {
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