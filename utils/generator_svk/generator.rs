// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;
use std::rc::Rc;
use utils__tokenizer_xml::{AVX2, Tokenizer};
use utils__tokenizer_xml::token::TokenType;
use crate::registry::Registry;
use crate::registry_types::RegistryTypes;
use crate::registry_type_base_type::{RegistryTypeBaseType};
use crate::registry_enums::{RegistryEnums};
use crate::registry_enum::RegistryEnum;
use crate::registry_type::{RegistryType, RegistryTypeType};
use crate::registry_type_bitmask::RegistryTypeBitmask;
use crate::registry_type_body::RegistryTypeBody;
use crate::registry_type_define::RegistryTypeDefine;
use crate::registry_type_enum::RegistryTypeEnum;
use crate::registry_type_funcpointer::RegistryTypeFuncpointer;
use crate::registry_type_handle::RegistryTypeHandle;
use crate::registry_type_include::RegistryTypeInclude;
use crate::registry_type_requires::RegistryTypeRequires;
use crate::registry_type_struct::RegistryTypeStruct;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct Generator {
    /// Xml файл со спецификацией от Кроносов.
    /// XML file with specification from Kronos.
    data_rc: Rc<Vec<u8>>,
    // Токенайзер для получения токенов xml.
    // Tokenizer for obtaining xml tokens.
    tokenizer: Tokenizer<AVX2>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные ассоциированные функции.
// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Generator {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create() -> Self {
        Self {
            data_rc: Rc::new(Vec::new()),
            tokenizer: Tokenizer::s_create(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Конструктор.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_createWithData(data_rc: Rc<Vec<u8>>) -> Self {
        Self {
            data_rc: data_rc.clone(),
            tokenizer: Tokenizer::s_createWithData(data_rc.clone()),
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Публичные методы.
// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Generator {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Установить новые данные.
    /// Set new data.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn setData(&mut self, data_rc: Rc<Vec<u8>>) {
        self.data_rc = data_rc;
        self.tokenizer.setData(self.data_rc.clone());
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn build(&mut self) -> String {
        //let registry_enum_section_ = RegistryEnumSection::s_create();

        let mut registry_ = Registry::s_create();
        //registry_.enum_section.push(registry_enum_section_);

        //Крутим пока не закончатся токены. Пока не будет токен
        loop {
            let token_ = self.tokenizer.nextToken1();

            // Ищем <types>
            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "types" {
                let registry_types_ = self.parseTypes().unwrap();

                registry_.registry_types_vec.push(registry_types_);
            }

            // Ищем <enums>
            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "enums" {
                let registry_enums_ = self.parseEnums().unwrap();

                registry_.registry_enums_as_enum_vec.push(registry_enums_);
            }

            // Ищем <extensions>
            /*if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "extensions" {
                registry_ = self.parseFromExtensions(registry_).unwrap();

                //let name_str_ = unsafe {std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*registry_enum_.name_rng.start() ..= *registry_enum_.name_rng.end()])};

                //registry_.enum_section.last_mut().unwrap().pushEnum(name_str_, registry_enum_);
            }*/
            
            if token_.asType() == TokenType::INVALID {
                break;
            }
        }

        self.generateWvk(self.data_rc.as_slice(), &registry_);

        String::new()
    }
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
    /// <types ...> ... </types>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseTypes(&mut self) -> Option<RegistryTypes> {
        // Анализируем непосредственно тег.
        // We analyze the tag directly.
        let (mut registry_types_, is_body_) = self.parseTypesTag()?;

        // Лупаем пока не встретим закрывающий тег.
        // Loop until we reach the closing tag.
        if is_body_ {
            loop {
                let token_ = self.tokenizer.nextToken1();

                // Начинаем парсинг <type>.
                // Start parsing <type>.
                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) == "type"} {
                    let registry_type_ = self.parseType()?;

                    registry_types_.type_vec.push(registry_type_);
                }

                // Конец <types>.
                // End of <types>.
                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) == "/types"} {
                    break;
                }
            } // loop {
        } // if !is_close_ {

        Some(registry_types_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <types ...>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseTypesTag(&mut self) -> Option<(RegistryTypes, bool)> {
        let mut registry_types_ = RegistryTypes::s_create();

        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {
                let token_ = self.tokenizer.nextToken1();

                registry_types_.comment_rng = token_.asRange();
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
                return None;
            }
        }; // let is_close_ = loop {

        Some((registry_types_, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type ...> ... </type>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseType(&mut self) -> Option<RegistryType> {
        // Анализируем непосредственно тег.
        // We analyze the tag directly.
        let (mut registry_type_, is_body_) = self.parseTypeTag()?;

        let mut dummy_ = 1;

        // Лупаем пока не встретим закрывающий тег.
        // Loop until we reach the closing tag.
        if is_body_ {
            loop {
                let token_ = self.tokenizer.nextToken1();

                //
                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) == "type"} {
                    let registry_type_body_ = self.parseTypeBody()?;

                    continue;
                }

                // Конец <types>.
                // End of <types>.
                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) == "/type"} {
                    break;
                }

                //if dummy_ == 0 {break;}
            } // loop {
        } // if !is_body_ {

        Some(registry_type_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type ...>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseTypeTag(&mut self) -> Option<(RegistryType, bool)> {
        let mut registry_type_ = RegistryType::s_create();

        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

            // Ищем атрибут 'requires'.
            // Search for the 'requires' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "requires" {
                let token_ = self.tokenizer.nextToken1();

                //let (mut registry_type_requires_, is_body_) = self.parseTypeTagAsRequire()?;

                //registry_type_requires_.requires_rng = token_.asRange();

                //registry_type_.r#type = RegistryTypeType::TYPE_REQUIRES(registry_type_requires_);

                //break is_body_;
            }

            // Ищем атрибут 'category'.
            // Search for the 'category' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "category" {
                let token_ = self.tokenizer.nextToken1();

                let category_str_ = unsafe {std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()])};
                println!("{}", category_str_);

                if category_str_ == "basetype" {
                    let (mut registry_type_basetype_, is_body_) = self.parseTypeTagAsBaseType()?;

                    registry_type_basetype_.category_rng = token_.asRange();

                    registry_type_.r#type = RegistryTypeType::TYPE_BASE_TYPE(registry_type_basetype_);

                    break is_body_;
                }

                else if category_str_ == "bitmask" {
                    let (mut registry_type_bitmask_, is_body_) = self.parseTypeTagAsBitmask()?;

                    registry_type_bitmask_.category_rng = token_.asRange();

                    registry_type_.r#type = RegistryTypeType::TYPE_BITMASK(registry_type_bitmask_);

                    break is_body_;
                }

                else if category_str_ == "define" {
                    let (mut registry_type_define_, is_body_) = self.parseTypeTagAsDefine()?;

                    registry_type_define_.category_rng = token_.asRange();

                    registry_type_.r#type = RegistryTypeType::TYPE_DEFINE(registry_type_define_);

                    break is_body_;
                }

                else if category_str_ == "enum" {
                    let (mut registry_type_enum_, is_body_) = self.parseTypeTagAsEnum()?;

                    registry_type_enum_.category_rng = token_.asRange();

                    registry_type_.r#type = RegistryTypeType::TYPE_ENUM(registry_type_enum_);

                    break is_body_;
                }

                else if category_str_ == "funcpointer" {
                    let (mut registry_type_funcpointer_, is_body_) = self.parseTypeTagAsFuncpointer()?;

                    registry_type_funcpointer_.category_rng = token_.asRange();

                    registry_type_.r#type = RegistryTypeType::TYPE_FUNCPOINTER(registry_type_funcpointer_);

                    break is_body_;
                }

                else if category_str_ == "handle" {
                    let (mut registry_type_handle_, is_body_) = self.parseTypeTagAsHandle()?;

                    registry_type_handle_.category_rng = token_.asRange();

                    registry_type_.r#type = RegistryTypeType::TYPE_HANDLE(registry_type_handle_);

                    break is_body_;
                }

                else if category_str_ == "include" {
                    let (mut registry_type_include_, is_body_) = self.parseTypeTagAsInclude()?;

                    registry_type_include_.category_rng = token_.asRange();

                    registry_type_.r#type = RegistryTypeType::TYPE_INCLUDE(registry_type_include_);

                    break is_body_;
                }

                else if category_str_ == "struct" {
                    let (mut registry_type_struct_, is_body_) = self.parseTypeTagAsStruct()?;

                    registry_type_struct_.category_rng = token_.asRange();

                    registry_type_.r#type = RegistryTypeType::TYPE_STRUCT(registry_type_struct_);

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
                return None;
            }
        }; // let is_close_ = loop {

        Some((registry_type_, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type category="basetype" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseTypeTagAsBaseType(&mut self) -> Option<(RegistryTypeBaseType, bool)> {
        let mut registry_type_base_type_ = RegistryTypeBaseType::s_create();

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
                return None;
            }
        }; // let is_body_ = loop {

        Some((registry_type_base_type_, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type category="bitmask" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseTypeTagAsBitmask(&mut self) -> Option<(RegistryTypeBitmask, bool)> {
        let mut registry_type_bitmask_ = RegistryTypeBitmask::s_create();

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
                return None;
            }
        }; // let is_body_ = loop {

        Some((registry_type_bitmask_, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type category="define" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseTypeTagAsDefine(&mut self) -> Option<(RegistryTypeDefine, bool)> {
        let mut registry_type_define_ = RegistryTypeDefine::s_create();

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
                return None;
            }
        }; // let is_body_ = loop {

        Some((registry_type_define_, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type category="enum" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseTypeTagAsEnum(&mut self) -> Option<(RegistryTypeEnum, bool)> {
        let mut registry_type_enum_ = RegistryTypeEnum::s_create();

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
                return None;
            }
        }; // let is_body_ = loop {

        Some((registry_type_enum_, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type category="funcpointer" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseTypeTagAsFuncpointer(&mut self) -> Option<(RegistryTypeFuncpointer, bool)> {
        let mut registry_type_funcpointer_ = RegistryTypeFuncpointer::s_create();

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
                return None;
            }
        }; // let is_body_ = loop {

        Some((registry_type_funcpointer_, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type category="handle" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseTypeTagAsHandle(&mut self) -> Option<(RegistryTypeHandle, bool)> {
        let mut registry_type_handle_ = RegistryTypeHandle::s_create();

        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

            // Ищем атрибут 'name'.
            // Search for the 'name' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {
                let token_ = self.tokenizer.nextToken1();

                registry_type_handle_.name_rng = token_.asRange();
            }

            // Ищем атрибут 'alias'.
            // Search for the 'alias' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "alias" {
                let token_ = self.tokenizer.nextToken1();

                registry_type_handle_.alias_rng = token_.asRange();
            }

            // Ищем атрибут 'parent'.
            // Search for the 'parent' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "parent" {
                let token_ = self.tokenizer.nextToken1();

                registry_type_handle_.parent_rng = token_.asRange();
            }

            // Ищем атрибут 'objtypeenum'.
            // Search for the 'objtypeenum' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "objtypeenum" {
                let token_ = self.tokenizer.nextToken1();

                registry_type_handle_.objtypeenum_rng = token_.asRange();
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
                return None;
            }
        }; // let is_body_ = loop {

        Some((registry_type_handle_, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type category="include" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseTypeTagAsInclude(&mut self) -> Option<(RegistryTypeInclude, bool)> {
        let mut registry_type_include_ = RegistryTypeInclude::s_create();

        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

            // Ищем атрибут 'name'.
            // Search for the 'name' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {
                let token_ = self.tokenizer.nextToken1();

                registry_type_include_.name_rng = token_.asRange();
            }

            // Ищем атрибут 'text'.
            // Search for the 'text' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "text" {
                let token_ = self.tokenizer.nextToken1();

                registry_type_include_.text_rng = token_.asRange();
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
                return None;
            }
        }; // let is_body_ = loop {

        Some((registry_type_include_, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type require="require" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseTypeTagAsRequire(&mut self) -> Option<(RegistryTypeRequires, bool)> {
        let mut registry_type_requires_ = RegistryTypeRequires::s_create();

        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

            // Ищем атрибут 'api'.
            // Search for the 'api' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "api" {
                let token_ = self.tokenizer.nextToken1();

                registry_type_requires_.api_rng = token_.asRange();
            }

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {
                let token_ = self.tokenizer.nextToken1();

                registry_type_requires_.comment_rng = token_.asRange();
            }

            // Ищем атрибут 'deprecated'.
            // Search for the 'deprecated' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "deprecated" {
                let token_ = self.tokenizer.nextToken1();

                registry_type_requires_.deprecated_rng = token_.asRange();
            }

            // Ищем атрибут 'name'.
            // Search for the 'name' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {
                let token_ = self.tokenizer.nextToken1();

                registry_type_requires_.name_rng = token_.asRange();
            }

            // Ищем атрибут 'requires'.
            // Search for the 'requires' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "requires" {
                let token_ = self.tokenizer.nextToken1();

                registry_type_requires_.requires_rng = token_.asRange();
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
                return None;
            }
        }; // let is_close_ = loop {

        Some((registry_type_requires_, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type category="struct" ... >
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseTypeTagAsStruct(&mut self) -> Option<(RegistryTypeStruct, bool)> {
        let mut registry_type_struct_ = RegistryTypeStruct::s_create();

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
                return None;
            }
        }; // let is_body_ = loop {

        Some((registry_type_struct_, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <type>type</type> <name>name</name>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseTypeBody(&mut self) -> Option<RegistryTypeBody> {
        let mut registry_type_body_ = RegistryTypeBody::s_create();

        self.tokenizer.nextToken1(); // >
        let token_ = self.tokenizer.nextToken1();

        let str_ = unsafe {std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()])};
        println!("{}", str_);

        if token_.asType() != TokenType::TEXT {
            return None;
        }

        registry_type_body_.type_rng = token_.asRange();

        let token_ = self.tokenizer.nextToken1(); // </
        let str_ = unsafe {std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()])};
        println!("{}", str_);
        let token_ = self.tokenizer.nextToken1();
        let str_ = unsafe {std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()])};
        println!("{}", str_);

        if !(token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) == "/type" }) {
            return None;
        }

        loop {
            let token_ = self.tokenizer.nextToken1();
            let str_ = unsafe {std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()])};
            println!("{}", str_);

            //
            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) == "name" } {
                let token_ = self.tokenizer.nextToken1(); // >
                let str_ = unsafe {std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()])};
                println!("{}", str_);
                let token_ = self.tokenizer.nextToken1();
                let str_ = unsafe {std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()])};
                println!("{}", str_);

                if token_.asType() != TokenType::TEXT {
                    return None;
                }

                registry_type_body_.name_rng = token_.asRange();

                self.tokenizer.nextToken1(); // </
                let token_ = self.tokenizer.nextToken1();

                if !(token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) == "/name" }) {
                    return None;
                }

                break;
            }
        }

        Some(registry_type_body_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <enums ...> ... </enums>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseEnums(&mut self) -> Option<RegistryEnums> {
        let mut registry_enums_ = RegistryEnums::s_create();

        // Анализируем непосредственно тег.
        // We analyze the tag directly.
        let is_body_;
        (registry_enums_, is_body_) = self.parseEnumsTag(registry_enums_)?;

        if is_body_ {
            loop {
                let token_ = self.tokenizer.nextToken1();

                // Ищем атрибут 'enum'.
                // Search for the 'enum' attribute.
                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "enum" {
                    let registry_enum_ = self.parseEnum()?;

                    registry_enums_.registry_enum_vec.push(registry_enum_);
                }
                
                // Конец.
                // End of.
                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "/enums" {
                    break;
                }

                // Если встретился не валидный токен или конечный токен.
                // If an invalid token or final token is encountered.
                else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                    return None;
                }
            }
        }

        Some(registry_enums_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <enums ...>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseEnumsTag(&mut self, mut registry_enums: RegistryEnums) -> Option<(RegistryEnums, bool)> {
        // Лупаем до тех пор, пока не встретим конец открывающего тега.
        // Loop until we encounter the end of the opening tag.
        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();

            // Ищем атрибут 'name'.
            // Search for the 'name' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {
                let token_ = self.tokenizer.nextToken1();

                registry_enums.name_rng = token_.asRange();
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {

            // Ищем атрибут 'type'.
            // Search for the 'type' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "type" {
                let token_ = self.tokenizer.nextToken1();

                registry_enums.type_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "type" {

            // Ищем атрибут 'bitwidth'.
            // Search for the 'bitwidth' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "bitwidth" {
                let token_ = self.tokenizer.nextToken1();

                registry_enums.bitwidth_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "bitwidth" {

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {
                let token_ = self.tokenizer.nextToken1();

                registry_enums.comment_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

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
                return None;
            }
        }; // let is_close_ = loop {

        Some((registry_enums, is_body_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <enum ...> ... </enum>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseEnum(&mut self) -> Option<RegistryEnum> {
        let mut registry_enum_ = RegistryEnum::s_create();

        // Анализируем непосредственно тег.
        // We analyze the tag directly.
        let is_body_;
        (registry_enum_, is_body_) = self.parseEnumTag(registry_enum_)?;

        if is_body_ {
            loop {}
        }

        Some(registry_enum_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <enum ...>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseEnumTag(&mut self, mut registry_enum: RegistryEnum) -> Option<(RegistryEnum, bool)> {
        // Лупаем до тех пор, пока не встретим конец открывающего тега.
        // Loop until we encounter the end of the opening tag.
        let is_body_ = loop {
            let token_ = self.tokenizer.nextToken1();
            
            // Ищем атрибут 'value'.
            // Search for the 'value' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "value" {
                let token_ = self.tokenizer.nextToken1();

                registry_enum.name_rng = token_.asRange();
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "value" {

            // Ищем атрибут 'bitpos'.
            // Search for the 'bitpos' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "bitpos" {
                let token_ = self.tokenizer.nextToken1();

                registry_enum.bitpos_rng = token_.asRange();
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "bitpos" {

            // Ищем атрибут 'offset'.
            // Search for the 'offset' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "offset" {
                let token_ = self.tokenizer.nextToken1();

                registry_enum.offset_rng = token_.asRange();
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "offset" {

            // Ищем атрибут 'dir'.
            // Search for the 'dir' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "dir" {
                let token_ = self.tokenizer.nextToken1();

                registry_enum.dir_rng = token_.asRange();
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "dir" {

            // Ищем атрибут 'alias'.
            // Search for the 'alias' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "alias" {
                let token_ = self.tokenizer.nextToken1();

                registry_enum.alias_rng = token_.asRange();
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "alias" {

            // Ищем атрибут 'extends'.
            // Search for the 'extends' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "extends" {
                let token_ = self.tokenizer.nextToken1();

                registry_enum.extends_rng = token_.asRange();
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "extends" {

            // Ищем атрибут 'protect'.
            // Search for the 'protect' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "protect" {
                let token_ = self.tokenizer.nextToken1();

                registry_enum.protect_rng = token_.asRange();
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "protect" {

            // Ищем атрибут 'api'.
            // Search for the 'api' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "api" {
                let token_ = self.tokenizer.nextToken1();

                registry_enum.api_rng = token_.asRange();
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "api" {

            // Ищем атрибут 'type'.
            // Search for the 'type' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "type" {
                let token_ = self.tokenizer.nextToken1();

                registry_enum.type_rng = token_.asRange();
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "type" {

            // Ищем атрибут 'name'.
            // Search for the 'name' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {
                let token_ = self.tokenizer.nextToken1();

                registry_enum.name_rng = token_.asRange();
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {

            // Ищем атрибут 'deprecated'.
            // Search for the 'deprecated' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "deprecated" {
                let token_ = self.tokenizer.nextToken1();

                registry_enum.deprecated_rng = token_.asRange();
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "deprecated" {

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {
                let token_ = self.tokenizer.nextToken1();

                registry_enum.comment_rng = token_.asRange();
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {
            

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
                return None;
            }
        }; // let is_close_ = loop {

        Some((registry_enum, is_body_))
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
                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "extension" {
                    registry = self.parseFromExtension(registry)?;

                } // if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "extension" {

                // Конец 'extensions'.
                // End of 'extensions'.
                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "/extensions" {
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
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {
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
                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "require" {
                    registry = self.parseFromRequire(registry)?;

                } // if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "require" {

                // Конец 'extension'.
                // End of 'extension'.
                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "/extension" {
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
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {
                let token_ = self.tokenizer.nextToken1();

                name_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Ищем атрибут 'number'.
            // Search for the 'number' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "number" {
                let token_ = self.tokenizer.nextToken1();

                number_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Ищем атрибут 'author'.
            // Search for the 'author' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "author" {
                let token_ = self.tokenizer.nextToken1();

                author_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Ищем атрибут 'contact'.
            // Search for the 'contact' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "contact" {
                let token_ = self.tokenizer.nextToken1();

                contact_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Ищем атрибут 'supported'.
            // Search for the 'supported' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "supported" {
                let token_ = self.tokenizer.nextToken1();

                supported_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Ищем атрибут 'ratified'.
            // Search for the 'ratified' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "ratified" {
                let token_ = self.tokenizer.nextToken1();

                ratified_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Ищем атрибут 'nofeatures'.
            // Search for the 'nofeatures' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "nofeatures" {
                let token_ = self.tokenizer.nextToken1();

                nofeatures_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {
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
                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "enum" {
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

                    let extends_str = unsafe {std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*extends_rng_.start() ..= *extends_rng_.end()])};

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
                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "/require" {
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
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "extnumber" {
                let token_ = self.tokenizer.nextToken1();

                extnumber_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "extnumber" {

            // Ищем атрибут 'offset'.
            // Search for the 'offset' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "offset" {
                let token_ = self.tokenizer.nextToken1();

                offset_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "offset" {

            // Ищем атрибут 'extends'.
            // Search for the 'extends' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "extends" {
                let token_ = self.tokenizer.nextToken1();

                extends_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "extends" {

            // Ищем атрибут 'dir'.
            // Search for the 'dir' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "dir" {
                let token_ = self.tokenizer.nextToken1();

                dir_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "dir" {

            // Ищем атрибут 'bitpos'.
            // Search for the 'bitpos' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "bitpos" {
                let token_ = self.tokenizer.nextToken1();

                bitpos_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "bitpos" {

            // Ищем атрибут 'name'.
            // Search for the 'name' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {
                let token_ = self.tokenizer.nextToken1();

                name_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {
                let token_ = self.tokenizer.nextToken1();

                comment_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {

            // Ищем атрибут 'value'.
            // Search for the 'value' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "value" {
                let token_ = self.tokenizer.nextToken1();

                value_rng_ = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "value" {

            // Ищем атрибут 'alias'.
            // Search for the 'alias' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "alias" {
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
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn generateWvk(&self, data: &[u8], registry: &Registry) -> Result<String, String> {
        let mut output_wvk_ = String::new();

        registry.registry_types_vec
            .iter()
            .for_each(|types_| {
                types_.type_vec
                    .iter()
                    .for_each(|type_|{
                        match &type_.r#type {
                            RegistryTypeType::TYPE_BASE_TYPE(type_) => {
                                let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.type_body.name_rng.start() ..= *type_.type_body.name_rng.end()])};
                                let comment_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.type_body.comment_rng.start() ..= *type_.type_body.comment_rng.end()])};

                                output_wvk_.push_str(&format!("a{}\n", name_str_));
                            }
                            _ => {}
                        }
                    })
                //let comment_str_ = unsafe {std::str::from_utf8_unchecked(&data[*comment_rng_.start() ..= *comment_rng_.end()])};

                //output_wvk_.push_str(&format!("// {}\n\n", comment_str_));

                //
                //type_vec_
                //    .iter()
                //    .for_each(|type_|{
                //        let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.body.name_rng.start() ..= *type_.body.name_rng.end()])};
                //        let type_str_ = unsafe {std::str::from_utf8_unchecked(&data[*type_.body.type_rng.start() ..= *type_.body.type_rng.end()])};

                //        output_wvk_.push_str(&format!("pub struct {}({});\n", name_str_, type_str_));
                //    });

                //output_wvk_.push_str("\n");
            });

        /*registry.registry_types_vec
            .iter()
            .for_each(|types_ | {
               types_.type_section_vec
                   .iter()
                   .for_each(|type_section_| {
                       //let comment_str = unsafe {std::str::from_utf8_unchecked(&data[*type_section_.comment_rng.start() ..= *type_section_.comment_rng.end()])};
                       //output_wvk_.push_str(&format!("//{}\n", comment_str));

                       type_section_
                           .iterTypes()
                           .for_each(|type_| {
                               //let name_str = unsafe {std::str::from_utf8_unchecked(&data[*type_.name_rng.start() ..= *type_.name_rng.end()])};
                               let category_str = unsafe {std::str::from_utf8_unchecked(&data[*type_.category_rng.start() ..= *type_.category_rng.end()])};
                               //let type_str = unsafe {std::str::from_utf8_unchecked(&data[*type_.type_rng.start() ..= *type_.type_rng.end()])};
                               //let comment_str = unsafe {std::str::from_utf8_unchecked(&data[*type_.comment_rng.start() ..= *type_.comment_rng.end()])};

                               if category_str == "enum" {
                                   //output_wvk_.push_str(&format!("pub struct {}(i32); //{}\n\n", name_str, comment_str));
                               }

                               else if category_str == "bitmask" {
                                   //output_wvk_.push_str(&format!("pub struct {}({}); //{}\n\n", name_str, type_str, comment_str));
                               }
                           })
                   })
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
                            //let bitpos_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.bitpos_rng.start() ..= *enum_.bitpos_rng.end()])};
                            let value_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.value_rng.start() ..= *enum_.value_rng.end()])};
                            //let deprecated_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.deprecated_rng.start() ..= *enum_.deprecated_rng.end()])};
                            //let alias_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.alias_rng.start() ..= *enum_.alias_rng.end()])};
                            let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.name_rng.start() ..= *enum_.name_rng.end()])};
                            //let type_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.type_rng.start() ..= *enum_.type_rng.end()])};
                            //let comment_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.comment_rng.start() ..= *enum_.comment_rng.end()])};

                            output_wvk_.push_str(&format!("\tpub const {} = {};\n", name_str_, value_str_));
                        });

                    output_wvk_.push_str(&format!("}}\n\n"));
                }
                if type_str == "bitmask" {
                    output_wvk_.push_str(&format!("pub type {} = ...; //{}\n", name_str, comment_str));
                    output_wvk_.push_str(&format!("impl {} {{\n", name_str));

                    let bitwidth_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enums_.bitwidth_rng.start() ..= *enums_.bitwidth_rng.end()])};

                    if bitwidth_str_.is_empty() {
                        enums_.registry_enum_vec
                            .iter()
                            .try_for_each(|enum_| -> Result<(), String> {
                                let bitpos_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.bitpos_rng.start() ..= *enum_.bitpos_rng.end()])};
                                //let value_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.value_rng.start() ..= *enum_.value_rng.end()])};
                                //let deprecated_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.deprecated_rng.start() ..= *enum_.deprecated_rng.end()])};
                                //let alias_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.alias_rng.start() ..= *enum_.alias_rng.end()])};
                                let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.name_rng.start() ..= *enum_.name_rng.end()])};
                                //let type_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.type_rng.start() ..= *enum_.type_rng.end()])};
                                //let comment_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.comment_rng.start() ..= *enum_.comment_rng.end()])};

                                let bitpos_ = if bitpos_str_.is_empty() {
                                    0
                                }

                                else {
                                    let bitpos_ = bitpos_str_.parse::<u32>()
                                        .map_err(|e| format!("{}", e))?;

                                    1u32 << bitpos_
                                };

                                output_wvk_.push_str(&format!("\tpub const {} = {};\n", name_str_, bitpos_));

                                Ok(())
                            })?;
                    }

                    else if bitwidth_str_ == "64" {
                        enums_.registry_enum_vec
                            .iter()
                            .try_for_each(|enum_| -> Result<(), String> {
                                let bitpos_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.bitpos_rng.start() ..= *enum_.bitpos_rng.end()])};
                                //let value_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.value_rng.start() ..= *enum_.value_rng.end()])};
                                //let deprecated_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.deprecated_rng.start() ..= *enum_.deprecated_rng.end()])};
                                //let alias_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.alias_rng.start() ..= *enum_.alias_rng.end()])};
                                let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.name_rng.start() ..= *enum_.name_rng.end()])};
                                //let type_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.type_rng.start() ..= *enum_.type_rng.end()])};
                                //let comment_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enum_.comment_rng.start() ..= *enum_.comment_rng.end()])};

                                let bitpos_ = if bitpos_str_.is_empty() {
                                    0
                                }

                                else {
                                    let bitpos_ = bitpos_str_.parse::<u64>()
                                        .map_err(|e| format!("{}", e))?;

                                    1u64 << bitpos_
                                };

                                output_wvk_.push_str(&format!("\tpub const {} = {};\n", name_str_, bitpos_));

                                Ok(())
                            })?;
                    }

                    output_wvk_.push_str(&format!("}}\n\n"));
                }

                Ok(())
            })?;*/

        println!("asd");

        Ok(output_wvk_)
    }
}