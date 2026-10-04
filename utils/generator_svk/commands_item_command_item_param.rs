// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use utils__tokenizer_xml::{Tokenizer, AVX2};
use utils__tokenizer_xml::token::TokenType;
use crate::commands_item_command_item_param_item_name::CommandsItemCommandItemParamItemName;
use crate::commands_item_command_item_param_item_type::CommandsItemCommandItemParamItemType;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// element param {
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
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct CommandsItemCommandItemParam {
    /// ApiAttr
    pub(crate) api: RangeInclusive<usize>,
    /// attribute len { text }
    pub(crate) len: RangeInclusive<usize>,
    /// attribute stride { text }
    pub(crate) stride: RangeInclusive<usize>,
    /// attribute externsync { text }
    pub(crate) extern_sync: RangeInclusive<usize>,
    /// OptionalAttr
    pub(crate) optional: RangeInclusive<usize>,
    /// attribute selector { text }
    pub(crate) selector: RangeInclusive<usize>,
    /// NoAutoValidityAttr
    pub(crate) no_auto_validity: RangeInclusive<usize>,
    /// attribute objecttype { text }
    pub(crate) object_type: RangeInclusive<usize>,
    /// attribute validstructs { VkTypeNameListRef_t }
    pub(crate) valid_structs: RangeInclusive<usize>,

    /// element type { TypeName_t }
    pub(crate) r#type: CommandsItemCommandItemParamItemType,
    /// element name { text }
    pub(crate) name: CommandsItemCommandItemParamItemName,
}

impl CommandsItemCommandItemParam {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            api: 1 ..= 0,
            len: 1 ..= 0,
            stride: 1 ..= 0,
            extern_sync: 1 ..= 0,
            optional: 1 ..= 0,
            selector: 1 ..= 0,
            no_auto_validity: 1 ..= 0,
            object_type: 1 ..= 0,
            valid_structs: 1 ..= 0,
            r#type: CommandsItemCommandItemParamItemType::create(),
            name: CommandsItemCommandItemParamItemName::create(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn parse(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<(), String> {
        let is_body_ = (&mut *self).parseAttributeTag(tokenizer, data)?;

        let mut text_ = None;

        if is_body_ {
            loop {
                let token_ = tokenizer.nextToken1();

                if token_.asType() == TokenType::TEXT {
                    text_ = Some(token_.asRange());
                }

                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "type"} {
                    let mut new_ = CommandsItemCommandItemParamItemType::create();
                    new_.parse(tokenizer, data)?;

                    // Если текстовый токен существует, забираем. Это префикс к type.
                    // If the text token exists, we retrieve it. This is the prefix for the type.
                    if let Some(v) = text_.take() {
                        new_.prefix = v;
                    }

                    self.r#type = new_;
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "name"} {
                    let mut new_ = CommandsItemCommandItemParamItemName::create();
                    new_.parse(tokenizer, data)?;

                    // Если текстовый токен существует, забираем. Это префикс к type.
                    // If the text token exists, we retrieve it. This is the prefix for the type.
                    if let Some(v) = text_.take() {
                        self.r#type.postfix = v;
                    }

                    self.name = new_;
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "/param"} {
                    break;
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

            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "api" {
                let token_ = tokenizer.nextToken1();

                self.api = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "len" {
                let token_ = tokenizer.nextToken1();

                self.len = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "stride" {
                let token_ = tokenizer.nextToken1();

                self.stride = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "externsync" {
                let token_ = tokenizer.nextToken1();

                self.extern_sync = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "optional" {
                let token_ = tokenizer.nextToken1();

                self.optional = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "selector" {
                let token_ = tokenizer.nextToken1();

                self.selector = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "noautovalidity" {
                let token_ = tokenizer.nextToken1();

                self.no_auto_validity = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "objecttype" {
                let token_ = tokenizer.nextToken1();

                self.object_type = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "validstructs" {
                let token_ = tokenizer.nextToken1();

                self.valid_structs = token_.asRange();
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