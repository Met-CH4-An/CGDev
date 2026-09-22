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
pub(crate) struct RegistryTypeRequires {
    /// ApiAttr?,
    pub(crate) api_rng: RangeInclusive<usize>,
    /// CommentAttr?,
    pub(crate) comment_rng: RangeInclusive<usize>,
    /// attribute deprecated { "unused" | "true" }?
    pub(crate) deprecated_rng: RangeInclusive<usize>,
    /// attribute name { TypeName_t },
    pub(crate) name_rng: RangeInclusive<usize>,
    /// attribute requires { text }
    pub(crate) requires_rng: RangeInclusive<usize>,
}

impl RegistryTypeRequires {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            api_rng: 1 ..= 0,
            comment_rng: 1 ..= 0,
            deprecated_rng: 1 ..= 0,
            name_rng: 1 ..= 0,
            requires_rng: 1 ..= 0,
        }
    }
}