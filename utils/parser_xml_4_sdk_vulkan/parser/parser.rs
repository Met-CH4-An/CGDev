// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::Deref;
use std::rc::Rc;
use crate::token::{Token, TokenType};
use crate::tokenizer::{Tokenizer, AVX2};
use crate::parser::vulkan_registry::VulkanRegistry;

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
    /// Токенайзер для получения токенов.
    /// Tokenizer for obtaining tokens.
    tokenizer: Tokenizer<AVX2>,
    /// Данные.
    /// Data.
    //data_ptr: *const u8,
    //data_length: usize,
    data_rc: Rc<Vec<u8>>,
    /// Структурированное содержимое файла со спецификацией вулкана.
    /// Structured contents of the volcano specification file.
    vulkan_registry: VulkanRegistry,
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
    pub fn s_create() -> Result<Self, String> {
        // Загружаем данные.
        // Loading data.
        let data_vec_ = Self::s_loadData("current.xml")?;

        let data_rc_ = Rc::new(data_vec_);
        //let data_ptr = data_rc_.deref().as_ptr();
        //let data_length = data_rc_.deref().len();
        let data_slice_ = data_rc_.deref().as_slice();

        // Создаем токенайзер.
        // Create a tokenizer.
        let mut tokenizer_ = Tokenizer::<AVX2>::s_create(data_rc_.clone())?;

        let mut parser_state_ = ParserState::ENUMS;
        loop {
            // Получаем следующий токен.
            // Get the next token.
            let token_ = tokenizer_.nextToken1();

            // Если встретили enums.
            // If enums are encountered.
            if token_.r#type == TokenType::TAG_NAME && unsafe { token_.asStr(data_slice_) } == "enums" {
                parser_state_ = ParserState::ENUMS;
            }

            match parser_state_ {
                // Если
                ParserState::ENUMS => {
                    
                }
                _ => {}
            }

            // Заканчиваем, если у токена специальный завершающий тип.
            // Terminate if the token has a special termination type.
            if let TokenType::END = token_.r#type {
                break;
            }
        }

        Ok(Self {
            tokenizer: tokenizer_,
            data_rc: data_rc_,
            vulkan_registry: VulkanRegistry::s_create(),
        })
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Публичные методы.
    // Public methods.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn buildRegistry(&mut self) {

    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    // Приватные ассоциированные функции.
    // Private associated functions.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_loadData(file_name_str: &str) -> Result<Vec<u8>, String> {
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
}