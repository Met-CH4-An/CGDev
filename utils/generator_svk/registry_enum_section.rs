// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::collections::HashMap;
use std::ops::RangeInclusive;
use crate::makeHash;
use crate::registry_enum::RegistryEnum;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// <- RegistryEnumSection
/// <enums name="name" type="type" comment="comment">       <- RegistryEnum
///     <enum type="type" value="value" name="name" />      <- RegistryEnumEnumerant
/// </enums>                                                <- RegistryEnum
/// <enums name="name" type="type" comment="comment">       <- RegistryEnum
///     <enum type="type" value="value" name="name" />      <- RegistryEnumEnumerant
/// </enums>                                                <- RegistryEnum
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct RegistryEnumSection {
    /// comment
    pub(crate) comment_rng: RangeInclusive<usize>,
    ///
    enums: Vec<RegistryEnum>,
    ///
    indices: HashMap<u64, usize>
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные ассоциированные функции.
// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryEnumSection {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные методы.
// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryEnumSection {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные ассоциированные функции.
// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryEnumSection {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn s_create() -> Self {
        Self {
            comment_rng: 1 ..= 0,
            enums: Vec::new(),
            indices: HashMap::new(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn s_createWithData(comment_rng: RangeInclusive<usize>, enums: Vec<RegistryEnum>, indices: HashMap<u64, usize>) -> Self {
        Self {
            comment_rng: comment_rng,
            enums: enums,
            indices: indices,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные методы.
// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryEnumSection {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn pushEnum(&mut self, name_str: &str, registry_enum: RegistryEnum) {
        let hash_ = makeHash(name_str);
        let index_ = self.enums.len();

        self.enums.push(registry_enum);

        self.indices.insert(hash_, index_);
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Перебирает перечисления и ищет нужный.
    /// Iterates through the enumerations and searches for the required one.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn findEnum(&self, name_str: &str) -> Option<&RegistryEnum> {
        let hash_ = makeHash(name_str);
        
        let index_ = *self.indices.get(&hash_)?;
        
        Some(&self.enums[index_])
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Перебирает перечисления и ищет нужный.
    /// Iterates through the enumerations and searches for the required one.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn findEnumMut(&mut self, name_str: &str) -> Option<&mut RegistryEnum> {
        let hash_ = makeHash(name_str);

        let index_ = *self.indices.get(&hash_)?;

        Some(&mut self.enums[index_])
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn iterEnums(&self) -> impl Iterator<Item = &RegistryEnum> {
        self.enums.iter()
    }
}