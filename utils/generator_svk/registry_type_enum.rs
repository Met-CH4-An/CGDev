// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use crate::registry_common_type_attributes::RegistryCommonTypeAttributes;
use crate::registry_type_body::RegistryTypeBody;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// TypeEnum =
///     CommonTypeAttributes,
///     attribute category { "enum" },
///     NameAttr,
///     attribute alias { text }?
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct RegistryTypeEnum {
    /// CommonTypeAttributes,
    pub(crate) common_type_attributes: RegistryCommonTypeAttributes,
    /// attribute category { "define" },
    pub(crate) category_rng: RangeInclusive<usize>,
    /// NameAttr,
    pub(crate) name_rng: RangeInclusive<usize>,
    /// attribute alias { text }?
    pub(crate) alias_rng: RangeInclusive<usize>,
}

impl RegistryTypeEnum {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            common_type_attributes: RegistryCommonTypeAttributes::create(),
            category_rng: 1 ..= 0,
            name_rng: 1 ..= 0,
            alias_rng: 1 ..= 0,
        }
    }
}