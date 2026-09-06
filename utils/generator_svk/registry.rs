// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::ops::RangeInclusive;
use crate::registry_types::RegistryTypes;
use crate::registry_enums::RegistryEnums;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct Registry {
    pub(crate) registry_types_vec: Vec<RegistryTypes>,
    //pub(crate) registry_type_as_bitmask_vec: Vec<(RangeInclusive<usize>, Vec<RegistryTypeAsBitmask>)>,
    pub(crate) registry_enums_as_enum_vec: Vec<RegistryEnums>,
    pub(crate) registry_enums_as_bitmask_vec: Vec<RegistryEnums>,
    pub(crate) requires_cash: HashMap<u64, (usize, usize)>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные ассоциированные функции.
// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Registry {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create() -> Self {
        Self {
            registry_types_vec: Vec::new(),
            //registry_type_as_bitmask_vec: Vec::new(),
            registry_enums_as_enum_vec: Vec::new(),
            registry_enums_as_bitmask_vec: Vec::new(),
            requires_cash: HashMap::new(),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные методы.
// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Registry {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные ассоциированные функции.
// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Registry {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные методы.
// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Registry {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn addRequire(&mut self, requires: &str, x: usize, y: usize) {
        let mut hasher_ = std::hash::DefaultHasher::new();

        requires.hash(&mut hasher_);

        let hash_ = hasher_.finish();
        
        self.requires_cash.insert(hash_, (x, y));
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ////
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /*pub(crate) fn findTypeAsBitmask(&mut self, requires: &str) -> Option<&RegistryTypeAsBitmask> {
        let mut hasher_ = std::hash::DefaultHasher::new();

        requires.hash(&mut hasher_);

        let hash_ = hasher_.finish();
        
        let (x, y)= self.requires_cash.get(&hash_)?;
        
        Some(&self.registry_type_as_bitmask_vec[*x].1[*y])
    }
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    #[inline(always)]
    pub(crate) fn findType(&self, name_str: &str) -> Option<&RegistryType> {
        self.registry_types_vec
            .iter()
            .find_map(|types_| {
                types_.findType(name_str)
            })
    }*/
}