// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use crate::types_item_type_item_base_type::TypeBaseType;
use crate::type_body::TypeBody;

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

impl CommonTypeAttributes {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            api: 1 ..= 0,
            comment: 1 ..= 0,
            requires: 1 ..= 0,
            deprecated: 1 ..= 0,
        }
    }
}
