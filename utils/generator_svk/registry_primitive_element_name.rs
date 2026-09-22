// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// element name { attribute alias { text }?, TypeName_t }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct RegistryPrimitiveElementName {
    /// attribute alias { text }
    pub(crate) alias_rng: RangeInclusive<usize>,
    ///
    pub(crate) prefix_rng: RangeInclusive<usize>,
    /// TypeName_t
    pub(crate) name_rng: RangeInclusive<usize>,
    ///
    pub(crate) postfix_rng: RangeInclusive<usize>,
}

impl RegistryPrimitiveElementName {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            alias_rng: 1 ..= 0,
            prefix_rng: 1 ..= 0,
            name_rng: 1 ..= 0,
            postfix_rng: 1 ..= 0,
        }
    }
}