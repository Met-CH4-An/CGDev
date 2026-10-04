// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use utils__tokenizer_xml::token::TokenType;
use utils__tokenizer_xml::{Tokenizer, AVX2};
use crate::interface_element::InterfaceElement;
use crate::comment_elt::CommentElt;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) enum ExtensionsItemExtensionItemRequireVariant {
    INTERFACE(InterfaceElement),
    COMMENT(CommentElt),
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
pub(crate) struct ExtensionsItemExtensionItemRequire {
    /// ApiAttr?
    api: RangeInclusive<usize>,
    /// ProfileNameAttr?
    profile_name: RangeInclusive<usize>,
    /// DependsAttr?
    depends: RangeInclusive<usize>,
    /// CommentAttr?
    comment: RangeInclusive<usize>,
    /// (InterfaceElement | CommentElt)
    pub(crate) elements: Vec<ExtensionsItemExtensionItemRequireVariant>,
}

impl ExtensionsItemExtensionItemRequire {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            api: 1 ..= 0,
            profile_name: 1 ..= 0,
            depends: 1 ..= 0,            
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

                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "interface"} {
                    let mut new_ = InterfaceElement::create();
                    new_.parse(tokenizer, data)?;

                    (&mut *self).elements.push(ExtensionsItemExtensionItemRequireVariant::INTERFACE(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "comment"} {
                    let mut new_ = CommentElt::create();
                    new_.parse(tokenizer, data)?;

                    (&mut *self).elements.push(ExtensionsItemExtensionItemRequireVariant::COMMENT(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "/require"} {
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

            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "api" {
                let token_ = tokenizer.nextToken1();

                self.api = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "profilename" {
                let token_ = tokenizer.nextToken1();

                self.profile_name = token_.asRange();
            }
                
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "depends" {
                let token_ = tokenizer.nextToken1();

                self.depends = token_.asRange();
            }           

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "comment" {
                let token_ = tokenizer.nextToken1();

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