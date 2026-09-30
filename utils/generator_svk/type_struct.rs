// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use crate::common_type_attributes::CommonTypeAttributes;
use crate::comment_elt::CommentElt;
use crate::registry_type_struct_member::RegistryTypeStructMember;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Конструктор.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) enum RegistryTypeStructElementVariant {
    MEMBER(RegistryTypeStructMember),
    COMMENT_ELT(CommentElt)
}

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
pub(crate) struct TypeStruct {
    /// CommonTypeAttributes,
    pub(crate) common_type_attributes: CommonTypeAttributes,
    /// attribute category { "struct" | "union" },
    pub(crate) category: RangeInclusive<usize>,
    /// NameAttr,
    pub(crate) name: RangeInclusive<usize>,
    /// attribute alias { text }?
    pub(crate) alias: RangeInclusive<usize>,
    /// attribute returnedonly { "true" }?
    pub(crate) returned_only: RangeInclusive<usize>,
    /// attribute structextends { StringList_t }?
    pub(crate) struct_extends: RangeInclusive<usize>,
    /// attribute allowduplicate { "true" | "false" }?
    pub(crate) allow_duplicate: RangeInclusive<usize>,
    /// attribute requiredlimittype { "true" }?
    pub(crate) required_limit_type: RangeInclusive<usize>,
    ///
    pub(crate) element_variant_vec: Vec<RegistryTypeStructElementVariant>,
}