// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Enum =
///     element enum {
///         ((attribute value { text }
///           & # needs to be split to handle the string defines as well as ints
///             attribute extends { TypeName_t }?)
///          | (attribute bitpos { xsd:long }
///             & attribute extends { VkTypeNameRef_t }?)
///          | (attribute extnumber { xsd:long }?
///             & attribute offset { xsd:long }
///             & attribute dir { "-" }?
///             & attribute extends { VkTypeNameRef_t })
///          | (attribute extends { VkTypeNameRef_t }?
///             & attribute alias {
///                   VkTypeNameRef_t | VkDefineOrEnumName_t
///               })
///          | (attribute value { text }
///             & attribute extends { VkTypeNameRef_t }?
///             & attribute alias {
///                   VkTypeNameRef_t | VkDefineOrEnumName_t
///               })
///          | (attribute bitpos { xsd:long }
///             & attribute extends { VkTypeNameRef_t }?
///             & attribute alias {
///                   VkTypeNameRef_t | VkDefineOrEnumName_t
///               }))?
///         & ProtectAttr?
///         & ApiAttr?
///         & attribute type { "uint8_t" | "uint32_t" | "uint64_t" | "float" }?
///         & attribute name { VkDefineOrEnumName_t }
///         & attribute deprecated { "aliased" | "unused" | "true" }?
///         & CommentAttr?
///     }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct RegistryEnum {
    /// attribute value { text }
    pub(crate) value_rng: RangeInclusive<usize>,
    /// attribute bitpos { xsd:long }
    pub(crate) bitpos_rng: RangeInclusive<usize>,
    /// attribute offset { xsd:long }
    pub(crate) offset_rng: RangeInclusive<usize>,
    /// attribute dir { "-" }?
    pub(crate) dir_rng: RangeInclusive<usize>,
    /// attribute alias { VkTypeNameRef_t | VkDefineOrEnumName_t }
    pub(crate) alias_rng: RangeInclusive<usize>,
    /// attribute extends { TypeName_t }?
    pub(crate) extends_rng: RangeInclusive<usize>,
    /// ProtectAttr?
    pub(crate) protect_rng: RangeInclusive<usize>,
    /// ApiAttr?
    pub(crate) api_rng: RangeInclusive<usize>,
    /// attribute type { "uint8_t" | "uint32_t" | "uint64_t" | "float" }?
    pub(crate) type_rng: RangeInclusive<usize>,
    /// attribute name { VkDefineOrEnumName_t }
    pub(crate) name_rng: RangeInclusive<usize>,
    /// attribute deprecated { "aliased" | "unused" | "true" }?
    pub(crate) deprecated_rng: RangeInclusive<usize>,
    /// CommentAttr?
    pub(crate) comment_rng: RangeInclusive<usize>,
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
            value_rng: 1 ..= 0,
            bitpos_rng: 1 ..= 0,
            offset_rng: 1 ..= 0,
            dir_rng: 1 ..= 0,
            alias_rng: 1 ..= 0,
            extends_rng: 1 ..= 0,
            protect_rng: 1 ..= 0,
            api_rng: 1 ..= 0,
            type_rng: 1 ..= 0,
            name_rng: 1 ..= 0,
            deprecated_rng: 1 ..= 0,
            comment_rng: 1 ..= 0,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные методы.
// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl RegistryEnum {}