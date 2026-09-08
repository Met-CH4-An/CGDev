// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use crate::registry_comment_elt::RegistryCommentElt;
use crate::registry_element_enum::RegistryElementEnum;
use crate::registry_element_name::RegistryElementName;
use crate::registry_element_type::RegistryElementType;

pub(crate) enum RegistryTypeBodyWithEnumElement  {
    TYPE(RegistryElementType),
    ENUM(RegistryElementEnum),
    COMMENT_ELT(RegistryCommentElt),
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
    /// element type { TypeName_t }
    pub(crate) element_type_vec: Vec<RegistryElementType>,
    /// element name { attribute alias { text }?, TypeName_t }?
    pub(crate) element_name: RegistryElementName,
    ///
    pub(crate) element_vec: Vec<RegistryTypeBodyWithEnumElement>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные ассоциированные функции.
// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryTypeBodyWithEnum {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные методы.
// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryTypeBodyWithEnum {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные ассоциированные функции.
// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryTypeBodyWithEnum {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn s_create() -> Self {
        Self {
            element_type_vec: Vec::new(),
            element_name: RegistryElementName::s_create(),
            element_vec: Vec::new(),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные методы.
// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryTypeBodyWithEnum {}