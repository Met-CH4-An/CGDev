// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use utils__tokenizer_xml::token::TokenType;
use utils__tokenizer_xml::{Tokenizer, AVX2};
use crate::extensions_item_extension_item_require::ExtensionsItemExtensionItemRequire;
use crate::extensions_item_extension_item_deprecate::ExtensionsItemExtensionItemDeprecate;
use crate::extensions_item_extension_item_remove::ExtensionsItemExtensionItemRemove;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) enum ExtensionsItemExtensionVariant {
    REQUIRE(ExtensionsItemExtensionItemRequire),
    DEPRECATE(ExtensionsItemExtensionItemDeprecate),
    REMOVE(ExtensionsItemExtensionItemRemove)
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
pub(crate) struct ExtensionsItemExtension {
    ///attribute name { VkExtName_t }
    pub(crate) name: RangeInclusive<usize>,
    /// attribute number { IntegerOrHex_t }
    pub(crate) number: RangeInclusive<usize>,
    /// attribute sortorder { xsd:integer }?
    pub(crate) sort_order: RangeInclusive<usize>,
    /// ProtectAttr?
    pub(crate) protect: RangeInclusive<usize>,
    /// attribute platform { text }?
    pub(crate) platform: RangeInclusive<usize>,
    /// attribute author { text }?
    pub(crate) author: RangeInclusive<usize>,
    /// attribute contact { text }?
    pub(crate) contact: RangeInclusive<usize>,
    /// attribute type { "instance" | "device" }?
    pub(crate) r#type: RangeInclusive<usize>,
    /// DependsAttr?
    pub(crate) depends: RangeInclusive<usize>,
    /// attribute supported { SupportedList_t | "disabled" }?
    pub(crate) supported: RangeInclusive<usize>,
    /// attribute ratified { SupportedList_t }?
    pub(crate) ratified: RangeInclusive<usize>,
    /// attribute promotedto { VkVersion_t | VkExtName_t }?
    pub(crate) promoted_to: RangeInclusive<usize>,
    /// attribute deprecatedby { "" | VkVersion_t | VkExtName_t }?
    pub(crate) deprecated_by: RangeInclusive<usize>,
    /// attribute obsoletedby { VkVersion_t | VkExtName_t }?
    pub(crate) obsoleted_by: RangeInclusive<usize>,
    /// attribute provisional { "true" }?
    pub(crate) provisional: RangeInclusive<usize>,
    /// attribute specialuse { StringList_t }?
    pub(crate) specialuse: RangeInclusive<usize>,
    /// attribute nofeatures { StringBool_t }?
    pub(crate) no_features: RangeInclusive<usize>,
    /// CommentAttr?,
    pub(crate) comment: RangeInclusive<usize>,
    
    elements: Vec<ExtensionsItemExtensionVariant>,
}

impl ExtensionsItemExtension {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            name: 1 ..= 0,
            number: 1 ..= 0,
            sort_order: 1 ..= 0,
            protect: 1 ..= 0,
            platform: 1 ..= 0,
            author: 1 ..= 0,
            contact: 1 ..= 0,
            r#type: 1 ..= 0,
            depends: 1 ..= 0,
            supported: 1 ..= 0,
            ratified: 1 ..= 0,
            promoted_to: 1 ..= 0,
            deprecated_by: 1 ..= 0,
            obsoleted_by: 1 ..= 0,
            provisional: 1 ..= 0,
            specialuse: 1 ..= 0,
            no_features: 1 ..= 0,
            comment: 1 ..= 0,
            elements: Vec::new(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn parse(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<(), String> {
        let is_body_ = (&mut *self).parseAttributeTag(tokenizer, data)?;

        if is_body_ {
            loop {
                let token_ = (&mut *tokenizer).nextToken1();

                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "require"} {
                    let mut new_ = ExtensionsItemExtensionItemRequire::create();
                    new_.parse(tokenizer, data)?;

                    (&mut *self).elements.push(ExtensionsItemExtensionVariant::REQUIRE(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "deprecate"} {
                    let mut new_ = ExtensionsItemExtensionItemDeprecate::create();
                    new_.parse(tokenizer, data)?;

                    (&mut *self).elements.push(ExtensionsItemExtensionVariant::DEPRECATE(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "remove"} {
                    let mut new_ = ExtensionsItemExtensionItemRemove::create();
                    new_.parse(tokenizer, data)?;

                    (&mut *self).elements.push(ExtensionsItemExtensionVariant::REMOVE(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "/extension"} {
                    break;
                }

                // Если встретился не валидный токен или конечный токен.
                // If an invalid token or final token is encountered.
                else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                    return Err(String::from("Не валидный формат vk.xml. Invalid vk.xml format."));
                }
            } // loop {
        } // if is_body_ {

        Ok(())
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseAttributeTag(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<bool, String> {
        let is_body_ = loop {
            let token_ = (&mut *tokenizer).nextToken1();

            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "name" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.name = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "number" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.number = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "sortorder" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.sort_order = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "protect" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.protect = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "platform" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.platform = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "author" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.author = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "contact" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.contact = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "type" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.r#type = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "depends" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.depends = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "supported" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.number = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "ratified" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.ratified = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "promotedto" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.promoted_to = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "deprecatedby" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.deprecated_by = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "obsoletedby" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.obsoleted_by = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "provisional" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.provisional = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "specialuse" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.specialuse = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "nofeatures" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.no_features = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "comment" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.comment = token_.asRange();
            }

            // Если встретили просто закрывающийся конец тега ('>'), сообщаем что есть тело.
            // If we encounter a simple closing end tag ('>'), we report that there is a body.
            else if token_.asType() == TokenType::TAG_END {
                break true;
            }

            // Если встретили самозакрывающийся конец тега ('/>'), сообщаем что тела нет.
            // If we encounter a self-closing end tag ('/>'), we report that there is no body.
            else if token_.asType() == TokenType::TAG_END_CLOSE {
                break false;
            }

            // Если встретился не валидный токен или конечный токен.
            // If an invalid token or final token is encountered.
            else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                return Err(String::from("Не валидный формат vk.xml. Invalid vk.xml format."));
            }
        }; // let is_body_ = loop {

        Ok(is_body_)
    }
}