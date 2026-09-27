// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use crate::registry_primitive_comment_elt::RegistryPrimitiveCommentElt;
use crate::registry_type::RegistryType;

pub(crate) enum RegistryTypesElementVariant {
    TYPE(RegistryType),
    COMMENT_ELT(RegistryPrimitiveCommentElt),
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Types = element types { CommentAttr?, (Type | CommentElt)* }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct RegistryTypes {
    /// CommentAttr?,
    pub(crate) comment_rng: RangeInclusive<usize>,
    /// (Type | CommentElt)*
    pub(crate) registry_types_element_variant_vec: Vec<RegistryTypesElementVariant>,
}

impl RegistryTypes {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            comment_rng: 1 ..= 0,
            registry_types_element_variant_vec: Vec::new(),
        }
    }
}