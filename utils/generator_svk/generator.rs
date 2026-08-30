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
use crate::registry_type::RegistryType;
use crate::registry_type_section::RegistryTypeSection;
use crate::registry_type_subsection::RegistryTypeSubsection;
use crate::registry_enum_section::RegistryEnumSection;
use crate::registry_enum::RegistryEnum;
use crate::registry_enum_enumerator::RegistryEnumEnumerator;
use crate::registry_enum_enumerator_extended::RegistryEnumEnumeratorExtended;

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
        let registry_enum_section_ = RegistryEnumSection::s_create();

        let mut registry_ = Registry::s_create();
        registry_.enum_section.push(registry_enum_section_);

        //Крутим пока не закончатся токены. Пока не будет токен
        loop {
            let token_ = self.tokenizer.nextToken1();

            // Ищем <types>
            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "types" {
                let registry_type_section_ = self.parseRegistryTypeSection();

                registry_.type_section.push(registry_type_section_);
            }

            // Ищем <enums>
            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "enums" {
                let registry_enum_ = self.parseRegistryEnum().unwrap();

                let name_str_ = unsafe {std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*registry_enum_.name_rng.start() ..= *registry_enum_.name_rng.end()])};

                registry_.enum_section.last_mut().unwrap().pushEnum(name_str_, registry_enum_);
            }

            // Ищем <extensions>
            if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "extensions" {
                registry_ = self.parseFromExtensions(registry_).unwrap();

                //let name_str_ = unsafe {std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*registry_enum_.name_rng.start() ..= *registry_enum_.name_rng.end()])};

                //registry_.enum_section.last_mut().unwrap().pushEnum(name_str_, registry_enum_);
            }
            
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
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryTypeSection(&mut self) -> RegistryTypeSection {
        let (comment_rng_,
            is_close_) = self.parseTypesTag();

        let mut registry_type_section_ = RegistryTypeSection::s_create();

        registry_type_section_.comment_rng = comment_rng_;

        let mut registry_type_subsection_ = RegistryTypeSubsection::s_create();

        if !is_close_ {
            loop {
                let token_ = self.tokenizer.nextToken1();

                // Начинаем парсинг нового RegistryType.
                // Start parsing the new RegistryType.
                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) == "type"} {
                    let registry_type_ = self.parseRegistryType().expect("Не валидный vk.xml");

                    //
                    let name_str_ = unsafe {std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*registry_type_.name_rng.start() ..= *registry_type_.name_rng.end()])};

                    registry_type_subsection_.pushType(name_str_, registry_type_);
                }

                // Начинаем парсинг нового RegistryTypeSubsection.
                // Start parsing the new RegistryTypeSubsection.
                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) == "comment"} {
                    registry_type_section_.pushSubsection(registry_type_subsection_);
                    registry_type_subsection_ = RegistryTypeSubsection::s_create();

                    // Лупаем до тех пор, пока не встретится '/comment'.
                    // We hit until we encounter '/comment'.
                    loop {
                        let token_ = self.tokenizer.nextToken1();

                        if token_.asType() == TokenType::TEXT {
                            registry_type_subsection_.comment_rng = token_.asRange();
                        }

                        if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "/comment" {
                            break;
                        }
                    } // loop {
                }

                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) == "/types"} {
                    break;
                }
            } // loop {
        } // if !is_close_ {

        registry_type_section_
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseTypesTag(&mut self) -> (RangeInclusive<usize>, bool) {
        let mut comment_rng = 1 ..= 0;

        let is_close_ = loop {
            // Получаем следующий токен.
            // Get the next token.
            let token_ = self.tokenizer.nextToken1();

            //let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
            //println!("{} = ", value_str_);

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {
                // Получаем следующий токен.
                // Get the next token.
                let token_ = self.tokenizer.nextToken1();

                comment_rng = token_.asRange();

                //let value_str_ = unsafe { std::str::from_utf8_unchecked(&self.data_rc.as_slice()[*token_.asRange().start() ..= *token_.asRange().end()]) };
                //println!("{}", value_str_);
            }

            if token_.asType() == TokenType::TAG_END {
                break false;
            }

            else if token_.asType() == TokenType::TAG_END_CLOSE {
                break true;
            }
        }; // let is_close_ = loop {

        (comment_rng, is_close_)
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    /// Разбирает тег 'type' и создает RegistryType.
    /// Из атрибутов извлекает requires, category, name и comment.
    /// Из вложенных тегов 'type' и 'name' извлекает диапазоны их текстового содержимого.
    /// Возвращает None при встрече INVALID или END токена.
    /// Parses the 'type' tag and creates a RegistryType.
    /// Retrieves requires, category, name and comment from attributes.
    /// From nested 'type' and 'name' tags, extract ranges of their text content.
    /// Returns None when an INVALID or END token is encountered.
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryType(&mut self) -> Option<RegistryType> {
        // Анализируем непосредственно тег 'type'.
        // We analyze the 'type' tag directly.
        let (requires_rng, category_rng, mut name_rng, comment_rng, is_close) = self.parseTypeTag()?;
        let mut type_rng = 1 ..= 0;

        // Если тег 'type' не самозакрывающийся, лупаем до тех пор, пока не встретим '/type'.
        // If the 'type' tag is not self-closing, loop around until we encounter '/type'.
        if !is_close {
            loop {
                let token_ = self.tokenizer.nextToken1();

                // Если внутри тега 'type' есть другой тег 'type' - это есть тип RegistryType.
                // If there is another 'type' tag inside the 'type' tag, this is a RegistryType.
                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "type" {
                    loop {
                        let token_ = self.tokenizer.nextToken1();

                        if token_.asType() == TokenType::TEXT {
                            type_rng = token_.asRange();
                        }

                        if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "/type" {
                            break;
                        }
                    }
                } // if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "type" {

                // Если внутри тег 'name'- это есть имя RegistryType.
                // If there is a 'name' tag inside, this is the name of the RegistryType.
                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {
                    loop {
                        let token_ = self.tokenizer.nextToken1();

                        if token_.asType() == TokenType::TEXT {
                            name_rng = token_.asRange();
                        }

                        if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "/name" {
                            break;
                        }
                    }
                } // else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {

                // RegistryType распарсен.
                // RegistryType parsed.
                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "/type" {
                    break;
                }

                // Если встретился не валидный токен или конечный токен.
                // If an invalid token or final token is encountered.
                else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                    return None;
                }
            } // loop {
        }

        Some(RegistryType::s_createWithData(requires_rng, category_rng, type_rng, name_rng, comment_rng))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseTypeTag(&mut self) -> Option<(RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, bool)> {
        let mut requires_rng = 1 ..= 0;
        let mut category_rng = 1 ..= 0;
        let mut name_rng = 1 ..= 0;
        let mut comment_rng = 1 ..= 0;

        // Лупаем до тех пор, пока не встретим конец открывающего тега.
        // Loop until we encounter the end of the opening tag.
        let is_close_ = loop {
            let token_ = self.tokenizer.nextToken1();

            // Ищем атрибут 'requires'.
            // Search for the 'requires' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "requires" {
                let token_ = self.tokenizer.nextToken1();

                requires_rng = token_.asRange();
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "requires" {

            // Ищем атрибут 'category'.
            // Search for the 'category' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "category" {
                let token_ = self.tokenizer.nextToken1();

                category_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "category" {

            // Ищем атрибут 'name'.
            // Search for the 'name' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {
                let token_ = self.tokenizer.nextToken1();

                name_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {
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

        Some((requires_rng, category_rng, name_rng, comment_rng, is_close_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryEnum(&mut self) -> Option<RegistryEnum> {
        // Анализируем непосредственно тег 'enums'.
        // We analyze the 'enums' tag directly.
        let (name_rng, type_rng, comment_rng, is_close) = self.parseEnumsTag()?;

        let mut registry_enum_enumerators_ = Vec::<RegistryEnumEnumerator>::new();
        let registry_enum_extended_enumerators_ = Vec::<RegistryEnumEnumeratorExtended>::new();

        // Если тег 'enums' не самозакрывающийся, лупаем до тех пор, пока не встретим '/enums'.
        // If the 'enums' tag is not self-closing, loop around until we encounter '/enums'.
        if !is_close {
            loop {
                let token_ = self.tokenizer.nextToken1();

                // Внутренний тег 'enum'.
                // Internal tag 'enum'.
                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "enum" {
                    let enumerant_ = self.parseRegistryEnumEnumerant()?;

                    registry_enum_enumerators_.push(enumerant_);

                } // if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "enum" {

                // RegistryEnum распарсен.
                // RegistryEnum parsed.
                else if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "/enums" {
                    break;
                }

                // Если встретился не валидный токен или конечный токен.
                // If an invalid token or final token is encountered.
                else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                    return None;
                }
            } // loop {
        }

        Some(RegistryEnum::s_createWithData(name_rng, type_rng, comment_rng, registry_enum_enumerators_, registry_enum_extended_enumerators_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseEnumsTag(&mut self) -> Option<(RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, bool)> {
        let mut name_rng = 1 ..= 0;
        let mut type_rng = 1 ..= 0;
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
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {

            // Ищем атрибут 'type'.
            // Search for the 'type' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "type" {
                let token_ = self.tokenizer.nextToken1();

                type_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "type" {

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {
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

        Some((name_rng, type_rng, comment_rng, is_close_))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseRegistryEnumEnumerant(&mut self) -> Option<RegistryEnumEnumerator> {
        // Анализируем непосредственно тег 'enum'.
        // We analyze the 'enum' tag directly.
        let (type_rng, value_rng, bitpos_rng, name_rng, comment_rng, is_close) = self.parseEnumTag()?;

        // Если тег 'enum' не самозакрывающийся, лупаем до тех пор, пока не встретим '/enum'.
        // If the 'enum' tag is not self-closing, loop around until we encounter '/enum'.
        if !is_close {
            loop {
                let token_ = self.tokenizer.nextToken1();

                // Не встречал пока в спеке <enum> ... </enum>.
                // I haven't seen <enum> ... </enum> in the spec yet.

                // RegistryEnum распарсен.
                // RegistryEnum parsed.
                if token_.asType() == TokenType::TAG_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "/enum" {
                    break;
                }

                // Если встретился не валидный токен или конечный токен.
                // If an invalid token or final token is encountered.
                else if token_.asType() == TokenType::INVALID || token_.asType() == TokenType::END {
                    return None;
                }
            } // loop {
        }

        Some(RegistryEnumEnumerator::s_createWithData(type_rng, value_rng, bitpos_rng, name_rng, comment_rng))
    }

    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    ///
    // ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    fn parseEnumTag(&mut self) -> Option<(RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, RangeInclusive<usize>, bool)> {
        let mut type_rng = 1 ..= 0;
        let mut value_rng = 1 ..= 0;
        let mut bitpos_rng = 1 ..= 0;
        let mut name_rng = 1 ..= 0;
        let mut comment_rng = 1 ..= 0;

        // Лупаем до тех пор, пока не встретим конец открывающего тега.
        // Loop until we encounter the end of the opening tag.
        let is_close_ = loop {
            let token_ = self.tokenizer.nextToken1();

            // Ищем атрибут 'type'.
            // Search for the 'type' attribute.
            if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "type" {
                let token_ = self.tokenizer.nextToken1();

                type_rng = token_.asRange();
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "type" {

            // Ищем атрибут 'value'.
            // Search for the 'value' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "value" {
                let token_ = self.tokenizer.nextToken1();

                value_rng = token_.asRange();
            } // if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "value" {

            // Ищем атрибут 'bitpos'.
            // Search for the 'bitpos' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "bitpos" {
                let token_ = self.tokenizer.nextToken1();

                bitpos_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "bitpos" {

            // Ищем атрибут 'name'.
            // Search for the 'name' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {
                let token_ = self.tokenizer.nextToken1();

                name_rng = token_.asRange();
            } // else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "name" {

            // Ищем атрибут 'comment'.
            // Search for the 'comment' attribute.
            else if token_.asType() == TokenType::ATTRIBUTE_NAME && unsafe { token_.asStr(self.data_rc.as_ptr()) } == "comment" {
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

        Some((type_rng, value_rng, bitpos_rng, name_rng, comment_rng, is_close_))
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
        // Анализируем непосредственно тег 'extension'.
        // We analyze the 'extension' tag directly.
        let (name_rng_,
            number_rng_,
            author_rng_,
            contact_rng_,
            supported_rng_,
            ratified_rng_,
            nofeatures_rng_,
            comment_rng_,
            is_close_) = self.parseExtensionTag()?;

        // Если тег 'enums' не самозакрывающийся, лупаем до тех пор, пока не встретим '/enums'.
        // If the 'enums' tag is not self-closing, loop around until we encounter '/enums'.
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

                    let registry_enum_enumerator_extended_ = RegistryEnumEnumeratorExtended::s_createWithData(extnumber_rng_,
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
                    }
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
    fn generateWvk(&self, data: &[u8], registry: &Registry) {
        let mut output_wvk_ = String::new();

        /*for section_ in registry.type_section.iter() {
            for subsection_ in section_.iterSubsection() {
                for type_ in subsection_.iterTypes() {
                    let name_str = unsafe {std::str::from_utf8_unchecked(&data[*type_.name_rng.start() ..= *type_.name_rng.end()])};
                    let category_str = unsafe {std::str::from_utf8_unchecked(&data[*type_.category_rng.start() ..= *type_.category_rng.end()])};
                    let type_str = unsafe {std::str::from_utf8_unchecked(&data[*type_.type_rng.start() ..= *type_.type_rng.end()])};
                    let comment_str = unsafe {std::str::from_utf8_unchecked(&data[*type_.comment_rng.start() ..= *type_.comment_rng.end()])};

                    output_wvk_.push_str(&format!("name = {}\n", name_str));
                    output_wvk_.push_str(&format!("category = {}\n", category_str));
                    output_wvk_.push_str(&format!("type = {}\n", type_str));
                    output_wvk_.push_str(&format!("comment = {}\n\n\n", comment_str));

                }
            }
        }*/

        registry.type_section
            .iter()
            .for_each(|section_| {
                section_.iterSubsection()
                    .for_each(|subsection_|{
                        subsection_.iterTypes()
                            .for_each(|type_| {
                                let name_str = unsafe {std::str::from_utf8_unchecked(&data[*type_.name_rng.start() ..= *type_.name_rng.end()])};
                                let category_str = unsafe {std::str::from_utf8_unchecked(&data[*type_.category_rng.start() ..= *type_.category_rng.end()])};
                                let type_str = unsafe {std::str::from_utf8_unchecked(&data[*type_.type_rng.start() ..= *type_.type_rng.end()])};
                                let comment_str = unsafe {std::str::from_utf8_unchecked(&data[*type_.comment_rng.start() ..= *type_.comment_rng.end()])};

                                output_wvk_.push_str(&format!("name = {}\n", name_str));
                                output_wvk_.push_str(&format!("category = {}\n", category_str));
                                output_wvk_.push_str(&format!("type = {}\n", type_str));
                                output_wvk_.push_str(&format!("comment = {}\n\n\n", comment_str));
                            })
                    })
            });

        for section_ in registry.enum_section.iter() {
            for enum_ in section_.iterEnums() {
                let name_str = unsafe {std::str::from_utf8_unchecked(&data[*enum_.name_rng.start() ..= *enum_.name_rng.end()])};
                let type_str = unsafe {std::str::from_utf8_unchecked(&data[*enum_.type_rng.start() ..= *enum_.type_rng.end()])};
                let comment_str = unsafe {std::str::from_utf8_unchecked(&data[*enum_.comment_rng.start() ..= *enum_.comment_rng.end()])};

                let type_str = match registry.findType(name_str) {
                    Some(registry_type) => {
                        unsafe {std::str::from_utf8_unchecked(&data[*registry_type.type_rng.start() ..= *registry_type.type_rng.end()])}
                    }
                    None => { "unknown" }
                };

                output_wvk_.push_str(&format!("pub struct {}({});\n", name_str, type_str));
                output_wvk_.push_str(&format!("type = {}\n", type_str));
                output_wvk_.push_str(&format!("comment = {}\n", comment_str));

                enum_.enumerators
                    .iter()
                    .for_each(|enumerant_| {
                        let type_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enumerant_.type_rng.start() ..= *enumerant_.type_rng.end()])};
                        let value_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enumerant_.value_rng.start() ..= *enumerant_.value_rng.end()])};
                        let bitpos_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enumerant_.bitpos_rng.start() ..= *enumerant_.bitpos_rng.end()])};
                        let name_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enumerant_.name_rng.start() ..= *enumerant_.name_rng.end()])};
                        let comment_str_ = unsafe {std::str::from_utf8_unchecked(&data[*enumerant_.comment_rng.start() ..= *enumerant_.comment_rng.end()])};

                        output_wvk_.push_str(&format!("\tname = {}\n", name_str_));
                        output_wvk_.push_str(&format!("\ttype = {}\n", type_str_));
                        output_wvk_.push_str(&format!("\tvalue = {}\n", value_str_));
                        output_wvk_.push_str(&format!("\tbitpos = {}\n", bitpos_str_));
                        output_wvk_.push_str(&format!("\tcomment = {}\n\n\n", comment_str_));
                    })
            }
        }

        println!("asd");
    }
}