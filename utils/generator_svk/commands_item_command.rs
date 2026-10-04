// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use utils__tokenizer_xml::token::TokenType;
use utils__tokenizer_xml::{Tokenizer, AVX2};
use crate::commands_item_command_item_alias::CommandsItemCommandItemAlias;
use crate::commands_item_command_item_description::CommandsItemCommandItemDescription;
use crate::commands_item_command_item_implicit_extern_sync_params::CommandsItemCommandItemImplicitExternSyncParams;
use crate::commands_item_command_item_param::CommandsItemCommandItemParam;
use crate::commands_item_command_item_proto::CommandsItemCommandItemProto;
use crate::enums::EnumsElementVariant;
use crate::enums_item_enum::EnumsItemEnum;
use crate::unused::Unused;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) enum CommandElementVariant {
    ALIAS(CommandsItemCommandItemAlias),
    DESCRIPTION(CommandsItemCommandItemDescription),
    IMPLICIT_EXTERN_SYNC_PARAMS(CommandsItemCommandItemImplicitExternSyncParams),
    PARAM(CommandsItemCommandItemParam),
    PROTO(CommandsItemCommandItemProto),
}

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
///             & element description { text }?
///             & element implicitexternsyncparams {
///                   element param { text }*
///               }?))
///     }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct CommandsItemCommand {
    /// attribute name { VkFuncName_t }
    pub(crate) name: RangeInclusive<usize>,
    /// attribute alias { VkFuncName_t }
    pub(crate) alias: RangeInclusive<usize>,
    /// ApiAttr
    pub(crate) api: RangeInclusive<usize>,
    /// attribute tasks { StringList_t }
    pub(crate) tasks: RangeInclusive<usize>,
    /// attribute queues { StringList_t }
    pub(crate) queues: RangeInclusive<usize>,
    ///attribute successcodes { VkEnumNameList_t }?,
    pub(crate) success_codes: RangeInclusive<usize>,
    /// attribute errorcodes { VkEnumNameList_t }?,
    pub(crate) error_codes: RangeInclusive<usize>,
    /// attribute renderpass { "inside" | "outside" | "both" }?,
    pub(crate) render_pass: RangeInclusive<usize>,
    /// attribute videocoding { "inside" | "outside" | "both" }?,
    pub(crate) video_coding: RangeInclusive<usize>,
    /// attribute conditionalrendering { "true" | "false" }?,
    pub(crate) conditional_rendering: RangeInclusive<usize>,
    /// attribute cmdbufferlevel { CmdbufferList_t }?,
    pub(crate) cmd_buffer_level: RangeInclusive<usize>,
    /// attribute allownoqueues { StringBool_t } ? ,
    pub(crate) allow_no_queues: RangeInclusive<usize>,
    /// attribute prefix { text }?,
    pub(crate) prefix: RangeInclusive<usize>,
    /// attribute suffix { text }?,
    pub(crate) suffix: RangeInclusive<usize>,
    /// attribute export { StringList_t }?,
    pub(crate) export: RangeInclusive<usize>,
    /// CommentAttr?,
    pub(crate) comment: RangeInclusive<usize>,

    ///
    pub(crate) elements: Vec<CommandElementVariant>,
}

impl CommandsItemCommand {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            name: 1 ..= 0,
            alias: 1 ..= 0,
            api: 1 ..= 0,
            tasks: 1 ..= 0,
            queues: 1 ..= 0,
            success_codes: 1 ..= 0,
            error_codes: 1 ..= 0,
            render_pass: 1 ..= 0,
            video_coding: 1 ..= 0,
            conditional_rendering: 1 ..= 0,
            cmd_buffer_level: 1 ..= 0,
            allow_no_queues: 1 ..= 0,
            prefix: 1 ..= 0,
            suffix: 1 ..= 0,
            export: 1 ..= 0,
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

                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "alias"} {
                    let mut new_ = CommandsItemCommandItemAlias::create();
                    new_.parse(tokenizer, data)?;

                    (&mut *self).elements.push(CommandElementVariant::ALIAS(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "description"} {
                    let mut new_ = CommandsItemCommandItemDescription::create();
                    new_.parse(tokenizer, data)?;

                    (&mut *self).elements.push(CommandElementVariant::DESCRIPTION(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "implicitexternsyncparams"} {
                    let mut new_ = CommandsItemCommandItemImplicitExternSyncParams::create();
                    new_.parse(tokenizer, data)?;

                    (&mut *self).elements.push(CommandElementVariant::IMPLICIT_EXTERN_SYNC_PARAMS(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "param"} {
                    let mut new_ = CommandsItemCommandItemParam::create();
                    new_.parse(tokenizer, data)?;

                    (&mut *self).elements.push(CommandElementVariant::PARAM(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "proto"} {
                    let mut new_ = CommandsItemCommandItemProto::create();
                    new_.parse(tokenizer, data)?;

                    (&mut *self).elements.push(CommandElementVariant::PROTO(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "/command"} {
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

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "alias" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.alias = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "api" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.api = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "tasks" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.tasks = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "queues" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.queues = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "successcodes" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.success_codes = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "errorcodes" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.error_codes = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "renderpass" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.render_pass = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "videocoding" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.video_coding = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "conditionalrendering" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.conditional_rendering = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "cmdbufferlevel" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.cmd_buffer_level = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "allownoqueues" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.allow_no_queues = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "prefix" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.prefix = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "suffix" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.suffix = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "export" {
                let token_ = (&mut *tokenizer).nextToken1();

                self.export = token_.asRange();
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