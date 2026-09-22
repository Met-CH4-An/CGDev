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
/// TypeBasetype =
///     CommonTypeAttributes,
///     attribute category { "basetype" },
///     TypeBody
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct RegistryTypeBaseType {
    /// CommonTypeAttributes,
    pub(crate) registry_common_type_attributes: RegistryCommonTypeAttributes,
    /// attribute category { "basetype" },
    pub(crate) category_rng: RangeInclusive<usize>,
    /// TypeBody
    pub(crate) registry_type_body: RegistryTypeBody,

}

impl RegistryTypeBaseType {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            registry_common_type_attributes: RegistryCommonTypeAttributes::create(),
            category_rng: 1 ..= 0,
            registry_type_body: RegistryTypeBody::create(),
        }
    }
}