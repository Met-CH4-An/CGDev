// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use crate::registry_type_body_with_enum::RegistryTypeBodyWithEnum;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// element member {
///                ApiAttr?,
///                attribute len { text }?,
///                attribute altlen { text }?,
///                attribute stride { text }?,
///                attribute externsync { text }?,
///                OptionalAttr?,
///                attribute selector { text }?,
///                attribute selection { VkEnumNameList_t }?,
///                NoAutoValidityAttr?,
///                attribute values { VkEnumNameList_t }?,
///                attribute limittype { text }?,
///                attribute objecttype { text }?,
///                attribute deprecated { text }?,
///                attribute featurelink { text }?,
///                attribute flagsextend { VkTypeNameRef_t }?,
///                attribute flagsextendmember { TypeName_t }?,
///                TypeBodyWithEnum
///            }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct RegistryTypeStructMember {
    /// ApiAttr?,
    pub(crate) api_rng: RangeInclusive<usize>,
    /// attribute len { text }?
    pub(crate) len_rng: RangeInclusive<usize>,
    /// attribute altlen { text }?
    pub(crate) alt_len_rng: RangeInclusive<usize>,
    /// attribute stride { text }?
    pub(crate) stride_rng: RangeInclusive<usize>,
    /// attribute externsync { text }?
    pub(crate) extern_sync_rng: RangeInclusive<usize>,
    /// OptionalAttr?
    pub(crate) optional_attr_rng: RangeInclusive<usize>,
    /// attribute selector { text }?
    pub(crate) selector_rng: RangeInclusive<usize>,
    /// attribute selection { VkEnumNameList_t }?
    pub(crate) selection_rng: RangeInclusive<usize>,
    /// NoAutoValidityAttr?
    pub(crate) no_auto_validity_attr_rng: RangeInclusive<usize>,
    /// attribute values { VkEnumNameList_t }?
    pub(crate) values_rng: RangeInclusive<usize>,
    /// attribute limittype { text }?
    pub(crate) limit_type_rng: RangeInclusive<usize>,
    /// attribute objecttype { text }?
    pub(crate) object_type_rng: RangeInclusive<usize>,
    /// attribute deprecated { text }?
    pub(crate) deprecated_rng: RangeInclusive<usize>,
    /// attribute featurelink { text }?
    pub(crate) feature_link_rng: RangeInclusive<usize>,
    /// attribute flagsextend { VkTypeNameRef_t }?
    pub(crate) flags_extend_rng: RangeInclusive<usize>,
    /// attribute flagsextendmember { TypeName_t }?
    pub(crate) flags_extend_member_rng: RangeInclusive<usize>,
    /// TypeBodyWithEnum
    pub(crate) type_body_with_enum: RegistryTypeBodyWithEnum,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные ассоциированные функции.
// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryTypeStructMember {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные методы.
// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryTypeStructMember {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные ассоциированные функции.
// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryTypeStructMember {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn s_create() -> Self {
        Self {
            api_rng: 1 ..= 0,
            len_rng: 1 ..= 0,
            alt_len_rng: 1 ..= 0,
            stride_rng: 1 ..= 0,
            extern_sync_rng: 1 ..= 0,
            optional_attr_rng: 1 ..= 0,
            selector_rng: 1 ..= 0,
            selection_rng: 1 ..= 0,
            no_auto_validity_attr_rng: 1 ..= 0,
            values_rng: 1 ..= 0,
            limit_type_rng: 1 ..= 0,
            object_type_rng: 1 ..= 0,
            deprecated_rng: 1 ..= 0,
            feature_link_rng: 1 ..= 0,
            flags_extend_rng: 1 ..= 0,
            flags_extend_member_rng: 1 ..= 0,
            type_body_with_enum: RegistryTypeBodyWithEnum::s_create(),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные методы.
// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryTypeStructMember {}