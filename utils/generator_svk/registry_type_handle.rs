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
/// TypeHandle =
///     CommonTypeAttributes,
///     attribute category { "handle" },
///     (
///        ( NameAttr,
///          attribute alias { text }
///        )
///      | ( attribute parent { TypeName_t }?,
///          attribute objtypeenum { text },
///          TypeBody
///        )
///     )
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct RegistryTypeHandle {
    /// CommonTypeAttributes,
    pub(crate) common_type_attributes: RegistryCommonTypeAttributes,
    /// attribute category { "define" },
    pub(crate) category_rng: RangeInclusive<usize>,
    /// NameAttr,
    pub(crate) name_rng: RangeInclusive<usize>,
    /// attribute alias { text }?
    pub(crate) alias_rng: RangeInclusive<usize>,
    /// attribute parent { TypeName_t }?
    pub(crate) parent_rng: RangeInclusive<usize>,
    /// attribute objtypeenum { text }
    pub(crate) objtypeenum_rng: RangeInclusive<usize>,
    /// TypeBody
    pub(crate) registry_type_body: RegistryTypeBody,
}

impl RegistryTypeHandle {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            common_type_attributes: RegistryCommonTypeAttributes::create(),
            category_rng: 1 ..= 0,
            name_rng: 1 ..= 0,
            alias_rng: 1 ..= 0,
            parent_rng: 1 ..= 0,
            objtypeenum_rng: 1 ..= 0,
            registry_type_body: RegistryTypeBody::create(),
        }
    }
}