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
pub(crate) struct RegistryCommonTypeAttributes {
    /// ApiAttr?,
    pub(crate) api_rng: RangeInclusive<usize>,
    /// CommentAttr?,
    pub(crate) comment_rng: RangeInclusive<usize>,
    /// attribute requires { text }?,
    pub(crate) requires_rng: RangeInclusive<usize>,
    /// attribute deprecated { "unused" | "true" }?
    pub(crate) deprecated_rng: RangeInclusive<usize>,
}

impl RegistryCommonTypeAttributes {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            api_rng: 1 ..= 0,
            comment_rng: 1 ..= 0,
            requires_rng: 1 ..= 0,
            deprecated_rng: 1 ..= 0,
        }
    }
}