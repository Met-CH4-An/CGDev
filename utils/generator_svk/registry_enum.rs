// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use utils__tokenizer_xml::token::TokenType;
use utils__tokenizer_xml::{Tokenizer, AVX2};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Enum =
///     element enum {
///         ((attribute value { text }
///           & # needs to be split to handle the string defines as well as ints
///             attribute extends { TypeName_t }?)
///          | (attribute bitpos { xsd:long }
///             & attribute extends { VkTypeNameRef_t }?)
///          | (attribute extnumber { xsd:long }?
///             & attribute offset { xsd:long }
///             & attribute dir { "-" }?
///             & attribute extends { VkTypeNameRef_t })
///          | (attribute extends { VkTypeNameRef_t }?
///             & attribute alias {
///                   VkTypeNameRef_t | VkDefineOrEnumName_t
///               })
///          | (attribute value { text }
///             & attribute extends { VkTypeNameRef_t }?
///             & attribute alias {
///                   VkTypeNameRef_t | VkDefineOrEnumName_t
///               })
///          | (attribute bitpos { xsd:long }
///             & attribute extends { VkTypeNameRef_t }?
///             & attribute alias {
///                   VkTypeNameRef_t | VkDefineOrEnumName_t
///               }))?
///         & ProtectAttr?
///         & ApiAttr?
///         & attribute type { "uint8_t" | "uint32_t" | "uint64_t" | "float" }?
///         & attribute name { VkDefineOrEnumName_t }
///         & attribute deprecated { "aliased" | "unused" | "true" }?
///         & CommentAttr?
///     }
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct Enum {
    /// attribute value { text }
    pub(crate) value: RangeInclusive<usize>,
    /// attribute bitpos { xsd:long }
    pub(crate) bit_pos: RangeInclusive<usize>,
    /// attribute extnumber { xsd:long }?
    pub(crate) ext_number: RangeInclusive<usize>,
    /// attribute offset { xsd:long }
    pub(crate) offset: RangeInclusive<usize>,
    /// attribute dir { "-" }?
    pub(crate) dir: RangeInclusive<usize>,
    /// attribute extends { TypeName_t }?
    pub(crate) extends: RangeInclusive<usize>,
    /// attribute alias { VkTypeNameRef_t | VkDefineOrEnumName_t }
    pub(crate) alias: RangeInclusive<usize>,
    /// ProtectAttr?
    pub(crate) protect: RangeInclusive<usize>,
    /// ApiAttr?
    pub(crate) api: RangeInclusive<usize>,
    /// attribute type { "uint8_t" | "uint32_t" | "uint64_t" | "float" }?
    pub(crate) r#type: RangeInclusive<usize>,
    /// attribute name { VkDefineOrEnumName_t }
    pub(crate) name: RangeInclusive<usize>,
    /// attribute deprecated { "aliased" | "unused" | "true" }?
    pub(crate) deprecated: RangeInclusive<usize>,
    /// CommentAttr?
    pub(crate) comment: RangeInclusive<usize>,
}

