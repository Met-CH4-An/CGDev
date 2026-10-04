// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use utils__tokenizer_xml::{Tokenizer, AVX2};
use utils__tokenizer_xml::token::TokenType;
use crate::types_item_type_item_type_body_with_enum::{TypesItemTypeItemTypeBodyWithEnum, TypesItemTypeItemTypeBodyWithEnumView};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct TypesItemTypeItemStructItemMemberView<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) content: &'a TypesItemTypeItemStructItemMember,
}

impl<'a> TypesItemTypeItemStructItemMemberView<'a> {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn api(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.api.start() ..= *self.content.api.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn len(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.len.start() ..= *self.content.len.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn altLen(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.alt_len.start() ..= *self.content.alt_len.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn stride(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.stride.start() ..= *self.content.stride.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn externSync(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.extern_sync.start() ..= *self.content.extern_sync.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn optional(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.optional.start() ..= *self.content.optional.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn selector(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.selector.start() ..= *self.content.selector.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn selection(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.selection.start() ..= *self.content.selection.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn noAutoValidity(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.no_auto_validity.start() ..= *self.content.no_auto_validity.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn values(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.values.start() ..= *self.content.values.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn limitType(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.limit_type.start() ..= *self.content.limit_type.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn objectType(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.object_type.start() ..= *self.content.object_type.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn deprecated(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.deprecated.start() ..= *self.content.deprecated.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn featureLink(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.feature_link.start() ..= *self.content.feature_link.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn flagsExtend(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.flags_extend.start() ..= *self.content.flags_extend.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn flagsExtendMember(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.flags_extend_member.start() ..= *self.content.flags_extend_member.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn typeBodyWithEnum(&self) -> TypesItemTypeItemTypeBodyWithEnumView {
        TypesItemTypeItemTypeBodyWithEnumView {
            data: self.data,
            content: &self.content.type_body_with_enum
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// element member {
///                ApiAttr?,
///                attribute len { text }?,
///                attribute altlen { text }?,
///                attribute stride { text }?,
///                attribute externsync { text }?,
///                OptionalAttr?,
///                attribute selector { text }?,
///                attribute selection { VkEnumNameList_t }?,
///                NoAutoValidityAttr?,
///                attribute values { VkEnumNameList_t }?,
///                attribute limittype { text }?,
///                attribute objecttype { text }?,
///                attribute deprecated { text }?,
///                attribute featurelink { text }?,
///                attribute flagsextend { VkTypeNameRef_t }?,
///                attribute flagsextendmember { TypeName_t }?,
///                TypeBodyWithEnum
///            }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct TypesItemTypeItemStructItemMember {
    /// ApiAttr?,
    pub(crate) api: RangeInclusive<usize>,
    /// attribute len { text }?
    pub(crate) len: RangeInclusive<usize>,
    /// attribute altlen { text }?
    pub(crate) alt_len: RangeInclusive<usize>,
    /// attribute stride { text }?
    pub(crate) stride: RangeInclusive<usize>,
    /// attribute externsync { text }?
    pub(crate) extern_sync: RangeInclusive<usize>,
    /// OptionalAttr?
    pub(crate) optional: RangeInclusive<usize>,
    /// attribute selector { text }?
    pub(crate) selector: RangeInclusive<usize>,
    /// attribute selection { VkEnumNameList_t }?
    pub(crate) selection: RangeInclusive<usize>,
    /// NoAutoValidityAttr?
    pub(crate) no_auto_validity: RangeInclusive<usize>,
    /// attribute values { VkEnumNameList_t }?
    pub(crate) values: RangeInclusive<usize>,
    /// attribute limittype { text }?
    pub(crate) limit_type: RangeInclusive<usize>,
    /// attribute objecttype { text }?
    pub(crate) object_type: RangeInclusive<usize>,
    /// attribute deprecated { text }?
    pub(crate) deprecated: RangeInclusive<usize>,
    /// attribute featurelink { text }?
    pub(crate) feature_link: RangeInclusive<usize>,
    /// attribute flagsextend { VkTypeNameRef_t }?
    pub(crate) flags_extend: RangeInclusive<usize>,
    /// attribute flagsextendmember { TypeName_t }?
    pub(crate) flags_extend_member: RangeInclusive<usize>,
    /// TypeBodyWithEnum
    pub(crate) type_body_with_enum: TypesItemTypeItemTypeBodyWithEnum,
}

impl TypesItemTypeItemStructItemMember {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            api: 1 ..= 0,
            len: 1 ..= 0,
            alt_len: 1 ..= 0,
            stride: 1 ..= 0,
            extern_sync: 1 ..= 0,
            optional: 1 ..= 0,
            selector: 1 ..= 0,
            selection: 1 ..= 0,
            no_auto_validity: 1 ..= 0,
            values: 1 ..= 0,
            limit_type: 1 ..= 0,
            object_type: 1 ..= 0,
            deprecated: 1 ..= 0,
            feature_link: 1 ..= 0,
            flags_extend: 1 ..= 0,
            flags_extend_member: 1 ..= 0,
            type_body_with_enum: TypesItemTypeItemTypeBodyWithEnum::create(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <member> ... </member>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn parse(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<(), String> {
        let is_body_ = self.parseAttributeTag(tokenizer, data)?;

        if is_body_ {
            self.type_body_with_enum.parse(tokenizer, data)?;
        } // if is_body_ {

        else {
            return Err(String::from("Не валидный формат vk.xml. Invalid vk.xml format."));
        }

        Ok(())
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <member ...>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseAttributeTag(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<bool, String> {
        let is_body_ = loop {
            let token_ = tokenizer.nextToken1();

            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "api" {
                let token_ = tokenizer.nextToken1();

                self.api = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "len" {
                let token_ = tokenizer.nextToken1();

                self.len = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "altlen" {
                let token_ = tokenizer.nextToken1();

                self.alt_len = token_.asRange();
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

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "selection" {
                let token_ = tokenizer.nextToken1();

                self.selection = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "noautovalidity" {
                let token_ = tokenizer.nextToken1();

                self.no_auto_validity = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "values" {
                let token_ = tokenizer.nextToken1();

                self.values = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "limittype" {
                let token_ = tokenizer.nextToken1();

                self.limit_type = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "objecttype" {
                let token_ = tokenizer.nextToken1();

                self.object_type = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "deprecated" {
                let token_ = tokenizer.nextToken1();

                self.deprecated = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "featurelink" {
                let token_ = tokenizer.nextToken1();

                self.feature_link = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "flagsextend" {
                let token_ = tokenizer.nextToken1();

                self.flags_extend = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "flagsextendmember" {
                let token_ = tokenizer.nextToken1();

                self.flags_extend_member = token_.asRange();
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