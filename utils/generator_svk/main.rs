// SPDX-License-Identifier: None
// Copyright (c) 2026 None

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// зависимости
// dependencies
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

use std::sync::Arc;
use utils__generator_svk__lib::{Registry, TypesItemTypeItemStructView, TypesItemTypeItemStructViewVariant};
use utils__generator_svk__lib::{
    TypesViewVariant,
    TypesItemTypeViewVariant
};

fn main() {
    // Загружаем данные.
    // Loading data.
    let data_vec_ = loadDataFromFile("vk.xml").expect("Не удалось загрузить файл. Failed to upload file.");

    let data_rc_ = Arc::new(data_vec_);

    // Создаем генератор.
    // Create a generator.
    let mut registry_ = Registry::create();

    registry_.setData(data_rc_);
    
    // Получаем сгенерированный svk.
    // Get the generated svk.
    registry_.build().unwrap();

    svk_generating(registry_);
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
///
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
fn loadDataFromFile(name : &str) -> Result<Vec<u8>, ()> {
    // путь до файла с спецификацией вулкана - ../../external/vulkan/cargo.toml/
    // path to the file with the volcano specification - ../../external/vulkan/cargo.toml/
    let path_ = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .join("external")
        .join("vulkan")
        .join(name);

    let data_ = std::fs::read(path_)
        .map_err(|std_error| {
            println!("{}", std_error);
            ()
        }
        )?;

    Ok(data_)
}

fn svk_generating(registry: Registry) {
    let mut struct_output_ = String::new();

    for types_view_ in registry.types().iter() {
        println!("{}\n", types_view_.comment());

        for types_view_variant in types_view_.content().iter() {
            match types_view_variant {
                TypesViewVariant::TYPE(item_type_) => {
                    match &item_type_.variant() {
                        TypesItemTypeViewVariant::STRUCT(item_struct_) => {
                            struct_output_ = svk_struct_generating(item_struct_, struct_output_);
                        }

                        _ => {}
                    }

                }
                _ => {}
            }
        }
    }

    println!("stope");
}

fn typeCppToRust(cpp_type: &str) -> Option<&str> {
    match cpp_type {
        "void" => {
            Some("std::ffi::c_void")
        }

        "char" => {
            Some("std::ffi::c_char")
        }

        "size_t" => {
            Some("usize")
        }

        "int8_t" => {
            Some("i8")
        }

        "uint8_t" => {
            Some("u8")
        }

        "int16_t" => {
            Some("i16")
        }

        "uint16_t" => {
            Some("u16")
        }

        "int32_t" => {
            Some("i32")
        }

        "uint32_t" => {
            Some("u32")
        }

        "int64_t" => {
            Some("i64")
        }

        "uint64_t" => {
            Some("u64")
        }

        "float" => {
            Some("f32")
        }

        "double" => {
            Some("f64")
        }

        _ => {
            Some(cpp_type)
        }
    }
}

fn modifierCppToRust(cpp_modifier: &str, pointer_qualification: Option<&'static str>) -> Option<&'static str> {
    let mut search_pos_ = 0;

    let output_= if let Some(v) = cpp_modifier.find("*") {
        search_pos_ += v + "*".len();

        if let Some(v) = cpp_modifier[search_pos_ ..].find("const") {
            search_pos_ += v + "const".len();

            if let Some(v) = cpp_modifier[search_pos_ ..].find("*") {
                if pointer_qualification.is_some() {
                    Some("*const *const")
                }
                else {
                    Some("*mut *const")
                }
            }

            else {
                Some("*const")
            }
        }

        else if let Some(v) = cpp_modifier[search_pos_ ..].find("*") {
            search_pos_ += v + "*".len();

            if let Some(v) = cpp_modifier[search_pos_ ..].find("const") {
                Some("*const *mut")
            }

            else {
                if pointer_qualification.is_some() {
                    Some("*mut *const")
                }
                else {
                    Some("*mut *mut")
                }
            }
        }

        else {
            if pointer_qualification.is_some() {
                Some("*const")
            }
            else {
                Some("*mut")
            }
        }
    }

    else if pointer_qualification.is_some() {
        Some("*const")
    }

    else {
        None
    };

    output_
}

fn svk_struct_generating(item_struct: &TypesItemTypeItemStructView, mut output: String) -> String {
    if item_struct.alias().is_empty() {
        output.push_str("#[repr(C)]\n");
        output.push_str(&format!("pub struct {} {{\n", item_struct.name()));

        for types_view_variant_ in item_struct.variants().iter() {
            match types_view_variant_ {
                TypesItemTypeItemStructViewVariant::MEMBER(item_member_view_) => {
                    let mut id_ = 0;
                    let mut type_rust_ = None;
                    let mut pointer_qualification_rust_ = None;

                    let type_body_with_enum_view_ = item_member_view_.typeBodyWithEnum();
                    let types_ = type_body_with_enum_view_.types();
                    for (id__, type_view_) in types_.iter().enumerate() {
                        // Если не const и не * и не пустое значение - значит нашли название типа.
                        // If it's not `const`, not `*`, and not a null value, then a type name has been found.
                        if !type_view_.value().contains("const")
                            && !type_view_.value().contains("*")
                            && !type_view_.value().is_empty() {

                            id_ = id__;
                            type_rust_ = typeCppToRust(type_view_.value());

                            // Проверяем в префиксах, есть ли const.
                            // Check the prefixes to see if “const” is present.
                            let pointer_qualification_ = if type_view_.prefix().contains("const") {
                                Some("const")
                            }

                            else {
                                None
                            };

                            // Анализируем постфикс.

                            let postfix_ = type_view_.postfix();

                            pointer_qualification_rust_ = modifierCppToRust(postfix_, pointer_qualification_);

                            break;
                        }
                    }

                    // Если тип найден, а квалификаторы нет, будем искать квалификаторы в примыкающих type.
                    // If the type is found but the qualifiers are not, we'll look for qualifiers in adjacent types.
                    if type_rust_.is_some() && pointer_qualification_rust_.is_none() {
                        let mut pointer_qualification_ = None;

                        // Смотрим предыдущий type, если он существует.
                        // Check for the previous type, if it exists.
                        if id_ > 0 {
                            let type_view_ = &types_[id_ - 1];
                            pointer_qualification_ = if type_view_.value().contains("const") {
                                Some("const")
                            }

                            else {
                                None
                            };
                        }

                        if id_ < types_.len() - 1 {
                            let type_view_ = &types_[id_ + 1];

                            pointer_qualification_rust_ = modifierCppToRust(type_view_.value(), pointer_qualification_);
                        }

                    }

                    if let Some(type_) = type_rust_ {
                        let item_name = type_body_with_enum_view_.name();
                        let name_ = item_name.value();

                        if let Some(qualification_) = pointer_qualification_rust_ {
                            output.push_str(&format!("\t{}: {} {}\n", name_, qualification_, type_));
                        }
                        else {
                            output.push_str(&format!("\t{}: {}\n", name_, type_));
                        }
                    }
                }

                TypesItemTypeItemStructViewVariant::COMMENT_ELT(v) => {
                    output.push_str(&format!("// {}\n", v.comment()));
                }
            }
        }


        output.push_str("}}\n\n");
    }

    else {
        println!("pub type {} = {};", item_struct.name(), item_struct.alias());
    }

    output
}

