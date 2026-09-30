// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use crate::type_base_type::RegistryTypeBaseType;
use crate::type_bitmask::RegistryTypeBitmask;
use crate::type_define::RegistryTypeDefine;
use crate::type_enum::RegistryTypeEnum;
use crate::type_func_pointer::RegistryTypeFuncpointer;
use crate::type_handle::RegistryTypeHandle;
use crate::type_include::RegistryTypeInclude;
use crate::type_requires::RegistryTypeRequires;
use crate::type_struct::RegistryTypeStruct;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) enum RegistryExtensionElementVariant {

}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Extension =
///     element extension {
///         attribute name { VkExtName_t },
///         attribute number { IntegerOrHex_t },
///         attribute sortorder { xsd:integer }?,
///         ProtectAttr?,
///         attribute platform { text }?,
///         attribute author { text }?,
///         attribute contact { text }?,
///         attribute type { "instance" | "device" }?,
///         DependsAttr?,
///         attribute supported { SupportedList_t | "disabled" }?,
///         attribute ratified { SupportedList_t }?,
///         attribute promotedto { VkVersion_t | VkExtName_t }?,
///         attribute deprecatedby { "" | VkVersion_t | VkExtName_t }?,
///         attribute obsoletedby { VkVersion_t | VkExtName_t }?,
///         attribute provisional { "true" }?,
///         attribute specialuse { StringList_t }?,
///         attribute nofeatures { StringBool_t }?,
///         CommentAttr?,
///         (element require {
///              ApiAttr?,
///              ProfileNameAttr?,
///              DependsAttr?,
///              CommentAttr?,
///              (InterfaceElement | CommentElt)*
///          }
///          | element deprecate {
///                ApiAttr?,
///             attribute explanationlink { text } ,
///                ProfileNameAttr?,
///                CommentAttr?,
///                (DeprecateElement | CommentElt)*
///          }
///          | element remove {
///                ApiAttr?,
///                ProfileNameAttr?,
///                CommentAttr?,
///                (InterfaceElement | CommentElt)*
///            })*
///     }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct RegistryExtension {
    ///         attribute name { VkExtName_t },
    name: RangeInclusive<usize>,
    ///         attribute number { IntegerOrHex_t },
    number: RangeInclusive<usize>,
    ///         attribute sortorder { xsd:integer }?,
    sort_order: RangeInclusive<usize>,
    ///         ProtectAttr?,
    protect: RangeInclusive<usize>,
    ///         attribute platform { text }?,
    platform: RangeInclusive<usize>,
    ///         attribute author { text }?,
    author: RangeInclusive<usize>,
    ///         attribute contact { text }?,
    contact: RangeInclusive<usize>,
    ///         attribute type { "instance" | "device" }?,
    r#type: RangeInclusive<usize>,
    ///         DependsAttr?,
    depends: RangeInclusive<usize>,
    ///         attribute supported { SupportedList_t | "disabled" }?,
    supported: RangeInclusive<usize>,
    ///         attribute ratified { SupportedList_t }?,
    ratified: RangeInclusive<usize>,
    ///         attribute promotedto { VkVersion_t | VkExtName_t }?,
    promotedto: RangeInclusive<usize>,
    ///         attribute deprecatedby { "" | VkVersion_t | VkExtName_t }?,
    deprecated_by: RangeInclusive<usize>,
    ///         attribute obsoletedby { VkVersion_t | VkExtName_t }?,
    obsoleted_by: RangeInclusive<usize>,
    ///         attribute provisional { "true" }?,
    provisional: RangeInclusive<usize>,
    ///         attribute specialuse { StringList_t }?,
    special_use: RangeInclusive<usize>,
    ///         attribute nofeatures { StringBool_t }?,
    no_features: RangeInclusive<usize>,
    ///         CommentAttr?,
    comment: RangeInclusive<usize>,
}

impl RegistryExtension {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            registry_type_element_variant: RegistryTypeElementVariant::UNKNOWN,
        }
    }
}