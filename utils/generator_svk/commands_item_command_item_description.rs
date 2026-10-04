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
/// Command =
///     element command {
///         (attribute name { VkFuncName_t },
///          attribute alias { VkFuncName_t },
///          ApiAttr?)
///         | (attribute tasks { StringList_t }?,
///            attribute queues { StringList_t }?,
///            attribute successcodes { VkEnumNameList_t }?,
///            attribute errorcodes { VkEnumNameList_t }?,
///            attribute renderpass { "inside" | "outside" | "both" }?,
///            attribute videocoding { "inside" | "outside" | "both" }?,
///            attribute conditionalrendering { "true" | "false" }?,
///            attribute cmdbufferlevel { CmdbufferList_t }?,
///            attribute allownoqueues { StringBool_t } ? ,
///            attribute prefix { text }?,
///            attribute suffix { text }?,
///            attribute export { StringList_t }?,
///            ApiAttr?,
///            CommentAttr?,
///            element proto {
///                mixed {
///                    element type { TypeName_t }?,
///                    element name { text }
///                }
///            },
///            element param {
///                ApiAttr?,
///                attribute len { text }?,
///                attribute altlen { text }?,
///                attribute stride { text }?,
///                attribute externsync { text }?,
///                OptionalAttr?,
///                attribute selector { text }?,
///                NoAutoValidityAttr?,
///                attribute objecttype { text }?,
///                attribute validstructs { VkTypeNameListRef_t }?,
///                mixed {
///                    element type { TypeName_t }?,
///                    element name { text }?
///                }
///            }*,
///            (element alias { NameAttr }?
///             & element description { text }?                         <---
///             & element implicitexternsyncparams {
///                   element param { text }*
///               }?))
///     }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct CommandsItemCommandItemDescription {
    ///
    pub(crate) prefix: RangeInclusive<usize>,
    /// element description { text }
    pub(crate) value: RangeInclusive<usize>,
    ///
    pub(crate) postfix: RangeInclusive<usize>,
}

impl CommandsItemCommandItemDescription {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            prefix: 1 ..= 0,
            value: 1 ..= 0,
            postfix: 1 ..= 0,
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn parse(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<(), String> {
        let is_body_ = (&mut *self).parseAttributeTag(tokenizer, data)?;

        if is_body_ {
            loop {
                let token_ = tokenizer.nextToken1();

                if token_.asType() == TokenType::TEXT {
                    self.value = token_.asRange();
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "/description"} {
                    break true;
                }

                // Если встретился не валидный токен или конечный токен.
                // If an invalid token or final token is encountered.
                else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                    return Err(String::from("Не валидный формат vk.xml. Invalid vk.xml format."));
                }
            }; // loop {
        } // if is_body_ {

        else {
            return Err(String::from("Не валидный формат vk.xml. Invalid vk.xml format."));
        }

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