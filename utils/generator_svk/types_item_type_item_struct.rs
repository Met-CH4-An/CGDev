// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use utils__tokenizer_xml::{Tokenizer, AVX2};
use utils__tokenizer_xml::token::TokenType;
use crate::types_item_type_item_common_type_attributes::{TypesItemTypeItemCommonTypeAttributes, TypesItemCommonTypeAttributesView};
use crate::comment_elt::{CommentElt, CommentEltView};
use crate::types_item_type_item_struct_item_member::{TypesItemTypeItemStructItemMember, TypesItemTypeItemStructItemMemberView};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub enum TypesItemTypeItemStructViewVariant<'a> {
    MEMBER(TypesItemTypeItemStructItemMemberView<'a>),
    COMMENT_ELT(CommentEltView<'a>)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct TypesItemTypeItemStructView<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) content: &'a TypesItemTypeItemStruct,
}

impl<'a> TypesItemTypeItemStructView<'a> {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn commonTypeAttributes(&self) -> TypesItemCommonTypeAttributesView {
        TypesItemCommonTypeAttributesView {
            data: self.data,
            content: &self.content.common_type_attributes
        }
    }
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn category(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.category.start() ..= *self.content.category.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn name(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.name.start() ..= *self.content.name.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn alias(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.alias.start() ..= *self.content.alias.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn returnedOnly(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.returned_only.start() ..= *self.content.returned_only.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn structExtends(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.struct_extends.start() ..= *self.content.struct_extends.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn allowDuplicate(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.allow_duplicate.start() ..= *self.content.allow_duplicate.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn requiredLimitType(&self) -> &str {
        unsafe {std::str::from_utf8_unchecked(&self.data[*self.content.required_limit_type.start() ..= *self.content.required_limit_type.end()]) }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn variants(&self) -> Vec<TypesItemTypeItemStructViewVariant> {
        self.content.variants
            .iter()
            .map(|v| {
                match v {
                    TypesItemTypeItemStructVariant::MEMBER(v) => {
                        let view_= TypesItemTypeItemStructItemMemberView {
                            data: self.data,
                            content: v
                        };

                        TypesItemTypeItemStructViewVariant::MEMBER(view_)
                    }

                    TypesItemTypeItemStructVariant::COMMENT_ELT(v) => {
                        let view_= CommentEltView {
                            data: self.data,
                            content: v
                        };

                        TypesItemTypeItemStructViewVariant::COMMENT_ELT(view_)
                    }
                }
                
            })
            .collect()
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// 
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) enum TypesItemTypeItemStructVariant {
    MEMBER(TypesItemTypeItemStructItemMember),
    COMMENT_ELT(CommentElt)
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// TypeStruct =
///     CommonTypeAttributes,
///     attribute category { "struct" | "union" },
///     (
///        ( NameAttr,
///          attribute alias { text }
///        )
///      | ( attribute name { TypeName_t },
///          attribute returnedonly { "true" }?,
///          attribute structextends { StringList_t }?,
///          attribute allowduplicate { "true" | "false" }?,
///          attribute requiredlimittype { "true" }?,
///          (
///            element member {
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
///            | CommentElt
///          )*
///        )
///     )
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct TypesItemTypeItemStruct {
    /// CommonTypeAttributes,
    pub(crate) common_type_attributes: TypesItemTypeItemCommonTypeAttributes,
    /// attribute category { "struct" | "union" },
    pub(crate) category: RangeInclusive<usize>,
    /// NameAttr,
    pub(crate) name: RangeInclusive<usize>,
    /// attribute alias { text }?
    pub(crate) alias: RangeInclusive<usize>,
    /// attribute returnedonly { "true" }?
    pub(crate) returned_only: RangeInclusive<usize>,
    /// attribute structextends { StringList_t }?
    pub(crate) struct_extends: RangeInclusive<usize>,
    /// attribute allowduplicate { "true" | "false" }?
    pub(crate) allow_duplicate: RangeInclusive<usize>,
    /// attribute requiredlimittype { "true" }?
    pub(crate) required_limit_type: RangeInclusive<usize>,
    ///
    pub(crate) variants: Vec<TypesItemTypeItemStructVariant>,
}

impl TypesItemTypeItemStruct {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            common_type_attributes: TypesItemTypeItemCommonTypeAttributes::create(),
            category: 1 ..= 0,
            name: 1 ..= 0,
            alias: 1 ..= 0,
            returned_only: 1 ..= 0,
            struct_extends: 1 ..= 0,
            allow_duplicate: 1 ..= 0,
            required_limit_type: 1 ..= 0,
            variants: Vec::new(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn parse(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<(), String> {
        loop {
            let token_ = tokenizer.nextToken1();

            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "member"} {
                let mut member_ = TypesItemTypeItemStructItemMember::create();
                member_.parse(tokenizer, data)?;

                self.variants.push(TypesItemTypeItemStructVariant::MEMBER(member_));
            }

            else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "comment"} {
                let mut comment_elt_ = CommentElt::create();
                comment_elt_.parse(tokenizer, data)?;

                self.variants.push(TypesItemTypeItemStructVariant::COMMENT_ELT(comment_elt_));
            }

            else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "/type"} {
                break;
            }

            // Если встретился не валидный токен или конечный токен.
            // If an invalid token or final token is encountered.
            else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                return Err(String::from("Не валидный формат vk.xml. Invalid vk.xml format."));
            }
        } // loop {
        
        Ok(())
    }

    
}