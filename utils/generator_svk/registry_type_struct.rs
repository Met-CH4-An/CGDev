// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use crate::registry_common_type_attributes::RegistryCommonTypeAttributes;
use crate::registry_type_struct_member::RegistryTypeStructMember;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// TypeStruct =
///     CommonTypeAttributes,
///     attribute category { "struct" | "union" },
///     (
///        ( NameAttr,
///          attribute alias { text }
///        )
///      | ( attribute name { TypeName_t },
///          attribute returnedonly { "true" }?,
///          attribute structextends { StringList_t }?,
///          attribute allowduplicate { "true" | "false" }?,
///          attribute requiredlimittype { "true" }?,
///          (
///            element member {
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
///            | CommentElt
///          )*
///        )
///     )
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct RegistryTypeStruct {
    /// CommonTypeAttributes,
    pub(crate) common_type_attributes: RegistryCommonTypeAttributes,
    /// attribute category { "struct" | "union" },
    pub(crate) category_rng: RangeInclusive<usize>,
    /// NameAttr,
    pub(crate) name_rng: RangeInclusive<usize>,
    /// attribute alias { text }?
    pub(crate) alias_rng: RangeInclusive<usize>,
    /// attribute returnedonly { "true" }?
    pub(crate) returned_only_rng: RangeInclusive<usize>,
    /// attribute structextends { StringList_t }?
    pub(crate) struct_extends_rng: RangeInclusive<usize>,
    /// attribute allowduplicate { "true" | "false" }?
    pub(crate) allow_duplicate_rng: RangeInclusive<usize>,
    /// attribute requiredlimittype { "true" }?
    pub(crate) required_limit_type_rng: RangeInclusive<usize>,
    ///
    pub(crate) member_vec: Vec<RegistryTypeStructMember>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные ассоциированные функции.
// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryTypeStruct {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные методы.
// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryTypeStruct {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные ассоциированные функции.
// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryTypeStruct {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn s_create() -> Self {
        Self {
            common_type_attributes: RegistryCommonTypeAttributes::s_create(),
            category_rng: 1 ..= 0,
            name_rng: 1 ..= 0,
            alias_rng: 1 ..= 0,
            returned_only_rng: 1 ..= 0,
            struct_extends_rng: 1 ..= 0,
            allow_duplicate_rng: 1 ..= 0,
            required_limit_type_rng: 1 ..= 0,
            member_vec: Vec::new(),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные методы.
// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryTypeStruct {}