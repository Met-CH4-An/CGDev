// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// CommonTypeAttributes =
///     ApiAttr?,
///     CommentAttr?,
///     attribute requires { text }?,
///     attribute deprecated { "unused" | "true" }?
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct CommonTypeAttributes {
    /// ApiAttr?,
    pub(crate) api: RangeInclusive<usize>,
    /// CommentAttr?,
    pub(crate) comment: RangeInclusive<usize>,
    /// attribute requires { text }?,
    pub(crate) requires: RangeInclusive<usize>,
    /// attribute deprecated { "unused" | "true" }?
    pub(crate) deprecated: RangeInclusive<usize>,
}