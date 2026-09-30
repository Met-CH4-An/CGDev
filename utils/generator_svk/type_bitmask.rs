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
/// TypeBitmask =
///     CommonTypeAttributes,
///     attribute category { "bitmask" },
///     (
///        ( NameAttr,
///          attribute alias { text }
///        )
///      | ( NameAttr?,
///          attribute bitvalues { text }?,
///          TypeBody
///        )
///     )
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct TypeBitmask {
    /// CommonTypeAttributes,
    pub(crate) common_type_attributes: CommonTypeAttributes,
    /// attribute category { "bitmask" },
    pub(crate) category: RangeInclusive<usize>,
    /// NameAttr,
    pub(crate) name: RangeInclusive<usize>,
    /// attribute alias { text }
    pub(crate) alias: RangeInclusive<usize>,
    /// attribute bitvalues { text }?,
    pub(crate) bit_values: RangeInclusive<usize>,
    /// TypeBody
    pub(crate) type_body: TypeBody,
}