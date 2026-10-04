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
/// CommentElt = element comment { text }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct CommentElt {
    /// text
    pub(crate) comment: RangeInclusive<usize>,
}

impl CommentElt {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            comment: 1 ..= 0,
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// CommentElt = element comment { text } <---
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn parse(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<(), String> {
        let is_body_ = self.parseAttributeTag(tokenizer, data)?;
        
        if is_body_ {
            loop {
                let token_ = (&mut *tokenizer).nextToken1();

                if token_.asType() == TokenType::TEXT {
                    self.comment = token_.asRange();
                }
                    
                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "/comment"} {
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
    /// CommentElt = element comment { text }
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseAttributeTag(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<bool, String> {
        let is_body_ = loop {
            let token_ = tokenizer.nextToken1();

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