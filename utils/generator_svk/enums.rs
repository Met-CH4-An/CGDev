// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use utils__tokenizer_xml::token::TokenType;
use utils__tokenizer_xml::{Tokenizer, AVX2};
use crate::enums_item_enum::EnumsItemEnum;
use crate::comment_elt::CommentElt;
use crate::unused::Unused;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) enum EnumsElementVariant {
    ENUM(EnumsItemEnum),
    UNUSED(Unused),
    COMMENT_ELT(CommentElt)
} 

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Enums =
///     element enums {
///         attribute name { text }?,
///         attribute type { text },
///         attribute bitwidth { "32" | "64" } ?,
///         CommentAttr?,
///         (Enum | Unused | CommentElt)*
///     }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct Enums {
    /// attribute name { text }?,
    pub(crate) name: RangeInclusive<usize>,
    /// attribute type { text },
    pub(crate) r#type: RangeInclusive<usize>,
    /// attribute bitwidth { "32" | "64" } ?,
    pub(crate) bitwidth: RangeInclusive<usize>,
    /// CommentAttr?,
    pub(crate) comment: RangeInclusive<usize>,
    /// (Enum | Unused | CommentElt)*
    pub(crate) element_variants: Vec<EnumsElementVariant>,
}

impl Enums {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            name: 1 ..= 0,
            r#type: 1 ..= 0,
            bitwidth: 1 ..= 0,
            comment: 1 ..= 0,
            element_variants: Vec::new(),
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

                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "enum"} {
                    let mut new_ = EnumsItemEnum::create();
                    new_.parse(tokenizer, data)?;

                    (&mut *self).element_variants.push(EnumsElementVariant::ENUM(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "unused"} {
                    let new_ = Unused::create(tokenizer, data)?;

                    (&mut *self).element_variants.push(EnumsElementVariant::UNUSED(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "comment"} {
                    let mut new_ = CommentElt::create();
                    new_.parse(tokenizer, data)?;

                    (&mut *self).element_variants.push(EnumsElementVariant::COMMENT_ELT(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "/enums"} {
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

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "type" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.r#type = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "bitwidth" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.bitwidth = token_.asRange();
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