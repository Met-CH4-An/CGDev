// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use utils__tokenizer_xml::{Tokenizer, AVX2};
use utils__tokenizer_xml::token::TokenType;
use crate::comment_elt::{CommentElt, CommentEltView};
use crate::types_item_type_item_type_body_with_enum_item_type::{TypesItemTypeItemTypeBodyWithEnumItemType, TypesItemTypeItemTypeBodyWithEnumItemTypeView};
use crate::types_item_type_item_type_body_with_enum_item_name::{TypesItemTypeItemTypeBodyWithEnumItemName, TypesItemTypeItemTypeBodyWithEnumItemNameView};
use crate::types_item_type_item_type_body_with_enum_item_enum::{TypesItemTypeItemTypeBodyWithEnumItemEnum, TypesItemTypeItemTypeBodyWithEnumItemEnumView};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub enum TypesItemTypeItemBodyWithEnumViewVariant<'a> {
    TYPE(TypesItemTypeItemTypeBodyWithEnumItemTypeView<'a>),
    ENUM(TypesItemTypeItemTypeBodyWithEnumItemEnumView<'a>),
    COMMENT_ELT(CommentEltView<'a>),
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct TypesItemTypeItemTypeBodyWithEnumView<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) content: &'a TypesItemTypeItemTypeBodyWithEnum,
}

impl<'a> TypesItemTypeItemTypeBodyWithEnumView<'a> {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn types(&self) -> Vec<TypesItemTypeItemTypeBodyWithEnumItemTypeView> {
        self.content.types
            .iter()
            .map(|v| {
                TypesItemTypeItemTypeBodyWithEnumItemTypeView {
                    data: self.data,
                    content: &v
                }
            })
            .collect()
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn name(&self) -> TypesItemTypeItemTypeBodyWithEnumItemNameView {
        TypesItemTypeItemTypeBodyWithEnumItemNameView {
            data: self.data,
            content: &self.content.name
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn variants(&self) -> Vec<TypesItemTypeItemBodyWithEnumViewVariant> {
        self.content.variants
            .iter()
            .map(|v| {
                match v {
                    TypesItemTypeItemTypeBodyWithEnumVariant::TYPE(v) => {
                        let view_ = TypesItemTypeItemTypeBodyWithEnumItemTypeView {
                            data: self.data,
                            content: v
                        };

                        TypesItemTypeItemBodyWithEnumViewVariant::TYPE(view_)
                    }

                    TypesItemTypeItemTypeBodyWithEnumVariant::ENUM(v) => {
                        let view_ = TypesItemTypeItemTypeBodyWithEnumItemEnumView {
                            data: self.data,
                            content: v
                        };

                        TypesItemTypeItemBodyWithEnumViewVariant::ENUM(view_)
                    }

                    TypesItemTypeItemTypeBodyWithEnumVariant::COMMENT_ELT(v) => {
                        let view_ = CommentEltView {
                            data: self.data,
                            content: v
                        };

                        TypesItemTypeItemBodyWithEnumViewVariant::COMMENT_ELT(view_)
                    }
                }
            })
            .collect()
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) enum TypesItemTypeItemTypeBodyWithEnumVariant {
    TYPE(TypesItemTypeItemTypeBodyWithEnumItemType),
    ENUM(TypesItemTypeItemTypeBodyWithEnumItemEnum),
    COMMENT_ELT(CommentElt),
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// TypeBodyWithEnum =
///         mixed {
///           element type { TypeName_t }
///         }*,
///         mixed {
///           element name { attribute alias { text }?, TypeName_t }?
///         }?,
///         mixed {
///           ( element type { TypeName_t }
///             | element enum { VkDefineOrEnumName_t }
///             | CommentElt
///           )
///         }*
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct TypesItemTypeItemTypeBodyWithEnum {
    /// mixed { element type { TypeName_t } }*,
    pub(crate) types: Vec<TypesItemTypeItemTypeBodyWithEnumItemType>,
    /// element name { attribute alias { text }?, TypeName_t }?
    pub(crate) name: TypesItemTypeItemTypeBodyWithEnumItemName,
    /// mixed {
    ///     ( element type { TypeName_t }
    ///         | CommentElt
    ///     )
    /// }*
    pub(crate) variants: Vec<TypesItemTypeItemTypeBodyWithEnumVariant>,
}
impl TypesItemTypeItemTypeBodyWithEnum {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            types: Vec::new(),
            name: TypesItemTypeItemTypeBodyWithEnumItemName::create(),
            variants: Vec::new(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///                                                                <---
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn parse(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<(), String> {
        let mut text: Option<RangeInclusive<usize>> = None;

        // Лупаем первое множество mixed, пока не встретим <name>.
        // Iterate through the first set, `mixed`, until we find <name>.
        let is_end_= loop {
            let token_ = tokenizer.nextToken1();

            // Перед <type> может быть текст.
            // Text may precede <type>.
            if token_.asType() == TokenType::TEXT {
                text = Some(token_.asRange());
            }

            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "type"} {
                let mut new_ = TypesItemTypeItemTypeBodyWithEnumItemType::create();
                new_.parse(tokenizer, data)?;

                // Если текстовый токен существует, забираем. Это префикс к type.
                // If the text token exists, we retrieve it. This is the prefix for the type.
                if let Some(v) = text.take() {
                    new_.prefix = v;
                }

                self.types.push(new_);
            }

            else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "name"} {
                let mut new_ = TypesItemTypeItemTypeBodyWithEnumItemName::create();
                new_.parse(tokenizer, data)?;

                // Если до этого был хоть один type. Берем последний. Это постфикс к type.
                // If there was at least one `type` before this, we take the last one. This is a postfix for `type`.
                if let Some(v) = self.types.last_mut() {
                    if let Some(v1) = text.take() {
                        v.postfix = v1;
                    }
                }

                // Если не было ни одного type - значит это префикс к name.
                // If there was no “type,” then it's a prefix for “name.”
                else if let Some(v1) = text.take() {
                    new_.prefix = v1;
                }

                self.name = new_;

                // Останавливаем луп, потому что после <name> идет следующий mixed {}*
                // Stop the loop because <name> is followed by the next mixed {}*
                break false;
            }

            else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "/member"} {
                break true;
            }

            // Если встретился не валидный токен или конечный токен.
            // If an invalid token or final token is encountered.
            else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                return Err(String::from("Не валидный формат vk.xml. Invalid vk.xml format."));
            }
        }; // loop {

        if !is_end_ {
            // Лупаем второе множество mixed.
            // Let's examine the second set, “mixed.”
            loop {
                let token_ = tokenizer.nextToken1();

                if token_.asType() == TokenType::TEXT {
                    text = Some(token_.asRange());
                }

                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "type"} {
                    let mut new_ = TypesItemTypeItemTypeBodyWithEnumItemType::create();
                    new_.parse(tokenizer, data)?;

                    // Если текстовый токен существует, забираем. Это префикс к type.
                    // If the text token exists, we retrieve it. This is the prefix for the type.
                    if let Some(v) = text.take() {
                        new_.prefix = v;
                    }

                    self.variants.push(TypesItemTypeItemTypeBodyWithEnumVariant::TYPE(new_));
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "enum"} {
                    let mut new_ = TypesItemTypeItemTypeBodyWithEnumItemEnum::create();
                    new_.parse(tokenizer, data)?;

                    self.variants.push(TypesItemTypeItemTypeBodyWithEnumVariant::ENUM(new_));

                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "comment"} {
                    let mut comment_elt_ = CommentElt::create();
                    comment_elt_.parse(tokenizer, data)?;

                    self.variants.push(TypesItemTypeItemTypeBodyWithEnumVariant::COMMENT_ELT(comment_elt_));

                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "/member" } {
                    break;
                }

                // Если встретился не валидный токен или конечный токен.
                // If an invalid token or final token is encountered.
                else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                    return Err(String::from("Не валидный формат vk.xml. Invalid vk.xml format."));
                }
            }
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