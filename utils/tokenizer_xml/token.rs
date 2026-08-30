// SPDX-License-Identifier: None
// Copyright (c) 2026 None

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::ops::RangeInclusive;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
#[derive(Clone, PartialEq)]
pub enum TokenType {
    /// '<'
    TAG_BEGIN,
    /// '</'
    TAG_BEGIN_CLOSE,
    /// '<?'
    TAG_BEGIN_INSTRUCTION,
    /// '>'
    TAG_END,
    /// '/>'
    TAG_END_CLOSE,
    /// '?>'
    TAG_END_INSTRUCTION,
    /// <name
    TAG_NAME,
    /// attribute_name=
    ATTRIBUTE_NAME,
    /// ="attribute_value"
    ATTRIBUTE_VALUE,
    /// <tag>text<tag>
    TEXT,
    ///
    INVALID,
    ///
    END,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// # example
///
/// `<tag>`
///
/// Token:
/// - `r#type` = `TAG_NAME`
/// - `data` = `tag`
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
pub struct Token {
    /// Тип токена
    /// Token type
    pub(crate) r#type : TokenType,
    /// Данные токена
    /// Token data
    pub(crate) data_rng : RangeInclusive<usize>,
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные ассоциированные функции.
/// Public associated functions.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Token {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_create(r#type: TokenType, data_rng: RangeInclusive<usize>) -> Self {
        Self {
            r#type,
            data_rng: data_rng,
        }
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn s_createEmpty() -> Self {
        Self {
            r#type: TokenType::TAG_END,
            data_rng: 0 ..= 0,
        }
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// Публичные методы.
/// Public methods.
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
impl Token {
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn asType(&self) -> TokenType {
        self.r#type.clone()
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub fn asRange(&self) -> RangeInclusive<usize> {
        self.data_rng.clone()
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    //
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    pub unsafe fn asStr(&self, data_ptr: *const u8) -> &str {
        /*let str_ = std::str::from_utf8_unchecked(&data[self.data_rng.start..self.data_rng.end]);
        str_;*/

        let data_ = std::slice::from_raw_parts(
            data_ptr.add(*self.data_rng.start()),
            self.data_rng.end() - self.data_rng.start() + 1,
        );

        std::str::from_utf8_unchecked(data_)
    }
}