impl Enum {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create(tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<Self, String> {
        let mut self_ = Self {
            value: 1 ..= 0,
            bit_pos: 1 ..= 0,
            ext_number: 1 ..= 0,
            offset: 1 ..= 0,
            dir: 1 ..= 0,
            extends: 1 ..= 0,
            alias: 1 ..= 0,
            protect: 1 ..= 0,
            api: 1 ..= 0,
            r#type: 1 ..= 0,
            name: 1 ..= 0,
            deprecated: 1 ..= 0,
            comment: 1 ..= 0,
        };

        self_.parse(tokenizer, data)?;

        Ok(self_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Enum =
    ///     element enum {
    ///         ((attribute value { text }
    ///           & # needs to be split to handle the string defines as well as ints
    ///             attribute extends { TypeName_t }?)
    ///          | (attribute bitpos { xsd:long }
    ///             & attribute extends { VkTypeNameRef_t }?)
    ///          | (attribute extnumber { xsd:long }?
    ///             & attribute offset { xsd:long }
    ///             & attribute dir { "-" }?
    ///             & attribute extends { VkTypeNameRef_t })
    ///          | (attribute extends { VkTypeNameRef_t }?
    ///             & attribute alias {
    ///                   VkTypeNameRef_t | VkDefineOrEnumName_t
    ///               })
    ///          | (attribute value { text }
    ///             & attribute extends { VkTypeNameRef_t }?
    ///             & attribute alias {
    ///                   VkTypeNameRef_t | VkDefineOrEnumName_t
    ///               })
    ///          | (attribute bitpos { xsd:long }
    ///             & attribute extends { VkTypeNameRef_t }?
    ///             & attribute alias {
    ///                   VkTypeNameRef_t | VkDefineOrEnumName_t
    ///               }))?
    ///         & ProtectAttr?
    ///         & ApiAttr?
    ///         & attribute type { "uint8_t" | "uint32_t" | "uint64_t" | "float" }?
    ///         & attribute name { VkDefineOrEnumName_t }
    ///         & attribute deprecated { "aliased" | "unused" | "true" }?
    ///         & CommentAttr?
    ///     }
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parse(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<(), String> {
        let is_body_ = self.parseAttributeTag(tokenizer, data)?;

        if is_body_ {
            return Err(String::from("Не валидный формат vk.xml. Invalid vk.xml format."));
        } // if is_body_ {

        Ok(())
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Enum =
    ///     element enum {                                                                              
    ///         ((attribute value { text }                                                  <---
    ///           & # needs to be split to handle the string defines as well as ints
    ///             attribute extends { TypeName_t }?)                                      <---
    ///          | (attribute bitpos { xsd:long }                                           <---
    ///             & attribute extends { VkTypeNameRef_t }?)                               <---
    ///          | (attribute extnumber { xsd:long }?                                       <---
    ///             & attribute offset { xsd:long }                                         <---
    ///             & attribute dir { "-" }?                                                <---
    ///             & attribute extends { VkTypeNameRef_t })                                <---
    ///          | (attribute extends { VkTypeNameRef_t }?                                  <---
    ///             & attribute alias {                                                     <---
    ///                   VkTypeNameRef_t | VkDefineOrEnumName_t
    ///               })
    ///          | (attribute value { text }                                                <---
    ///             & attribute extends { VkTypeNameRef_t }?                                <---
    ///             & attribute alias {                                                     <---
    ///                   VkTypeNameRef_t | VkDefineOrEnumName_t
    ///               })
    ///          | (attribute bitpos { xsd:long }                                           <---
    ///             & attribute extends { VkTypeNameRef_t }?                                <---
    ///             & attribute alias {                                                     <---
    ///                   VkTypeNameRef_t | VkDefineOrEnumName_t
    ///               }))?
    ///         & ProtectAttr?                                                              <---
    ///         & ApiAttr?                                                                  <---
    ///         & attribute type { "uint8_t" | "uint32_t" | "uint64_t" | "float" }?         <---
    ///         & attribute name { VkDefineOrEnumName_t }                                   <---
    ///         & attribute deprecated { "aliased" | "unused" | "true" }?                   <---
    ///         & CommentAttr?                                                              <---
    ///     }
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseAttributeTag(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<bool, String> {
        let is_body_ = loop {
            let token_ = tokenizer.nextToken1();

            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "value" {
                let token_ = tokenizer.nextToken1();

                self.value = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "bitpos" {
                let token_ = tokenizer.nextToken1();

                self.bit_pos = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "extnumber" {
                let token_ = tokenizer.nextToken1();

                self.ext_number = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "offset" {
                let token_ = tokenizer.nextToken1();

                self.offset = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "dir" {
                let token_ = tokenizer.nextToken1();

                self.dir = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "extends" {
                let token_ = tokenizer.nextToken1();

                self.extends = token_.asRange();
            }
                
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "alias" {
                let token_ = tokenizer.nextToken1();

                self.alias = token_.asRange();
            }           

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "protect" {
                let token_ = tokenizer.nextToken1();

                self.protect = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "api" {
                let token_ = tokenizer.nextToken1();

                self.api = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "type" {
                let token_ = tokenizer.nextToken1();

                self.r#type = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "name" {
                let token_ = tokenizer.nextToken1();

                self.name = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(data.as_ptr()) } == "deprecated" {
                let token_ = tokenizer.nextToken1();

                self.deprecated = token_.asRange();
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
                return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
            }
        }; // let is_body_ = loop {

        Ok(is_body_)
    }
}