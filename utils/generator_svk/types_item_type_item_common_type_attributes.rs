// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct TypesItemCommonTypeAttributesView<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) content: &'a TypesItemTypeItemCommonTypeAttributes,
}

impl<'a> TypesItemCommonTypeAttributesView<'a> {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn api(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.api.start() ..= *self.content.api.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn comment(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.comment.start() ..= *self.content.comment.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn requires(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.requires.start() ..= *self.content.requires.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn deprecated(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.deprecated.start() ..= *self.content.deprecated.end()]) }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// CommonTypeAttributes =
///     ApiAttr?,
///     CommentAttr?,
///     attribute requires { text }?,
///     attribute deprecated { "unused" | "true" }?
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct TypesItemTypeItemCommonTypeAttributes {
    /// ApiAttr?
    pub(crate) api: RangeInclusive<usize>,
    /// CommentAttr?
    pub(crate) comment: RangeInclusive<usize>,
    /// attribute requires { text }?
    pub(crate) requires: RangeInclusive<usize>,
    /// attribute deprecated { "unused" | "true" }?
    pub(crate) deprecated: RangeInclusive<usize>,
}

impl TypesItemTypeItemCommonTypeAttributes {
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
