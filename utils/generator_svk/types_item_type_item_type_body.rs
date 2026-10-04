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
use crate::types_item_type_item_type_body_item_type::{TypesItemTypeItemTypeBodyItemType, TypesItemTypeItemTypeBodyItemTypeView};
use crate::types_item_type_item_type_body_item_name::{TypesItemTypeItemTypeBodyItemName, TypesItemTypeItemTypeBodyItemNameView};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub enum TypesItemTypeItemTypeBodyVariantView<'a> {
    TYPE(TypesItemTypeItemTypeBodyItemTypeView<'a>),
    COMMENT_ELT(CommentEltView<'a>),
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct TypesItemTypeItemTypeBodyView<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) content: &'a TypesItemTypeItemTypeBody,
}

impl<'a> TypesItemTypeItemTypeBodyView<'a> {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn types(&self) -> Vec<TypesItemTypeItemTypeBodyItemTypeView> {
        self.content.types
            .iter()
            .map(|v| {
                TypesItemTypeItemTypeBodyItemTypeView {
                    data: self.data,
                    content: v,
                }
            })
            .collect()
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn name(&self) -> TypesItemTypeItemTypeBodyItemNameView {
        TypesItemTypeItemTypeBodyItemNameView {
            data: self.data,
            content: &self.content.name,
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn variants(&self) -> Vec<TypesItemTypeItemTypeBodyVariantView> {
        self.content.contents
            .iter()
            .map(|v| {
                match v {
                    TypesItemTypeItemTypeBodyVariant::TYPE(v) => {
                        let view_= TypesItemTypeItemTypeBodyItemTypeView {
                            data: self.data,
                            content: v,
                        };
                        
                        TypesItemTypeItemTypeBodyVariantView::TYPE(view_)
                    }

                    TypesItemTypeItemTypeBodyVariant::COMMENT_ELT(v) => {
                        let view_= CommentEltView {
                            data: self.data,
                            content: v,
                        };

                        TypesItemTypeItemTypeBodyVariantView::COMMENT_ELT(view_)
                    }
                }
            })
            .collect()
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) enum TypesItemTypeItemTypeBodyVariant {
    TYPE(TypesItemTypeItemTypeBodyItemType),
    COMMENT_ELT(CommentElt),
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// TypeBody =
///         mixed {
///           element type { TypeName_t }
///         }*,
///         mixed {
///          element name { attribute alias { text }?, TypeName_t }?
///         }?,
///         mixed {
///           ( element type { TypeName_t }
///             | CommentElt
///           )
///         }*
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct TypesItemTypeItemTypeBody {
    /// mixed { element type { TypeName_t } }*,
    pub(crate) types: Vec<TypesItemTypeItemTypeBodyItemType>,
    /// element name { attribute alias { text }?, TypeName_t }?
    pub(crate) name: TypesItemTypeItemTypeBodyItemName,
    /// mixed {
    ///     ( element type { TypeName_t }
    ///         | CommentElt
    ///     )
    /// }*
    pub(crate) contents: Vec<TypesItemTypeItemTypeBodyVariant>,
}

impl TypesItemTypeItemTypeBody {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            types: Vec::new(),
            name: TypesItemTypeItemTypeBodyItemName::create(),
            contents: Vec::new(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn parse(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<(), String> {
        let is_body_ = (&mut *self).parseAttributeTag(tokenizer, data)?;
        
        if is_body_ {
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
                    let mut type_body_type_ = TypesItemTypeItemTypeBodyItemType::create();
                    type_body_type_.parse(tokenizer, data)?;

                    // Если текстовый токен существует, забираем. Это префикс к type.
                    // If the text token exists, we retrieve it. This is the prefix for the type.
                    if let Some(v) = text.take() {
                        type_body_type_.prefix = v;
                    }

                    self.types.push(type_body_type_);
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "name"} {
                    let mut type_body_name_ = TypesItemTypeItemTypeBodyItemName::create();
                    type_body_name_.parse(tokenizer, data)?;

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
                        type_body_name_.prefix = v1;
                    }

                    self.name = type_body_name_;

                    // Останавливаем луп, потому что после <name> идет следующий mixed {}*
                    // Stop the loop because <name> is followed by the next mixed {}*
                    break false;
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "/type"} {
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
                        let mut type_body_type_ = TypesItemTypeItemTypeBodyItemType::create();
                        type_body_type_.parse(tokenizer, data)?;

                        // Если текстовый токен существует, забираем. Это префикс к type.
                        // If the text token exists, we retrieve it. This is the prefix for the type.
                        if let Some(v) = text.take() {
                            type_body_type_.prefix = v;
                        }

                        self.contents.push(TypesItemTypeItemTypeBodyVariant::TYPE(type_body_type_));
                    }

                    else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "comment"} {
                        let mut comment_elt_ = CommentElt::create();
                        comment_elt_.parse(tokenizer, data)?;

                        self.contents.push(TypesItemTypeItemTypeBodyVariant::COMMENT_ELT(comment_elt_));

                    }

                    else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "/type" } {
                        break;
                    }

                    // Если встретился не валидный токен или конечный токен.
                    // If an invalid token or final token is encountered.
                    else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                        return Err(String::from("Не валидный формат vk.xml. Invalid vk.xml format."));
                    }
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