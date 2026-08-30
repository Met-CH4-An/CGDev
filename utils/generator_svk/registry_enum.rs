// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use crate::registry_enum_enumerator::RegistryEnumEnumerator;
use crate::registry_enum_enumerator_extended::RegistryEnumEnumeratorExtended;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// <enums name="name" type="type" comment="comment">       <- RegistryEnum
///     <enum type="type" value="value" name="name" />      <- RegistryEnumEnumerant
/// </enums>                                                <- RegistryEnum
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct RegistryEnum {
    /// name
    pub(crate) name_rng: RangeInclusive<usize>,
    /// type
    pub(crate) type_rng: RangeInclusive<usize>,
    /// comment
    pub(crate) comment_rng: RangeInclusive<usize>,
    /// enumerants
    pub(crate) enumerators: Vec<RegistryEnumEnumerator>,
    /// enumerants extended
    pub(crate) extended_enumerators: Vec<RegistryEnumEnumeratorExtended>,
    
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные ассоциированные функции.
// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryEnum {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные методы.
// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryEnum {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные ассоциированные функции.
// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryEnum {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn s_create() -> Self {
        Self {
            name_rng: 1 ..= 0,
            type_rng: 1 ..= 0,
            comment_rng: 1 ..= 0,
            enumerators: Vec::new(),
            extended_enumerators: Vec::new(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn s_createWithData(name_rng: RangeInclusive<usize>, type_rng: RangeInclusive<usize>, comment_rng: RangeInclusive<usize>, enumerators: Vec<RegistryEnumEnumerator>, extended_enumerators: Vec<RegistryEnumEnumeratorExtended>) -> Self {
        Self {
            name_rng: name_rng,
            type_rng: type_rng,
            comment_rng: comment_rng,
            enumerators: enumerators,
            extended_enumerators: extended_enumerators,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные методы.
// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryEnum {}