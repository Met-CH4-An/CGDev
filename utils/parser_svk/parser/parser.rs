// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::hash::{Hash, Hasher};
use std::ops::{Deref, RangeInclusive};
use std::rc::Rc;
use crate::parser::binding::Binding;
use crate::parser::binding_constant::BindingConstant;
use crate::parser::binding_constants::BindingConstants;
use crate::parser::binding_enum::BindingEnum;
use crate::parser::binding_enum_extends::BindingEnumExtends;
use crate::parser::binding_enums::BindingEnums;
use crate::token::{Token, TokenType};
use crate::tokenizer::{Tokenizer, AVX2};
use crate::parser::vulkan_registry::VulkanRegistry;
use crate::parser::vulkan_registry_enums::VulkanRegistryEnums;
use crate::parser::vulkan_registry_enums_enum::VulkanRegistryEnumsEnum;
use crate::parser::vulkan_registry_enums_enum_ex::VulkanRegistryEnumsEnumEx;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
enum ParserState {
    ENUMS,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct Parser {
    data_ptr: *const u8,
    data_length: usize,
    pub data_rc: Rc<Vec<u8>>,
    // Токенайзер для получения токенов xml.
    // Tokenizer for obtaining xml tokens.
    tokenizer: Tokenizer<AVX2>,
    /// Дефолтный хэшер.
    ///
    hasher: std::hash::DefaultHasher,
    registry: VulkanRegistry,

}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Parser {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Публичные ассоциированные функции.
    // Public associated functions.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create() -> Self {
        Self {
            data_ptr: std::ptr::null(),
            data_length: 0,
            data_rc: Rc::new(Vec::<u8>::new()),
            tokenizer: Tokenizer::s_create(),
            hasher: std::hash::DefaultHasher::new(),
            registry: VulkanRegistry::s_create(),
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Публичные методы.
    // Public methods.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn buildRegistry(&mut self, vulkan_file_xml_name: &str) -> Result<VulkanRegistry, String>{
        // Наш готовящийся биндинг.
        // Our upcoming binding.
        let mut binding_ = Binding::s_create();

        // Загружаем данные.
        // Loading data.
        let data_vec_ = Self::s_loadData(vulkan_file_xml_name)?;

        self.data_rc = Rc::new(data_vec_);
        self.data_ptr = self.data_rc.as_ptr();
        self.data_length = self.data_rc.len();

        // Передаём данные в токенайзер.
        // Pass data to the tokenizer.
        self.tokenizer.setData(self.data_rc.clone());

        let mut parser_state_ = ParserState::ENUMS;
        loop {
            // Получаем следующий токен.
            // Get the next token.
            let token_ = self.tokenizer.nextToken1();

            if *token_.asRange().start() > 1358020 {
                //println!("asdasdasd");
                //break;
            }

            //println!("{}", unsafe { token_.asStr(self.data_ptr) });

            // Если встретили enums.
            // If enums are encountered.
            if token_.r#type == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_ptr) } == "enums" {
                self.parseEnumsSection(&mut binding_);

                //self.registry.vulkan_enums_hmap.insert(hash_, enums_);

                continue;
            }

            // Если встретили feature.
            // If feature are encountered.
            //if token_.r#type == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_ptr) } == "feature" {
            //    self.parseFeatureBlock();
            //}

            // Если встретили feature.
            // If feature are encountered.
            if token_.r#type == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_ptr) } == "extension" {
                self.parseExtensionSection(&mut binding_);
            }

            // Заканчиваем, если у токена специальный завершающий тип.
            // Terminate if the token has a special termination type.
            if let TokenType::END = token_.r#type {
                break;
            }
        }

        let mut output = String::new();
        unsafe { binding_.appendsString(self.data_rc.as_slice(), &mut output) }

        self.registry.print(self.data_rc.clone());

        Ok(unsafe { std::ptr::read(&self.registry)})
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Приватные ассоциированные функции.
    // Private associated functions.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn s_loadData(file_name_str: &str) -> Result<Vec<u8>, String> {
        // путь до файла с спецификацией вулкана - ../../external/vulkan/cargo.toml/
        // path to the file with the volcano specification - ../../external/vulkan/cargo.toml/
        let path_ = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap()
            .parent().unwrap()
            .join("external")
            .join("vulkan")
            .join(file_name_str);

        let data_ = std::fs::read(path_)
            .map_err(|std_error| {
                format!("{}", std_error)
            }
            )?;

        Ok(data_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Приватные методы.
    // Private methods.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Парсит секцию вида:
    /// <enums>
    ///     <enum>
    ///     <enum>
    /// </enums>
    ///
    /// Parses a section of the form:
    /// <enums>
    ///     <enum>
    ///     <enum>
    /// </enums>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseEnumsSection(&mut self, binding: &mut Binding) {
        let (name_rng, type_rng, comment_rng, is_close_) = self.parseEnumsTag();

        let mut hash_: u64 = 0;
        hash_ = self.makeHash(*name_rng.start() ..= *name_rng.end());

        let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*type_rng.start() ..= *type_rng.end()]) };

        // Если тэг enums не закрылся, итерируем пока не встретим </enums>.
        // If the enums tag has not closed, iterate until we encounter </enums>.
        if !is_close_ {
            if value_str_ == "constants" {
                self.parseEnumsSectionAsConstant(binding, hash_);
            }
            else if value_str_ == "enum" {
                self.parseEnumsSectionAsEnum(binding, hash_);
            }

        } // if !is_close_ {
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseEnumsSectionAsConstant(&mut self, binding: &mut Binding, hash: u64) {
        let mut binding_constants_ = BindingConstants::s_create();

        loop {
            // Получаем следующий токен.
            // Get the next token.
            let token_ = self.tokenizer.nextToken1();

            // Начинаем разбор тега <enum>.
            // Start parsing the <enum> tag.
            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_ptr) == "enum"} {
                let (type_rng, value_rng, name_rng, comment_rng, is_close_) = self.parseEnumTagAsConstant();

                if !is_close_ {
                    panic!("Кроносы изменили формат vk.xml. Секции enum не поддерживается.");
                }

                let binding_constant_ = BindingConstant::s_create(type_rng, value_rng, name_rng, comment_rng);

                binding_constants_.binding_constant_vec.push(binding_constant_);
            }

            else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_ptr) == "enums"} {
                panic!("Кроносы изменили формат vk.xml. Блок enums внутри enums не поддерживается.");
            }

            else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_ptr) == "/enums"} {
                break;
            }
        } // loop {

        binding.constants_hmap.insert(hash, binding_constants_);
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseEnumsSectionAsEnum(&mut self, binding: &mut Binding, hash: u64) {
        let mut binding_enums_ = BindingEnums::s_create();

        loop {
            // Получаем следующий токен.
            // Get the next token.
            let token_ = self.tokenizer.nextToken1();

            // Начинаем разбор тега <enum>.
            // Start parsing the <enum> tag.
            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_ptr) == "enum"} {
                let (value_rng, name_rng, comment_rng, is_close_) = self.parseEnumTagAsEnum();

                if !is_close_ {
                    panic!("Кроносы изменили формат vk.xml. Секции enum не поддерживается.");
                }

                let binding_enum_ = BindingEnum::s_create(value_rng, name_rng, comment_rng);

                binding_enums_.binding_enum_vec.push(binding_enum_);
            }

            else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_ptr) == "enums"} {
                panic!("Кроносы изменили формат vk.xml. Блок enums внутри enums не поддерживается.");
            }

            else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_ptr) == "/enums"} {
                break;
            }
        } // loop {

        binding.enums_hmap.insert(hash, binding_enums_);
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseEnumsTag(&mut self) -> (RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, bool) {
        let mut name_rng = 1 ..= 0;
        let mut type_rng = 1 ..= 0;
        let mut comment_rng = 1 ..= 0;

        let is_close_ = loop {
            // Получаем следующий токен.
            // Get the next token.
            let token_ = self.tokenizer.nextToken1();

            let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
            println!("{} = ", value_str_);

            // Ищем атрибут 'name'.
            // Search for the 'name' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "name" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                name_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'type'.
            // Search for the 'type' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "type" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                type_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "comment" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                comment_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            if token_.asType() == TokenType::TAG_END {
                break false;
            }

            else if token_.asType() == TokenType::TAG_END_CLOSE {
                break true;
            }
        }; // let is_close_ = loop {

        (name_rng, type_rng, comment_rng, is_close_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseEnumTagAsConstant(&mut self) -> (RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, bool) {
        let mut type_rng = 1 ..= 0;
        let mut value_rng = 1 ..= 0;
        let mut name_rng = 1 ..= 0;
        let mut comment_rng = 1 ..= 0;

        let is_close_ = loop {
            // Получаем следующий токен.
            // Get the next token.
            let token_ = self.tokenizer.nextToken1();

            let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
            println!("{} = ", value_str_);

            // Ищем атрибут 'type'.
            // Search for the 'type' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "type" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                type_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'value'.
            // Search for the 'value' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "value" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                value_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'name'.
            // Search for the 'name' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "name" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                name_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "comment" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                comment_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
                println!("{}", value_str_);
            }
            
            if token_.asType() == TokenType::TAG_END {
                break false;
            }

            else if token_.asType() == TokenType::TAG_END_CLOSE {
                break true;
            }
        }; // let is_close_ = loop {

        (type_rng, value_rng, name_rng, comment_rng, is_close_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseEnumTagAsEnum(&mut self) -> (RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, bool) {
        let mut value_rng = 1 ..= 0;
        let mut name_rng = 1 ..= 0;
        let mut comment_rng = 1 ..= 0;

        let is_close_ = loop {
            // Получаем следующий токен.
            // Get the next token.
            let token_ = self.tokenizer.nextToken1();

            let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
            println!("{} = ", value_str_);

            // Ищем атрибут 'value'.
            // Search for the 'value' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "value" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                value_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'name'.
            // Search for the 'name' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "name" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                name_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "comment" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                comment_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            if token_.asType() == TokenType::TAG_END {
                break false;
            }

            else if token_.asType() == TokenType::TAG_END_CLOSE {
                break true;
            }
        }; // let is_close_ = loop {

        (value_rng, name_rng, comment_rng, is_close_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Парсит секцию вида:
    /// <extension>
    ///     <require>
    ///     </require>
    /// </extension>
    ///
    /// Parses a section of the form:
    /// <extension>
    ///     <require>
    ///     </require>
    /// </extension>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseExtensionSection(&mut self, binding: &mut Binding) {
        let (name_rng_,
            number_rng_,
            type_rng_,
            author_rng_,
            contact_rng_,
            supported_rng_,
            ratified_rng_,
            nofeatures_rng_,
            comment_rng_,
            is_close_) = self.parseExtensionTag();

        let mut hash_: u64 = 0;
        hash_ = self.makeHash(*name_rng_.start() ..= *name_rng_.end());

        let name_extension_rng_ = name_rng_;
        let number_extension_rng_ = number_rng_;

        // Если тэг enums не закрылся, итерируем пока не встретим </enums>.
        // If the enums tag has not closed, iterate until we encounter </enums>.
        if !is_close_ {
            self.parseRequireSection(binding, name_extension_rng_, number_extension_rng_);
        } // if !is_close_ {
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// example:
    /// <extension name="name" number="number" type="type" author="author" contact="contact" supported="supported" ratified="ratified" nofeatures="nofeatures">
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseExtensionTag(&mut self) -> (RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, bool) {
        let mut name_rng = 1 ..= 0;
        let mut number_rng = 1 ..= 0;
        let mut type_rng = 1 ..= 0;
        let mut author_rng = 1 ..= 0;
        let mut contact_rng = 1 ..= 0;
        let mut supported_rng = 1 ..= 0;
        let mut ratified_rng = 1 ..= 0;
        let mut nofeatures_rng = 1 ..= 0;
        let mut comment_rng = 1 ..= 0;

        let is_close_ = loop {
            // Получаем следующий токен.
            // Get the next token.
            let token_ = self.tokenizer.nextToken1();

            let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
            println!("{} = ", value_str_);

            // Ищем атрибут 'name'.
            // Search for the 'name' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "name" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                name_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'number'.
            // Search for the 'number' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "number" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                number_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'type'.
            // Search for the 'type' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "type" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                type_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'author'.
            // Search for the 'author' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "author" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                author_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'contact'.
            // Search for the 'contact' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "contact" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                contact_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'supported'.
            // Search for the 'supported' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "supported" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                supported_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'ratified'.
            // Search for the 'ratified' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "ratified" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                ratified_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'nofeatures'.
            // Search for the 'nofeatures' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "nofeatures" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                nofeatures_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "comment" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                comment_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            if token_.asType() == TokenType::TAG_END {
                break false;
            }

            else if token_.asType() == TokenType::TAG_END_CLOSE {
                break true;
            }
        }; // let is_close_ = loop {

        (name_rng, number_rng, type_rng, author_rng, contact_rng, supported_rng, ratified_rng, nofeatures_rng, comment_rng, is_close_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Парсит секцию вида:
    /// <require>
    ///     <enum/>
    ///     <type/>
    ///     <command/>
    /// </require>
    ///
    /// Parses a section of the form:
    /// <require>
    ///     <enum/>
    ///     <type/>
    ///     <command/>
    /// </require>
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRequireSection(&mut self, binding: &mut Binding, name_extension_rng: RangeInclusive<usize>, number_extension_rng: RangeInclusive<usize>) {
        let (depends_rng,
            is_close_) = self.parseRequireTag();

        // Если тэг enums не закрылся, итерируем пока не встретим </enums>.
        // If the enums tag has not closed, iterate until we encounter </enums>.
        if !is_close_ {
            loop {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                // Начинаем разбор тега <enum>.
                // Start parsing the <enum> tag.
                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_ptr) == "enum"} {
                    let (value_rng,
                        offset_rng,
                        extends_rng,
                        dir_rng,
                        name_rng,
                        comment_rng,
                        is_close_) = self.parseEnumTagAsExtends();

                    if !is_close_ {
                        panic!("Кроносы изменили формат vk.xml. Секции enum не поддерживается.");
                    }

                    let extends_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*extends_rng.start() ..= *extends_rng.end()]) };

                    if extends_str_ == "" {
                        let mut hash_: u64 = 0;
                        hash_ = self.makeHash(*name_rng.start() ..= *name_rng.end());

                        //BindingConstant::s_create();
                    }

                    // Если у <enum> имеется extends="extends".
                    else {
                        let mut hash_: u64 = 0;
                        hash_ = self.makeHash(*extends_rng.start() ..= *extends_rng.end());

                        let binding_enums_ = binding.enums_hmap.get_mut(&hash_).unwrap();

                        let binding_enum_extends_ = BindingEnumExtends::s_create(number_extension_rng.clone(), offset_rng, dir_rng, name_rng, comment_rng);

                        binding_enums_.binding_enum_extends_vec.push(binding_enum_extends_);
                    }

                    //let binding_constant_ = BindingConstant::s_create(type_rng, value_rng, name_rng, comment_rng);

                    //binding_constants_.binding_constant_vec.push(binding_constant_);
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_ptr) == "enums"} {
                    panic!("Кроносы изменили формат vk.xml. Блок enums внутри enums не поддерживается.");
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_ptr) == "/require"} {
                    break;
                }
            } // loop {
        } // if !is_close_ {
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// example:
    /// <require depends="depends">
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRequireTag(&mut self) -> (RangeInclusive<usize>, bool) {
        let mut depends_rng = 1 ..= 0;

        let is_close_ = loop {
            // Получаем следующий токен.
            // Get the next token.
            let token_ = self.tokenizer.nextToken1();

            let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
            println!("{} = ", value_str_);

            // Ищем атрибут 'depends'.
            // Search for the 'depends' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "depends" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                depends_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            if token_.asType() == TokenType::TAG_END {
                break false;
            }

            else if token_.asType() == TokenType::TAG_END_CLOSE {
                break true;
            }
        }; // let is_close_ = loop {

        (depends_rng, is_close_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// example:
    /// <enum value="value" name="name" />
    /// <enum offset="offset" extends="extends" dir="dir" name="name" />
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseEnumTagAsExtends(&mut self) -> (RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, bool) {
        let mut value_rng = 1 ..= 0;
        let mut offset_rng = 1 ..= 0;
        let mut extends_rng = 1 ..= 0;
        let mut dir_rng = 1 ..= 0;
        let mut name_rng = 1 ..= 0;
        let mut comment_rng = 1 ..= 0;

        let is_close_ = loop {
            // Получаем следующий токен.
            // Get the next token.
            let token_ = self.tokenizer.nextToken1();

            let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
            println!("{} = ", value_str_);

            // Ищем атрибут 'value'.
            // Search for the 'value' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "value" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                value_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'offset'.
            // Search for the 'offset' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "offset" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                offset_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'extends'.
            // Search for the 'extends' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "extends" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                extends_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'dir'.
            // Search for the 'dir' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "dir" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                dir_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'name'.
            // Search for the 'name' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "name" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                name_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "comment" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                comment_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            if token_.asType() == TokenType::TAG_END {
                break false;
            }

            else if token_.asType() == TokenType::TAG_END_CLOSE {
                break true;
            }
        }; // let is_close_ = loop {

        (value_rng, offset_rng, extends_rng, dir_rng, name_rng, comment_rng, is_close_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseFeatureTag(&mut self) -> (u64, bool) {
        let is_close_ = loop {
            // Получаем следующий токен.
            // Get the next token.
            let token_ = self.tokenizer.nextToken1();

            //let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
            //println!("{}", value_str_);

            if token_.asType() == TokenType::TAG_END {
                break false;
            }

            if token_.asType() == TokenType::TAG_END_CLOSE {
                break true;
            };
        };


        (0, is_close_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// <enum extends="VkStructureType" extnumber="158" offset="0" name="VK_STRUCTURE_TYPE_BIND_BUFFER_MEMORY_INFO" />
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseEnumTagWithExtends(&mut self) -> (VulkanRegistryEnumsEnumEx, RangeInclusive<usize>, bool) {
        let mut enum_ex_ = VulkanRegistryEnumsEnumEx::s_create();
        let mut extends_rng = 1usize ..= 0usize;

        let is_close_ = loop {
            // Получаем следующий токен.
            // Get the next token.
            let token_ = self.tokenizer.nextToken1();

            //let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
            //println!("{}", value_str_);

            // Ищем атрибут с именем 'bitpos' => bitpos="5".
            // Search for an attribute named 'bitpos' => bitpos="5".
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "bitpos" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                enum_ex_.bitpos_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут с именем 'extends' => extends="VkStructureType".
            // Search for an attribute named 'extends' => extends="VkStructureType".
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "extends" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                extends_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут с именем 'extnumber' => extnumber="158".
            // Search for an attribute named 'extnumber' => extnumber="158".
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "extnumber" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                enum_ex_.extnumber_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут с именем 'name'
            // Search for an attribute named 'name'
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "name" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                enum_ex_.name_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут с именем 'offset' => offset="0".
            // Search for an attribute named 'offset' => offset="0".
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "offset" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                enum_ex_.offset_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут с именем 'dir'
            // Search for an attribute named 'dir'
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "dir" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                enum_ex_.dir_rng = token_.asRange();

                let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
                println!("{}", value_str_);
            }

            // Ищем атрибут с именем 'comment'
            // Search for an attribute named 'comment'
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_ptr) } == "comment" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                enum_ex_.comment_rng = token_.asRange();

                //let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start()..=*token_.asRange().end()]) };
                //println!("{}", value_str_);
            }

            if token_.asType() == TokenType::TAG_END {
                break false;
            }

            if token_.asType() == TokenType::TAG_END_CLOSE {
                break true;
            };
        };

        (enum_ex_, extends_rng, is_close_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn makeHash(&mut self, data_rng: RangeInclusive<usize>) -> u64 {
        let mut hasher_ = std::hash::DefaultHasher::new();

        // Делаем хеш имени.
        // Make a hash of the name.
        let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*data_rng.start() ..= *data_rng.end()]) };

        value_str_.hash(&mut hasher_);

        hasher_.finish()
    }
}