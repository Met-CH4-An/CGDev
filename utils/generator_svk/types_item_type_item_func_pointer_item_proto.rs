// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use utils__tokenizer_xml::{Tokenizer, AVX2};
use utils__tokenizer_xml::token::TokenType;
use crate::types_item_type_item_func_pointer_item_proto_item_type::{TypesItemTypeItemFuncPointerItemProtoItemType, TypesItemTypeItemFuncPointerItemProtoItemTypeView};
use crate::types_item_type_item_func_pointer_item_proto_item_name::{TypesItemTypeItemFuncPointerItemProtoItemName, TypesItemTypeItemFuncPointerItemProtoItemNameView};

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct TypesItemTypeItemFuncPointerItemProtoView<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) content: &'a TypesItemTypeItemFuncPointerItemProto,
}

impl<'a> TypesItemTypeItemFuncPointerItemProtoView<'a> {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn r#type(&self) -> TypesItemTypeItemFuncPointerItemProtoItemTypeView {
        TypesItemTypeItemFuncPointerItemProtoItemTypeView {
            data: self.data,
            content: &self.content.r#type
        }
    }
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn name(&self) -> TypesItemTypeItemFuncPointerItemProtoItemNameView {
        TypesItemTypeItemFuncPointerItemProtoItemNameView {
            data: self.data,
            content: &self.content.name
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// TypeFuncpointer =
///     CommonTypeAttributes,
///     attribute category { "funcpointer" },
///     element proto {                                 <---
///         mixed {                                     <---
///             element type { TypeName_t }?,           <---
///             element name { text }                   <---
///         }                                           <---
///     },
///     element param {
///         ApiAttr?,
///         attribute len { text }?,
///         attribute altlen { text }?,
///         attribute stride { text }?,
///         attribute externsync { text }?,
///         OptionalAttr?,
///         attribute selector { text }?,
///         NoAutoValidityAttr?,
///         attribute objecttype { text }?,
///         attribute validstructs { VkTypeNameListRef_t }?,
///         mixed {
///             element type { TypeName_t }?,
///             element name { text }?
///         }
///     }*
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub(crate) struct TypesItemTypeItemFuncPointerItemProto {
    /// element type { TypeName_t }
    pub(crate) r#type: TypesItemTypeItemFuncPointerItemProtoItemType,
    /// element name { text }
    pub(crate) name: TypesItemTypeItemFuncPointerItemProtoItemName,
}

impl TypesItemTypeItemFuncPointerItemProto {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            r#type: TypesItemTypeItemFuncPointerItemProtoItemType::create(),
            name: TypesItemTypeItemFuncPointerItemProtoItemName::create(),
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
                    let mut type_func_pointer_proto_type_ = TypesItemTypeItemFuncPointerItemProtoItemType::create();
                    type_func_pointer_proto_type_.parse(tokenizer, data)?;

                    // Если текстовый токен существует, забираем. Это префикс к type.
                    // If the text token exists, we retrieve it. This is the prefix for the type.
                    if let Some(v) = text_.take() {
                        type_func_pointer_proto_type_.prefix = v;
                    }

                    self.r#type = type_func_pointer_proto_type_;
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "name"} {
                    let mut type_func_pointer_proto_name_ = TypesItemTypeItemFuncPointerItemProtoItemName::create();
                    type_func_pointer_proto_name_.parse(tokenizer, data)?;

                    // Если текстовый токен существует, забираем. Это префикс к type.
                    // If the text token exists, we retrieve it. This is the prefix for the type.
                    if let Some(v) = text_.take() {
                        self.r#type.postfix = v;
                    }

                    self.name = type_func_pointer_proto_name_;
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "/proto"} {
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
    fn parseAttributeTag(&mut self, tokenizer: &mut Tokenizer<AVX2>, _data: &[u8]) -> Result<bool, String> {
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