// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use utils__tokenizer_xml::{Tokenizer, AVX2};
use utils__tokenizer_xml::token::TokenType;
use crate::comment_elt::CommentElt;
use crate::type_body_type::TypeBodyType;
use crate::type_body_name::TypeBodyName;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) enum TypeBodyElementVariant {
    TYPE(TypeBodyType),
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
pub(crate) struct TypeBody {
    /// mixed { element type { TypeName_t } }*,
    pub(crate) type_body_types: Vec<TypeBodyType>,
    /// element name { attribute alias { text }?, TypeName_t }?
    pub(crate) type_body_name: TypeBodyName,
    /// mixed {
    ///     ( element type { TypeName_t }
    ///         | CommentElt
    ///     )
    /// }*
    pub(crate) type_body_element_variants: Vec<TypeBodyElementVariant>,
}

impl TypeBody {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create(tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<Self, String> {
        let mut self_ = Self {
            type_body_types: Vec::new(),
            type_body_name: TypeBodyName{
                alias: 1 ..= 0,
                prefix: 1 ..= 0,
                name: 1 ..= 0,
                postfix: 1 ..= 0,
            },
            type_body_element_variants: Vec::new(),
        };
        
        self_.parse(tokenizer, data)?;
        
        Ok(self_)
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
    fn parse(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8]) -> Result<(), String> {
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
                    let mut type_body_type_ = TypeBodyType::create(tokenizer, data)?;

                    // Если текстовый токен существует, забираем. Это префикс к type.
                    // If the text token exists, we retrieve it. This is the prefix for the type.
                    if let Some(v) = text.take() {
                        type_body_type_.prefix = v;
                    }

                    self.type_body_types.push(type_body_type_);
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "name"} {
                    let mut type_body_name_ = TypeBodyName::create(tokenizer, data)?;

                    // Если до этого был хоть один type. Берем последний. Это постфикс к type.
                    // If there was at least one `type` before this, we take the last one. This is a postfix for `type`.
                    if let Some(v) = self.type_body_types.last_mut() {
                        if let Some(v1) = text.take() {
                            v.postfix = v1;
                        }
                    }

                    // Если не было ни одного type - значит это префикс к name.
                    // If there was no “type,” then it's a prefix for “name.”
                    else if let Some(v1) = text.take() {
                        type_body_name_.prefix = v1;
                    }

                    self.type_body_name = type_body_name_;

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
                        let mut type_body_type_ = TypeBodyType::create(tokenizer, data)?;

                        // Если текстовый токен существует, забираем. Это префикс к type.
                        // If the text token exists, we retrieve it. This is the prefix for the type.
                        if let Some(v) = text.take() {
                            type_body_type_.prefix = v;
                        }

                        self.type_body_element_variants.push(TypeBodyElementVariant::TYPE(type_body_type_));
                    }

                    else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "comment"} {
                        let comment_elt_ = CommentElt::create(tokenizer, data)?;

                        self.type_body_element_variants.push(TypeBodyElementVariant::COMMENT_ELT(comment_elt_));

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
        
        else {
            return Err(String::from("Не валидный формат vk.xml. Invalid vk.xml format."));
        }        

        Ok(())
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