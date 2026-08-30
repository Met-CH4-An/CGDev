// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// <extensions comment="comment">
///     <extension name="name" number="number" type="type" author="author" contact="contact" supported="supported" ratified="ratified" nofeatures="nofeatures">
///         <require>
///             <enum value="value" name="name" />
///             <enum offset="offset" extends="extends" dir="dir" name="name" />
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct RegistryEnumEnumeratorExtended {
    /// extnumber
    pub(crate) extnumber_rng: RangeInclusive<usize>,
    /// offset
    pub(crate) offset_rng: RangeInclusive<usize>,
    /// extends
    pub(crate) extends_rng: RangeInclusive<usize>,
    /// dir
    pub(crate) dir_rng: RangeInclusive<usize>,
    /// bitpos
    pub(crate) bitpos_rng: RangeInclusive<usize>,
    /// name
    pub(crate) name_rng: RangeInclusive<usize>,
    /// comment
    pub(crate) comment_rng: RangeInclusive<usize>,
    /// value
    pub(crate) value_rng: RangeInclusive<usize>,
    /// alias
    pub(crate) alias_rng: RangeInclusive<usize>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные ассоциированные функции.
// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryEnumEnumeratorExtended {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные методы.
// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryEnumEnumeratorExtended {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные ассоциированные функции.
// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryEnumEnumeratorExtended {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn s_create() -> Self {
        Self {
            extnumber_rng: 1 ..= 0,
            offset_rng: 1 ..= 0,
            extends_rng: 1 ..= 0,
            dir_rng: 1 ..= 0,
            bitpos_rng: 1 ..= 0,
            name_rng: 1 ..= 0,
            comment_rng: 1 ..= 0,
            value_rng: 1 ..= 0,
            alias_rng: 1 ..= 0,
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn s_createWithData(extnumber_rng: RangeInclusive<usize>, 
                                   offset_rng: RangeInclusive<usize>,
                                   extends_rng: RangeInclusive<usize>,
                                   dir_rng: RangeInclusive<usize>,
                                   bitpos_rng: RangeInclusive<usize>,
                                   name_rng: RangeInclusive<usize>,
                                   comment_rng: RangeInclusive<usize>,
                                   value_rng: RangeInclusive<usize>,
                                   alias_rng: RangeInclusive<usize>) -> Self {
        Self {
            extnumber_rng: extnumber_rng,
            offset_rng: offset_rng,
            extends_rng: extends_rng,
            dir_rng: dir_rng,
            bitpos_rng: bitpos_rng,
            name_rng: name_rng,
            comment_rng: comment_rng,
            value_rng: value_rng,
            alias_rng: alias_rng,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные методы.
// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryEnumEnumeratorExtended {}