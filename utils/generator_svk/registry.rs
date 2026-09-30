// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Arc;
use utils__tokenizer_xml::{AVX2, Tokenizer};
use utils__tokenizer_xml::token::TokenType;
use crate::comment_elt::RegistryPrimitiveCommentElt;
use crate::common_type_attributes::RegistryCommonTypeAttributes;
use crate::registry_primitive_element_enum::RegistryPrimitiveElementEnum;
use crate::registry_primitive_element_name::RegistryPrimitiveElementName;
use crate::registry_primitive_element_type::RegistryPrimitiveElementType;
use crate::types::{RegistryTypes, RegistryTypesElementVariant};
use crate::type_base_type::{RegistryTypeBaseType};
use crate::registry_enums::{RegistryEnums};
use crate::registry_enum::RegistryEnum;
use crate::registry_type::{RegistryType, RegistryTypeElementVariant};
use crate::type_bitmask::RegistryTypeBitmask;
use crate::type_body::{RegistryTypeBody, RegistryTypeBodyElementVariant};
use crate::type_body_with_enum::{RegistryTypeBodyWithEnum, RegistryTypeBodyWithEnumElementVariant};
use crate::type_define::RegistryTypeDefine;
use crate::type_enum::RegistryTypeEnum;
use crate::type_func_pointer::RegistryTypeFuncpointer;
use crate::type_handle::RegistryTypeHandle;
use crate::type_include::RegistryTypeInclude;
use crate::type_requires::RegistryTypeRequires;
use crate::type_struct::{RegistryTypeStruct, RegistryTypeStructElementVariant};
use crate::registry_type_struct_member::RegistryTypeStructMember;

macro_rules! snake_case {
    ($name:expr) => {{
        let mut result_ = String::with_capacity($name.len() + 8);

        for (i_, c_) in $name.chars().enumerate() {
            if c_.is_uppercase() {
                if i_ != 0 {
                    result_.push('_');
                }

                result_.extend(c_.to_lowercase());
            } else if c_.is_ascii_digit() {
                if i_ != 0 && !result_.ends_with('_') {
                    result_.push('_');
                }

                result_.push(c_);
            } else {
                result_.push(c_);
            }
        }

        result_
    }};
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct Registry {
    /// Данные.
    /// Data.
    pub(crate) data: Arc<Vec<u8>>,
    /// Токенайзер.
    /// Tokenizer.
    pub(crate) tokenizer: Arc<Tokenizer<AVX2>>,
    pub(crate) registry_types_vec: Vec<RegistryTypes>,
    pub(crate) registry_enums_vec: Vec<RegistryEnums>,
    pub(crate) requires_cash: HashMap<u64, (usize, usize)>,
}

impl Registry {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn create() -> Self {
        Self {
            data: Arc::new(Vec::new()),
            tokenizer: Arc::new(Tokenizer::create()),
            registry_types_vec: Vec::new(),
            registry_enums_vec: Vec::new(),
            requires_cash: HashMap::new(),
        }
    }
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn createWithData(data: Arc<Vec<u8>>) -> Self {
        Self {
            data: data.clone(),
            tokenizer: Arc::new(Tokenizer::createWithData(data)),
            registry_types_vec: Vec::new(),
            registry_enums_vec: Vec::new(),
            requires_cash: HashMap::new(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Установить новые данные.
    /// Set new data.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn setData(&mut self, data: Arc<Vec<u8>>) {
        self.data = data;
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn build(&mut self) -> String {
        loop {
            let token_ = self.tokenizer.nextToken1();

            // Ищем <types>
            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "types" {
                //let registry_types_ = self.parseRegistryTypes().unwrap();

                //registry_.registry_types_vec.push(registry_types_);
            }

            // Ищем <enums>
            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "enums" {
                RegistryEnums::parse();
                //registry_.registry_enums_vec.push(registry_enums_);
            }

            if token_.asType() == TokenType::INVALID {
                break;
            }
        }

        String::new()
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные методы.
// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Registry {
    

    
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные ассоциированные функции.
// Private associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Generator {}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Приватные методы.
// Private methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Generator {
    

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Type =
    ///     element type {
    ///         TypeBasetype
    ///       | TypeBitmask
    ///       | TypeDefine
    ///       | TypeEnum
    ///       | TypeFuncpointer
    ///       | TypeHandle
    ///       | TypeInclude
    ///       | TypeRequires
    ///       | TypeStruct
    ///     }
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryType(&mut self) -> Result<RegistryType, String> {
        let mut output_ = RegistryType::create();

        // Анализируем непосредственно тег.
        // We analyze the tag directly.
        let is_body_;
        (output_, is_body_) = self.parseRegistryTypeTag(output_)?;

        if is_body_ {
            output_.registry_type_element_variant = match output_.registry_type_element_variant {
                RegistryTypeElementVariant::BASE_TYPE(mut type_) => {
                    type_ = self.parseRegistryTypeAsBaseType(type_)?;

                    RegistryTypeElementVariant::BASE_TYPE(type_)
                }

                RegistryTypeElementVariant::BITMASK(mut type_) => {
                    type_ = self.parseRegistryTypeAsBitmask(type_)?;

                    RegistryTypeElementVariant::BITMASK(type_)
                }

                RegistryTypeElementVariant::DEFINE(mut type_) => {
                    type_ = self.parseRegistryTypeAsDefine(type_)?;

                    RegistryTypeElementVariant::DEFINE(type_)
                }

                RegistryTypeElementVariant::ENUM(mut type_) => {
                    type_ = self.parseRegistryTypeAsEnum(type_)?;

                    RegistryTypeElementVariant::ENUM(type_)
                }

                RegistryTypeElementVariant::FUNC_POINTER(mut type_) => {
                    type_ = self.parseRegistryTypeAsFuncpointer(type_)?;

                    RegistryTypeElementVariant::FUNC_POINTER(type_)
                }

                RegistryTypeElementVariant::HANDLE(mut type_) => {
                    type_ = self.parseRegistryTypeAsHandle(type_)?;

                    RegistryTypeElementVariant::HANDLE(type_)
                }

                RegistryTypeElementVariant::INCLUDE(mut type_) => {
                    type_ = self.parseRegistryTypeAsInclude(type_)?;

                    RegistryTypeElementVariant::INCLUDE(type_)
                }

                RegistryTypeElementVariant::REQUIRES(mut type_) => {
                    type_ = self.parseRegistryTypeAsRequires(type_)?;

                    RegistryTypeElementVariant::REQUIRES(type_)
                }

                RegistryTypeElementVariant::STRUCT(mut type_) => {
                    type_ = self.parseRegistryTypeAsStruct(type_)?;

                    RegistryTypeElementVariant::STRUCT(type_)
                }

                type_ => {
                    type_
                }
            }
        } // if !is_body_ {

        Ok(output_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeAsBaseType(&mut self, mut input: RegistryTypeBaseType) -> Result<RegistryTypeBaseType, String> {
        input.registry_type_body = self.parseRegistryTypeBody()?;

        Ok(input)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeAsBitmask(&mut self, mut input: RegistryTypeBitmask) -> Result<RegistryTypeBitmask, String> {
        input.registry_type_body = self.parseRegistryTypeBody()?;

        Ok(input)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeAsDefine(&mut self, mut input: RegistryTypeDefine) -> Result<RegistryTypeDefine, String> {
        input.type_body = self.parseRegistryTypeBody()?;

        let str_ = unsafe {std::str::from_utf8_unchecked(&self.data.as_slice()[*input.type_body.registry_primitive_element_name.name_rng.start() ..= *input.type_body.registry_primitive_element_name.name_rng.end()])};
        println!("name = {}", str_);

        Ok(input)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeAsEnum(&mut self, _input: RegistryTypeEnum) -> Result<RegistryTypeEnum, String> {
        loop {
            let token_ = self.tokenizer.nextToken1();

            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "/type"} {
                break;
            }

            // Если встретился не валидный токен или конечный токен.
            // If an invalid token or final token is encountered.
            else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
            }
        } // loop {

        Ok(_input)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeAsFuncpointer(&mut self, _input: RegistryTypeFuncpointer) -> Result<RegistryTypeFuncpointer, String> {
        loop {
            let token_ = self.tokenizer.nextToken1();

            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "/type"} {
                break;
            }

            // Если встретился не валидный токен или конечный токен.
            // If an invalid token or final token is encountered.
            else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
            }
        } // loop {

        Ok(_input)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeAsHandle(&mut self, mut input: RegistryTypeHandle) -> Result<RegistryTypeHandle, String> {
        input.registry_type_body = self.parseRegistryTypeBody()?;

        Ok(input)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeAsInclude(&mut self, _input: RegistryTypeInclude) -> Result<RegistryTypeInclude, String> {
        loop {
            let token_ = self.tokenizer.nextToken1();

            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "/type"} {
                break;
            }

            // Если встретился не валидный токен или конечный токен.
            // If an invalid token or final token is encountered.
            else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
            }
        } // loop {

        Ok(_input)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeAsRequires(&mut self, _input: RegistryTypeRequires) -> Result<RegistryTypeRequires, String> {
        loop {
            let token_ = self.tokenizer.nextToken1();

            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "/type"} {
                break;
            }

            // Если встретился не валидный токен или конечный токен.
            // If an invalid token or final token is encountered.
            else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
            }
        } // loop {

        Ok(_input)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeAsStruct(&mut self, mut input: RegistryTypeStruct) -> Result<RegistryTypeStruct, String> {
        loop {
            let token_ = self.tokenizer.nextToken1();
            
            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "member"} {
                let registry_type_struct_member_ = self.parseRegistryTypeStructMember()?;

                input.element_variant_vec.push(RegistryTypeStructElementVariant::MEMBER(registry_type_struct_member_));
            }

            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "comment"} {
                let registry_primitive_comment_elt_ = self.parseRegistryPrimitiveCommentElt()?;

                input.element_variant_vec.push(RegistryTypeStructElementVariant::COMMENT_ELT(registry_primitive_comment_elt_));
            }

            // Конец.
            // End of.
            else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "/type"} {
                break;
            }

            // Если встретился не валидный токен или конечный токен.
            // If an invalid token or final token is encountered.
            else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
            }
        } // loop {

        Ok(input)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type ...>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeTag(&mut self, mut input: RegistryType) -> Result<(RegistryType, bool), String> {
        let mut api_rng_ = 1 ..= 0;
        let mut comment_rng_ = 1 ..= 0;
        let mut requires_rng_ = 1 ..= 0;
        let mut deprecated_rng_ = 1 ..= 0;

        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "api" {
                let token_ = self.tokenizer.nextToken1();

                api_rng_ = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "comment" {
                let token_ = self.tokenizer.nextToken1();

                comment_rng_ = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "requires" {
                let token_ = self.tokenizer.nextToken1();

                requires_rng_ = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "deprecated" {
                let token_ = self.tokenizer.nextToken1();

                deprecated_rng_ = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "category" {
                // Все что до category - RegistryCommonTypeAttributes.
                // Everything before category is RegistryCommonTypeAttributes.
                let registry_common_type_attributes_ = RegistryCommonTypeAttributes {
                    api_rng: api_rng_.clone(),
                    comment_rng: comment_rng_.clone(),
                    requires_rng: requires_rng_.clone(),
                    deprecated_rng: deprecated_rng_.clone(),
                };

                let token_ = self.tokenizer.nextToken1();

                let category_str_ = unsafe {std::str::from_utf8_unchecked(&self.data.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()])};

                if category_str_ == "basetype" {
                    let mut registry_type_base_type_ = RegistryTypeBaseType::create();
                    let is_body_;
                    (registry_type_base_type_, is_body_) = self.parseRegistryTypeTagAsBaseType(registry_type_base_type_)?;

                    registry_type_base_type_.registry_common_type_attributes = registry_common_type_attributes_;
                    registry_type_base_type_.category_rng = token_.asRange();

                    input.registry_type_element_variant = RegistryTypeElementVariant::BASE_TYPE(registry_type_base_type_);

                    break is_body_;
                }

                else if category_str_ == "bitmask" {
                    let mut registry_type_bitmask_ = RegistryTypeBitmask::create();
                    let is_body_;
                    (registry_type_bitmask_, is_body_) = self.parseRegistryTypeTagAsBitmask(registry_type_bitmask_)?;

                    registry_type_bitmask_.common_type_attributes = registry_common_type_attributes_;
                    registry_type_bitmask_.category_rng = token_.asRange();

                    input.registry_type_element_variant = RegistryTypeElementVariant::BITMASK(registry_type_bitmask_);

                    break is_body_;
                }

                else if category_str_ == "define" {
                    let mut registry_type_define_ = RegistryTypeDefine::create();
                    let is_body_;
                    (registry_type_define_, is_body_) = self.parseRegistryTypeTagAsDefine(registry_type_define_)?;

                    registry_type_define_.common_type_attributes = registry_common_type_attributes_;
                    registry_type_define_.category_rng = token_.asRange();

                    input.registry_type_element_variant = RegistryTypeElementVariant::DEFINE(registry_type_define_);

                    break is_body_;
                }

                else if category_str_ == "enum" {
                    let mut registry_type_enum_ = RegistryTypeEnum::create();
                    let is_body_;
                    (registry_type_enum_, is_body_) = self.parseRegistryTypeTagAsEnum(registry_type_enum_)?;

                    registry_type_enum_.common_type_attributes = registry_common_type_attributes_;
                    registry_type_enum_.category_rng = token_.asRange();

                    input.registry_type_element_variant = RegistryTypeElementVariant::ENUM(registry_type_enum_);

                    break is_body_;
                }

                else if category_str_ == "funcpointer" {
                    let mut registry_type_func_pointer_ = RegistryTypeFuncpointer::create();
                    let is_body_;
                    (registry_type_func_pointer_, is_body_) = self.parseRegistryTypeTagAsFuncpointer(registry_type_func_pointer_)?;

                    registry_type_func_pointer_.common_type_attributes = registry_common_type_attributes_;
                    registry_type_func_pointer_.category_rng = token_.asRange();

                    input.registry_type_element_variant = RegistryTypeElementVariant::FUNC_POINTER(registry_type_func_pointer_);

                    break is_body_;
                }

                else if category_str_ == "handle" {
                    let mut registry_type_handle_ = RegistryTypeHandle::create();
                    let is_body_;
                    (registry_type_handle_, is_body_) = self.parseRegistryTypeTagAsHandle(registry_type_handle_)?;

                    registry_type_handle_.common_type_attributes = registry_common_type_attributes_;
                    registry_type_handle_.category_rng = token_.asRange();

                    input.registry_type_element_variant = RegistryTypeElementVariant::HANDLE(registry_type_handle_);

                    break is_body_;
                }

                else if category_str_ == "include" {
                    let mut registry_type_include_ = RegistryTypeInclude::create();
                    let is_body_;
                    (registry_type_include_, is_body_) = self.parseRegistryTypeTagAsInclude(registry_type_include_)?;

                    registry_type_include_.common_type_attributes = registry_common_type_attributes_;
                    registry_type_include_.category_rng = token_.asRange();

                    input.registry_type_element_variant = RegistryTypeElementVariant::INCLUDE(registry_type_include_);

                    break is_body_;
                }

                else if category_str_ == "struct" || category_str_ == "union" {
                    let mut registry_type_struct_ = RegistryTypeStruct::create();
                    let is_body_;
                    (registry_type_struct_, is_body_) = self.parseRegistryTypeTagAsStruct(registry_type_struct_)?;

                    registry_type_struct_.common_type_attributes = registry_common_type_attributes_;
                    registry_type_struct_.category_rng = token_.asRange();

                    input.registry_type_element_variant = RegistryTypeElementVariant::STRUCT(registry_type_struct_);

                    break is_body_;
                }
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "category" {

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

        Ok((input, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type category="basetype" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeTagAsBaseType(&mut self, mut _input: RegistryTypeBaseType) -> Result<(RegistryTypeBaseType, bool), String> {
        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

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
                return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
            }
        }; // let is_body_ = loop {

        Ok((_input, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type category="bitmask" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeTagAsBitmask(&mut self, mut input: RegistryTypeBitmask) -> Result<(RegistryTypeBitmask, bool), String> {
        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "name" {
                let token_ = self.tokenizer.nextToken1();

                input.name_rng = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "alias" {
                let token_ = self.tokenizer.nextToken1();

                input.alias_rng = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "bitvalues" {
                let token_ = self.tokenizer.nextToken1();

                input.bitvalues_rng = token_.asRange();
            }

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
                return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
            }
        }; // let is_body_ = loop {

        Ok((input, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type category="define" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeTagAsDefine(&mut self, mut _input: RegistryTypeDefine) -> Result<(RegistryTypeDefine, bool), String> {
        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

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
                return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
            }
        }; // let is_body_ = loop {

        Ok((_input, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type category="enum" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeTagAsEnum(&mut self, mut _input: RegistryTypeEnum) -> Result<(RegistryTypeEnum, bool), String> {
        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

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
                return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
            }
        }; // let is_body_ = loop {

        Ok((_input, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type category="funcpointer" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeTagAsFuncpointer(&mut self, mut _input: RegistryTypeFuncpointer) -> Result<(RegistryTypeFuncpointer, bool), String> {
        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

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
                return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
            }
        }; // let is_body_ = loop {

        Ok((_input, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type category="handle" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeTagAsHandle(&mut self, mut input: RegistryTypeHandle) -> Result<(RegistryTypeHandle, bool), String> {
        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "name" {
                let token_ = self.tokenizer.nextToken1();

                input.name_rng = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "alias" {
                let token_ = self.tokenizer.nextToken1();

                input.alias_rng = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "parent" {
                let token_ = self.tokenizer.nextToken1();

                input.parent_rng = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "objtypeenum" {
                let token_ = self.tokenizer.nextToken1();

                input.objtypeenum_rng = token_.asRange();
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

        Ok((input, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type category="include" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeTagAsInclude(&mut self, mut input: RegistryTypeInclude) -> Result<(RegistryTypeInclude, bool), String> {
        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "name" {
                let token_ = self.tokenizer.nextToken1();

                input.name_rng = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "text" {
                let token_ = self.tokenizer.nextToken1();

                input.text_rng = token_.asRange();
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

        Ok((input, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type require="require" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeTagAsRequire(&mut self, mut input: RegistryTypeRequires) -> Result<(RegistryTypeRequires, bool), String> {
        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "api" {
                let token_ = self.tokenizer.nextToken1();

                input.api_rng = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "comment" {
                let token_ = self.tokenizer.nextToken1();

                input.comment_rng = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "deprecated" {
                let token_ = self.tokenizer.nextToken1();

                input.deprecated_rng = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "name" {
                let token_ = self.tokenizer.nextToken1();

                input.name_rng = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "requires" {
                let token_ = self.tokenizer.nextToken1();

                input.requires_rng = token_.asRange();
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
        }; // let is_close_ = loop {

        Ok((input, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type category="struct" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeTagAsStruct(&mut self, mut input: RegistryTypeStruct) -> Result<(RegistryTypeStruct, bool), String> {
        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "name" {
                let token_ = self.tokenizer.nextToken1();

                input.name_rng = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "alias" {
                let token_ = self.tokenizer.nextToken1();

                input.alias_rng = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "returnedonly" {
                let token_ = self.tokenizer.nextToken1();

                input.returned_only_rng = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "structextends" {
                let token_ = self.tokenizer.nextToken1();

                input.struct_extends_rng = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "allowduplicate" {
                let token_ = self.tokenizer.nextToken1();

                input.allow_duplicate_rng = token_.asRange();
            }

            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "requiredlimittype" {
                let token_ = self.tokenizer.nextToken1();

                input.required_limit_type_rng = token_.asRange();
            }

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
                return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
            }
        }; // let is_body_ = loop {

        Ok((input, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <member> ... </member>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeStructMember(&mut self) -> Result<RegistryTypeStructMember, String> {
        let mut output_ = RegistryTypeStructMember::create();

        // Анализируем непосредственно тег.
        // We analyze the tag directly.
        let is_body_;
        (output_, is_body_) = self.parseRegistryTypeStructMemberTag(output_)?;

        if is_body_ {
            let registry_type_body_with_enum_ = self.parseRegistryTypeBodyWithEnum()?;

            output_.registry_type_body_with_enum = registry_type_body_with_enum_;
        } // if is_body_ {

        Ok(output_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <member ...>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeStructMemberTag(&mut self, mut input: RegistryTypeStructMember) -> Result<(RegistryTypeStructMember, bool), String> {
        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

            // Ищем атрибут 'api'.
            // Search for the 'api' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "api" {
                let token_ = self.tokenizer.nextToken1();

                input.api_rng = token_.asRange();
            }

            // Ищем атрибут 'len'.
            // Search for the 'len' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "len" {
                let token_ = self.tokenizer.nextToken1();

                input.len_rng = token_.asRange();
            }

            // Ищем атрибут 'altlen'.
            // Search for the 'altlen' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "altlen" {
                let token_ = self.tokenizer.nextToken1();

                input.alt_len_rng = token_.asRange();
            }

            // Ищем атрибут 'stride'.
            // Search for the 'stride' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "stride" {
                let token_ = self.tokenizer.nextToken1();

                input.stride_rng = token_.asRange();
            }

            // Ищем атрибут 'externsync'.
            // Search for the 'externsync' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "externsync" {
                let token_ = self.tokenizer.nextToken1();

                input.extern_sync_rng = token_.asRange();
            }

            // Ищем атрибут 'optional'.
            // Search for the 'optional' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "optional" {
                let token_ = self.tokenizer.nextToken1();

                input.optional_attr_rng = token_.asRange();
            }

            // Ищем атрибут 'selector'.
            // Search for the 'selector' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "selector" {
                let token_ = self.tokenizer.nextToken1();

                input.selector_rng = token_.asRange();
            }

            // Ищем атрибут 'selection'.
            // Search for the 'selection' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "selection" {
                let token_ = self.tokenizer.nextToken1();

                input.selection_rng = token_.asRange();
            }

            // Ищем атрибут 'noautovalidity'.
            // Search for the 'noautovalidity' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "noautovalidity" {
                let token_ = self.tokenizer.nextToken1();

                input.no_auto_validity_attr_rng = token_.asRange();
            }

            // Ищем атрибут 'values'.
            // Search for the 'values' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "values" {
                let token_ = self.tokenizer.nextToken1();

                input.values_rng = token_.asRange();
            }

            // Ищем атрибут 'limittype'.
            // Search for the 'limittype' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "limittype" {
                let token_ = self.tokenizer.nextToken1();

                input.limit_type_rng = token_.asRange();
            }

            // Ищем атрибут 'objecttype'.
            // Search for the 'objecttype' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "objecttype" {
                let token_ = self.tokenizer.nextToken1();

                input.object_type_rng = token_.asRange();
            }

            // Ищем атрибут 'deprecated'.
            // Search for the 'deprecated' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "deprecated" {
                let token_ = self.tokenizer.nextToken1();

                input.deprecated_rng = token_.asRange();
            }

            // Ищем атрибут 'featurelink'.
            // Search for the 'featurelink' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "featurelink" {
                let token_ = self.tokenizer.nextToken1();

                input.feature_link_rng = token_.asRange();
            }

            // Ищем атрибут 'flagsextend'.
            // Search for the 'flagsextend' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "flagsextend" {
                let token_ = self.tokenizer.nextToken1();

                input.flags_extend_rng = token_.asRange();
            }

            // Ищем атрибут 'flagsextendmember'.
            // Search for the 'flagsextendmember' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "flagsextendmember" {
                let token_ = self.tokenizer.nextToken1();

                input.flags_extend_member_rng = token_.asRange();
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

        Ok((input, is_body_))
    }
    
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseFromExtensions(&mut self, mut registry: Registry) -> Option<Registry> {
        // Анализируем непосредственно тег 'extensions'.
        // We analyze the 'extensions' tag directly.
        let (comment_rng, is_close) = self.parseExtensionsTag()?;

        // Если тег 'extensions' не самозакрывающийся, лупаем до тех пор, пока не встретим '/extensions'.
        // If the 'extensions' tag is not self-closing, loop around until we encounter '/extensions'.
        if !is_close {
            loop {
                let token_ = self.tokenizer.nextToken1();

                // Внутренний тег 'extension'.
                // Internal tag 'extension'.
                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "extension" {
                    registry = self.parseFromExtension(registry)?;

                } // if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "extension" {

                // Конец 'extensions'.
                // End of 'extensions'.
                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "/extensions" {
                    break;
                }

                // Если встретился не валидный токен или конечный токен.
                // If an invalid token or final token is encountered.
                else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                    return None;
                }
            } // loop {
        }

        Some(registry)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseExtensionsTag(&mut self) -> Option<(RangeInclusive<usize>, bool)> {
        let mut comment_rng = 1 ..= 0;

        // Лупаем до тех пор, пока не встретим конец открывающего тега.
        // Loop until we encounter the end of the opening tag.
        let is_close_ = loop {
            let token_ = self.tokenizer.nextToken1();

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "comment" {
                let token_ = self.tokenizer.nextToken1();

                comment_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Если встретили просто закрывающийся конец тега ('>'), сообщаем что тег закрылся без самозакрытия.
            // If we encounter a simply closing end of a tag ('>'), we report that the tag closed without self-closing.
            else if token_.asType() == TokenType::TAG_END {
                break false;
            }

            // Если встретили самозакрывающийся конец тега ('/>'), сообщаем что тег с самозакрытием.
            // If we encounter a self-closing end of a tag ('/>'), we report that the tag is self-closing.
            else if token_.asType() == TokenType::TAG_END_CLOSE {
                break true;
            }

            // Если встретился не валидный токен или конечный токен.
            // If an invalid token or final token is encountered.
            else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                return None;
            }
        }; // let is_close_ = loop {

        Some((comment_rng, is_close_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseFromExtension(&mut self, mut registry: Registry) -> Option<Registry> {
        // Анализируем непосредственно тег <extension>.
        // We analyze the <extension> tag directly.
        let (name_rng_,
            number_rng_,
            author_rng_,
            contact_rng_,
            supported_rng_,
            ratified_rng_,
            nofeatures_rng_,
            comment_rng_,
            is_close_) = self.parseExtensionTag()?;

        // Если тег <extension> не самозакрывающийся, лупаем до тех пор, пока не встретим </extension>.
        // If the <extension> tag is not self-closing, loop around until we encounter </extension>.
        if !is_close_ {
            loop {
                let token_ = self.tokenizer.nextToken1();

                // Внутренний тег 'require'.
                // Internal tag 'require'.
                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "require" {
                    registry = self.parseFromRequire(registry)?;

                } // if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "require" {

                // Конец 'extension'.
                // End of 'extension'.
                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "/extension" {
                    break;
                }

                // Если встретился не валидный токен или конечный токен.
                // If an invalid token or final token is encountered.
                else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                    return None;
                }
            } // loop {
        }

        Some(registry)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseExtensionTag(&mut self) -> Option<(RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, bool)> {
        let mut name_rng = 1 ..= 0;
        let mut number_rng = 1 ..= 0;
        let mut author_rng = 1 ..= 0;
        let mut contact_rng = 1 ..= 0;
        let mut supported_rng = 1 ..= 0;
        let mut ratified_rng = 1 ..= 0;
        let mut nofeatures_rng = 1 ..= 0;
        let mut comment_rng = 1 ..= 0;

        // Лупаем до тех пор, пока не встретим конец открывающего тега.
        // Loop until we encounter the end of the opening tag.
        let is_close_ = loop {
            let token_ = self.tokenizer.nextToken1();

            // Ищем атрибут 'name'.
            // Search for the 'name' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "name" {
                let token_ = self.tokenizer.nextToken1();

                name_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Ищем атрибут 'number'.
            // Search for the 'number' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "number" {
                let token_ = self.tokenizer.nextToken1();

                number_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Ищем атрибут 'author'.
            // Search for the 'author' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "author" {
                let token_ = self.tokenizer.nextToken1();

                author_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Ищем атрибут 'contact'.
            // Search for the 'contact' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "contact" {
                let token_ = self.tokenizer.nextToken1();

                contact_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Ищем атрибут 'supported'.
            // Search for the 'supported' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "supported" {
                let token_ = self.tokenizer.nextToken1();

                supported_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Ищем атрибут 'ratified'.
            // Search for the 'ratified' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "ratified" {
                let token_ = self.tokenizer.nextToken1();

                ratified_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Ищем атрибут 'nofeatures'.
            // Search for the 'nofeatures' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "nofeatures" {
                let token_ = self.tokenizer.nextToken1();

                nofeatures_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "comment" {
                let token_ = self.tokenizer.nextToken1();

                comment_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Если встретили просто закрывающийся конец тега ('>'), сообщаем что тег закрылся без самозакрытия.
            // If we encounter a simply closing end of a tag ('>'), we report that the tag closed without self-closing.
            else if token_.asType() == TokenType::TAG_END {
                break false;
            }

            // Если встретили самозакрывающийся конец тега ('/>'), сообщаем что тег с самозакрытием.
            // If we encounter a self-closing end of a tag ('/>'), we report that the tag is self-closing.
            else if token_.asType() == TokenType::TAG_END_CLOSE {
                break true;
            }

            // Если встретился не валидный токен или конечный токен.
            // If an invalid token or final token is encountered.
            else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                return None;
            }
        }; // let is_close_ = loop {

        Some((name_rng, number_rng, author_rng, contact_rng, supported_rng, ratified_rng, nofeatures_rng, comment_rng, is_close_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseFromRequire(&mut self, mut registry: Registry) -> Option<Registry> {
        // Анализируем непосредственно тег 'extension'.
        // We analyze the 'extension' tag directly.
        let is_close_ = self.parseRequireTag()?;

        // Если тег 'enums' не самозакрывающийся, лупаем до тех пор, пока не встретим '/enums'.
        // If the 'enums' tag is not self-closing, loop around until we encounter '/enums'.
        if !is_close_ {
            loop {
                let token_ = self.tokenizer.nextToken1();

                // Внутренний тег 'enum'.
                // Internal tag 'enum'.
                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "enum" {
                    let (extnumber_rng_,
                        offset_rng_,
                        extends_rng_,
                        dir_rng_,
                        bitpos_rng_,
                        name_rng_,
                        comment_rng_,
                        value_rng,
                        alias_rng,
                        is_close_) = self.parseEnumTagExtended()?;

                    let extends_str = unsafe {std::str::from_utf8_unchecked(&self.data.as_slice()[*extends_rng_.start() ..= *extends_rng_.end()])};

                    /*let registry_enum_enumerator_extended_ = RegistryEnumEnumeratorExtended::s_createWithData(extnumber_rng_,
                                                                                                              offset_rng_,
                                                                                                              extends_rng_,
                                                                                                              dir_rng_,
                                                                                                              bitpos_rng_,
                                                                                                              name_rng_,
                                                                                                              comment_rng_,
                                                                                                              value_rng,
                                                                                                              alias_rng);

                    if let Some(registry_enum_) = registry.findEnumMut(extends_str) {
                        registry_enum_.extended_enumerators.push(registry_enum_enumerator_extended_);
                    }*/
                } // if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "enum" {

                // Конец 'require'.
                // End of 'require'.
                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "/require" {
                    break;
                }

                // Если встретился не валидный токен или конечный токен.
                // If an invalid token or final token is encountered.
                else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                    return None;
                }
            } // loop {
        }

        Some(registry)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRequireTag(&mut self) -> Option<(bool)> {
        // Лупаем до тех пор, пока не встретим конец открывающего тега.
        // Loop until we encounter the end of the opening tag.
        let is_close_ = loop {
            let token_ = self.tokenizer.nextToken1();

            // Если встретили просто закрывающийся конец тега ('>'), сообщаем что тег закрылся без самозакрытия.
            // If we encounter a simply closing end of a tag ('>'), we report that the tag closed without self-closing.
            if token_.asType() == TokenType::TAG_END {
                break false;
            }

            // Если встретили самозакрывающийся конец тега ('/>'), сообщаем что тег с самозакрытием.
            // If we encounter a self-closing end of a tag ('/>'), we report that the tag is self-closing.
            else if token_.asType() == TokenType::TAG_END_CLOSE {
                break true;
            }

            // Если встретился не валидный токен или конечный токен.
            // If an invalid token or final token is encountered.
            else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                return None;
            }
        }; // let is_close_ = loop {

        Some(is_close_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseEnumTagExtended(&mut self) -> Option<(RangeInclusive<usize>,
                                                  RangeInclusive<usize>,
                                                  RangeInclusive<usize>,
                                                  RangeInclusive<usize>,
                                                  RangeInclusive<usize>,
                                                  RangeInclusive<usize>,
                                                  RangeInclusive<usize>,
                                                  RangeInclusive<usize>,
                                                  RangeInclusive<usize>, bool)> {
        let mut extnumber_rng_ = 1 ..= 0;
        let mut offset_rng_ = 1 ..= 0;
        let mut extends_rng_ = 1 ..= 0;
        let mut dir_rng_ = 1 ..= 0;
        let mut bitpos_rng_ = 1 ..= 0;
        let mut name_rng_ = 1 ..= 0;
        let mut comment_rng_ = 1 ..= 0;
        let mut value_rng_ = 1 ..= 0;
        let mut alias_rng_ = 1 ..= 0;

        // Лупаем до тех пор, пока не встретим конец открывающего тега.
        // Loop until we encounter the end of the opening tag.
        let is_close_ = loop {
            let token_ = self.tokenizer.nextToken1();

            // Ищем атрибут 'extnumber'.
            // Search for the 'extnumber' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "extnumber" {
                let token_ = self.tokenizer.nextToken1();

                extnumber_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "extnumber" {

            // Ищем атрибут 'offset'.
            // Search for the 'offset' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "offset" {
                let token_ = self.tokenizer.nextToken1();

                offset_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "offset" {

            // Ищем атрибут 'extends'.
            // Search for the 'extends' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "extends" {
                let token_ = self.tokenizer.nextToken1();

                extends_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "extends" {

            // Ищем атрибут 'dir'.
            // Search for the 'dir' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "dir" {
                let token_ = self.tokenizer.nextToken1();

                dir_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "dir" {

            // Ищем атрибут 'bitpos'.
            // Search for the 'bitpos' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "bitpos" {
                let token_ = self.tokenizer.nextToken1();

                bitpos_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "bitpos" {

            // Ищем атрибут 'name'.
            // Search for the 'name' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "name" {
                let token_ = self.tokenizer.nextToken1();

                name_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "comment" {
                let token_ = self.tokenizer.nextToken1();

                comment_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Ищем атрибут 'value'.
            // Search for the 'value' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "value" {
                let token_ = self.tokenizer.nextToken1();

                value_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "value" {

            // Ищем атрибут 'alias'.
            // Search for the 'alias' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "alias" {
                let token_ = self.tokenizer.nextToken1();

                alias_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "alias" {

            // Если встретили просто закрывающийся конец тега ('>'), сообщаем что тег закрылся без самозакрытия.
            // If we encounter a simply closing end of a tag ('>'), we report that the tag closed without self-closing.
            else if token_.asType() == TokenType::TAG_END {
                break false;
            }

            // Если встретили самозакрывающийся конец тега ('/>'), сообщаем что тег с самозакрытием.
            // If we encounter a self-closing end of a tag ('/>'), we report that the tag is self-closing.
            else if token_.asType() == TokenType::TAG_END_CLOSE {
                break true;
            }

            // Если встретился не валидный токен или конечный токен.
            // If an invalid token or final token is encountered.
            else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                return None;
            }
        }; // let is_close_ = loop {

        Some((extnumber_rng_, offset_rng_, extends_rng_, dir_rng_, bitpos_rng_, name_rng_, comment_rng_, value_rng_, alias_rng_, is_close_))
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
    fn parseRegistryTypeBodyWithEnum(&mut self) -> Result<RegistryTypeBodyWithEnum, String> {
        let mut output_ = RegistryTypeBodyWithEnum::create();
        let mut text_rng = 1 ..= 0;

        // Лупаем первое множество mixed, пока не встретим <name>.
        // Iterate through the first set, `mixed`, until we find <name>.
        let is_end_= loop {
            let token_ = self.tokenizer.nextToken1();

            if token_.asType() == TokenType::TEXT {
                text_rng = token_.asRange();
            }

            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "type"} {
                let mut registry_primitive_element_type_ = self.parseRegistryPrimitiveElementType()?;

                registry_primitive_element_type_.prefix_rng = text_rng.clone();

                output_.registry_primitive_element_type_vec.push(registry_primitive_element_type_);

            }

            else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "name"} {
                let mut registry_primitive_element_name_ = self.parseRegistryPrimitiveElementName()?;

                if let Some(v) = output_.registry_primitive_element_type_vec.last_mut() {
                    v.postfix_rng = text_rng.clone();
                }

                else {
                    registry_primitive_element_name_.prefix_rng = text_rng.clone();
                }

                output_.registry_primitive_element_name = registry_primitive_element_name_;

                // Останавливаем луп, потому что после <name> идет следующий mixed {}*
                // Stop the loop because <name> is followed by the next mixed {}*
                break false;
            }

            else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "/type"} {
                break true;
            }

            // Если встретился не валидный токен или конечный токен.
            // If an invalid token or final token is encountered.
            else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
            }
        }; // loop {

        output_.registry_primitive_element_name.postfix_rng = text_rng.clone();

        if !is_end_ {
            // Лупаем второе множество mixed.
            // Let's examine the second set, “mixed.”
            loop {
                let token_ = self.tokenizer.nextToken1();

                if token_.asType() == TokenType::TEXT {
                    text_rng = token_.asRange();
                }

                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "type"} {
                    let mut registry_primitive_element_type_ = self.parseRegistryPrimitiveElementType()?;

                    registry_primitive_element_type_.prefix_rng = text_rng.clone();

                    output_.registry_type_body_with_enum_element_variant_vec.push(RegistryTypeBodyWithEnumElementVariant::TYPE(registry_primitive_element_type_));

                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "comment"} {
                    let registry_primitive_comment_elt_ = self.parseRegistryPrimitiveCommentElt()?;

                    output_.registry_type_body_with_enum_element_variant_vec.push(RegistryTypeBodyWithEnumElementVariant::COMMENT(registry_primitive_comment_elt_));

                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "enum"} {
                    let registry_primitive_element_enum_ = self.parseRegistryPrimitiveElementEnum()?;

                    output_.registry_type_body_with_enum_element_variant_vec.push(RegistryTypeBodyWithEnumElementVariant::ENUM(registry_primitive_element_enum_));

                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "/member" } {
                    break;
                }

                // Если встретился не валидный токен или конечный токен.
                // If an invalid token or final token is encountered.
                else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                    return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
                }
            }
        }

        Ok(output_)
    }

    

    

    

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// element enum { VkDefineOrEnumName_t }
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryPrimitiveElementEnum(&mut self) -> Result<RegistryPrimitiveElementEnum, String> {
        let mut output_ = RegistryPrimitiveElementEnum::create();

        // Анализируем непосредственно тег.
        // We analyze the tag directly.
        let is_body_;
        (output_, is_body_) = self.parseRegistryPrimitiveElementEnumTag(output_)?;

        if is_body_ {
            loop {
                let token_ = self.tokenizer.nextToken1();

                if token_.asType() == TokenType::TEXT {
                    output_.enum_rng = token_.asRange();
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "/enum" } {
                    break;
                }

                // Если встретился не валидный токен или конечный токен.
                // If an invalid token or final token is encountered.
                else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                    return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
                }
            } // loop {
        } // if is_body_ {

        Ok(output_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <enum ...>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryPrimitiveElementEnumTag(&mut self, _input: RegistryPrimitiveElementEnum) -> Result<(RegistryPrimitiveElementEnum, bool), String> {
        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

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
                return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
            }
        }; // let is_body_ = loop {

        Ok((_input, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// element name { attribute alias { text }?, TypeName_t }
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryPrimitiveElementName(&mut self) -> Result<RegistryPrimitiveElementName, String> {
        let mut output_ = RegistryPrimitiveElementName::create();

        // Анализируем непосредственно тег.
        // We analyze the tag directly.
        let is_body_;
        (output_, is_body_) = self.parseRegistryPrimitiveElementNameTag(output_)?;

        if is_body_ {
            loop {
                let token_ = self.tokenizer.nextToken1();

                if token_.asType() == TokenType::TEXT {
                    output_.name_rng = token_.asRange();
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "/name" } {
                    break;
                }

                // Если встретился не валидный токен или конечный токен.
                // If an invalid token or final token is encountered.
                else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                    return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
                }
            } // loop {
        } // if is_body_ {

        Ok(output_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <name ...>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryPrimitiveElementNameTag(&mut self, mut input: RegistryPrimitiveElementName) -> Result<(RegistryPrimitiveElementName, bool), String> {
        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data.as_ptr()) } == "alias" {
                let token_ = self.tokenizer.nextToken1();

                input.alias_rng = token_.asRange();
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

        Ok((input, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// element type { TypeName_t }
    /// RegistryElementType
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryPrimitiveElementType(&mut self) -> Result<RegistryPrimitiveElementType, String> {
        let mut output_ = RegistryPrimitiveElementType::create();

        // Анализируем непосредственно тег.
        // We analyze the tag directly.
        let is_body_;
        (output_, is_body_) = self.parseRegistryPrimitiveElementTypeTag(output_)?;

        if is_body_ {
            loop {
                let token_ = self.tokenizer.nextToken1();

                if token_.asType() == TokenType::TEXT {
                    output_.type_rng = token_.asRange();
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data.as_ptr()) == "/type" } {
                    break;
                }

                // Если встретился не валидный токен или конечный токен.
                // If an invalid token or final token is encountered.
                else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                    return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
                }
            } // loop {
        } // if is_body_ {

        Ok(output_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type ...>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryPrimitiveElementTypeTag(&mut self, _input: RegistryPrimitiveElementType) -> Result<(RegistryPrimitiveElementType, bool), String> {
        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

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
                return Err(String::from("Не валидный токен или конечный токен. Invalid token or end token."));
            }
        }; // let is_body_ = loop {

        Ok((_input, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn generateWvk(&self, data: &[u8], registry: &Registry) -> Result<String, String> {
        let mut output_wvk_ = String::new();

        let mut physical_device_properties_all_indices_ = Vec::<Vec<usize>>::new();

        registry.registry_types_vec
            .iter()
            .enumerate()
            .for_each(|(id_types_, types_)| {
                let mut physical_device_properties_indices_ = Vec::<usize>::new();

                types_.registry_types_element_variant_vec
                    .iter()
                    .enumerate()
                    .for_each(|(id_type_, types_element_)|{
                        match &types_element_ {
                            RegistryTypesElementVariant::TYPE(type_) => {
                                match &type_.registry_type_element_variant {
                                    RegistryTypeElementVariant::BASE_TYPE(type_) => {
                                        let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.registry_type_body.registry_primitive_element_name.name_rng.start() ..= *type_.registry_type_body.registry_primitive_element_name.name_rng.end()])};
                                        let type_str_ = if let Some(v) = type_.registry_type_body.registry_primitive_element_type_vec.first() {
                                            unsafe {std::str::from_utf8_unchecked(&data[*v.type_rng.start() ..= *v.type_rng.end()])}
                                        }
                                        else {
                                            ""
                                        };

                                        //let type_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.registry_type_body.registry_primitive_element_type_vec[0].type_rng.start() ..= *type_.registry_type_body.registry_primitive_element_type_vec[0].type_rng.end()])};
                                        //let comment_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.registry_type_body..comment_rng.start() ..= *type_.type_body.comment_rng.end()])};

                                        let type_str_ = if type_str_ == "uint32_t" {
                                            "u32"
                                        }

                                        else if type_str_ == "uint64_t" {
                                            "u64"
                                        }

                                        else {
                                            //return;
                                            "*mut std::ffi::c_void"
                                        };

                                        output_wvk_.push_str(&format!("pub type {} = {}; // \n\n", name_str_, type_str_));
                                    }

                                    RegistryTypeElementVariant::BITMASK(type_) => {
                                        let alias_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.alias_rng.start() ..= *type_.alias_rng.end()])};
                                        let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.name_rng.start() ..= *type_.name_rng.end()])};
                                        let name_body_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.registry_type_body.registry_primitive_element_name.name_rng.start() ..= *type_.registry_type_body.registry_primitive_element_name.name_rng.end()])};

                                        let type_str_ = if let Some(v) = type_.registry_type_body.registry_primitive_element_type_vec.first() {
                                            unsafe {std::str::from_utf8_unchecked(&data[*v.type_rng.start() ..= *v.type_rng.end()])}
                                        }
                                        else {
                                            ""
                                        };

                                        //let type_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.registry_type_body.registry_primitive_element_type_vec[0].type_rng.start() ..= *type_.registry_type_body.registry_primitive_element_type_vec[0].type_rng.end()])};
                                        //let comment_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.type_body.comment_rng.start() ..= *type_.type_body.comment_rng.end()])};

                                        if alias_str_.is_empty() {
                                            output_wvk_.push_str(&format!("#[derive(Copy, Clone, PartialEq)]\n"));
                                            output_wvk_.push_str(&format!("pub struct {}(pub {}); // \n", name_body_str_, type_str_));
                                            output_wvk_.push_str(&format!("impl std::ops::BitAnd for {} {{\n", name_body_str_));
                                            output_wvk_.push_str(&format!("\ttype Output = Self;\n\n"));
                                            output_wvk_.push_str(&format!("\tfn bitand(self, rhs: Self) -> Self {{\n"));
                                            output_wvk_.push_str(&format!("\t\treturn Self(self.0 & rhs.0);\n"));
                                            output_wvk_.push_str(&format!("\t}}\n"));
                                            output_wvk_.push_str(&format!("}}\n\n"));
                                        }

                                        else {
                                            output_wvk_.push_str(&format!("pub type {} = {}; // \n\n", name_str_, alias_str_));
                                        }
                                    }
                                    // #[repr(C)]
                                    // pub struct VkPhysicalDevice_T {
                                    //     _private: [u8; 0],
                                    // }
                                    RegistryTypeElementVariant::HANDLE(type_) => {
                                        let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.registry_type_body.registry_primitive_element_name.name_rng.start() ..= *type_.registry_type_body.registry_primitive_element_name.name_rng.end()])};
                                        let type_str_ = if let Some(v) = type_.registry_type_body.registry_primitive_element_type_vec.first() {
                                            unsafe {std::str::from_utf8_unchecked(&data[*v.type_rng.start() ..= *v.type_rng.end()])}
                                        }
                                        else {
                                            ""
                                        };

                                        //let type_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.registry_type_body.registry_primitive_element_type_vec[0].type_rng.start() ..= *type_.registry_type_body.registry_primitive_element_type_vec[0].type_rng.end()])};
                                        //let comment_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.type_body.comment_rng.start() ..= *type_.type_body.comment_rng.end()])};

                                        if name_str_.is_empty() {
                                            return;
                                        }
                                        
                                        output_wvk_.push_str(&format!("#[repr(C)]\n"));
                                        output_wvk_.push_str(&format!("pub struct {} {{\n", name_str_));
                                        output_wvk_.push_str(&format!("\tprivate: [u8; 0]\n"));
                                        output_wvk_.push_str(&format!("}}\n"));
                                    }
                                    //physical_device_properties_structs_
                                    RegistryTypeElementVariant::STRUCT(type_) => {
                                        let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.name_rng.start() ..= *type_.name_rng.end()])};
                                        let struct_extends_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.struct_extends_rng.start() ..= *type_.struct_extends_rng.end()])};

                                        if name_str_ == "VkLayerSettingEXT" {
                                            println!("VkLayerSettingEXT");
                                        }

                                        // Если структура является расширением для VkPhysicalDeviceProperties2. Запишем индексы в массив.
                                        // Дальше по этим структурам сгенерируем код для wvk_physical_device_x_properties::WvkPhysicalDeviceXProperties2.
                                        // If the structure is an extension of VkPhysicalDeviceProperties2, we'll store the indices in an array.
                                        // Next, we'll use these structures to generate code for wvk_physical_device_x_properties::WvkPhysicalDeviceXProperties2.
                                        if struct_extends_str_ == "VkPhysicalDeviceProperties2" {
                                            physical_device_properties_indices_.push(id_type_);
                                        }

                                        if name_str_.is_empty() {
                                            return;
                                        }
                                        output_wvk_.push_str(&format!("#[repr(C)]\n"));
                                        output_wvk_.push_str(&format!("pub struct {} {{\n", name_str_));

                                        type_.element_variant_vec
                                            .iter()
                                            .for_each(|v| {
                                                match v {
                                                    RegistryTypeStructElementVariant::MEMBER(member_) => {
                                                        let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*member_.registry_type_body_with_enum.registry_primitive_element_name.name_rng.start() ..= *member_.registry_type_body_with_enum.registry_primitive_element_name.name_rng.end()])};
                                                        let type_str_ = unsafe {std::str::from_utf8_unchecked(&data[*member_.registry_type_body_with_enum.registry_primitive_element_type_vec[0].type_rng.start() ..= *member_.registry_type_body_with_enum.registry_primitive_element_type_vec[0].type_rng.end()])};
                                                        let prefix_str_ = unsafe {std::str::from_utf8_unchecked(&data[*member_.registry_type_body_with_enum.registry_primitive_element_type_vec[0].prefix_rng.start() ..= *member_.registry_type_body_with_enum.registry_primitive_element_type_vec[0].prefix_rng.end()])};
                                                        let postfix_str_ = unsafe {std::str::from_utf8_unchecked(&data[*member_.registry_type_body_with_enum.registry_primitive_element_type_vec[0].postfix_rng.start() ..= *member_.registry_type_body_with_enum.registry_primitive_element_type_vec[0].postfix_rng.end()])};

                                                        if name_str_ == "ppEnabledExtensionNames" {
                                                            println!("stope");
                                                        }

                                                        let prefix_str_ = if prefix_str_.starts_with("const") {
                                                            "const"
                                                        }
                                                        else { "" };

                                                        let postfix_str_ = if postfix_str_.find("* const*").is_some() {
                                                            "* const*"
                                                        }
                                                        else if postfix_str_.find("*").is_some() {
                                                            "*"
                                                        }
                                                        else { "" };

                                                        let prefix_rust_str = if prefix_str_ == "const" && postfix_str_ == "*" {
                                                            "*const "
                                                        }
                                                        else if prefix_str_ == "const" && postfix_str_ == "* const*" {
                                                            "*const *const"
                                                        }
                                                        else if prefix_str_ == "" && postfix_str_ == "*" {
                                                            "*mut "
                                                        }
                                                        else {
                                                            ""
                                                        };

                                                        let name_str_ = if name_str_ == "type" {
                                                            "r#type"
                                                        }

                                                        else {
                                                            name_str_
                                                        };

                                                        let type_str_ = if type_str_.starts_with("PFN_") {
                                                            &format!("crate::svk_commands::{}", type_str_)
                                                        }
                                                        else {
                                                            type_str_
                                                        };

                                                        let type_rust_str_ = if type_str_ == "void" {
                                                            "std::ffi::c_void"
                                                        }

                                                        else if type_str_ == "char" {
                                                            "i8"
                                                        }

                                                        else if type_str_ == "int8_t" {
                                                            "i8"
                                                        }

                                                        else if type_str_ == "uint8_t" {
                                                            "u8"
                                                        }

                                                        else if type_str_ == "int16_t" {
                                                            "i16"
                                                        }

                                                        else if type_str_ == "uint16_t" {
                                                            "u16"
                                                        }

                                                        else if type_str_ == "int32_t" {
                                                            "i32"
                                                        }

                                                        else if type_str_ == "uint32_t" {
                                                            "u32"
                                                        }

                                                        else if type_str_ == "int64_t" {
                                                            "i64"
                                                        }

                                                        else if type_str_ == "uint64_t" {
                                                            "u64"
                                                        }

                                                        else if type_str_ == "float" {
                                                            "f32"
                                                        }

                                                        else if type_str_ == "double" {
                                                            "f64"
                                                        }

                                                        else if type_str_ == "size_t" {
                                                            "usize"
                                                        }

                                                        else if type_str_ == "int" {
                                                            "i32"
                                                        }

                                                        else if type_str_ == "unsigned int" {
                                                            "u32"
                                                        }

                                                        else {
                                                            type_str_
                                                        };

                                                        let type_rust_str_ = if let Some(RegistryTypeBodyWithEnumElementVariant::ENUM(enum_)) = member_.registry_type_body_with_enum.registry_type_body_with_enum_element_variant_vec.first() {
                                                            let enum_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.enum_rng.start() ..= *enum_.enum_rng.end()])};
                                                            let enum_str_ = format!("[{}; {} as usize]", type_rust_str_,enum_str_);
                                                            enum_str_
                                                        }

                                                        else {
                                                            type_rust_str_.to_string()
                                                        };

                                                        output_wvk_.push_str(&format!("\tpub {}: {} {},\n", name_str_, prefix_rust_str, type_rust_str_));
                                                    }

                                                    _ => {}
                                                }

                                            });

                                        output_wvk_.push_str(&format!("}}\n\n"));
                                    }

                                    _ => {}
                                }
                            }

                            RegistryTypesElementVariant::COMMENT_ELT(comment_elt_) => {

                            }
                        }
                    });

                physical_device_properties_all_indices_.push(physical_device_properties_indices_);
            });

        registry.registry_enums_vec
            .iter()
            .try_for_each(|enums_| -> Result<(), String> {
                let name_str = unsafe {std::str::from_utf8_unchecked(&data[*enums_.name_rng.start() ..= *enums_.name_rng.end()])};
                let type_str = unsafe {std::str::from_utf8_unchecked(&data[*enums_.type_rng.start() ..= *enums_.type_rng.end()])};
                let comment_str = unsafe {std::str::from_utf8_unchecked(&data[*enums_.comment_rng.start() ..= *enums_.comment_rng.end()])};

                if type_str == "enum" {

                    output_wvk_.push_str(&format!("pub struct {}(i32); //{}\n", name_str, comment_str));
                    output_wvk_.push_str(&format!("impl {} {{\n", name_str));

                    enums_.registry_enum_vec
                        .iter()
                        .for_each(|enum_| {
                            let alias_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.alias_rng.start() ..= *enum_.alias_rng.end()])};
                            let value_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.value_rng.start() ..= *enum_.value_rng.end()])};
                            let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.name_rng.start() ..= *enum_.name_rng.end()])};

                            if alias_str_.is_empty() {
                                output_wvk_.push_str(&format!("\tpub const {}: Self = Self({});\n", name_str_, value_str_));
                            }

                            else {
                                output_wvk_.push_str(&format!("\tpub const {}: Self = Self::{};\n", name_str_, alias_str_));
                            }
                        });

                    output_wvk_.push_str(&format!("}}\n\n"));
                }
                if type_str == "bitmask" {
                    let name_str__ = name_str.replace("FlagBits", "Flags");

                    output_wvk_.push_str(&format!("pub type {} = {}; //{}\n", name_str, name_str__, comment_str));
                    output_wvk_.push_str(&format!("impl {} {{\n", name_str));

                    let bitwidth_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enums_.bitwidth_rng.start() ..= *enums_.bitwidth_rng.end()])};

                    if bitwidth_str_.is_empty() {
                        enums_.registry_enum_vec
                            .iter()
                            .try_for_each(|enum_| -> Result<(), String> {
                                let alias_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.alias_rng.start() ..= *enum_.alias_rng.end()])};
                                let bitpos_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.bitpos_rng.start() ..= *enum_.bitpos_rng.end()])};
                                let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.name_rng.start() ..= *enum_.name_rng.end()])};

                                let bitpos_ = if bitpos_str_.is_empty() {
                                    0
                                }

                                else {
                                    let bitpos_ = bitpos_str_.parse::<u32>()
                                        .map_err(|e| format!("{}", e))?;

                                    1u32 << bitpos_
                                };

                                if alias_str_.is_empty() {
                                    output_wvk_.push_str(&format!("\tpub const {}: {} = {}({});\n", name_str_, name_str__, name_str__, bitpos_));
                                }

                                else {
                                    output_wvk_.push_str(&format!("\tpub const {}: {} = Self::{};\n", name_str_, name_str__, alias_str_));
                                }

                                Ok(())
                            })?;
                    }

                    else if bitwidth_str_ == "64" {
                        enums_.registry_enum_vec
                            .iter()
                            .try_for_each(|enum_| -> Result<(), String> {
                                let alias_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.alias_rng.start() ..= *enum_.alias_rng.end()])};
                                let bitpos_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.bitpos_rng.start() ..= *enum_.bitpos_rng.end()])};
                                let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.name_rng.start() ..= *enum_.name_rng.end()])};

                                let bitpos_ = if bitpos_str_.is_empty() {
                                    0
                                }

                                else {
                                    let bitpos_ = bitpos_str_.parse::<u64>()
                                        .map_err(|e| format!("{}", e))?;

                                    1u64 << bitpos_
                                };

                                if alias_str_.is_empty() {
                                    output_wvk_.push_str(&format!("\tpub const {}: {} = {}({});\n", name_str_, name_str__, name_str__, bitpos_));
                                }

                                else {
                                    output_wvk_.push_str(&format!("\tpub const {}: {} = Self::{};\n", name_str_, name_str__, alias_str_));
                                }

                                Ok(())
                            })?;
                    }

                    output_wvk_.push_str(&format!("}}\n\n"));
                }

                Ok(())
            })?;

        self.generateWvkPhysicalDeviceXProperties2(registry, physical_device_properties_all_indices_);

        Ok(output_wvk_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Функция генерирует содержимое для:
    /// struct WvkPhysicalDeviceXProperties2 {
    ///     ...
    /// }
    /// и
    /// pub fn create() -> Self {
    ///     Self {
    ///         ...
    ///     }
    /// }
    ///
    /// This function generates content for:
    /// struct WvkPhysicalDeviceXProperties2 {
    ///     ...
    /// }
    /// and
    /// pub fn create() -> Self {
    ///     Self {
    ///         ...
    ///     }
    /// }
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn generateWvkPhysicalDeviceXProperties2(&self, registry: &Registry, indices: Vec::<Vec<usize>>) {
        let mut struct_field_output_ = String::new();       // Содержимое структуры WvkPhysicalDeviceXProperties2.
        let mut struct_create_output_ = String::new();      // Содержимое функции WvkPhysicalDeviceXProperties2::create().

        // Итерируем индексы для RegistryTypes
        indices
            .iter()
            .enumerate()
            .for_each(|(id_types_, types_)| {
                // Итерируем индексы для RegistryType
                types_
                    .iter()
                    .for_each(|id_type_| {
                        let types_element_ = &registry.registry_types_vec[id_types_].registry_types_element_variant_vec[*id_type_];

                        // Индексы формируются извне и указывают конкретно на RegistryTypesElement::TYPE -> RegistryTypeType::TYPE_STRUCT.
                        match &types_element_ {
                            RegistryTypesElementVariant::TYPE(type_) => {
                                match &type_.registry_type_element_variant {
                                    RegistryTypeElementVariant::STRUCT(type_) => {
                                        let name_ = unsafe {std::str::from_utf8_unchecked(&self.data.as_slice()[*type_.name_rng.start() ..= *type_.name_rng.end()])};
                                        let name1_ = name_;
                                        let mut name_field_ = String::with_capacity(name_.len() * 2);

                                        // Преобразуем название структуры в имя для поля структуры. VkStructName10EXT -> vk_struct_name_10_ext

                                        // Префиксы.

                                        let (name_, prefix_) = if let Some(v) = name_.strip_prefix("Vk") {
                                            (v, "vk")
                                        }

                                        else {
                                            (name_, "")
                                        };

                                        // Суффиксы.

                                        let (name_, suffix_) = if let Some(v) = name_.strip_suffix("KHR") {
                                            (v, "_khr")
                                        }

                                        else if let Some(v) = name_.strip_suffix("EXT") {
                                            (v, "_ext")
                                        }

                                        else if let Some(v) = name_.strip_suffix("NV") {
                                            (v, "_nv")
                                        }

                                        else if let Some(v) = name_.strip_suffix("AMD") {
                                            (v, "_amd")
                                        }

                                        else if let Some(v) = name_.strip_suffix("VALVE") {
                                            (v, "_valve")
                                        }

                                        else if let Some(v) = name_.strip_suffix("GOOGLE") {
                                            (v, "_google")
                                        }

                                        else if let Some(v) = name_.strip_suffix("ANDROID") {
                                            (v, "_android")
                                        }

                                        else if let Some(v) = name_.strip_suffix("MSFT") {
                                            (v, "_msft")
                                        }

                                        else if let Some(v) = name_.strip_suffix("MESA") {
                                            (v, "_mesa")
                                        }

                                        else if let Some(v) = name_.strip_suffix("ARM") {
                                            (v, "_arm")
                                        }

                                        else if let Some(v) = name_.strip_suffix("QCOM") {
                                            (v, "_qcom")
                                        }

                                        else if let Some(v) = name_.strip_suffix("NVX") {
                                            (v, "_nvx")
                                        }

                                        else if let Some(v) = name_.strip_suffix("AMDX") {
                                            (v, "_amdx")
                                        }

                                        else if let Some(v) = name_.strip_suffix("OHOS") {
                                            (v, "_ohos")
                                        }

                                        else {
                                            (name_, "")
                                        };

                                        // Добавляем префикс.

                                        name_field_.push_str(prefix_);

                                        // Итерируем имя и заменяем [uppercase] на ['_' + lowercase]

                                        let mut chars_ = name_.chars().peekable();

                                        while let Some(v) = chars_.next() {
                                            if v == 'P'
                                                && chars_.next_if_eq(&'C').is_some()
                                                && chars_.next_if_eq(&'I').is_some() {

                                                name_field_.push_str("_pci");
                                            }

                                            else if v == 'I'
                                                && chars_.next_if_eq(&'D').is_some() {

                                                name_field_.push_str("_id");
                                            }

                                            else if v.is_ascii_uppercase() {
                                                name_field_.push('_');
                                                name_field_.push(v.to_ascii_lowercase());
                                            }

                                            else if v.is_ascii_digit() {
                                                name_field_.push('_');
                                                name_field_.push(v);
                                            }

                                            else {
                                                name_field_.push(v);
                                            }
                                        } // while let Some(v) = chars_.next() {

                                        name_field_.push_str(suffix_);

                                        match &type_.element_variant_vec[0] {
                                            RegistryTypeStructElementVariant::MEMBER(member_) => {
                                                let values_member_ = unsafe {std::str::from_utf8_unchecked(&self.data.as_slice()[*member_.values_rng.start() ..= *member_.values_rng.end()])};

                                                struct_field_output_.push_str(&format!("\tpub {}: Option<MaybeUninit<svk::{}>>,\n", name_field_, name1_));
                                                struct_create_output_.push_str(&format!("\t\t\t{}: Some(maybe_init!(svk::{}, svk::VkStructureType::{})),\n", name_field_, name1_, values_member_));
                                            }

                                            _ => {}
                                        }
                                    }

                                    _ => {}
                                }
                            }
                            _ => {}

                        } // match &types_element_ {
                    }); // .for_each(|id_type_| {
            }); // .for_each(|(id_types_, types_)| {

        println!("asdasdas");
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn generatingExtensionFiles(&self, registry: &Registry) {
        
    }
}

