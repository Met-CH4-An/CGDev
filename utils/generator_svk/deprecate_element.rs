// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use utils__tokenizer_xml::token::TokenType;
use utils__tokenizer_xml::{Tokenizer, AVX2};
use crate::deprecate_element_item_type::DeprecateElementItemType;
use crate::deprecate_element_item_enum::DeprecateElementItemEnum;
use crate::deprecate_element_item_command::DeprecateElementItemCommand;
use crate::deprecate_element_item_feature::DeprecateElementItemFeature;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) enum DeprecateElementVariant {
    TYPE(DeprecateElementItemType),
    ENUM(DeprecateElementItemEnum),
    COMMAND(DeprecateElementItemCommand),
    FEATURE(DeprecateElementItemFeature),
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// DeprecateElement =
///     element type {
/// # The 'name' can be an arbitrary string, such as a C #include path, in which case this fails
/// #        attribute name { xsd:NCName },
///         attribute name { text },
///         attribute supersededby { text }?,
///         CommentAttr?
///     }
///     | element enum {
///         attribute name { VkDefineOrEnumName_t },
///         attribute supersededby { VkDefineOrEnumName_t }?,
///         CommentAttr?
///     }
///     | element command {
///           attribute name { VkFuncName_t },
///           attribute supersededby { VkFuncName_t }?,
///           CommentAttr?
///       }
///     | element feature {
///           attribute name { text },
///           attribute struct { text },
///           attribute supersededby { text }?,
///           CommentAttr?
///       }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct DeprecateElement {
    ///
    pub(crate) elements: Vec<DeprecateElementVariant>,
}

impl DeprecateElement {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
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

                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "type"} {
                    let mut new_ = DeprecateElementItemType::create();
                    new_.parse(tokenizer, data)?;

                    (&mut *self).elements.push(DeprecateElementVariant::TYPE(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "enum"} {
                    let mut new_ = DeprecateElementItemEnum::create();
                    new_.parse(tokenizer, data)?;

                    (&mut *self).elements.push(DeprecateElementVariant::ENUM(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "command"} {
                    let mut new_ = DeprecateElementItemCommand::create();
                    new_.parse(tokenizer, data)?;

                    (&mut *self).elements.push(DeprecateElementVariant::COMMAND(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "feature"} {
                    let mut new_ = DeprecateElementItemFeature::create();
                    new_.parse(tokenizer, data)?;

                    (&mut *self).elements.push(DeprecateElementVariant::FEATURE(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "/deprecate"} {
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

            // Если встретили просто закрывающийся конец тега ('>'), сообщаем что есть тело.
            // If we encounter a simple closing end tag ('>'), we report that there is a body.
            if token_.asType() == TokenType::TAG_END {
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