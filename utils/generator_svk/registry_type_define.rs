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
/// TypeDefine =
///     CommonTypeAttributes,
///     attribute category { "define" },
///     NameAttr?,
///     TypeBody
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct RegistryTypeDefine {
    /// CommonTypeAttributes,
    pub(crate) common_type_attributes: RegistryCommonTypeAttributes,
    /// attribute category { "define" },
    pub(crate) category_rng: RangeInclusive<usize>,
    /// NameAttr,
    pub(crate) name_rng: RangeInclusive<usize>,
    /// TypeBody
    pub(crate) type_body: RegistryTypeBody,
}

impl RegistryTypeDefine {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            common_type_attributes: RegistryCommonTypeAttributes::create(),
            category_rng: 1 ..= 0,
            name_rng: 1 ..= 0,
            type_body: RegistryTypeBody::create(),
        }
    }
}