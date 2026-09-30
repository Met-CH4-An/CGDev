// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use crate::common_type_attributes::CommonTypeAttributes;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// TypeFuncpointer =
///     CommonTypeAttributes,
///     attribute category { "funcpointer" },
///     element proto {
///         mixed {
///             element type { TypeName_t }?,
///             element name { text }
///         }
///     },
///     element param {
///         ApiAttr?,
///         attribute len { text }?,
///         attribute altlen { text }?,
///         attribute stride { text }?,
///         attribute externsync { text }?,
///         OptionalAttr?,
///         attribute selector { text }?,
///         NoAutoValidityAttr?,
///         attribute objecttype { text }?,
///         attribute validstructs { VkTypeNameListRef_t }?,
///         mixed {
///             element type { TypeName_t }?,
///             element name { text }?
///         }
///     }*
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct TypeFuncPointer {
    /// CommonTypeAttributes,
    pub(crate) common_type_attributes: CommonTypeAttributes,
    /// attribute category { "bitmask" },
    pub(crate) category: RangeInclusive<usize>,
}