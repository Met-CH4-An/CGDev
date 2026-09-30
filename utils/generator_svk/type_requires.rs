// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// TypeRequires =
///     ApiAttr?,
///     CommentAttr?,
///     attribute deprecated { "unused" | "true" }?,
///     attribute name { TypeName_t },
///     attribute requires { text }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct TypeRequires {
    /// ApiAttr?,
    pub(crate) api: RangeInclusive<usize>,
    /// CommentAttr?,
    pub(crate) comment: RangeInclusive<usize>,
    /// attribute deprecated { "unused" | "true" }?
    pub(crate) deprecated: RangeInclusive<usize>,
    /// attribute name { TypeName_t },
    pub(crate) name: RangeInclusive<usize>,
    /// attribute requires { text }
    pub(crate) requires: RangeInclusive<usize>,
}