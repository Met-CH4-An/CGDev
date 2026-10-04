// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use utils__tokenizer_xml::{Tokenizer, AVX2};
use utils__tokenizer_xml::token::TokenType;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// InterfaceElement =
///     element type {
/// # The 'name' can be an arbitrary string, such as a C #include path, in which case this fails
/// #        attribute name { xsd:NCName },
///         attribute name { text },
///         CommentAttr?,
///         SimpleProtectAttr?
///     }
///     | Enum
///     | element command {
///           attribute name { VkFuncName_t },
///           CommentAttr?,
///           SimpleProtectAttr?
///       }
///     | element feature {
///           attribute name { text },
///           attribute struct { text },
///           CommentAttr?
///       }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct InterfaceElementItemCommand {
    /// attribute name { VkFuncName_t },
    pub(crate) name: RangeInclusive<usize>,
    /// CommentAttr?,
    pub(crate) comment: RangeInclusive<usize>,
    /// SimpleProtectAttr?
    pub(crate) protect: RangeInclusive<usize>,
}

impl InterfaceElementItemCommand {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            name: 1 ..= 0,
            comment: 1 ..= 0,
            protect: 1 ..= 0,
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn parse(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<(), String> {
        let is_body_ = self.parseAttributeTag(tokenizer, data)?;

        if is_body_ {
            return Err(String::from("Не валидный формат vk.xml. Invalid vk.xml format."));
        } // if is_body_ {

        Ok(())
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseAttributeTag(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<bool, String> {
        let is_body_ = loop {
            let token_ = tokenizer.nextToken1();

            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "name" {
                let token_ = tokenizer.nextToken1();

                self.name = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "comment" {
                let token_ = tokenizer.nextToken1();

                self.comment = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "protect" {
                let token_ = tokenizer.nextToken1();

                self.protect = token_.asRange();
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