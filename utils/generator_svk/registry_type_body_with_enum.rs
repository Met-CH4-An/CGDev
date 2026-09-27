// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::registry_primitive_comment_elt::RegistryPrimitiveCommentElt;
use crate::registry_primitive_element_enum::RegistryPrimitiveElementEnum;
use crate::registry_primitive_element_name::RegistryPrimitiveElementName;
use crate::registry_primitive_element_type::RegistryPrimitiveElementType;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) enum RegistryTypeBodyWithEnumElementVariant {
    TYPE(RegistryPrimitiveElementType),
    ENUM(RegistryPrimitiveElementEnum),
    COMMENT(RegistryPrimitiveCommentElt),
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// TypeBodyWithEnum =
///         mixed {
///           element type { TypeName_t }
///         }*,
///         mixed {
///           element name { attribute alias { text }?, TypeName_t }?
///         }?,
///         mixed {
///           ( element type { TypeName_t }
///             | element enum { VkDefineOrEnumName_t }
///             | CommentElt
///           )
///         }*
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct RegistryTypeBodyWithEnum {
    /// mixed {
    ///     element type { TypeName_t }
    /// }*,
    pub(crate) registry_primitive_element_type_vec: Vec<RegistryPrimitiveElementType>,
    /// element name { attribute alias { text }?, TypeName_t }?
    pub(crate) registry_primitive_element_name: RegistryPrimitiveElementName,
    /// mixed {
    ///     ( element type { TypeName_t }
    ///     | element enum { VkDefineOrEnumName_t }
    ///     | CommentElt
    ///     )
    /// }*
    pub(crate) registry_type_body_with_enum_element_variant_vec: Vec<RegistryTypeBodyWithEnumElementVariant>,
}
impl RegistryTypeBodyWithEnum {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            registry_primitive_element_type_vec: Vec::new(),
            registry_primitive_element_name: RegistryPrimitiveElementName::create(),
            registry_type_body_with_enum_element_variant_vec: Vec::new(),
        }
    }
}