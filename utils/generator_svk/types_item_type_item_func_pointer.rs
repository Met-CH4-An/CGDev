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
use crate::types_item_type_item_func_pointer_item_proto::TypesItemTypeItemFuncPointerItemProto;
use crate::types_item_type_item_func_pointer_item_param::TypesItemTypeItemFuncPointerItemParam;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// TypeFuncpointer =
///     CommonTypeAttributes,
///     attribute category { "funcpointer" },
///     element proto {
///         mixed {
///             element type { TypeName_t }?,
///             element name { text }
///         }
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
pub(crate) struct TypesItemTypeItemFuncPointer {
    /// CommonTypeAttributes,
    pub(crate) common_type_attributes: CommonTypeAttributes,
    /// attribute category { "bitmask" },
    pub(crate) category: RangeInclusive<usize>,
    ///element proto {
    pub(crate) proto: TypesItemTypeItemFuncPointerItemProto,
    ///element proto {
    pub(crate) params: Vec<TypesItemTypeItemFuncPointerItemParam>,
}

impl TypesItemTypeItemFuncPointer {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn create() -> Self {
        Self {
            common_type_attributes: CommonTypeAttributes::create(),
            category: 1 ..= 0,
            proto: TypesItemTypeItemFuncPointerItemProto::create(),
            params: Vec::new(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub(crate) fn parse(&mut self, tokenizer: &mut Tokenizer<AVX2>, data: &[u8], is_body: bool) -> Result<(), String> {
        if is_body {
            loop {
                let token_ = tokenizer.nextToken1();

                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "proto"} {
                    let mut type_func_pointer_proto_ = TypesItemTypeItemFuncPointerItemProto::create();
                    type_func_pointer_proto_.parse(tokenizer, data)?;


                    self.proto = type_func_pointer_proto_;
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "param"} {
                    let mut type_func_pointer_param_ = TypesItemTypeItemFuncPointerItemParam::create();
                    type_func_pointer_param_.parse(tokenizer, data)?;

                    self.params.push(type_func_pointer_param_);
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(data.as_ptr()) == "/type"} {
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
        } // if is_body_ {

        Ok(())
    }
}