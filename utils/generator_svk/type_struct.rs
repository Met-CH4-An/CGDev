// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use utils__tokenizer_xml::{Tokenizer, AVX2};
use utils__tokenizer_xml::token::TokenType;
use crate::common_type_attributes::CommonTypeAttributes;
use crate::comment_elt::CommentElt;
use crate::types_item_type::TypesItemType;
use crate::type_struct_member::TypeStructMember;
use crate::types::TypesElementVariant;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Конструктор.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) enum TypeStructElementVariant {
    MEMBER(TypeStructMember),
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
pub(crate) struct TypeStruct {
    /// CommonTypeAttributes,
    pub(crate) common_type_attributes: CommonTypeAttributes,
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
    pub(crate) element_variants: Vec<TypeStructElementVariant>,
}

impl TypeStruct {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            common_type_attributes: CommonTypeAttributes::create(),
            category: 1 ..= 0,
            name: 1 ..= 0,
            alias: 1 ..= 0,
            returned_only: 1 ..= 0,
            struct_extends: 1 ..= 0,
            allow_duplicate: 1 ..= 0,
            required_limit_type: 1 ..= 0,
            element_variants: Vec::new(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn parse(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<(), String> {
        loop {
            let token_ = tokenizer.nextToken1();

            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "member"} {
                let mut member_ = TypeStructMember::create();
                member_.parse(tokenizer, data)?;

                self.element_variants.push(TypeStructElementVariant::MEMBER(member_));
            }

            else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "comment"} {
                let mut comment_elt_ = CommentElt::create();
                comment_elt_.parse(tokenizer, data)?;

                self.element_variants.push(TypeStructElementVariant::COMMENT_ELT(comment_elt_));
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